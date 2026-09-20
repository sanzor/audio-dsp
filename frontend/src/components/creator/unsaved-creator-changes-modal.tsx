import { useState } from "react";
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useCreatorStore, isSourceDirty, isMetadataDirty } from "@/Stores/CreatorStore";
import { useCompositeCanvasStore } from "@/Stores/CompositeCanvasStore";
import { useTransformDraftController } from "@/controllers/TransformDraftController";

// Guards a select/create action blocked by unsaved creator work. There are
// three independent flavors of "unsaved" that can trigger `pendingTransformAction`
// (see CreatorStore.requestSelectTransform/requestCreateTransform):
//   - a dirty Rust source buffer in the code editor (editingTransformSource)
//   - a dirty wiring graph on the composite canvas (CompositeCanvasStore)
//   - a dirty name/description buffer in the properties panel
//     (editingTransformMetadata) — this used to persist independently via a
//     PATCH endpoint on blur, but as of
//     agents/decisions/0012-draft-name-description-editable.md, name/
//     description only ever persist through Save, so a metadata-only edit
//     left unsaved when switching transforms must be caught here too, or
//     it's silently lost with no warning.
// The source/composite flavors are mutually exclusive at any given moment:
// creator-workspace.tsx routes a selected transform to exactly one of
// CreatorCodeEditor or CompositeCanvas, never both, and every applied
// select/create action resets editingTransformSource, editingTransformMetadata,
// and the composite store together (applyTransformAction unconditionally
// calls useCompositeCanvasStore.getState().reset()), so a stale dirty flag
// from a previously-open transform can never survive to combine with a
// different transform's dirty state. The properties panel (and its metadata
// buffer) is always mounted alongside whichever of the other two is active,
// so metadataDirty can coexist with either sourceDirty or the composite case
// — the case distinction below only decides which save payload shape
// (source_code vs graph_definition) to use; the metadata is folded into
// whichever one applies.
export function UnsavedCreatorChangesModal() {
  const [isSaving, setIsSaving] = useState(false);
  // Set only by the "neither surface has loaded" guard in handleSave below —
  // a manually-thrown bail-out there never touches saveMutation, so its own
  // isError/error state (rendered further down) would never reflect this
  // case. Cleared whenever the dialog is dismissed (Stay/Discard) or Save is
  // retried, so a stale message can't outlive the state that produced it.
  const [blockedMessage, setBlockedMessage] = useState<string | null>(null);
  const pending = useCreatorStore((s) => s.pendingTransformAction);
  const editing = useCreatorStore((s) => s.editingTransformSource);
  const editingMetadata = useCreatorStore((s) => s.editingTransformMetadata);
  const resolvePendingTransformAction = useCreatorStore((s) => s.resolvePendingTransformAction);
  const cancelPendingTransformAction = useCreatorStore((s) => s.cancelPendingTransformAction);
  const markTransformSourceSaved = useCreatorStore((s) => s.markTransformSourceSaved);
  const markTransformMetadataSaved = useCreatorStore((s) => s.markTransformMetadataSaved);

  const compositeGraph = useCompositeCanvasStore((s) => s.editingGraph);
  const compositeDirty = useCompositeCanvasStore((s) => s.isDirty());
  const markCompositeSaved = useCompositeCanvasStore((s) => s.markSaved);
  const toGraphDefinition = useCompositeCanvasStore((s) => s.toGraphDefinition);

  const sourceDirty = isSourceDirty(editing);
  // Which surface is currently open, independent of dirtiness — decides the
  // save payload shape for a metadata-only edit (no source/graph change),
  // since Save always requires one of source_code/graph_definition.
  const surfaceIsComposite = compositeGraph != null;
  const isCompositeCase = !sourceDirty && compositeDirty && surfaceIsComposite;
  const metadataDirty = isMetadataDirty(editingMetadata);
  const isMetadataOnlyCase = !sourceDirty && !isCompositeCase && metadataDirty;

  // Save target is whichever transform is actually open (the one being
  // navigated AWAY from), not pending.transformId (the target being
  // navigated TO) - mirrors composite-canvas.tsx's own handleSave for the
  // composite case. One unified mutation instance now that useSaveTransform
  // takes a SaveDraftParams variant instead of two separate hooks.
  const targetTransformId = surfaceIsComposite
    ? compositeGraph?.transformId ?? -1
    : editing?.transformId ?? editingMetadata?.transformId ?? -1;
  const { handleSaveAsync, saveMutation } = useTransformDraftController(targetTransformId);

  if (!pending || (!sourceDirty && !isCompositeCase && !isMetadataOnlyCase)) return null;

  const handleDiscard = () => {
    setBlockedMessage(null);
    resolvePendingTransformAction();
  };

  const handleStay = () => {
    setBlockedMessage(null);
    cancelPendingTransformAction();
  };

  const handleSave = async () => {
    setBlockedMessage(null);
    setIsSaving(true);
    try {
      const metadataPayload =
        metadataDirty && editingMetadata != null
          ? { name: editingMetadata.name, description: editingMetadata.description }
          : {};
      if (surfaceIsComposite && compositeGraph) {
        await handleSaveAsync({ graph_definition: toGraphDefinition(), ...metadataPayload });
        markCompositeSaved();
      } else if (editing) {
        await handleSaveAsync({ source_code: editing.source, ...metadataPayload });
        markTransformSourceSaved(editing.transformId, editing.source);
      } else {
        // Neither surface is populated to save through. Reachable for a
        // metadata-only edit on a composite draft made before the composite
        // canvas's own graph-definition query has resolved: compositeGraph
        // (CompositeCanvasStore.editingGraph) is only set once that query
        // lands, and composite drafts never populate `editing`
        // (editingTransformSource is a primitive-only buffer). Bail out
        // visibly instead of silently skipping the save — otherwise the
        // code below would still mark the metadata saved and resolve the
        // pending action, discarding the edit with no sign anything went
        // wrong (see agents/decisions/0012-draft-name-description-editable.md
        // regression notes).
        setBlockedMessage(
          "This transform hasn't finished loading yet — wait a moment and try Save again, or Discard to lose this edit."
        );
        return;
      }
      if (metadataDirty && editingMetadata != null) {
        markTransformMetadataSaved(targetTransformId, editingMetadata.name, editingMetadata.description);
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

  const title = isCompositeCase
    ? "Unsaved Composite Graph"
    : sourceDirty
      ? "Unsaved Transform Source"
      : "Unsaved Transform Metadata";
  const description = isCompositeCase
    ? "You have unsaved changes to this transform's composite graph. Save them before switching, or discard them."
    : sourceDirty
      ? "You have unsaved changes to this transform's source. Save them before switching, or discard them."
      : "You have an unsaved name/description edit for this transform. Save it before switching, or discard it.";

  return (
    <Dialog open onOpenChange={handleStay}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>

        <p className="text-sm text-muted-foreground">{description}</p>

        {blockedMessage ? (
          <span className="font-mono text-[10px]" style={{ color: "#ff6b6b" }}>
            {blockedMessage}
          </span>
        ) : (
          saveMutation.isError && (
            <span className="font-mono text-[10px]" style={{ color: "#ff6b6b" }}>
              {(saveMutation.error as Error | null)?.message}
            </span>
          )
        )}

        <DialogFooter>
          <Button variant="outline" onClick={handleStay} disabled={isSaving}>
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
