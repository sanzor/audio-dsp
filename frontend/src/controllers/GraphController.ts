// hooks/useGraphController.ts

import { useUIStore } from "@/Stores/UIStore";
import { useDeleteGraph, useEditGraph } from "@/Orchestrators/Graphs/useGraphMutations";
import { useGraphStore } from "@/Stores/GraphStore";

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
    handleDetailsGraph: (graphId: string) => {
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
    handleRenameGraph: (graphId: string) => {
      const graph = graphMap.get(graphId);
      if (!graph) {
        console.error('Graph not found:', { graphId });
        return;
      }
      openModal({ type: 'renameGraph', graphId });
      closeContextMenu();
    },

    handleSubmitRenameGraph: async (graphId: string, newName: string) => {
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
    handleDeleteGraph: async (graphId: string) => {
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
    handleCopyGraph: (graphId: string) => {
      const graph = graphMap.get(graphId);
      if (!graph) {
        console.error('Graph not found:', { graphId });
        return;
      }
      copyToClipboard({ type: 'graph', graphId });
      closeContextMenu();
    },
  };
}