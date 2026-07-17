import ReactFlow, {
  Background,
  BackgroundVariant,
  ReactFlowProvider,
  type NodeProps,
} from "reactflow";
import { Handle, Position } from "reactflow";
import "reactflow/dist/style.css";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";
import type { TransformPort } from "@/domain/Transform/TransformPort";


// ─── Custom single-transform node ────────────────────────────────────────────

interface TransformNodeData {
  name: string;
  inputs: TransformPort[];
  outputs: TransformPort[];
}

function TransformPreviewNode({ data }: NodeProps<TransformNodeData>) {
  const rowHeight = 28;
  const rows = Math.max(data.inputs.length, data.outputs.length, 1);
  const height = rows * rowHeight + 48;

  return (
    <div
      style={{
        width: 240,
        height,
        backgroundColor: "#1e1e1e",
        border: "2px solid #adc6ff",
        borderRadius: 8,
        boxShadow: "0 0 24px rgba(173,198,255,0.12)",
        display: "flex",
        flexDirection: "column",
        overflow: "hidden",
        position: "relative",
      }}
    >
      {/* Title bar */}
      <div
        style={{
          padding: "8px 12px",
          borderBottom: "1px solid rgba(255,255,255,0.08)",
          fontSize: 11,
          fontFamily: "JetBrains Mono, monospace",
          color: "#adc6ff",
          fontWeight: 700,
          letterSpacing: "0.05em",
          flexShrink: 0,
        }}
      >
        {data.name}
      </div>

      {/* Port rows */}
      <div style={{ flex: 1, display: "flex", position: "relative" }}>
        {/* Inputs column */}
        <div style={{ flex: 1, display: "flex", flexDirection: "column", paddingTop: 4 }}>
          {data.inputs.map((port, i) => (
            <div
              key={port.port_id}
              style={{
                height: rowHeight,
                display: "flex",
                alignItems: "center",
                paddingLeft: 16,
                position: "relative",
              }}
            >
              <Handle
                type="target"
                position={Position.Left}
                id={`in-${port.port_id}`}
                style={{
                  left: -1,
                  top: "50%",
                  transform: "translateY(-50%)",
                  width: 8,
                  height: 8,
                  backgroundColor: "#adc6ff",
                  border: "none",
                }}
              />
              <span
                style={{
                  fontSize: 10,
                  fontFamily: "JetBrains Mono, monospace",
                  color: "#adc6ff",
                  opacity: 0.8,
                }}
              >
                {port.name}
              </span>
            </div>
          ))}
        </div>

        {/* Outputs column */}
        <div style={{ flex: 1, display: "flex", flexDirection: "column", paddingTop: 4 }}>
          {data.outputs.map((port, i) => (
            <div
              key={port.port_id}
              style={{
                height: rowHeight,
                display: "flex",
                alignItems: "center",
                justifyContent: "flex-end",
                paddingRight: 16,
                position: "relative",
              }}
            >
              <span
                style={{
                  fontSize: 10,
                  fontFamily: "JetBrains Mono, monospace",
                  color: "#4ae176",
                  opacity: 0.8,
                }}
              >
                {port.name}
              </span>
              <Handle
                type="source"
                position={Position.Right}
                id={`out-${port.port_id}`}
                style={{
                  right: -1,
                  top: "50%",
                  transform: "translateY(-50%)",
                  width: 8,
                  height: 8,
                  backgroundColor: "#4ae176",
                  border: "none",
                }}
              />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

const NODE_TYPES = { transformPreview: TransformPreviewNode };

// ─── Inner canvas that reads selected transform ───────────────────────────────

function CreatorCanvasInner() {
  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const { data: definition, isLoading } = useGetTransformDefinition(selectedId);

  if (selectedId == null) {
    return (
      <div className="w-full h-full flex items-center justify-center">
        <span className="text-sm" style={{ color: "var(--text-muted)" }}>
          Select or create a transform to begin.
        </span>
      </div>
    );
  }

  if (isLoading || !definition) {
    return (
      <div className="w-full h-full flex items-center justify-center">
        <span className="text-xs font-mono" style={{ color: "var(--text-muted)" }}>
          Loading...
        </span>
      </div>
    );
  }

  const inputs = definition.ports.filter((p) => p.direction === "input");
  const outputs = definition.ports.filter((p) => p.direction === "output");

  const node = {
    id: String(definition.transform_id),
    type: "transformPreview" as const,
    position: { x: 0, y: 0 },
    data: { name: definition.name, inputs, outputs },
    draggable: false,
  };

  return (
    <ReactFlow
      nodes={[node]}
      edges={[]}
      nodeTypes={NODE_TYPES}
      fitView
      fitViewOptions={{ padding: 0.4 }}
      nodesDraggable={false}
      nodesConnectable={false}
      elementsSelectable={false}
      zoomOnScroll
      panOnDrag
    >
      <Background
        variant={BackgroundVariant.Dots}
        gap={24}
        size={1}
        color="rgba(255,255,255,0.04)"
      />
    </ReactFlow>
  );
}

// ─── Public export ────────────────────────────────────────────────────────────

export function CreatorCanvas() {
  return (
    <ReactFlowProvider>
      <CreatorCanvasInner />
    </ReactFlowProvider>
  );
}
