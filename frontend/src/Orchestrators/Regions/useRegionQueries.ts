import { useEffect } from "react";
import { useAuth } from "@/Auth/UseAuth";

import { useRegionStore } from "@/Stores/RegionStore";
import { useQuery } from "@tanstack/react-query";

import type { TrackRegion } from "@/Domain/Region/TrackRegion";
import type { NormalizedTrackRegion } from "@/Domain/Region/NormalizedTrackRegion";
import { normalizeRegion } from "@/Domain/Region/Mappers";
import { apiGetRegion, apiGetRegionsForRegionSet } from "@/Services/RegionsService";

export const useGetRegion = (regionId: string) => {
  const { user } = useAuth();

  const query = useQuery<TrackRegion, Error, NormalizedTrackRegion | undefined>({
    queryKey: ['region', regionId],
    queryFn: () => apiGetRegion(regionId),
    enabled: !!regionId && !!user,

    select: (data) => data ? normalizeRegion(data) : undefined,
  });

  useEffect(() => {
    if (query.data) useRegionStore.getState().addRegion(query.data);
  }, [query.data]);

  return query;
};

export const useGetRegionForRegionSet = (setId: string) => {
  const { user } = useAuth();

  const query = useQuery<TrackRegion[], Error, NormalizedTrackRegion[]>({
    queryKey: ['regions', 'region-set', setId],
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
