import { create } from "zustand";
import { useUIStore } from "@/Stores/UIStore";
import { useCompositeCanvasStore } from "@/Stores/CompositeCanvasStore";

export interface EditingTransformSource {
  transformId: number;
  originalSource: string;
  source: string;
}

export interface EditingTransformMetadata {
  transformId: number;
  originalName: string;
  originalDescription: string;
  name: string;
  description: string;
}

export type PendingTransformAction =
  | { kind: "select"; transformId: number }
  | { kind: "create" };

interface CreatorState {
  selectedTransformId: number | null;
  // Unguarded — only for cases that are known not to conflict with an
  // in-progress edit (e.g. right after creating a brand new transform).
  setSelectedTransformId: (id: number | null) => void;

  // The live (possibly unsaved) source buffer for whichever transform is
  // currently open in the code editor. Lives here rather than as local
  // component state so the switch/create guards below can read it without a
  // callback into the editor component — mirrors UIStore's editingRegionBounds.
  editingTransformSource: EditingTransformSource | null;
  beginEditingTransformSource: (transformId: number, initialSource: string) => void;
  updateEditingTransformSource: (source: string) => void;
  // Advances the "saved" baseline to what was actually persisted, without
  // clobbering any further edits made while the save request was in flight.
  markTransformSourceSaved: (transformId: number, savedSource: string) => void;

  // Same pattern, mirrored for the properties panel's name/description
  // buffer — lets code-editor.tsx's Save Draft button flush a pending
  // metadata edit without a callback into the sibling panel component.
  editingTransformMetadata: EditingTransformMetadata | null;
  beginEditingTransformMetadata: (transformId: number, initialName: string, initialDescription: string) => void;
  updateEditingTransformMetadata: (patch: Partial<{ name: string; description: string }>) => void;
  markTransformMetadataSaved: (transformId: number, savedName: string, savedDescription: string) => void;

  // Set when a select/create action was blocked by unsaved source changes;
  // the confirm modal resolves or cancels it.
  pendingTransformAction: PendingTransformAction | null;
  requestSelectTransform: (transformId: number) => void;
  requestCreateTransform: () => void;
  resolvePendingTransformAction: () => void;
  cancelPendingTransformAction: () => void;
}

// Exported so the unsaved-changes modal can derive the same "is the code
// editor's buffer actually dirty" check without duplicating the comparison.
export const isSourceDirty = (editing: EditingTransformSource | null) =>
  editing != null && editing.source !== editing.originalSource;

// Same pattern as isSourceDirty, mirrored for the metadata buffer.
export const isMetadataDirty = (editing: EditingTransformMetadata | null) =>
  editing != null &&
  (editing.name !== editing.originalName || editing.description !== editing.originalDescription);

export const useCreatorStore = create<CreatorState>()((set, get) => {
  const applyTransformAction = (action: PendingTransformAction) => {
    useCompositeCanvasStore.getState().reset();
    if (action.kind === "select") {
      set({
        selectedTransformId: action.transformId,
        editingTransformSource: null,
        editingTransformMetadata: null,
        pendingTransformAction: null,
      });
    } else {
      set({ editingTransformSource: null, editingTransformMetadata: null, pendingTransformAction: null });
      useUIStore.getState().openModal({ type: "createTransform" });
    }
  };

  return {
        selectedTransformId: null,
        setSelectedTransformId: (id) => set({ selectedTransformId: id }),

        editingTransformSource: null,
        beginEditingTransformSource: (transformId, initialSource) =>
          set({ editingTransformSource: { transformId, originalSource: initialSource, source: initialSource } }),
        updateEditingTransformSource: (source) =>
          set((state) => {
            if (!state.editingTransformSource) return {};
            return { editingTransformSource: { ...state.editingTransformSource, source } };
          }),
        markTransformSourceSaved: (transformId, savedSource) =>
          set((state) => {
            if (!state.editingTransformSource || state.editingTransformSource.transformId !== transformId) return {};
            return { editingTransformSource: { ...state.editingTransformSource, originalSource: savedSource } };
          }),

        editingTransformMetadata: null,
        beginEditingTransformMetadata: (transformId, initialName, initialDescription) =>
          set({
            editingTransformMetadata: {
              transformId,
              originalName: initialName,
              originalDescription: initialDescription,
              name: initialName,
              description: initialDescription,
            },
          }),
        updateEditingTransformMetadata: (patch) =>
          set((state) => {
            if (!state.editingTransformMetadata) return {};
            return { editingTransformMetadata: { ...state.editingTransformMetadata, ...patch } };
          }),
        markTransformMetadataSaved: (transformId, savedName, savedDescription) =>
          set((state) => {
            if (!state.editingTransformMetadata || state.editingTransformMetadata.transformId !== transformId) return {};
            return {
              editingTransformMetadata: {
                ...state.editingTransformMetadata,
                originalName: savedName,
                originalDescription: savedDescription,
              },
            };
          }),

        pendingTransformAction: null,
        requestSelectTransform: (transformId) => {
          const state = get();
          if (state.selectedTransformId === transformId) return;

          const editing = state.editingTransformSource;
          const leavingEditedTransform = editing != null && editing.transformId !== transformId;

          const compositeGraph = useCompositeCanvasStore.getState().editingGraph;
          const leavingEditedComposite = compositeGraph != null && compositeGraph.transformId !== transformId;

          const editingMetadata = state.editingTransformMetadata;
          const leavingEditedMetadata = editingMetadata != null && editingMetadata.transformId !== transformId;

          if (
            (leavingEditedTransform && isSourceDirty(editing)) ||
            (leavingEditedComposite && useCompositeCanvasStore.getState().isDirty()) ||
            (leavingEditedMetadata && isMetadataDirty(editingMetadata))
          ) {
            set({ pendingTransformAction: { kind: "select", transformId } });
            return;
          }
          applyTransformAction({ kind: "select", transformId });
        },

        requestCreateTransform: () => {
          const editing = get().editingTransformSource;
          const editingMetadata = get().editingTransformMetadata;
          if (
            isSourceDirty(editing) ||
            useCompositeCanvasStore.getState().isDirty() ||
            isMetadataDirty(editingMetadata)
          ) {
            set({ pendingTransformAction: { kind: "create" } });
            return;
          }
          applyTransformAction({ kind: "create" });
        },

        resolvePendingTransformAction: () => {
          const pending = get().pendingTransformAction;
          if (pending) applyTransformAction(pending);
        },

        cancelPendingTransformAction: () => set({ pendingTransformAction: null }),
  };
});
