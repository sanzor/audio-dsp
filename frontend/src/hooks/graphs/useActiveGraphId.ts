import { useGraphStore } from "@/Stores/GraphStore";
import { useUIStore } from "@/Stores/UIStore";

export function useActiveGraphId(): number | undefined {
  const regionId = useUIStore((s) => s.activeSelection.regionId);

  return useGraphStore((s) => {
    if (regionId == null) return undefined;
    for (const graph of s.graphs.values()) {
      if (graph.regionId === regionId) return graph.id;
    }
    return undefined;
  });
}
