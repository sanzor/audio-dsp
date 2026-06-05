import { buildRuntimeGraph } from "./buildRuntimeGraph"
import { compileGraph } from "./compile-graph/compileGraph"
import { resolveBinaries } from "./resolve-binaries/resolveBinaries"
import type { CompiledGraphResult } from "./compile-graph/compileGraph"
import type { PipelineErrorResult } from "./pipelineErrorResult"
import { validateGraph } from "./validateRuntimeGraph"
import type { RunPipelineParams } from "./RunPipelineParams"


export interface CompileOkResult {
  ok: true;
  descriptor: CompiledGraphResult;
  transformIds: number[];
}


export type CompileGraphResult =
  | CompileOkResult
  | PipelineErrorResult;

  
export function runPipeline(
  { graphParams: { graphId, nodes, edges }, binaries }: RunPipelineParams,
): CompileGraphResult {
  const runtimeGraph = buildRuntimeGraph(graphId, nodes, edges)
  const validated = validateGraph(runtimeGraph)
  if (!validated.ok) return validated

  const compiled = compileGraph(validated.value.graph)
  if (!compiled) return { ok: false, reason: "compile_failed", detail: "Compile failed.", transformIds: validated.value.transformIds }

  const nodeTransformMap = new Map(
    Array.from(validated.value.graph.nodes.entries(), ([nodeId, node]) => [nodeId, node.transformId])
  )
  const resolved = resolveBinaries({ nodeTransformMap, binaryTransformsMap: binaries })
  if (!resolved) return { ok: false, reason: "missing_binaries", detail: "Some binaries could not be resolved.", transformIds: validated.value.transformIds }

  return {
    ok: true,
    descriptor: { compiledGraph: compiled, resolved_binaries: resolved.binaries },
    transformIds: validated.value.transformIds,
  }
}
