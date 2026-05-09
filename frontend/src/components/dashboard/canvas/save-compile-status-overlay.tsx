export type SaveCompileStatusState = "hidden" | "waiting" | "error";

interface SaveCompileStatusOverlayProps {
  state: SaveCompileStatusState;
  message: string | null;
}

export function SaveCompileStatusOverlay({
  state,
  message,
}: SaveCompileStatusOverlayProps) {
  if (state === "hidden") return null;

  return (
    <div className="pointer-events-none absolute inset-0 z-30 flex items-center justify-center px-4">
      <div
        className="max-w-sm rounded-xl border px-4 py-3 shadow-xl"
        style={{
          background: "rgba(8, 15, 30, 0.96)",
          borderColor:
            state === "waiting"
              ? "rgba(96, 165, 250, 0.4)"
              : "rgba(248, 113, 113, 0.45)",
          color: "var(--text-main)",
        }}
      >
        <div className="flex items-center gap-3">
          {state === "waiting" ? (
            <div
              className="h-4 w-4 animate-spin rounded-full border-2 border-white/20 border-t-sky-300"
              aria-hidden="true"
            />
          ) : (
            <div
              className="h-2.5 w-2.5 rounded-full"
              style={{ background: "rgba(248, 113, 113, 0.95)" }}
              aria-hidden="true"
            />
          )}
          <div>
            <div className="text-sm font-medium">
              {state === "waiting" ? "Compiling saved graph..." : "Compile after save failed"}
            </div>
            {message && (
              <div className="text-xs leading-5" style={{ color: "var(--text-muted)" }}>
                {message}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
