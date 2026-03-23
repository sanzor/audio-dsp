import { Activity, SlidersHorizontal, Music2, Clock, Waves } from "lucide-react";
import { useTransformStore } from "@/Stores/TransformStore";
import { useListTransforms } from "@/hooks/transforms/queries";
import type { Transform } from "@/domain/Transform/Transform";

// ─── Hardcoded defaults (shown until backend data loads / as builtins) ────────
const DEFAULT_TRANSFORMS: Transform[] = [
  { transform_id: -1, name: "EQ",         icon: "Activity",          ports: [] },
  { transform_id: -2, name: "Compressor", icon: "SlidersHorizontal", ports: [] },
  { transform_id: -3, name: "Reverb",     icon: "Music2",            ports: [] },
  { transform_id: -4, name: "Delay",      icon: "Clock",             ports: [] },
  { transform_id: -5, name: "Flanger",    icon: "Waves",             ports: [] },
  { transform_id: -6, name: "Chorus",     icon: "Waves",             ports: [] },
];

const ICON_MAP: Record<string, React.ComponentType<{ className?: string }>> = {
  Activity, SlidersHorizontal, Music2, Clock, Waves,
};

function TransformIcon({ icon }: { icon?: string }) {
  const Icon = (icon && ICON_MAP[icon]) || Activity;
  return <Icon className="w-6 h-6" />;
}

export function TransformStorePanel() {
  const { fetchNextPage, hasNextPage, isFetchingNextPage } = useListTransforms();
  const fetched = Array.from(useTransformStore((s) => s.transforms).values());

  // Merge: defaults first, then fetched (fetched take precedence by name dedup)
  const fetchedNames = new Set(fetched.map((t) => t.name));
  const defaults = DEFAULT_TRANSFORMS.filter((d) => !fetchedNames.has(d.name));
  const all = [...defaults, ...fetched];

  return (
    <>
      <div className="panel-header text-center text-sm">Store</div>
      <div className="panel-content px-1">
        {all.map((t) => (
          <button key={t.transform_id} className="store-btn" draggable>
            <TransformIcon icon={t.icon} />
            <span className="truncate w-full text-center">{t.name}</span>
          </button>
        ))}

        {hasNextPage && (
          <button
            className="store-btn text-xs"
            onClick={() => fetchNextPage()}
            disabled={isFetchingNextPage}
          >
            {isFetchingNextPage ? "..." : "More"}
          </button>
        )}
      </div>
    </>
  );
}
