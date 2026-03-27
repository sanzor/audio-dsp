import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { useAudioCacheStore } from "@/Stores/AudioCacheStore";
import { apiGetStoredTrack } from "@/Services/TracksService";
import { QUERY_KEYS } from "@/constants/queryKeys";

export const useGetStoredTrack = (trackId: number | null) => {
  const setAudio = useAudioCacheStore((s) => s.setAudio);
  const objectUrl = useAudioCacheStore((s) => (trackId != null ? (s.getUrl(trackId) ?? null) : null));

  const query = useQuery({
    queryKey: QUERY_KEYS.storedAudio.byTrackId(trackId ?? 0),
    queryFn: () => apiGetStoredTrack({ track_id: trackId! }),
    enabled: trackId != null && !useAudioCacheStore.getState().hasAudio(trackId),
    staleTime: Infinity,
    gcTime: Infinity,
  });

  useEffect(() => {
    if (query.data) setAudio(query.data.track_id, query.data.blob);
  }, [query.data, setAudio]);

  const isLoading = !!trackId && objectUrl === null && query.isFetching;
  return { objectUrl, isLoading };
};
