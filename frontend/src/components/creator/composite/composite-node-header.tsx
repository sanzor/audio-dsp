import { creatorToolbarColors } from "../creatorToolbarColors";
import type { NodeDisableSafety } from "./compositeReachability";
import { SAFETY_COLOR } from "./compositeSafetyColors";

const SAFETY_LABEL: Record<NodeDisableSafety, string> = {
  safe: "Safe to disable — no exposed input/output path depends on this node",
  "load-bearing": "Disabling this node would break connectivity from an exposed input to an exposed output",
  disabled: "Currently disabled — excluded from Save/Publish and Play (playback)",
};

interface NodeHeaderProps {
  title: string;
  safety: NodeDisableSafety;
  onToggleDisabled: () => void;
}

export function NodeHeader({ title, safety, onToggleDisabled }: NodeHeaderProps) {
  return (
    <div
      style={{
        padding: "6px 10px",
        borderBottom: "1px solid rgba(255,255,255,0.08)",
        fontSize: 11,
        fontFamily: "JetBrains Mono, monospace",
        color: creatorToolbarColors.blue.color,
        fontWeight: 700,
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        flexShrink: 0,
        gap: 6,
      }}
    >
      <span className="truncate" style={{ flex: 1, minWidth: 0 }}>
        {title}
      </span>
      <button
        onClick={(e) => {
          e.stopPropagation();
          onToggleDisabled();
        }}
        title={SAFETY_LABEL[safety]}
        style={{
          width: 9,
          height: 9,
          borderRadius: "50%",
          backgroundColor: SAFETY_COLOR[safety],
          border: "none",
          cursor: "pointer",
          padding: 0,
          flexShrink: 0,
        }}
      />
    </div>
  );
}
