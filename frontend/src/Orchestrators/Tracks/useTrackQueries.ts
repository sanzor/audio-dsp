import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { useTrackStore } from "@/Stores/TrackStore";
import { normalizeTrackWithCascade } from "./utils";
import type { TrackMeta } from "@/Domain/Track/TrackMeta";
import type { NormalizedTrackMeta } from "@/Domain/Track/NormalizedTrackMeta";
import { apiGetTracks, apiGetTrackInfo } from "@/Services/TracksService";

export const useListTracks = () => {
  const query = useQuery<TrackMeta[], Error, NormalizedTrackMeta[]>({
    queryKey: ['tracks'],
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
    queryKey: ['track', trackId],
    queryFn: async () => {
      const result = await apiGetTrackInfo({ track_id: trackId });
      return result.track; // <-- extract TrackMeta from GetTrackResult
    },
    enabled: !!trackId,
    select: normalizeTrackWithCascade,
  });

  useEffect(() => {
    if (query.data) useTrackStore.getState().addTrack(query.data);
  }, [query.data]);

  return query;
};
