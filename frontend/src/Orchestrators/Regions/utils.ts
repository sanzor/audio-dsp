import type { NormalizedTrackRegion } from "@/Domain/Region/NormalizedTrackRegion";
import { useRegionStore } from "@/Stores/RegionStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useGraphStore } from "@/Stores/GraphStore";
import { cascadeDeleteGraph, normalizeGraph } from "../Graphs/utils";
import type { TrackRegion } from "@/Domain/Region/TrackRegion";


export const normalizeRegionWithCascade = (
  regionApi: TrackRegion
): NormalizedTrackRegion => {
  const addGraph = useGraphStore.getState().addGraph;
  const { graph, ...rest } = regionApi;

  if (graph) {
    const normalizedGraph = normalizeGraph(graph);
    addGraph(normalizedGraph);
  }

  return {
    ...rest,
    graphId: graph ? graph.id : null,
  };
};

export const cascadeDeleteRegion = (regionId: string): void => {
  const getRegion = useRegionStore.getState().getRegion;
  const removeRegion = useRegionStore.getState().removeRegion;
  const detachRegionFromSet = useRegionSetStore.getState().detachRegion;

  const region = getRegion(regionId);
  if (!region) return;

  if (region.graphId) {
    cascadeDeleteGraph(region.graphId);
  }

  detachRegionFromSet(region.regionSetId, regionId);
  removeRegion(regionId);
};
