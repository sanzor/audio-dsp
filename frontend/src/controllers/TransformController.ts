// hooks/useTransformController.ts
import { useUIStore } from "@/Stores/UIStore";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCreateTransform, useDeleteTransform } from "@/hooks/transforms/mutations";
import type { CreateTransformParams } from "@/Services/TransformService";

export function useTransformController() {
  // Zustand selectors
  const closeModal = useUIStore((state) => state.closeModal);
  const openModal = useUIStore((state) => state.openModal);
  const setSelectedTransformId = useCreatorStore((state) => state.setSelectedTransformId);
  const selectedTransformId = useCreatorStore((state) => state.selectedTransformId);

  // Data and mutations
  const createTransformMutation = useCreateTransform();
  const deleteTransformMutation = useDeleteTransform();

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
    handleDeleteTransform: async (transformId: number) => {
      await deleteTransformMutation.mutateAsync(transformId);
      if (selectedTransformId === transformId) {
        setSelectedTransformId(null);
      }
    },
    deleteTransformMutation,
  };
}
