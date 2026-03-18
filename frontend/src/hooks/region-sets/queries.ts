import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { useAuthStore } from "@/Stores/authStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { apiGetRegionSet, apiGetRegionSetsForTrack } from "@/Services/RegionSetsService";
import { normalizeRegionSetWithCascade } from "./mutations";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { TrackRegionSet } from "@/domain/RegionSet/TrackRegionSet";
import type { NormalizedTrackRegionSet } from "@/domain/RegionSet/NormalizedTrackRegionSet";

export const useGetRegionSet = (regionSetId: string) => {
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

export const useGetAllRegionSetsForTrack = (trackId: string) => {
  const user = useAuthStore((state) => state.user);

  const query = useQuery<TrackRegionSet[], Error, NormalizedTrackRegionSet[]>({
    queryKey: QUERY_KEYS.regionSets.byTrack(trackId),
    queryFn: async () => {
      const response = await apiGetRegionSetsForTrack(trackId);
      return response.sets;
    },
    enabled: !!trackId && !!user,
    select: (regionSets) => regionSets.map(normalizeRegionSetWithCascade),
  });

  useEffect(() => {
    if (query.data) useRegionSetStore.getState().setAllRegionSets(query.data);
  }, [query.data]);

  return query;
};
