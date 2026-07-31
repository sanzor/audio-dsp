import { PositionedMenu, MenuItem } from "./PositionedMenu";

interface GraphContextMenuProps {
  x: number;
  y: number;
  graphId: string;
  onClose: () => void;
  onRemove: (id: string) => void;
  onRename: (id: string) => void;
  onCopy: (id: string) => void;
  onDetails: (id: string) => void;
}

export function GraphContextMenu({
  x, y, graphId, onClose,
  onDetails, onRemove, onRename, onCopy,
}: GraphContextMenuProps) {
  const close = (fn: () => void) => { fn(); onClose(); };

  return (
    <PositionedMenu x={x} y={y} onClose={onClose}>
      <MenuItem onClick={() => close(() => onDetails(graphId))}>Details</MenuItem>
      <MenuItem onClick={() => close(() => onRename(graphId))}>Rename</MenuItem>
      <MenuItem onClick={() => close(() => onCopy(graphId))}>Copy</MenuItem>
      <MenuItem onClick={() => close(() => onRemove(graphId))}>Delete</MenuItem>
    </PositionedMenu>
  );
}
