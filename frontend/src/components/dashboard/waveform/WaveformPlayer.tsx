import { useRegionSetViewModel, useTrackViewModelById } from "@/Selectors/trackViewModels";
import { useWaveformAudio } from "./WaveformAudio";
import { WaveformRenderer } from "./WaveformRenderer";
import { useUIStore } from "@/Stores/UIStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useRegionController } from "@/controllers/RegionController";
import { useRegionSetController } from "@/controllers/RegionSetController";

export function WaveformPlayer() {
  const openedContext = useUIStore(x => x.openedContext);
  const selectedContext = useUIStore(x => x.selectedContext);

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

  const resolvedTrack = useTrackViewModelById(resolvedTrackId);

  const regionSetId =
    openedContext?.type === "regionSet"
      ? openedRegionSetId
      : selectedRegionSet?.trackId === resolvedTrackId
        ? selectedRegionSetId
        : resolvedTrack?.regionSets[0]?.id;

  const regionSetViewModel = useRegionSetViewModel(resolvedTrackId, regionSetId);
  const { objectUrl, isLoading } = useWaveformAudio(resolvedTrackId);

  const regionController = useRegionController();
  const regionSetController = useRegionSetController();

  if (openedContext?.type !== "track" && openedContext?.type !== "regionSet") return null;
  if (isLoading) return <div>Loading audio...</div>;
  if (!objectUrl) return <div>Missing required data</div>;

  return (
    <WaveformRenderer
      url={objectUrl}
      regionSet={regionSetViewModel ?? undefined}
      onRegionDetails={(regionId) => regionController.handleDetailsRegion(regionId)}
      onEditRegion={(regionId) => regionController.handleEditRegion(regionId)}
      onDeleteRegion={(regionId) => regionController.handleDeleteRegion(regionId)}
      onCopyRegion={(regionId) => regionController.handleCopyRegion(regionId)}
      onUpdateRegionBounds={(regionId, start, end) => regionController.handleUpdateRegionBounds(regionId, start, end)}
      onCreateRegionClick={regionSetId ? (time) => regionSetController.handleCreateRegion(regionSetId, time) : undefined}
      onCreateRegionDrag={regionSetId ? (start, end) => regionSetController.handleCreateRegion(regionSetId, start, end) : undefined}
    />
  );
}
