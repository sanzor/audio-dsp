import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useTrackStore } from '@/Stores/TrackStore';
import type { CopyTrackParams } from '@/Dtos/Tracks/CopyTrackParams';
import type { CopyTrackResult } from '@/Dtos/Tracks/CopyTrackResult';
import type { RemoveTrackParams } from '@/Dtos/Tracks/RemoveTrackParams';
import type { RemoveTrackResult } from '@/Dtos/Tracks/RemoveTrackResult';
import type { NormalizedTrackMeta } from '@/Domain/Track/NormalizedTrackMeta';
import { apiAddTrack as apiCreateTrack, apiCopyTrack, apiRemoveTrack } from '@/Services/TracksService';
import { normalizeTrackWithCascade, cascadeDeleteTrack } from './utils';
import type { CreateTrackParams } from '@/Dtos/Tracks/AddTrackParams';
import type { CreateTrackResult } from '@/Dtos/Tracks/AddTrackResult';

/**
 * Copy Track Mutation
 * - Calls API to copy the entire tree
 * - Cascades normalization through all child stores
 * - Invalidates relevant queries
 */

export const useCreateTrack = () => {
  const queryClient = useQueryClient();
  const addTrack = useTrackStore.getState().addTrack;
  

  return useMutation<CreateTrackResult, Error, CreateTrackParams>({
    mutationFn: (createParams: CreateTrackParams) => apiCreateTrack(createParams),
    onSuccess: (data) => {
      addTrack({trackId:data.track_id,trackInfo:data.track_info,region_sets_ids:[]})

      queryClient.invalidateQueries({ queryKey: ['track', data.track_id] });
      queryClient.setQueryData(['track', data.track_id],data.track_id);
    },
    onError: (error: Error) => {
      console.error('Failed to create region set:', error);
    },
  });
};

export const useCopyTrack = () => {
  const queryClient = useQueryClient();
  const addTrack = useTrackStore.getState().addTrack;

  return useMutation<CopyTrackResult, Error, CopyTrackParams>({
    mutationFn: (copyParams: CopyTrackParams) => apiCopyTrack(copyParams),
    onSuccess: (data: CopyTrackResult) => {
      // 1. Normalize the entire tree (cascades to RegionSet, Region, Graph, Nodes, Edges stores)
      const normalizedCopy = normalizeTrackWithCascade(data.track);

      // 2. Add the new copy to TrackStore
      addTrack(normalizedCopy);

      // 3. Invalidate queries to refresh UI
      queryClient.invalidateQueries({ queryKey: ['tracks'] });

      // Optionally set the new data directly to avoid refetch
      queryClient.setQueryData(['track', normalizedCopy.trackId], normalizedCopy);
    },

    onError: (error: Error) => {
      console.error('Failed to copy track:', error);
    },
  });
};

/**
 * Delete Track Mutation
 * - Calls API to delete
 * - Cascades deletion through all child stores
 * - Invalidates relevant queries
 */
export const useDeleteTrack = () => {
  const queryClient = useQueryClient();
  const getTrack = useTrackStore.getState().getTrack;

  return useMutation<
    RemoveTrackResult,
    Error,
    RemoveTrackParams,
    { previousTrack?: NormalizedTrackMeta }
  >({
    mutationFn: (removeParams: RemoveTrackParams) => apiRemoveTrack(removeParams),

    // Optimistic update: remove from store immediately
    onMutate: async (removeParams: RemoveTrackParams) => {
      const trackId = removeParams.trackId;

      // Cancel outgoing refetches
      await queryClient.cancelQueries({ queryKey: ['track', trackId] });

      // Snapshot the current value for rollback
      const previousTrack = getTrack(trackId);

      // Optimistically remove from all stores
      if (previousTrack) {
        cascadeDeleteTrack(trackId);
      }

      return { previousTrack };
    },

    onSuccess: (
      _data: RemoveTrackResult,
      removeParams: RemoveTrackParams,
      context: { previousTrack?: NormalizedTrackMeta } | undefined
    ) => {
      const trackId = removeParams.trackId;

      // Invalidate parent project's tracks list
      if (context?.previousTrack) {
        queryClient.invalidateQueries({ queryKey: ['tracks'] });
      }

      // Remove the individual query
      queryClient.removeQueries({ queryKey: ['track', trackId] });
    },

    onError: (
      error: Error,
      removeParams: RemoveTrackParams,
      context: { previousTrack?: NormalizedTrackMeta } | undefined
    ) => {
      console.error('Failed to delete track:', error);

      if (context?.previousTrack) {
        queryClient.invalidateQueries({ queryKey: ['track', removeParams.trackId] });
        queryClient.invalidateQueries({ queryKey: ['tracks'] });
      }
    },
  });
};

/**
 * Rename Track Mutation (simpler example)
 */
export const useRenameTrack = () => {
  const queryClient = useQueryClient();
  const updateTrack = useTrackStore.getState().updateTrack;
  const getTrack = useTrackStore.getState().getTrack;

  return useMutation<
    void,
    Error,
    { trackId: string; newName: string },
    { previousTrack?: NormalizedTrackMeta }
  >({
    mutationFn: async ({ trackId, newName }) => {
      const response = await fetch(`/api/tracks/${trackId}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: newName }),
      });
      if (!response.ok) throw new Error('Failed to rename track');
    },

    onMutate: async ({ trackId, newName }) => {
      const previousTrack = getTrack(trackId);
      if (previousTrack) {
        updateTrack(trackId, {
          trackInfo: {
            ...previousTrack.trackInfo,
            name: newName,
          },
        });
      }

      return { previousTrack };
    },

    onError: (
      _error: Error,
      { trackId }: { trackId: string; newName: string },
      context: { previousTrack?: NormalizedTrackMeta } | undefined
    ) => {
      if (context?.previousTrack) {
        updateTrack(trackId, context.previousTrack);
      }
    },

    onSettled: (
      _data: void | undefined,
      _error: Error | null,
      { trackId }: { trackId: string; newName: string }
    ) => {
      queryClient.invalidateQueries({ queryKey: ['track', trackId] });
    },
  });
};

/**
 * Facade hook that bundles all track mutations
 */
export const useTrackMutations = () => ({
  create: useCreateTrack(),
  copy: useCopyTrack(),
  remove: useDeleteTrack(),
  rename: useRenameTrack(),
});
