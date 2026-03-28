import { useMutation } from '@tanstack/react-query';
import { useRegionSetStore } from '@/Stores/RegionSetStore';
import { useRegionStore } from '@/Stores/RegionStore';
import { useGraphStore } from '@/Stores/GraphStore';
import {
  apiAddRegion,
  apiCopyRegion,
  apiEditRegion,
  apiRemoveRegion,
  type CopyRegionParams,
  type CopyRegionResult,
  type CreateRegionParams,
  type CreateRegionResult,
  type EditRegionParams,
  type EditRegionResult,
  type RemoveRegionParams,
  type RemoveRegionResult,
} from '@/Services/RegionsService';
import { cascadeDeleteGraph } from '@/hooks/graphs/mutations';
import type { NormalizedTrackRegion } from '@/domain/Region/NormalizedTrackRegion';
import type { TrackRegion } from '@/domain/Region/TrackRegion';
import type { TrackRegionSet } from '@/domain/RegionSet/TrackRegionSet';
import type { NormalizedTrackRegionSet } from '@/domain/RegionSet/NormalizedTrackRegionSet';

// ─── Normalize / cascade helpers ────────────────────────────────────────────

export const normalizeRegionWithCascade = (regionApi: TrackRegion): NormalizedTrackRegion => {
  const { graph, ...rest } = regionApi;
  if (graph) useGraphStore.getState().addGraph(graph);
  return { ...rest, graphId: graph ? graph.id : null };
};

export const normalizeRegionSetWithCascade = (regionSetApi: TrackRegionSet): NormalizedTrackRegionSet => {
  const { addRegion, removeRegionsBySetId } = useRegionStore.getState();
  const regionIds: number[] = [];

  removeRegionsBySetId(regionSetApi.id);

  for (const regionApi of regionSetApi.regions) {
    const normalized = normalizeRegionWithCascade(regionApi);
    addRegion(normalized);
    regionIds.push(normalized.regionId);
  }

  const { regions: _regions, ...rest } = regionSetApi;
  return { ...rest, region_ids: regionIds };
};

export const cascadeDeleteRegion = (regionId: number): void => {
  const { getRegion, removeRegion } = useRegionStore.getState();
  const { detachRegion } = useRegionSetStore.getState();

  const region = getRegion(regionId);
  if (!region) return;

  if (region.graphId) cascadeDeleteGraph(region.graphId);
  detachRegion(region.regionSetId, regionId);
  removeRegion(regionId);
};

// ─── Mutations ───────────────────────────────────────────────────────────────

export const useCreateRegion = () => {
  const addRegionSet = useRegionSetStore.getState().addRegionSet;

  return useMutation<CreateRegionResult, Error, CreateRegionParams>({
    mutationFn: (params) => apiAddRegion(params),
    onSuccess: (data) => {
      const normalized = normalizeRegionSetWithCascade(data.regionSet);
      addRegionSet(normalized);
    },
    onError: (error) => {
      console.error('Failed to create region:', error);
    },
  });
};

export const useCopyRegion = () => {
  const addRegion = useRegionStore.getState().addRegion;
  const attachRegion = useRegionSetStore.getState().attachRegion;

  return useMutation<CopyRegionResult, Error, CopyRegionParams>({
    mutationFn: (params) => apiCopyRegion(params),
    onSuccess: (data) => {
      const normalized = normalizeRegionWithCascade(data.region);
      addRegion(normalized);
      attachRegion(data.region.regionSetId, data.region.regionId);
    },
    onError: (error) => {
      console.error('Failed to copy region:', error);
    },
  });
};

export const useEditRegion = () => {
  const editRegion = useRegionStore.getState().updateRegion;

  return useMutation<EditRegionResult, Error, EditRegionParams>({
    mutationFn: (params) => apiEditRegion(params),
    onSuccess: (data) => {
      const normalized = normalizeRegionWithCascade(data.region);
      editRegion(data.region.regionId, normalized);
    },
    onError: (error) => {
      console.error('Failed to edit region:', error);
    },
  });
};

export const useDeleteRegion = () => {
  const getRegion = useRegionStore.getState().getRegion;
  const addRegionSet = useRegionSetStore.getState().addRegionSet;

  return useMutation<
    RemoveRegionResult,
    Error,
    RemoveRegionParams,
    { previousRegion?: NormalizedTrackRegion; previousRegionSetId?: number }
  >({
    mutationFn: (params) => apiRemoveRegion(params),
    onMutate: (params) => {
      const regionId = params.regionId;
      const prev = getRegion(regionId);
      const previousRegionSetId = prev?.regionSetId;
      if (prev) cascadeDeleteRegion(regionId);
      return { previousRegion: prev, previousRegionSetId };
    },
    onSuccess: (data) => {
      const normalized = normalizeRegionSetWithCascade(data.regionSet);
      addRegionSet(normalized);
    },
    onError: (_error, _params, ctx) => {
      if (ctx?.previousRegion) {
        useRegionStore.getState().addRegion(ctx.previousRegion);
        useRegionSetStore.getState().attachRegion(ctx.previousRegion.regionSetId, ctx.previousRegion.regionId);
      }
    },
  });
};

export const useRegionMutations = () => ({
  create: useCreateRegion(),
  copy: useCopyRegion(),
  edit: useEditRegion(),
  remove: useDeleteRegion(),
});
