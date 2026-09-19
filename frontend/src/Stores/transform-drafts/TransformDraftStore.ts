import { create } from "zustand";
import type { TransformDraftDefinition } from "@/domain/transform-drafts/TransformDraftDefinition";

// Bucket 2 (draft) mirror of Stores/TransformStore.ts's bucket-3 shape —
// keyed by transform_id, same as TransformStore's `definitions` Map. Fed by
// hooks/transform-drafts/mutations.ts's useCreateTransform/useSaveTransform
// onSuccess handlers; nothing currently reads from this store (that data was
// previously discarded entirely), so this is additive, not a behavior
// change to any existing consumer.
interface TransformDraftState {
  drafts: Map<number, TransformDraftDefinition>;
  upsertDraft: (draft: TransformDraftDefinition) => void;
  setDraft: (draft: TransformDraftDefinition) => void;
  clear: () => void;
}

export const useTransformDraftStore = create<TransformDraftState>()((set) => ({
  drafts: new Map(),
  upsertDraft: (draft) =>
    set((state) => {
      const drafts = new Map(state.drafts);
      drafts.set(draft.transform_id, draft);
      return { drafts };
    }),
  setDraft: (draft) =>
    set((state) => {
      const drafts = new Map(state.drafts);
      drafts.set(draft.transform_id, draft);
      return { drafts };
    }),
  clear: () => set({ drafts: new Map() }),
}));
