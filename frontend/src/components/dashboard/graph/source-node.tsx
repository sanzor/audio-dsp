import { Handle, Position, type NodeProps } from "reactflow";

export function SourceNode(_: NodeProps) {
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
