import { MenuItem, PositionedMenu } from "./PositionedMenu";

interface WaveformTimelineContextMenuProps {
  regionSetId: number;
  x: number;
  y: number;
  startTime: number;
  onCreateRegionClick: (regionSetId: number, startTime: number) => void;
  onClose: () => void;
}

export function WaveformTimelineContextMenu({
  regionSetId,
  x,
  y,
  startTime,
  onCreateRegionClick,
  onClose,
}: WaveformTimelineContextMenuProps) {
  return (
    <PositionedMenu x={x} y={y} onClose={onClose}>
      <MenuItem onClick={() => { onCreateRegionClick(regionSetId, startTime); onClose(); }}>
        Create Region
      </MenuItem>
    </PositionedMenu>
  );
}
