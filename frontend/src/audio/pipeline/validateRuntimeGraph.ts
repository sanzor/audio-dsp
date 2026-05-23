import type { ActiveGraph, BinaryStatus } from "@/Stores/ActiveGraphState";

export interface CompileRuntimeGraphDeps {
  binaries: Map<number, Uint8Array>;
  statuses: Map<number, BinaryStatus>;
}

export type CompileRuntimeGraphErrorReason =
  | "empty_graph"
  | "invalid_graph"
  | "binary_load_failed"
  | "missing_binaries"
  | "compile_failed";

export interface CompileErrorResult {
  ok: false;
  reason: CompileRuntimeGraphErrorReason;
  detail: string;
  transformIds: number[];
}

export interface ValidatedRuntimeGraph {
  graph: ActiveGraph;
  transformIds: number[];
}

export interface CompileParams {
  graph: ActiveGraph;
  transformIds: number[];
  binaries: Map<number, Uint8Array>;
}

export type ValidateRuntimeGraphResult =
  | { ok: true; value: ValidatedRuntimeGraph }
  | CompileErrorResult;

export type ResolveRuntimeGraphBinariesResult =
  | { ok: true; binaries: Map<number, Uint8Array> }
  | CompileErrorResult;

export type PrepareRuntimeGraphCompileResult =
  | { ok: true; value: CompileParams }
  | CompileErrorResult;

function errorResult(
  reason: CompileRuntimeGraphErrorReason,
  detail: string,
  transformIds: number[],
): CompileErrorResult {
  return {
    ok: false,
    reason,
    detail,
    transformIds,
  };
}

function collectRuntimeGraphTransformIds(graph: ActiveGraph): number[] {
  return Array.from(
    new Set(Array.from(graph.nodes.values(), (node) => node.transformId)),
  );
}

export function validateRuntimeGraph(runtimeGraph: ActiveGraph | null): ValidateRuntimeGraphResult {
  if (!runtimeGraph || runtimeGraph.nodes.size === 0) {
    return errorResult("empty_graph", "No transform nodes.", []);
  }

  const transformIds = collectRuntimeGraphTransformIds(runtimeGraph);
  const invalidTransformIds = transformIds.filter(
    (transformId) => !Number.isInteger(transformId) || transformId <= 0,
  );

  if (invalidTransformIds.length > 0) {
    return errorResult(
      "invalid_graph",
      `Invalid transform IDs: ${invalidTransformIds.join(", ")}`,
      invalidTransformIds,
    );
  }

  const invalidEdgeIds: number[] = [];
  for (const edge of runtimeGraph.edges.values()) {
    if (!runtimeGraph.nodes.has(edge.fromNodeId) || !runtimeGraph.nodes.has(edge.toNodeId)) {
      invalidEdgeIds.push(edge.id);
    }
  }

  if (invalidEdgeIds.length > 0) {
    return errorResult(
      "invalid_graph",
      `Edges reference missing nodes: ${invalidEdgeIds.join(", ")}`,
      transformIds,
    );
  }

  return {
    ok: true,
    value: {
      graph: runtimeGraph,
      transformIds,
    },
  };
}

export function resolveRuntimeGraphBinaries(
  transformIds: number[],
  deps: CompileRuntimeGraphDeps,
): ResolveRuntimeGraphBinariesResult {
  const failedIds = transformIds.filter((transformId) => deps.statuses.get(transformId) === "error");
  if (failedIds.length > 0) {
    return errorResult(
      "binary_load_failed",
      `Binary load failed: ${failedIds.join(", ")}`,
      failedIds,
    );
  }

  const binaries = new Map<number, Uint8Array>();
  const missingIds = transformIds.filter((transformId) => {
    const binary = deps.binaries.get(transformId);
    if (!binary) {
      return true;
    }

    binaries.set(transformId, binary);
    return false;
  });

  if (missingIds.length > 0) {
    const fetchingIds = missingIds.filter((transformId) => deps.statuses.get(transformId) === "fetching");
    const idleIds = missingIds.filter((transformId) => !fetchingIds.includes(transformId));
    const detailParts: string[] = [];

    if (fetchingIds.length > 0) {
      detailParts.push(`Fetching binaries: ${fetchingIds.join(", ")}`);
    }
    if (idleIds.length > 0) {
      detailParts.push(`Missing binaries: ${idleIds.join(", ")}`);
    }

    return errorResult(
      "missing_binaries",
      detailParts.join(". "),
      missingIds,
    );
  }

  return {
    ok: true,
    binaries,
  };
}

export function prepareRuntimeGraphCompile(
  validated: ValidatedRuntimeGraph,
  deps: CompileRuntimeGraphDeps,
): PrepareRuntimeGraphCompileResult {
  const resolved = resolveRuntimeGraphBinaries(validated.transformIds, deps);
  if (!resolved.ok) {
    return resolved;
  }

  return {
    ok: true,
    value: {
      graph: validated.graph,
      transformIds: validated.transformIds,
      binaries: resolved.binaries,
    },
  };
}
