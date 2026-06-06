import { useState } from "react";

interface TransformEntry {
  id: string;
  name: string;
  icon: string;
  active?: boolean;
}

interface TransformGroup {
  label: string;
  items: TransformEntry[];
}

const LIBRARY: TransformGroup[] = [
  {
    label: "UNITARY TRANSFORMS",
    items: [
      { id: "rms", name: "RMS_DETECTOR (Rust)", icon: "⚙", active: true },
      { id: "lpf", name: "Low Pass Filter", icon: "⊘" },
      { id: "comp", name: "Compressor", icon: "⊘" },
    ],
  },
  {
    label: "COMPOSITE BLUEPRINTS",
    items: [
      { id: "master", name: "Mastering Chain", icon: "⬡" },
      { id: "vocal", name: "Vocal Processor", icon: "⬡" },
    ],
  },
];

export function CreatorLeftPanel() {
  const [activeId, setActiveId] = useState("rms");

  return (
    <aside
      className="flex flex-col w-64 flex-shrink-0 overflow-hidden"
      style={{
        backgroundColor: "var(--bg-darker)",
        borderRight: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      {/* Header */}
      <div
        className="px-3 py-2.5 flex flex-col gap-0.5"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
      >
        <div className="flex items-center gap-2">
          <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
            CREATOR WORKSPACE
          </span>
          <span
            className="text-[8px] px-1 rounded font-mono"
            style={{ backgroundColor: "rgba(255,255,255,0.08)", color: "var(--text-muted)" }}
          >
            v1.4.2-beta
          </span>
        </div>
        <span className="text-[10px] font-mono" style={{ color: "var(--text-muted)" }}>
          Library / Existing Transforms
        </span>
      </div>

      {/* Transform list */}
      <div className="flex-1 overflow-y-auto py-2">
        {LIBRARY.map((group) => (
          <div key={group.label} className="mb-3">
            <div className="flex items-center gap-1.5 px-3 py-1 mb-0.5">
              <span className="text-[9px] font-mono" style={{ color: "var(--text-muted)" }}>
                ▾ {group.label}
              </span>
            </div>
            {group.items.map((item) => (
              <button
                key={item.id}
                onClick={() => setActiveId(item.id)}
                className="w-full flex items-center gap-2.5 h-8 px-4 text-xs transition-colors text-left"
                style={
                  activeId === item.id
                    ? {
                        color: "#adc6ff",
                        borderLeft: "2px solid #adc6ff",
                        backgroundColor: "rgba(173,198,255,0.08)",
                      }
                    : { color: "var(--text-muted)", borderLeft: "2px solid transparent" }
                }
              >
                <span className="text-[14px]">{item.icon}</span>
                <span>{item.name}</span>
              </button>
            ))}
          </div>
        ))}
      </div>

      {/* Create buttons */}
      <div className="p-2" style={{ borderTop: "1px solid rgba(255,255,255,0.06)" }}>
        <div className="grid grid-cols-2 gap-2 mb-2">
          <button
            className="h-8 text-[10px] font-mono rounded flex items-center justify-center gap-1 transition-colors"
            style={{ border: "1px solid #adc6ff", color: "#adc6ff" }}
          >
            + RUST NODE
          </button>
          <button
            className="h-8 text-[10px] font-mono rounded flex items-center justify-center gap-1 transition-colors"
            style={{ border: "1px solid #4ae176", color: "#4ae176" }}
          >
            ⬡ COMPOSITE
          </button>
        </div>
        <button
          className="w-full h-8 rounded text-[10px] font-mono font-bold flex items-center justify-center gap-2 transition-opacity hover:opacity-90"
          style={{ backgroundColor: "#adc6ff", color: "#002e6a" }}
        >
          ↑ Deploy Transform
        </button>
      </div>

      {/* Bottom links */}
      <div style={{ borderTop: "1px solid rgba(255,255,255,0.06)" }}>
        {["Store", "Support"].map((label) => (
          <button
            key={label}
            className="w-full flex items-center gap-3 h-9 px-4 text-xs transition-colors"
            style={{ color: "var(--text-muted)" }}
          >
            {label}
          </button>
        ))}
      </div>
    </aside>
  );
}
