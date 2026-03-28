import { PositionedMenu, MenuItem } from "./PositionedMenu";

interface RegionContextMenuProps {
  x: number;
  y: number;
  regionId: number;
  onClose: () => void;
  onRemove: (id: number) => void;
  onRename: (id: number) => void;
  onCopyRegion: (id: number) => void;
  onCreateGraph: (id: number) => void;
  onPasteGraph?: (id: number) => void;
  canPasteGraph?: boolean;
  onDetails: (id: number) => void;
}

export function RegionContextMenu({
  x, y, regionId, onClose,
  onDetails, onRemove, onRename, onCopyRegion, onCreateGraph,
  onPasteGraph, canPasteGraph = false,
}: RegionContextMenuProps) {
  const close = (fn: () => void) => { fn(); onClose(); };

  return (
    <PositionedMenu x={x} y={y} onClose={onClose}>
      <MenuItem onClick={() => close(() => onDetails(regionId))}>Details</MenuItem>
      <MenuItem onClick={() => close(() => onRename(regionId))}>Rename</MenuItem>
      <MenuItem onClick={() => close(() => onCopyRegion(regionId))}>Copy Region</MenuItem>
      <MenuItem onClick={() => close(() => onCreateGraph(regionId))}>Create Graph</MenuItem>
      <MenuItem
        disabled={!canPasteGraph}
        onClick={() => close(() => onPasteGraph?.(regionId))}
      >
        Paste Graph
      </MenuItem>
      <MenuItem onClick={() => close(() => onRemove(regionId))}>Delete</MenuItem>
    </PositionedMenu>
  );
}
