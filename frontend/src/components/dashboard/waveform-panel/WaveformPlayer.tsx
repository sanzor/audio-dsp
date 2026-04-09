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
  const editingRegionBounds = useUIStore((x) => x.editingRegionBounds);
  const setActiveRegion = useUIStore((x) => x.setActiveRegion);
  const clearActiveSelection = useUIStore((x) => x.clearActiveSelection);
  const beginEditingRegionBounds = useUIStore((x) => x.beginEditingRegionBounds);
  const updateEditingRegionBounds = useUIStore((x) => x.updateEditingRegionBounds);
  const clearEditingRegionBounds = useUIStore((x) => x.clearEditingRegionBounds);

  const { trackId, regionSetId, regionId } = activeSelection;
  const isEditingBounds = editingRegionBounds != null;
  const hasUnsavedBounds =
    editingRegionBounds != null &&
    (editingRegionBounds.start !== editingRegionBounds.originalStart ||
      editingRegionBounds.end !== editingRegionBounds.originalEnd);

  const regionSetViewModel = useRegionSetViewModel(trackId, regionSetId);
  const { objectUrl, isLoading } = useWaveformAudio(trackId);

  const regionController = useRegionController();
  const regionSetController = useRegionSetController();

  const [createMode, setCreateMode] = useState(false);
  const waveRef = useRef<WaveSurfer | null>(null);
  const playback = useWaveSurferPlaybackController(waveRef);

  const handleSaveBounds = async () => {
    if (!editingRegionBounds) return;
    await regionController.handleUpdateRegionBounds(
      editingRegionBounds.regionId,
      editingRegionBounds.start,
      editingRegionBounds.end,
    );
    clearEditingRegionBounds();
  };

  if (!trackId) return null;
  if (isLoading) return <div>Loading audio...</div>;
  if (!objectUrl) return <div>Missing required data</div>;

  return (
    <div
      className="flex min-h-0 h-full flex-1 flex-col overflow-hidden rounded-lg border shadow-lg"
      style={{ backgroundColor: "var(--bg-darker)", borderColor: "rgba(255,255,255,0.08)" }}
    >
      <WaveformToolbar
        canCreate={regionSetId != null}
        selectedRegionId={regionId ?? undefined}
        createMode={createMode}
        editBoundsMode={isEditingBounds}
        hasUnsavedBounds={hasUnsavedBounds}
        onCreateClick={() => setCreateMode(true)}
        onCancelCreate={() => setCreateMode(false)}
        onEditBoundsClick={() => beginEditingRegionBounds(regionId!)}
        onCancelEditBounds={() => clearEditingRegionBounds()}
        onSaveEditBounds={() => void handleSaveBounds()}
        onRenameClick={() => regionController.handleEditRegion(regionId!)}
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
          onCancelCreate={() => setCreateMode(false)}
          editingRegionBounds={editingRegionBounds}
          onRegionSelect={(id) => setActiveRegion(id)}
          onRegionDeselect={() => clearActiveSelection()}
          onRegionDetails={(id) => regionController.handleDetailsRegion(id)}
          onEditRegion={(id) => regionController.handleEditRegion(id)}
          onDeleteRegion={(id) => regionController.handleDeleteRegion(id)}
          onCopyRegion={(id) => regionController.handleCopyRegion(id)}
          onUpdateRegionBounds={(_id, start, end) => updateEditingRegionBounds(start, end)}
          onCreateRegionClick={regionSetId != null ? (time) => regionSetController.handleCreateRegion(regionSetId, time) : undefined}
          onCreateRegionDrag={regionSetId != null ? (start, end) => {
            regionSetController.handleCreateRegion(regionSetId, start, end);
            setCreateMode(false);
          } : undefined}
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
