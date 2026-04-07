"use client"

import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
} from "@/components/ui/sidebar"
import type { TrackMetaViewModel } from "@/domain/Track/TrackMetaViewModel"
import type {  RightClickContext } from "@/Stores/UIStore"
import { TrackItem } from "./dashboard/sidebar/track-item"

export interface NavMainProps{
   tracks:TrackMetaViewModel[],
   onRightClick: (ctx: RightClickContext) => void
}
export function NavMain({
  tracks,
  onRightClick

}:NavMainProps) {


    return (
    <SidebarGroup>
      <SidebarGroupLabel>Tracks</SidebarGroupLabel>
      <SidebarMenu>
        {tracks.map((item) => (
          <TrackItem
            key={item.trackId}
            track={item}
            onRightClick={onRightClick}
          />
        ))}
      </SidebarMenu>
    </SidebarGroup>
  );
}
