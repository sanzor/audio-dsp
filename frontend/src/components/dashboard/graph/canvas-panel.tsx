import ReactFlow, { Background } from "reactflow";
import "reactflow/dist/style.css";

export function CanvasPanel() {
  return (
    <div className="w-full h-full relative">
      <ReactFlow
        fitView
      >
        <Background />
      </ReactFlow>
    </div>
  );
}