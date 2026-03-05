// /src/Stores/useRegionSetStore.ts

import type { Graph } from '@/Domain/Graph/Graph';
import { create, type StoreApi, type UseBoundStore } from 'zustand';




// ----------------------------------------------------
// 1. Normalized State & Action Definitions
// ----------------------------------------------------

type GraphCache = Map<string, Graph>;

interface GraphState {
    graphs: GraphCache;
    loading: boolean;
}

interface GraphActions {
    // CRUD Operations for the Set itself
    getGraph: (graphId: string) => Graph | undefined;
    setAllGraphs: (graphs: Graph[]) => void;
    addGraph: (graph: Graph) => void;
    removeGraph: (graphId: string) => void;
    updateGraph: (graphId: string, updates: Partial<Graph>) => void;
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
        const setMap = new Map<string, Graph>();
        newGraphs.forEach(s => setMap.set(s.id, s));
        set({ graphs: setMap, loading: false });
    },

    getGraph: (graphId: string) => {
        return get().graphs.get(graphId);
    },

    addGraph: (graphToAdd: Graph) => set((state: GraphState) => {
        const newMap = new Map(state.graphs);
        newMap.set(graphToAdd.id, graphToAdd);
        return { graphs: newMap };
    }),

    removeGraph: (graphId: string) => set((state: GraphState) => {
        const newMap = new Map(state.graphs);
        newMap.delete(graphId);
        return { graphs: newMap };
    }),

    updateGraph: (graphId: string, updates: Partial<Graph>) => set((state: GraphState) => {
        const setEntity = state.graphs.get(graphId);
        if (!setEntity) return state;

        const newMap = new Map(state.graphs);
        newMap.set(graphId, { ...setEntity, ...updates });
        return { graphs: newMap };
    })
}));
