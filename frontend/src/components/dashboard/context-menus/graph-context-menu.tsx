import {  ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger } from "../../ui/context-menu"

interface GraphContextMenuProps{
  x: number;
  y: number;
  graphId:string,
  onClose:()=>void;
  onRemove: (graphId:string) => void
  onRename: (graphId:string) => void
  onCopy: (graphId:string) => void
  onDetails:(graphId:string)=>void
}

export function GraphContextMenu({
  x,
  y,
  graphId,
  onClose,
  onDetails,
  onRemove,
  onRename,
  onCopy
}: GraphContextMenuProps) {
   
  return (
    <ContextMenu>
      <ContextMenuTrigger></ContextMenuTrigger>
       <ContextMenuContent style={{ position: "absolute", top: y, left: x, zIndex: 1000 }} onClick={onClose}>   
         <ContextMenuItem onClick={() => {onDetails(graphId);onClose()}}>Details</ContextMenuItem>
        <ContextMenuItem onClick={() => {onRename(graphId);onClose();}}>Rename</ContextMenuItem>
         <ContextMenuItem onClick={() => {onCopy(graphId);onClose();}}>Copy</ContextMenuItem>
         <ContextMenuItem onClick={() => {onRemove(graphId);onClose();}}>Delete</ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}
