import { useCallback, useEffect, useMemo, useRef, useState, type RefObject } from "react";
import type WaveSurfer from "wavesurfer.js";

export const PLAYBACK_RATES = [0.5, 0.75, 1, 1.25, 1.5, 2] as const;
export const PLAYBACK_RATE_PRESETS = [1, 1.5, 2] as const;

export interface WaveSurferPlaybackControls {
  isLoading: boolean;
  error: string | null;
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  volume: number;
  playbackRate: number;
  beginLoading: () => void;
  bindWaveform: (waveform: WaveSurfer) => () => void;
  hasWaveform: boolean;
  play: () => void;
  playRange: (start: number, end?: number) => void;
  pause: () => void;
  stop: () => void;
  seekToTime: (time: number) => void;
  setVolume: (value: number) => void;
  setPlaybackRate: (value: number) => void;
  stepPlaybackRate: (direction: -1 | 1) => void;
}

export function useWaveSurferPlaybackControls(
  waveRef: RefObject<WaveSurfer | null>,
): WaveSurferPlaybackControls {
  type RangeAwareWaveSurfer = WaveSurfer & {
    play: (start?: number, end?: number) => void;
    setTime?: (time: number) => void;
    seekTo?: (progress: number) => void;
  };

  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [volume, setVolume] = useState(80);
  const [playbackRate, setPlaybackRate] = useState<number>(1);
  const [hasWaveform, setHasWaveform] = useState(false);
  const volumeRef = useRef(volume);
  const playbackRateRef = useRef(playbackRate);

  useEffect(() => {
    volumeRef.current = volume;
  }, [volume]);

  useEffect(() => {
    playbackRateRef.current = playbackRate;
  }, [playbackRate]);

  useEffect(() => {
    if (!waveRef.current) return;
    waveRef.current.setVolume(volume / 100);
  }, [volume, waveRef]);

  useEffect(() => {
    if (!waveRef.current) return;
    waveRef.current.setPlaybackRate(playbackRate);
  }, [playbackRate, waveRef]);

  const beginLoading = useCallback(() => {
    setHasWaveform(false);
    setIsLoading(true);
    setError(null);
    setIsPlaying(false);
    setCurrentTime(0);
    setDuration(0);
  }, []);

  const bindWaveform = useCallback((waveform: WaveSurfer) => {
    setHasWaveform(true);
    waveform.setVolume(volumeRef.current / 100);
    waveform.setPlaybackRate(playbackRateRef.current);
    setIsPlaying(false);
    setCurrentTime(0);
    setDuration(0);

    const offReady = waveform.once("ready", () => {
      setIsLoading(false);
      setDuration(waveform.getDuration());
    });

    const offError = waveform.on("error", (waveformError) => {
      console.error("Waveform error:", waveformError);
      setError(`Failed to load audio: ${waveformError}`);
      setIsLoading(false);
    });

    const offPlay = waveform.on("play", () => {
      setIsPlaying(true);
    });

    const offPause = waveform.on("pause", () => {
      setIsPlaying(false);
    });

    const offFinish = waveform.on("finish", () => {
      setIsPlaying(false);
      setCurrentTime(waveform.getDuration());
    });

    const offTimeUpdate = waveform.on("timeupdate", (time) => {
      setCurrentTime(time);
    });

    return () => {
      setHasWaveform(false);
      offReady();
      offError();
      offPlay();
      offPause();
      offFinish();
      offTimeUpdate();
    };
  }, []);

  const play = useCallback(() => {
    waveRef.current?.play();
  }, [waveRef]);

  const seekToTime = useCallback((time: number) => {
    const waveform = waveRef.current as RangeAwareWaveSurfer | null;
    if (!waveform) return;

    const safeTime = Math.max(0, time);
    if (typeof waveform.setTime === "function") {
      waveform.setTime(safeTime);
      setCurrentTime(safeTime);
      return;
    }

    if (typeof waveform.seekTo === "function") {
      const totalDuration = waveform.getDuration();
      if (totalDuration > 0) {
        waveform.seekTo(Math.min(1, safeTime / totalDuration));
        setCurrentTime(safeTime);
      }
    }
  }, [waveRef]);

  const playRange = useCallback((start: number, end?: number) => {
    const waveform = waveRef.current as RangeAwareWaveSurfer | null;
    if (!waveform) return;

    const safeStart = Math.max(0, start);
    const safeEnd = end == null ? undefined : Math.max(safeStart, end);
    setCurrentTime(safeStart);
    waveform.play(safeStart, safeEnd);
  }, [waveRef]);

  const pause = useCallback(() => {
    waveRef.current?.pause();
  }, [waveRef]);

  const stop = useCallback(() => {
    if (!waveRef.current) return;
    waveRef.current.stop();
    setIsPlaying(false);
    setCurrentTime(0);
  }, [waveRef]);

  const stepPlaybackRate = useCallback((direction: -1 | 1) => {
    setPlaybackRate((currentRate) => {
      const currentIndex = PLAYBACK_RATES.findIndex((rate) => rate === currentRate);
      const safeIndex = currentIndex >= 0 ? currentIndex : PLAYBACK_RATES.indexOf(1);
      const nextIndex = Math.min(
        PLAYBACK_RATES.length - 1,
        Math.max(0, safeIndex + direction),
      );

      return PLAYBACK_RATES[nextIndex];
    });
  }, []);

  return useMemo(() => ({
    isLoading,
    error,
    isPlaying,
    currentTime,
    duration,
    volume,
    playbackRate,
    beginLoading,
    bindWaveform,
    hasWaveform,
    play,
    playRange,
    pause,
    stop,
    seekToTime,
    setVolume,
    setPlaybackRate,
    stepPlaybackRate,
  }), [
    beginLoading,
    bindWaveform,
    currentTime,
    duration,
    error,
    hasWaveform,
    isLoading,
    isPlaying,
    pause,
    play,
    playRange,
    playbackRate,
    seekToTime,
    stepPlaybackRate,
    stop,
    volume,
  ]);
}
