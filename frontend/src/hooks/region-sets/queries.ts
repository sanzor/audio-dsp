import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { useAuthStore } from "@/Stores/authStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useTrackStore } from "@/Stores/TrackStore";
import { apiGetRegionSet, apiGetRegionSetsForTrack, apiGetAllRegionSets } from "@/Services/RegionSetsService";
import { normalizeRegionSetWithCascade } from "./mutations";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { TrackRegionSet } from "@/domain/RegionSet/TrackRegionSet";
import type { NormalizedTrackRegionSet } from "@/domain/RegionSet/NormalizedTrackRegionSet";

export const useGetRegionSet = (regionSetId: number) => {
  const user = useAuthStore((state) => state.user);

  const query = useQuery<TrackRegionSet, Error, NormalizedTrackRegionSet | undefined>({
    queryKey: QUERY_KEYS.regionSets.byId(regionSetId),
    queryFn: () => apiGetRegionSet(regionSetId),
    enabled: !!regionSetId && !!user,
    select: (data) => (data ? normalizeRegionSetWithCascade(data) : undefined),
  });

  useEffect(() => {
    if (query.data) useRegionSetStore.getState().addRegionSet(query.data);
  }, [query.data]);

  return query;
};

export const useGetAllRegionSets = () => {
  const user = useAuthStore((state) => state.user);

  const query = useQuery({
    queryKey: QUERY_KEYS.regionSets.all(),
    queryFn: apiGetAllRegionSets,
    enabled: !!user,
  });

  useEffect(() => {
    if (!query.data) return;
    const { addRegionSet } = useRegionSetStore.getState();
    const { attachRegionSet } = useTrackStore.getState();
    Object.entries(query.data.track_region_sets_map).forEach(([trackIdStr, regionSets]) => {
      const trackId = Number(trackIdStr);
      regionSets.forEach((rs) => {
        const normalized = normalizeRegionSetWithCascade(rs);
        addRegionSet(normalized);
        attachRegionSet(trackId, normalized.id);
      });
    });
  }, [query.data]);

  return query;
};

export const useGetAllRegionSetsForTrack = (trackId: number) => {
  const user = useAuthStore((state) => state.user);

  const query = useQuery<TrackRegionSet[], Error, NormalizedTrackRegionSet[]>({
    queryKey: QUERY_KEYS.regionSets.byTrack(trackId),
    queryFn: async () => {
      const response = await apiGetRegionSetsForTrack(trackId);
      return response.regionSets;
    },
    enabled: !!trackId && !!user,
    select: (regionSets) => regionSets.map(normalizeRegionSetWithCascade),
  });

  useEffect(() => {
    if (query.data) {
      const { addRegionSet } = useRegionSetStore.getState();
      const { attachRegionSet } = useTrackStore.getState();
      query.data.forEach(rs => {
        addRegionSet(rs);
        attachRegionSet(rs.trackId, rs.id);
      });
    }
  }, [query.data]);

  return query;
};
