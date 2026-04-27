import type React from "react";

interface CanvasToolbarProps {
  selectedGraphId: number | undefined;
  hasClearableNodes: boolean;
  onSave: () => void;
  onFitView: () => void;
  onClearNodes: () => void;
  onRename: () => void;
  onDelete: () => void;
  onCopy: () => void;
}

export function CanvasToolbar({
  selectedGraphId,
  hasClearableNodes,
  onSave,
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

  const canClear = hasGraph && hasClearableNodes;

  return (
    <div
      className="relative z-10 flex shrink-0 items-center gap-2 px-3 py-1"
      style={{
        background: "var(--bg-darker)",
        borderBottom: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      <button style={hasGraph ? btnBase : btnDisabled} disabled={!hasGraph} onClick={onSave}>
        Save
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
