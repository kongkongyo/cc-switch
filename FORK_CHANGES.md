# fork-minimal 分支变更说明

本文档记录 `fork-minimal` 分支相对于上游 `main` 分支的所有修改。

## 功能概述

本分支在上游全部功能基础上，新增/恢复以下功能：

1. **供应商卡片模型名称显示**
2. **自动获取模型列表**
3. **恢复"测试模型"按钮**（上游已隐藏）

---

## 用户视角

### 供应商卡片模型名称显示

主界面每个供应商卡片上，在供应商名称后面直接显示当前配置的模型名称（如 `claude-sonnet-4-5-20250514`），与供应商名称同行显示，无需点进编辑页面即可查看。

支持的应用类型：
- Claude — 读取 `ANTHROPIC_MODEL` 环境变量
- Codex — 读取 config.toml 中的 `model` 字段
- Gemini — 读取 `GEMINI_MODEL` 环境变量

### 自动获取模型列表

编辑供应商时，Claude、Codex、Gemini、OpenCode 四种表单中均新增"自动获取模型"按钮。点击后自动通过供应商的 API 地址拉取可用模型列表，用户选择即可填入，无需手动查找和输入模型 ID。

### 恢复"测试模型"按钮

上游在 `68a0c304` 中以"供应商请求格式复杂难以统一测试"为由隐藏了该按钮。本分支恢复此功能，在每个供应商卡片的操作按钮中重新显示"测试模型"按钮（OpenCode 类型除外），点击后通过流式健康检查验证供应商可用性。

---

## 开发者视角

### 变更文件清单

| 文件 | 变更类型 | 说明 |
|------|---------|------|
| `src-tauri/src/services/provider/models.rs` | 新增 | 后端模型获取模块（OpenAI 兼容 `/v1/models` 接口） |
| `src-tauri/src/services/provider/mod.rs` | 修改 | 注册 models 子模块，导出 `FetchOpenAiModelsResponse`，新增 `fetch_openai_models` 方法 |
| `src-tauri/src/commands/provider.rs` | 修改 | 新增 `fetch_provider_models_openai` tauri command |
| `src-tauri/src/lib.rs` | 修改 | 注册 `fetch_provider_models_openai` command |
| `src-tauri/Cargo.toml` | 修改 | 新增依赖 |
| `src/lib/api/providers.ts` | 修改 | 新增 `fetchModelsOpenAi()` API 调用封装 |
| `src/components/providers/ProviderCard.tsx` | 修改 | 新增 `extractModelName()` 函数，模型名称紧跟供应商名称后同行显示 |
| `src/components/providers/ProviderList.tsx` | 修改 | 恢复 `useStreamCheck` 导入、`handleTest` 回调和 `onTest`/`isTesting` 属性传递 |
| `src/components/providers/forms/ProviderForm.tsx` | 修改 | 新增模型获取状态管理、回调逻辑和 UI 集成 |
| `src/components/providers/forms/ClaudeFormFields.tsx` | 修改 | 新增获取按钮和模型建议列表 |
| `src/components/providers/forms/CodexFormFields.tsx` | 修改 | 新增获取按钮和模型建议列表 |
| `src/components/providers/forms/GeminiFormFields.tsx` | 修改 | 新增获取按钮和模型建议列表 |
| `src/components/providers/forms/OpenCodeFormFields.tsx` | 修改 | 新增获取按钮、导入按钮和模型建议列表 |
| `src/components/providers/forms/hooks/useCodexConfigState.ts` | 修改 | 移除冗余空值检查 |
| `src/utils/providerConfigUtils.ts` | 修改 | 修复清空模型名时未删除 TOML 中 model 行的问题 |
| `.gitignore` | 修改 | 新增忽略规则 |

### 后端架构

```
src-tauri/src/services/provider/models.rs   ← 核心：OpenAI /v1/models 请求实现
        ↑
src-tauri/src/services/provider/mod.rs      ← ProviderService::fetch_openai_models()
        ↑
src-tauri/src/commands/provider.rs          ← tauri command: fetch_provider_models_openai
        ↑
src-tauri/src/lib.rs                        ← command 注册
```

### 前端架构

```
src/lib/api/providers.ts                    ← fetchModelsOpenAi() API 封装
        ↑
src/components/providers/forms/ProviderForm.tsx  ← 状态管理 + 回调
        ↑
ClaudeFormFields / CodexFormFields /        ← UI：获取按钮 + datalist 建议
GeminiFormFields / OpenCodeFormFields

src/components/providers/ProviderCard.tsx   ← extractModelName() + 供应商名称后同行渲染
src/components/providers/ProviderList.tsx   ← 恢复 useStreamCheck + handleTest
```

---

## 同步上游指南

本分支基于上游 `main` 通过 cherry-pick 方式构建，包含以下功能提交：

```
1a0c9b71 feat: 添加提取模型名称的功能
6f4b18da feat: 添加模型自动获取功能并更新相关组件
8f7294d4 feat: 添加流式健康检查功能
e6f3f80b feat: 添加分叉变更说明文档
```

另有未提交的调整：模型名称显示位置移至供应商名称后同行。

同步上游时推荐操作：

```bash
git fetch upstream
git rebase upstream/main
# 解决冲突（如有），确保两个功能提交保持在最上方
```
