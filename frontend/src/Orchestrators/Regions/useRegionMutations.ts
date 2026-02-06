import { useMutation } from '@tanstack/react-query';
import { useRegionSetStore } from '@/Stores/RegionSetStore';
import { useRegionStore } from '@/Stores/RegionStore';
import { cascadeDeleteRegion, normalizeRegionWithCascade } from './utils'
import { apiAddRegion, apiCopyRegion, apiEditRegion, apiRemoveRegion } from '@/Services/RegionsService';
import type { CopyRegionParams } from '@/Dtos/Regions/CopyRegionParams';
import type { CopyRegionResult } from '@/Dtos/Regions/CopyRegionResult';
import type { CreateRegionParams } from '@/Dtos/Regions/CreateRegionParams';
import type { CreateRegionResult } from '@/Dtos/Regions/CreateRegionResult';
import type { EditRegionParams } from '@/Dtos/Regions/EditRegionParams';
import type { EditRegionResult } from '@/Dtos/Regions/EditRegionResult';
import type { RemoveRegionParams } from '@/Dtos/Regions/RemoveRegionParams';
import type { RemoveRegionResult } from '@/Dtos/Regions/RemoveRegionResult';
import type { NormalizedTrackRegion } from '@/Domain/Region/NormalizedTrackRegion';



export const useCreateRegion = () => {
  const addRegion = useRegionStore.getState().addRegion;
  const attachRegion = useRegionSetStore.getState().attachRegion;
  return useMutation<CreateRegionResult, Error, CreateRegionParams>({
    mutationFn: (params) => apiAddRegion(params),
    onSuccess: (data) => {
      //update region set , attach region
      const region=data.region;
      const normalizedRegion=normalizeRegionWithCascade(region);
      addRegion(normalizedRegion);
      attachRegion(region.regionSetId,region.regionId);
    },
    onError: (error) => {
      console.error('Failed to create region', error);
    },
  });
};

export const useCopyRegion = () => {
  const addRegion = useRegionStore.getState().addRegion;
  const attachRegion = useRegionSetStore.getState().attachRegion;
  return useMutation<CopyRegionResult, Error, CopyRegionParams>({
    mutationFn: (params) => apiCopyRegion(params),
    onSuccess: (data) => {
      const region=data.region;
      const normalizedRegion=normalizeRegionWithCascade(region);
      addRegion(normalizedRegion);
      attachRegion(region.regionSetId,region.regionId);
    },
    onError: (error) => {
      console.error('Failed to copy region', error);
    },
  });
};

export const useEditRegion = () => {
  const editRegion = useRegionStore.getState().updateRegion;
  return useMutation<EditRegionResult, Error, EditRegionParams>({
    mutationFn: (params) => apiEditRegion(params),
    onSuccess: (data) => {
       const region=data.region;
       const normalizedRegion=normalizeRegionWithCascade(region);
       editRegion(region.regionId,normalizedRegion);
    },
    onError: (error) => {
      console.error('Failed to edit region', error);
    },
  });
};

export const useDeleteRegion = () => {
  const getRegion = useRegionStore.getState().getRegion;

  return useMutation<
    RemoveRegionResult,
    Error,
    RemoveRegionParams,
    { previousRegion?: NormalizedTrackRegion }
  >({
    mutationFn: (params) => apiRemoveRegion(params),
    onMutate: (params) => {
      const prev = getRegion(params.regionId);

      if (prev) {
        cascadeDeleteRegion(params.regionId);
      }

      return { previousRegion: prev };
    },
    onError: (
      _error: Error,
      _params: RemoveRegionParams,
      ctx: { previousRegion?: NormalizedTrackRegion } | undefined
    ) => {
      if (ctx?.previousRegion) {
        useRegionStore.getState().addRegion(ctx.previousRegion);
        useRegionSetStore.getState().attachRegion(
          ctx.previousRegion.regionSetId,
          ctx.previousRegion.regionId
        );
      }
    },
  });
};

/**
 * Facade hook that bundles all region mutations
 */
export const useRegionMutations = () => ({
  create: useCreateRegion(),
  copy: useCopyRegion(),
  edit: useEditRegion(),
  remove: useDeleteRegion(),
});
