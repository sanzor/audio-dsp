import type { CompileOkResult } from "./compileOkResult";
import type { CompileErrorResult } from "./validateRuntimeGraph";

export type CompileGraphResult =
  | CompileOkResult
  | CompileErrorResult;
