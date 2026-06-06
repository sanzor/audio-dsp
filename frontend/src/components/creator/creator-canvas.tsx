import ReactFlow, {
  Background,
  BackgroundVariant,
  ReactFlowProvider,
  useNodesState,
  useEdgesState,
  type Node,
  type Edge,
} from "reactflow";
import "reactflow/dist/style.css";

const INITIAL_NODES: Node[] = [
  {
    id: "source",
    type: "input",
    position: { x: 80, y: 140 },
    data: { label: "INPUT_BUFFER" },
    style: {
      backgroundColor: "#1e1e1e",
      border: "1px solid rgba(255,255,255,0.1)",
      color: "#e5e5e5",
      fontSize: 11,
      fontFamily: "JetBrains Mono, monospace",
    },
  },
  {
    id: "rms",
    position: { x: 360, y: 155 },
    data: { label: "RMS_DETECTOR" },
    style: {
      backgroundColor: "#1e1e1e",
      border: "2px solid #adc6ff",
      color: "#adc6ff",
      fontSize: 11,
      fontFamily: "JetBrains Mono, monospace",
      boxShadow: "0 0 12px rgba(173,198,255,0.15)",
    },
  },
  {
    id: "sink",
    type: "output",
    position: { x: 640, y: 175 },
    data: { label: "COMPOSITE: OUTPUT" },
    style: {
      backgroundColor: "#1e1e1e",
      border: "1px solid rgba(74,225,118,0.5)",
      color: "#4ae176",
      fontSize: 11,
      fontFamily: "JetBrains Mono, monospace",
    },
  },
];

const INITIAL_EDGES: Edge[] = [
  { id: "e-src-rms", source: "source", target: "rms", style: { stroke: "#adc6ff", strokeWidth: 2, opacity: 0.6 } },
  { id: "e-rms-sink", source: "rms", target: "sink", style: { stroke: "#4ae176", strokeWidth: 2, strokeDasharray: "4", opacity: 0.6 } },
];

function CreatorCanvasInner() {
  const [nodes, , onNodesChange] = useNodesState(INITIAL_NODES);
  const [edges, , onEdgesChange] = useEdgesState(INITIAL_EDGES);

  return (
    <div className="relative w-full h-full">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        fitView
        deleteKeyCode={["Backspace", "Delete"]}
      >
        <Background
          variant={BackgroundVariant.Dots}
          gap={24}
          size={1}
          color="rgba(255,255,255,0.04)"
        />
      </ReactFlow>

      {/* Canvas title overlay */}
      <div className="absolute top-4 left-4 pointer-events-none z-10">
        <div className="text-sm font-semibold" style={{ color: "var(--text-main)" }}>
          Dynamic Multiband Compressor
        </div>
        <div className="text-xs mt-0.5" style={{ color: "var(--text-muted)" }}>
          Mastering / Dynamics / v2.1.0
        </div>
      </div>

      {/* Canvas controls */}
      <div className="absolute bottom-4 left-4 flex gap-2 z-10">
        {["zoom_in", "pan"].map((label) => (
          <button
            key={label}
            className="w-8 h-8 flex items-center justify-center rounded text-xs transition-colors"
            style={{
              backgroundColor: "var(--bg-panel)",
              border: "1px solid rgba(255,255,255,0.08)",
              color: "var(--text-muted)",
            }}
          >
            {label === "zoom_in" ? "+" : "✥"}
          </button>
        ))}
      </div>
    </div>
  );
}

export function CreatorCanvas() {
  return (
    <ReactFlowProvider>
      <CreatorCanvasInner />
    </ReactFlowProvider>
  );
}
