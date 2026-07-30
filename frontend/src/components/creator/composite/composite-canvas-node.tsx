// This module exports the node-type map and data shape alongside the
// component itself (react-flow's nodeTypes pattern) — not component-only,
// so Fast Refresh can't isolate it.
/* eslint-disable react-refresh/only-export-components */
import { Handle, Position, type NodeProps } from "reactflow";
import { useCompositeCanvasStore } from "@/Stores/CompositeCanvasStore";
import { useTransformStore } from "@/Stores/TransformStore";
import type { NodeDisableSafety } from "./compositeReachability";

const ROW_HEIGHT = 28;

// ─── Node ─────────────────────────────────────────────────────────────────────
// Generalizes canvas.tsx's TransformPreviewNode: multiple instances, real
// per-port Handles keyed by port NAME (not port_id, since composite wiring
// references ports by name — port_id is reassigned on every leaf republish).

export interface CompositeNodeData {
  nodeId: number;
  transformId: number;
  disabled: boolean;
  safety: NodeDisableSafety;
}

// Three-state border/dot color for the enable/disable control (Phase 3):
// safe-to-disable (green) / load-bearing (amber — disabling breaks
// connectivity from an exposed input to an exposed output) / currently
// disabled (dim gray). Independent of node selection highlighting.
const SAFETY_COLOR: Record<NodeDisableSafety, string> = {
  safe: "#4ae176",
  "load-bearing": "#ffd166",
  disabled: "#6b7280",
};

const SAFETY_LABEL: Record<NodeDisableSafety, string> = {
  safe: "Safe to disable — no exposed input/output path depends on this node",
  "load-bearing": "Disabling this node would break connectivity from an exposed input to an exposed output",
  disabled: "Currently disabled — excluded from Save/Publish and Preview/Play",
};

function CompositeTransformNode({ data }: NodeProps<CompositeNodeData>) {
  // Reads from the cache useResolveTransformDefinitions (in the parent
  // canvas) already populated for every node's transform — no per-node fetch.
  const definition = useTransformStore((s) => s.definitions.get(data.transformId));
  const removeNode = useCompositeCanvasStore((s) => s.removeNode);
  const selectNode = useCompositeCanvasStore((s) => s.selectNode);
  const toggleNodeDisabled = useCompositeCanvasStore((s) => s.toggleNodeDisabled);
  const selectedNodeId = useCompositeCanvasStore((s) => s.selectedNodeId);

  const inputs = definition?.ports.filter((p) => p.direction === "input") ?? [];
  const outputs = definition?.ports.filter((p) => p.direction === "output") ?? [];
  const rows = Math.max(inputs.length, outputs.length, 1);
  const height = rows * ROW_HEIGHT + 48;
  const isSelected = selectedNodeId === data.nodeId;
  const borderColor = isSelected ? "#adc6ff" : SAFETY_COLOR[data.safety];

  return (
    <div
      onClick={() => selectNode(data.nodeId)}
      title="View source and details"
      style={{
        width: 220,
        height,
        backgroundColor: "#1e1e1e",
        border: `2px solid ${borderColor}`,
        borderRadius: 8,
        boxShadow: isSelected ? "0 0 24px rgba(173,198,255,0.12)" : "none",
        opacity: data.disabled ? 0.5 : 1,
        display: "flex",
        flexDirection: "column",
        overflow: "hidden",
        position: "relative",
        cursor: "pointer",
      }}
    >
      <div
        style={{
          padding: "6px 10px",
          borderBottom: "1px solid rgba(255,255,255,0.08)",
          fontSize: 11,
          fontFamily: "JetBrains Mono, monospace",
          color: "#adc6ff",
          fontWeight: 700,
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          flexShrink: 0,
          gap: 6,
        }}
      >
        <span className="truncate" style={{ flex: 1, minWidth: 0 }}>
          {definition?.name ?? `#${data.nodeId}`}
        </span>
        <button
          onClick={(e) => {
            e.stopPropagation();
            toggleNodeDisabled(data.nodeId);
          }}
          title={SAFETY_LABEL[data.safety]}
          style={{
            width: 9,
            height: 9,
            borderRadius: "50%",
            backgroundColor: SAFETY_COLOR[data.safety],
            border: "none",
            cursor: "pointer",
            padding: 0,
            flexShrink: 0,
          }}
        />
        <button
          onClick={(e) => {
            e.stopPropagation();
            removeNode(data.nodeId);
          }}
          style={{ color: "#ff6b6b", background: "none", border: "none", cursor: "pointer", fontSize: 12, flexShrink: 0 }}
          title="Remove from composite"
        >
          ×
        </button>
      </div>

      <div style={{ flex: 1, display: "flex", position: "relative" }}>
        <div style={{ flex: 1, display: "flex", flexDirection: "column", paddingTop: 4 }}>
          {inputs.map((port) => (
            <div key={port.name} style={{ height: ROW_HEIGHT, display: "flex", alignItems: "center", paddingLeft: 14, position: "relative" }}>
              <Handle
                type="target"
                position={Position.Left}
                id={`in-${port.name}`}
                style={{ left: -1, top: "50%", transform: "translateY(-50%)", width: 8, height: 8, backgroundColor: "#adc6ff", border: "none" }}
              />
              <span style={{ fontSize: 10, fontFamily: "JetBrains Mono, monospace", color: "#adc6ff", opacity: 0.8 }}>{port.name}</span>
            </div>
          ))}
        </div>
        <div style={{ flex: 1, display: "flex", flexDirection: "column", paddingTop: 4 }}>
          {outputs.map((port) => (
            <div key={port.name} style={{ height: ROW_HEIGHT, display: "flex", alignItems: "center", justifyContent: "flex-end", paddingRight: 14, position: "relative" }}>
              <span style={{ fontSize: 10, fontFamily: "JetBrains Mono, monospace", color: "#4ae176", opacity: 0.8 }}>{port.name}</span>
              <Handle
                type="source"
                position={Position.Right}
                id={`out-${port.name}`}
                style={{ right: -1, top: "50%", transform: "translateY(-50%)", width: 8, height: 8, backgroundColor: "#4ae176", border: "none" }}
              />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export const NODE_TYPES = { composite: CompositeTransformNode };
