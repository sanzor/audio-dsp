import { useMemo } from "react";
import { useTransformStore } from "@/Stores/TransformStore";
import { usePreloadBinaries } from "@/hooks/transforms/queries";

export function usePreloadWasmBinaries(): void {
  const summaries = useTransformStore((s) => s.summaries);
  const transformIds = useMemo(() => [...summaries.keys()], [summaries]);
  usePreloadBinaries(transformIds);
}
