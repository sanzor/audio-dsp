"use client";

import {
  SidebarMenuSubItem,
  SidebarMenuSubButton,
} from "@/components/ui/sidebar";
import type { TrackRegionViewModel } from "@/Domain/Region/TrackRegionViewModel";
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore";


interface RegionItemProps {
  region: TrackRegionViewModel;
  onRightClick: (ctx: RightClickContext) => void;
  onSelect: (ctx: SelectedContext) => void;
  onOpen:(ctx:OpenedContext)=>void;
}

export function RegionItem({ region, onRightClick,onSelect,onOpen}: RegionItemProps) {
  return (
    <SidebarMenuSubItem
      onClick={() => onSelect({ type: "region", regionId: region.regionId })}
      onDoubleClick={() => onOpen({ type: "region", regionId: region.regionId })}
      onContextMenu={e => {
        e.preventDefault();
        onRightClick({
          type: "region",
          regionId: region.regionId,
          x: e.clientX,
          y: e.clientY,
        });
      }}
    >
      <SidebarMenuSubButton asChild>
        <span>{region.name}</span>
      </SidebarMenuSubButton>
    </SidebarMenuSubItem>
  );
}
