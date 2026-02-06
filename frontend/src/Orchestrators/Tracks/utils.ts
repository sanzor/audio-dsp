


import { useRegionSetStore } from "@/Stores/RegionSetStore";
import type { TrackMeta } from "@/Domain/Track/TrackMeta";
import type { NormalizedTrackMeta } from "@/Domain/Track/NormalizedTrackMeta";
import { cascadeDeleteRegionSet, normalizeRegionSetWithCascade } from "../RegionSets/utils";
import { useTrackStore } from "@/Stores/TrackStore";


/**
 * Cascade normalization: Updates all child stores and returns the normalized parent
 */
export const normalizeTrackWithCascade = (
  trackApi: TrackMeta
): NormalizedTrackMeta => {
  const regionSetsIds: string[] = [];
  const addRegionSet = useRegionSetStore.getState().addRegionSet;

  // Cascade to children: normalize each region and update RegionStore
  for (const regionSet of trackApi.regionSets) {
    // normalizeRegion handles its ow n cascade (Graph -> Nodes/Edges)
    const normalizedRegionSet = normalizeRegionSetWithCascade(regionSet);
    
    // Update the child store
    addRegionSet(normalizedRegionSet);
    regionSetsIds.push(regionSet.id);
  }

  // Return the normalized parent with child IDs
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { regionSets, ...rest } = trackApi;
  return {
    ...rest,
    region_sets_ids: regionSetsIds,

  };
}

export const cascadeDeleteTrack = (trackId: string): void => {
  // 1. Get store accessors (using .getState() because we're outside React)
  const getTrack = useTrackStore.getState().getTrack;
  const removeTrack = useTrackStore.getState().removeTrack;

  // 2. Get the entity to find its children
  const trackToDelete = getTrack(trackId);
  if (!trackToDelete) return; // Already deleted or doesn't exist

  // 3. CASCADE DOWN: Delete all child region sets first
  //    Each child deletion will cascade further (Region Set -> Region → Graph → Nodes/Edges)
  for (const regionId of trackToDelete.region_sets_ids) {
    cascadeDeleteRegionSet(regionId); // This recursively deletes Graph, Nodes, Edges
  }

  // 4. DELETE PARENT: Finally, remove this RegionSet from its store
  removeTrack(trackId);
};