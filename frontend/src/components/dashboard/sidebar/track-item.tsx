import { ChevronRight } from "lucide-react";
import {
  Collapsible,
  CollapsibleTrigger,
  CollapsibleContent,
} from "@/components/ui/collapsible";
import { SidebarMenuItem, SidebarMenuButton, SidebarMenuSub } from "@/components/ui/sidebar";
import type { TrackMetaViewModel } from "@/Domain/Track/TrackMetaViewModel";
import { RegionSetItem } from "./region-set-item";
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore";

interface TrackItemProps {
  track: TrackMetaViewModel;
  onSelect: (ctx: SelectedContext) => void;
  onOpen:(ctx:OpenedContext)=>void;
  onRightClick: (ctx: RightClickContext) => void;
}

export function TrackItem({ track, onSelect,onOpen, onRightClick }: TrackItemProps) {
  return (
    <Collapsible defaultOpen className="group/collapsible">
      <SidebarMenuItem>
        <CollapsibleTrigger asChild>
          <SidebarMenuButton 
              onClick={() => onSelect({ type: "track", trackId: track.trackId })} 
              onDoubleClick={()=>onOpen({type:"track",trackId:track.trackId})}>
            <span className="truncate">{track.trackInfo.name}</span>
            <ChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
          </SidebarMenuButton>
        </CollapsibleTrigger>

        <CollapsibleContent>
          <SidebarMenuSub>
            {track.regionSets.map(regionSet => (
              <RegionSetItem
                key={regionSet.id}
                regionSet={regionSet}
                onRightClick={onRightClick}
                onSelect={onSelect}
                onOpen={onOpen}
              />
            ))}
          </SidebarMenuSub>
        </CollapsibleContent>
      </SidebarMenuItem>
    </Collapsible>
  );
}
