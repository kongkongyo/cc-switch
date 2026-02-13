use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use crate::app_config::AppType;
use crate::error::AppError;
use crate::store::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAiModelDescriptor {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchOpenAiModelsResponse {
    pub models: Vec<OpenAiModelDescriptor>,
    pub resolved_url: String,
    pub elapsed_ms: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiModelsPayload {
    #[serde(default)]
    data: Vec<OpenAiModelItem>,
}

#[derive(Debug, Deserialize)]
struct OpenAiModelItem {
    id: Option<String>,
    #[serde(default)]
    owned_by: Option<String>,
    #[serde(default)]
    created: Option<i64>,
}

pub async fn fetch_openai_models(
    state: &AppState,
    app_type: AppType,
    provider_id: Option<&str>,
    base_url: &str,
    api_key: &str,
    timeout_secs: Option<u64>,
) -> Result<FetchOpenAiModelsResponse, AppError> {
    let base_url = base_url.trim().trim_end_matches('/').to_string();
    if base_url.is_empty() {
        return Err(AppError::InvalidInput("Base URL 不能为空".to_string()));
    }

    let api_key = api_key.trim().to_string();
    if api_key.is_empty() {
        return Err(AppError::InvalidInput("API Key 不能为空".to_string()));
    }

    let timeout = Duration::from_secs(timeout_secs.unwrap_or(15).clamp(5, 120));
    let client = build_client_for_fetch(state, &app_type, provider_id)?;
    let urls = build_models_urls(&base_url);
    let start = Instant::now();
    let mut warnings: Vec<String> = Vec::new();
    let mut last_error: Option<AppError> = None;

    for (index, url) in urls.iter().enumerate() {
        let is_last = index + 1 == urls.len();
        let response = client
            .get(url)
            .header("authorization", format!("Bearer {api_key}"))
            .header("accept", "application/json")
            .timeout(timeout)
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status().as_u16();
                if resp.status().is_success() {
                    let payload = resp
                        .json::<Value>()
                        .await
                        .map_err(|e| AppError::Message(format!("模型接口返回非 JSON: {e}")))?;
                    let models = parse_models_from_payload(payload)?;
                    return Ok(FetchOpenAiModelsResponse {
                        models,
                        resolved_url: url.to_string(),
                        elapsed_ms: start.elapsed().as_millis() as u64,
                        warnings,
                    });
                }

                let body = resp.text().await.unwrap_or_default();
                let should_fallback = !is_last && (status == 404 || status == 405);
                if should_fallback {
                    warnings.push(format!(
                        "地址 {url} 返回 HTTP {status}，已尝试备用地址"
                    ));
                    continue;
                }
                return Err(map_http_status_error(status, &body));
            }
            Err(err) => {
                last_error = Some(map_request_error(err));
                if !is_last {
                    warnings.push(format!("地址 {url} 请求失败，已尝试备用地址"));
                    continue;
                }
            }
        }
    }

    if let Some(err) = last_error {
        return Err(err);
    }

    Err(AppError::Message("获取模型失败".to_string()))
}

fn build_client_for_fetch(
    state: &AppState,
    app_type: &AppType,
    provider_id: Option<&str>,
) -> Result<reqwest::Client, AppError> {
    let provider_id = provider_id.map(|s| s.trim()).filter(|s| !s.is_empty());
    if let Some(pid) = provider_id {
        let provider = state
            .db
            .get_provider_by_id(pid, app_type.as_str())?
            .ok_or_else(|| AppError::Message(format!("供应商 {pid} 不存在")))?;
        let proxy_config = provider.meta.as_ref().and_then(|m| m.proxy_config.as_ref());
        return Ok(crate::proxy::http_client::get_for_provider(proxy_config));
    }

    Ok(crate::proxy::http_client::get())
}

fn build_models_urls(base_url: &str) -> Vec<String> {
    let normalized = base_url.trim().trim_end_matches('/').to_string();
    if normalized.is_empty() {
        return vec![];
    }

    if normalized.ends_with("/models") {
        return vec![normalized];
    }

    let mut urls = Vec::new();
    if !normalized.ends_with("/v1") {
        let fallback = format!("{normalized}/v1/models");
        if !urls.contains(&fallback) {
            urls.push(fallback);
        }
    }
    let direct = format!("{normalized}/models");
    if !urls.contains(&direct) {
        urls.push(direct);
    }
    urls
}

fn parse_models_from_payload(payload: Value) -> Result<Vec<OpenAiModelDescriptor>, AppError> {
    let parsed: OpenAiModelsPayload = serde_json::from_value(payload)
        .map_err(|e| AppError::Message(format!("模型列表解析失败: {e}")))?;

    let mut deduped: BTreeMap<String, OpenAiModelDescriptor> = BTreeMap::new();
    for item in parsed.data {
        let id = item
            .id
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let Some(id) = id else {
            continue;
        };
        deduped.entry(id.clone()).or_insert(OpenAiModelDescriptor {
            id,
            owned_by: item.owned_by,
            created: item.created,
        });
    }

    Ok(deduped.into_values().collect())
}

fn map_http_status_error(status: u16, body: &str) -> AppError {
    let tail = compact_error_tail(body);
    match status {
        401 | 403 => AppError::Message(format!("鉴权失败，请检查 API Key（HTTP {status}）{tail}")),
        404 | 405 => AppError::Message(format!(
            "模型接口不存在，请确认端点是否 OpenAI 兼容（HTTP {status}）{tail}"
        )),
        429 => AppError::Message(format!("请求过于频繁，请稍后重试（HTTP 429）{tail}")),
        _ => AppError::Message(format!("获取模型失败（HTTP {status}）{tail}")),
    }
}

fn compact_error_tail(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let single_line = trimmed.replace('\n', " ");
    let mut short = single_line.chars().take(180).collect::<String>();
    if single_line.chars().count() > 180 {
        short.push_str("...");
    }
    format!(": {short}")
}

fn map_request_error(err: reqwest::Error) -> AppError {
    if err.is_timeout() {
        return AppError::Message("请求超时，请检查网络或代理配置".to_string());
    }
    AppError::Message(format!("请求失败: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_models_urls_handles_v1_and_fallback() {
        let urls = build_models_urls("https://api.example.com");
        assert_eq!(
            urls,
            vec![
                "https://api.example.com/v1/models".to_string(),
                "https://api.example.com/models".to_string()
            ]
        );

        let urls = build_models_urls("https://api.example.com/v1");
        assert_eq!(urls, vec!["https://api.example.com/v1/models".to_string()]);
    }

    #[test]
    fn parse_models_from_payload_dedupes_and_sorts() {
        let payload = json!({
            "data": [
                { "id": "z-model", "owned_by": "a" },
                { "id": "a-model", "owned_by": "b" },
                { "id": "z-model", "owned_by": "c" },
                { "id": "" },
                { "name": "invalid" }
            ]
        });

        let parsed = parse_models_from_payload(payload).expect("parse should succeed");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].id, "a-model");
        assert_eq!(parsed[1].id, "z-model");
        assert_eq!(parsed[1].owned_by.as_deref(), Some("a"));
    }

    #[test]
    fn parse_models_from_payload_accepts_empty_data() {
        let payload = json!({ "data": [] });
        let parsed = parse_models_from_payload(payload).expect("parse should succeed");
        assert!(parsed.is_empty());
    }
}
