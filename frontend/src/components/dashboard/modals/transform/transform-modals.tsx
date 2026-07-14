import { useUIStore } from "@/Stores/UIStore";
import { TransformDetailsModal } from "./transform-details-modal";
import { CreateTransformModal } from "./create-transform-modal";
import { useTransformController } from "@/controllers/TransformController";

export function TransformModals() {
  const modalState = useUIStore((s) => s.modalState);
  const closeModal = useUIStore((s) => s.closeModal);
  const transformController = useTransformController();

  if (!modalState) return null;

  switch (modalState.type) {
    case "createTransform":
      return (
        <CreateTransformModal
          open
          onClose={closeModal}
          onSubmit={transformController.handleSubmitCreateTransform}
        />
      );

    case "transformDetails":
      return (
        <TransformDetailsModal
          transformId={modalState.transformId}
          open
          onClose={closeModal}
        />
      );
  }
}
