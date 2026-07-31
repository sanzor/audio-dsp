// This module exports the node-type map and data shape alongside the
// component itself (react-flow's nodeTypes pattern) — not component-only,
// so Fast Refresh can't isolate it.
/* eslint-disable react-refresh/only-export-components */
import { type NodeProps } from "reactflow";
import { useCompositeCanvasStore } from "@/Stores/CompositeCanvasStore";
import { useTransformStore } from "@/Stores/TransformStore";
import type { NodeDisableSafety } from "./compositeReachability";
import { NodeHeader, SAFETY_COLOR } from "./composite-node-header";
import { PortColumn, ROW_HEIGHT } from "./composite-node-ports";

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
      <NodeHeader
        title={definition?.name ?? `#${data.nodeId}`}
        safety={data.safety}
        onToggleDisabled={() => toggleNodeDisabled(data.nodeId)}
        onRemove={() => removeNode(data.nodeId)}
      />

      <div style={{ flex: 1, display: "flex", position: "relative" }}>
        <PortColumn direction="input" ports={inputs} />
        <PortColumn direction="output" ports={outputs} />
      </div>
    </div>
  );
}

export const NODE_TYPES = { composite: CompositeTransformNode };
