import { PositionedMenu, MenuItem } from "./PositionedMenu";

interface RegionSetContextMenuProps {
  x: number;
  y: number;
  regionSetId: number;
  onClose: () => void;
  onCreateRegion: (id: number) => void;
  onRemove: (id: number) => void;
  onRename: (id: number) => void;
  onCopyRegionSet: (id: number) => void;
  onPasteRegion: (id: number) => void;
  canPasteRegion: boolean;
  onDetails: (id: number) => void;
}

export function RegionSetContextMenu({
  x, y, regionSetId, onClose,
  onCreateRegion, onDetails, onRemove, onRename, onCopyRegionSet,
  onPasteRegion, canPasteRegion,
}: RegionSetContextMenuProps) {
  const close = (fn: () => void) => { fn(); onClose(); };

  return (
    <PositionedMenu x={x} y={y} onClose={onClose}>
      <MenuItem onClick={() => close(() => onCreateRegion(regionSetId))}>Create Region</MenuItem>
      <MenuItem onClick={() => close(() => onDetails(regionSetId))}>Details</MenuItem>
      <MenuItem onClick={() => close(() => onRename(regionSetId))}>Rename</MenuItem>
      <MenuItem onClick={() => close(() => onCopyRegionSet(regionSetId))}>Copy</MenuItem>
      <MenuItem disabled={!canPasteRegion} onClick={() => close(() => onPasteRegion(regionSetId))}>Paste Region</MenuItem>
      <MenuItem onClick={() => close(() => onRemove(regionSetId))}>Delete</MenuItem>
    </PositionedMenu>
  );
}
