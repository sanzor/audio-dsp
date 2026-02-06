// TrackContextMenuContainer.tsx
import { useTrackController } from "@/controllers/TrackController";
import { useRegionSetController } from "@/controllers/RegionSetController";
import { useUIStore } from "@/Stores/UIStore";
import { TrackContextMenu } from "./track-context-menu";


export function TrackContextMenuContainer() {
  const rightClickContext = useUIStore(state => state.rightClickContext);
  const clipboard = useUIStore(state => state.clipboard);
  const closeContextMenu = useUIStore(state => state.closeContextMenu);
  const trackController = useTrackController();
  const regionSetController = useRegionSetController();

  if (rightClickContext?.type !== "track") return null;
 //used typed clipboards
  const { trackId, x, y } = rightClickContext;

 

  return (
     <TrackContextMenu
       x={x}
       y={y}
       trackId={trackId}
       onCreateRegionSet={() => trackController.handleCreateRegionSet(trackId)}
       onClose={closeContextMenu}
       onDetails={() => trackController.handleDetailsTrack(trackId)}
       onRename={() => trackController.handleRenameTrack(trackId)}
       onCopyTrack={() => trackController.handleCopyTrack(trackId)}
       onPasteRegionSet={() => regionSetController.handlePasteRegionSet(trackId)}
       canPasteRegionSet={clipboard?.type === "regionSet"}
       onRemove={() => trackController.handleDeleteTrack(trackId)}
     />
   );
 
}
