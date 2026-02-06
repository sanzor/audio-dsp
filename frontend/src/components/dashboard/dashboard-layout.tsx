import type { ReactNode } from "react";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";

interface DashboardLayoutProps {
  store: ReactNode;
  canvas: ReactNode;
  waveform: ReactNode;
}

export function DashboardLayout({ store, canvas, waveform }: DashboardLayoutProps) {
  return (
    <div className="w-full h-screen flex overflow-hidden">

      {/* Main content area - takes remaining space */}
      <div className="flex-1 flex flex-col overflow-hidden min-w-0">
        <PanelGroup direction="horizontal" className="flex-1 overflow-hidden">
          {/* Store/Transform Panel */}
          <Panel defaultSize={20} minSize={15} maxSize={30} className="bg-gray-50 border-r overflow-hidden">
            {store}
          </Panel>

          <PanelResizeHandle className="w-1 bg-gray-300 hover:bg-gray-400 cursor-col-resize" />

          {/* Canvas Panel */}
          <Panel minSize={40} className="bg-white overflow-hidden">
            {canvas}
          </Panel>
        </PanelGroup>

        {/* Waveform at bottom */}
        <div className="border-t bg-white h-40 shadow-inner overflow-hidden">
          {waveform}
        </div>
      </div>
    </div>
  );
}