import type React from "react";
import type { GraphPlaybackState } from "@/Stores/AudioEffectsStore";

interface CanvasToolbarProps {
  selectedGraphId: number | undefined;
  hasClearableNodes: boolean;
  isSaving: boolean;
  effectsEnabled: boolean;
  graphPlaybackState: GraphPlaybackState;
  onCompile: () => void;
  onSave: () => void;
  onToggleEffects: () => void;
  onFitView: () => void;
  onClearNodes: () => void;
  onRename: () => void;
  onDelete: () => void;
  onCopy: () => void;
}

export function CanvasToolbar({
  selectedGraphId,
  hasClearableNodes,
  isSaving,
  effectsEnabled,
  graphPlaybackState,
  onCompile,
  onSave,
  onToggleEffects,
  onFitView,
  onClearNodes,
  onRename,
  onDelete,
  onCopy,
}: CanvasToolbarProps) {
  const hasGraph = selectedGraphId != null;

  const btnBase: React.CSSProperties = {
    padding: "3px 10px",
    fontSize: "0.78rem",
    borderRadius: 4,
    border: "1px solid rgba(255,255,255,0.12)",
    cursor: "pointer",
    background: "var(--bg-dark)",
    color: "var(--text-main)",
  };

  const btnDisabled: React.CSSProperties = {
    ...btnBase,
    opacity: 0.35,
    cursor: "not-allowed",
  };

  const btnDanger: React.CSSProperties = {
    ...btnBase,
    color: "#f87171",
    borderColor: "rgba(248,113,113,0.3)",
  };

  const btnDangerDisabled: React.CSSProperties = {
    ...btnDanger,
    opacity: 0.35,
    cursor: "not-allowed",
  };

  const btnActive: React.CSSProperties = {
    ...btnBase,
    color: "#dcfce7",
    borderColor: "rgba(34,197,94,0.5)",
    background: "rgba(22,163,74,0.18)",
    boxShadow: "0 0 0 1px rgba(34,197,94,0.18) inset",
  };

  const statusBase: React.CSSProperties = {
    padding: "3px 10px",
    fontSize: "0.72rem",
    borderRadius: 999,
    border: "1px solid rgba(255,255,255,0.12)",
    background: "rgba(255,255,255,0.04)",
    color: "var(--text-muted)",
    letterSpacing: "0.02em",
  };

  const statusOk: React.CSSProperties = {
    ...statusBase,
    color: "#dcfce7",
    borderColor: "rgba(34,197,94,0.45)",
    background: "rgba(22,163,74,0.18)",
  };

  const statusBad: React.CSSProperties = {
    ...statusBase,
    color: "#fecaca",
    borderColor: "rgba(248,113,113,0.38)",
    background: "rgba(185,28,28,0.18)",
  };

  const canClear = hasGraph && hasClearableNodes;

  return (
    <div
      className="relative z-10 flex shrink-0 items-center gap-2 px-3 py-1"
      style={{
        background: "var(--bg-darker)",
        borderBottom: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      {hasGraph ? (
        <>
          <div style={graphPlaybackState.compiled ? statusOk : statusBad}>
            {graphPlaybackState.compiled ? "Compiled" : "Not Compiled"}
          </div>
          <div style={graphPlaybackState.playable ? statusOk : statusBad}>
            {graphPlaybackState.playable ? "Playable" : "Not Playable"}
          </div>
        </>
      ) : (
        <div style={statusBase}>No Graph</div>
      )}
      <button
        style={hasGraph ? (effectsEnabled ? btnActive : btnBase) : btnDisabled}
        disabled={!hasGraph}
        onClick={onToggleEffects}
      >
        {effectsEnabled ? "Effects Off" : "Effects On"}
      </button>
      <button
        style={hasGraph ? (graphPlaybackState.compiled ? btnActive : btnBase) : btnDisabled}
        disabled={!hasGraph}
        onClick={onCompile}
      >
        Compile
      </button>
      <button style={hasGraph && !isSaving ? btnBase : btnDisabled} disabled={!hasGraph || isSaving} onClick={onSave}>
        {isSaving ? "Saving..." : "Save"}
      </button>
      <button style={hasGraph ? btnBase : btnDisabled} disabled={!hasGraph} onClick={onFitView}>
        Fit View
      </button>
      <button style={canClear ? btnBase : btnDisabled} disabled={!canClear} onClick={onClearNodes}>
        Clear Nodes
      </button>
      <button style={hasGraph ? btnBase : btnDisabled} disabled={!hasGraph} onClick={onRename}>
        Rename
      </button>
      <button style={hasGraph ? btnDanger : btnDangerDisabled} disabled={!hasGraph} onClick={onDelete}>
        Delete
      </button>
      <button style={hasGraph ? btnBase : btnDisabled} disabled={!hasGraph} onClick={onCopy}>
        Copy
      </button>
    </div>
  );
}
