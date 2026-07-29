import ReactFlow, {
  Background,
  BackgroundVariant,
  ReactFlowProvider,
  type NodeProps,
} from "reactflow";
import { Handle, Position } from "reactflow";
import "reactflow/dist/style.css";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCreatorPreviewStore } from "@/Stores/CreatorPreviewStore";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";
import type { TransformPort } from "@/domain/Transform/TransformPort";


// ─── Live meter bar ───────────────────────────────────────────────────────────
// Subscribes directly to the preview store rather than receiving levels as
// props from CreatorCanvasInner — isolates the ~20Hz re-render to this leaf
// element instead of the whole ReactFlow tree. Renders nothing unless this
// exact transform is the one currently previewing, so there's no regression
// to the static port list the rest of the time.

interface LiveMeterBarProps {
  direction: "input" | "output";
  transformId: number;
}

function LiveMeterBar({ direction, transformId }: LiveMeterBarProps) {
  const previewStatus = useCreatorPreviewStore((s) => s.status);
  const previewTransformId = useCreatorPreviewStore((s) => s.previewTransformId);
  const level = useCreatorPreviewStore((s) => (direction === "input" ? s.inputLevel : s.outputLevel));

  if (previewStatus !== "playing" || previewTransformId !== transformId) return null;

  const color = direction === "input" ? "#adc6ff" : "#4ae176";
  const widthPct = Math.min(100, level.peak * 100);

  return (
    <div
      style={{
        height: 3,
        margin: "4px 12px",
        borderRadius: 2,
        backgroundColor: "rgba(255,255,255,0.08)",
        overflow: "hidden",
        flexShrink: 0,
      }}
    >
      <div style={{ height: "100%", width: `${widthPct}%`, backgroundColor: color }} />
    </div>
  );
}

// ─── Custom single-transform node ────────────────────────────────────────────

interface TransformNodeData {
  transformId: number;
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
          <LiveMeterBar direction="input" transformId={data.transformId} />
          {data.inputs.map((port) => (
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
          <LiveMeterBar direction="output" transformId={data.transformId} />
          {data.outputs.map((port) => (
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
    data: { transformId: definition.transform_id, name: definition.name, inputs, outputs },
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
