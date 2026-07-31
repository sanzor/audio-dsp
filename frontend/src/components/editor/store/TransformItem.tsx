import { Activity } from "lucide-react";
import { useUIStore } from "@/Stores/UIStore";
import type { TransformSummary } from "@/domain/Transform/TransformSummary";

const ICON_MAP: Record<string, React.ComponentType<{ className?: string }>> = {
  Activity,
};

interface TransformItemProps {
  transform: TransformSummary;
}

export function TransformItem({ transform }: TransformItemProps) {
  const Icon = (transform.icon && ICON_MAP[transform.icon]) || Activity;
  const openModal = useUIStore((s) => s.openModal);

  function onDragStart(e: React.DragEvent) {
    e.dataTransfer.setData(
      "application/transform",
      JSON.stringify({ transformId: transform.transform_id, name: transform.name })
    );
    e.dataTransfer.effectAllowed = "move";
  }

  return (
    <button
      className="store-btn"
      draggable
      onDragStart={onDragStart}
      onDoubleClick={() => openModal({ type: "transformDetails", transformId: transform.transform_id })}
    >
      <Icon className="w-6 h-6" />
      <span className="truncate w-full text-center">{transform.name}</span>
    </button>
  );
}
