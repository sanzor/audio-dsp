import type { TrackMetaViewModel } from "@/domain/Track/TrackMetaViewModel";
import type { RightClickContext } from "@/Stores/UIStore";
import { TrackItem } from "./track-item";

interface NavMainProps {
  tracks: TrackMetaViewModel[];
  onRightClick: (ctx: RightClickContext) => void;
}

export function NavMain({ tracks, onRightClick }: NavMainProps) {
  return (
    <div className="flex flex-col gap-0">
      {tracks.map((track) => (
        <TrackItem
          key={track.trackId}
          track={track}
          onRightClick={onRightClick}
        />
      ))}
    </div>
  );
}
