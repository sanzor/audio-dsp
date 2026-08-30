import { useState } from "react";
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useCreatorStore, isSourceDirty } from "@/Stores/CreatorStore";
import { useCompositeCanvasStore } from "@/Stores/CompositeCanvasStore";
import { useSaveTransform, useSaveCompositeTransform } from "@/hooks/transforms/mutations";

// Guards a select/create action blocked by unsaved creator work. There are
// two independent flavors of "unsaved" that can trigger `pendingTransformAction`
// (see CreatorStore.requestSelectTransform/requestCreateTransform):
//   - a dirty Rust source buffer in the code editor (editingTransformSource)
//   - a dirty wiring graph on the composite canvas (CompositeCanvasStore)
// The two are mutually exclusive at any given moment: creator-workspace.tsx
// routes a selected transform to exactly one of CreatorCodeEditor or
// CompositeCanvas, never both, and every applied select/create action resets
// *both* editingTransformSource and the composite store (applyTransformAction
// unconditionally calls useCompositeCanvasStore.getState().reset()), so a
// stale dirty flag from a previously-open transform can never survive to
// combine with a different transform's dirty state. That means checking
// sourceDirty first and only falling back to the composite case is safe -
// there's no reachable state where both are simultaneously the reason this
// modal is open.
export function UnsavedCreatorChangesModal() {
  const [isSaving, setIsSaving] = useState(false);
  const pending = useCreatorStore((s) => s.pendingTransformAction);
  const editing = useCreatorStore((s) => s.editingTransformSource);
  const compiledDraftByTransform = useCreatorStore((s) => s.compiledDraftByTransform);
  const resolvePendingTransformAction = useCreatorStore((s) => s.resolvePendingTransformAction);
  const cancelPendingTransformAction = useCreatorStore((s) => s.cancelPendingTransformAction);
  const markTransformSourceSaved = useCreatorStore((s) => s.markTransformSourceSaved);

  const compositeGraph = useCompositeCanvasStore((s) => s.editingGraph);
  const compositeDirty = useCompositeCanvasStore((s) => s.isDirty());
  const markCompositeSaved = useCompositeCanvasStore((s) => s.markSaved);
  const toGraphDefinition = useCompositeCanvasStore((s) => s.toGraphDefinition);

  const sourceDirty = isSourceDirty(editing);
  const isCompositeCase = !sourceDirty && compositeDirty && compositeGraph != null;

  const saveMutation = useSaveTransform(editing?.transformId ?? -1);
  // Save target here is the composite currently open on the canvas (the one
  // being navigated AWAY from), not pending.transformId (the target being
  // navigated TO) - mirrors composite-canvas.tsx's own handleSave.
  const saveCompositeMutation = useSaveCompositeTransform(compositeGraph?.transformId ?? -1);

  if (!pending || (!sourceDirty && !isCompositeCase)) return null;

  // Whichever branch is actually active for this open (mirrors the same
  // isCompositeCase switch used in handleSave) - used to surface that
  // mutation's rejected error inline, same as composite-canvas.tsx's toolbar.
  const activeMutation = isCompositeCase ? saveCompositeMutation : saveMutation;

  const handleDiscard = () => {
    resolvePendingTransformAction();
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      if (isCompositeCase && compositeGraph) {
        await saveCompositeMutation.mutateAsync(toGraphDefinition());
        markCompositeSaved();
      } else if (editing) {
        const compiledDraft = compiledDraftByTransform[editing.transformId];
        await saveMutation.mutateAsync({
          source_code: editing.source,
          wasm_base64: compiledDraft?.sourceCode === editing.source ? compiledDraft.wasmBase64 : undefined,
        });
        markTransformSourceSaved(editing.transformId, editing.source);
      }
      resolvePendingTransformAction();
    } catch {
      // Rejected (e.g. the same composite-validator errors the toolbar's own
      // Save can hit - dangling node, etc). The mutation's isError/error
      // state already reflects this and is rendered below; keep the dialog
      // open so the user can go fix the graph (Stay) or fall back to Discard
      // instead of the save silently appearing to do nothing.
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <Dialog open onOpenChange={cancelPendingTransformAction}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{isCompositeCase ? "Unsaved Composite Graph" : "Unsaved Transform Source"}</DialogTitle>
        </DialogHeader>

        <p className="text-sm text-muted-foreground">
          {isCompositeCase
            ? "You have unsaved changes to this transform's composite graph. Save them before switching, or discard them."
            : "You have unsaved changes to this transform's source. Save them before switching, or discard them."}
        </p>

        {activeMutation.isError && (
          <span className="font-mono text-[10px]" style={{ color: "#ff6b6b" }}>
            {(activeMutation.error as Error | null)?.message}
          </span>
        )}

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
