import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { useAuthStore } from "@/Stores/authStore";
import { useGraphStore } from "@/Stores/GraphStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useRegionStore } from "@/Stores/RegionStore";
import { useTrackStore } from "@/Stores/TrackStore";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { Graph } from "@/domain/Graph/Graph";
import type { NormalizedTrackRegion } from "@/domain/Region/NormalizedTrackRegion";
import type { NormalizedTrackRegionSet } from "@/domain/RegionSet/NormalizedTrackRegionSet";
import type { NormalizedTrackMeta } from "@/domain/Track/NormalizedTrackMeta";
import type { TrackMeta } from "@/domain/Track/TrackMeta";
import { apiGetWorkspace } from "@/Services/WorkspaceService";

interface NormalizedWorkspace {
  tracks: NormalizedTrackMeta[];
  regionSets: NormalizedTrackRegionSet[];
  regions: NormalizedTrackRegion[];
  graphs: Graph[];
}

const normalizeWorkspace = (workspace: TrackMeta[]): NormalizedWorkspace => {
  const tracks: NormalizedTrackMeta[] = [];
  const regionSets: NormalizedTrackRegionSet[] = [];
  const regions: NormalizedTrackRegion[] = [];
  const graphs: Graph[] = [];

  for (const track of workspace) {
    const regionSetIds: number[] = [];

    for (const regionSet of track.regionSets) {
      const regionIds: number[] = [];
      regionSetIds.push(regionSet.id);

      for (const region of regionSet.regions) {
        if (region.graph) {
          graphs.push(region.graph);
        }

        regions.push({
          regionId: region.regionId,
          regionSetId: region.regionSetId,
          name: region.name,
          start: region.start,
          end: region.end,
          graphId: region.graph?.id ?? null,
        });

        regionIds.push(region.regionId);
      }

      regionSets.push({
        id: regionSet.id,
        trackId: regionSet.trackId,
        name: regionSet.name,
        region_ids: regionIds,
      });
    }

    tracks.push({
      trackId: track.trackId,
      trackInfo: track.trackInfo,
      region_sets_ids: regionSetIds,
    });
  }

  return { tracks, regionSets, regions, graphs };
};

export const useWorkspace = (projectId: number | null | undefined) => {
  const user = useAuthStore((state) => state.user);

  const query = useQuery<TrackMeta[], Error, NormalizedWorkspace>({
    queryKey: QUERY_KEYS.workspace.byProject(projectId ?? 0),
    queryFn: () => apiGetWorkspace(projectId!),
    enabled: Boolean(projectId && user),
    select: normalizeWorkspace,
  });

  useEffect(() => {
    if (!query.data) return;

    useTrackStore.getState().setAllTracks(query.data.tracks);
    useRegionSetStore.getState().setAllRegionSets(query.data.regionSets);
    useRegionStore.getState().setAllRegions(query.data.regions);
    useGraphStore.getState().setAllGraphs(query.data.graphs);
  }, [query.data]);

  return query;
};
