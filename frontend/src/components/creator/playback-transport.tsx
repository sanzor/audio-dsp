import { Pause, Play, Square } from "lucide-react";
import { Button } from "@/components/ui/button";

// Creator-scoped sibling of
// components/editor/waveform-panel/PlaybackControls.tsx -- same button
// choices and `formatTime` style, used here as a visual/structural reference
// only (not imported). Trimmed to play/pause/stop + time readout per the
// task's scope: no volume/playback-rate, no "effects" toggle (there's no
// analog here). Deliberately its own component per the Editor/Creator
// separation rule even though the shape looks identical to the editor's --
// named PlaybackTransport (not PlaybackControls) specifically to stay
// unambiguous from that Editor-surface component in logs/stack traces.

interface PlaybackTransportProps {
  hasWaveform: boolean;
  isLoading: boolean;
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  onPlay: () => void;
  onPause: () => void;
  onStop: () => void;
  /** Extra disable condition on top of hasWaveform/isLoading -- e.g. the
   * active engine (primitive/composite) isn't currently startable (not
   * compiled, or an empty composite graph). Only affects the Play/Pause
   * buttons; Stop stays gated on isPlaying alone. */
  disabled?: boolean;
}

export function PlaybackTransport({
  hasWaveform,
  isLoading,
  isPlaying,
  currentTime,
  duration,
  onPlay,
  onPause,
  onStop,
  disabled,
}: PlaybackTransportProps) {
  const controlsDisabled = !hasWaveform || isLoading;

  return (
    <div className="flex shrink-0 items-center gap-2">
      <Button
        type="button"
        size="icon"
        variant="outline"
        onClick={onStop}
        disabled={controlsDisabled || !isPlaying}
        aria-label="Stop waveform playback"
        className="h-6 w-6 border-white/10 bg-white/5 text-white hover:bg-white/10 hover:text-white"
      >
        <Square className="size-3 fill-current" />
      </Button>
      {isPlaying ? (
        <Button
          type="button"
          size="icon"
          variant="outline"
          onClick={onPause}
          disabled={controlsDisabled}
          aria-label="Pause waveform playback"
          className="h-6 w-6 border-white/10 bg-white/5 text-white hover:bg-white/10 hover:text-white"
        >
          <Pause className="size-3 fill-current" />
        </Button>
      ) : (
        <Button
          type="button"
          size="icon"
          onClick={onPlay}
          disabled={controlsDisabled || Boolean(disabled)}
          aria-label="Play waveform"
          className="h-6 w-6 bg-[var(--accent-blue)] text-white shadow-none hover:bg-[var(--accent-blue)]/90"
        >
          <Play className="size-3 fill-current" />
        </Button>
      )}
      <div className="min-w-16 font-mono text-[10px] tabular-nums" style={{ color: "var(--text-muted)" }}>
        {formatTime(currentTime)} / {formatTime(duration)}
      </div>
    </div>
  );
}

function formatTime(timeSeconds: number): string {
  if (!Number.isFinite(timeSeconds)) return "00:00";
  const totalSeconds = Math.max(0, Math.floor(timeSeconds));
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`;
}
