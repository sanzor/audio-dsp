import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useTrackStore } from '@/Stores/TrackStore';
import { useRegionSetStore } from '@/Stores/RegionSetStore';
import {
  apiAddTrack as apiCreateTrack,
  apiCopyTrack,
  apiRemoveTrack,
  type CreateTrackParams,
  type CreateTrackResult,
  type CopyTrackParams,
  type CopyTrackResult,
  type RemoveTrackParams,
  type RemoveTrackResult,
} from '@/Services/TracksService';
import { normalizeRegionSetWithCascade, cascadeDeleteRegionSet } from '@/hooks/region-sets/mutations';
import { QUERY_KEYS } from '@/constants/queryKeys';
import type { NormalizedTrackMeta } from '@/domain/Track/NormalizedTrackMeta';
import type { TrackMeta } from '@/domain/Track/TrackMeta';

// ─── Normalize / cascade helpers ────────────────────────────────────────────

export const normalizeTrackWithCascade = (trackApi: TrackMeta): NormalizedTrackMeta => {
  const addRegionSet = useRegionSetStore.getState().addRegionSet;
  const regionSetsIds: number[] = [];

  for (const regionSet of trackApi.regionSets) {
    const normalized = normalizeRegionSetWithCascade(regionSet);
    addRegionSet(normalized);
    regionSetsIds.push(regionSet.id);
  }

  const { regionSets: _regionSets, ...rest } = trackApi;
  return { ...rest, region_sets_ids: regionSetsIds };
};

export const cascadeDeleteTrack = (trackId: number): void => {
  const { getTrack, removeTrack } = useTrackStore.getState();
  const track = getTrack(trackId);
  if (!track) return;

  for (const regionSetId of track.region_sets_ids) {
    cascadeDeleteRegionSet(regionSetId);
  }
  removeTrack(trackId);
};

// ─── Mutations ───────────────────────────────────────────────────────────────

export const useCreateTrack = () => {
  const queryClient = useQueryClient();
  const addTrack = useTrackStore.getState().addTrack;

  return useMutation<CreateTrackResult, Error, CreateTrackParams>({
    mutationFn: (params) => apiCreateTrack(params),
    onSuccess: (data) => {
      addTrack({ trackId: data.track_id, trackInfo: data.track_info, region_sets_ids: [] });
      queryClient.invalidateQueries({ queryKey: QUERY_KEYS.tracks.byId(data.track_id) });
    },
    onError: (error) => {
      console.error('Failed to create track:', error);
    },
  });
};

export const useCopyTrack = () => {
  const queryClient = useQueryClient();
  const addTrack = useTrackStore.getState().addTrack;

  return useMutation<CopyTrackResult, Error, CopyTrackParams>({
    mutationFn: (params) => apiCopyTrack(params),
    onSuccess: (data) => {
      const normalized = normalizeTrackWithCascade(data.track);
      addTrack(normalized);
      queryClient.invalidateQueries({ queryKey: QUERY_KEYS.tracks.all() });
      queryClient.setQueryData(QUERY_KEYS.tracks.byId(normalized.trackId), normalized);
    },
    onError: (error) => {
      console.error('Failed to copy track:', error);
    },
  });
};

export const useDeleteTrack = () => {
  const queryClient = useQueryClient();
  const getTrack = useTrackStore.getState().getTrack;

  return useMutation<RemoveTrackResult, Error, RemoveTrackParams, { previousTrack?: NormalizedTrackMeta }>({
    mutationFn: (params) => apiRemoveTrack(params),
    onMutate: async (params) => {
      await queryClient.cancelQueries({ queryKey: QUERY_KEYS.tracks.byId(params.trackId) });
      const previousTrack = getTrack(params.trackId);
      if (previousTrack) cascadeDeleteTrack(params.trackId);
      return { previousTrack };
    },
    onSuccess: (_data, params, ctx) => {
      if (ctx?.previousTrack) {
        queryClient.invalidateQueries({ queryKey: QUERY_KEYS.tracks.all() });
      }
      queryClient.removeQueries({ queryKey: QUERY_KEYS.tracks.byId(params.trackId) });
    },
    onError: (_error, params, ctx) => {
      console.error('Failed to delete track');
      if (ctx?.previousTrack) {
        queryClient.invalidateQueries({ queryKey: QUERY_KEYS.tracks.byId(params.trackId) });
        queryClient.invalidateQueries({ queryKey: QUERY_KEYS.tracks.all() });
      }
    },
  });
};

export const useRenameTrack = () => {
  const queryClient = useQueryClient();
  const updateTrack = useTrackStore.getState().updateTrack;
  const getTrack = useTrackStore.getState().getTrack;

  return useMutation<void, Error, { trackId: number; newName: string }, { previousTrack?: NormalizedTrackMeta }>({
    mutationFn: async ({ trackId, newName }) => {
      const res = await fetch(`/api/tracks/${trackId}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: newName }),
      });
      if (!res.ok) throw new Error('Failed to rename track');
    },
    onMutate: ({ trackId, newName }) => {
      const previousTrack = getTrack(trackId);
      if (previousTrack) {
        updateTrack(trackId, { trackInfo: { ...previousTrack.trackInfo, name: newName } });
      }
      return { previousTrack };
    },
    onError: (_error, { trackId }, ctx) => {
      if (ctx?.previousTrack) updateTrack(trackId, ctx.previousTrack);
    },
    onSettled: (_data, _error, { trackId }) => {
      queryClient.invalidateQueries({ queryKey: QUERY_KEYS.tracks.byId(trackId) });
    },
  });
};

export const useTrackMutations = () => ({
  create: useCreateTrack(),
  copy: useCopyTrack(),
  remove: useDeleteTrack(),
  rename: useRenameTrack(),
});
