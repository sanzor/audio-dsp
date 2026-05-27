import type { CompiledGraphResult } from "./compileGraph";

export interface CompileOkResult {
  ok: true;
  descriptor: CompiledGraphResult;
  transformIds: number[];
}
