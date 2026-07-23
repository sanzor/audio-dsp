# 0004: Multi-input named ports — ABI shape, port-kind/cardinality schema, fail-closed backward compat

- **Status:** Accepted
- **Date:** 2026-07-23

## Context

`features/multi_input_ports.md` (product-owner-scoped, business-analyst-passed) identified that `graph-worklet.js`'s `generateTransformFunction` unconditionally sums every edge feeding a node into one buffer before `process()` ever runs — a transform declaring multiple input ports in `metadata()` could never actually receive them as separate signals. This blocked sidechain compressors, A/B crossfaders, and any transform needing distinct-not-summed inputs.

This decision covers the `backend/transform-sdk` slice only (creator-agent's side): the `Transform` trait/ABI shape, the `PortKind`/`PortCardinality` schema, `metadata_introspector.rs` validation, the `transform_ports` migration, and the creator-side republish warning. The frontend routing/UI slice (`graph-worklet.js`, `GraphCompiler.ts`, canvas) is separate, sequenced work for `editor-agent`, out of scope here.

Three points were resolved directly with the user during scoping, superseding the feature doc's own first draft:

1. **One trait, not two.** The original draft proposed keeping `Transform` (single-input, in-place mutation) unchanged and adding an opt-in `MultiInputTransform` (array input, separate output param), justified by "trait choice must be a compile-time decision." Talking it through found this wasn't a real constraint: the engine's pre-summing was forced by a signature with room for only one array, not an independent design choice. Giving the signature room for N arrays removes the need for a second trait entirely.
2. **Return value, not out-parameter.** A dedicated `output: &mut [f32]` parameter was considered and rejected: the wasm side already does a fresh arena `alloc()` per call for input/params, so a freshly-allocated return value is the same cost pattern applied symmetrically to output — not new complexity — and is more idiomatic besides.
3. **Republish warning: yes, build it now.** Confirmed by the user as in-scope for this pass, not deferred.

## Decision

1. **`Transform` trait becomes:**
   ```rust
   pub trait Transform: Default {
       fn process(&mut self, samples: &[&[f32]], params: &Params<'_>) -> Vec<f32>;
       fn metadata() -> TransformMetadata;
   }
   ```
   `samples` always has one entry per declared `Direction::Input` port, in declared order — a single-input transform (the entire pre-existing catalog) gets a slice of length 1, no special case. Return value is always exactly one output array, replacing in-place mutation. No second trait, no `export_multi_input_transform!` macro.

2. **`PortMetadata` gains `kind: PortKind` (`Program` | `Sidechain`) and `cardinality: PortCardinality` (`Single` | `Many`)**, both new closed enums, serialized lowercase. `Program` = main signal, fails closed if unwired (rejected upstream of `process()`, at the graph-pipeline validation layer — not this SDK's enforcement). `Sidechain` = control/detector, unwired always resolves to silence, never an error. `Single` = exactly one edge may target the port; `Many` is an explicit opt-in for the narrow legitimate summing case (was previously the implicit whole-node default). Output ports must be `Program`/`Single` — introspection rejects anything else.

3. **Wasm-level marshaling mirrors the existing `transform_metadata_ptr`/`transform_metadata_len` pattern:**
   ```rust
   pub extern "C" fn process(
       samples_ptr: *const f32, // num_inputs buffers of block_len floats, contiguous, in port order
       num_inputs: usize,
       block_len: usize,
       params_ptr: *const f32,
       params_len: usize,
   ) -> *const f32; // arena-allocated, always block_len floats, no separate length query
   ```
   A new `transform_abi_version() -> u32` export (constant `TRANSFORM_ABI_VERSION = 2`) is feature-detection: its absence on a module signals "legacy" — compiled against a pre-multi-input-ports SDK, still speaking the old in-place `process(ptr, len, params_ptr, params_len)` (no return value). Worklet-side dispatch on this is editor-agent's, out of scope here; this SDK only exports it correctly.

4. **`metadata_introspector.rs` gains:** `PortKind`/`PortCardinality` deserialize from fixed enums; output ports must be `Program`/`Single`; exactly one output port total (previously unchecked — now load-bearing since the ABI has one dedicated output pointer); port names unique within a direction (needed for `.port("name")`-style lookups); and an ABI/metadata consistency rule — a module with no `transform_abi_version` export must declare exactly one `Program` input port, since the old runtime can't route a second signal. In practice this last rule is close to unreachable for a *fresh* compile (the updated `export_transform!` always emits `transform_abi_version` now, since `transform-sdk` is path-pinned per compile job, not version-pinned — see the feature doc's Open coordination item 1), but it's a real, still-checkable rule, not dead code.

5. **`transform_ports` migration (`0016_transform_ports_kind_cardinality`):** adds `kind`/`cardinality`, both `NOT NULL DEFAULT` (`'program'`/`'single'`), same pattern as migrations 0014/0015 — every currently-published transform (all implicitly single-input, main-signal, single-edge before this feature existed) is correctly backfilled by the default alone. Defaults dropped after backfill, so every future `publish_compiled_transform` write must supply both explicitly. `get_transform_definition()`'s `jsonb_build_object` updated to include both. No `transform_params` schema change — named param access is pure SDK/macro ergonomics over existing columns.

6. **Named param access folded in, additive, no wire-format change:** `Params<'a>` wraps the raw `&[f32]` with the names declared in `metadata()`, implements `Index<usize, Output = f32>` (so `params[0]`-style call sites keep compiling unchanged) plus `.named("threshold")`.

7. **Backward-compat risk accepted, but fail closed.** A creator republishing a transform from 1→N inputs can break editor graphs referencing it — no version pin exists anywhere (`agents/decisions/0002`'s already-accepted "republish overwrites in place" precedent). Accepted, on the condition that the runtime fails closed (a visible error) for an unwired `Program` port rather than reviving the old silent-summing bug. The actual enforcement point is the graph-pipeline validation layer (editor-agent's, out of scope here) — this SDK only keeps the `PortKind` enum's doc comments accurate about that contract.

8. **Republish warning: built.** At Publish time, the creator surface calls a new advisory endpoint (`GET /transforms/{id}/publish/port-diff`, backed by `TransformsProvider::diff_publish_port_shape`) that diffs the about-to-be-published port shape (bucket 2, `transform_saved_state`) against what's currently live (bucket 3, `transform_ports`) — count/kind/cardinality per port, not name (a pure rename doesn't trip it). If `changed`, `code-editor.tsx`'s publish handler shows a non-blocking `window.confirm` before proceeding; if the check itself fails, publish proceeds anyway (advisory only, never gates the real publish). `changed` is always `false` for a transform's first-ever publish (nothing to compare against).

## Consequences

- `agents/transforms.md`: ABI contract section rewritten for the new `Transform` signature, `PortKind`/`PortCardinality`, the wasm marshaling shape, `transform_abi_version`, and the republish-diff endpoint.
- `agents/invariants.md`: data-integrity section gains the port-kind/cardinality schema rule and the fail-closed-vs-sidechain-silent rule, with an explicit note on where enforcement actually lives (graph-pipeline validation, not this SDK).
- `database/audio_db/migrations/0016_transform_ports_kind_cardinality.{up,down}.sql` — new columns + `get_transform_definition()` update.
- **Not built in this pass, by design — editor-agent's, sequenced after this lands** (per the feature doc's own Sequencing section, step 3):
  - `graph-worklet.js`'s ABI-version dispatch and per-port routing (replacing unconditional `addAll`).
  - `GraphCompiler.ts`/persistence fixes (`buildRuntimeGraph.ts`'s hardcoded `fromPortId: 0, toPortId: 0`, `NodePortInput[]` shape).
  - Canvas UI (`MultiPortTransformNode`, port-aware handles, connection validation).
  - `validateRuntimeGraph.ts`'s fail-closed compile-time check for unwired `Program` ports on multi-input nodes — this is the actual enforcement point decision 7 above depends on; until editor-agent builds it, a republished multi-input transform's fail-closed guarantee is not yet backed by working code, only by this SDK's contract and doc comments.
- **BA note carried forward for the record (not creator-agent's action item):** the republish warning in decision 8 only informs the *creator*; the downstream editor-side consumer whose graph breaks only finds out reactively via the fail-closed validation error once editor-agent builds it. BA's recommendation was to accept that asymmetry for this pass, contingent on editor-agent resolving the feature doc's Open coordination item 4 ("auto-validate graphs on load") as yes, plus a non-blocking backlog item for future "graphs affected by this republish" surfacing. Both are editor-agent's/product-owner's follow-up.
- Handed to `regression-review-agent` per `agents/ownership.md`'s escalation rule (crosses the creator-write/editor-read boundary on `transform_ports`).
