import { useRegionSetViewModel, useTrackViewModelById } from "@/Selectors/trackViewModels";
import { useWaveformAudio } from "./WaveformAudio";
import { WaveformRenderer } from "./WaveformRenderer";
import { useUIStore } from "@/Stores/UIStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";

// Assuming onRegionDetails, onEditRegion, etc., are defined/imported
// ... (or passed as props)

export function WaveformPlayer() {
  // 1. CALL ALL HOOKS UNCONDITIONALLY
  const openedContext = useUIStore(x => x.openedContext);
  const openModal = useUIStore(x => x.openModal);
  const copyToClipboard = useUIStore(x => x.copyToClipboard);

  // Safely determine IDs
  const regionSetId = openedContext?.type === "regionSet" ? openedContext.regionSetId : undefined;
  
  const regionSet = useRegionSetStore(x => 
    regionSetId ? x.regionSets.get(regionSetId) : undefined
  );
  
  const trackId = regionSet?.trackId;
  
  // Call hooks with potentially undefined values
  // Pass undefined instead of null to indicate "no data yet"
  const regionSetViewModel = useRegionSetViewModel(trackId, regionSetId);
  const track = useTrackViewModelById(trackId);
  const { objectUrl, isLoading } = useWaveformAudio(trackId ?? null);

  // 2. CONDITIONAL CHECKS AFTER ALL HOOKS
  if (openedContext?.type !== "regionSet") {
    return null;
  }
  
  // Check for loading state or missing data
  if (isLoading) {
    return <div>Loading audio...</div>;
  }
  
  // Check if we have all required data - regionSetViewModel can be null
  if (!objectUrl || !track || !regionSet || !regionSetViewModel) {
    return <div>Missing required data</div>;
  }

  // 3. RENDER - TypeScript now knows regionSetViewModel is non-null
  return (
    <WaveformRenderer
      url={objectUrl}
      regionSet={regionSetViewModel}
      onRegionDetails={(regionId) => openModal({ type: "detailsRegion", regionId })}
      onEditRegion={(regionId) => openModal({ type: "renameRegion", regionId })}
      onDeleteRegion={(regionId) => openModal({ type: "deleteRegion", regionId })}
      onCopyRegion={(regionId) => copyToClipboard({ type: "region", regionId })}
      onCreateRegionClick={(time) => {
        openModal({ 
          type: "createRegion", 
          regionSetId: regionSet.id, 
          startTime: time 
        });
      }}
      onCreateRegionDrag={(start, end) => {
        openModal({ 
          type: "createRegion", 
          regionSetId: regionSet.id, 
          startTime: start,
          endTime: end
        });
      }}
    />
  );
}