import { create, type StoreApi, type UseBoundStore } from 'zustand';
import type { Graph } from '@/domain/Graph/Graph';

// -------------------------------------------------------
// Types
// -------------------------------------------------------

// Tracks whether the .wasm binary for this node has been fetched.
// 'idle'     — not fetched yet, will be fetched on next play
// 'fetching' — request in flight
// 'ready'    — binary present, safe to send to worklet
// 'error'    — fetch failed
export type BinaryStatus = 'idle' | 'fetching' | 'ready' | 'error';

export interface ActiveNode {
  id: number;                          // negative = unsaved temp ID, positive = persisted DB id
  transformId: number;
  position: { x: number; y: number };
  params: Record<string, number>;      // param name → current value, sent to worklet at runtime
  binary: Uint8Array | null;           // fetched .wasm bytes, null until first play
  binaryStatus: BinaryStatus;
  binaryError: string | null;
}

export interface ActiveEdge {
  id: number;                          // negative = unsaved temp ID, positive = persisted DB id
  fromNodeId: number;
  toNodeId: number;
  fromPortId: number;
  toPortId: number;
}

export interface ActiveGraph {
  id: number | null;                   // null = not yet saved to backend
  regionId: number | null;
  name: string;
  nodes: Map<number, ActiveNode>;
  edges: Map<number, ActiveEdge>;
  isDirty: boolean;
  enabled: boolean;                    // whether audio routes through the graph on play
}

// -------------------------------------------------------
// Temp ID counter — negative so they never collide with DB IDs
// -------------------------------------------------------

let _tempId = -1;
const nextTempId = (): number => _tempId--;

// -------------------------------------------------------
// State & Actions
// -------------------------------------------------------

interface State {
  activeGraph: ActiveGraph | null;
}

interface Actions {
  loadFromGraph: (graph: Graph) => void;
  createEmpty: (regionId: number, name: string) => void;
  clear: () => void;

  addNode: (transformId: number, position: { x: number; y: number }) => number;
  removeNode: (nodeId: number) => void;
  updateNodePosition: (nodeId: number, position: { x: number; y: number }) => void;
  updateNodeParams: (nodeId: number, params: Record<string, number>) => void;
  setNodeBinary: (nodeId: number, binary: Uint8Array) => void;
  setNodeBinaryStatus: (nodeId: number, status: BinaryStatus, error?: string) => void;

  connectNodes: (fromNodeId: number, toNodeId: number, fromPortId: number, toPortId: number) => number;
  disconnectEdge: (edgeId: number) => void;

  setEnabled: (enabled: boolean) => void;

  persistIds: (nodeIdMap: Map<number, number>, edgeIdMap: Map<number, number>, graphId: number) => void;
}

type ActiveGraphStore = State & Actions;

// -------------------------------------------------------
// Helper — immutably patch a single node
// -------------------------------------------------------

function patchNode(
  state: State,
  nodeId: number,
  patch: Partial<ActiveNode>,
): Partial<State> {
  const graph = state.activeGraph;
  if (!graph) return state;
  const node = graph.nodes.get(nodeId);
  if (!node) return state;
  const nodes = new Map(graph.nodes);
  nodes.set(nodeId, { ...node, ...patch });
  return { activeGraph: { ...graph, nodes, isDirty: true } };
}

// -------------------------------------------------------
// Store
// -------------------------------------------------------

export const useActiveGraphState: UseBoundStore<StoreApi<ActiveGraphStore>> = create<ActiveGraphStore>((set, get) => ({
  activeGraph: null,

  // Populate from a persisted graph fetched from the backend.
  // Binaries are null — fetched lazily on first play.
  loadFromGraph: (graph) => {
    const nodes = new Map<number, ActiveNode>(
      graph.nodes.map((n) => [n.id, {
        id: n.id,
        transformId: 0,        // extended once backend carries transformId on nodes
        position: n.position,
        params: {},
        binary: null,
        binaryStatus: 'idle',
        binaryError: null,
      }])
    );

    const edges = new Map<number, ActiveEdge>(
      graph.edges.map((e) => [e.id, {
        id: e.id,
        fromNodeId: e.fromNodeId,
        toNodeId: e.toNodeId,
        fromPortId: 0,
        toPortId: 0,
      }])
    );

    set({
      activeGraph: {
        id: graph.id,
        regionId: graph.regionId,
        name: graph.name,
        nodes,
        edges,
        isDirty: false,
        enabled: false,
      },
    });
  },

  // Start a blank graph for a region that has no graph yet.
  createEmpty: (regionId, name) => {
    set({
      activeGraph: {
        id: null,
        regionId,
        name,
        nodes: new Map(),
        edges: new Map(),
        isDirty: false,
        enabled: false,
      },
    });
  },

  clear: () => set({ activeGraph: null }),

  // Returns the temp ID so the orchestrator can give ReactFlow the same ID,
  // keeping both in sync from the moment of creation.
  addNode: (transformId, position) => {
    const id = nextTempId();
    const graph = get().activeGraph;
    if (!graph) return id;
    const nodes = new Map(graph.nodes);
    nodes.set(id, {
      id,
      transformId,
      position,
      params: {},
      binary: null,
      binaryStatus: 'idle',
      binaryError: null,
    });
    set({ activeGraph: { ...graph, nodes, isDirty: true } });
    return id;
  },

  // Removes the node and any edges that referenced it.
  removeNode: (nodeId) => {
    const graph = get().activeGraph;
    if (!graph) return;
    const nodes = new Map(graph.nodes);
    nodes.delete(nodeId);
    const edges = new Map(graph.edges);
    for (const [edgeId, edge] of edges) {
      if (edge.fromNodeId === nodeId || edge.toNodeId === nodeId) {
        edges.delete(edgeId);
      }
    }
    set({ activeGraph: { ...graph, nodes, edges, isDirty: true } });
  },

  updateNodePosition: (nodeId, position) => {
    set((state) => patchNode(state, nodeId, { position }));
  },

  // Params are just numbers passed to the WASM module at runtime.
  // No recompile needed — the orchestrator sends updated params to the worklet directly.
  updateNodeParams: (nodeId, params) => {
    set((state) => patchNode(state, nodeId, { params }));
  },

  // Called by the orchestrator after a successful GET /transforms/{id}/wasm.
  setNodeBinary: (nodeId, binary) => {
    set((state) => patchNode(state, nodeId, { binary, binaryStatus: 'ready', binaryError: null }));
  },

  setNodeBinaryStatus: (nodeId, status, error) => {
    set((state) => patchNode(state, nodeId, { binaryStatus: status, binaryError: error ?? null }));
  },

  // Returns the temp edge ID so the orchestrator can give ReactFlow the same ID.
  connectNodes: (fromNodeId, toNodeId, fromPortId, toPortId) => {
    const id = nextTempId();
    const graph = get().activeGraph;
    if (!graph) return id;
    const edges = new Map(graph.edges);
    edges.set(id, { id, fromNodeId, toNodeId, fromPortId, toPortId });
    set({ activeGraph: { ...graph, edges, isDirty: true } });
    return id;
  },

  disconnectEdge: (edgeId) => {
    const graph = get().activeGraph;
    if (!graph) return;
    const edges = new Map(graph.edges);
    edges.delete(edgeId);
    set({ activeGraph: { ...graph, edges, isDirty: true } });
  },

  setEnabled: (enabled) => {
    const graph = get().activeGraph;
    if (!graph) return;
    set({ activeGraph: { ...graph, enabled } });
  },

  // After save, backend returns real IDs. We replace every negative temp ID
  // so future saves/deletes/fetches use the correct backend IDs.
  persistIds: (nodeIdMap, edgeIdMap, graphId) => {
    const graph = get().activeGraph;
    if (!graph) return;

    const nodes = new Map<number, ActiveNode>();
    for (const [oldId, node] of graph.nodes) {
      const newId = nodeIdMap.get(oldId) ?? oldId;
      nodes.set(newId, { ...node, id: newId });
    }

    const edges = new Map<number, ActiveEdge>();
    for (const [oldId, edge] of graph.edges) {
      const newId = edgeIdMap.get(oldId) ?? oldId;
      edges.set(newId, {
        ...edge,
        id: newId,
        fromNodeId: nodeIdMap.get(edge.fromNodeId) ?? edge.fromNodeId,
        toNodeId: nodeIdMap.get(edge.toNodeId) ?? edge.toNodeId,
      });
    }

    set({ activeGraph: { ...graph, id: graphId, nodes, edges, isDirty: false } });
  },
}));
