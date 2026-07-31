import type { TrackRegionViewModel } from "@/domain/Region/TrackRegionViewModel";
import type { RightClickContext } from "@/Stores/UIStore";
import { useUIStore } from "@/Stores/UIStore";

interface RegionItemProps {
  region: TrackRegionViewModel;
  onRightClick: (ctx: RightClickContext) => void;
}

export function RegionItem({ region, onRightClick }: RegionItemProps) {
  const isSelected = useUIStore((s) => s.activeSelection.regionId === region.regionId);
  const setActiveRegion = useUIStore((s) => s.setActiveRegion);

  return (
    <div
      className={`tree-node tree-node-indented-2${isSelected ? " selected-red" : ""}`}
      onClick={() => setActiveRegion(region.regionId)}
      onContextMenu={(e) => {
        e.preventDefault();
        onRightClick({ type: "region", regionId: region.regionId, x: e.clientX, y: e.clientY });
      }}
    >
      <span className="truncate">{region.name}</span>
    </div>
  );
}
