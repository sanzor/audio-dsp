import ReactFlow, {
  Background,
  BackgroundVariant,
  ReactFlowProvider,
  useNodesState,
  useEdgesState,
  useReactFlow,
  addEdge,
  type Node,
  type Connection,
} from "reactflow";
import { useCallback, useEffect } from "react";
import "reactflow/dist/style.css";
import { useUIStore } from "@/Stores/UIStore";
import { useGraphStore } from "@/Stores/GraphStore";
import { useRegionStore } from "@/Stores/RegionStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";

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
      graph?.nodes.map((n) => ({
        id: String(n.id),
        position: n.position,
        data: { nodeId: n.id, transformId: n.transformId ?? null },
      })) ?? []
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

      const { transformId, name } = JSON.parse(raw) as {
        transformId: number;
        name: string;
      };
      const position = screenToFlowPosition({ x: e.clientX, y: e.clientY });

      const node: Node = {
        id: `transform-${transformId}-${Date.now()}`,
        type: "default",
        position,
        data: { label: name, transformId },
      };

      setNodes((ns) => [...ns, node]);
    },
    [canDropTransform, screenToFlowPosition, setNodes]
  );

  return (
    <div
      className="canvas-area w-full h-full"
      onDragOver={onDragOver}
      onDrop={onDrop}
    >
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onNodeDoubleClick={onNodeDoubleClick}
        deleteKeyCode={["Backspace", "Delete"]}
        fitView
      >
        <Background
          variant={BackgroundVariant.Lines}
          gap={20}
          color="rgba(255,255,255,0.03)"
        />
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
