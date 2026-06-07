import { create } from "zustand";

interface CreatorState {
  selectedTransformId: number | null;
  setSelectedTransformId: (id: number | null) => void;
}

export const useCreatorStore = create<CreatorState>((set) => ({
  selectedTransformId: null,
  setSelectedTransformId: (id) => set({ selectedTransformId: id }),
}));
