import { useEffect, useMemo } from "react";
import { apiResolveTransformDefinitions } from "@/Services/TransformService";
import { useGraphStore } from "@/Stores/GraphStore";
import { useTransformStore } from "@/Stores/TransformStore";
import { useActiveGraphId } from "@/hooks/graphs/useActiveGraphId";
import { usePreloadBinaries } from "@/hooks/transforms/queries";

export function useHydrateActiveGraphTransforms(): void {
  const activeGraphId = useActiveGraphId();
  const activeGraph = useGraphStore((state) =>
    activeGraphId != null ? state.graphs.get(activeGraphId) : undefined
  );
  const activeGraphVersion = activeGraph?.updatedAt?.getTime() ?? 0;
  const definitions = useTransformStore((state) => state.definitions);
  const upsertDefinitions = useTransformStore((state) => state.upsertDefinitions);

  const transformIds = useMemo(() => {
    if (!activeGraph) return [];
    return Array.from(
      new Set(
        activeGraph.nodes
          .map((node) => node.transformId)
          .filter((id): id is number => id != null)
      )
    );
  }, [activeGraph, activeGraphVersion]);

  usePreloadBinaries(transformIds);

  useEffect(() => {
    if (transformIds.length === 0) return;

    const missingDefinitionIds = transformIds.filter((id) => !definitions.has(id));
    if (missingDefinitionIds.length === 0) return;

    let cancelled = false;

    const hydrate = async () => {
      try {
        const resolved = await apiResolveTransformDefinitions(missingDefinitionIds);
        if (!cancelled) upsertDefinitions(resolved);
      } catch (error) {
        if (cancelled) return;
        console.error("Failed to hydrate active graph transforms", error);
      }
    };

    void hydrate();
    return () => { cancelled = true; };
  }, [transformIds, definitions, upsertDefinitions]);
}
