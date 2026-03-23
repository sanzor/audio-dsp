import ReactFlow, { Background, BackgroundVariant } from "reactflow";
import "reactflow/dist/style.css";

export function CanvasPanel() {
  return (
    <div className="canvas-area w-full h-full">
      <ReactFlow fitView>
        <Background
          variant={BackgroundVariant.Lines}
          gap={20}
          color="rgba(255,255,255,0.03)"
        />
      </ReactFlow>
    </div>
  );
}