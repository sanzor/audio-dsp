// controllers/TransformDraftController.ts
//
// Bucket 2 (draft) controller — absorbs what used to live in
// controllers/TransformController.ts (create/delete) and extends it to also
// wrap save/validate-source/validate-graph/publish, which were previously
// called directly from code-editor.tsx and composite-canvas.tsx, bypassing
// any controller. Handler shapes below mirror those two components'
// existing call patterns (saveMutation/checkMutation/publishMutation in
// code-editor.tsx; saveMutation/validateMutation/publishMutation in
// composite-canvas.tsx) — each mutation is still returned directly so a
// component can read its isPending/isError/error state inline exactly as
// before, while the imperative trigger goes through a handle* method here.
import { useUIStore } from "@/Stores/UIStore";
import { useCreatorStore } from "@/Stores/CreatorStore";
import {
  useCreateTransform,
  useDeleteTransform,
  useSaveTransform,
  useValidateTransformSourceCode,
  useValidateCompositeGraph,
  usePublishTransform,
} from "@/hooks/transform-drafts/mutations";
import type {
  CreateTransformParams,
  SaveDraftParams,
  PublishDraftParams,
  ValidateGraphResponse,
} from "@/Services/transform-drafts/TransformDraftsService";
import type { CompositeGraphDefinition } from "@/domain/Transform/CompositeGraphDefinition";

// `transformId` is optional (defaults to -1, mirroring the `selectedId ?? -1`
// pattern code-editor.tsx/composite-canvas.tsx used directly against the
// mutation hooks before this controller existed) — callers that only need
// create/delete (e.g. transforms-sidebar.tsx, which has no single "current"
// transform) can omit it; callers driving save/validate/publish for one open
// transform (code-editor.tsx, composite-canvas.tsx,
// unsaved-creator-changes-modal.tsx) pass the transform they're editing.
export function useTransformDraftController(transformId: number = -1) {
  // Zustand selectors
  const closeModal = useUIStore((state) => state.closeModal);
  const openModal = useUIStore((state) => state.openModal);
  const setSelectedTransformId = useCreatorStore((state) => state.setSelectedTransformId);
  const selectedTransformId = useCreatorStore((state) => state.selectedTransformId);

  // Data and mutations
  const createTransformMutation = useCreateTransform();
  const deleteTransformMutation = useDeleteTransform();
  const saveMutation = useSaveTransform(transformId);
  const checkMutation = useValidateTransformSourceCode(transformId);
  const validateGraphMutation = useValidateCompositeGraph(transformId);
  const publishMutation = usePublishTransform(transformId);

  return {
    // ============================================
    // CREATE TRANSFORM
    // ============================================
    handleCreateTransform: () => {
      openModal({ type: "createTransform" });
    },

    handleSubmitCreateTransform: async (params: CreateTransformParams) => {
      try {
        const definition = await createTransformMutation.mutateAsync(params);
        setSelectedTransformId(definition.transform_id);
        closeModal(); // ✅ Close modal on success
        return definition;
      } catch (error) {
        console.error("Failed to create transform:", error);
        // ❌ Don't close modal on error - let user fix/retry
        throw error;
      }
    },

    // ============================================
    // DELETE TRANSFORM (draft only — never-published transforms)
    // ============================================
    // The backend enforces the never-published rule (409 Conflict
    // otherwise); this just surfaces that error to the caller rather than
    // silently swallowing it. See
    // agents/decisions/0002-transform-draft-lifecycle-decisions.md.
    handleDeleteTransform: async (transformIdToDelete: number) => {
      await deleteTransformMutation.mutateAsync(transformIdToDelete);
      if (selectedTransformId === transformIdToDelete) {
        setSelectedTransformId(null);
      }
    },
    deleteTransformMutation,

    // ============================================
    // SAVE (source or composite graph)
    // ============================================
    handleSave: (params: SaveDraftParams, onSuccess?: () => void) => {
      saveMutation.mutate(params, onSuccess ? { onSuccess } : undefined);
    },
    handleSaveAsync: (params: SaveDraftParams) => saveMutation.mutateAsync(params),
    saveMutation,

    // ============================================
    // VALIDATE — source (code editor "Check") and graph (composite "Validate")
    // ============================================
    handleCheckSource: (
      source_code: string,
      callbacks?: { onSuccess?: () => void; onError?: () => void }
    ) => {
      checkMutation.mutate(source_code, callbacks);
    },
    checkMutation,

    handleValidateGraph: (
      graph: CompositeGraphDefinition,
      callbacks?: { onSuccess?: (result: ValidateGraphResponse) => void; onError?: () => void }
    ) => {
      validateGraphMutation.mutate(graph, callbacks);
    },
    validateGraphMutation,

    // ============================================
    // PUBLISH
    // ============================================
    handlePublish: (params: PublishDraftParams) => {
      publishMutation.mutate(params);
    },
    publishMutation,
  };
}
