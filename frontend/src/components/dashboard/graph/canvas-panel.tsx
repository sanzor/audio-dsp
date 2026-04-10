import ReactFlow, {
  Background,
  BackgroundVariant,
  ReactFlowProvider,
  useNodesState,
  useReactFlow,
  type Node,
} from "reactflow";
import { useCallback, useEffect } from "react";
import "reactflow/dist/style.css";
import { useUIStore } from "@/Stores/UIStore";
import { useGraphStore } from "@/Stores/GraphStore";

function CanvasInner() {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const { screenToFlowPosition } = useReactFlow();
  const regionId = useUIStore((s) => s.activeSelection.regionId);

  useEffect(() => {
    const graph = regionId != null
      ? Array.from(useGraphStore.getState().graphs.values()).find((g) => g.regionId === regionId)
      : undefined;

    setNodes(
      graph?.nodes.map((n) => ({
        id: String(n.id),
        position: n.position,
        data: { nodeId: n.id },
      })) ?? []
    );
  }, [regionId, setNodes]);

  const onDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  }, []);

  const onDrop = useCallback(
    (e: React.DragEvent) => {
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
    [screenToFlowPosition, setNodes]
  );

  return (
    <div
      className="canvas-area w-full h-full"
      onDragOver={onDragOver}
      onDrop={onDrop}
    >
      <ReactFlow
        nodes={nodes}
        onNodesChange={onNodesChange}
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
