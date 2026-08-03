import { useListTransforms } from "@/hooks/transforms/queries";

// Draggable list of existing transforms a composite can be built from.
// Filtered to published primitives only — a composite has no transform_binary
// row (no wasm bytes) and graph-worklet.js can only WebAssembly.instantiate a
// real binary, so neither an unpublished draft nor another composite can be
// wired in as a leaf yet (the backend validator independently re-enforces
// both — see composite_validator.rs). Uses the exact same native
// drag-and-drop convention as the Editor's palette (TransformItem.tsx):
// dataTransfer key "application/transform", payload {transformId, name}.
export function CompositePalette() {
  const query = useListTransforms();
  const transforms = (query.data?.pages.flatMap((p) => p.transforms) ?? []).filter(
    (t) => t.kind === "primitive" && t.published
  );

  function onDragStart(e: React.DragEvent, transformId: number, name: string) {
    e.dataTransfer.setData("application/transform", JSON.stringify({ transformId, name }));
    e.dataTransfer.effectAllowed = "move";
  }

  // The composite's own boundary nodes — static UI affordances (not fetched
  // from the API, unlike the transform list below). Distinct dataTransfer
  // key so composite-canvas.tsx's onDrop can tell an IO-node drop apart from
  // a transform-leaf drop.
  function onIoDragStart(e: React.DragEvent, direction: "input" | "output") {
    e.dataTransfer.setData("application/composite-io", JSON.stringify({ direction }));
    e.dataTransfer.effectAllowed = "move";
  }

  return (
    <aside
      className="flex flex-col w-60 flex-shrink-0 overflow-hidden"
      style={{ backgroundColor: "var(--bg-darker)", borderRight: "1px solid rgba(255,255,255,0.06)" }}
    >
      <div className="px-3 py-2" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
        <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
          COMPOSITE I/O
        </span>
        <p className="mt-1 text-[9px]" style={{ color: "var(--text-muted)", opacity: 0.7 }}>
          Drag onto the canvas to create this composite's boundary.
        </p>
      </div>
      <div className="p-2 flex flex-col gap-1" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
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
      <div className="px-3 py-2" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
        <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
          PUBLISHED TRANSFORMS
        </span>
        <p className="mt-1 text-[9px]" style={{ color: "var(--text-muted)", opacity: 0.7 }}>
          Drag onto the canvas to wire in.
        </p>
      </div>
      <div className="flex-1 overflow-y-auto p-2 flex flex-col gap-1">
        {query.isLoading && (
          <span className="text-[10px] font-mono" style={{ color: "var(--text-muted)" }}>
            Loading…
          </span>
        )}
        {!query.isLoading && transforms.length === 0 && (
          <span className="text-[10px] font-mono" style={{ color: "var(--text-muted)", opacity: 0.6 }}>
            No published primitive transforms yet.
          </span>
        )}
        {transforms.map((t) => (
          <div
            key={t.transform_id}
            draggable
            onDragStart={(e) => onDragStart(e, t.transform_id, t.name)}
            className="px-2 py-1.5 rounded text-xs cursor-grab"
            style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.05)", color: "var(--text-main)" }}
            title={t.description}
          >
            {t.name}
          </div>
        ))}
        {query.hasNextPage && (
          <button
            onClick={() => query.fetchNextPage()}
            className="text-[9px] font-mono py-1"
            style={{ color: "var(--text-muted)" }}
          >
            Load more…
          </button>
        )}
      </div>
    </aside>
  );
}
