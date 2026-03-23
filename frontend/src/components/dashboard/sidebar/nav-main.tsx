import type { TrackMetaViewModel } from "@/domain/Track/TrackMetaViewModel";
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore";
import { TrackItem } from "./track-item";

interface NavMainProps {
  tracks: TrackMetaViewModel[];
  onSelect: (ctx: SelectedContext) => void;
  onOpen: (ctx: OpenedContext) => void;
  onRightClick: (ctx: RightClickContext) => void;
}

export function NavMain({ tracks, onSelect, onOpen, onRightClick }: NavMainProps) {
  return (
    <div className="flex flex-col gap-0">
      {tracks.map((track) => (
        <TrackItem
          key={track.trackId}
          track={track}
          onSelect={onSelect}
          onOpen={onOpen}
          onRightClick={onRightClick}
        />
      ))}
    </div>
  );
}
