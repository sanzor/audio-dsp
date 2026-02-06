import {  ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger } from "../../ui/context-menu"

interface RegionContextMenuProps{
  x: number;
  y: number;
  regionId:string,
  onClose:()=>void;
  onRemove: (regionId: string) => void
  onRename: (regionId: string) => void
  onCopyRegion: (regionId: string) => void
  onCreateGraph:(regionId:string)=>void
  onPasteGraph?: (regionId: string) => void
  canPasteGraph?: boolean
  onDetails:(regionId: string)=>void
}

export function RegionContextMenu({
  x,
  y,
  regionId,
  onClose,
  onDetails,
  onRemove,
  onRename,
  onCopyRegion,
  onCreateGraph,
  onPasteGraph,
  canPasteGraph = false,
}: RegionContextMenuProps) {
  return (
    <ContextMenu>
      <ContextMenuTrigger></ContextMenuTrigger>
       <ContextMenuContent style={{ position: "absolute", top: y, left: x, zIndex: 1000 }} onClick={onClose}>   
        <ContextMenuItem onClick={() => {onDetails(regionId);onClose()}}>Details</ContextMenuItem>
       <ContextMenuItem onClick={() => {onRename(regionId);onClose();}}>Rename</ContextMenuItem>
         <ContextMenuItem onClick={() => {onCopyRegion(regionId);onClose();}}>Copy Region</ContextMenuItem>
         <ContextMenuItem onClick={() => {onCreateGraph(regionId);onClose();}}>Create Graph</ContextMenuItem>
         <ContextMenuItem
          disabled={!canPasteGraph}
          onClick={() => {
            if (canPasteGraph && onPasteGraph) {
              onPasteGraph(regionId);
            }
            onClose();
          }}
        >
          Paste Graph
        </ContextMenuItem>
         <ContextMenuItem onClick={() => {onRemove(regionId);onClose();}}>Delete</ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}
