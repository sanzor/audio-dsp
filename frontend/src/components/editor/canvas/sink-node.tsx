import { Handle, Position, type NodeProps } from "reactflow";

export function SinkNode(_: NodeProps) {
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
