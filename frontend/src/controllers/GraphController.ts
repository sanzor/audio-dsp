// hooks/useGraphController.ts

import { useUIStore } from "@/Stores/UIStore";
import { useDeleteGraph, useEditGraph, useSaveGraphState } from "@/hooks/graphs/mutations";
import { useGraphStore } from "@/Stores/GraphStore";
import type { Node as RFNode, Edge as RFEdge } from "reactflow";

export function useGraphController() {
  // Zustand selectors
  const copyToClipboard = useUIStore(state => state.copyToClipboard);
  const closeModal = useUIStore(state => state.closeModal);
  const openModal = useUIStore(state => state.openModal);
  const closeContextMenu = useUIStore(state => state.closeContextMenu);

  // Data and mutations
  const graphMap = useGraphStore(x => x.graphs);
  const deleteGraphMutation = useDeleteGraph();
  const renameGraphMutation = useEditGraph();
  const saveGraphStateMutation = useSaveGraphState();

  return {
    // ============================================
    // DETAILS GRAPH
    // ============================================
    handleDetailsGraph: (graphId: number) => {
      const graph = graphMap.get(graphId);
      if (!graph) {
        console.error('Graph not found:', { graphId });
        return;
      }
      openModal({ type: 'detailsGraph', graphId });
      closeContextMenu();
    },

    // ============================================
    // RENAME GRAPH
    // ============================================
    handleRenameGraph: (graphId: number) => {
      const graph = graphMap.get(graphId);
      if (!graph) {
        console.error('Graph not found:', { graphId });
        return;
      }
      openModal({ type: 'renameGraph', graphId });
      closeContextMenu();
    },

    handleSubmitRenameGraph: async (graphId: number, newName: string) => {
      try {
        await renameGraphMutation.mutateAsync({ id: graphId, name: newName });
        closeModal();
      } catch (error) {
        console.error('Failed to rename graph:', error);
        throw error;
      }
    },

    // ============================================
    // DELETE GRAPH
    // ============================================
    handleDeleteGraph: async (graphId: number) => {
      try {
        await deleteGraphMutation.mutateAsync({ graph_id: graphId });
        closeContextMenu();
      } catch (error) {
        console.error('Failed to delete graph:', error);
        closeContextMenu();
        throw error;
      }
    },

    // ============================================
    // COPY GRAPH
    // ============================================
    handleCopyGraph: (graphId: number) => {
      const graph = graphMap.get(graphId);
      if (!graph) {
        console.error('Graph not found:', { graphId });
        return;
      }
      copyToClipboard({ type: 'graph', graphId });
      closeContextMenu();
    },

    // ============================================
    // SAVE GRAPH STATE
    // ============================================
    handleSaveGraph: async (graphId: number, nodes: RFNode[], edges: RFEdge[]) => {
      let nextTempId = -1;
      const normalizedNodeIds = new Map<string, number>();
      const normalizedEdgeIds = new Map<string, number>();
      const normalizeId = (rawId: string, cache: Map<string, number>) => {
        const numericId = Number(rawId);
        if (!Number.isNaN(numericId)) {
          return numericId;
        }
        const existing = cache.get(rawId);
        if (existing != null) {
          return existing;
        }
        const tempId = nextTempId--;
        cache.set(rawId, tempId);
        return tempId;
      };

      const repr = {
        schemaVersion: 1,
        nodes: nodes.map((n) => ({
          id: normalizeId(String(n.id), normalizedNodeIds),
          nodeType: n.data.nodeType as string,
          transformId: (n.data.transformId as number | null) ?? null,
          position: n.position,
          params: (n.data.params as Record<string, number> | undefined) ?? {},
        })),
        edges: edges.map((e) => ({
          id: normalizeId(String(e.id), normalizedEdgeIds),
          fromNodeId: normalizeId(String(e.source), normalizedNodeIds),
          toNodeId: normalizeId(String(e.target), normalizedNodeIds),
        })),
      };
      await saveGraphStateMutation.mutateAsync({ graphId, state: JSON.stringify(repr) });
    },

    handleUpdateNodeParams: (nodeId: number, params: Record<string, number>) => {
      for (const graph of graphMap.values()) {
        const nodeIndex = graph.nodes.findIndex((node) => node.id === nodeId);
        if (nodeIndex === -1) continue;

        const nextNodes = [...graph.nodes];
        nextNodes[nodeIndex] = {
          ...nextNodes[nodeIndex],
          params,
        };

        useGraphStore.getState().updateGraph(graph.id, {
          nodes: nextNodes,
          updatedAt: new Date(),
        });
        return;
      }

      console.error('Node not found:', { nodeId });
    },

    // ============================================
    // CLEAR GRAPH NODES
    // ============================================
    handleClearGraphNodes: (graphId: number) => {
      if (!graphMap.get(graphId)) return;
      openModal({ type: 'clearGraphNodes', graphId });
    },

    handleConfirmClearGraphNodes: async (graphId: number) => {
      const graph = graphMap.get(graphId);
      if (!graph) return;
      const structuralNodes = graph.nodes.filter(
        (n) => n.nodeType === "source" || n.nodeType === "sink"
      );
      const repr = {
        schemaVersion: 1,
        nodes: structuralNodes.map((n) => ({
          id: n.id,
          nodeType: n.nodeType,
          transformId: null,
          position: n.position,
          params: {},
        })),
        edges: [],
      };
      await saveGraphStateMutation.mutateAsync({ graphId, state: JSON.stringify(repr) });
      closeModal();
    },
  };
}
