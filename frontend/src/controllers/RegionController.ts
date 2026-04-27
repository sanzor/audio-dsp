// hooks/useRegionSetController.ts
import { useUIStore } from "@/Stores/UIStore";
import type { PasteGraphParams } from "@/Stores/PasteParams";
import { useCopyGraph, useCreateGraph } from "@/hooks/graphs/mutations";
import { useDeleteRegion, useEditRegion } from "@/hooks/regions/mutations";
import type { CreateGraphParams } from "@/Services/GraphService";
import { useRegionStore } from "@/Stores/RegionStore";
import { useGraphStore } from "@/Stores/GraphStore";


export function useRegionController() {
  // Zustand selectors
  const copyToClipboard = useUIStore(state => state.copyToClipboard);
  const closeModal = useUIStore(state => state.closeModal);
  const openModal = useUIStore(state => state.openModal);
  const closeContextMenu = useUIStore(state => state.closeContextMenu);
  
  // Data and mutations
  const regionMap = useRegionStore(x=>x.regions);
  const graphMap = useGraphStore(x=>x.graphs);
  const createGraphMutation = useCreateGraph();
  const copyGraphMutation = useCopyGraph();
  const deleteRegionMutation = useDeleteRegion();
  const editRegionMutation = useEditRegion();



  return {
    // ============================================
    // CREATE REGION
    // ============================================
    handleCreateGraph: (regionId:number) => {
      const region =regionMap.get(regionId);
      if (!region) {
        console.error('Region  not found:', { regionId });
        return;
      }
      
      openModal({ type: 'createGraph', regionId });
      closeContextMenu(); // ✅ Close context menu when opening modal
    },

    handleSubmitCreateGraph: async (params: CreateGraphParams) => {
      try {
        await createGraphMutation.mutateAsync(params);
        closeModal(); // ✅ Close modal on success
        // Optional: Show success toast
      } catch (error) {
        console.error('Failed to create graph:', error);
        // ❌ Don't close modal on error - let user fix/retry
        // Optional: Show error toast or inline error
        throw error; // Re-throw so modal can handle it
      }
    },

    // ============================================
    // DETAILS REGION SET
    // ============================================
    handleDetailsRegion: (regionId:number) => {
      const region =regionMap.get(regionId);
      if (!region) {
        console.error('Region not found:', { regionId });
        return;
      }
      
      openModal({ type: 'detailsRegion', regionId });
      closeContextMenu(); // ✅ Close context menu when opening modal
    },

    // ============================================
    // RENAME REGION SET
    // ============================================
    handleEditRegion: (regionId:number) => {
      const region =regionMap.get(regionId);
      if (!region) {
        console.error('Region  not found:', { regionId });
        return;
      }
      
      openModal({ type: "renameRegion", regionId });
      closeContextMenu(); // ✅ Close context menu when opening modal
    },

    handleSubmitRenameRegion: async (regionId:number, newName: string) => {
      const region = regionMap.get(regionId);
      if (!region) {
        console.error('Region not found:', { regionId });
        throw new Error('Region not found');
      }
      try {
        await editRegionMutation.mutateAsync({
          regionId,
          name: newName,
        });
        closeModal(); // ✅ Close modal on success
        // Optional: Show success toast
      } catch (error) {
        console.error('Failed to rename region:', error);
        // ❌ Don't close modal on error
        throw error;
      }
    },

    handleUpdateRegionBounds: async (regionId: number, startTime: number, endTime: number) => {
      const region = regionMap.get(regionId);
      if (!region) {
        console.error('Region not found:', { regionId });
        throw new Error('Region not found');
      }

      await editRegionMutation.mutateAsync({
        regionId,
        startTime,
        endTime,
      });
    },

    // ============================================
    // DELETE REGION
    // ============================================
    handleDeleteRegion: (regionId: number) => {
      if (!regionMap.get(regionId)) return;
      openModal({ type: 'deleteRegion', regionId });
      closeContextMenu();
    },

    handleConfirmDeleteRegion: async (regionId: number) => {
      try {
        await deleteRegionMutation.mutateAsync({ regionId });
        closeModal();
      } catch (error) {
        console.error('Failed to delete region:', error);
        throw error;
      }
    },

    // ============================================
    // COPY REGION SET
    // ============================================
    handleCopyRegion: (regionId:number) => {
      const region =regionMap.get(regionId);
      if (!region) {
        console.error('Region  not found:', { regionId });
        return;
      }
      
      copyToClipboard({ type: "region", regionId});
      closeContextMenu(); // ✅ Close context menu after action
      
      // Optional: Show success toast
      console.log("Copied region:", region.name);
    },

    // ============================================
    // PASTE REGION
    // ============================================
    handlePasteGraph(destRegionId:number) {
      // 1. Validate source type
      const clipboard = useUIStore.getState().clipboard;
      if (!clipboard || clipboard.type !== "graph") return;

      // 2. Validate destination existence
      const destRegion =regionMap.get(destRegionId);
      if (!destRegion) {
        console.error('Region  not found:', { destRegionId });
        return;
      }
      // 2. Validate destination existence
      

      // 3. (Optional but recommended) Validate source existence
     

      const sourceGraph = graphMap.get(clipboard.graphId);
      if (!sourceGraph) return;

      // 4. Everything valid → open modal
      openModal({
        type: "pasteGraph",
        params: {
          source: {
            graphId:sourceGraph.id
          },
          destination: {
            regionId:destRegionId
          }
        }
      });

      closeContextMenu();
    },

    handleSubmitPasteGraph: async (params: PasteGraphParams, graphName: string) => {
      try {
        await copyGraphMutation.mutateAsync({
            copyName:graphName,
            destinationRegionId:params.destination.regionId,
            sourceGraphId:params.source.graphId
        });
        closeModal(); // ✅ Close modal on success
        // Optional: Show success toast
      } catch (error) {
        console.error('Failed to paste region:', error);
        // ❌ Don't close modal on error
        throw error;
      }
    },
  };
}
