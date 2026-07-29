import { useCreatorPreviewStore } from "@/Stores/CreatorPreviewStore";

export function CreatorStatusBar() {
  const status = useCreatorPreviewStore((s) => s.status);
  const cpuLoadPct = useCreatorPreviewStore((s) => s.cpuLoadPct);
  const latencyMs = useCreatorPreviewStore((s) => s.latencyMs);

  const isPlaying = status === "playing";
  const engineColor = isPlaying ? "#4ae176" : status === "error" ? "#ff6b6b" : "var(--text-muted)";
  const engineLabel = isPlaying ? "PREVIEWING" : status === "loading" ? "LOADING…" : status === "error" ? "ERROR" : "IDLE";

  return (
    <footer
      className="h-6 flex items-center justify-between px-4 text-[10px] font-mono select-none flex-shrink-0"
      style={{
        backgroundColor: "var(--bg-darkest)",
        borderTop: "1px solid rgba(255,255,255,0.06)",
        color: "var(--text-muted)",
      }}
    >
      <div className="flex items-center gap-6">
        <div className="flex items-center gap-1.5">
          <span style={{ color: engineColor }}>●</span>
          <span>ENGINE: {engineLabel}</span>
        </div>
        <span>LATENCY: {isPlaying && latencyMs != null ? `${latencyMs.toFixed(1)}ms` : "—"}</span>
        <span>CPU: {isPlaying && cpuLoadPct != null ? `${cpuLoadPct.toFixed(0)}%` : "—"}</span>
      </div>
      <div className="flex items-center gap-4">
        <span>UTF-8</span>
        <span
          className="font-bold px-2 py-0.5 rounded text-[10px]"
          style={{ color: "#ffb786", border: "1px solid rgba(255,183,134,0.3)" }}
        >
          RUST (WASM)
        </span>
      </div>
    </footer>
  );
}
