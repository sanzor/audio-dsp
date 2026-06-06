import { Link } from "react-router-dom";

type BuildTab = "build" | "test" | "deploy";

interface CreatorHeaderProps {
  activeTab: BuildTab;
  onTabChange: (tab: BuildTab) => void;
}

const TABS: { id: BuildTab; label: string }[] = [
  { id: "build", label: "Build" },
  { id: "test", label: "Test" },
  { id: "deploy", label: "Deploy" },
];

export function CreatorHeader({ activeTab, onTabChange }: CreatorHeaderProps) {
  return (
    <header
      className="flex items-center justify-between px-4 h-12 flex-shrink-0 gap-4"
      style={{
        backgroundColor: "var(--bg-darkest)",
        borderBottom: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      {/* Left: title + tabs */}
      <div className="flex items-center gap-6">
        <div className="flex items-center gap-3">
          <span className="text-sm font-semibold" style={{ color: "var(--text-main)" }}>
            Creator Workspace
          </span>
          {/* Mode switcher */}
          <div
            className="flex items-center rounded text-[10px] font-mono overflow-hidden"
            style={{ border: "1px solid rgba(255,255,255,0.12)" }}
          >
            <Link
              to="/dashboard"
              className="px-2.5 py-1 transition-colors"
              style={{ color: "var(--text-muted)", backgroundColor: "transparent" }}
            >
              DAW
            </Link>
            <span
              className="px-2.5 py-1"
              style={{ color: "#adc6ff", backgroundColor: "rgba(173,198,255,0.12)" }}
            >
              CREATOR
            </span>
          </div>
        </div>

        {/* Build / Test / Deploy tabs */}
        <nav className="flex items-center gap-1">
          {TABS.map((tab) => (
            <button
              key={tab.id}
              onClick={() => onTabChange(tab.id)}
              className="px-3 py-1 text-xs font-mono transition-colors rounded"
              style={
                activeTab === tab.id
                  ? { color: "#adc6ff", borderBottom: "2px solid #adc6ff", borderRadius: 0 }
                  : { color: "var(--text-muted)" }
              }
            >
              {tab.label}
            </button>
          ))}
        </nav>
      </div>

      {/* Center: search */}
      <div className="flex-1 max-w-md">
        <div className="relative">
          <svg
            className="absolute left-3 top-1/2 -translate-y-1/2 w-3.5 h-3.5 pointer-events-none"
            style={{ color: "var(--text-muted)" }}
            fill="none" stroke="currentColor" viewBox="0 0 24 24"
          >
            <path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} />
          </svg>
          <input
            type="text"
            placeholder="Search transforms, nodes..."
            className="w-full rounded py-1.5 pl-8 pr-3 text-xs"
            style={{
              backgroundColor: "var(--bg-dark)",
              color: "var(--text-main)",
              border: "1px solid rgba(255,255,255,0.08)",
              outline: "none",
            }}
          />
        </div>
      </div>

      {/* Right: language badge */}
      <span
        className="font-mono font-bold px-2 py-0.5 rounded text-[10px]"
        style={{ color: "#ffb786", border: "1px solid rgba(255,183,134,0.3)" }}
      >
        RUST (WASM)
      </span>
    </header>
  );
}
