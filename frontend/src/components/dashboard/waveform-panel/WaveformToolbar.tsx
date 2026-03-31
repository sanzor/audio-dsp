import type React from "react";

interface WaveformToolbarProps {
  canCreate: boolean;
  selectedRegionId: number | undefined;
  createMode: boolean;
  onCreateClick: () => void;
  onCancelCreate: () => void;
  onEditClick: () => void;
  onDeleteClick: () => void;
  onCopyClick: () => void;
}

export function WaveformToolbar({
  canCreate,
  selectedRegionId,
  createMode,
  onCreateClick,
  onCancelCreate,
  onEditClick,
  onDeleteClick,
  onCopyClick,
}: WaveformToolbarProps) {
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

  const hasRegion = selectedRegionId != null;

  if (createMode) {
    return (
      <div
        className="relative z-10 flex shrink-0 items-center gap-2 px-3 py-1"
        style={{
          background: "var(--bg-darker)",
          borderBottom: "1px solid rgba(255,255,255,0.06)",
        }}
      >
        <span style={{ fontSize: "0.78rem", color: "var(--text-muted)" }}>
          Adjust selection then click Create
        </span>
        <button style={btnBase} onClick={onCancelCreate}>
          Cancel
        </button>
      </div>
    );
  }

  return (
    <div
      className="relative z-10 flex shrink-0 items-center gap-2 px-3 py-1"
      style={{
        background: "var(--bg-darker)",
        borderBottom: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      <button style={canCreate ? btnBase : btnDisabled} disabled={!canCreate} onClick={onCreateClick}>
        + Create Region
      </button>
      <button style={hasRegion ? btnBase : btnDisabled} disabled={!hasRegion} onClick={onEditClick}>
        Edit
      </button>
      <button style={hasRegion ? btnDanger : btnDangerDisabled} disabled={!hasRegion} onClick={onDeleteClick}>
        Delete
      </button>
      <button style={hasRegion ? btnBase : btnDisabled} disabled={!hasRegion} onClick={onCopyClick}>
        Copy
      </button>
    </div>
  );
}
