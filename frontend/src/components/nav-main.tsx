"use client"

import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
} from "@/components/ui/sidebar"
import type { TrackMetaViewModel } from "@/Domain/Track/TrackMetaViewModel"
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore"
import { TrackItem } from "./dashboard/sidebar/track-item"

export interface NavMainProps{
   tracks:TrackMetaViewModel[],
   onSelect:(ctx: SelectedContext)=>void,
   onOpen:(ctx:OpenedContext)=>void,
   onRightClick: (ctx: RightClickContext) => void
}
export function NavMain({
  tracks,
  onSelect,
  onOpen,
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
            onSelect={onSelect}
            onOpen={onOpen}
            onRightClick={onRightClick}
          />
        ))}
      </SidebarMenu>
    </SidebarGroup>
  );
}
