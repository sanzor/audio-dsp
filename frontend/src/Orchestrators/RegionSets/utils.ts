import type { NormalizedTrackRegionSet } from "@/Domain/RegionSet/NormalizedTrackRegionSet";
import type { TrackRegionSet } from "@/Domain/RegionSet/TrackRegionSet";
import { cascadeDeleteRegion, normalizeRegionWithCascade } from "../Regions/utils";
import { useRegionStore } from "@/Stores/RegionStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useTrackStore } from "@/Stores/TrackStore";

export const normalizeRegionSetWithCascade = (
  regionSetApi: TrackRegionSet
): NormalizedTrackRegionSet => {
  const regionIds: string[] = [];
  const addRegion = useRegionStore.getState().addRegion;
  const removeRegionsBySetId = useRegionStore.getState().removeRegionsBySetId;

  removeRegionsBySetId(regionSetApi.id);

  for (const regionApi of regionSetApi.regions) {
    const normalizedRegion = normalizeRegionWithCascade(regionApi);
    addRegion(normalizedRegion);
    regionIds.push(normalizedRegion.regionId);
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { regions, ...rest } = regionSetApi;
  return {
    ...rest,
    region_ids: regionIds,
  };
};

export const normalizeRegionSet = normalizeRegionSetWithCascade;

export const cascadeDeleteRegionSet = (setId: string): void => {
  const getRegionSet = useRegionSetStore.getState().getRegionSet;
  const removeRegionSet = useRegionSetStore.getState().removeRegionSet;
  const detachRegionSet = useTrackStore.getState().detachRegionSet;

  const regionSet = getRegionSet(setId);
  if (!regionSet) return;

  for (const regionId of regionSet.region_ids) {
    cascadeDeleteRegion(regionId);
  }

  detachRegionSet(regionSet.trackId, setId);
  removeRegionSet(setId);
};
