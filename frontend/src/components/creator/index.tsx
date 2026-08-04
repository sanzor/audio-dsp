import { useState } from "react";
import { CreatorHeader } from "./header";
import { TransformsSidebar } from "./transforms-sidebar";
import { CreatorWorkspace } from "./creator-workspace";
import { TransformPropertiesPanel } from "./transform-properties-panel";
import { PlaybackStripe } from "./playback-stripe";
import { CreatorStatusBar } from "./creator-status-bar";
import { UnsavedCreatorChangesModal } from "./unsaved-creator-changes-modal";
import { SourcesPanelModal } from "./sources-panel-modal";
import { TransformModals } from "@/components/editor/modals/transform/transform-modals";

type BuildTab = "build" | "test" | "deploy";

export function CreatorShell() {
  const [activeTab, setActiveTab] = useState<BuildTab>("build");
  const [sourcesOpen, setSourcesOpen] = useState(false);

  return (
    <div
      className="h-screen flex flex-col overflow-hidden"
      style={{ backgroundColor: "var(--bg-darkest)", color: "var(--text-main)" }}
    >
      <CreatorHeader activeTab={activeTab} onTabChange={setActiveTab} onOpenSources={() => setSourcesOpen(true)} />

      <div className="flex flex-1 overflow-hidden">
        <TransformsSidebar />
        <CreatorWorkspace />
        <TransformPropertiesPanel />
      </div>

      <PlaybackStripe />

      <CreatorStatusBar />

      <TransformModals />
      <UnsavedCreatorChangesModal />
      <SourcesPanelModal open={sourcesOpen} onClose={() => setSourcesOpen(false)} />
    </div>
  );
}
