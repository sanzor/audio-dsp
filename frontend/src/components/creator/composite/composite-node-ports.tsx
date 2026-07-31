import { Handle, Position } from "reactflow";
import type { TransformPort } from "@/domain/Transform/TransformPort";

export const ROW_HEIGHT = 28;

interface PortColumnProps {
  direction: "input" | "output";
  ports: TransformPort[];
}

export function PortColumn({ direction, ports }: PortColumnProps) {
  const isInput = direction === "input";
  const color = isInput ? "#adc6ff" : "#4ae176";
  return (
    <div style={{ flex: 1, display: "flex", flexDirection: "column", paddingTop: 4 }}>
      {ports.map((port) => (
        <div
          key={port.name}
          style={{
            height: ROW_HEIGHT,
            display: "flex",
            alignItems: "center",
            justifyContent: isInput ? undefined : "flex-end",
            paddingLeft: isInput ? 14 : undefined,
            paddingRight: isInput ? undefined : 14,
            position: "relative",
          }}
        >
          {isInput && (
            <Handle
              type="target"
              position={Position.Left}
              id={`in-${port.name}`}
              style={{ left: -1, top: "50%", transform: "translateY(-50%)", width: 8, height: 8, backgroundColor: color, border: "none" }}
            />
          )}
          <span style={{ fontSize: 10, fontFamily: "JetBrains Mono, monospace", color, opacity: 0.8 }}>{port.name}</span>
          {!isInput && (
            <Handle
              type="source"
              position={Position.Right}
              id={`out-${port.name}`}
              style={{ right: -1, top: "50%", transform: "translateY(-50%)", width: 8, height: 8, backgroundColor: color, border: "none" }}
            />
          )}
        </div>
      ))}
    </div>
  );
}
