import { useRegionSetViewModel } from "@/Selectors/trackViewModels";
import { useWaveformAudio } from "./WaveformAudio";
import { WaveformRenderer } from "./WaveformRenderer";
import { useUIStore } from "@/Stores/UIStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useRegionController } from "@/controllers/RegionController";
import { useRegionSetController } from "@/controllers/RegionSetController";

export function WaveformPlayer() {
  const openedContext = useUIStore(x => x.openedContext);

  // regionSetId is already a string from UIStore
  const regionSetId = openedContext?.type === "regionSet" ? openedContext.regionSetId : undefined;
  const regionSet = useRegionSetStore(x => regionSetId ? x.regionSets.get(regionSetId) : undefined);

  const trackIdStr: string | undefined =
    openedContext?.type === "track" ? openedContext.trackId :
    openedContext?.type === "regionSet" ? regionSet?.trackId?.toString() :
    undefined;

  const trackId: number | null = trackIdStr != null ? Number(trackIdStr) : null;

  const regionSetViewModel = useRegionSetViewModel(trackIdStr, regionSetId);
  const { objectUrl, isLoading } = useWaveformAudio(trackId);

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
      onCreateRegionClick={regionSetId ? (time) => regionSetController.handleCreateRegion(regionSetId, time) : undefined}
      onCreateRegionDrag={regionSetId ? (start, end) => regionSetController.handleCreateRegion(regionSetId, start, end) : undefined}
    />
  );
}