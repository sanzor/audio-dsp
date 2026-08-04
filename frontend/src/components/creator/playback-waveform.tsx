import { useEffect, useRef } from "react";
import WaveSurfer from "wavesurfer.js";
import { useCreatorPlaybackStore } from "@/Stores/CreatorPlaybackStore";
import { useGetSourceAudio } from "@/hooks/sources/queries";
import { useCreatorWaveformPlayback } from "./useCreatorWaveformPlayback";
import { PlaybackTransport } from "./playback-transport";

// Creator-owned waveform view for the "Try it" playback session's "source"
// input mode (see CreatorPlaybackStore.ts). Visualizes the selected signal
// source and lets the user click/drag to pick where playback of that source
// starts -- WaveSurfer here renders the waveform and handles hit-testing
// only, it never becomes the actual audio path. The real audio still flows
// through CreatorPlaybackStore's own AudioBufferSourceNode -> inputAnalyser
// chain (attachInputSource); picking a new offset here just updates
// playbackSourceOffsetSeconds, which re-triggers that chain live if playback
// is already running.
//
// Transport (Play/Pause/Stop) is owned by the caller (playback-stripe.tsx),
// not this component -- it drives the real playback engine (composite or
// primitive, whichever surface is active), not a local WaveSurfer audition.
// This component only renders the waveform + the transport buttons wired to
// those caller-supplied handlers, plus its own hasWaveform/isLoading/
// duration bookkeeping (useCreatorWaveformPlayback), which stays local since
// it's purely about whether *this* WaveSurfer instance finished loading.
//
// Sibling of components/editor/waveform-panel/WaveformRenderer.tsx's
// lifecycle pattern (create WaveSurfer in a useEffect keyed on the audio
// URL, destroy + null the ref on cleanup) but much simpler: no regions to
// hit-test, so `interact: true` lets WaveSurfer handle click/drag-to-seek
// natively instead of Editor's manual pointerdown + isPointInsideRegion
// approach.
//
// Reuses SourceAudioCacheStore.ts via useGetSourceAudio (the same cache
// sources-panel-modal.tsx's inline <audio> audition uses) rather than
// fetching/decoding a second copy of the source's bytes.

function formatOffset(seconds: number): string {
  if (!Number.isFinite(seconds) || seconds < 0) return "0:00";
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, "0")}`;
}

interface PlaybackWaveformProps {
  /** Whether the real playback engine (not this waveform) is currently playing. */
  isPlaying: boolean;
  /** True when the currently active surface can't be started right now (not
   * compiled / empty composite graph / nothing selected). */
  playDisabled: boolean;
  onPlay: () => void;
  onStop: () => void;
}

export function PlaybackWaveform({ isPlaying, playDisabled, onPlay, onStop }: PlaybackWaveformProps) {
  const playbackInputMode = useCreatorPlaybackStore((s) => s.playbackInputMode);
  const playbackSourceId = useCreatorPlaybackStore((s) => s.playbackSourceId);
  const playbackSourceOffsetSeconds = useCreatorPlaybackStore((s) => s.playbackSourceOffsetSeconds);
  const setPlaybackSourceOffset = useCreatorPlaybackStore((s) => s.setPlaybackSourceOffset);

  const { objectUrl, isLoading: isFetching } = useGetSourceAudio(playbackSourceId);

  const waveformRef = useRef<HTMLDivElement | null>(null);
  const waveRef = useRef<WaveSurfer | null>(null);
  const waveformPlayback = useCreatorWaveformPlayback(waveRef);
  const setPlaybackSourceOffsetRef = useRef(setPlaybackSourceOffset);
  useEffect(() => {
    setPlaybackSourceOffsetRef.current = setPlaybackSourceOffset;
  }, [setPlaybackSourceOffset]);

  const beginLoading = waveformPlayback.beginLoading;
  const bindWaveform = waveformPlayback.bindWaveform;

  useEffect(() => {
    const container = waveformRef.current;
    if (!container || !objectUrl) return;

    beginLoading();

    const waveform = WaveSurfer.create({
      container,
      url: objectUrl,
      waveColor: "rgb(100, 152, 200)",
      progressColor: "rgb(100,100,100)",
      cursorColor: "#f97316",
      height: 56,
      interact: true,
      mediaControls: false,
    });

    waveRef.current = waveform;
    const cleanupPlaybackBindings = bindWaveform(waveform);

    // Restore whatever start offset is already chosen for this source (e.g.
    // reopening the panel after picking one earlier) instead of resetting
    // the visible cursor to 0 on every remount.
    const offReady = waveform.once("ready", () => {
      const currentOffset = useCreatorPlaybackStore.getState().playbackSourceOffsetSeconds;
      if (currentOffset > 0) waveform.setTime(currentOffset);
    });

    // Fires on both click and drag -- see wavesurfer's WaveSurferEvents.
    // This is the one place a user picks a new playback start point.
    const offInteraction = waveform.on("interaction", (newTime) => {
      setPlaybackSourceOffsetRef.current(newTime);
    });

    return () => {
      offReady();
      offInteraction();
      cleanupPlaybackBindings();
      waveform.destroy();
      waveRef.current = null;
    };
  }, [beginLoading, bindWaveform, objectUrl]);

  if (playbackInputMode !== "source" || playbackSourceId == null) return null;

  if (isFetching && !objectUrl) {
    return (
      <div className="px-2 py-1 text-[10px]" style={{ color: "var(--text-muted)" }}>
        Loading waveform…
      </div>
    );
  }

  if (waveformPlayback.error) {
    return (
      <div className="px-2 py-1 text-[10px]" style={{ color: "#ff6b6b" }}>
        {waveformPlayback.error}
      </div>
    );
  }

  return (
    <div className="flex flex-1 items-center gap-3 px-3 py-1.5 min-w-0">
      <PlaybackTransport
        hasWaveform={waveformPlayback.hasWaveform}
        isLoading={waveformPlayback.isLoading}
        isPlaying={isPlaying}
        currentTime={playbackSourceOffsetSeconds}
        duration={waveformPlayback.duration}
        onPlay={onPlay}
        onPause={onStop}
        onStop={onStop}
        disabled={playDisabled}
      />
      <div
        ref={waveformRef}
        className="flex-1 min-w-0 overflow-hidden rounded"
        style={{ backgroundColor: "var(--bg-darker)", border: "1px solid rgba(255,255,255,0.08)" }}
      />
      <div className="flex items-center gap-2 flex-shrink-0">
        <span className="font-mono text-[10px]" style={{ color: "var(--text-muted)" }} title="Where playback starts for this source">
          START: {formatOffset(playbackSourceOffsetSeconds)}
        </span>
      </div>
    </div>
  );
}
