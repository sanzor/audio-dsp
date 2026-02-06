import { useEffect } from "react";
import { useAuth } from "@/Auth/UseAuth";
import type { NormalizedTrackRegionSet } from "@/Domain/RegionSet/NormalizedTrackRegionSet";
import type { TrackRegionSet } from "@/Domain/RegionSet/TrackRegionSet";
import { apiGetRegionSet, apiGetRegionSetsForTrack } from "@/Services/RegionSetsService";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useQuery } from "@tanstack/react-query";
import { normalizeRegionSet } from "./utils";

export const useGetRegionSet = (regionSetId: string) => {
  const { user } = useAuth();

  const query = useQuery<TrackRegionSet, Error, NormalizedTrackRegionSet | undefined>({
    queryKey: ['regionSet', regionSetId],
    queryFn: () => apiGetRegionSet(regionSetId),
    enabled: !!regionSetId && !!user,

    select: (data) => data ? normalizeRegionSet(data) : undefined,
  });

  useEffect(() => {
    if (query.data) useRegionSetStore.getState().addRegionSet(query.data);
  }, [query.data]);

  return query;
};

export const useGetAllRegionSetsForTrack = (trackId: string) => {
  const { user } = useAuth();

  const query = useQuery<TrackRegionSet[], Error, NormalizedTrackRegionSet[]>({
    queryKey: ['regionSets', 'track', trackId],
    queryFn: async () => {
      const response = await apiGetRegionSetsForTrack(trackId);
      return response.sets;
    },
    enabled: !!trackId && !!user,

    select: (regionSets) => regionSets.map(normalizeRegionSet),
  });

  useEffect(() => {
    if (query.data) useRegionSetStore.getState().setAllRegionSets(query.data);
  }, [query.data]);

  return query;
};
