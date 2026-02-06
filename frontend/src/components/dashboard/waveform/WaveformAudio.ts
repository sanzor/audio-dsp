import { useAudioPlaybackCache } from "@/Providers/UsePlaybackCache";
import { useTrackViewModelById } from "@/Selectors/trackViewModels";
import { apiGetStoredTrack } from "@/Services/TracksService";
import { useEffect, useState } from "react";

export function useWaveformAudio(trackId: string | null) {
  const track = useTrackViewModelById(trackId);
  const { getBlob, setBlob } = useAudioPlaybackCache();
  const [objectUrl, setObjectUrl] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    if (!trackId || !track) return;

    let cancelled = false;
    setIsLoading(true);

    (async () => {
      let blob = getBlob(trackId);

      if (!blob) {
        const response = await apiGetStoredTrack({ track_id: trackId });
        blob = response.blob;
        setBlob(trackId, blob);
      }

      if (cancelled) return;

      const url = URL.createObjectURL(blob);
      setObjectUrl(prev => {
        if (prev) URL.revokeObjectURL(prev);
        return url;
      });

      setIsLoading(false);
    })();

    return () => {
      cancelled = true;
      setObjectUrl(prev => {
        if (prev) URL.revokeObjectURL(prev);
        return null;
      });
    };
  }, [trackId,getBlob,setBlob,track]);

  return { objectUrl, isLoading };
}
