import { useRef, useState } from "react";
import type { TrackRegionViewModel } from "@/domain/Region/TrackRegionViewModel";

const REGION_GAP = 0.05;
const MIN_WIDTH = 0.01;
const HANDLE_PX = 12;

interface Props {
  regions: TrackRegionViewModel[];
  duration: number;
  containerRef: React.RefObject<HTMLDivElement | null>;
  onConfirm?: (start: number, end: number) => void;
  onDiscard?: () => void;
}

function findFirstFreeGap(regions: TrackRegionViewModel[], duration: number): { start: number; end: number } {
  const sorted = [...regions].sort((a, b) => a.start - b.start);
  if (sorted.length === 0) return { start: 0, end: duration };
  if (sorted[0].start > MIN_WIDTH + REGION_GAP) return { start: 0, end: sorted[0].start - REGION_GAP };
  for (let i = 0; i < sorted.length - 1; i++) {
    const gs = sorted[i].end + REGION_GAP;
    const ge = sorted[i + 1].start - REGION_GAP;
    if (ge - gs >= MIN_WIDTH) return { start: gs, end: ge };
  }
  const lastEnd = sorted[sorted.length - 1].end + REGION_GAP;
  if (duration - lastEnd >= MIN_WIDTH) return { start: lastEnd, end: duration };
  return { start: 0, end: duration };
}

function hasOverlap(start: number, end: number, regions: TrackRegionViewModel[]): boolean {
  return regions.some((r) => start < r.end && end > r.start);
}

function clampToFreeInterval(
  start: number, end: number, regions: TrackRegionViewModel[], duration: number
): { start: number; end: number } {
  const sorted = [...regions].sort((a, b) => a.start - b.start);
  let gapStart = 0;
  let gapEnd = duration;
  for (const region of sorted) {
    if (region.end + REGION_GAP <= start) { gapStart = region.end + REGION_GAP; }
    else if (region.start - REGION_GAP >= start) { gapEnd = region.start - REGION_GAP; break; }
    else { return findFirstFreeGap(regions, duration); }
  }
  const cs = Math.max(gapStart, start);
  const ce = Math.min(gapEnd, end);
  if (ce - cs < MIN_WIDTH) return findFirstFreeGap(regions, duration);
  return { start: cs, end: ce };
}

function fmt(s: number): string {
  const m = Math.floor(s / 60);
  const sec = (s % 60).toFixed(2).padStart(5, "0");
  return m > 0 ? `${m}:${sec}` : `${sec}s`;
}

type DragType = "start" | "end" | "body";

export function CreateRegionOverlay({ regions, duration, containerRef, onConfirm, onDiscard }: Props) {
  const initial = findFirstFreeGap(regions, duration);
  const [draft, setDraft] = useState(initial);
  const draftRef = useRef(initial);
  // When conflicting, holds the original draft so Cancel can restore it
  const savedDraft = useRef<{ start: number; end: number } | null>(null);

  const dragType = useRef<DragType | null>(null);
  const lastX = useRef(0);
  const [isDragging, setIsDragging] = useState(false);
  const [conflicting, setConflicting] = useState(false);

  const pxToTime = (px: number): number => {
    const w = containerRef.current?.getBoundingClientRect().width ?? 1;
    return (px / w) * duration;
  };

  const onCaptureMove = (e: React.PointerEvent<HTMLDivElement>) => {
    if (!dragType.current) return;
    const dx = pxToTime(e.clientX - lastX.current);
    lastX.current = e.clientX;
    const { start, end } = draftRef.current;
    const w = end - start;
    let next: { start: number; end: number };
    if (dragType.current === "start") {
      next = { start: Math.max(0, Math.min(start + dx, end - MIN_WIDTH)), end };
    } else if (dragType.current === "end") {
      next = { start, end: Math.min(duration, Math.max(end + dx, start + MIN_WIDTH)) };
    } else {
      const ns = Math.max(0, Math.min(start + dx, duration - w));
      next = { start: ns, end: Math.min(duration, ns + w) };
    }
    draftRef.current = next;
    setDraft(next);
    setConflicting(false);
    savedDraft.current = null;
  };

  const onCaptureUp = () => { dragType.current = null; setIsDragging(false); };

  const startDrag = (type: DragType, clientX: number) => {
    dragType.current = type;
    lastX.current = clientX;
    setIsDragging(true);
  };

  const handleCreate = () => {
    if (hasOverlap(draft.start, draft.end, regions)) {
      // Compute clamped bounds and show them as a preview
      const clamped = clampToFreeInterval(draft.start, draft.end, regions, duration);
      savedDraft.current = { ...draft }; // remember original for Cancel
      draftRef.current = clamped;
      setDraft(clamped);
      setConflicting(true);
      return;
    }
    onConfirm?.(draft.start, draft.end);
  };

  const handleConfirmClamped = () => {
    savedDraft.current = null;
    setConflicting(false);
    onConfirm?.(draft.start, draft.end);
  };

  const handleCancelConflict = () => {
    // Restore original selection, stay in create mode
    if (savedDraft.current) {
      draftRef.current = savedDraft.current;
      setDraft(savedDraft.current);
      savedDraft.current = null;
    }
    setConflicting(false);
  };

  const startPct = (draft.start / duration) * 100;
  const endPct   = (draft.end   / duration) * 100;
  const widthPct = endPct - startPct;

  return (
    <>
      {isDragging && (
        <div
          style={{ position: "fixed", inset: 0, zIndex: 9999, cursor: "grabbing" }}
          onPointerMove={onCaptureMove}
          onPointerUp={onCaptureUp}
          onPointerCancel={onCaptureUp}
        />
      )}

      {/* Fill */}
      <div
        className="pointer-events-none absolute inset-y-0"
        style={{
          left: `${startPct}%`, width: `${widthPct}%`,
          background: conflicting ? "rgba(239,68,68,0.18)" : "rgba(99,179,237,0.15)",
          zIndex: 20,
        }}
      />
      {/* Start line */}
      <div className="pointer-events-none absolute inset-y-0"
        style={{ left: `${startPct}%`, width: 2, background: conflicting ? "#ef4444" : "#63b3ed", zIndex: 21 }} />
      {/* End line */}
      <div className="pointer-events-none absolute inset-y-0"
        style={{ left: `${endPct}%`, width: 2, background: conflicting ? "#ef4444" : "#63b3ed", transform: "translateX(-2px)", zIndex: 21 }} />

      {/* Body */}
      <div
        className="absolute inset-y-0 cursor-move"
        style={{ left: `${startPct}%`, width: `${widthPct}%`, zIndex: 22 }}
        onPointerDown={(e) => { e.preventDefault(); e.stopPropagation(); startDrag("body", e.clientX); }}
      />
      {/* Start handle */}
      <div
        className="absolute inset-y-0 cursor-ew-resize"
        style={{ left: `calc(${startPct}% - ${HANDLE_PX / 2}px)`, width: HANDLE_PX, zIndex: 23 }}
        onPointerDown={(e) => { e.preventDefault(); e.stopPropagation(); startDrag("start", e.clientX); }}
      />
      {/* End handle */}
      <div
        className="absolute inset-y-0 cursor-ew-resize"
        style={{ left: `calc(${endPct}% - ${HANDLE_PX / 2}px)`, width: HANDLE_PX, zIndex: 23 }}
        onPointerDown={(e) => { e.preventDefault(); e.stopPropagation(); startDrag("end", e.clientX); }}
      />

      {conflicting ? (
        <div
          style={{
            position: "absolute", top: 6,
            left: `calc(${startPct + widthPct / 2}% - 130px)`,
            zIndex: 100, display: "flex", alignItems: "center", gap: 6,
            background: "#1e1e2e", border: "1px solid #ef4444", borderRadius: 6,
            padding: "3px 8px", fontSize: "0.72rem", color: "#ef4444", whiteSpace: "nowrap",
          }}
          onPointerDown={(e) => e.stopPropagation()}
        >
          <span>Adjusted: {fmt(draft.start)} – {fmt(draft.end)}</span>
          <button onClick={handleConfirmClamped}
            style={{ background: "#ef4444", color: "white", border: "none", borderRadius: 4, padding: "2px 8px", cursor: "pointer", fontSize: "0.72rem" }}>
            Create
          </button>
          <button onClick={handleCancelConflict}
            style={{ background: "transparent", color: "#aaa", border: "1px solid #444", borderRadius: 4, padding: "2px 8px", cursor: "pointer", fontSize: "0.72rem" }}>
            Cancel
          </button>
          <button onClick={onDiscard}
            style={{ background: "transparent", color: "#888", border: "1px solid #333", borderRadius: 4, padding: "2px 8px", cursor: "pointer", fontSize: "0.72rem" }}>
            Discard
          </button>
        </div>
      ) : (
        <div
          style={{
            position: "absolute", top: 6,
            left: `calc(${startPct + widthPct / 2}% - 52px)`,
            zIndex: 24, display: "flex", gap: 4,
          }}
          onPointerDown={(e) => e.stopPropagation()}
        >
          <button
            onClick={handleCreate}
            style={{ padding: "2px 10px", fontSize: "0.72rem", borderRadius: 4, background: "#3b82f6", color: "white", border: "none", cursor: "pointer" }}
          >
            Create
          </button>
          <button
            onClick={onDiscard}
            style={{ padding: "2px 10px", fontSize: "0.72rem", borderRadius: 4, background: "transparent", color: "#aaa", border: "1px solid #444", cursor: "pointer" }}
          >
            Discard
          </button>
        </div>
      )}
    </>
  );
}
