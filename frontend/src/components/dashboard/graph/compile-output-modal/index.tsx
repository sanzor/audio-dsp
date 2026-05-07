import type { CompileLogEntry, CompileRun, CompileStep, CompileStepStatus } from "@/Stores/AudioEffectsStore";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

interface CompileOutputModalProps {
  open: boolean;
  run: CompileRun | null;
  onClear: () => void;
  onOpenChange: (open: boolean) => void;
}

const STEP_STATUS_STYLES: Record<CompileStepStatus, { dot: string; text: string }> = {
  pending: { dot: "rgba(148, 163, 184, 0.75)", text: "var(--text-muted)" },
  running: { dot: "rgba(96, 165, 250, 0.9)", text: "#dbeafe" },
  success: { dot: "rgba(74, 222, 128, 0.95)", text: "#dcfce7" },
  warning: { dot: "rgba(251, 191, 36, 0.95)", text: "#fde68a" },
  error: { dot: "rgba(248, 113, 113, 0.95)", text: "#fecaca" },
};

const LOG_COLORS: Record<CompileLogEntry["level"], string> = {
  info: "var(--text-muted)",
  success: "#86efac",
  warning: "#fcd34d",
  error: "#fca5a5",
};

function formatClock(timestamp: number): string {
  return new Date(timestamp).toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
}

function renderStep(step: CompileStep) {
  const style = STEP_STATUS_STYLES[step.status];

  return (
    <div
      key={step.id}
      className="rounded-md border px-3 py-2"
      style={{
        borderColor: "rgba(255,255,255,0.08)",
        background: "rgba(255,255,255,0.025)",
      }}
    >
      <div className="flex items-center gap-2 text-sm" style={{ color: style.text }}>
        <span
          className="inline-block h-2.5 w-2.5 rounded-full"
          style={{ background: style.dot }}
        />
        <span>{step.label}</span>
      </div>
      {step.detail && (
        <div className="mt-1 text-xs leading-5" style={{ color: "var(--text-muted)" }}>
          {step.detail}
        </div>
      )}
    </div>
  );
}

export function CompileOutputModal({
  open,
  run,
  onClear,
  onOpenChange,
}: CompileOutputModalProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        showCloseButton
        className="max-w-5xl border p-0 sm:max-w-5xl"
        style={{
          background: "linear-gradient(180deg, rgba(8, 15, 30, 0.98), rgba(4, 10, 20, 0.98))",
          borderColor: "rgba(255,255,255,0.12)",
          color: "var(--text-main)",
        }}
      >
        <div
          className="flex items-center justify-between gap-3 px-4 py-3"
          style={{ borderBottom: "1px solid rgba(255,255,255,0.08)" }}
        >
          <div className="flex items-center gap-2">
            <div
              className="rounded-t-md border border-b-0 px-3 py-1 text-xs font-medium"
              style={{
                borderColor: "rgba(255,255,255,0.12)",
                background: "rgba(255,255,255,0.07)",
                color: "var(--text-main)",
              }}
            >
              Compile
            </div>
          </div>
          <button
            type="button"
            onClick={onClear}
            className="rounded px-2 py-1 text-xs"
            style={{ color: "var(--text-muted)", background: "rgba(255,255,255,0.04)" }}
          >
            Clear
          </button>
        </div>

        <DialogHeader className="px-4 pt-4">
          <DialogTitle>Compile Output</DialogTitle>
          <DialogDescription>
            Manual compile opens this dialog and streams the current run here.
          </DialogDescription>
        </DialogHeader>

        <div className="grid min-h-[520px] grid-cols-1 gap-0 md:grid-cols-[320px_minmax(0,1fr)]">
          <div
            className="border-r px-4 py-4"
            style={{ borderColor: "rgba(255,255,255,0.08)" }}
          >
            <div className="mb-3 text-xs uppercase tracking-[0.16em]" style={{ color: "var(--text-muted)" }}>
              Steps
            </div>
            {run ? (
              <div className="space-y-3">
                <div
                  className="rounded-md border px-3 py-2 text-sm"
                  style={{
                    borderColor: "rgba(255,255,255,0.08)",
                    background: "rgba(255,255,255,0.025)",
                    color: "var(--text-main)",
                  }}
                >
                  <div>{run.trigger} compile</div>
                  <div className="text-xs" style={{ color: "var(--text-muted)" }}>
                    {run.status} • {formatClock(run.startedAt)}
                  </div>
                </div>
                <div className="space-y-2">
                  {run.steps.map(renderStep)}
                </div>
              </div>
            ) : (
              <div className="text-sm" style={{ color: "var(--text-muted)" }}>
                No compile output yet.
              </div>
            )}
          </div>

          <div className="min-h-0 px-4 py-4">
            <div className="mb-3 text-xs uppercase tracking-[0.16em]" style={{ color: "var(--text-muted)" }}>
              Log
            </div>
            <div className="max-h-[420px] space-y-3 overflow-auto pr-1 font-mono text-xs">
              {!run ? (
                <div style={{ color: "var(--text-muted)" }}>Compile messages will appear here.</div>
              ) : run.logs.length === 0 ? (
                <div style={{ color: "var(--text-muted)" }}>No log lines recorded.</div>
              ) : (
                <div
                  className="rounded-md border p-3"
                  style={{
                    borderColor: "rgba(255,255,255,0.08)",
                    background: "rgba(255,255,255,0.025)",
                  }}
                >
                  <div className="mb-2 text-[11px]" style={{ color: "var(--text-muted)" }}>
                    current run
                  </div>
                  <div className="space-y-1.5">
                    {run.logs.map((entry) => (
                      <div key={entry.id} className="flex gap-3">
                        <span style={{ color: "rgba(148, 163, 184, 0.9)" }}>{formatClock(entry.at)}</span>
                        <span style={{ color: LOG_COLORS[entry.level], minWidth: 56 }}>
                          {entry.level.toUpperCase()}
                        </span>
                        <span style={{ color: "var(--text-main)" }}>{entry.message}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
