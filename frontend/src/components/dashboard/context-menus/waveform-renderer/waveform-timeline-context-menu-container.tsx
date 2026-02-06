// RegionContextMenuContainer.tsx - Updated
import { useUIStore } from "@/Stores/UIStore";
import { WaveformTimelineContextMenu } from "./waveform-timeline-context-menu";
import { useRegionSetController } from "@/controllers/RegionSetController";


export function WaveformTimelineContextMenuContainer() {
  const rightClickContext=useUIStore(state=>state.rightClickContext);
  const closeContextMenu=useUIStore(state=>state.closeContextMenu);
  const controller=useRegionSetController();

  if (rightClickContext?.type !== "waveform_timeline") return null;
 //used typed clipboards
  const { regionSetId, x, y,time } = rightClickContext;

  return (
     <WaveformTimelineContextMenu
       x={x}
       y={y}
       regionSetId={regionSetId}
       onClose={closeContextMenu}
       onCreateRegionClick={controller.handleCreateRegion}
       startTime={time}
     />
   );
 
}
