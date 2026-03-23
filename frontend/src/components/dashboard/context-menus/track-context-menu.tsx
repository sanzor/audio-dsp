import { PositionedMenu, MenuItem } from "./PositionedMenu";

interface TrackContextMenuProps {
  x: number;
  y: number;
  trackId: string;
  onClose: () => void;
  onCreateRegionSet: (id: string) => void;
  onRemove: (id: string) => void;
  onRename: (id: string) => void;
  onCopyTrack: (id: string) => void;
  onDetails: (id: string) => void;
  onPasteRegionSet: (id: string) => void;
  canPasteRegionSet: boolean;
}

export function TrackContextMenu({
  x, y, trackId, onClose,
  onCreateRegionSet, onDetails, onRemove, onRename, onCopyTrack,
  onPasteRegionSet, canPasteRegionSet,
}: TrackContextMenuProps) {
  const close = (fn: () => void) => { fn(); onClose(); };

  return (
    <PositionedMenu x={x} y={y} onClose={onClose}>
      <MenuItem onClick={() => close(() => onCreateRegionSet(trackId))}>Create Region Set</MenuItem>
      <MenuItem onClick={() => close(() => onDetails(trackId))}>Details</MenuItem>
      <MenuItem onClick={() => close(() => onRename(trackId))}>Rename</MenuItem>
      <MenuItem onClick={() => close(() => onCopyTrack(trackId))}>Copy</MenuItem>
      <MenuItem disabled={!canPasteRegionSet} onClick={() => close(() => onPasteRegionSet(trackId))}>Paste Region Set</MenuItem>
      <MenuItem onClick={() => close(() => onRemove(trackId))}>Delete</MenuItem>
    </PositionedMenu>
  );
}
