import { Plus } from "lucide-react";
import { NavMain } from "./nav-main";
import type { TrackMetaViewModel } from "@/domain/Track/TrackMetaViewModel";
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore";

interface SidebarPanelProps {
  tracks: TrackMetaViewModel[];
  onAddTrackClick: () => void;
  onRightClick: (ctx: RightClickContext) => void;
  onSelect: (ctx: SelectedContext) => void;
  onOpen: (ctx: OpenedContext) => void;
}

export function SidebarPanel({
  tracks,
  onAddTrackClick,
  onRightClick,
  onSelect,
  onOpen,
}: SidebarPanelProps) {
  return (
    <>
      <div className="panel-header flex items-center justify-between">
        <span>Tracks</span>
        <button
          onClick={onAddTrackClick}
          className="w-6 h-6 rounded flex items-center justify-center hover:bg-white/10 transition-colors"
          title="Add Track"
          style={{ background: "none", border: "none", padding: 0, cursor: "pointer" }}
        >
          <Plus className="w-4 h-4" style={{ color: "var(--text-muted)" }} />
        </button>
      </div>
      <div className="panel-content">
        <NavMain
          tracks={tracks}
          onSelect={onSelect}
          onOpen={onOpen}
          onRightClick={onRightClick}
        />
      </div>
    </>
  );
}
