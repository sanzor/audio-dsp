// RegionContextMenuContainer.tsx - Updated
import { useUIStore } from "@/Stores/UIStore";
import { WaveformRegionContextMenu } from "./waveform-region-context-menu";
import { useRegionController } from "@/controllers/RegionController";


export function WaveformRegionContextMenuContainer() {
  const rightClickContext=useUIStore(state=>state.rightClickContext);
  const closeContextMenu=useUIStore(state=>state.closeContextMenu);
  const controller=useRegionController();

  if (rightClickContext?.type !== "waveform_region") return null;
 //used typed clipboards
  const { regionId, x, y } = rightClickContext;

  return (
     <WaveformRegionContextMenu
       x={x}
       y={y}
       regionId={regionId}
       onClose={closeContextMenu}
       onDetailsRegion={()=>controller.handleDetailsRegion(regionId)}
       onEditRegion={()=>controller.handleEditRegion(regionId)}
       onCopyRegion={()=>controller.handleCopyRegion(regionId)}
       onDeleteRegion={() => controller.handleDeleteRegion(regionId)}
     />
   );
 
}
