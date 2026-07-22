import { useState } from "react";
import { CreatorHeader } from "./header";
import { TransformsSidebar } from "./transforms-sidebar";
import { CreatorWorkspace } from "./creator-workspace";
import { TransformPropertiesPanel } from "./transform-properties-panel";
import { CreatorStatusBar } from "./creator-status-bar";
import { UnsavedTransformSourceModal } from "./unsaved-transform-source-modal";
import { TransformModals } from "@/components/dashboard/modals/transform/transform-modals";

type BuildTab = "build" | "test" | "deploy";

export function CreatorShell() {
  const [activeTab, setActiveTab] = useState<BuildTab>("build");

  return (
    <div
      className="h-screen flex flex-col overflow-hidden"
      style={{ backgroundColor: "var(--bg-darkest)", color: "var(--text-main)" }}
    >
      <CreatorHeader activeTab={activeTab} onTabChange={setActiveTab} />

      <div className="flex flex-1 overflow-hidden">
        <TransformsSidebar />
        <CreatorWorkspace />
        <TransformPropertiesPanel />
      </div>

      <CreatorStatusBar />

      <TransformModals />
      <UnsavedTransformSourceModal />
    </div>
  );
}
