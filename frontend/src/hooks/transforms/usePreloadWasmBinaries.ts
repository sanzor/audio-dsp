import { useEffect } from "react";
import { useTransformStore } from "@/Stores/TransformStore";
import { ensureTransformBinariesExist } from "./useEnsureTransformBinaries";

export function usePreloadWasmBinaries(): void {
  const summaries = useTransformStore((s) => s.summaries);
  const ensureTransformBinaries = ensureTransformBinariesExist();

  useEffect(() => {
    const transformIds = [...summaries.keys()];
    if (transformIds.length === 0) {
      return;
    }

    void ensureTransformBinaries(transformIds);
  }, [summaries, ensureTransformBinaries]);
}
