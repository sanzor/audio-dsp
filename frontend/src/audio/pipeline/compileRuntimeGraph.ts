import type { ActiveNode } from "@/Stores/ActiveGraphState";
import { compileActiveGraph, type CompiledGraph } from "@/audio/pipeline/GraphCompiler";
import type {
  CompileParams,
  CompileErrorResult,
} from "@/audio/pipeline/validateRuntimeGraph";

export interface CompiledGraphResult {
  compiledGraph: CompiledGraph;
  resolved_binaries: Record<number, Uint8Array>;
}

export interface CompileOkResult {
  ok: true;
  descriptor: CompiledGraphResult;
  transformIds: number[];
}

export type CompileGraphResult =
  | CompileOkResult
  | CompileErrorResult;



function constructResolvedBinaries(
  nodes: Iterable<ActiveNode>,
  binaries: Map<number, Uint8Array>,
): Record<number, Uint8Array> | null {
  const resolvedBinaries: Record<number, Uint8Array> = {};

  for (const node of nodes) {
    const binary = binaries.get(node.transformId);
    if (!binary) {
      return null;
    }

    resolvedBinaries[node.transformId] = binary;
  }

  return resolvedBinaries;
}

export function compileGraph(
  params: CompileParams,
): CompileGraphResult {
  try {
    const compiledGraph = compileActiveGraph(params.graph);
    if (!compiledGraph) {
      return {
        ok: false,
        reason: "empty_graph",
        detail: "Graph is empty.",
        transformIds: params.transformIds,
      };
    }
    const resolvedBinaries = constructResolvedBinaries(
      params.graph.nodes.values(),
      params.binaries,
    );
    if (!resolvedBinaries) {
      return {
        ok:false,
        reason: "missing_binaries",
        detail: "Compile finished but resolved binaries were incomplete.",
        transformIds:[]
      }
    }
    return {
      ok: true,
      descriptor: {
        compiledGraph,
        resolved_binaries: resolvedBinaries,
      },
      transformIds: params.transformIds,
    };
  } catch (error) {
    return {
      ok: false,
      reason: "compile_failed",
      detail: String(error),
      transformIds: params.transformIds,
    }
  }
}
