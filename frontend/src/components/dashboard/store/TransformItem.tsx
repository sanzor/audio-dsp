import { Activity } from "lucide-react";
import type { Transform } from "@/domain/Transform/Transform";

const ICON_MAP: Record<string, React.ComponentType<{ className?: string }>> = {
  Activity,
};

interface TransformItemProps {
  transform: Transform;
}

export function TransformItem({ transform }: TransformItemProps) {
  const Icon = (transform.icon && ICON_MAP[transform.icon]) || Activity;

  return (
    <button className="store-btn" draggable>
      <Icon className="w-6 h-6" />
      <span className="truncate w-full text-center">{transform.name}</span>
    </button>
  );
}
