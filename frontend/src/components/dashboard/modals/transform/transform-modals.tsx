import { useUIStore } from "@/Stores/UIStore";
import { TransformDetailsModal } from "./transform-details-modal";

export function TransformModals() {
  const modalState = useUIStore((s) => s.modalState);
  const closeModal = useUIStore((s) => s.closeModal);

  if (!modalState) return null;

  switch (modalState.type) {
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
