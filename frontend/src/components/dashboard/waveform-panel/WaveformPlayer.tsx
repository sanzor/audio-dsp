import { useRef, useState } from "react";
import type WaveSurfer from "wavesurfer.js";
import { useRegionSetViewModel } from "@/Selectors/trackViewModels";
import { useWaveformAudio } from "./WaveformAudio";
import { WaveformRenderer } from "./WaveformRenderer";
import { WaveformToolbar } from "./WaveformToolbar";
import { PlaybackControls } from "./PlaybackControls";
import { useWaveSurferPlaybackController } from "./useWaveSurferPlaybackController";
import { useUIStore } from "@/Stores/UIStore";
import { useRegionController } from "@/controllers/RegionController";
import { useRegionSetController } from "@/controllers/RegionSetController";

export function WaveformPlayer() {
  const activeSelection = useUIStore((x) => x.activeSelection);
  const setActiveRegion = useUIStore((x) => x.setActiveRegion);
  const clearActiveSelection = useUIStore((x) => x.clearActiveSelection);

  const { trackId, regionSetId, regionId } = activeSelection;

  const regionSetViewModel = useRegionSetViewModel(trackId, regionSetId);
  const { objectUrl, isLoading } = useWaveformAudio(trackId);

  const regionController = useRegionController();
  const regionSetController = useRegionSetController();

  const [createMode, setCreateMode] = useState(false);
  const waveRef = useRef<WaveSurfer | null>(null);
  const playback = useWaveSurferPlaybackController(waveRef);

  if (!trackId) return null;
  if (isLoading) return <div>Loading audio...</div>;
  if (!objectUrl) return <div>Missing required data</div>;
  if (!regionSetId) return <div>No region set — add one to start editing.</div>;

  return (
    <div
      className="flex min-h-0 h-full flex-1 flex-col overflow-hidden rounded-lg border shadow-lg"
      style={{ backgroundColor: "var(--bg-darker)", borderColor: "rgba(255,255,255,0.08)" }}
    >
      <WaveformToolbar
        canCreate={regionSetId != null}
        selectedRegionId={regionId ?? undefined}
        createMode={createMode}
        onCreateClick={() => setCreateMode(true)}
        onCancelCreate={() => setCreateMode(false)}
        onEditClick={() => regionController.handleEditRegion(regionId!)}
        onDeleteClick={() => regionController.handleDeleteRegion(regionId!)}
        onCopyClick={() => regionController.handleCopyRegion(regionId!)}
      />
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <WaveformRenderer
          url={objectUrl}
          waveRef={waveRef}
          playback={playback}
          regionSet={regionSetViewModel ?? undefined}
          selectedRegionId={regionId ?? undefined}
          createMode={createMode}
          onRegionSelect={(id) => setActiveRegion(id)}
          onRegionDeselect={() => clearActiveSelection()}
          onRegionDetails={(id) => regionController.handleDetailsRegion(id)}
          onEditRegion={(id) => regionController.handleEditRegion(id)}
          onDeleteRegion={(id) => regionController.handleDeleteRegion(id)}
          onCopyRegion={(id) => regionController.handleCopyRegion(id)}
          onUpdateRegionBounds={(id, start, end) => regionController.handleUpdateRegionBounds(id, start, end)}
          onCreateRegionClick={(time) => regionSetController.handleCreateRegion(regionSetId, time)}
          onCreateRegionDrag={(start, end) => {
            regionSetController.handleCreateRegion(regionSetId, start, end);
            setCreateMode(false);
          }}
        />
        <PlaybackControls
          hasWaveform={playback.hasWaveform}
          isLoading={playback.isLoading}
          isPlaying={playback.isPlaying}
          currentTime={playback.currentTime}
          duration={playback.duration}
          volume={playback.volume}
          playbackRate={playback.playbackRate}
          onPlay={playback.play}
          onPause={playback.pause}
          onStop={playback.stop}
          onVolumeChange={playback.setVolume}
          onPlaybackRateChange={playback.setPlaybackRate}
          onPlaybackRateStep={playback.stepPlaybackRate}
        />
      </div>
    </div>
  );
}
