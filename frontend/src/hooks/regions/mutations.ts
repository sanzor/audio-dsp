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
import type { NormalizedTrackRegion } from '@/Domain/Region/NormalizedTrackRegion';
import type { TrackRegion } from '@/Domain/Region/TrackRegion';

// ─── Normalize / cascade helpers ────────────────────────────────────────────

export const normalizeRegionWithCascade = (regionApi: TrackRegion): NormalizedTrackRegion => {
  const { graph, ...rest } = regionApi;
  if (graph) useGraphStore.getState().addGraph(graph);
  return { ...rest, graphId: graph ? graph.id : null };
};

export const cascadeDeleteRegion = (regionId: string): void => {
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
  const addRegion = useRegionStore.getState().addRegion;
  const attachRegion = useRegionSetStore.getState().attachRegion;

  return useMutation<CreateRegionResult, Error, CreateRegionParams>({
    mutationFn: (params) => apiAddRegion(params),
    onSuccess: (data) => {
      const normalized = normalizeRegionWithCascade(data.region);
      addRegion(normalized);
      attachRegion(data.region.regionSetId, data.region.regionId);
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

  return useMutation<RemoveRegionResult, Error, RemoveRegionParams, { previousRegion?: NormalizedTrackRegion }>({
    mutationFn: (params) => apiRemoveRegion(params),
    onMutate: (params) => {
      const prev = getRegion(params.regionId);
      if (prev) cascadeDeleteRegion(params.regionId);
      return { previousRegion: prev };
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
