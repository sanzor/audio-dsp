import ReactFlow, {
  Background,
  BackgroundVariant,
  ReactFlowProvider,
  useNodesState,
  useEdgesState,
  useReactFlow,
  addEdge,
  Handle,
  Position,
  type Node,
  type Connection,
  type NodeProps,
} from "reactflow";
import { useCallback, useEffect, useMemo } from "react";
import "reactflow/dist/style.css";
import { useUIStore } from "@/Stores/UIStore";
import { useGraphStore } from "@/Stores/GraphStore";
import { useRegionStore } from "@/Stores/RegionStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import type { NodeType } from "@/domain/Graph/Node";

// ─── Structural node components ──────────────────────────────────────────────

function SourceNode({ data }: NodeProps) {
  return (
    <div
      style={{
        padding: "10px 16px",
        borderRadius: 8,
        border: "1.5px solid rgba(52,211,153,0.5)",
        background: "rgba(6,78,59,0.85)",
        color: "#6ee7b7",
        fontWeight: 600,
        fontSize: 13,
        minWidth: 90,
        textAlign: "center",
        userSelect: "none",
      }}
    >
      {data.label as string}
      <Handle
        type="source"
        position={Position.Right}
        style={{ background: "#6ee7b7", width: 10, height: 10 }}
      />
    </div>
  );
}

function SinkNode({ data }: NodeProps) {
  return (
    <div
      style={{
        padding: "10px 16px",
        borderRadius: 8,
        border: "1.5px solid rgba(167,139,250,0.5)",
        background: "rgba(46,16,101,0.85)",
        color: "#c4b5fd",
        fontWeight: 600,
        fontSize: 13,
        minWidth: 90,
        textAlign: "center",
        userSelect: "none",
      }}
    >
      <Handle
        type="target"
        position={Position.Left}
        style={{ background: "#c4b5fd", width: 10, height: 10 }}
      />
      {data.label as string}
    </div>
  );
}

const NODE_TYPES = { source: SourceNode, sink: SinkNode };

// ─── Canvas ───────────────────────────────────────────────────────────────────

function CanvasInner() {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const { screenToFlowPosition } = useReactFlow();
  const openModal = useUIStore((s) => s.openModal);
  const activeSelection = useUIStore((s) => s.activeSelection);
  const region = useRegionStore((s) =>
    activeSelection.regionId != null ? s.regions.get(activeSelection.regionId) : undefined
  );
  const regionSet = useRegionSetStore((s) =>
    activeSelection.regionSetId != null ? s.regionSets.get(activeSelection.regionSetId) : undefined
  );
  const regionId = activeSelection.regionId;
  const canDropTransform =
    region != null &&
    regionSet != null &&
    region.regionSetId === regionSet.id &&
    regionSet.region_ids.includes(region.regionId);

  useEffect(() => {
    const graph = regionId != null
      ? Array.from(useGraphStore.getState().graphs.values()).find((g) => g.regionId === regionId)
      : undefined;

    setNodes(
      graph?.nodes.map((n) => {
        const isStructural = n.nodeType === "source" || n.nodeType === "sink";
        return {
          id: String(n.id),
          type: (n.nodeType ?? "default") as NodeType,
          position: n.position,
          deletable: !isStructural,
          draggable: true,
          data: {
            label: n.nodeType === "source" ? "Source" : n.nodeType === "sink" ? "Sink" : String(n.id),
            nodeId: n.id,
            transformId: n.transformId ?? null,
            nodeType: n.nodeType,
          },
        };
      }) ?? []
    );
    setEdges(
      graph?.edges.map((e) => ({
        id: String(e.id),
        source: String(e.fromNodeId),
        target: String(e.toNodeId),
      })) ?? []
    );
  }, [regionId, setNodes, setEdges]);

  const onConnect = useCallback(
    (connection: Connection) => setEdges((es) => addEdge(connection, es)),
    [setEdges]
  );

  const onNodeDoubleClick = useCallback(
    (_: React.MouseEvent, node: Node) => {
      const nodeType = node.data.nodeType as NodeType;
      if (nodeType === "source" || nodeType === "sink") return;
      openModal({
        type: "nodeDetails",
        nodeId: (node.data.nodeId as number) ?? null,
        transformId: (node.data.transformId as number) ?? null,
      });
    },
    [openModal]
  );

  const onDragOver = useCallback((e: React.DragEvent) => {
    if (!canDropTransform) {
      e.dataTransfer.dropEffect = "none";
      return;
    }
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  }, [canDropTransform]);

  const onDrop = useCallback(
    (e: React.DragEvent) => {
      if (!canDropTransform) return;
      e.preventDefault();
      const raw = e.dataTransfer.getData("application/transform");
      if (!raw) return;

      const { transformId, name } = JSON.parse(raw) as { transformId: number; name: string };
      const position = screenToFlowPosition({ x: e.clientX, y: e.clientY });

      const node: Node = {
        id: `transform-${transformId}-${Date.now()}`,
        type: "default",
        position,
        data: { label: name, transformId, nodeType: "default" },
      };

      setNodes((ns) => [...ns, node]);
    },
    [canDropTransform, screenToFlowPosition, setNodes]
  );

  // Stable nodeTypes reference — must not be recreated on every render
  const nodeTypes = useMemo(() => NODE_TYPES, []);

  return (
    <div className="canvas-area w-full h-full" onDragOver={onDragOver} onDrop={onDrop}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onNodeDoubleClick={onNodeDoubleClick}
        deleteKeyCode={["Backspace", "Delete"]}
        fitView
      >
        <Background variant={BackgroundVariant.Lines} gap={20} color="rgba(255,255,255,0.03)" />
      </ReactFlow>
    </div>
  );
}

export function CanvasPanel() {
  return (
    <ReactFlowProvider>
      <CanvasInner />
    </ReactFlowProvider>
  );
}
