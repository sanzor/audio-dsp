import { useMemo } from "react";
import type { TrackMetaViewModel } from "@/Domain/Track/TrackMetaViewModel";
import type { TrackRegionSetViewModel } from "@/Domain/RegionSet/TrackRegionSetViewModel";
import type { TrackRegionViewModel } from "@/Domain/Region/TrackRegionViewModel";
import type { Graph } from "@/Domain/Graph/Graph";
import type { NormalizedTrackMeta } from "@/Domain/Track/NormalizedTrackMeta";
import type { NormalizedTrackRegionSet } from "@/Domain/RegionSet/NormalizedTrackRegionSet";
import type { NormalizedTrackRegion } from "@/Domain/Region/NormalizedTrackRegion";
import { useTrackStore } from "@/Stores/TrackStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useRegionStore } from "@/Stores/RegionStore";
import { useGraphStore } from "@/Stores/GraphStore";

const getGraph = (graphMap: Map<string, Graph>, graphId: string | null): Graph | null => {
  if (!graphId) return null;
  return graphMap.get(graphId) ?? null;
};

const buildRegionViewModel = (
  region: NormalizedTrackRegion,
  graphMap: Map<string, Graph>
): TrackRegionViewModel => {
  const { graphId, ...rest } = region;
  return {
    ...rest,
    graph: getGraph(graphMap, graphId),
  };
};

const buildRegionSetViewModel = (
  regionSet: NormalizedTrackRegionSet,
  regionMap: Map<string, NormalizedTrackRegion>,
  graphMap: Map<string, Graph>
): TrackRegionSetViewModel => {
  const { region_ids, ...rest } = regionSet;

  const regions: TrackRegionViewModel[] = region_ids
    .map(regionId => regionMap.get(regionId))
    .filter((region): region is NormalizedTrackRegion => Boolean(region))
    .map(region => buildRegionViewModel(region, graphMap));

  return {
    ...rest,
    regions,
  };
};

const buildTrackViewModel = (
  track: NormalizedTrackMeta,
  regionSetMap: Map<string, NormalizedTrackRegionSet>,
  regionMap: Map<string, NormalizedTrackRegion>,
  graphMap: Map<string, Graph>
): TrackMetaViewModel => {
  const { region_sets_ids, ...rest } = track;

  const regionSets: TrackRegionSetViewModel[] = region_sets_ids
    .map(setId => regionSetMap.get(setId))
    .filter((set): set is NormalizedTrackRegionSet => Boolean(set))
    .map(set => buildRegionSetViewModel(set, regionMap, graphMap));

  return {
    ...rest,
    regionSets,
  };
};

const buildTrackViewModelMap = (
  tracks: Map<string, NormalizedTrackMeta>,
  regionSets: Map<string, NormalizedTrackRegionSet>,
  regions: Map<string, NormalizedTrackRegion>,
  graphs: Map<string, Graph>
): Map<string, TrackMetaViewModel> => {
  const result = new Map<string, TrackMetaViewModel>();
  tracks.forEach(track => {
    const viewModel = buildTrackViewModel(track, regionSets, regions, graphs);
    result.set(track.trackId, viewModel);
  });
  return result;
};

export const useTrackViewModelMap = () => {
  const trackMap = useTrackStore(state => state.tracks);
  const regionSetMap = useRegionSetStore(state => state.regionSets);
  const regionMap = useRegionStore(state => state.regions);
  const graphMap = useGraphStore(state => state.graphs);

  return useMemo(
    () => buildTrackViewModelMap(trackMap, regionSetMap, regionMap, graphMap),
    [trackMap, regionSetMap, regionMap, graphMap]
  );
};

export const useTrackViewModels = (): TrackMetaViewModel[] => {
  const map = useTrackViewModelMap();
  return useMemo(() => Array.from(map.values()), [map]);
};

export const useTrackViewModelById = (trackId: string | null | undefined): TrackMetaViewModel | null => {
  const map = useTrackViewModelMap();
  return useMemo(() => {
    if (!trackId) return null;
    return map.get(trackId) ?? null;
  }, [map, trackId]);
};

export const useRegionSetViewModel = (
  trackId: string | null | undefined,
  regionSetId: string | null | undefined
): TrackRegionSetViewModel | null => {
  const track = useTrackViewModelById(trackId);
  return useMemo(() => {
    if (!track || !regionSetId) return null;
    return track.regionSets.find(set => set.id === regionSetId) ?? null;
  }, [track, regionSetId]);
};

export const useRegionViewModel = (
  trackId: string | null | undefined,
  regionSetId: string | null | undefined,
  regionId: string | null | undefined
): TrackRegionViewModel | null => {
  const regionSet = useRegionSetViewModel(trackId, regionSetId);
  return useMemo(() => {
    if (!regionSet || !regionId) return null;
    return regionSet.regions.find(region => region.regionId === regionId) ?? null;
  }, [regionSet, regionId]);
};
