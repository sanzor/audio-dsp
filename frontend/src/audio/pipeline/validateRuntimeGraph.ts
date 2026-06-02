import type { ActiveGraph } from "@/Stores/ActiveGraphState";
import type { PipelineErrorResult } from "./pipelineErrorResult";
import { errorResult } from "./pipelineErrorResult";

export type CompileRuntimeGraphErrorReason =
  | "empty_graph"
  | "invalid_graph"
  | "binary_load_failed"
  | "missing_binaries"
  | "compile_failed";


export interface ValidatedRuntimeGraph {
  graph: ActiveGraph;
  transformIds: number[];
}

export type ValidateGraphResult =
  | { ok: true; value: ValidatedRuntimeGraph }
  | PipelineErrorResult;



function collectTransformIds(graph: ActiveGraph): number[] {
  return Array.from(new Set(Array.from(graph.nodes.values(), (n) => n.transformId)));
}

export function validateGraph(runtimeGraph: ActiveGraph | null): ValidateGraphResult {
  if (!runtimeGraph || runtimeGraph.nodes.size === 0) {
    return errorResult("empty_graph", "No transform nodes.", []);
  }

  const transformIds = collectTransformIds(runtimeGraph);
  const invalidTransformIds = transformIds.filter((id) => !Number.isInteger(id) || id <= 0);
  if (invalidTransformIds.length > 0) {
    return errorResult("invalid_graph", `Invalid transform IDs: ${invalidTransformIds.join(", ")}`, invalidTransformIds);
  }

  const invalidEdgeIds: number[] = [];
  for (const edge of runtimeGraph.edges.values()) {
    if (!runtimeGraph.nodes.has(edge.fromNodeId) || !runtimeGraph.nodes.has(edge.toNodeId)) {
      invalidEdgeIds.push(edge.id);
    }
  }
  if (invalidEdgeIds.length > 0) {
    return errorResult("invalid_graph", `Edges reference missing nodes: ${invalidEdgeIds.join(", ")}`, transformIds);
  }

  return { ok: true, value: { graph: runtimeGraph, transformIds } };
}

