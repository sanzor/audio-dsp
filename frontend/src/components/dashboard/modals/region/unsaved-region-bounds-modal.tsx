import { useState } from "react";
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Button } from "@/components/ui/button";
import type { ActiveSelection } from "@/Stores/UIStore";
import { useUIStore } from "@/Stores/UIStore";
import { useRegionController } from "@/controllers/RegionController";

interface UnsavedRegionBoundsModalProps {
  open: boolean;
  nextSelection: ActiveSelection;
  onClose: () => void;
}

export function UnsavedRegionBoundsModal({
  open,
  nextSelection,
  onClose,
}: UnsavedRegionBoundsModalProps) {
  const [isSaving, setIsSaving] = useState(false);
  const editingRegionBounds = useUIStore((state) => state.editingRegionBounds);
  const applyActiveSelection = useUIStore((state) => state.applyActiveSelection);
  const clearEditingRegionBounds = useUIStore((state) => state.clearEditingRegionBounds);
  const regionController = useRegionController();

  const handleDiscard = () => {
    clearEditingRegionBounds();
    applyActiveSelection(nextSelection);
    onClose();
  };

  const handleSave = async () => {
    if (!editingRegionBounds) {
      applyActiveSelection(nextSelection);
      onClose();
      return;
    }

    try {
      setIsSaving(true);
      await regionController.handleUpdateRegionBounds(
        editingRegionBounds.regionId,
        editingRegionBounds.start,
        editingRegionBounds.end,
      );
      clearEditingRegionBounds();
      applyActiveSelection(nextSelection);
      onClose();
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Unsaved Region Bounds</DialogTitle>
        </DialogHeader>

        <p className="text-sm text-muted-foreground">
          You have unsaved region bound changes. Save them before changing selection, or discard them.
        </p>

        <DialogFooter>
          <Button variant="outline" onClick={onClose} disabled={isSaving}>
            Stay
          </Button>
          <Button variant="outline" onClick={handleDiscard} disabled={isSaving}>
            Discard
          </Button>
          <Button onClick={() => void handleSave()} disabled={isSaving}>
            {isSaving ? "Saving..." : "Save"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
