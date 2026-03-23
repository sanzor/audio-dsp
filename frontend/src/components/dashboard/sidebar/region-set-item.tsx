import { useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import { RegionItem } from "./region-item";
import type { TrackRegionSetViewModel } from "@/domain/RegionSet/TrackRegionSetViewModel";
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore";
import { useUIStore } from "@/Stores/UIStore";

interface Props {
  regionSet: TrackRegionSetViewModel;
  onRightClick: (ctx: RightClickContext) => void;
  onSelect: (ctx: SelectedContext) => void;
  onOpen: (ctx: OpenedContext) => void;
}

export function RegionSetItem({ regionSet, onRightClick, onSelect, onOpen }: Props) {
  const [open, setOpen] = useState(true);
  const selected = useUIStore((s) => s.selectedContext);
  const isSelected = selected?.type === "regionSet" && selected.regionSetId === regionSet.id;

  return (
    <div>
      <div
        className={`tree-node tree-node-indented${isSelected ? " selected-blue" : ""}`}
        onClick={() => {
          onSelect({ type: "regionSet", regionSetId: regionSet.id });
          setOpen((o) => !o);
        }}
        onDoubleClick={() => onOpen({ type: "regionSet", regionSetId: regionSet.id })}
        onContextMenu={(e) => {
          e.preventDefault();
          onRightClick({ type: "regionSet", regionSetId: regionSet.id, x: e.clientX, y: e.clientY });
        }}
      >
        {open ? (
          <ChevronDown className="w-4 h-4 mr-2 flex-shrink-0" />
        ) : (
          <ChevronRight className="w-4 h-4 mr-2 flex-shrink-0" />
        )}
        <span className="truncate">{regionSet.name}</span>
      </div>

      {open &&
        regionSet.regions.map((region) => (
          <RegionItem
            key={region.regionId}
            region={region}
            onRightClick={onRightClick}
            onSelect={onSelect}
            onOpen={onOpen}
          />
        ))}
    </div>
  );
}
