import { useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import type { TrackMetaViewModel } from "@/domain/Track/TrackMetaViewModel";
import { RegionSetItem } from "./region-set-item";
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore";
import { useUIStore } from "@/Stores/UIStore";

interface TrackItemProps {
  track: TrackMetaViewModel;
  onSelect: (ctx: SelectedContext) => void;
  onOpen: (ctx: OpenedContext) => void;
  onRightClick: (ctx: RightClickContext) => void;
}

export function TrackItem({ track, onSelect, onOpen, onRightClick }: TrackItemProps) {
  const [open, setOpen] = useState(true);
  const selected = useUIStore((s) => s.selectedContext);
  const isSelected = selected?.type === "track" && selected.trackId === track.trackId;

  return (
    <div>
      <div
        className={`tree-node${isSelected ? " selected-blue" : ""}`}
        onClick={() => {
          onSelect({ type: "track", trackId: track.trackId });
          onOpen({ type: "track", trackId: track.trackId });
          setOpen((o) => !o);
        }}
        onDoubleClick={() => onOpen({ type: "track", trackId: track.trackId })}
        onContextMenu={(e) => {
          e.preventDefault();
          onRightClick({ type: "track", trackId: track.trackId, x: e.clientX, y: e.clientY });
        }}
      >
        {open ? (
          <ChevronDown className="w-4 h-4 mr-2 flex-shrink-0" />
        ) : (
          <ChevronRight className="w-4 h-4 mr-2 flex-shrink-0" />
        )}
        <span className="truncate">{track.trackInfo.name}</span>
      </div>

      {open &&
        track.regionSets.map((regionSet) => (
          <RegionSetItem
            key={regionSet.id}
            regionSet={regionSet}
            onRightClick={onRightClick}
            onSelect={onSelect}
            onOpen={onOpen}
          />
        ))}
    </div>
  );
}
