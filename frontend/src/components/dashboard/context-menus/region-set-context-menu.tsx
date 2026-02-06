
import {  ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger } from "../../ui/context-menu"

interface RegionSetContextMenuProps{
  x: number;
  y: number;
  regionSetId:string,
  onClose:()=>void;
  onCreateRegion:(regionSetId:string)=>void
  onRemove: (regionSetId: string) => void
  onRename: (regionSetId: string) => void
  onCopyRegionSet: (regionSetId: string) => void
  onPasteRegion:(destRegionSetId:string)=>void
  canPasteRegion:boolean
  onDetails:(regionSetId:string)=>void
}

export function RegionSetContextMenu({
  x,
  y,
  regionSetId,
  onClose,
  onCreateRegion,
  onDetails,
  onRemove,
  onRename,
  onCopyRegionSet,
  onPasteRegion,
  canPasteRegion
}: RegionSetContextMenuProps) {
  return (
    <ContextMenu>
      <ContextMenuTrigger></ContextMenuTrigger>
       <ContextMenuContent style={{ position: "absolute", top: y, left: x, zIndex: 1000 }} onClick={onClose}>   
        <ContextMenuItem onClick={() => {onCreateRegion(regionSetId); onClose()}}>Create Region</ContextMenuItem>
         <ContextMenuItem onClick={() => {onDetails(regionSetId);onClose()}}>Details</ContextMenuItem>
        <ContextMenuItem onClick={() => {onRename(regionSetId);onClose();}}>Rename</ContextMenuItem>
         <ContextMenuItem onClick={() => {onCopyRegionSet(regionSetId);onClose();}}>Copy</ContextMenuItem>
         <ContextMenuItem onClick={() => {onRemove(regionSetId);onClose();}}>Delete</ContextMenuItem>
         <ContextMenuItem disabled={!canPasteRegion} onClick={()=>onPasteRegion(regionSetId)}>Paste</ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}
