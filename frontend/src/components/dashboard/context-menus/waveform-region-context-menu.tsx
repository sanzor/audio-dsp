import { MenuItem, PositionedMenu } from "./PositionedMenu";

interface WaveformRegionContextMenuProps {
  x: number;
  y: number;
  regionId: number;
  onEditRegion: (regionId: number) => void;
  onDetailsRegion: (regionId: number) => void;
  onDeleteRegion: (regionId: number) => void;
  onCopyRegion: (regionId: number) => void;
  onClose: () => void;
}

export function WaveformRegionContextMenu({
  regionId,
  x,
  y,
  onEditRegion,
  onDetailsRegion,
  onDeleteRegion,
  onCopyRegion,
  onClose,
}: WaveformRegionContextMenuProps) {
  return (
    <PositionedMenu x={x} y={y} onClose={onClose}>
      <MenuItem onClick={() => { onEditRegion(regionId); onClose(); }}>Edit</MenuItem>
      <MenuItem onClick={() => { onDetailsRegion(regionId); onClose(); }}>Details</MenuItem>
      <MenuItem onClick={() => { onDeleteRegion(regionId); onClose(); }}>Delete</MenuItem>
      <MenuItem onClick={() => { onCopyRegion(regionId); onClose(); }}>Copy</MenuItem>
    </PositionedMenu>
  );
}
