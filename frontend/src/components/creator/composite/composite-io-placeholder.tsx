// Client-side-only "ghost slot" onboarding affordance for the composite
// canvas — shown when the composite currently has zero exposed ports in a
// given direction, so a fresh (or still-unwired) composite doesn't read as
// a blank canvas with no hint of where audio enters/exits. Purely visual:
// not a CanvasNode/CompositeNode, not draggable, not connectable, never
// touches editingGraph or toGraphDefinition()'s output. See
// composite-canvas.tsx for the exposed-port-direction derivation that
// decides when each slot is shown/hidden.
//
// Rendered as a React Flow `Panel` — viewport-anchored (top-left/top-right
// corner of the canvas), not canvas-coordinate-anchored like a real node —
// specifically so it can never be mistaken for a draggable/connectable
// node: no Handles, no position in flow-space, no entry in `nodes`.
// `pointerEvents: "none"` keeps it a pure static hint (v1 scope — an
// interactive "click to create" control is a possible v2, not this) and
// lets drag/drop of a palette transform pass through to the canvas
// underneath rather than being intercepted by the hint.

import { Panel } from "reactflow";
import { creatorToolbarColors } from "../creatorToolbarColors";

export type IoDirection = "input" | "output";

const COPY: Record<IoDirection, { label: string; hint: string }> = {
  input: {
    label: "INPUT",
    hint: "Drag a transform in, then expose one of its input ports here to create this composite's input.",
  },
  output: {
    label: "OUTPUT",
    hint: "Drag a transform in, then expose one of its output ports here to create this composite's output.",
  },
};

interface CompositeIoGhostSlotProps {
  direction: IoDirection;
}

export function CompositeIoGhostSlot({ direction }: CompositeIoGhostSlotProps) {
  const { label, hint } = COPY[direction];
  return (
    <Panel position={direction === "input" ? "top-left" : "top-right"}>
      <div
        style={{
          width: 180,
          padding: "10px 12px",
          borderRadius: 8,
          border: "1.5px dashed rgba(173,198,255,0.35)",
          backgroundColor: "rgba(173,198,255,0.04)",
          pointerEvents: "none",
          userSelect: "none",
        }}
      >
        <span
          style={{
            fontFamily: "JetBrains Mono, monospace",
            fontSize: 10,
            fontWeight: 700,
            letterSpacing: 1,
            color: creatorToolbarColors.blue.color,
            opacity: 0.6,
          }}
        >
          {label}
        </span>
        <p style={{ margin: "4px 0 0", fontSize: 9, lineHeight: 1.4, color: "var(--text-muted)", opacity: 0.7 }}>
          {hint}
        </p>
      </div>
    </Panel>
  );
}
