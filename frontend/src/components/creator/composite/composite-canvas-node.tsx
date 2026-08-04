// This module exports the node-type map and data shape alongside the
// component itself (react-flow's nodeTypes pattern) — not component-only,
// so Fast Refresh can't isolate it.
/* eslint-disable react-refresh/only-export-components */
import { useState } from "react";
import { Handle, Position, type NodeProps } from "reactflow";
import { useCompositeCanvasStore } from "@/Stores/CompositeCanvasStore";
import { useTransformStore } from "@/Stores/TransformStore";
import { IO_NODE_PORT_NAME } from "@/domain/Transform/CompositeGraphDefinition";
import type { NodeDisableSafety } from "./compositeReachability";
import { NodeHeader, SAFETY_COLOR } from "./composite-node-header";
import { PortColumn, ROW_HEIGHT } from "./composite-node-ports";
import { RemoveNodeButton } from "./composite-node-remove-button";

// ─── Node ─────────────────────────────────────────────────────────────────────
// Generalizes canvas.tsx's TransformCanvasNode: multiple instances, real
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
      title="View source and details"
      style={{
        width: 220,
        height,
        opacity: data.disabled ? 0.5 : 1,
        position: "relative",
        cursor: "pointer",
      }}
    >
      {/* Clipped content wrapper: owns the rounded-corner overflow: hidden
          clipping for the header + port-column content. The remove badge
          below is a SIBLING of this wrapper (not nested inside it) so it can
          be positioned outside the card's border without being cut off. */}
      <div
        onClick={() => selectNode(data.nodeId)}
        style={{
          width: "100%",
          height: "100%",
          backgroundColor: "#1e1e1e",
          border: `2px solid ${borderColor}`,
          borderRadius: 8,
          boxShadow: isSelected ? "0 0 24px rgba(173,198,255,0.12)" : "none",
          display: "flex",
          flexDirection: "column",
          overflow: "hidden",
        }}
      >
        <NodeHeader
          title={definition?.name ?? `#${data.nodeId}`}
          safety={data.safety}
          onToggleDisabled={() => toggleNodeDisabled(data.nodeId)}
        />

        <div style={{ flex: 1, display: "flex", position: "relative" }}>
          <PortColumn direction="input" ports={inputs} />
          <PortColumn direction="output" ports={outputs} />
        </div>
      </div>

      <RemoveNodeButton onRemove={() => removeNode(data.nodeId)} />
    </div>
  );
}

// ─── IO node ──────────────────────────────────────────────────────────────────
// A composite's own Input/Output boundary nodes — one implicit signal handle
// each (a fixed pseudo-port named IO_NODE_PORT_NAME, never a real
// TransformPort since these aren't transforms). Editable name (double-click
// to rename via renameIoNode), no source/details inspector since there's no
// backing TransformDefinition to show.

export interface CompositeIoNodeData {
  nodeId: number;
  direction: "input" | "output";
  name: string;
}

function CompositeIoNode({ data }: NodeProps<CompositeIoNodeData>) {
  const removeNode = useCompositeCanvasStore((s) => s.removeNode);
  const renameIoNode = useCompositeCanvasStore((s) => s.renameIoNode);
  const [isEditing, setIsEditing] = useState(false);
  const [draftName, setDraftName] = useState(data.name);

  const isInput = data.direction === "input";
  const color = isInput ? "#adc6ff" : "#4ae176";
  const label = isInput ? "INPUT" : "OUTPUT";

  function startEditing() {
    setDraftName(data.name);
    setIsEditing(true);
  }

  function commit() {
    setIsEditing(false);
    const trimmed = draftName.trim();
    if (trimmed && trimmed !== data.name) renameIoNode(data.nodeId, trimmed);
  }

  return (
    <div
      style={{
        width: 160,
        backgroundColor: "#1e1e1e",
        border: `2px solid ${color}`,
        borderRadius: 8,
        position: "relative",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <div
        style={{
          padding: "6px 10px",
          borderBottom: "1px solid rgba(255,255,255,0.08)",
          fontSize: 10,
          fontFamily: "JetBrains Mono, monospace",
          color,
          fontWeight: 700,
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          flexShrink: 0,
          gap: 6,
        }}
      >
        <span>{label}</span>
      </div>
      <RemoveNodeButton onRemove={() => removeNode(data.nodeId)} />
      <div style={{ padding: "10px 12px", display: "flex", alignItems: "center", minHeight: 32 }}>
        {isEditing ? (
          <input
            autoFocus
            value={draftName}
            onChange={(e) => setDraftName(e.target.value)}
            onBlur={commit}
            onKeyDown={(e) => {
              if (e.key === "Enter") commit();
              if (e.key === "Escape") {
                setDraftName(data.name);
                setIsEditing(false);
              }
            }}
            style={{
              width: "100%",
              background: "var(--bg-darker)",
              border: "1px solid rgba(255,255,255,0.15)",
              borderRadius: 4,
              color: "var(--text-main)",
              fontSize: 11,
              fontFamily: "JetBrains Mono, monospace",
              padding: "2px 6px",
            }}
          />
        ) : (
          <span
            onDoubleClick={startEditing}
            title="Double-click to rename"
            className="truncate"
            style={{ fontSize: 11, fontFamily: "JetBrains Mono, monospace", color: "var(--text-main)", cursor: "text" }}
          >
            {data.name}
          </span>
        )}
      </div>
      {isInput ? (
        <Handle
          type="source"
          position={Position.Right}
          id={`out-${IO_NODE_PORT_NAME}`}
          style={{ right: -1, top: "50%", transform: "translateY(-50%)", width: 8, height: 8, backgroundColor: color, border: "none" }}
        />
      ) : (
        <Handle
          type="target"
          position={Position.Left}
          id={`in-${IO_NODE_PORT_NAME}`}
          style={{ left: -1, top: "50%", transform: "translateY(-50%)", width: 8, height: 8, backgroundColor: color, border: "none" }}
        />
      )}
    </div>
  );
}

export const NODE_TYPES = { composite: CompositeTransformNode, compositeIo: CompositeIoNode };
