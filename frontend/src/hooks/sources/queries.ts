import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { apiListSources, apiGetSourceAudio } from "@/Services/SourceService";
import { useSourceAudioCacheStore } from "@/Stores/SourceAudioCacheStore";
import { useAuthStore } from "@/Stores/authStore";
import { QUERY_KEYS } from "@/constants/queryKeys";

// list-sources isn't project-scoped server-side (returns sources from every
// project -- a known, accepted backend gap), so this only gates on being
// authenticated, not on an active project.
export function useListSources() {
  const user = useAuthStore((state) => state.user);
  return useQuery({
    queryKey: QUERY_KEYS.sources.all(),
    queryFn: apiListSources,
    enabled: Boolean(user),
  });
}

// Fetches and caches a source's raw audio bytes as an object URL -- mirrors
// hooks/stored-tracks/queries.ts's useGetStoredTrack. Used for inline
// audition (<audio src={objectUrl}>) in the sources management panel.
export function useGetSourceAudio(sourceId: number | null) {
  const setAudio = useSourceAudioCacheStore((s) => s.setAudio);
  const objectUrl = useSourceAudioCacheStore((s) => (sourceId != null ? (s.getUrl(sourceId) ?? null) : null));

  const query = useQuery({
    queryKey: QUERY_KEYS.sources.audioBySourceId(sourceId ?? 0),
    queryFn: () => apiGetSourceAudio(sourceId!),
    enabled: sourceId != null && !useSourceAudioCacheStore.getState().hasAudio(sourceId),
    staleTime: Infinity,
    gcTime: Infinity,
  });

  useEffect(() => {
    if (query.data && sourceId != null) setAudio(sourceId, query.data);
  }, [query.data, sourceId, setAudio]);

  const isLoading = !!sourceId && objectUrl === null && query.isFetching;
  return { objectUrl, isLoading };
}
