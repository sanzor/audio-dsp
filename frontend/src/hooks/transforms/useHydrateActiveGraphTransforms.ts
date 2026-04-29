import { useEffect, useMemo } from "react";
import { apiGetTransformBinaries, apiResolveTransformDefinitions } from "@/Services/TransformService";
import { useGraphStore } from "@/Stores/GraphStore";
import { useTransformStore } from "@/Stores/TransformStore";
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore";
import { useActiveGraphId } from "@/hooks/graphs/useActiveGraphId";

export function useHydrateActiveGraphTransforms(): void {
  const activeGraphId = useActiveGraphId();
  const activeGraph = useGraphStore((state) =>
    activeGraphId != null ? state.graphs.get(activeGraphId) : undefined
  );
  const activeGraphVersion = activeGraph?.updatedAt?.getTime() ?? 0;
  const definitions = useTransformStore((state) => state.definitions);
  const upsertDefinitions = useTransformStore((state) => state.upsertDefinitions);
  const binaries = useWasmBinaryStore((state) => state.binaries);
  const status = useWasmBinaryStore((state) => state.status);
  const setBinary = useWasmBinaryStore((state) => state.setBinary);
  const setStatus = useWasmBinaryStore((state) => state.setStatus);

  const transformIds = useMemo(() => {
    if (!activeGraph) return [];
    return Array.from(
      new Set(
        activeGraph.nodes
          .map((node) => node.transformId)
          .filter((transformId): transformId is number => transformId != null)
      )
    );
  }, [activeGraph, activeGraphVersion]);

  useEffect(() => {
    if (transformIds.length === 0) {
      return;
    }

    const missingDefinitionIds = transformIds.filter((transformId) => !definitions.has(transformId));
    const missingBinaryIds = transformIds.filter(
      (transformId) => !binaries.has(transformId) && status.get(transformId) == null
    );

    if (missingDefinitionIds.length === 0 && missingBinaryIds.length === 0) {
      return;
    }

    let cancelled = false;

    const hydrate = async () => {
      try {
        if (missingDefinitionIds.length > 0) {
          const resolved = await apiResolveTransformDefinitions(missingDefinitionIds);
          if (!cancelled) {
            upsertDefinitions(resolved);
          }
        }

        if (missingBinaryIds.length > 0) {
          for (const transformId of missingBinaryIds) {
            setStatus(transformId, "fetching");
          }

          const resolvedBinaries = await apiGetTransformBinaries(missingBinaryIds);
          if (cancelled) {
            return;
          }

          for (const transformId of missingBinaryIds) {
            const binary = resolvedBinaries.get(transformId);
            if (!binary) {
              setStatus(transformId, "error");
              continue;
            }
            setBinary(transformId, binary);
          }
        }
      } catch (error) {
        if (cancelled) {
          return;
        }
        console.error("Failed to hydrate active graph transforms", error);
        for (const transformId of missingBinaryIds) {
          setStatus(transformId, "error");
        }
      }
    };

    void hydrate();

    return () => {
      cancelled = true;
    };
  }, [
    binaries,
    definitions,
    setBinary,
    setStatus,
    status,
    transformIds,
    upsertDefinitions,
  ]);
}
