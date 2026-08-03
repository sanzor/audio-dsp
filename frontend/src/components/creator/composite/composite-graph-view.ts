import { useMemo } from "react";
import type { Edge as RFEdge, Node as RFNode } from "reactflow";
import { useResolveTransformDefinitions } from "@/hooks/transforms/queries";
import { computeNodeDisableSafety, type NodeDisableSafety } from "./compositeReachability";
import type { CompositeNodeData, CompositeIoNodeData } from "./composite-canvas-node";
import type { CanvasNode, EditingCompositeGraph } from "@/Stores/CompositeCanvasStore";

// Which nodes are the composite's Input/Output boundary nodes in each
// direction — shared by the reachability-safety coloring below and the
// ghost-slot visibility check (zero IO nodes in a direction means that
// ghost slot should still be showing).
function exposedDirectionNodeIds(editingGraph: EditingCompositeGraph) {
  const inputNodeIds = new Set<number>();
  const outputNodeIds = new Set<number>();
  for (const n of editingGraph.nodes.values()) {
    if (n.node_kind === "input") inputNodeIds.add(n.node_id);
    else if (n.node_kind === "output") outputNodeIds.add(n.node_id);
  }
  return { inputNodeIds, outputNodeIds };
}

// Derives the ReactFlow-facing view-model (nodes/edges + exposed-boundary
// flags) from the in-progress editing graph. Pure derivation, no handlers.
export function useCompositeGraphView(editingGraph: EditingCompositeGraph | null) {
  const distinctTransformIds = useMemo(
    () =>
      editingGraph
        ? [
            ...new Set(
              [...editingGraph.nodes.values()]
                .filter((n): n is Extract<CanvasNode, { node_kind: "leaf" }> => n.node_kind === "leaf")
                .map((n) => n.transform_id)
            ),
          ]
        : [],
    [editingGraph]
  );
  useResolveTransformDefinitions(distinctTransformIds);

  // Phase 3 reachability coloring — recomputed on every graph edit (node/
  // edge add/remove, disable toggle all produce a new `editingGraph`
  // reference, since every store mutation replaces it). Purely client-side;
  // no equivalent traversal exists in composite_validator.rs. See
  // compositeReachability.ts and agents/decisions/0005-composite-node-inspector.md.
  const nodeSafety = useMemo(() => {
    if (!editingGraph) return new Map<number, NodeDisableSafety>();
    const { inputNodeIds: exposedInputNodeIds, outputNodeIds: exposedOutputNodeIds } =
      exposedDirectionNodeIds(editingGraph);
    return computeNodeDisableSafety({
      nodeIds: [...editingGraph.nodes.keys()],
      edges: [...editingGraph.edges.values()],
      exposedInputNodeIds,
      exposedOutputNodeIds,
      disabledNodes: editingGraph.disabledNodes,
    });
  }, [editingGraph]);

  // Ghost-slot visibility (composite-io-placeholder.tsx): each direction's
  // hint disappears independently the moment at least one port in that
  // direction is exposed — deliberately NOT gated on nodes.size === 0, since
  // a composite can have leaf nodes placed but nothing exposed yet.
  const { hasExposedInput, hasExposedOutput } = useMemo(() => {
    if (!editingGraph) return { hasExposedInput: false, hasExposedOutput: false };
    const { inputNodeIds, outputNodeIds } = exposedDirectionNodeIds(editingGraph);
    return { hasExposedInput: inputNodeIds.size > 0, hasExposedOutput: outputNodeIds.size > 0 };
  }, [editingGraph]);

  const rfNodes: RFNode<CompositeNodeData | CompositeIoNodeData>[] = useMemo(
    () =>
      editingGraph
        ? [...editingGraph.nodes.values()].map((n: CanvasNode) =>
            n.node_kind === "leaf"
              ? {
                  id: String(n.node_id),
                  type: "composite",
                  position: n.position,
                  data: {
                    nodeId: n.node_id,
                    transformId: n.transform_id,
                    disabled: editingGraph.disabledNodes.has(n.node_id),
                    safety: nodeSafety.get(n.node_id) ?? "safe",
                  },
                }
              : {
                  id: String(n.node_id),
                  type: "compositeIo",
                  position: n.position,
                  data: {
                    nodeId: n.node_id,
                    direction: n.node_kind,
                    name: n.name,
                  },
                }
          )
        : [],
    [editingGraph, nodeSafety]
  );

  const rfEdges: RFEdge[] = useMemo(
    () =>
      editingGraph
        ? [...editingGraph.edges.values()].map((e) => ({
            id: `${e.from_node_id}:${e.from_port}->${e.to_node_id}:${e.to_port}`,
            source: String(e.from_node_id),
            target: String(e.to_node_id),
            sourceHandle: `out-${e.from_port}`,
            targetHandle: `in-${e.to_port}`,
          }))
        : [],
    [editingGraph]
  );

  return { rfNodes, rfEdges, hasExposedInput, hasExposedOutput };
}
