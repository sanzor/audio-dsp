import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useAuthStore } from "@/Stores/authStore";
import { DashboardLayout } from "./dashboard-layout";
import { TransformStorePanel } from "./store/transform-store-panel";
import { CanvasPanel } from "./canvas/canvas-panel";
import { useTrackViewModels } from "@/Selectors/trackViewModels";
import { WaveformPlayer } from "./waveform-panel/WaveformPlayer";
import { GlobalContextMenu } from "./context-menus/GlobalContextMenu";
import { GlobalModal } from "./modals/GlobalModal";
import { useUIStore } from "@/Stores/UIStore";
import { useTrackController } from "@/controllers/TrackController";
import { useProjectStore } from "@/Stores/projectStore";
import { useTrackStore } from "@/Stores/TrackStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useRegionStore } from "@/Stores/RegionStore";
import { useGraphStore } from "@/Stores/GraphStore";
import { useTransformStore } from "@/Stores/TransformStore";
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore";
import { QUERY_KEYS } from "@/constants/queryKeys";
import { useWorkspace } from "@/hooks/workspace/queries";
import { useActiveGraphId } from "@/hooks/graphs/useActiveGraphId";
import { useHydrateActiveGraphTransforms } from "@/hooks/transforms/useHydrateActiveGraphTransforms";
import { ProjectsPanel } from "./projects/ProjectsPanel";
import { SidebarPanel } from "./sidebar/SidebarPanel";
import { useAudioEffectsStore } from "@/Stores/WorkletStore";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

export function Dashboard() {
  const user = useAuthStore((state) => state.user);
  const activeProjectId = useProjectStore((s) => s.activeProject?.project_id);
  const activeGraphId = useActiveGraphId();
  const toggleEffectsEnabled = useAudioEffectsStore((s) => s.toggleEffectsEnabled);
  const queryClient = useQueryClient();
  useHydrateActiveGraphTransforms();

  useEffect(() => {
    if (!activeProjectId) return;
    useTrackStore.getState().clear();
    useRegionSetStore.getState().clear();
    useRegionStore.getState().clear();
    useGraphStore.getState().clear();
    useTransformStore.getState().clear();
    useWasmBinaryStore.getState().clear();
    useUIStore.getState().clearActiveSelection();
    queryClient.invalidateQueries({ queryKey: QUERY_KEYS.workspace.byProject(activeProjectId) });
  }, [activeProjectId, queryClient]);

  useWorkspace(activeProjectId);
  const trackController = useTrackController();
  const { openContextMenu, setActiveTrack } = useUIStore();
  const activeTrackId = useUIStore((s) => s.activeSelection.trackId);
  const sidebarTracks = useTrackViewModels();

  useEffect(() => {
    if (activeTrackId) return;
    if (sidebarTracks.length === 0) return;
    setActiveTrack(sidebarTracks[0].trackId);
  }, [sidebarTracks, activeTrackId, setActiveTrack]);

  useEffect(() => {
    const handler = (event: KeyboardEvent) => {
      if (!event.shiftKey || event.key.toLowerCase() !== "g") return;
      const active = document.activeElement as HTMLElement | null;
      if (active && (active.tagName === "INPUT" || active.tagName === "TEXTAREA" || active.isContentEditable)) return;
      if (activeGraphId == null) return;
      event.preventDefault();
      toggleEffectsEnabled();
    };

    document.addEventListener("keydown", handler);
    return () => document.removeEventListener("keydown", handler);
  }, [activeGraphId, toggleEffectsEnabled]);

  return (
    <>
      {/* Overlays — outside the grid so they don't occupy grid slots */}
      <GlobalContextMenu />
      <GlobalModal />

      <div className="daw-layout">
        {/* Top header (56px row) */}
        <header
          className="flex items-center justify-between px-4 border-b gap-4"
          style={{ backgroundColor: "var(--bg-darkest)", borderColor: "rgba(255,255,255,0.05)" }}
        >
          <div className="font-semibold text-sm whitespace-nowrap" style={{ color: "var(--text-main)" }}>
            Collaborative DAW
          </div>

          <div className="flex-1 max-w-md">
            <div className="relative">
              <svg
                className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 pointer-events-none"
                style={{ color: "var(--text-muted)" }}
                fill="none" stroke="currentColor" viewBox="0 0 24 24"
              >
                <path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} />
              </svg>
              <input
                type="text"
                placeholder="Search"
                className="w-full rounded-md py-1.5 pl-9 pr-3 text-sm"
                style={{
                  backgroundColor: "var(--bg-dark)",
                  color: "var(--text-main)",
                  border: "none",
                  outline: "none",
                }}
              />
            </div>
          </div>

          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <button
                className="flex items-center gap-2 rounded-full py-1 px-3 text-sm"
                style={{ backgroundColor: "var(--bg-dark)", color: "var(--text-main)", border: "none", cursor: "pointer" }}
              >
                <div
                  className="w-6 h-6 rounded-full flex items-center justify-center flex-shrink-0"
                  style={{ backgroundColor: "var(--bg-panel)" }}
                >
                  <svg className="w-4 h-4" style={{ color: "var(--text-muted)" }} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} />
                  </svg>
                </div>
                <span className="hidden sm:inline truncate max-w-28">{user?.name ?? "Account"}</span>
              </button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.1)" }}>
              <DropdownMenuLabel style={{ color: "var(--text-muted)" }}>{user?.email ?? ""}</DropdownMenuLabel>
              <DropdownMenuSeparator style={{ backgroundColor: "rgba(255,255,255,0.05)" }} />
              <DropdownMenuItem style={{ color: "var(--text-main)" }}>Account</DropdownMenuItem>
              <DropdownMenuItem style={{ color: "var(--text-main)" }}>Log out</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </header>

        {/* Workspace grid (remaining height) */}
        <DashboardLayout
          projects={<ProjectsPanel />}
          sidebar={
            <SidebarPanel
              tracks={sidebarTracks}
              onAddTrackClick={() => trackController.handleCreateTrack(null)}
              onRightClick={openContextMenu}
            />
          }
          store={<TransformStorePanel />}
          canvas={<CanvasPanel />}
          timeline={<WaveformPlayer />}
        />
      </div>
    </>
  );
}
