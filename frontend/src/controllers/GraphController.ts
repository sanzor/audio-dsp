// hooks/useGraphController.ts

import { useUIStore } from "@/Stores/UIStore";
import { useDeleteGraph, useEditGraph } from "@/hooks/graphs/mutations";
import { useGraphStore } from "@/Stores/GraphStore";
import { apiSaveGraphState } from "@/Services/GraphService";
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
      const repr = {
        schemaVersion: 1,
        nodes: nodes.flatMap((n) => {
          const numId = Number(n.id);
          if (Number.isNaN(numId)) return [];
          return [{
            id: numId,
            nodeType: n.data.nodeType as string,
            transformId: (n.data.transformId as number | null) ?? null,
            position: n.position,
          }];
        }),
        edges: edges.flatMap((e) => {
          const fromId = Number(e.source);
          const toId = Number(e.target);
          if (Number.isNaN(fromId) || Number.isNaN(toId)) return [];
          return [{
            id: Number(e.id) || 0,
            fromNodeId: fromId,
            toNodeId: toId,
          }];
        }),
      };
      await apiSaveGraphState({ graphId, state: JSON.stringify(repr) });
    },
  };
}