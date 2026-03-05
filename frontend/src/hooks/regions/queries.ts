import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { useAuth } from "@/Auth/UseAuth";
import { useRegionStore } from "@/Stores/RegionStore";
import { apiGetRegion, apiGetRegionsForRegionSet } from "@/Services/RegionsService";
import { normalizeRegion } from "@/domain/Region/Mappers";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { TrackRegion } from "@/domain/Region/TrackRegion";
import type { NormalizedTrackRegion } from "@/domain/Region/NormalizedTrackRegion";

export const useGetRegion = (regionId: string) => {
  const { user } = useAuth();

  const query = useQuery<TrackRegion, Error, NormalizedTrackRegion | undefined>({
    queryKey: QUERY_KEYS.regions.byId(regionId),
    queryFn: () => apiGetRegion(regionId),
    enabled: !!regionId && !!user,
    select: (data) => (data ? normalizeRegion(data) : undefined),
  });

  useEffect(() => {
    if (query.data) useRegionStore.getState().addRegion(query.data);
  }, [query.data]);

  return query;
};

export const useGetRegionsForRegionSet = (setId: string) => {
  const { user } = useAuth();

  const query = useQuery<TrackRegion[], Error, NormalizedTrackRegion[]>({
    queryKey: QUERY_KEYS.regions.bySet(setId),
    queryFn: async () => {
      const response = await apiGetRegionsForRegionSet(setId);
      return response.regions;
    },
    enabled: !!setId && !!user,
    select: (regions) => regions.map(normalizeRegion),
  });

  useEffect(() => {
    if (query.data) useRegionStore.getState().setAllRegions(query.data);
  }, [query.data]);

  return query;
};
