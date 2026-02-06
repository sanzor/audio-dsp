"use client"

import { Plus } from "lucide-react"
import { NavMain } from "@/components/nav-main"
import { NavUser } from "@/components/nav-user"
import { TeamSwitcher } from "@/components/team-switcher"

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from "@/components/ui/sidebar"

import type { TrackMetaViewModel } from "@/Domain/Track/TrackMetaViewModel"
import { Button } from "../../ui/button"
import type { OpenedContext, RightClickContext, SelectedContext } from "@/Stores/UIStore"

export interface AppSidebarProps {
  tracks: TrackMetaViewModel[]
  onAddTrackClick: () => void
  onRightClick: (context: RightClickContext) => void
  onSelect: (ctx: SelectedContext) => void
  onOpen: (ctx: OpenedContext) => void;
  user: {
    name: string
    email: string
    avatar: string
  }
  teams: {
    name: string
    logo: React.ComponentType<{ className?: string }>
    plan: string
  }[]
}

export function AppSidebar({
  tracks,
  onAddTrackClick,
  onRightClick,
  onSelect,
  onOpen,
  user,
  teams
}: AppSidebarProps) {
  return (
    <Sidebar 
      collapsible="icon"
      style={{
        "--sidebar-width": "200px",
        "--sidebar-width-icon": "48px",
      } as React.CSSProperties}
    >
      <SidebarHeader>
        <TeamSwitcher teams={teams} />
      </SidebarHeader>

      <SidebarContent>
        <div className="px-2">
          <Button
            onClick={onAddTrackClick}
            className="w-full !bg-green-600 !text-white hover:!bg-green-700"
          >
            <Plus className="mr-2 h-5 w-5" />
            Add Track
          </Button>
        </div>

        <NavMain
          tracks={tracks}
          onRightClick={onRightClick}
          onSelect={onSelect}
          onOpen={onOpen}
        />
      </SidebarContent>

      <SidebarFooter>
        <NavUser user={user} />
      </SidebarFooter>

      <SidebarRail />
    </Sidebar>
  )
}