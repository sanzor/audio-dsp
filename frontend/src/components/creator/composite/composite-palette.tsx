import { usePublishedTransforms } from "@/hooks/transforms/queries";

// The composite canvas's own palette: the I/O boundary blocks (static, not
// fetched) plus "the store" -- published transforms draggable onto the
// canvas as leaves. Only a published primitive is draggable: composites
// can't be leaves yet, and an unpublished draft has no resolvable artifact
// for graph-worklet.js to instantiate. usePublishedTransforms already only
// returns published rows (GET /v1/workspaces/{id}/transforms/published), so
// no per-row published check is needed here -- unlike the general
// transforms-sidebar.tsx browse/select/edit/delete list, which deliberately
// includes the caller's own unpublished drafts and has nothing to do with
// dragging.
export function CompositePalette() {
  const { data: publishedTransforms = [] } = usePublishedTransforms();
  const draggableLeaves = publishedTransforms.filter((t) => t.kind === "primitive");

  // Static UI affordances. Distinct dataTransfer key so composite-canvas.tsx's
  // onDrop can tell an IO-node drop apart from a transform-leaf drop.
  function onIoDragStart(e: React.DragEvent, direction: "input" | "output") {
    e.dataTransfer.setData("application/composite-io", JSON.stringify({ direction }));
    e.dataTransfer.effectAllowed = "move";
  }

  function onTransformDragStart(e: React.DragEvent, transformId: number, name: string) {
    e.dataTransfer.setData("application/transform", JSON.stringify({ transformId, name }));
    e.dataTransfer.effectAllowed = "move";
  }

  return (
    <aside
      className="flex flex-col w-36 flex-shrink-0 overflow-hidden"
      style={{ backgroundColor: "var(--bg-darker)", borderRight: "1px solid rgba(255,255,255,0.06)" }}
    >
      <div className="px-3 py-2" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
        <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
          COMPOSITE I/O
        </span>
      </div>
      <div className="p-2 flex flex-col gap-1">
        <div
          draggable
          onDragStart={(e) => onIoDragStart(e, "input")}
          className="px-2 py-1.5 rounded text-xs cursor-grab"
          style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(173,198,255,0.25)", color: "#adc6ff" }}
        >
          Input
        </div>
        <div
          draggable
          onDragStart={(e) => onIoDragStart(e, "output")}
          className="px-2 py-1.5 rounded text-xs cursor-grab"
          style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(74,225,118,0.25)", color: "#4ae176" }}
        >
          Output
        </div>
      </div>

      <div className="px-3 py-2" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)", borderTop: "1px solid rgba(255,255,255,0.06)" }}>
        <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
          STORE
        </span>
      </div>
      <div className="flex-1 min-h-0 overflow-y-auto p-2 flex flex-col gap-1">
        {draggableLeaves.length === 0 && (
          <span className="text-[10px]" style={{ color: "var(--text-muted)", opacity: 0.7 }}>
            No published transforms yet.
          </span>
        )}
        {draggableLeaves.map((t) => (
          <div
            key={t.transform_id}
            draggable
            onDragStart={(e) => onTransformDragStart(e, t.transform_id, t.name)}
            title="Drag onto the canvas to insert"
            className="px-2 py-1.5 rounded text-xs cursor-grab truncate"
            style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.08)", color: "var(--text-muted)" }}
          >
            {t.name}
          </div>
        ))}
      </div>
    </aside>
  );
}
