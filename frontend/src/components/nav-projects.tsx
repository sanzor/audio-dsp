import { Folder } from "lucide-react"
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import type { TrackMetaViewModel } from "@/domain/Track/TrackMetaViewModel";
import { useEffect } from "react";


export interface NaveProjectsProps{
  tracks:TrackMetaViewModel[],
  onRemoveTrack?:(trackId:string)=>void

}
export function NavProjects({tracks}:NaveProjectsProps) {
  function TrackSidebarItem({track}:{track:TrackMetaViewModel}){
  return(
     <SidebarMenuItem key={track.trackId}>
      <SidebarMenuButton asChild>
        <div className="flex items-center gap-2">
          <Folder className="w-4 h-4" />
          <span>{track.trackInfo.name}</span>
        </div>
      </SidebarMenuButton>
      <SidebarMenu className="ml-4">
        {track.regionSets.flatMap((regionSet) =>
          regionSet.regions.map((region) => (
            <SidebarMenuItem key={region.regionId}>
              <SidebarMenuButton asChild>
                <a href={`/track/${track.trackId}/region/${region.regionId}`}>
                  <span className="text-sm text-muted-foreground hover:text-foreground">
                    {regionSet.name} / {region.name}
                  </span>
                </a>
              </SidebarMenuButton>
            </SidebarMenuItem>
          ))
        )}
      </SidebarMenu>
    </SidebarMenuItem>);
}
  useEffect(() => {
  console.log("📦 tracks updated:", tracks);
}, [tracks]); // ✅ fixed-size dependency list
  return (
    <SidebarGroup className="group-data-[collapsible=icon]:hidden">
      <SidebarGroupLabel>Projects</SidebarGroupLabel>
      <SidebarMenu>
        {tracks.map((track) => {
          console.log("🎯 Rendering track:", track);
          return <TrackSidebarItem key={track.trackId} track={track} />;
        })}
      </SidebarMenu>
    </SidebarGroup>
  );
}
