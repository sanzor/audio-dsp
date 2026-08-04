import { useCallback, useMemo, useState, type RefObject } from "react";
import type WaveSurfer from "wavesurfer.js";

// Creator-scoped sibling of
// components/editor/waveform-panel/useWaveSurferPlaybackControls.ts -- same
// ready/error -> React-state binding pattern, but trimmed further than that
// file: this hook no longer offers play/pause/stop at all. WaveSurfer here
// renders the waveform and handles click/drag-to-seek hit-testing only; it
// was previously also given its own decorative play/pause/stop transport
// (a separate local audition of the raw source file), but that produced a
// second "is it playing" state alongside the real playback engine's
// (CreatorPlaybackStore.status) once the bottom playback stripe
// (playback-stripe.tsx) started driving the real engine from the same
// buttons -- see agents/architecture.md's real-time/UI-safety framing on
// keeping playback state unambiguous. So this hook is now purely "is the
// waveform loaded, and how long is it" -- isPlaying/currentTime tracking
// (via wavesurfer's own play/pause/timeupdate events) was removed along
// with the actions that would have triggered them.

export interface CreatorWaveformPlaybackControls {
  isLoading: boolean;
  error: string | null;
  duration: number;
  hasWaveform: boolean;
  beginLoading: () => void;
  bindWaveform: (waveform: WaveSurfer) => () => void;
  seekToTime: (time: number) => void;
}

export function useCreatorWaveformPlayback(
  waveRef: RefObject<WaveSurfer | null>
): CreatorWaveformPlaybackControls {
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [duration, setDuration] = useState(0);
  const [hasWaveform, setHasWaveform] = useState(false);

  const beginLoading = useCallback(() => {
    setHasWaveform(false);
    setIsLoading(true);
    setError(null);
    setDuration(0);
  }, []);

  const bindWaveform = useCallback((waveform: WaveSurfer) => {
    setHasWaveform(true);
    setDuration(0);

    const offReady = waveform.once("ready", () => {
      setIsLoading(false);
      setDuration(waveform.getDuration());
    });

    const offError = waveform.on("error", (waveformError) => {
      console.error("Playback waveform error:", waveformError);
      setError(`Failed to load audio: ${waveformError}`);
      setIsLoading(false);
    });

    return () => {
      setHasWaveform(false);
      offReady();
      offError();
    };
  }, []);

  const seekToTime = useCallback((time: number) => {
    const waveform = waveRef.current;
    if (!waveform) return;
    const safeTime = Math.max(0, time);
    waveform.setTime(safeTime);
  }, [waveRef]);

  return useMemo(() => ({
    isLoading,
    error,
    duration,
    hasWaveform,
    beginLoading,
    bindWaveform,
    seekToTime,
  }), [beginLoading, bindWaveform, duration, error, hasWaveform, isLoading, seekToTime]);
}
