import { useState } from "react";
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useSaveTransform } from "@/hooks/transforms/mutations";

export function UnsavedTransformSourceModal() {
  const [isSaving, setIsSaving] = useState(false);
  const pending = useCreatorStore((s) => s.pendingTransformAction);
  const editing = useCreatorStore((s) => s.editingTransformSource);
  const resolvePendingTransformAction = useCreatorStore((s) => s.resolvePendingTransformAction);
  const cancelPendingTransformAction = useCreatorStore((s) => s.cancelPendingTransformAction);
  const markTransformSourceSaved = useCreatorStore((s) => s.markTransformSourceSaved);
  const saveMutation = useSaveTransform(editing?.transformId ?? -1);

  if (!pending || !editing) return null;

  const handleDiscard = () => {
    resolvePendingTransformAction();
  };

  const handleSave = async () => {
    try {
      setIsSaving(true);
      await saveMutation.mutateAsync({ source_code: editing.source });
      markTransformSourceSaved(editing.transformId, editing.source);
      resolvePendingTransformAction();
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <Dialog open onOpenChange={cancelPendingTransformAction}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Unsaved Transform Source</DialogTitle>
        </DialogHeader>

        <p className="text-sm text-muted-foreground">
          You have unsaved changes to this transform's source. Save them before switching, or discard them.
        </p>

        <DialogFooter>
          <Button variant="outline" onClick={cancelPendingTransformAction} disabled={isSaving}>
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
