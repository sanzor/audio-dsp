import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { useTrackStore } from "@/Stores/TrackStore";
import { apiGetTracks, apiGetTrackInfo } from "@/Services/TracksService";
import { normalizeTrackWithCascade } from "./mutations";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { TrackMeta } from "@/domain/Track/TrackMeta";
import type { NormalizedTrackMeta } from "@/domain/Track/NormalizedTrackMeta";

export const useListTracks = () => {
  const query = useQuery<TrackMeta[], Error, NormalizedTrackMeta[]>({
    queryKey: QUERY_KEYS.tracks.all(),
    queryFn: apiGetTracks,
    select: (tracks) => tracks.map(normalizeTrackWithCascade),
  });

  useEffect(() => {
    if (query.data) useTrackStore.getState().setAllTracks(query.data);
  }, [query.data]);

  return query;
};

export const useGetTrack = (trackId: string) => {
  const query = useQuery<TrackMeta, Error, NormalizedTrackMeta>({
    queryKey: QUERY_KEYS.tracks.byId(trackId),
    queryFn: async () => {
      const result = await apiGetTrackInfo({ track_id: trackId });
      return result.track;
    },
    enabled: !!trackId,
    select: normalizeTrackWithCascade,
  });

  useEffect(() => {
    if (query.data) useTrackStore.getState().addTrack(query.data);
  }, [query.data]);

  return query;
};
