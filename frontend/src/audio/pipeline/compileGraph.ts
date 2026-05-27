import type { ActiveNode } from "@/Stores/ActiveGraphState";
import { compileGraph as compileActiveGraph } from "@/audio/pipeline/GraphCompiler";
import type { CompileGraphResult } from "./compileGraphResult";
import type { CompiledGraph } from "./compiledGraph";
import type { CompileParams } from "./compileParams";

export type { CompileGraphResult } from "./compileGraphResult";

export interface CompiledGraphResult {
  compiledGraph: CompiledGraph;
  resolved_binaries: Record<number, Uint8Array>;
}

function resolveTransformBinaries(
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

function compileFailedResult(
  detail: string,
  transformIds: number[],
): CompileGraphResult {
  return {
    ok: false,
    reason: "compile_failed",
    detail,
    transformIds,
  };
}

export function compileGraph(
  params: CompileParams,
): CompileGraphResult {
  try {
    const compiledGraph = compileActiveGraph(params.graph);
    if (!compiledGraph) {
      return compileFailedResult("Compile failed.", params.transformIds);
    }

    const resolvedBinaries = resolveTransformBinaries(
      params.graph.nodes.values(),
      params.binaries,
    );

    if (!resolvedBinaries) {
      return compileFailedResult(
        "Compile finished but resolved binaries were incomplete.",
        params.transformIds,
      );
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
    return compileFailedResult(
      error instanceof Error ? error.message : "Compile failed.",
      params.transformIds,
    );
  }
}
