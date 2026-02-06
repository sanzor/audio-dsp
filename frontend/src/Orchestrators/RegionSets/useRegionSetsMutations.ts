// /src/Hooks/RegionSet/useRegionSetMutations.ts

import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useRegionSetStore } from '@/Stores/RegionSetStore';
import { useTrackStore } from '@/Stores/TrackStore';
import { useRegionStore } from '@/Stores/RegionStore';

import type { CopyRegionSetParams } from '@/Dtos/RegionSets/CoyRegionSetParams';
import type { CreateRegionSetParams } from '@/Dtos/RegionSets/CreateRegionSetParams';
import type { RemoveRegionSetParams } from '@/Dtos/RegionSets/RemoveRegionSetParams';
import { apiCopyRegionSet, apiCreateRegionSet, apiRemoveRegionSet } from '@/Services/RegionSetsService';
import { cascadeDeleteRegionSet, normalizeRegionSetWithCascade } from './utils';
import type { CreateRegionSetResult } from '@/Dtos/RegionSets/CreateRegionSetResult';
import type { NormalizedTrackRegionSet } from '@/Domain/RegionSet/NormalizedTrackRegionSet';
import type { NormalizedTrackRegion } from '@/Domain/Region/NormalizedTrackRegion';

/**
 * Copy RegionSet Mutation
 * - Calls API to copy the entire tree
 * - Cascades normalization through all child stores
 * - Invalidates relevant queries
 */
export const useCopyRegionSet = () => {
  const queryClient = useQueryClient();
  const addRegionSet = useRegionSetStore.getState().addRegionSet;
  const attachRegionSet = useTrackStore.getState().attachRegionSet;

  return useMutation<CreateRegionSetResult, Error, CopyRegionSetParams>({
    mutationFn: (copySetParams: CopyRegionSetParams) => apiCopyRegionSet(copySetParams),
    
    onSuccess: (data: CreateRegionSetResult) => {
      // 1. Normalize the entire tree (cascades to Region, Graph, Nodes, Edges stores)
      const normalizedCopy = normalizeRegionSetWithCascade(data.region_set);
      
      // 2. Add the new copy to RegionSetStore
      addRegionSet(normalizedCopy);
      attachRegionSet(normalizedCopy.trackId, normalizedCopy.id);
      
      // 3. Invalidate queries to refresh UI
      // Invalidate the parent track's region sets list
      const trackId = normalizedCopy.trackId;
      queryClient.invalidateQueries({ queryKey: ['regionSets', 'track', trackId] });
      
      // Optionally set the new data directly to avoid refetch
      queryClient.setQueryData(
        ['regionSet', normalizedCopy.id],
        normalizedCopy
      );
    },
    
    onError: (error: Error) => {
      console.error('Failed to copy region set:', error);
      // Could add toast notification here
    },
  });
};

/**
 * Create RegionSet Mutation
 */
export const useCreateRegionSet = () => {
  const queryClient = useQueryClient();
  const addRegionSet = useRegionSetStore.getState().addRegionSet;
  const attachRegionSet = useTrackStore.getState().attachRegionSet;

  return useMutation<CreateRegionSetResult, Error, CreateRegionSetParams>({
    mutationFn: (createParams: CreateRegionSetParams) => apiCreateRegionSet(createParams),
    onSuccess: (data) => {
      const normalized = normalizeRegionSetWithCascade(data.region_set);
      addRegionSet(normalized);
      attachRegionSet(normalized.trackId, normalized.id);

      queryClient.invalidateQueries({ queryKey: ['regionSets', 'track', normalized.trackId] });
      queryClient.setQueryData(['regionSet', normalized.id], normalized);
    },
    onError: (error: Error) => {
      console.error('Failed to create region set:', error);
    },
  });
};

/**
 * Delete RegionSet Mutation
 * - Calls API to delete
 * - Cascades deletion through all child stores
 * - Invalidates relevant queries
 */
export const useDeleteRegionSet = () => {
  const queryClient = useQueryClient();
  const getRegionSet = useRegionSetStore.getState().getRegionSet;
  const getRegion = useRegionStore.getState().getRegion;

  return useMutation<
    void,
    Error,
    RemoveRegionSetParams,
    { previousSet?: NormalizedTrackRegionSet; previousRegions: NormalizedTrackRegion[] }
  >({
    mutationFn: (removeSetParams: RemoveRegionSetParams) => apiRemoveRegionSet(removeSetParams),

    // Optimistic update: remove from store immediately
    onMutate: async (removeSetParams: RemoveRegionSetParams) => {
      const setId = removeSetParams.regionSetId;

      // Cancel outgoing refetches
      await queryClient.cancelQueries({ queryKey: ['regionSet', setId] });

      // Snapshot the current value for rollback
      const previousSet = getRegionSet(setId);
      const previousRegions = previousSet
        ? previousSet.region_ids
            .map((regionId) => getRegion(regionId))
            .filter((region): region is NormalizedTrackRegion => Boolean(region))
        : [];

      // Optimistically remove from all stores
      if (previousSet) {
        cascadeDeleteRegionSet(setId);
      }

      return { previousSet, previousRegions };
    },

    onSuccess: (
      _data: void,
      removeSetParams: RemoveRegionSetParams,
      context: { previousSet?: NormalizedTrackRegionSet; previousRegions: NormalizedTrackRegion[] } | undefined
    ) => {
      const setId = removeSetParams.regionSetId;

      // Invalidate parent track's region sets list
      if (context?.previousSet) {
        queryClient.invalidateQueries({
          queryKey: ['regionSets', 'track', context.previousSet.trackId],
        });
      }

      // Remove the individual query
      queryClient.removeQueries({ queryKey: ['regionSet', setId] });
    },

    onError: (
      error: Error,
      removeSetParams: RemoveRegionSetParams,
      context: { previousSet?: NormalizedTrackRegionSet; previousRegions: NormalizedTrackRegion[] } | undefined
    ) => {
      console.error('Failed to delete region set:', error);

      // Rollback: restore the entity if we have the snapshot
      if (context?.previousSet) {
        const addRegionSet = useRegionSetStore.getState().addRegionSet;
        const addRegion = useRegionStore.getState().addRegion;
        const attachRegionSet = useTrackStore.getState().attachRegionSet;

        addRegionSet(context.previousSet);
        attachRegionSet(context.previousSet.trackId, context.previousSet.id);

        context.previousRegions.forEach((region) => addRegion(region));
      }
      queryClient.invalidateQueries({ queryKey: ['regionSet', removeSetParams.regionSetId] });
    },
  });
};

/**
 * Rename RegionSet Mutation (simpler example)
 */
export const useRenameRegionSet = () => {
  const queryClient = useQueryClient();
  const updateRegionSet = useRegionSetStore.getState().updateRegionSet;

  return useMutation<
    void,
    Error,
    { setId: string; newName: string }
  >({
    mutationFn: async ({ setId, newName }) => {
      const response = await fetch(`/api/region-sets/${setId}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: newName }),
      });
      if (!response.ok) throw new Error('Failed to rename');
    },

    onMutate: async ({ setId, newName }) => {
      // Optimistic update
      updateRegionSet(setId, { name: newName });
    },

    onSuccess: (_, { setId }) => {
      queryClient.invalidateQueries({ queryKey: ['regionSet', setId] });
    },
  });
};

/**
 * Facade hook that bundles all region set mutations
 */
export const useRegionSetMutations = () => ({
  create: useCreateRegionSet(),
  copy: useCopyRegionSet(),
  remove: useDeleteRegionSet(),
  rename: useRenameRegionSet(),
});
