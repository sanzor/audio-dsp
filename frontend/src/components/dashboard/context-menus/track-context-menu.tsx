import { ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger } from "@/components/ui/context-menu";


interface TrackContextMenuProps{
  x: number;
  y: number;
  trackId: string;
  onClose:()=>void;
  onCreateRegionSet:(id:string)=>void
  onRemove: (id: string) => void
  onRename: (id: string) => void
  onCopyTrack: (id: string) => void
  onDetails:(id:string)=>void
  onPasteRegionSet:(id:string)=>void
  canPasteRegionSet:boolean
}

export function TrackContextMenu({
  x,
  y,
  trackId,
  onClose,
  onCreateRegionSet,
  onDetails,
  onRemove,
  onRename,
  onCopyTrack,
  onPasteRegionSet,
  canPasteRegionSet,
}: TrackContextMenuProps) {
  return (
    <ContextMenu>
      <ContextMenuTrigger></ContextMenuTrigger>
       <ContextMenuContent
          style={{ position: "absolute", top: y, left: x, zIndex: 1000 }}
          onClick={onClose}
        >   
        <ContextMenuItem onClick={() => {onCreateRegionSet(trackId); onClose()}}>Create Region Set</ContextMenuItem>
        <ContextMenuItem onClick={() => {onDetails(trackId);onClose()}}>Details</ContextMenuItem>
        <ContextMenuItem onClick={() => {onRename(trackId);onClose();}}>Rename</ContextMenuItem>
        <ContextMenuItem onClick={() => {onCopyTrack(trackId);onClose();}}>Copy</ContextMenuItem>
        <ContextMenuItem 
          disabled={!canPasteRegionSet}
          onClick={() => {
            if (canPasteRegionSet) {
              onPasteRegionSet(trackId);
            }
            onClose();
          }}
        >
          Paste Region Set
        </ContextMenuItem>
        <ContextMenuItem onClick={() => {onRemove(trackId);onClose();}}>Delete</ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}
