import { create } from "zustand";
import type {
  CompositeEdge,
  CompositeGraphDefinition,
  CompositeNode,
} from "@/domain/Transform/CompositeGraphDefinition";

// The in-progress composite-transform wiring graph being authored on the
// Creator's composite canvas. Kept as its own store rather than folded into
// CreatorStore.ts — this shape (nodes/edges maps) doesn't fit anything
// already there, unlike editingTransformSource's single string.
//
// Node position is purely a canvas-local, client-side concern — the backend
// (CompositeGraphSnapshot) has no position field, so it's never persisted.
// Reopening a saved composite re-lays-out nodes deterministically (see
// composite-canvas.tsx); dragging within a session is free-form.

export interface CanvasLeafNode {
  node_id: number;
  node_kind: "leaf";
  transform_id: number;
  position: { x: number; y: number };
}

export interface CanvasIoNode {
  node_id: number;
  node_kind: "input" | "output";
  name: string;
  position: { x: number; y: number };
}

// A composite's externally-visible ports are derived from Input/Output
// nodes wired into the graph (see CompositeGraphDefinition.ts) — so
// CanvasNode is a union just like the wire-format CompositeNode, both
// leaf and IO nodes living in the same `nodes` map.
export type CanvasNode = CanvasLeafNode | CanvasIoNode;

const edgeKey = (e: Pick<CompositeEdge, "from_node_id" | "from_port" | "to_node_id" | "to_port">) =>
  `${e.from_node_id}:${e.from_port}->${e.to_node_id}:${e.to_port}`;

// Default names for freshly-dropped IO nodes are generated as
// `${direction}_${n}`, unique across every Input/Output node's name in the
// graph (both directions share one namespace — see CompositeIoNode.name's
// doc comment in CompositeGraphDefinition.ts).
function nextIoName(graph: EditingCompositeGraph, direction: "input" | "output"): string {
  const existingNames = new Set(
    [...graph.nodes.values()].filter((n): n is CanvasIoNode => n.node_kind !== "leaf").map((n) => n.name)
  );
  let i = 1;
  let candidate = `${direction}_${i}`;
  while (existingNames.has(candidate)) {
    i += 1;
    candidate = `${direction}_${i}`;
  }
  return candidate;
}

export interface EditingCompositeGraph {
  transformId: number;
  nodes: Map<number, CanvasNode>;
  edges: Map<string, CompositeEdge>;
  // Session-only "temporarily excluded from the compiled graph" flag — no
  // backend concept, no field on CompositeNode/CompositeGraphDefinition (see
  // agents/decisions/0005-composite-node-inspector.md). Reset whenever a
  // composite is (re)opened via beginEditingCompositeGraph. A disabled node
  // and its incident edges are stripped out of toGraphDefinition()'s output
  // (so Save persists the graph as if the node were removed) and out of the
  // Preview/Play compile input (composite-canvas.tsx) — never out of `nodes`/
  // `edges` themselves, so the node stays visible (grayed out) on the canvas.
  disabledNodes: Set<number>;
  nextNodeId: number;
  revision: number;
  savedRevision: number;
}

interface CompositeCanvasState {
  editingGraph: EditingCompositeGraph | null;
  // Per-node inspector selection driving the bottom panel in
  // composite-canvas.tsx — net-new, replaces the old openModal({type:
  // "transformDetails"}) call on node click. Lives outside EditingCompositeGraph
  // since it's a pure view concern, not part of the graph being edited.
  selectedNodeId: number | null;

  beginEditingCompositeGraph: (
    transformId: number,
    initial: CompositeGraphDefinition | undefined,
    positions: Map<number, { x: number; y: number }>
  ) => void;
  addNode: (transformId: number, position: { x: number; y: number }) => number;
  addIoNode: (direction: "input" | "output", position: { x: number; y: number }) => number;
  renameIoNode: (nodeId: number, name: string) => void;
  removeNode: (nodeId: number) => void;
  moveNode: (nodeId: number, position: { x: number; y: number }) => void;
  addEdge: (edge: CompositeEdge) => void;
  removeEdge: (edge: Pick<CompositeEdge, "from_node_id" | "from_port" | "to_node_id" | "to_port">) => void;
  selectNode: (nodeId: number | null) => void;
  toggleNodeDisabled: (nodeId: number) => void;
  markSaved: () => void;
  isDirty: () => boolean;
  toGraphDefinition: () => CompositeGraphDefinition;
  reset: () => void;
}

function bump(state: EditingCompositeGraph): Pick<EditingCompositeGraph, "revision"> {
  return { revision: state.revision + 1 };
}

export const useCompositeCanvasStore = create<CompositeCanvasState>()((set, get) => ({
  editingGraph: null,
  selectedNodeId: null,

  beginEditingCompositeGraph: (transformId, initial, positions) => {
    const nodes = new Map<number, CanvasNode>();
    let maxNodeId = 0;
    for (const n of initial?.nodes ?? []) {
      const position = positions.get(n.node_id) ?? { x: 0, y: 0 };
      if (n.node_kind === "leaf") {
        nodes.set(n.node_id, { node_id: n.node_id, node_kind: "leaf", transform_id: n.transform_id, position });
      } else {
        nodes.set(n.node_id, { node_id: n.node_id, node_kind: n.node_kind, name: n.name, position });
      }
      maxNodeId = Math.max(maxNodeId, n.node_id);
    }
    const edges = new Map<string, CompositeEdge>();
    for (const e of initial?.edges ?? []) {
      edges.set(edgeKey(e), e);
    }
    set({
      editingGraph: {
        transformId,
        nodes,
        edges,
        disabledNodes: new Set(),
        nextNodeId: maxNodeId + 1,
        revision: 0,
        savedRevision: 0,
      },
      selectedNodeId: null,
    });
  },

  addNode: (transformId, position) => {
    const graph = get().editingGraph;
    if (!graph) return -1;
    const nodeId = graph.nextNodeId;
    const nodes = new Map(graph.nodes);
    nodes.set(nodeId, { node_id: nodeId, node_kind: "leaf", transform_id: transformId, position });
    set({ editingGraph: { ...graph, nodes, nextNodeId: nodeId + 1, ...bump(graph) } });
    return nodeId;
  },

  addIoNode: (direction, position) => {
    const graph = get().editingGraph;
    if (!graph) return -1;
    const nodeId = graph.nextNodeId;
    const name = nextIoName(graph, direction);
    const nodes = new Map(graph.nodes);
    nodes.set(nodeId, { node_id: nodeId, node_kind: direction, name, position });
    set({ editingGraph: { ...graph, nodes, nextNodeId: nodeId + 1, ...bump(graph) } });
    return nodeId;
  },

  renameIoNode: (nodeId, name) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const node = graph.nodes.get(nodeId);
    if (!node || node.node_kind === "leaf") return;
    const trimmed = name.trim();
    if (!trimmed) return;
    const existingNames = new Set(
      [...graph.nodes.values()]
        .filter((n): n is CanvasIoNode => n.node_kind !== "leaf" && n.node_id !== nodeId)
        .map((n) => n.name)
    );
    if (existingNames.has(trimmed)) return;
    const nodes = new Map(graph.nodes);
    nodes.set(nodeId, { ...node, name: trimmed });
    set({ editingGraph: { ...graph, nodes, ...bump(graph) } });
  },

  removeNode: (nodeId) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const nodes = new Map(graph.nodes);
    nodes.delete(nodeId);
    const edges = new Map(
      [...graph.edges].filter(([, e]) => e.from_node_id !== nodeId && e.to_node_id !== nodeId)
    );
    const disabledNodes = new Set(graph.disabledNodes);
    disabledNodes.delete(nodeId);
    set((state) => ({
      editingGraph: { ...graph, nodes, edges, disabledNodes, ...bump(graph) },
      selectedNodeId: state.selectedNodeId === nodeId ? null : state.selectedNodeId,
    }));
  },

  moveNode: (nodeId, position) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const node = graph.nodes.get(nodeId);
    if (!node) return;
    const nodes = new Map(graph.nodes);
    nodes.set(nodeId, { ...node, position });
    // Position is cosmetic only — doesn't affect wiring correctness, so it
    // doesn't bump revision (dragging a node shouldn't mark the graph dirty
    // for save-guard purposes... but simplicity here matters more than that
    // nuance; bump anyway so an accidental drag before Save isn't silently lost).
    set({ editingGraph: { ...graph, nodes, ...bump(graph) } });
  },

  addEdge: (edge) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const edges = new Map(graph.edges);
    edges.set(edgeKey(edge), edge);
    set({ editingGraph: { ...graph, edges, ...bump(graph) } });
  },

  removeEdge: (edge) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const edges = new Map(graph.edges);
    edges.delete(edgeKey(edge));
    set({ editingGraph: { ...graph, edges, ...bump(graph) } });
  },

  selectNode: (nodeId) => set({ selectedNodeId: nodeId }),

  // Toggling bumps revision (marks the graph dirty) even though the
  // flag itself is never persisted — Save's payload genuinely changes shape
  // (the node and its incident edges drop out, see toGraphDefinition below),
  // so the dirty indicator must reflect that real consequence.
  toggleNodeDisabled: (nodeId) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const disabledNodes = new Set(graph.disabledNodes);
    if (disabledNodes.has(nodeId)) disabledNodes.delete(nodeId);
    else disabledNodes.add(nodeId);
    set({ editingGraph: { ...graph, disabledNodes, ...bump(graph) } });
  },

  markSaved: () => {
    const graph = get().editingGraph;
    if (!graph) return;
    set({ editingGraph: { ...graph, savedRevision: graph.revision } });
  },

  isDirty: () => {
    const graph = get().editingGraph;
    return graph != null && graph.revision !== graph.savedRevision;
  },

  // Disabled nodes are filtered out here (and their incident edges with
  // them) — Save persists the graph as though they'd been removed. There's
  // no persisted "disabled" state to round-trip; re-opening this composite
  // later starts every node enabled again.
  toGraphDefinition: () => {
    const graph = get().editingGraph;
    if (!graph) return { nodes: [], edges: [] };
    const enabledNodes = [...graph.nodes.values()].filter((n) => !graph.disabledNodes.has(n.node_id));
    const enabledIds = new Set(enabledNodes.map((n) => n.node_id));
    const nodes: CompositeNode[] = enabledNodes.map((n) =>
      n.node_kind === "leaf"
        ? { node_id: n.node_id, node_kind: "leaf", transform_id: n.transform_id }
        : { node_id: n.node_id, node_kind: n.node_kind, name: n.name }
    );
    return {
      nodes,
      edges: [...graph.edges.values()].filter((e) => enabledIds.has(e.from_node_id) && enabledIds.has(e.to_node_id)),
    };
  },

  reset: () => set({ editingGraph: null, selectedNodeId: null }),
}));
