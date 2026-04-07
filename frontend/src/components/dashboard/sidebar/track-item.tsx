import { useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import type { TrackMetaViewModel } from "@/domain/Track/TrackMetaViewModel";
import { RegionSetItem } from "./region-set-item";
import type { RightClickContext } from "@/Stores/UIStore";
import { useUIStore } from "@/Stores/UIStore";

interface TrackItemProps {
  track: TrackMetaViewModel;
  onRightClick: (ctx: RightClickContext) => void;
}

export function TrackItem({ track, onRightClick }: TrackItemProps) {
  const [open, setOpen] = useState(true);
  const isSelected = useUIStore((s) => s.activeSelection.trackId === track.trackId && s.activeSelection.regionSetId === null);
  const setActiveTrack = useUIStore((s) => s.setActiveTrack);

  return (
    <div>
      <div
        className={`tree-node${isSelected ? " selected-blue" : ""}`}
        onClick={() => {
          setActiveTrack(track.trackId);
          setOpen((o) => !o);
        }}
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
          />
        ))}
    </div>
  );
}
