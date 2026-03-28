import { PositionedMenu, MenuItem } from "./PositionedMenu";

interface TrackContextMenuProps {
  x: number;
  y: number;
  trackId: number;
  onClose: () => void;
  onCreateRegionSet: (id: number) => void;
  onRemove: (id: number) => void;
  onRename: (id: number) => void;
  onCopyTrack: (id: number) => void;
  onDetails: (id: number) => void;
  onPasteRegionSet: (id: number) => void;
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
