import { ContextMenu, ContextMenuContent, ContextMenuItem } from "@/components/ui/context-menu";
import { ContextMenuTrigger } from "@radix-ui/react-context-menu";


interface WaveformTimelineContextMenuProps{
  regionSetId:string,
  x: number;
  y: number;
  startTime:number,
  onCreateRegionClick:(regionSetId:string,startTime:number)=>void;
  onClose:()=>void;
}

export function WaveformTimelineContextMenu({
  regionSetId,
  x,
  y,
  startTime,
  onCreateRegionClick,
  onClose 
}: WaveformTimelineContextMenuProps) {
  return (
    <ContextMenu>
      <ContextMenuTrigger>
        
      </ContextMenuTrigger>
       <ContextMenuContent style={{ 
        position: "absolute", 
        top: y, 
        left: x, 
        zIndex: 1000
         }} 
         onClick={onClose}
         >   
        <ContextMenuItem onClick={() => {
          onCreateRegionClick(regionSetId,startTime);
          onClose()}}>Create Region</ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

