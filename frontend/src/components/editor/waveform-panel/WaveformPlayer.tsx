import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type WaveSurfer from "wavesurfer.js";
import { useWorkletSetup } from "@/audio/hooks/useWorkletSetup";
import { useRegionSetViewModel } from "@/Selectors/trackViewModels";
import { useWaveformAudio } from "./WaveformAudio";
import { WaveformRenderer } from "./WaveformRenderer";
import { WaveformToolbar } from "./WaveformToolbar";
import { PlaybackControls } from "./PlaybackControls";
import { useWaveSurferPlaybackControls } from "./useWaveSurferPlaybackControls";
import { useUIStore } from "@/Stores/UIStore";
import { useWorkletStore } from "@/Stores/WorkletStore";
import { useRegionController } from "@/controllers/RegionController";
import { useRegionSetController } from "@/controllers/RegionSetController";
import { useActiveGraphId } from "@/hooks/graphs/useActiveGraphId";

export function WaveformPlayer() {
  const activeSelection = useUIStore((x) => x.activeSelection);
  const editingRegionBounds = useUIStore((x) => x.editingRegionBounds);
  const setActiveRegion = useUIStore((x) => x.setActiveRegion);
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
  const [createDraftBounds, setCreateDraftBounds] = useState<{ start: number; end: number } | null>(null);
  const waveRef = useRef<WaveSurfer | null>(null);
  const playback = useWaveSurferPlaybackControls(waveRef);
  const { onWaveformBound, setEffects } = useWorkletSetup();
  const activeGraphId = useActiveGraphId();
  const effectsEnabled = useWorkletStore((s) => s.effectsEnabled);
  const setEffectsEnabled = useWorkletStore((s) => s.setEffectsEnabled);
  const hasGraph = activeGraphId != null;
  const isPlaying = playback.isPlaying;
  const bindPlaybackWaveform = playback.bindWaveform;
  const play = playback.play;
  const playRange = playback.playRange;
  const seekToTime = playback.seekToTime;

  const selectedRegion = useMemo(() => {
    if (!regionSetViewModel || !regionId) return null;

    const region = regionSetViewModel.regions.find((candidate) => candidate.regionId === regionId);
    if (!region) return null;

    if (editingRegionBounds && editingRegionBounds.regionId === region.regionId) {
      return {
        ...region,
        start: editingRegionBounds.start,
        end: editingRegionBounds.end,
      };
    }

    return region;
  }, [editingRegionBounds, regionId, regionSetViewModel]);

  const bindWaveform = useCallback((ws: WaveSurfer) => {
    const cleanupPlayback = bindPlaybackWaveform(ws);
    const cleanupEffects = onWaveformBound(ws);
    return () => { cleanupPlayback(); cleanupEffects(); };
  }, [bindPlaybackWaveform, onWaveformBound]);

  useEffect(() => {
    setEffects(hasGraph && effectsEnabled);
  }, [setEffects, hasGraph, effectsEnabled]);

  useEffect(() => {
    if (!createMode) {
      setCreateDraftBounds(null);
      return;
    }

    if (createDraftBounds && !isPlaying) {
      seekToTime(createDraftBounds.start);
    }
  }, [createDraftBounds, createMode, isPlaying, seekToTime]);

  const handlePlay = useCallback(() => {
    if (createMode && createDraftBounds) {
      playRange(createDraftBounds.start, createDraftBounds.end);
      return;
    }

    if (selectedRegion) {
      playRange(selectedRegion.start, selectedRegion.end);
      return;
    }

    play();
  }, [createDraftBounds, createMode, play, playRange, selectedRegion]);

  const handleCreateRegionDraftChange = useCallback((start: number, end: number) => {
    setCreateDraftBounds((current) => {
      if (current && current.start === start && current.end === end) {
        return current;
      }

      return { start, end };
    });
  }, []);

  const handleRegionSelect = useCallback((nextRegionId: number) => {
    const nextRegion = regionSetViewModel?.regions.find((region) => region.regionId === nextRegionId);

    if (!isPlaying && regionId == null && nextRegion) {
      playback.stop();
      seekToTime(nextRegion.start);
    }

    setActiveRegion(nextRegionId);
  }, [isPlaying, playback, regionId, regionSetViewModel, seekToTime, setActiveRegion]);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key !== "Delete" && e.key !== "Backspace") return;
      const active = document.activeElement as HTMLElement | null;
      if (active && (active.tagName === "INPUT" || active.tagName === "TEXTAREA" || active.isContentEditable)) return;
      if (active?.closest(".react-flow__renderer")) return;
      if (!regionId || isEditingBounds || createMode) return;
      e.preventDefault();
      regionController.handleDeleteRegion(regionId);
    };
    document.addEventListener("keydown", handler);
    return () => document.removeEventListener("keydown", handler);
  }, [regionId, isEditingBounds, createMode, regionController]);

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
          playback={{ ...playback, bindWaveform }}
          regionSet={regionSetViewModel ?? undefined}
          selectedRegionId={regionId ?? undefined}
          createMode={createMode}
          onCancelCreate={() => setCreateMode(false)}
          editingRegionBounds={editingRegionBounds}
          onRegionSelect={handleRegionSelect}
          onRegionDetails={(id) => regionController.handleDetailsRegion(id)}
          onEditRegion={(id) => regionController.handleEditRegion(id)}
          onDeleteRegion={(id) => regionController.handleDeleteRegion(id)}
          onCopyRegion={(id) => regionController.handleCopyRegion(id)}
          onUpdateRegionBounds={(_id, start, end) => updateEditingRegionBounds(start, end)}
          onCreateRegionDraftChange={handleCreateRegionDraftChange}
          onCreateRegionClick={regionSetId != null ? (time) => regionSetController.handleCreateRegion(regionSetId, time) : undefined}
          onCreateRegionDrag={regionSetId != null ? (start, end) => {
            regionSetController.handleCreateRegion(regionSetId, start, end);
            setCreateMode(false);
          } : undefined}
        />
        <PlaybackControls
          hasWaveform={playback.hasWaveform}
          hasGraph={hasGraph}
          effectsEnabled={hasGraph && effectsEnabled}
          isLoading={playback.isLoading}
          isPlaying={playback.isPlaying}
          currentTime={playback.currentTime}
          duration={playback.duration}
          volume={playback.volume}
          playbackRate={playback.playbackRate}
          onPlay={handlePlay}
          onPause={playback.pause}
          onStop={playback.stop}
          onToggleEffects={() => setEffectsEnabled(!effectsEnabled)}
          onVolumeChange={playback.setVolume}
          onPlaybackRateChange={playback.setPlaybackRate}
          onPlaybackRateStep={playback.stepPlaybackRate}
        />
      </div>
    </div>
  );
}
