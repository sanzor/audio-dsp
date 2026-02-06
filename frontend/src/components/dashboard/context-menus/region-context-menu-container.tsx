// RegionContextMenuContainer.tsx - Updated
import { useUIStore } from "@/Stores/UIStore";
import { useRegionController } from "@/controllers/RegionController";
import { RegionContextMenu } from "./region-context-menu";


export function RegionContextMenuContainer() {
  const rightClickContext=useUIStore(state=>state.rightClickContext);
  const clipboard = useUIStore(state => state.clipboard);
  const closeContextMenu=useUIStore(state=>state.closeContextMenu);
  const controller=useRegionController();

  if (rightClickContext?.type !== "region") return null;
 //used typed clipboards
  const { regionId, x, y } = rightClickContext;

 

  return (
     <RegionContextMenu
       x={x}
       y={y}
       regionId={regionId}
       onCreateGraph={()=>controller.handleCreateGraph(regionId)}
       onClose={closeContextMenu}
       onDetails={()=>controller.handleDetailsRegion(regionId)}
       onRename={()=>controller.handleEditRegion(regionId)}
       onCopyRegion={()=>controller.handleCopyRegion(regionId)}
       onPasteGraph={()=>controller.handlePasteGraph(regionId)}
       canPasteGraph={clipboard?.type === "graph"}
       onRemove={() => controller.handleDeleteRegion(regionId)}
     />
   );
 
}
