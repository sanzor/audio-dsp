import type { TrackRegionViewModel } from "@/domain/Region/TrackRegionViewModel";
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore";
import { useUIStore } from "@/Stores/UIStore";

interface RegionItemProps {
  region: TrackRegionViewModel;
  onRightClick: (ctx: RightClickContext) => void;
  onSelect: (ctx: SelectedContext) => void;
  onOpen: (ctx: OpenedContext) => void;
}

export function RegionItem({ region, onRightClick, onSelect, onOpen }: RegionItemProps) {
  const selected = useUIStore((s) => s.selectedContext);
  const isSelected = selected?.type === "region" && selected.regionId === region.regionId;

  return (
    <div
      className={`tree-node tree-node-indented-2${isSelected ? " selected-red" : ""}`}
      onClick={() => onSelect({ type: "region", regionId: region.regionId })}
      onDoubleClick={() => onOpen({ type: "region", regionId: region.regionId })}
      onContextMenu={(e) => {
        e.preventDefault();
        onRightClick({ type: "region", regionId: region.regionId, x: e.clientX, y: e.clientY });
      }}
    >
      <span className="truncate">{region.name}</span>
    </div>
  );
}
