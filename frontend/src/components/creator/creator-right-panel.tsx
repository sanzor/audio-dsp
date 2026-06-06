import { useState } from "react";

const IO_CHANNELS = [
  { name: "IN L", type: "FLOAT32", color: "#adc6ff" },
  { name: "IN R", type: "FLOAT32", color: "#adc6ff" },
  { name: "WET OUT", type: "FLOAT32", color: "#4ae176" },
];

const TELEMETRY_BARS = [40, 55, 70, 45, 30, 60, 80, 50, 40, 90];

export function CreatorRightPanel() {
  const [windowSize, setWindowSize] = useState(512);
  const [threshold, setThreshold] = useState(60);
  const [autoGain, setAutoGain] = useState(true);

  return (
    <aside
      className="flex flex-col w-80 flex-shrink-0 overflow-hidden"
      style={{
        backgroundColor: "var(--bg-darker)",
        borderLeft: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      {/* Header */}
      <div
        className="px-4 py-3 flex flex-col gap-1"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
      >
        <div className="flex items-center justify-between">
          <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
            PROPERTIES
          </span>
        </div>
        <div className="text-sm font-semibold" style={{ color: "#adc6ff" }}>
          RMS_DETECTOR
        </div>
        <div className="flex gap-2 mt-1">
          <div className="flex-1 flex flex-col gap-0.5">
            <span className="text-[9px] font-mono" style={{ color: "var(--text-muted)" }}>Node Type</span>
            <div
              className="text-xs px-2 py-0.5 rounded"
              style={{ border: "1px solid rgba(255,255,255,0.1)", color: "var(--text-main)", backgroundColor: "var(--bg-dark)" }}
            >
              Unitary (Rust)
            </div>
          </div>
          <div className="flex-1 flex flex-col gap-0.5">
            <span className="text-[9px] font-mono" style={{ color: "var(--text-muted)" }}>Compilation</span>
            <div
              className="text-xs px-2 py-0.5 rounded flex items-center gap-1"
              style={{ border: "1px solid rgba(34,197,94,0.3)", color: "#22c55e", backgroundColor: "rgba(34,197,94,0.08)" }}
            >
              ✓ OK
            </div>
          </div>
        </div>
      </div>

      {/* Scrollable content */}
      <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-5">

        {/* Node Config */}
        <section>
          <h3
            className="text-[9px] font-mono mb-3 pb-1"
            style={{ color: "var(--text-muted)", borderBottom: "1px solid rgba(255,255,255,0.06)" }}
          >
            NODE CONFIG
          </h3>
          <div className="flex flex-col gap-4">
            <div className="flex flex-col gap-1">
              <div className="flex justify-between text-xs">
                <span style={{ color: "var(--text-muted)" }}>Window Size (samples)</span>
                <span className="font-mono" style={{ color: "#adc6ff" }}>{windowSize}</span>
              </div>
              <input
                type="range" min={64} max={2048} value={windowSize}
                onChange={(e) => setWindowSize(Number(e.target.value))}
                className="w-full h-1 rounded appearance-none cursor-pointer accent-[#adc6ff]"
                style={{ backgroundColor: "var(--bg-dark)" }}
              />
              <div className="flex justify-between text-[10px]" style={{ color: "var(--text-muted)" }}>
                <span>64</span><span>2048</span>
              </div>
            </div>

            <div className="flex flex-col gap-1">
              <div className="flex justify-between text-xs">
                <span style={{ color: "var(--text-muted)" }}>Threshold (dB)</span>
                <span className="font-mono" style={{ color: "#adc6ff" }}>-{threshold}</span>
              </div>
              <input
                type="range" min={0} max={60} value={threshold}
                onChange={(e) => setThreshold(Number(e.target.value))}
                className="w-full h-1 rounded appearance-none cursor-pointer accent-[#adc6ff]"
                style={{ backgroundColor: "var(--bg-dark)" }}
              />
              <div className="flex justify-between text-[10px]" style={{ color: "var(--text-muted)" }}>
                <span>-60</span><span>0</span>
              </div>
            </div>

            <div className="flex items-center justify-between">
              <span className="text-xs" style={{ color: "var(--text-muted)" }}>Auto-Gain Makeup</span>
              <button
                onClick={() => setAutoGain((v) => !v)}
                className="w-8 h-4 rounded-full relative flex items-center px-0.5 transition-colors"
                style={{ backgroundColor: autoGain ? "#adc6ff" : "var(--bg-dark)" }}
              >
                <div
                  className="w-3 h-3 rounded-full transition-all"
                  style={{
                    backgroundColor: autoGain ? "#002e6a" : "var(--text-muted)",
                    marginLeft: autoGain ? "auto" : 0,
                  }}
                />
              </button>
            </div>
          </div>
        </section>

        {/* I/O Channels */}
        <section>
          <h3
            className="text-[9px] font-mono mb-3 pb-1"
            style={{ color: "var(--text-muted)", borderBottom: "1px solid rgba(255,255,255,0.06)" }}
          >
            I/O CHANNELS
          </h3>
          <div className="flex flex-col gap-2">
            {IO_CHANNELS.map((ch) => (
              <div
                key={ch.name}
                className="flex items-center justify-between p-2 rounded"
                style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.05)" }}
              >
                <div className="flex items-center gap-2">
                  <div className="w-2 h-2 rounded-full" style={{ backgroundColor: ch.color }} />
                  <span className="text-xs" style={{ color: "var(--text-main)" }}>{ch.name}</span>
                </div>
                <span className="text-[10px] font-mono" style={{ color: "var(--text-muted)" }}>{ch.type}</span>
              </div>
            ))}
          </div>
        </section>

        {/* Telemetry */}
        <section>
          <h3
            className="text-[9px] font-mono mb-3 pb-1"
            style={{ color: "var(--text-muted)", borderBottom: "1px solid rgba(255,255,255,0.06)" }}
          >
            REAL-TIME TELEMETRY
          </h3>
          <div
            className="h-24 rounded relative overflow-hidden flex items-end"
            style={{ backgroundColor: "var(--bg-darkest)", border: "1px solid rgba(255,255,255,0.06)" }}
          >
            <div className="w-full h-full flex items-end gap-px px-1 opacity-40">
              {TELEMETRY_BARS.map((h, i) => (
                <div
                  key={i}
                  className="flex-1 rounded-sm"
                  style={{ height: `${h}%`, backgroundColor: "#adc6ff" }}
                />
              ))}
            </div>
            <div className="absolute inset-0 flex items-center justify-center">
              <span
                className="text-[10px] font-mono px-2 py-0.5 rounded"
                style={{ color: "#adc6ff", backgroundColor: "rgba(26,26,26,0.7)" }}
              >
                MONITORING...
              </span>
            </div>
          </div>
        </section>
      </div>

      {/* Footer actions */}
      <div
        className="flex gap-2 p-3"
        style={{ borderTop: "1px solid rgba(255,255,255,0.06)" }}
      >
        <button
          className="flex-1 h-8 rounded text-[10px] font-mono transition-colors"
          style={{ border: "1px solid rgba(255,255,255,0.12)", color: "var(--text-muted)" }}
        >
          RESET
        </button>
        <button
          className="flex-1 h-8 rounded text-[10px] font-mono font-bold transition-opacity hover:opacity-90"
          style={{ backgroundColor: "#adc6ff", color: "#002e6a" }}
        >
          APPLY
        </button>
      </div>
    </aside>
  );
}
