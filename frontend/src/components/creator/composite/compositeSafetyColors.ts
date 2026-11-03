import type { NodeDisableSafety } from "./compositeReachability";

// Three-state border/dot color for the enable/disable control (Phase 3):
// safe-to-disable (green) / load-bearing (amber — disabling breaks
// connectivity from an exposed input to an exposed output) / currently
// disabled (dim gray). Independent of node selection highlighting.
export const SAFETY_COLOR: Record<NodeDisableSafety, string> = {
  safe: "#4ae176",
  "load-bearing": "#ffd166",
  disabled: "#6b7280",
};
