// The composite canvas's own boundary-node palette. Used to hold both this
// I/O block section AND a "PUBLISHED TRANSFORMS" draggable leaf list -- that
// list has been folded into transforms-sidebar.tsx (a row there is
// draggable under the same conditions this file used to enforce: composite
// open, primitive + published), so this component now only exposes the
// Input/Output boundary blocks, the one port-creation mechanism that isn't
// changing (see agents/skills/creator -- moving port creation to code was
// considered and rejected; composites have no source code today).
export function CompositePalette() {
  // The composite's own boundary nodes — static UI affordances (not fetched
  // from the API). Distinct dataTransfer key so composite-canvas.tsx's
  // onDrop can tell an IO-node drop apart from a transform-leaf drop
  // (dropped from transforms-sidebar.tsx via the "application/transform" key).
  function onIoDragStart(e: React.DragEvent, direction: "input" | "output") {
    e.dataTransfer.setData("application/composite-io", JSON.stringify({ direction }));
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
    </aside>
  );
}
