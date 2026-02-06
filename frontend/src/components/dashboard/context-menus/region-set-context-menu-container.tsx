// RegionContextMenuContainer.tsx - Updated
import { useUIStore } from "@/Stores/UIStore";
import { useRegionSetController } from "@/controllers/RegionSetController";
import { RegionSetContextMenu } from "./region-set-context-menu";


export function RegionSetContextMenuContainer() {
  const rightClickContext=useUIStore(state=>state.rightClickContext);
  const clipboard = useUIStore(state => state.clipboard);
  const closeContextMenu=useUIStore(state=>state.closeContextMenu);
  const controller=useRegionSetController();

  if (rightClickContext?.type !== "regionSet") return null;
 //used typed clipboards
  const { regionSetId, x, y } = rightClickContext;

 

  return (
     <RegionSetContextMenu
       x={x}
       y={y}
       regionSetId={regionSetId}
       onCreateRegion={()=>controller.handleCreateRegion(regionSetId)}
       onClose={closeContextMenu}
       onDetails={()=>controller.handleDetailsRegionSet(regionSetId)}
       onRename={()=>controller.handleEditRegionSet(regionSetId)}
       onCopyRegionSet={()=>controller.handleCopyRegionSet(regionSetId)}
       onPasteRegion={()=>controller.handlePasteRegion(regionSetId)}
       canPasteRegion={clipboard?.type === "region"}
       onRemove={() => controller.handleDeleteRegionSet(regionSetId)}
     />
   );
 
}
