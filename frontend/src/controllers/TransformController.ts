// hooks/useTransformController.ts
import { useUIStore } from "@/Stores/UIStore";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCreateTransform } from "@/hooks/transforms/mutations";
import type { CreateTransformParams } from "@/Services/TransformService";

export function useTransformController() {
  // Zustand selectors
  const closeModal = useUIStore((state) => state.closeModal);
  const openModal = useUIStore((state) => state.openModal);
  const setSelectedTransformId = useCreatorStore((state) => state.setSelectedTransformId);

  // Data and mutations
  const createTransformMutation = useCreateTransform();

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
  };
}
