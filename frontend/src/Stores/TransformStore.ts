import { create } from "zustand";
import type { Transform } from "@/domain/Transform/Transform";

interface TransformState {
  transforms: Map<number, Transform>;
  setTransforms: (transforms: Transform[]) => void;
}

export const useTransformStore = create<TransformState>()((set) => ({
  transforms: new Map(),
  setTransforms: (transforms) =>
    set({ transforms: new Map(transforms.map((t) => [t.transform_id, t])) }),
}));
