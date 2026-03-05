import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useRegionSetStore } from '@/Stores/RegionSetStore';
import { useRegionStore } from '@/Stores/RegionStore';
import { useTrackStore } from '@/Stores/TrackStore';
import {
  apiCopyRegionSet,
  apiCreateRegionSet,
  apiRemoveRegionSet,
  type CopyRegionSetParams,
  type CreateRegionSetParams,
  type CreateRegionSetResult,
  type RemoveRegionSetParams,
} from '@/Services/RegionSetsService';
import { normalizeRegionWithCascade, cascadeDeleteRegion } from '@/hooks/regions/mutations';
import { QUERY_KEYS } from '@/constants/queryKeys';
import type { NormalizedTrackRegionSet } from '@/domain/RegionSet/NormalizedTrackRegionSet';
import type { NormalizedTrackRegion } from '@/domain/Region/NormalizedTrackRegion';
import type { TrackRegionSet } from '@/domain/RegionSet/TrackRegionSet';

// ─── Normalize / cascade helpers ────────────────────────────────────────────

export const normalizeRegionSetWithCascade = (regionSetApi: TrackRegionSet): NormalizedTrackRegionSet => {
  const { addRegion, removeRegionsBySetId } = useRegionStore.getState();
  const regionIds: string[] = [];

  removeRegionsBySetId(regionSetApi.id);

  for (const regionApi of regionSetApi.regions) {
    const normalized = normalizeRegionWithCascade(regionApi);
    addRegion(normalized);
    regionIds.push(normalized.regionId);
  }

  const { regions: _regions, ...rest } = regionSetApi;
  return { ...rest, region_ids: regionIds };
};

export const cascadeDeleteRegionSet = (setId: string): void => {
  const { getRegionSet, removeRegionSet } = useRegionSetStore.getState();
  const { detachRegionSet } = useTrackStore.getState();

  const regionSet = getRegionSet(setId);
  if (!regionSet) return;

  for (const regionId of regionSet.region_ids) {
    cascadeDeleteRegion(regionId);
  }

  detachRegionSet(regionSet.trackId, setId);
  removeRegionSet(setId);
};

// ─── Mutations ───────────────────────────────────────────────────────────────

export const useCreateRegionSet = () => {
  const queryClient = useQueryClient();
  const addRegionSet = useRegionSetStore.getState().addRegionSet;
  const attachRegionSet = useTrackStore.getState().attachRegionSet;

  return useMutation<CreateRegionSetResult, Error, CreateRegionSetParams>({
    mutationFn: (params) => apiCreateRegionSet(params),
    onSuccess: (data) => {
      const normalized = normalizeRegionSetWithCascade(data.region_set);
      addRegionSet(normalized);
      attachRegionSet(normalized.trackId, normalized.id);
      queryClient.invalidateQueries({ queryKey: QUERY_KEYS.regionSets.byTrack(normalized.trackId) });
      queryClient.setQueryData(QUERY_KEYS.regionSets.byId(normalized.id), normalized);
    },
    onError: (error) => {
      console.error('Failed to create region set:', error);
    },
  });
};

export const useCopyRegionSet = () => {
  const queryClient = useQueryClient();
  const addRegionSet = useRegionSetStore.getState().addRegionSet;
  const attachRegionSet = useTrackStore.getState().attachRegionSet;

  return useMutation<CreateRegionSetResult, Error, CopyRegionSetParams>({
    mutationFn: (params) => apiCopyRegionSet(params),
    onSuccess: (data) => {
      const normalized = normalizeRegionSetWithCascade(data.region_set);
      addRegionSet(normalized);
      attachRegionSet(normalized.trackId, normalized.id);
      queryClient.invalidateQueries({ queryKey: QUERY_KEYS.regionSets.byTrack(normalized.trackId) });
      queryClient.setQueryData(QUERY_KEYS.regionSets.byId(normalized.id), normalized);
    },
    onError: (error) => {
      console.error('Failed to copy region set:', error);
    },
  });
};

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
    mutationFn: (params) => apiRemoveRegionSet(params),
    onMutate: async (params) => {
      await queryClient.cancelQueries({ queryKey: QUERY_KEYS.regionSets.byId(params.regionSetId) });
      const previousSet = getRegionSet(params.regionSetId);
      const previousRegions = previousSet
        ? previousSet.region_ids.map((id) => getRegion(id)).filter((r): r is NormalizedTrackRegion => Boolean(r))
        : [];
      if (previousSet) cascadeDeleteRegionSet(params.regionSetId);
      return { previousSet, previousRegions };
    },
    onSuccess: (_data, params, ctx) => {
      if (ctx?.previousSet) {
        queryClient.invalidateQueries({ queryKey: QUERY_KEYS.regionSets.byTrack(ctx.previousSet.trackId) });
      }
      queryClient.removeQueries({ queryKey: QUERY_KEYS.regionSets.byId(params.regionSetId) });
    },
    onError: (_error, params, ctx) => {
      console.error('Failed to delete region set');
      if (ctx?.previousSet) {
        const { addRegionSet } = useRegionSetStore.getState();
        const { addRegion } = useRegionStore.getState();
        const { attachRegionSet } = useTrackStore.getState();
        addRegionSet(ctx.previousSet);
        attachRegionSet(ctx.previousSet.trackId, ctx.previousSet.id);
        ctx.previousRegions.forEach((r) => addRegion(r));
      }
      queryClient.invalidateQueries({ queryKey: QUERY_KEYS.regionSets.byId(params.regionSetId) });
    },
  });
};

export const useRenameRegionSet = () => {
  const queryClient = useQueryClient();
  const updateRegionSet = useRegionSetStore.getState().updateRegionSet;

  return useMutation<void, Error, { setId: string; newName: string }>({
    mutationFn: async ({ setId, newName }) => {
      const res = await fetch(`/api/region-sets/${setId}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: newName }),
      });
      if (!res.ok) throw new Error('Failed to rename');
    },
    onMutate: ({ setId, newName }) => {
      updateRegionSet(setId, { name: newName });
    },
    onSuccess: (_, { setId }) => {
      queryClient.invalidateQueries({ queryKey: QUERY_KEYS.regionSets.byId(setId) });
    },
  });
};

export const useRegionSetMutations = () => ({
  create: useCreateRegionSet(),
  copy: useCopyRegionSet(),
  remove: useDeleteRegionSet(),
  rename: useRenameRegionSet(),
});
