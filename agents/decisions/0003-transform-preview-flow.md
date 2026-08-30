# 0003: Compiled-binary transport — preview flow vs. save flow are separate

- **Status:** Superseded by 0009
- **Date:** 2026-07-22

## Context

While refining `features/transform_drafts.md`, a state-machine question came up: the draft's source (`A`) and its compiled binary (`B`, derived from `A` via a compile ticket) both need to reach the frontend at different points — should they travel together, and should Save round-trip the binary bytes back up to the backend?

The original draft's lifecycle includes a step the first refinement pass (0002) missed entirely: "user compiles the source code and **tries it**" — i.e. the creator can run the just-compiled binary client-side before deciding to save. No such execution path exists in the creator surface today (`code-editor.tsx` only shows compile status text). This is distinct from Save, which already exists and works by reference (`{source_code, resource_id}` — see `agents/transforms.md` bucket 2), with the backend copying its own validated row from `transform_resources` into `transform_saved_state` rather than trusting client-supplied bytes.

The question was whether Save should instead send both `A` and the actual binary bytes together, now that the binary needs to reach the frontend anyway for preview.

## Decision

Treat "get the binary to the frontend to run it" and "persist the binary" as two independent data flows:

1. **Preview flow (new, not yet implemented):**
   - Backend: extend the compile ticket/resource response to carry the wasm bytes as base64 (`wasm_base64`), reusing the exact encoding `TransformBinaryDto` already uses for published binaries (`BASE64_STANDARD.encode`, `transforms_controller.rs`). Today `CompileTicketStatusDto`/`CompileResourceDto` (`ticket_controller.rs`) carry only `resource_id` — this is new, not already wired up, despite the pattern existing elsewhere.
   - Frontend: on compile success, decode the base64 (reuse `decodeBase64Binary` from `TransformService.ts`) and run it through **the same worklet pipeline the editor already uses** (`graph-worklet.js` / `WorkletController` / `WorkletMessageSender`), by synthesizing a degenerate one-node `CompiledGraph` (single `executionOrder` entry, no feedback, minimal buffer count) wrapping the just-compiled binary — rather than writing a separate hand-rolled preview runtime. This guarantees the preview executes under the identical ABI/calling-convention (`WebAssembly.instantiate` with zero imports, same `alloc`/`process` contract, real audio-thread execution) that the editor will use once the transform is published, so preview and production can't silently diverge in behavior.
   - These bytes are never sent back to the backend.
2. **Save flow (existing, unchanged):** stays reference-based — `{source_code, resource_id}`. The backend copies the already-introspected, already-validated `transform_resources` row into `transform_saved_state` server-side. The frontend having a copy of the bytes for preview purposes does not change this: the backend still cannot verify client-supplied binary bytes are what its own compile pipeline actually produced and introspected without either blindly trusting them (bypassing compile-time validation) or re-running the same fuel-limited wasmtime introspection on every Save (duplicating the compile step and widening where untrusted code executes). `resource_id` avoids both.

## Consequences

- `features/transform_drafts.md`: "Try it" added back as an explicit lifecycle step, flagged not-yet-implemented.
- `agents/transforms.md`: note added that the creator surface needs its own binary-fetch-for-preview path, separate from Save's resource_id mechanism and separate from the editor's published-binary fetch.
- No change to Save's contract or to the source/binary consistency-gate fix already recorded in `agents/decisions/0002-transform-draft-lifecycle-decisions.md`.
- Bucket 1 retention/eviction (tickets/resources are ephemeral, not meant to accumulate forever) is confirmed as a real future need but requires a dedicated background worker — deferred, not blocking, since `transform_saved_state` never depends on ticket/resource rows surviving past the moment of Save.

## Correction (implementation pass, 2026-07-22)

The Decision section above (bullet 1, second line) names `WorkletController` alongside `graph-worklet.js`/`WorkletMessageSender` as what the preview flow reuses. That's superseded by the more specific constraint `agents/ownership.md`'s Shared zones section landed on: `WorkletController` is explicitly **not** safe to reuse (it's a module-scope singleton that writes into editor-only global Zustand state on every connect/graph-ready/error event). The actual implementation (`frontend/src/components/creator/creatorTransformPreview.ts`) reuses only `graph-worklet.js` and `WorkletMessageSender`, via its own creator-scoped connect/disconnect wrapper. Leaving the original bullet as-written per the append-only convention for this log; this note is the correction.
