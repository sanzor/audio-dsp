import { useState } from "react";
import { CreatorHeader } from "./creator-header";
import { CreatorLeftPanel } from "./creator-left-panel";
import { CreatorWorkspace } from "./creator-workspace";
import { CreatorRightPanel } from "./creator-right-panel";
import { CreatorStatusBar } from "./creator-status-bar";

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
        <CreatorLeftPanel />
        <CreatorWorkspace />
        <CreatorRightPanel />
      </div>

      <CreatorStatusBar />
    </div>
  );
}
