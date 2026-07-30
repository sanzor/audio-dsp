import { create } from "zustand";
import type {
  CompositeEdge,
  CompositeExposedPort,
  CompositeGraphDefinition,
} from "@/domain/Transform/CompositeGraphDefinition";

// The in-progress composite-transform wiring graph being authored on the
// Creator's composite canvas. Kept as its own store rather than folded into
// CreatorStore.ts — this shape (nodes/edges/exposed-ports maps) doesn't fit
// anything already there, unlike editingTransformSource's single string.
//
// Node position is purely a canvas-local, client-side concern — the backend
// (CompositeGraphSnapshot) has no position field, so it's never persisted.
// Reopening a saved composite re-lays-out nodes deterministically (see
// composite-canvas.tsx); dragging within a session is free-form.

export interface CanvasNode {
  node_id: number;
  transform_id: number;
  position: { x: number; y: number };
}

const edgeKey = (e: Pick<CompositeEdge, "from_node_id" | "from_port" | "to_node_id" | "to_port">) =>
  `${e.from_node_id}:${e.from_port}->${e.to_node_id}:${e.to_port}`;

const exposedKey = (nodeId: number, portName: string) => `${nodeId}:${portName}`;

export interface EditingCompositeGraph {
  transformId: number;
  nodes: Map<number, CanvasNode>;
  edges: Map<string, CompositeEdge>;
  exposedPorts: Map<string, CompositeExposedPort>;
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
  removeNode: (nodeId: number) => void;
  moveNode: (nodeId: number, position: { x: number; y: number }) => void;
  addEdge: (edge: CompositeEdge) => void;
  removeEdge: (edge: Pick<CompositeEdge, "from_node_id" | "from_port" | "to_node_id" | "to_port">) => void;
  setExposedPort: (nodeId: number, portName: string, exposedName: string) => void;
  clearExposedPort: (nodeId: number, portName: string) => void;
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
      nodes.set(n.node_id, {
        node_id: n.node_id,
        transform_id: n.transform_id,
        position: positions.get(n.node_id) ?? { x: 0, y: 0 },
      });
      maxNodeId = Math.max(maxNodeId, n.node_id);
    }
    const edges = new Map<string, CompositeEdge>();
    for (const e of initial?.edges ?? []) {
      edges.set(edgeKey(e), e);
    }
    const exposedPorts = new Map<string, CompositeExposedPort>();
    for (const p of initial?.exposed_ports ?? []) {
      exposedPorts.set(exposedKey(p.node_id, p.port_name), p);
    }
    set({
      editingGraph: {
        transformId,
        nodes,
        edges,
        exposedPorts,
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
    nodes.set(nodeId, { node_id: nodeId, transform_id: transformId, position });
    set({ editingGraph: { ...graph, nodes, nextNodeId: nodeId + 1, ...bump(graph) } });
    return nodeId;
  },

  removeNode: (nodeId) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const nodes = new Map(graph.nodes);
    nodes.delete(nodeId);
    const edges = new Map(
      [...graph.edges].filter(([, e]) => e.from_node_id !== nodeId && e.to_node_id !== nodeId)
    );
    const exposedPorts = new Map([...graph.exposedPorts].filter(([, p]) => p.node_id !== nodeId));
    const disabledNodes = new Set(graph.disabledNodes);
    disabledNodes.delete(nodeId);
    set((state) => ({
      editingGraph: { ...graph, nodes, edges, exposedPorts, disabledNodes, ...bump(graph) },
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
    // An edge can't simultaneously be exposed — connecting a previously
    // dangling/exposed port disconnects its exposure.
    const exposedPorts = new Map(graph.exposedPorts);
    exposedPorts.delete(exposedKey(edge.from_node_id, edge.from_port));
    exposedPorts.delete(exposedKey(edge.to_node_id, edge.to_port));
    set({ editingGraph: { ...graph, edges, exposedPorts, ...bump(graph) } });
  },

  removeEdge: (edge) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const edges = new Map(graph.edges);
    edges.delete(edgeKey(edge));
    set({ editingGraph: { ...graph, edges, ...bump(graph) } });
  },

  setExposedPort: (nodeId, portName, exposedName) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const exposedPorts = new Map(graph.exposedPorts);
    exposedPorts.set(exposedKey(nodeId, portName), { node_id: nodeId, port_name: portName, exposed_name: exposedName });
    set({ editingGraph: { ...graph, exposedPorts, ...bump(graph) } });
  },

  clearExposedPort: (nodeId, portName) => {
    const graph = get().editingGraph;
    if (!graph) return;
    const exposedPorts = new Map(graph.exposedPorts);
    exposedPorts.delete(exposedKey(nodeId, portName));
    set({ editingGraph: { ...graph, exposedPorts, ...bump(graph) } });
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

  // Disabled nodes are filtered out here (and their incident edges/exposed
  // ports with them) — Save persists the graph as though they'd been
  // removed. There's no persisted "disabled" state to round-trip; re-opening
  // this composite later starts every node enabled again.
  toGraphDefinition: () => {
    const graph = get().editingGraph;
    if (!graph) return { nodes: [], edges: [], exposed_ports: [] };
    const enabledNodes = [...graph.nodes.values()].filter((n) => !graph.disabledNodes.has(n.node_id));
    const enabledIds = new Set(enabledNodes.map((n) => n.node_id));
    return {
      nodes: enabledNodes.map((n) => ({ node_id: n.node_id, transform_id: n.transform_id })),
      edges: [...graph.edges.values()].filter((e) => enabledIds.has(e.from_node_id) && enabledIds.has(e.to_node_id)),
      exposed_ports: [...graph.exposedPorts.values()].filter((p) => enabledIds.has(p.node_id)),
    };
  },

  reset: () => set({ editingGraph: null, selectedNodeId: null }),
}));
