import { useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import { RegionItem } from "./region-item";
import type { TrackRegionSetViewModel } from "@/domain/RegionSet/TrackRegionSetViewModel";
import type { RightClickContext } from "@/Stores/UIStore";
import { useUIStore } from "@/Stores/UIStore";

interface Props {
  regionSet: TrackRegionSetViewModel;
  onRightClick: (ctx: RightClickContext) => void;
}

export function RegionSetItem({ regionSet, onRightClick }: Props) {
  const [open, setOpen] = useState(true);
  const isSelected = useUIStore((s) => s.activeSelection.regionSetId === regionSet.id && s.activeSelection.regionId === null);
  const setActiveRegionSet = useUIStore((s) => s.setActiveRegionSet);

  return (
    <div>
      <div
        className={`tree-node tree-node-indented${isSelected ? " selected-blue" : ""}`}
        onClick={() => {
          setActiveRegionSet(regionSet.id);
          setOpen((o) => !o);
        }}
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
          />
        ))}
    </div>
  );
}
