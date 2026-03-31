import { useEffect, useRef, useState } from "react";
import type { TrackRegionViewModel } from "@/domain/Region/TrackRegionViewModel";

const REGION_GAP = 0.05;
const MIN_WIDTH = 0.05;

interface Props {
  regions: TrackRegionViewModel[];
  duration: number;
  containerRef: React.RefObject<HTMLDivElement | null>;
  onConfirm?: (start: number, end: number) => void;
}

function findFirstFreeGap(
  regions: TrackRegionViewModel[],
  duration: number
): { start: number; end: number } {
  const sorted = [...regions].sort((a, b) => a.start - b.start);

  if (sorted.length === 0) return { start: 0, end: duration };

  if (sorted[0].start > MIN_WIDTH + REGION_GAP) {
    return { start: 0, end: sorted[0].start - REGION_GAP };
  }

  for (let i = 0; i < sorted.length - 1; i++) {
    const gapStart = sorted[i].end + REGION_GAP;
    const gapEnd = sorted[i + 1].start - REGION_GAP;
    if (gapEnd - gapStart >= MIN_WIDTH) return { start: gapStart, end: gapEnd };
  }

  const lastEnd = sorted[sorted.length - 1].end + REGION_GAP;
  if (duration - lastEnd >= MIN_WIDTH) return { start: lastEnd, end: duration };

  return { start: 0, end: duration };
}

// Returns the bounds of the free gap that contains both draftStart and draftEnd
function getGapBounds(
  draftStart: number,
  draftEnd: number,
  regions: TrackRegionViewModel[],
  duration: number
): { min: number; max: number } {
  const sorted = [...regions].sort((a, b) => a.start - b.start);
  let min = 0;
  let max = duration;

  for (const region of sorted) {
    if (region.end + REGION_GAP <= draftStart) {
      min = region.end + REGION_GAP;
    }
    if (region.start - REGION_GAP >= draftEnd) {
      max = region.start - REGION_GAP;
      break;
    }
  }

  return { min, max };
}

// Moves the whole selection by dx. If it crosses a region, jumps to the next free gap.
function moveBodyToGap(
  start: number,
  end: number,
  dx: number,
  regions: TrackRegionViewModel[],
  duration: number
): { start: number; end: number } {
  const width = end - start;
  let newStart = start + dx;
  let newEnd = end + dx;
  const sorted = [...regions].sort((a, b) => a.start - b.start);

  if (dx > 0) {
    const blocker = sorted.find(
      (r) => newEnd > r.start - REGION_GAP && newStart < r.end + REGION_GAP
    );
    if (blocker) {
      newStart = blocker.end + REGION_GAP;
      newEnd = newStart + width;
      const next = sorted.find((r) => r.start - REGION_GAP > newStart);
      if (next) newEnd = Math.min(newEnd, next.start - REGION_GAP);
      newEnd = Math.min(newEnd, duration);
      if (newEnd - newStart < MIN_WIDTH) return { start, end };
    }
  } else if (dx < 0) {
    const blocker = [...sorted]
      .reverse()
      .find((r) => newStart < r.end + REGION_GAP && newEnd > r.start - REGION_GAP);
    if (blocker) {
      newEnd = blocker.start - REGION_GAP;
      newStart = newEnd - width;
      const prev = [...sorted].reverse().find((r) => r.end + REGION_GAP < newEnd);
      if (prev) newStart = Math.max(newStart, prev.end + REGION_GAP);
      newStart = Math.max(newStart, 0);
      if (newEnd - newStart < MIN_WIDTH) return { start, end };
    }
  }

  newStart = Math.max(0, Math.min(newStart, duration - width));
  newEnd = Math.min(duration, newStart + width);
  if (newEnd - newStart < MIN_WIDTH) return { start, end };

  return { start: newStart, end: newEnd };
}

export function CreateRegionOverlay({ regions, duration, containerRef, onConfirm }: Props) {
  const initial = findFirstFreeGap(regions, duration);
  const [draft, setDraft] = useState(initial);
  const draftRef = useRef(initial);
  const dragState = useRef<{ type: "start" | "end" | "body"; lastX: number } | null>(null);

  useEffect(() => {
    draftRef.current = draft;
  }, [draft]);

  useEffect(() => {
    const onMove = (e: PointerEvent) => {
      const ds = dragState.current;
      const container = containerRef.current;
      if (!ds || !container || duration <= 0) return;

      const { width } = container.getBoundingClientRect();
      if (width <= 0) return;

      const dx = ((e.clientX - ds.lastX) / width) * duration;
      ds.lastX = e.clientX;

      const { start, end } = draftRef.current;
      const gap = getGapBounds(start, end, regions, duration);

      let next: { start: number; end: number };
      if (ds.type === "start") {
        next = { start: Math.max(gap.min, Math.min(start + dx, end - MIN_WIDTH)), end };
      } else if (ds.type === "end") {
        next = { start, end: Math.min(gap.max, Math.max(end + dx, start + MIN_WIDTH)) };
      } else {
        next = moveBodyToGap(start, end, dx, regions, duration);
      }

      draftRef.current = next;
      setDraft(next);
    };

    const onUp = () => { dragState.current = null; };

    window.addEventListener("pointermove", onMove);
    window.addEventListener("pointerup", onUp);
    return () => {
      window.removeEventListener("pointermove", onMove);
      window.removeEventListener("pointerup", onUp);
    };
  }, [containerRef, regions, duration]);

  const startPct = (draft.start / duration) * 100;
  const endPct = (draft.end / duration) * 100;
  const widthPct = endPct - startPct;

  const onHandleDown = (type: "start" | "end" | "body") => (e: React.PointerEvent) => {
    e.preventDefault();
    e.stopPropagation();
    dragState.current = { type, lastX: e.clientX };
  };

  return (
    <>
      {/* Fill */}
      <div
        className="pointer-events-none absolute inset-y-0"
        style={{
          left: `${startPct}%`,
          width: `${widthPct}%`,
          background: "rgba(99,179,237,0.15)",
          zIndex: 20,
        }}
      />
      {/* Start cursor line */}
      <div
        className="pointer-events-none absolute inset-y-0"
        style={{ left: `${startPct}%`, width: 2, background: "#63b3ed", zIndex: 21 }}
      />
      {/* End cursor line */}
      <div
        className="pointer-events-none absolute inset-y-0"
        style={{ left: `${endPct}%`, width: 2, background: "#63b3ed", transform: "translateX(-2px)", zIndex: 21 }}
      />
      {/* Body drag area */}
      <div
        className="absolute inset-y-0 cursor-move"
        style={{ left: `${startPct}%`, width: `${widthPct}%`, zIndex: 22 }}
        onPointerDown={onHandleDown("body")}
      />
      {/* Start handle hit area */}
      <div
        className="absolute inset-y-0 cursor-ew-resize"
        style={{ left: `calc(${startPct}% - 5px)`, width: 10, zIndex: 23 }}
        onPointerDown={onHandleDown("start")}
      />
      {/* End handle hit area */}
      <div
        className="absolute inset-y-0 cursor-ew-resize"
        style={{ left: `calc(${endPct}% - 5px)`, width: 10, zIndex: 23 }}
        onPointerDown={onHandleDown("end")}
      />
      {/* Confirm button centered over selection */}
      {onConfirm && (
        <button
          style={{
            position: "absolute",
            top: 6,
            left: `calc(${startPct + widthPct / 2}% - 28px)`,
            zIndex: 24,
            padding: "2px 10px",
            fontSize: "0.72rem",
            borderRadius: 4,
            background: "#3b82f6",
            color: "white",
            border: "none",
            cursor: "pointer",
          }}
          onPointerDown={(e) => e.stopPropagation()}
          onClick={() => onConfirm(draft.start, draft.end)}
        >
          Create
        </button>
      )}
    </>
  );
}
