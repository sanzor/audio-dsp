export function CreatorStatusBar() {
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
          <span style={{ color: "#4ae176" }}>●</span>
          <span>ENGINE: CONNECTED (LOCAL)</span>
        </div>
        <span>LATENCY: 4.2ms</span>
        <span>CPU: 12%</span>
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
