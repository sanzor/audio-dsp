import type { UseMutationResult } from "@tanstack/react-query";
import { apiGetPublishPortShapeDiff, type PortShapeSummary } from "@/Services/TransformService";
import type { TransformDefinition } from "@/domain/Transform/TransformDefinition";

// Republish port-shape warning: an advisory, non-blocking pre-check.
// Never blocks Publish itself — if the check request fails, we proceed as
// if nothing changed rather than leaving the creator stuck. See
// agents/decisions/0004-multi-input-named-ports.md.
export function usePublishWithPortShapeDiff(
  transformId: number | null,
  publishMutation: UseMutationResult<TransformDefinition, Error, void, unknown>
) {
  async function handlePublish() {
    if (transformId == null) return;

    try {
      const diff = await apiGetPublishPortShapeDiff(transformId);
      if (diff.changed) {
        const describe = (ports: PortShapeSummary[]) =>
          ports.length === 0
            ? "(none)"
            : ports.map((p) => `${p.name} [${p.direction}, ${p.kind}/${p.cardinality}]`).join(", ");
        const proceed = window.confirm(
          "This transform's port shape has changed since it was last published.\n\n" +
            `Currently published: ${describe(diff.current)}\n` +
            `About to publish: ${describe(diff.incoming)}\n\n` +
            "Editor graphs already wired to the old shape will fail closed with a visible error rather than silently misrouting audio, but they will need to be re-wired. Publish anyway?"
        );
        if (!proceed) return;
      }
    } catch {
      // Advisory only — never block the actual publish on this check failing.
    }

    publishMutation.mutate();
  }

  return { handlePublish };
}
