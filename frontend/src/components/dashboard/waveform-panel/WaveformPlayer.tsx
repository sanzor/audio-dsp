import { useRef, useState } from "react";
import type WaveSurfer from "wavesurfer.js";
import { useRegionSetViewModel } from "@/Selectors/trackViewModels";
import { useWaveformAudio } from "./WaveformAudio";
import { WaveformRenderer } from "./WaveformRenderer";
import { WaveformToolbar } from "./WaveformToolbar";
import { PlaybackControls } from "./PlaybackControls";
import { useWaveSurferPlaybackController } from "./useWaveSurferPlaybackController";
import { useUIStore } from "@/Stores/UIStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useRegionController } from "@/controllers/RegionController";
import { useRegionSetController } from "@/controllers/RegionSetController";

export function WaveformPlayer() {
  const openedContext = useUIStore(x => x.openedContext);
  const selectedContext = useUIStore(x => x.selectedContext);
  const select = useUIStore(x => x.select);

  const trackId: number | null =
    openedContext?.type === "track" ? openedContext.trackId :
    openedContext?.type === "regionSet" ? null :
    null;

  const selectedRegionSetId =
    selectedContext?.type === "regionSet" ? selectedContext.regionSetId : undefined;

  const selectedRegionSet = useRegionSetStore((state) =>
    selectedRegionSetId != null ? state.regionSets.get(selectedRegionSetId) : undefined,
  );

  const openedRegionSetId = openedContext?.type === "regionSet" ? openedContext.regionSetId : undefined;
  const openedRegionSet = useRegionSetStore((state) =>
    openedRegionSetId != null ? state.regionSets.get(openedRegionSetId) : undefined,
  );

  const resolvedTrackId =
    openedContext?.type === "regionSet" ? openedRegionSet?.trackId ?? null : trackId;

  const firstRegionSetForTrack = useRegionSetStore((state) =>
    resolvedTrackId != null
      ? [...state.regionSets.values()].find((rs) => rs.trackId === resolvedTrackId)
      : undefined
  );

  const regionSetId =
    openedContext?.type === "regionSet"
      ? openedRegionSetId
      : selectedRegionSet?.trackId === resolvedTrackId
        ? selectedRegionSetId
        : firstRegionSetForTrack?.id;

  const regionSetViewModel = useRegionSetViewModel(resolvedTrackId, regionSetId);

  const { objectUrl, isLoading } = useWaveformAudio(resolvedTrackId);

  const regionController = useRegionController();
  const regionSetController = useRegionSetController();

  const [createMode, setCreateMode] = useState(false);
  const waveRef = useRef<WaveSurfer | null>(null);
  const playback = useWaveSurferPlaybackController(waveRef);

  const selectedRegionId =
    selectedContext?.type === 'region' ? selectedContext.regionId : undefined;

  if (openedContext?.type !== "track" && openedContext?.type !== "regionSet") return null;
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
        selectedRegionId={selectedRegionId}
        createMode={createMode}
        onCreateClick={() => setCreateMode(true)}
        onCancelCreate={() => setCreateMode(false)}
        onEditClick={() => regionController.handleEditRegion(selectedRegionId!)}
        onDeleteClick={() => regionController.handleDeleteRegion(selectedRegionId!)}
        onCopyClick={() => regionController.handleCopyRegion(selectedRegionId!)}
      />
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <WaveformRenderer
          url={objectUrl}
          waveRef={waveRef}
          playback={playback}
          regionSet={regionSetViewModel ?? undefined}
          selectedRegionId={selectedRegionId}
          createMode={createMode}
          onRegionSelect={(regionId) => select({ type: 'region', regionId })}
          onRegionDeselect={() => select(null)}
          onRegionDetails={(regionId) => regionController.handleDetailsRegion(regionId)}
          onEditRegion={(regionId) => regionController.handleEditRegion(regionId)}
          onDeleteRegion={(regionId) => regionController.handleDeleteRegion(regionId)}
          onCopyRegion={(regionId) => regionController.handleCopyRegion(regionId)}
          onUpdateRegionBounds={(regionId, start, end) => regionController.handleUpdateRegionBounds(regionId, start, end)}
          onCreateRegionClick={regionSetId ? (time) => regionSetController.handleCreateRegion(regionSetId, time) : undefined}
          onCreateRegionDrag={regionSetId ? (start, end) => {
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
