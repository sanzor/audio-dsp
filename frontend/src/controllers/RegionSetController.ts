// hooks/useRegionSetController.ts
import { useUIStore } from "@/Stores/UIStore";
import type { PasteRegionParams, PasteRegionSetParams } from "@/Stores/PasteParams";
import { useCopyRegion, useCreateRegion, } from "@/Orchestrators/Regions/useRegionMutations";

import { useRegionStore } from "@/Stores/RegionStore";
import { useTrackStore } from "@/Stores/TrackStore";

import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useCopyRegionSet, useDeleteRegionSet, useRenameRegionSet } from "@/Orchestrators/RegionSets/useRegionSetsMutations";
import type { CreateRegionParams } from "@/Dtos/Regions/CreateRegionParams";


export function useRegionSetController() {
  // Zustand selectors
  const copyToClipboard = useUIStore(state => state.copyToClipboard);
  const closeModal = useUIStore(state => state.closeModal);
  const openModal = useUIStore(state => state.openModal);
  const closeContextMenu = useUIStore(state => state.closeContextMenu);
  
  // Data and mutations
  const regionSetsMap = useRegionSetStore(x=>x.regionSets);
  const regionsMap = useRegionStore(x=>x.regions);
  const trackMap = useTrackStore(x=>x.tracks);
  const createRegionMutation = useCreateRegion();
  const useCopyRegionMutation = useCopyRegion();
  const copyRegionSetMutation = useCopyRegionSet();
  const deleteRegionSetMutation = useDeleteRegionSet();
  const renameRegionSetMutation = useRenameRegionSet();



  return {
    // ============================================
    // CREATE REGION
    // ============================================
    handleCreateRegion: (regionSetId:string,start?:number,stop?:number) => {
      const regionSet =regionSetsMap.get(regionSetId);
      if (!regionSet) {
        console.error('RegionSet  not found:', { regionSetId });
        return;
      }
      
      openModal({ type: 'createRegion', regionSetId ,startTime:start,endTime:stop});
      closeContextMenu(); // ✅ Close context menu when opening modal
    },



    handleSubmitCreateRegion: async (params: CreateRegionParams) => {
      try {
        await createRegionMutation.mutateAsync(params);
        closeModal(); // ✅ Close modal on success
        // Optional: Show success toast
      } catch (error) {
        console.error('Failed to create region:', error);
        // ❌ Don't close modal on error - let user fix/retry
        // Optional: Show error toast or inline error
        throw error; // Re-throw so modal can handle it
      }
    },

    // ============================================
    // DETAILS REGION SET
    // ============================================
    handleDetailsRegionSet: (regionSetId:string) => {
      const region =regionSetsMap.get(regionSetId);
      if (!region) {
        console.error('RegionSet not found:', { regionId: regionSetId });
        return;
      }
      
      openModal({ type: 'detailsRegionSet', regionSetId });
      closeContextMenu(); // ✅ Close context menu when opening modal
    },

    // ============================================
    // RENAME REGION SET
    // ============================================
    handleEditRegionSet: (regionSetId:string) => {
      const region =regionSetsMap.get(regionSetId);
      if (!region) {
        console.error('RegionSet  not found:', { regionSetId: regionSetId });
        return;
      }
      
      openModal({ type: "renameRegionSet", regionSetId: regionSetId });
      closeContextMenu(); // ✅ Close context menu when opening modal
    },

    handleSubmitRenameRegionSet: async (regionSetId:string, newName: string) => {
      try {
        await renameRegionSetMutation.mutateAsync({ setId:regionSetId,newName:newName });
        closeModal(); // ✅ Close modal on success
        // Optional: Show success toast
      } catch (error) {
        console.error('Failed to rename region set:', error);
        // ❌ Don't close modal on error
        throw error;
      }
    },

    // ============================================
    // DELETE REGION 
    // ============================================
    handleDeleteRegionSet: async (regionSetId:string) => {
      try {
        await deleteRegionSetMutation.mutateAsync({ regionSetId:regionSetId });
        closeContextMenu(); // ✅ Close context menu after successful action
        // Optional: Show success toast
      } catch (error) {
        console.error('Failed to delete region set:', error);
        closeContextMenu(); // ✅ Still close context menu on error
        // Optional: Show error toast
        throw error;
      }
    },

    // ============================================
    // COPY REGION SET
    // ============================================
    handleCopyRegionSet: (regionSetId:string) => {
      const regionSet =regionSetsMap.get(regionSetId);
      if (!regionSet) {
        console.error('Region set not found:', { regionSetId: regionSetId });
        return;
      }
      
      copyToClipboard({ type: "regionSet", regionSetId: regionSetId});
      closeContextMenu(); // ✅ Close context menu after action
      
      // Optional: Show success toast
      console.log("Copied region set:", regionSet.name);
    },

    // ============================================
    // PASTE REGION
    // ============================================
    handlePasteRegion(destRegionSetId:string) {
      // 1. Validate source type
      const clipboard = useUIStore.getState().clipboard;
      if (!clipboard || clipboard.type !== "region") return;

      // 2. Validate destination existence
      const destRegionSet =regionSetsMap.get(destRegionSetId);
      if (!destRegionSet) {
        console.error('Region set  not found:', { destRegionSetId: destRegionSetId });
        return;
      }
      // 2. Validate destination existence
      

      // 3. (Optional but recommended) Validate source existence
     

      const sourceRegion = regionsMap.get(clipboard.regionId);
      if (!sourceRegion) return;

      // 4. Everything valid → open modal
      openModal({
        type: "pasteRegion",
        params: {
          source: {
            regionId: sourceRegion.regionId
          },
          destination: {
            regionSetId: destRegionSetId
          }
        }
      });

      closeContextMenu();
    },

    handleSubmitPasteRegion: async (params: PasteRegionParams, regionName: string) => {
      try {
        await useCopyRegionMutation.mutateAsync({
            copyName:regionName,
            destinationRegionSetId:params.destination.regionSetId,
            sourceRegionId:params.source.regionId
        });
        closeModal(); // ✅ Close modal on success
        // Optional: Show success toast
      } catch (error) {
        console.error('Failed to paste region:', error);
        // ❌ Don't close modal on error
        throw error;
      }
    },

    // ============================================
    // PASTE REGION SET
    // ============================================
    handlePasteRegionSet(destTrackId: string) {
      // 1. Validate source type
      const clipboard = useUIStore.getState().clipboard;
      if (!clipboard || clipboard.type !== "regionSet") return;

      // 2. Validate destination existence
      const destTrack = trackMap.get(destTrackId);
      if (!destTrack) {
        console.error('Track not found:', { destTrackId });
        return;
      }

      // 3. Validate source existence
      const sourceRegionSet = regionSetsMap.get(clipboard.regionSetId);
      if (!sourceRegionSet) {
        console.error('Source region set not found:', { regionSetId: clipboard.regionSetId });
        return;
      }

      // 4. Everything valid → open modal
      openModal({
        type: "pasteRegionSet",
        params: {
          source: {
            regionSetId: clipboard.regionSetId
          },
          destination: {
            trackId: destTrackId
          }
        }
      });

      closeContextMenu();
    },

    handleSubmitPasteRegionSet: async (params: PasteRegionSetParams, regionSetName: string) => {
      try {
        await copyRegionSetMutation.mutateAsync({
            destTrackId: params.destination.trackId,
            sourceRegionSetId: params.source.regionSetId,
            copy_region_set_name: regionSetName
        });
        closeModal(); // ✅ Close modal on success
        // Optional: Show success toast
      } catch (error) {
        console.error('Failed to paste region set:', error);
        // ❌ Don't close modal on error
        throw error;
      }
    },
  };
}