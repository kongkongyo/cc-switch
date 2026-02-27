import { useState, useCallback } from "react";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";
import {
  streamCheckProvider,
  type StreamCheckResult,
} from "@/lib/api/model-test";
import type { AppId } from "@/lib/api";
import { useResetCircuitBreaker } from "@/lib/query/failover";

export function useStreamCheck(appId: AppId) {
  const { t } = useTranslation();
  const [checkingIds, setCheckingIds] = useState<Set<string>>(new Set());
  const resetCircuitBreaker = useResetCircuitBreaker();

  const checkProvider = useCallback(
    async (
      providerId: string,
      providerName: string,
    ): Promise<StreamCheckResult | null> => {
      setCheckingIds((prev) => new Set(prev).add(providerId));

      try {
        const result = await streamCheckProvider(appId, providerId);

        if (result.status === "operational") {
          toast.success(
            t("streamCheck.operational", {
              name: providerName,
              time: result.responseTimeMs,
              defaultValue: `${providerName} 运行正常 (${result.responseTimeMs}ms)`,
            }),
            { closeButton: true },
          );

          // 测试通过后重置熔断器状态
          resetCircuitBreaker.mutate({ providerId, appType: appId });
        } else if (result.status === "degraded") {
          toast.warning(
            t("streamCheck.degraded", {
              name: providerName,
              time: result.responseTimeMs,
              defaultValue: `${providerName} 响应较慢 (${result.responseTimeMs}ms)`,
            }),
          );

          // 降级状态也重置熔断器，因为至少能通信
          resetCircuitBreaker.mutate({ providerId, appType: appId });
        } else {
          toast.error(
            t("streamCheck.failed", {
              name: providerName,
              error: result.message,
              defaultValue: `${providerName} 检查失败: ${result.message}`,
            }),
            {
              description: t("streamCheck.failedHint", {
                defaultValue:
                  "检测结果仅供参考，部分供应商可能不支持此检测方式，建议手动测试确认。",
              }),
            },
          );
        }

        return result;
      } catch (e) {
        toast.error(
          t("streamCheck.error", {
            name: providerName,
            error: String(e),
            defaultValue: `${providerName} 检查出错: ${String(e)}`,
          }),
          {
            description: t("streamCheck.failedHint", {
              defaultValue:
                "检测结果仅供参考，部分供应商可能不支持此检测方式，建议手动测试确认。",
            }),
          },
        );
        return null;
      } finally {
        setCheckingIds((prev) => {
          const next = new Set(prev);
          next.delete(providerId);
          return next;
        });
      }
    },
    [appId, t, resetCircuitBreaker],
  );

  const isChecking = useCallback(
    (providerId: string) => checkingIds.has(providerId),
    [checkingIds],
  );

  return { checkProvider, isChecking };
}
