import { create } from "zustand";

const DEFAULT_COUNT = 20;

interface PurchasedProductsState {
  index: number;
  count: number;
  orgIdFilter: number | null;
  setOrgIdFilter: (orgId: number | null) => void;
  setPage: (index: number) => void;
  nextPage: () => void;
  prevPage: () => void;
  reset: () => void;
}

export const usePurchasedProductsStore = create<PurchasedProductsState>()((set) => ({
  index: 0,
  count: DEFAULT_COUNT,
  orgIdFilter: null,

  setOrgIdFilter: (orgId) => set({ orgIdFilter: orgId, index: 0 }),
  setPage: (index) => set({ index }),
  nextPage: () => set((s) => ({ index: s.index + s.count })),
  prevPage: () => set((s) => ({ index: Math.max(0, s.index - s.count) })),
  reset: () => set({ index: 0, orgIdFilter: null }),
}));
