import { useCallback, useEffect, useMemo } from "react";
import type { Edge as RFEdge, Node as RFNode } from "reactflow";
import { apiGetTransformBinaries } from "@/Services/TransformService";
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore";
import type { ActiveEdge, ActiveGraph, ActiveNode } from "@/Stores/ActiveGraphState";
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore";
import type { CompiledGraph } from "@/audio/types/compiled";

type RuntimeShapeNode = {
  id: string | number;
  nodeType: string | undefined;
  transformId: number | null | undefined;
  params: Record<string, number>;
};

type RuntimeShapeEdge = {
  id: string | number;
  source: string | number;
  target: string | number;
};

function toNumericId(raw: string | number): number | null {
  const numeric = Number(raw);
  return Number.isFinite(numeric) ? numeric : null;
}

function buildRuntimeGraph(
  graphId: number | undefined,
  runtimeNodesShape: RuntimeShapeNode[],
  runtimeEdgesShape: RuntimeShapeEdge[],
): ActiveGraph | null {
  const sourceIds = new Set<number>();
  const sinkIds = new Set<number>();
  const runtimeNodes = new Map<number, ActiveNode>();

  for (const node of runtimeNodesShape) {
    const nodeId = toNumericId(node.id);
    if (nodeId == null) continue;

    const nodeType = node.nodeType;
    if (nodeType === "source") {
      sourceIds.add(nodeId);
      continue;
    }
    if (nodeType === "sink") {
      sinkIds.add(nodeId);
      continue;
    }

    const transformId = node.transformId;
    if (transformId == null) continue;

    runtimeNodes.set(nodeId, {
      id: nodeId,
      transformId,
      position: { x: 0, y: 0 },
      params: node.params,
      binary: null,
      binaryStatus: "idle",
      binaryError: null,
    });
  }

  if (runtimeNodes.size === 0) {
    return null;
  }

  const runtimeEdges = new Map<number, ActiveEdge>();
  for (const edge of runtimeEdgesShape) {
    const edgeId = toNumericId(edge.id);
    const fromNodeId = toNumericId(edge.source);
    const toNodeId = toNumericId(edge.target);
    if (edgeId == null || fromNodeId == null || toNodeId == null) continue;

    // Source/sink are UI constructs. Runtime nodes consume raw input when they
    // have no transform predecessors, and the last transform writes to output.
    if (sourceIds.has(fromNodeId) || sinkIds.has(toNodeId) || sourceIds.has(toNodeId) || sinkIds.has(fromNodeId)) {
      continue;
    }
    if (!runtimeNodes.has(fromNodeId) || !runtimeNodes.has(toNodeId)) {
      continue;
    }

    runtimeEdges.set(edgeId, {
      id: edgeId,
      fromNodeId,
      toNodeId,
      fromPortId: 0,
      toPortId: 0,
    });
  }

  return {
    id: graphId ?? null,
    regionId: null,
    name: "Live Canvas Graph",
    nodes: runtimeNodes,
    edges: runtimeEdges,
    isDirty: false,
    enabled: true,
  };
}

export function useLiveCanvasGraphRuntime(
  graphId: number | undefined,
  effectsEnabled: boolean,
  nodes: RFNode[],
  edges: RFEdge[],
): () => void {
  const graphController = useAudioEffectsStore((state) => state.graphController);
  const workletConnected = useAudioEffectsStore((state) => state.workletConnected);
  const setRuntimeState = useAudioEffectsStore((state) => state.setRuntimeState);
  const setGraphPlaybackState = useAudioEffectsStore((state) => state.setGraphPlaybackState);
  const binaries = useWasmBinaryStore((state) => state.binaries);
  const status = useWasmBinaryStore((state) => state.status);
  const setBinary = useWasmBinaryStore((state) => state.setBinary);
  const setStatus = useWasmBinaryStore((state) => state.setStatus);

  const runtimeShapeKey = useMemo(
    () =>
      JSON.stringify({
        nodes: nodes.map((node) => ({
          id: node.id,
          nodeType: node.data.nodeType as string | undefined,
          transformId: node.data.transformId as number | null | undefined,
          params: (node.data.params as Record<string, number> | undefined) ?? {},
        })),
        edges: edges.map((edge) => ({
          id: edge.id,
          source: edge.source,
          target: edge.target,
        })),
      }),
    [nodes, edges],
  );

  const runtimeShape = useMemo(
    () => JSON.parse(runtimeShapeKey) as { nodes: RuntimeShapeNode[]; edges: RuntimeShapeEdge[] },
    [runtimeShapeKey],
  );

  const runtimeGraph = useMemo(
    () => buildRuntimeGraph(graphId, runtimeShape.nodes, runtimeShape.edges),
    [graphId, runtimeShape],
  );
  const transformIds = useMemo(
    () => (runtimeGraph ? Array.from(new Set(Array.from(runtimeGraph.nodes.values()).map((node) => node.transformId))) : []),
    [runtimeGraph],
  );
  const transformIdsKey = useMemo(
    () => [...transformIds].sort((left, right) => left - right).join(","),
    [transformIds],
  );

  useEffect(() => {
    if (transformIds.length === 0) {
      return;
    }

    const missingBinaryIds = transformIds.filter(
      (transformId) => !binaries.has(transformId) && status.get(transformId) == null,
    );
    if (missingBinaryIds.length === 0) {
      return;
    }

    let cancelled = false;
    setRuntimeState(
      "hydrating",
      `Fetching transform binaries for IDs: ${missingBinaryIds.join(", ")}`,
    );

    const hydrate = async () => {
      try {
        for (const transformId of missingBinaryIds) {
          setStatus(transformId, "fetching");
        }

        const resolvedBinaries = await apiGetTransformBinaries(missingBinaryIds);
        if (cancelled) return;

        for (const transformId of missingBinaryIds) {
          const binary = resolvedBinaries.get(transformId);
          if (!binary) {
            setStatus(transformId, "error");
            continue;
          }
          setBinary(transformId, binary);
        }
      } catch (error) {
        if (cancelled) return;
        for (const transformId of missingBinaryIds) {
          setStatus(transformId, "error");
        }
        setRuntimeState("error", error instanceof Error ? error.message : "Failed to fetch transform binaries");
      }
    };

    void hydrate();

    return () => {
      cancelled = true;
    };
  }, [binaries, setBinary, setRuntimeState, setStatus, status, transformIds, transformIdsKey]);

  const compileNow = useCallback(() => {
    if (runtimeGraph == null) {
      const message = "No transform nodes in the current canvas graph.";
      setGraphPlaybackState({ compiled: false, playable: false, reason: message });
      setRuntimeState("idle", message);
      return;
    }

    const missingBinaryIds = transformIds.filter((transformId) => !binaries.has(transformId));
    const failedBinaryIds = transformIds.filter((transformId) => status.get(transformId) === "error");

    if (failedBinaryIds.length > 0) {
      const message = `Failed to load ${failedBinaryIds.length} transform binary${failedBinaryIds.length === 1 ? "" : "ies"}.`;
      setGraphPlaybackState({ compiled: false, playable: false, reason: message });
      setRuntimeState("error", message);
      return;
    }

    if (missingBinaryIds.length > 0) {
      const message = `Waiting on transform binaries for IDs: ${missingBinaryIds.join(", ")}`;
      setGraphPlaybackState({ compiled: false, playable: false, reason: message });
      setRuntimeState("hydrating", message);
      return;
    }

    let compiledGraph: CompiledGraph | null;
    try {
      compiledGraph = graphController.compileGraph(runtimeGraph);
    } catch (error) {
      const message = error instanceof Error ? error.message : "Failed to compile the live graph.";
      setGraphPlaybackState({ compiled: false, playable: false, reason: message });
      setRuntimeState("error", message);
      return;
    }

    if (!compiledGraph) {
      const message = "Graph compile is waiting on transform binaries.";
      setGraphPlaybackState({ compiled: false, playable: false, reason: message });
      setRuntimeState("hydrating", message);
      return;
    }

    if (!workletConnected) {
      const message = `Graph compiled with ${transformIds.length} transform${transformIds.length === 1 ? "" : "s"}, but the audio worklet is not connected yet.`;
      setGraphPlaybackState({ compiled: true, playable: false, reason: message });
      setRuntimeState("idle", message);
      return;
    }

    if (!effectsEnabled) {
      const message = `Graph compiled with ${transformIds.length} transform${transformIds.length === 1 ? "" : "s"}, but effects are currently bypassed.`;
      setGraphPlaybackState({ compiled: true, playable: false, reason: message });
      setRuntimeState("idle", message);
      return;
    }

    try {
      graphController.loadCompiledGraph(compiledGraph);
      const message = `Loaded live graph with ${runtimeGraph.nodes.size} transform node${runtimeGraph.nodes.size === 1 ? "" : "s"}.`;
      setGraphPlaybackState({ compiled: true, playable: true, reason: message });
      setRuntimeState("ready", message);
    } catch (error) {
      const message = error instanceof Error ? error.message : "Failed to compile the live graph.";
      setGraphPlaybackState({ compiled: false, playable: false, reason: message });
      setRuntimeState("error", message);
    }
  }, [
    binaries,
    effectsEnabled,
    graphController,
    runtimeGraph,
    setGraphPlaybackState,
    setRuntimeState,
    status,
    transformIds,
    workletConnected,
  ]);

  useEffect(() => {
    compileNow();
  }, [
    compileNow,
  ]);

  return compileNow;
}
