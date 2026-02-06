import { ChevronRight } from "lucide-react";
import {
  Collapsible,
  CollapsibleTrigger,
  CollapsibleContent,
} from "@/components/ui/collapsible";
import { SidebarMenuItem, SidebarMenuButton, SidebarMenuSub } from "@/components/ui/sidebar";
import { RegionItem } from "./region-item";
import type { TrackRegionSetViewModel } from "@/Domain/RegionSet/TrackRegionSetViewModel";
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore";

interface Props {
  regionSet: TrackRegionSetViewModel;
  onRightClick: (ctx: RightClickContext) => void;
  onSelect: (ctx: SelectedContext) => void;
  onOpen:(ctx:OpenedContext)=>void;
}

export function RegionSetItem({ regionSet, onRightClick, onSelect, onOpen}: Props) {
  return (
    <Collapsible defaultOpen className="group/collapsible">
      <SidebarMenuItem>
        <CollapsibleTrigger asChild>
          <SidebarMenuButton
            onClick={() => onSelect({ type: "regionSet", regionSetId: regionSet.id })}
            onDoubleClick={() => onOpen({ type: "regionSet", regionSetId: regionSet.id })}
            onContextMenu={e => {
              e.preventDefault();
              onRightClick({ type: "regionSet", regionSetId: regionSet.id, x: e.clientX, y: e.clientY });
            }}
          >
            {regionSet.name}
            <ChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
          </SidebarMenuButton>
        </CollapsibleTrigger>

        <CollapsibleContent>
          <SidebarMenuSub>
            {regionSet.regions.map(region => (
              <RegionItem
                key={region.regionId}
                region={region}
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
