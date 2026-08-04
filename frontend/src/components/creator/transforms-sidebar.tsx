import { useState } from "react";
import { Plus, Trash2 } from "lucide-react";
import { useListTransforms, useGetTransformDefinition } from "@/hooks/transforms/queries";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useTransformController } from "@/controllers/TransformController";
import { creatorToolbarColors } from "./creatorToolbarColors";
import type { TransformSummary } from "@/domain/Transform/TransformSummary";

// Persistent per-kind color used for the small dot before each row's name --
// sibling convention to composite-node-header.tsx's SAFETY_COLOR (a style
// reference only, not reused directly: that one is a three-state
// safe/load-bearing/disabled indicator, this is a fixed two-state kind
// indicator that never changes for a given row).
const KIND_DOT_COLOR: Record<TransformSummary["kind"], string> = {
  primitive: creatorToolbarColors.blue.color,
  composite: creatorToolbarColors.pink.color,
};

const KIND_LABEL: Record<TransformSummary["kind"], string> = {
  primitive: "Primitive transform",
  composite: "Composite transform",
};

export function TransformsSidebar() {
  const [filter, setFilter] = useState("");
  // Tracks which row's delete failed (e.g. 409 — already published), so the
  // error can be shown inline next to that specific row rather than
  // globally, mirroring the inline error pattern already used for
  // publish errors in code-editor.tsx.
  const [deleteErrorFor, setDeleteErrorFor] = useState<{ transformId: number; message: string } | null>(null);

  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const requestSelectTransform = useCreatorStore((s) => s.requestSelectTransform);
  const requestCreateTransform = useCreatorStore((s) => s.requestCreateTransform);
  const { handleDeleteTransform, deleteTransformMutation } = useTransformController();

  const query = useListTransforms();

  // Drives whether any row is draggable at all -- a leaf can only be
  // inserted onto a composite's canvas, so dragging only makes sense while a
  // composite is the currently open transform (folded in from the former
  // composite-palette.tsx "PUBLISHED TRANSFORMS" list, which enforced the
  // same gating).
  const { data: selectedDefinition } = useGetTransformDefinition(selectedId);
  const isCompositeOpen = selectedDefinition?.transform_id === selectedId && selectedDefinition?.kind === "composite";

  async function onDelete(transformId: number, e: React.MouseEvent) {
    e.stopPropagation();
    setDeleteErrorFor(null);
    try {
      await handleDeleteTransform(transformId);
    } catch (error) {
      setDeleteErrorFor({ transformId, message: error instanceof Error ? error.message : "Failed to delete" });
    }
  }

  // Same dataTransfer key/payload shape as the former composite-palette.tsx
  // onDragStart, so composite-canvas.tsx's onDrop handler needs zero changes.
  function onTransformDragStart(e: React.DragEvent, transformId: number, name: string) {
    e.dataTransfer.setData("application/transform", JSON.stringify({ transformId, name }));
    e.dataTransfer.effectAllowed = "move";
  }

  const allTransforms = query.data?.pages.flatMap((p) => p.transforms) ?? [];
  const filtered = filter
    ? allTransforms.filter((t) => t.name.toLowerCase().includes(filter.toLowerCase()))
    : allTransforms;

  return (
    <aside
      className="flex flex-col w-64 flex-shrink-0 overflow-hidden"
      style={{
        backgroundColor: "var(--bg-darker)",
        borderRight: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      {/* Header */}
      <div
        className="px-3 py-2.5 flex items-center justify-between"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
      >
        <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
          TRANSFORMS
        </span>
        <button
          onClick={requestCreateTransform}
          className="w-6 h-6 rounded flex items-center justify-center hover:bg-white/10 transition-colors"
          title="New Transform"
          style={{ background: "none", border: "none", padding: 0, cursor: "pointer" }}
        >
          <Plus className="w-4 h-4" style={{ color: "var(--text-muted)" }} />
        </button>
      </div>

      {/* Filter input */}
      <div className="px-3 py-2" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
        <input
          type="text"
          placeholder="Filter..."
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          className="w-full rounded px-2 py-1 text-xs"
          style={{
            backgroundColor: "var(--bg-dark)",
            border: "1px solid rgba(255,255,255,0.08)",
            color: "var(--text-main)",
            outline: "none",
          }}
        />
      </div>

      {/* List */}
      <div className="flex-1 overflow-y-auto py-1">
        {query.isLoading && (
          <div className="px-4 py-3 text-xs" style={{ color: "var(--text-muted)" }}>
            Loading...
          </div>
        )}
        {filtered.map((t) => {
          // A row is draggable-onto-canvas only when a composite is open AND
          // the row itself is a published primitive -- composites can't be
          // leaves yet, and an unpublished draft has no transform_binary for
          // graph-worklet.js to instantiate. Same restriction the former
          // composite-palette.tsx "PUBLISHED TRANSFORMS" list enforced.
          const isDraggable = isCompositeOpen && t.kind === "primitive" && t.published;
          const isSelected = selectedId === t.transform_id;

          const rowStyle: React.CSSProperties = isSelected
            ? { color: "#adc6ff", borderLeft: "2px solid #adc6ff", backgroundColor: "rgba(173,198,255,0.08)" }
            : isDraggable
              ? {
                  color: "var(--text-muted)",
                  borderLeft: "2px solid rgba(74,225,118,0.35)",
                  backgroundColor: "rgba(74,225,118,0.04)",
                }
              : { color: "var(--text-muted)", borderLeft: "2px solid transparent" };

          return (
            <div
              key={t.transform_id}
              className="group relative"
              draggable={isDraggable}
              onDragStart={isDraggable ? (e) => onTransformDragStart(e, t.transform_id, t.name) : undefined}
              title={isDraggable ? "Drag onto the composite canvas to insert" : undefined}
              style={{ cursor: isDraggable ? "grab" : "default" }}
            >
              <button
                onClick={() => requestSelectTransform(t.transform_id)}
                className="w-full flex flex-col h-auto py-2 pl-4 pr-8 text-left transition-colors"
                style={rowStyle}
              >
                <span className="flex items-center gap-1.5 w-full min-w-0">
                  <span
                    className="w-1.5 h-1.5 rounded-full flex-shrink-0"
                    style={{ backgroundColor: KIND_DOT_COLOR[t.kind] }}
                    title={KIND_LABEL[t.kind]}
                  />
                  <span className="text-xs truncate">{t.name}</span>
                </span>
                {t.description && (
                  <span
                    className="text-[10px] mt-0.5 truncate w-full"
                    style={{ color: "var(--text-muted)", opacity: 0.7 }}
                  >
                    {t.description}
                  </span>
                )}
              </button>
              <button
                onClick={(e) => onDelete(t.transform_id, e)}
                disabled={deleteTransformMutation.isPending}
                title="Delete draft (only allowed if never published)"
                className="absolute right-2 top-2 w-5 h-5 rounded flex items-center justify-center opacity-0 group-hover:opacity-100 hover:bg-white/10 transition-opacity"
                style={{ background: "none", border: "none", padding: 0, cursor: "pointer" }}
              >
                <Trash2 className="w-3 h-3" style={{ color: "#ff8a8a" }} />
              </button>
              {deleteErrorFor != null && deleteErrorFor.transformId === t.transform_id && (
                <div
                  className="px-4 pb-1.5 text-[10px] font-mono truncate"
                  title={deleteErrorFor.message}
                  style={{ color: "#ff8a8a" }}
                >
                  {deleteErrorFor.message}
                </div>
              )}
            </div>
          );
        })}

        {!query.isLoading && filtered.length === 0 && (
          <div className="px-4 py-3 text-xs" style={{ color: "var(--text-muted)" }}>
            {filter ? "No matches." : "No transforms yet."}
          </div>
        )}

        {query.hasNextPage && (
          <button
            onClick={() => query.fetchNextPage()}
            disabled={query.isFetchingNextPage}
            className="w-full text-[10px] font-mono py-2 transition-colors"
            style={{ color: "var(--text-muted)" }}
          >
            {query.isFetchingNextPage ? "Loading..." : "Load more"}
          </button>
        )}
      </div>
    </aside>
  );
}
