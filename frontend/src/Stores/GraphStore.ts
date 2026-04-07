// /src/Stores/useRegionSetStore.ts

import type { Graph } from '@/domain/Graph/Graph';
import { create, type StoreApi, type UseBoundStore } from 'zustand';




// ----------------------------------------------------
// 1. Normalized State & Action Definitions
// ----------------------------------------------------

type GraphCache = Map<number, Graph>;

interface GraphState {
    graphs: GraphCache;
    loading: boolean;
}

interface GraphActions {
    // CRUD Operations for the Set itself
    getGraph: (graphId: number) => Graph | undefined;
    setAllGraphs: (graphs: Graph[]) => void;
    addGraph: (graph: Graph) => void;
    removeGraph: (graphId: number) => void;
    updateGraph: (graphId: number, updates: Partial<Graph>) => void;
    clear: () => void;
}

type GraphStore = GraphState & GraphActions;

// ----------------------------------------------------
// 2. Zustand Store Implementation
// ----------------------------------------------------

export const useGraphStore: UseBoundStore<StoreApi<GraphStore>> = create<GraphStore>((set, get) => ({
    graphs: new Map(),
    loading: true,

    // --- CRUD ---

    setAllGraphs: (newGraphs: Graph[]) => {
        const setMap = new Map<number, Graph>();
        newGraphs.forEach(s => setMap.set(s.id, s));
        set({ graphs: setMap, loading: false });
    },

    getGraph: (graphId: number) => {
        return get().graphs.get(graphId);
    },

    addGraph: (graphToAdd: Graph) => set((state: GraphState) => {
        const newMap = new Map(state.graphs);
        newMap.set(graphToAdd.id, graphToAdd);
        return { graphs: newMap };
    }),

    removeGraph: (graphId: number) => set((state: GraphState) => {
        const newMap = new Map(state.graphs);
        newMap.delete(graphId);
        return { graphs: newMap };
    }),

    updateGraph: (graphId: number, updates: Partial<Graph>) => set((state: GraphState) => {
        const setEntity = state.graphs.get(graphId);
        if (!setEntity) return state;

        const newMap = new Map(state.graphs);
        newMap.set(graphId, { ...setEntity, ...updates });
        return { graphs: newMap };
    }),

    clear: () => set({ graphs: new Map(), loading: true }),
}));
