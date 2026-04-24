import { Pause, Play, Square, Volume2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { PLAYBACK_RATE_PRESETS, PLAYBACK_RATES } from "./useWaveSurferPlaybackControls";

interface PlaybackControlsProps {
  hasWaveform: boolean;
  isLoading: boolean;
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  volume: number;
  playbackRate: number;
  onPlay: () => void;
  onPause: () => void;
  onStop: () => void;
  onVolumeChange: (value: number) => void;
  onPlaybackRateChange: (value: number) => void;
  onPlaybackRateStep: (direction: -1 | 1) => void;
}

export function PlaybackControls({
  hasWaveform,
  isLoading,
  isPlaying,
  currentTime,
  duration,
  volume,
  playbackRate,
  onPlay,
  onPause,
  onStop,
  onVolumeChange,
  onPlaybackRateChange,
  onPlaybackRateStep,
}: PlaybackControlsProps) {
  const controlsDisabled = !hasWaveform || isLoading;

  return (
    <div
      className="flex shrink-0 items-center justify-between gap-4 border-t px-4 py-3"
      style={{ backgroundColor: "var(--bg-darkest)", borderColor: "rgba(255,255,255,0.08)" }}
    >
      <div className="flex items-center gap-2">
        <Button
          type="button"
          size="icon"
          variant="outline"
          onClick={onStop}
          disabled={controlsDisabled}
          aria-label="Stop playback"
          className="border-white/10 bg-white/5 text-white hover:bg-white/10 hover:text-white"
        >
          <Square className="size-4 fill-current" />
        </Button>
        <Button
          type="button"
          size="icon"
          variant="outline"
          onClick={onPause}
          disabled={controlsDisabled || !isPlaying}
          aria-label="Pause playback"
          className="border-white/10 bg-white/5 text-white hover:bg-white/10 hover:text-white"
        >
          <Pause className="size-4 fill-current" />
        </Button>
        <Button
          type="button"
          size="icon"
          onClick={onPlay}
          disabled={controlsDisabled}
          aria-label="Play waveform"
          className="bg-[var(--accent-blue)] text-white shadow-none hover:bg-[var(--accent-blue)]/90"
        >
          <Play className="size-4 fill-current" />
        </Button>
        <div className="min-w-28 text-xs tabular-nums" style={{ color: "var(--text-muted)" }}>
          {formatTime(currentTime)} / {formatTime(duration)}
        </div>
      </div>

      <div className="flex items-center gap-6">
        <div className="flex items-center gap-2">
          <Button
            type="button"
            size="sm"
            variant="outline"
            onClick={() => onPlaybackRateStep(-1)}
            disabled={controlsDisabled || playbackRate <= PLAYBACK_RATES[0]}
            aria-label="Decrease playback speed"
            className="h-8 border-white/10 bg-white/5 px-2 text-white hover:bg-white/10 hover:text-white"
          >
            -
          </Button>
          {PLAYBACK_RATE_PRESETS.map((rate) => (
            <Button
              key={rate}
              type="button"
              size="sm"
              variant={playbackRate === rate ? "default" : "outline"}
              onClick={() => onPlaybackRateChange(rate)}
              disabled={controlsDisabled}
              aria-label={`Set playback speed to ${rate}x`}
              className={
                playbackRate === rate
                  ? "h-8 bg-[var(--accent-blue)] px-2.5 text-white shadow-none hover:bg-[var(--accent-blue)]/90"
                  : "h-8 border-white/10 bg-white/5 px-2.5 text-white hover:bg-white/10 hover:text-white"
              }
            >
              {rate}x
            </Button>
          ))}
          <Button
            type="button"
            size="sm"
            variant="outline"
            onClick={() => onPlaybackRateStep(1)}
            disabled={controlsDisabled || playbackRate >= PLAYBACK_RATES[PLAYBACK_RATES.length - 1]}
            aria-label="Increase playback speed"
            className="h-8 border-white/10 bg-white/5 px-2 text-white hover:bg-white/10 hover:text-white"
          >
            +
          </Button>
        </div>

        <div className="flex min-w-40 items-center gap-3">
          <Volume2 className="size-4" style={{ color: "var(--text-muted)" }} />
          <Slider
            min={0}
            max={100}
            step={1}
            value={[volume]}
            onValueChange={(value) => onVolumeChange(value[0] ?? 0)}
            aria-label="Volume"
            className="w-32"
          />
        </div>
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
