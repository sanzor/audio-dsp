import type { ReactNode } from "react";

interface DashboardLayoutProps {
  projects: ReactNode;
  sidebar: ReactNode;
  store: ReactNode;
  canvas: ReactNode;
  timeline: ReactNode;
}

export function DashboardLayout({
  projects,
  sidebar,
  store,
  canvas,
  timeline,
}: DashboardLayoutProps) {
  return (
    <div className="daw-workspace">
      {/* Col 1 – Projects (240px) */}
      <section className="panel">{projects}</section>

      {/* Col 2 – Sidebar tree (240px) */}
      <section className="panel">{sidebar}</section>

      {/* Col 3 – Transform store (100px) */}
      <section className="panel">{store}</section>

      {/* Col 4 – Canvas + Timeline stacked */}
      <section className="flex flex-col gap-3 min-w-0 overflow-hidden">
        <div className="panel flex-1 overflow-hidden">
          <div className="panel-header">Canvas</div>
          <div className="flex-1 overflow-hidden" style={{ height: "calc(100% - 48px)" }}>
            {canvas}
          </div>
        </div>
        <div className="panel timeline-container">
          <div className="panel-header py-2 text-sm">Timeline</div>
          <div className="flex-1 overflow-hidden">{timeline}</div>
        </div>
      </section>
    </div>
  );
}
