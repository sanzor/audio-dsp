import type { TrackRegionSetViewModel } from "@/domain/RegionSet/TrackRegionSetViewModel";

interface WaveformToolbarProps {
  regionSet: TrackRegionSetViewModel | undefined;
  selectedRegionId: number | undefined;
  createMode: boolean;
  onCreateClick: () => void;
  onCancelCreate: () => void;
  onEditClick: () => void;
  onDeleteClick: () => void;
  onCopyClick: () => void;
}

export function WaveformToolbar({
  regionSet,
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

  const btnDanger: React.CSSProperties = {
    ...btnBase,
    color: "#f87171",
    borderColor: "rgba(248,113,113,0.3)",
  };

  if (createMode) {
    return (
      <div
        className="flex items-center gap-2 px-3 py-1 shrink-0"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
      >
        <span style={{ fontSize: "0.78rem", color: "var(--text-muted)" }}>
          Drag on waveform to define region bounds
        </span>
        <button style={btnBase} onClick={onCancelCreate}>
          Cancel
        </button>
      </div>
    );
  }

  return (
    <div
      className="flex items-center gap-2 px-3 py-1 shrink-0"
      style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
    >
      <button style={btnBase} disabled={!regionSet} onClick={onCreateClick}>
        + Create Region
      </button>
      {selectedRegionId != null && (
        <>
          <button style={btnBase} onClick={onEditClick}>
            Edit
          </button>
          <button style={btnDanger} onClick={onDeleteClick}>
            Delete
          </button>
          <button style={btnBase} onClick={onCopyClick}>
            Copy
          </button>
        </>
      )}
    </div>
  );
}
