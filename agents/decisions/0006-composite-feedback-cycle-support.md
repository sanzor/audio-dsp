# 0006: Composite transforms support internal feedback cycles

- **Status:** Accepted
- **Date:** 2026-07-31

## Context

Question investigated: do composite transforms (Creator's composite canvas, `frontend/src/components/creator/composite/**`) support an internal feedback cycle — a connection that loops back on itself inside the composite's own wiring?

Investigation found that composite preview compilation does not have its own graph-compilation logic. `frontend/src/components/creator/composite/composite-preview-controls.ts` imports directly from the Editor-owned `frontend/src/audio/pipeline/GraphCompiler.ts`:

```ts
import { process as compileGraphInput, inputPortCountOf, inputPortIndexByName, type GraphInput } from "@/audio/pipeline/GraphCompiler";
```

and calls `compileGraphInput` every time a composite's "Try it" preview compiles. `GraphCompiler.ts`'s `process()` does DFS back-edge detection, topo-sort excluding back-edges, and feedback-buffer assignment — the exact same mechanism Editor graphs use for their own feedback loops. This is pre-existing, working code; it was not added as part of this investigation.

Separately, while confirming the cyclic-shape coverage was actually correct end-to-end, a real bug was found and fixed by `editor-agent` in `GraphCompiler.ts`/`graph-worklet.js`: `buildNodes()` only reserved a node's output buffer write based on `hasForwardOut`, so a back-edge source node with no other forward outgoing edge (the simplest cyclic shape — a chain whose last node loops back to an earlier one) got `outputBufferIndex: -1`, and `graph-worklet.js` never wrote that node's audio into its reserved feedback buffer. Fix: `CompiledNode` now carries an independent `writesToOutput` field (true whenever a node has no forward consumer), `outputBufferIndex` is reserved whenever `hasForwardOut || hasBackOut`, and the worklet codegen emits the buffer write and the output write as two independent `if`s instead of a mutually-exclusive `if`/`else`. This is verified by 7 passing tests in `frontend/src/audio/pipeline/GraphCompiler.test.ts`: a `describe("GraphCompiler cycle handling (composite preview call path)")` block (4 tests, including `"FIXED: a back-edge source with NO other outgoing edge..."`) and a `describe("generateTransformFunction write-through codegen")` block (3 tests asserting the independent buffer/output writes directly).

`composite_validator.rs` was also checked and needs no change: it validates node/port/edge membership only (every node references a real published primitive, every edge connects a real port, no dangling unexposed Program-kind inputs, exposed-port names valid/unique, at least one output exposed). It never does graph traversal or checks acyclicity, so a cyclic composite graph is exactly as structurally valid by its rules as an acyclic one.

## Decision

Composite transforms genuinely support internal feedback cycles today, with no caveat, via the composite preview path's direct reuse of Editor's `GraphCompiler.process` (see the import in `composite-preview-controls.ts` cited above). The one cyclic shape that was silently broken (back-edge source with no forward consumer) is now fixed and covered by the 7 tests in `GraphCompiler.test.ts` described above.

This reuse — `composite-preview-controls.ts` importing `process`/`inputPortCountOf`/`inputPortIndexByName` straight from `GraphCompiler.ts` — is formally accepted as an intentional, pre-existing shared-utility exception to the Creator/Editor separation rule (`feedback_editor_creator_separation.md`). It is pure, stateless graph-compilation math, not Editor UI or Editor-owned mutable state, and it stays as-is: uncopied, unforked, not something to "fix" going forward. The separation rule continues to apply to new work; it is not being relitigated or applied retroactively to this existing, working import.

Explicitly out of scope: a published composite cannot today be placed as a node inside a real Editor graph (no wasm binary for `kind = "composite"` transforms, no flatten/inline logic in `GraphCompiler.ts`, `graph-worklet.js`, or the Editor canvas/palette to consume one). So "a composite's internal cycle interacting with a parent Editor graph's own cycle handling" is not a reachable runtime scenario today — noted here as context, not addressed by this decision.

## Consequences

- No change to `agents/architecture.md`'s Creator/Editor boundary section — no new shared surface is introduced by this decision; an existing one is being formally acknowledged and documented.
- `agents/ownership.md`'s "Shared zones" section gets a new bullet documenting the `composite-preview-controls.ts` → `GraphCompiler.ts` dependency, so a future change to `GraphCompiler.ts`'s cycle-handling behavior or `CompiledNode`/`CompiledGraph`/`GraphInput` shapes is checked against the composite-preview call site too, not just Editor graph compilation.
- No changes to `GraphCompiler.ts`, `graph-worklet.js`, `composite-preview-controls.ts`, or `composite_validator.rs` as part of this decision (the `GraphCompiler.ts`/`graph-worklet.js` fix referenced in Context already landed separately, under `editor-agent`).
