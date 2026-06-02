import type { CompileRuntimeGraphErrorReason } from "./validateRuntimeGraph";


export interface PipelineErrorResult {
  ok: false;
  reason: CompileRuntimeGraphErrorReason;
  detail: string;
  transformIds: number[];
}
export function errorResult(
  reason: CompileRuntimeGraphErrorReason,
  detail: string,
  transformIds: number[],
): PipelineErrorResult {
  return { ok: false, reason, detail, transformIds };
}