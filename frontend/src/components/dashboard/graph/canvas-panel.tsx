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
  type NodeChange,
} from "reactflow";
import { useCallback, useEffect, useMemo } from "react";
import "reactflow/dist/style.css";
import { useUIStore } from "@/Stores/UIStore";
import { useGraphStore } from "@/Stores/GraphStore";
import { useRegionStore } from "@/Stores/RegionStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import type { NodeType } from "@/domain/Graph/Node";
import { CanvasToolbar } from "./canvas-toolbar";
import { useGraphController } from "@/controllers/GraphController";

// ─── Structural node components ──────────────────────────────────────────────

function SourceNode(_: NodeProps) {
  return (
    <div
      style={{
        width: 88,
        height: 88,
        borderRadius: "50%",
        background: "radial-gradient(circle at 40% 40%, rgba(6,182,212,0.28) 0%, rgba(8,145,178,0.10) 100%)",
        border: "2px solid #06b6d4",
        boxShadow: "0 0 14px rgba(6,182,212,0.35)",
        color: "#67e8f9",
        fontWeight: 700,
        fontSize: 12,
        letterSpacing: "0.05em",
        textTransform: "uppercase",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        userSelect: "none",
        position: "relative",
      }}
    >
      Input
      <Handle
        type="source"
        position={Position.Right}
        style={{ background: "#06b6d4", border: "2px solid #164e63", width: 11, height: 11 }}
      />
    </div>
  );
}

function SinkNode(_: NodeProps) {
  return (
    <div
      style={{
        width: 88,
        height: 88,
        borderRadius: "12px",
        transform: "rotate(45deg)",
        background: "linear-gradient(135deg, rgba(245,158,11,0.22) 0%, rgba(180,83,9,0.12) 100%)",
        border: "2px solid #f59e0b",
        boxShadow: "0 0 14px rgba(245,158,11,0.30)",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        userSelect: "none",
        position: "relative",
      }}
    >
      <span
        style={{
          transform: "rotate(-45deg)",
          color: "#fcd34d",
          fontWeight: 700,
          fontSize: 12,
          letterSpacing: "0.05em",
          textTransform: "uppercase",
        }}
      >
        Output
      </span>
      <Handle
        type="target"
        position={Position.Left}
        style={{ background: "#f59e0b", border: "2px solid #78350f", width: 11, height: 11, transform: "rotate(-45deg) translateY(-50%)" }}
      />
    </div>
  );
}

const NODE_TYPES = { source: SourceNode, sink: SinkNode };

// ─── Canvas ───────────────────────────────────────────────────────────────────

function CanvasInner() {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const onNodesChangeSafe = useCallback(
    (changes: NodeChange[]) => {
      const filtered = changes.filter(
        (c) => !(c.type === "remove" && nodes.find((n) => n.id === c.id && (n.data.nodeType === "source" || n.data.nodeType === "sink")))
      );
      onNodesChange(filtered);
    },
    [nodes, onNodesChange],
  );
  const { screenToFlowPosition, fitView } = useReactFlow();
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

  const activeGraphId = useGraphStore((s) => {
    if (regionId == null) return undefined;
    for (const g of s.graphs.values()) {
      if (g.regionId === regionId) return g.id;
    }
    return undefined;
  });

  const activeGraphVersion = useGraphStore((s) =>
    activeGraphId != null ? s.graphs.get(activeGraphId)?.updatedAt?.getTime() : undefined
  );

  const graphController = useGraphController();

  useEffect(() => {
    const graph = activeGraphId != null
      ? useGraphStore.getState().getGraph(activeGraphId)
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
            label: n.nodeType === "source" ? "Input" : n.nodeType === "sink" ? "Output" : String(n.id),
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
  }, [regionId, activeGraphId, activeGraphVersion, setNodes, setEdges]);

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

  // ─── Toolbar handlers ───────────────────────────────────────────────────────

  const handleSave = useCallback(async () => {
    if (activeGraphId == null) return;
    await graphController.handleSaveGraph(activeGraphId, nodes, edges);
  }, [activeGraphId, nodes, edges, graphController]);

  const handleFitView = useCallback(() => {
    fitView({ padding: 0.2 });
  }, [fitView]);

  const handleClearNodes = useCallback(() => {
    if (activeGraphId == null) return;
    graphController.handleClearGraphNodes(activeGraphId);
  }, [activeGraphId, graphController]);

  const handleRename = useCallback(() => {
    if (activeGraphId == null) return;
    graphController.handleRenameGraph(activeGraphId);
  }, [activeGraphId, graphController]);

  const handleDelete = useCallback(async () => {
    if (activeGraphId == null) return;
    await graphController.handleDeleteGraph(activeGraphId);
  }, [activeGraphId, graphController]);

  const handleCopy = useCallback(() => {
    if (activeGraphId == null) return;
    graphController.handleCopyGraph(activeGraphId);
  }, [activeGraphId, graphController]);

  const hasClearableNodes = nodes.some((n) => n.data.nodeType === "default");

  // Stable nodeTypes reference — must not be recreated on every render
  const nodeTypes = useMemo(() => NODE_TYPES, []);

  return (
    <div className="flex flex-col w-full h-full">
      <CanvasToolbar
        selectedGraphId={activeGraphId}
        hasClearableNodes={hasClearableNodes}
        onSave={handleSave}
        onFitView={handleFitView}
        onClearNodes={handleClearNodes}
        onRename={handleRename}
        onDelete={handleDelete}
        onCopy={handleCopy}
      />
      <div className="canvas-area flex-1 min-h-0" onDragOver={onDragOver} onDrop={onDrop}>
        <ReactFlow
          nodes={nodes}
          edges={edges}
          nodeTypes={nodeTypes}
          onNodesChange={onNodesChangeSafe}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          onNodeDoubleClick={onNodeDoubleClick}
          deleteKeyCode={["Backspace", "Delete"]}
          fitView
        >
          <Background variant={BackgroundVariant.Lines} gap={20} color="rgba(255,255,255,0.03)" />
        </ReactFlow>
      </div>
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
