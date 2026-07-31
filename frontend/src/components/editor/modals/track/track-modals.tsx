import { useUIStore } from "@/Stores/UIStore";
import { useTrackController } from "@/controllers/TrackController";
import { DetailsTrackModal } from "./details-track-modal";
import { RenameTrackModal } from "./rename-track-modal";
import { CreateTrackModal } from "./create-track-modal";
import { PasteTrackModal } from "./paste-track-modal";

export function TrackModals() {
  const modalState = useUIStore(state => state.modalState);
  const closeModal = useUIStore(state => state.closeModal);
  const trackController = useTrackController();

  if (!modalState) return null;

  switch (modalState.type) {
    case "detailsTrack":
      return <DetailsTrackModal trackId={modalState.trackId} open onClose={closeModal} />;

    case "renameTrack":
      return <RenameTrackModal
        trackId={modalState.trackId}
        open
        onClose={closeModal}
        onSubmit={trackController.handleSubmitRenameTrack}
    />;

    case "createTrack":
      return (
        <CreateTrackModal
          canonicalAudio={modalState.canonicalAudio}
          onSubmit={trackController.handleSubmitCreateTrack}
          open
          onClose={closeModal}
        />
      );

    case "pasteTrack":
      return (
        <PasteTrackModal
          trackId={modalState.trackId}
          onSubmit={trackController.handleSubmitPasteTrack}  
          open
          onClose={closeModal}
        />
      );
    default:
      return null;
  }
}