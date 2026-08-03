import { useState } from "react";
import { creatorToolbarColors } from "../creatorToolbarColors";

// Shared remove ("×") affordance for composite-canvas nodes (leaf/transform
// nodes via CompositeTransformNode, IO nodes via CompositeIoNode). Rendered
// as a sibling of the node's clipped content wrapper — NOT nested inside it —
// so it can sit visually outside the node's border without being cut off by
// that wrapper's overflow: hidden (see composite-canvas-node.tsx for the
// containing structure).
interface RemoveNodeButtonProps {
  onRemove: () => void;
}

export function RemoveNodeButton({ onRemove }: RemoveNodeButtonProps) {
  const [hovered, setHovered] = useState(false);
  const red = creatorToolbarColors.red.color;

  return (
    <button
      onClick={(e) => {
        e.stopPropagation();
        onRemove();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      title="Remove from composite"
      style={{
        position: "absolute",
        top: -9,
        right: -9,
        width: 20,
        height: 20,
        borderRadius: "50%",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        backgroundColor: hovered ? red : "#1e1e1e",
        border: `1.5px solid ${red}`,
        color: hovered ? "#fff" : red,
        fontSize: 13,
        lineHeight: 1,
        padding: 0,
        cursor: "pointer",
        zIndex: 10,
      }}
    >
      ×
    </button>
  );
}
