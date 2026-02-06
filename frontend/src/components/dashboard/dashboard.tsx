import { useAuth } from "@/Auth/UseAuth";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { AppSidebar } from "./sidebar/app-sidebar";
import { SidebarProvider } from "../ui/sidebar-provider";
import { DashboardLayout } from "./dashboard-layout";
import { TransformStorePanel } from "./store/transform-store-panel";
import { CanvasPanel } from "./graph/canvas-panel";
import { SidebarInset } from "../ui/sidebar";
import { useTrackViewModels } from "@/Selectors/trackViewModels";
import { WaveformPlayer } from "./waveform/WaveformPlayer";
import { RegionSetContextMenuContainer } from "./context-menus/region-set-context-menu-container";
import { TrackContextMenuContainer } from "./context-menus/track-context-menu-container";
import { useUIStore, type OpenedContext, type SelectedContext } from "@/Stores/UIStore";
import { useTrackController } from "@/controllers/TrackController";
import { RegionContextMenuContainer } from "./context-menus/region-context-menu-container";
import { GraphContextMenuContainer } from "./context-menus/graph-context-menu-container";
import { WaveformRegionContextMenuContainer } from "./context-menus/waveform-renderer/waveform-region-context-menu-container";
import { WaveformTimelineContextMenuContainer } from "./context-menus/waveform-renderer/waveform-timeline-context-menu-container";



export function Dashboard() {
  const navigate = useNavigate();
  const { user, loading } = useAuth();

  const trackController=useTrackController();
  const {open,select,openContextMenu} = useUIStore();


  const sidebarTracks = useTrackViewModels();

  useEffect(() => {
    if (!loading && !user) {
      navigate("/login");
    }
  }, [loading, navigate, user]);

  const handleSelect = (ctx: SelectedContext) => {
    select(ctx);
  };
  const handleOpen=(ctx:OpenedContext)=>{
    open(ctx);
  }

  return (

      <SidebarProvider defaultOpen={true}>
        
      <TrackContextMenuContainer></TrackContextMenuContainer>
      <RegionSetContextMenuContainer></RegionSetContextMenuContainer>
      <RegionContextMenuContainer></RegionContextMenuContainer>
      <GraphContextMenuContainer></GraphContextMenuContainer>
      <WaveformRegionContextMenuContainer></WaveformRegionContextMenuContainer>
      <WaveformTimelineContextMenuContainer></WaveformTimelineContextMenuContainer>

      <div className="flex h-screen w-full overflow-hidden">
        {/* Sidebar - fixed width */}
        <AppSidebar
          tracks={sidebarTracks}
          onAddTrackClick={()=>trackController.handleCreateTrack}
          onRightClick={openContextMenu}
          onSelect={handleSelect}
          onOpen={handleOpen}
          user={{
            name: user?.name ?? "Unknown User",
            email: user?.email ?? "",
            avatar: user?.photo ?? "/default-avatar.png",
          }}
          teams={[
            { name: "My Workspace", logo: () => null, plan: "Free" },
          ]}
        />

        {/* Main content - takes remaining space */}
        <SidebarInset className="flex-1 overflow-hidden">
          <DashboardLayout
            store={<TransformStorePanel />}
            canvas={<CanvasPanel />}
            waveform={<WaveformPlayer/>}
          />
        </SidebarInset>
      </div>
      
    </SidebarProvider>
  );
}


