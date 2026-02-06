import { ContextMenu, ContextMenuContent, ContextMenuItem } from "@/components/ui/context-menu";
import { ContextMenuTrigger } from "@radix-ui/react-context-menu";


interface WaveformRegionContextMenuProps{
  x: number;
  y: number;
  regionId:string,
  onEditRegion:(regionId:string)=>void;
  onDetailsRegion:(regionId:string)=>void;
  onDeleteRegion:(regionId:string)=>void;
  onCopyRegion:(regionId:string)=>void;
  onClose:()=>void;
}

export function WaveformRegionContextMenu({
  regionId,
  x,
  y,
  onEditRegion,
  onDetailsRegion,
  onDeleteRegion,
  onCopyRegion,
  onClose 
}: WaveformRegionContextMenuProps) {
  return (
    <ContextMenu>
      <ContextMenuTrigger></ContextMenuTrigger>
       <ContextMenuContent style={{ position: "absolute", top: y, left: x, zIndex: 1000 }} onClick={onClose}>   
          <ContextMenuItem onClick={() =>{ onEditRegion(regionId);onClose()}}>Edit</ContextMenuItem>
          <ContextMenuItem onClick={() => {onDetailsRegion(regionId); onClose()}}>Details</ContextMenuItem>
          <ContextMenuItem onClick={() => {onDeleteRegion(regionId)}}>Delete</ContextMenuItem>
          <ContextMenuItem onClick={()=>  {onCopyRegion(regionId)}}>Copy</ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

