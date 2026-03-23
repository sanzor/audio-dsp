import { PositionedMenu, MenuItem } from "./PositionedMenu";

interface RegionSetContextMenuProps {
  x: number;
  y: number;
  regionSetId: string;
  onClose: () => void;
  onCreateRegion: (id: string) => void;
  onRemove: (id: string) => void;
  onRename: (id: string) => void;
  onCopyRegionSet: (id: string) => void;
  onPasteRegion: (id: string) => void;
  canPasteRegion: boolean;
  onDetails: (id: string) => void;
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
