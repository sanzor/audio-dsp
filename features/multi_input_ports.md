Multi-Input Named Ports

## Problem

A graph node's `process()` receives exactly one input buffer today, no matter how many input ports the transform declares in `metadata()`. `frontend/src/audio/worklet/graph-worklet.js`'s `generateTransformFunction` collects every edge feeding a node and sums them into one buffer via `addAll()` before `process()` ever runs — unconditionally, per node, regardless of declared port count. Two signals summed upstream cannot be recovered inside `process()`; this is data loss, not an ergonomics gap (contrast with the separate, minor issue of params being unnamed-positional at the ABI boundary — see "Folded-in: named param access" below).

This blocks an entire class of transforms:
- **Sidechain compressor** — needs a *program* signal (gets processed) kept separate from a *key/detector* signal (triggers compression, never audible). Today the detector signal becomes part of the output.
- **A/B crossfader** — needs "In A"/"In B" kept distinct so a `mix` param can interpolate (`out = A*(1-t) + B*t`). Summed inputs make this mathematically unrecoverable — you get `A+B` regardless of `mix`.
- **Ratio/balance mixer** — N distinct inputs with independent gain/pan, kept apart inside one transform's `process()`.

This is cross-surface: the ABI lives in `backend/transform-sdk` (creator-agent), the routing/UI lives in `frontend/src/audio/worklet/graph-worklet.js` and the canvas (editor-agent). Per `agents/ownership.md`'s escalation rule, `product-owner` scoped this before either surface began designing.

## Settled decisions

These were decided by the user off product-owner's scoping brief, before creator-agent/editor-agent designed against them:

1. **Port kinds now, not deferred.** The port model distinguishes `Program` (main signal, symmetric/equal-priority) from `Sidechain` (control/detector, may be left unwired) from day one — not a bare port count. Reason: sidechain is a flagship use case and needs "may be left unwired" semantics a count-only model can't express; adding it later would be a second breaking change to `PortMetadata`.
2. **Backward-compat risk accepted, but fail closed.** A creator republishing a transform from 1→N inputs can silently break existing editor graphs referencing it — same risk class as the already-accepted "republish overwrites in place, no version pin" precedent (`agents/decisions/0002`). Accepted, but the runtime must **fail closed** (a clear, visible error) rather than silently reviving today's summing bug for graphs whose edges don't specify a port.
3. **Named param access folded into the same ABI pass.** Separately, `process()` params are unnamed-positional (`params[0]`, `params[1]`) even though `metadata()` declares names. Since the same `Transform` trait and `export_transform!` macro are being touched anyway, named/keyed param access ships in this pass rather than as a later follow-up.
4. **Priority: next up.** Active work, not queued behind other roadmap items.

## Contract shape (`backend/transform-sdk`)

**Revised directly with the user after the initial creator-agent/editor-agent pass — superseding the two-trait split below.** The original draft split `Transform` (single-input, in-place mutation, unchanged) from a new opt-in `MultiInputTransform` (array of inputs, separate output param), justified by "trait choice must be a compile-time decision." Talking it through surfaced two simplifications:

1. **The engine's pre-summing was never an independent design choice — it was forced by a signature that only had room for one array.** Give the signature room for N arrays and the engine no longer needs to decide anything on the transform's behalf; it just hands over what's wired. That argues for *one* signature that's always array-shaped, not two traits gated on input count.
2. **A dedicated `output: &mut [f32]` out-parameter is unnecessary complexity.** The wasm side already does a fresh arena `alloc()` on every call for input/params today — the arena is deliberately "bump-and-wrap, allocate every call, never individually freed" per its own doc comment — so returning a freshly-allocated result is the same cost pattern applied symmetrically to output, not a new one. A return value is also more idiomatic and keeps the signature at a clean two arguments.

```rust
pub trait Transform: Default {
    fn process(&mut self, samples: &[&[f32]], params: &Params<'_>) -> Vec<f32>;
    fn metadata() -> TransformMetadata;
}
```

One trait, one signature, for every transform, single- or multi-input alike:

- `samples` — one entry per declared `Direction::Input` port, in declared order. A single-input transform (the whole existing catalog — EQ, gain, saturation) gets a slice of length 1 and reads `samples[0]`; nothing else about writing one changes.
- `params` — unchanged from the original draft (see "Named param access" below).
- Return value — the node's output for this quantum, replacing today's in-place mutation. Always exactly one output array, matching the "exactly one output port" rule enforced by introspection (see Validation below).

No second trait, no `MultiInputTransform`, no `export_multi_input_transform!` macro, no runtime/compile-time trait-choice question. The only thing that changes is what the *engine* hands over: it stops merging edges across ports before the call, and instead hands over one already-resolved array per port (see "Runtime routing" below — unaffected by this revision, the per-port resolution was already designed to happen ahead of the call).

### Buffer vs. samples — not two concepts

Raised directly by the user and worth stating as a settled point: a "buffer" (`graph-worklet.js`'s routing pool, `buffers[node.outputBufferIndex]`) and "samples" (the array a transform reads or returns) are the same object, not two related-but-distinct things. A buffer is nothing more than an array of samples plus an index so a downstream node can look it up without copying. For a mid-graph node, its output buffer *is* its output samples array — there's no wrapper type or intermediate representation to design here. This was already implicitly true for the single-input case and doesn't change under multi-input: each declared port just gets its own reference into that same shared buffer pool, instead of the pool entries getting pre-summed across ports before the call.

### Wasm-level marshaling

Mirrors the pattern `transform_metadata_ptr()`/`transform_metadata_len()` already use for returning a result from wasm to host, rather than introducing a new one:

```rust
#[no_mangle]
pub extern "C" fn process(
    samples_ptr: *const f32, // contiguous: num_inputs buffers of block_len
                              // floats each, back-to-back, in declared port order
    num_inputs: usize,
    block_len: usize,
    params_ptr: *const f32,
    params_len: usize,
) -> *const f32; // arena-allocated result, always block_len floats — no
                  // separate length query needed, block_len is already known
```

Single-input transforms compile to `num_inputs = 1` — same wire shape, no special case. The only remaining version-detection need is for **already-published binaries**, compiled against the old SDK's in-place, single-buffer `process(ptr, len, params_ptr, params_len)` export — those still need to be told apart from new-shape modules so the worklet knows which calling convention to use (still via a `transform_abi_version()` feature-detection export, absence meaning "legacy"). That's a narrower, one-time compatibility bridge, not a choice between two ongoing designs — **still editor-agent's to wire up in `graph-worklet.js`, still not addressed in the editor-side pass — see Open coordination below.**

### `PortMetadata` schema

```rust
pub struct PortMetadata {
    pub name: String,
    pub direction: Direction,
    pub order: i32,
    pub description: Option<String>,
    pub kind: PortKind,               // new
    pub cardinality: PortCardinality, // new
}

pub enum PortKind {
    Program,   // main signal(s); fails closed if unwired (see below)
    Sidechain, // control/detector; unwired always yields silence, never an error
}

pub enum PortCardinality {
    Single, // exactly one edge may target this port
    Many,   // N edges sum (addAll-style) into this one port — the narrow,
            // legitimate case (e.g. multiple mic feeds into one "mix" port),
            // now an explicit opt-in instead of today's unconditional
            // cross-port default
}
```

Output ports carry the same fields but only `Program`/`Single` is valid on one — introspection rejects anything else rather than silently ignoring it.

### Unconnected input port

- `kind: Sidechain`, unconnected → **silence buffer**, unconditional.
- `kind: Program`, unconnected → **fail closed** (rejected upstream of `process()` — see Validation). Rejected the alternative of "pass through the sole connected input": it only has an unambiguous meaning for the 2-port degenerate case and doesn't generalize to N inputs, and it would let a crossfader's `mix` param blend real audio against silence with no indication anything's wrong — the exact "plays wrong audio, no error" failure decision 2 exists to avoid.

### Named param access (folded-in item, decision 3)

Additive, no wire-format change — `graph-worklet.js`'s `callWasm` still hands a flat positional `Float32Array`. `Params<'a>` wraps the raw slice with the names already declared in `metadata()` and implements `Index<usize, Output = f32>`, so existing `params[0]`-style source **keeps compiling unchanged** even though the trait signature moves from `&[f32]` to `&Params<'_>`. `.named("threshold")` is pure upside on top.

### Validation (`metadata_introspector.rs`)

New checks: `PortKind`/`PortCardinality` deserialize from a fixed enum (free via serde); output ports must be `Program`/`Single`; exactly one output port (was previously unchecked — now load-bearing since the ABI has one dedicated output pointer); port names unique within a direction (needed now that `.port("name")` lookup exists); and an ABI/metadata consistency check — a module exporting only legacy `process` (no `transform_abi_version`) must declare exactly one `Program` input port, since the old runtime has no way to route a second signal.

### Migration

`transform_ports` gets two new columns, `kind` and `cardinality`, both `NOT NULL DEFAULT` (`'program'`/`'single'`) — every currently-published transform is correctly backfilled by the default alone, same pattern as migrations 0014/0015. `get_transform_definition()`'s `jsonb_build_object` needs both fields added or they silently vanish from the read path. `transform_params` needs **no schema change** — named access is pure SDK/macro ergonomics over columns that already exist. `transform_resources`/`transform_saved_state` are JSONB, so new `PortMetadata` fields flow through automatically.

## Runtime routing and UI (frontend) — editor-agent

### Correction to initial scoping

Product-owner's brief assumed `backend/domain/src/graphs/edge.rs`'s `Edge.from_port_id`/`to_port_id` were dormant-but-live plumbing. Editor-agent traced the actual save/load path and found `graph_state` is an **opaque JSON blob column** (`DbGraph`) — `graphs::edge::Edge`/`graphs::node::Node` aren't on the live persistence code path at all. **This is good news for scope: no Rust/DB changes are needed to carry port ids through persistence.** Similarly, `ActiveGraphState`'s `connectNodes`/`persistIds` etc. are unreachable dead code — only its *types* are reused as the compiler pipeline's internal representation.

The one real hardcoded choke point: `buildRuntimeGraph.ts:56` unconditionally sets `fromPortId: 0, toPortId: 0` on every edge, discarding any port info even where it exists upstream. `GraphService.ts`'s wire mapping (`ApiGraphEdge`/`mapGraphEdge`) already reads/writes `fromPortId`/`toPortId` and needs no changes.

### `CompiledNode` shape

```ts
interface NodePortInput {
  portId: number;
  portOrder: number;         // wasm arg position
  kind: 'sidechain' | 'program';
  sources: NodeInputSource[]; // 0..N edges landing on this port
}

interface CompiledNode {
  nodeId: number;
  transformId: number;
  params: number[];
  inputs: NodePortInput[];    // one entry per declared input port, in port_order
  outputBufferIndex: number;
}
```

### `graph-worklet.js` routing

Replace the single node-wide `addAll → callWasm` with per-port routing: for each declared port, gather its `sources`; 0 sources → `SIL()` (silence); 1 source → pass through; 2+ sources → `addAll()` **scoped to that port** (this is where `Many`-cardinality summing legitimately still happens — correctly scoped now, instead of blindly across the whole node). `callWasm` moves from `(instance, params, input: Float32Array)` to `(instance, params, inputs: Float32Array[])`. Per the revised ABI (see "Wasm-level marshaling" above), `callWasm` writes the resolved per-port arrays contiguously into one arena region and calls the single fixed-arity `process(samples_ptr, num_inputs, block_len, params_ptr, params_len) -> result_ptr`, then reads `block_len` floats back from `result_ptr` — no separate out-pointer to pre-allocate, and no arity-specific call variants needed since `num_inputs` is just a parameter, not part of the call shape.

### Unwired-port runtime behavior

Both `Sidechain` and unwired `Program` resolve to silence *at the routing layer* — but for `Program`, this is defense-in-depth only. The actual gate is compile-time validation (below): a graph with an unwired `Program` port on a multi-input node should never reach the worklet in the first place. This reconciles with creator-agent's ABI-level "fail closed" stance — enforcement lives in the graph pipeline, not the ABI or the worklet's per-call routing.

### Fail-closed validation

Lives in `validateRuntimeGraph.ts`, before `compileGraph()`/`SET_GRAPH` — not worklet load time. Reuses the existing `CompileRuntimeGraphErrorReason`/`PipelineErrorResult` mechanism already wired into `SaveCompileStatusOverlay`. New rule: any node whose `TransformDefinition` declares >1 input port requires every incoming edge to carry a resolvable `toPortId`; nodes with exactly one declared input port stay valid with no `toPortId` (preserves every existing single-input saved graph unchanged). Chosen over worklet-side validation because it runs synchronously on the main thread before `SET_GRAPH` is ever posted (matches `agents/invariants.md`'s real-time rules), and reuses plumbing that already exists rather than duplicating port knowledge across the postMessage boundary.

Error surface: name the specific transform/port in the message, and flag the specific offending node(s) on canvas (not just a toast) — `PipelineErrorResult` already threads transform ids through; extend it to carry node ids too.

### Canvas UI

New registered node type (`MultiPortTransformNode`) replacing the generic-`"default"` fallback for transform nodes, modeled on the creator's existing read-only `TransformPreviewNode` (`canvas.tsx`) but made interactive/connectable. Reuses its handle-id convention (`in-${port_id}`/`out-${port_id}`, worth extracting into a shared helper both canvases import).

Visual rules: single-input/single-output nodes keep today's compact box (no change for the common case); a node with >1 input port grows into the taller, row-per-port layout — so a 1→N republish is visibly legible on canvas without opening a details modal. `Sidechain` ports render as a hollow/dashed handle (steady-state, expected to sometimes be empty); `Program` ports render solid, and an *unwired* `Program` port gets a warning affordance (amber outline) since it's more likely a mistake mid-edit.

Connection validation via React Flow's `isValidConnection`: block a second edge onto an already-full `Single`-cardinality port before drop (blocked-cursor affordance while dragging). The "edge with no resolvable target port" case is barely reachable via human dragging once every port has an explicit handle — its real backstop is programmatic/AI-assisted graph edits, which is exactly why the compile-time check above is load-bearing and not just a UI nicety.

### Persistence fixes (frontend-only)

1. `buildRuntimeGraph.ts:56` — read actual `sourceHandle`/`targetHandle` instead of hardcoding `0`.
2. `GraphCompiler.ts` — add `fromPortId`/`toPortId` to `GraphInputEdge`, thread through `compileGraph()`'s edge mapping, and group `buildNodes()`'s edge sources by target port instead of flattening — this is what actually produces `NodePortInput[]`.
3. `GraphController.ts`'s `handleSaveGraph` — read `e.sourceHandle`/`e.targetHandle` into the persisted edge repr.

No backend/DB changes needed for persistence (per the correction above).

## Open coordination (not yet reconciled — needs a short second pass before implementation)

1. **ABI version dispatch in `graph-worklet.js` is undesigned.** Narrower now than originally scoped — since the contract simplified to one signature (see "Contract shape" above), the worklet only needs to tell apart two shapes, not choose between two ongoing designs: legacy binaries (old 4-arg in-place `process(ptr, len, params_ptr, params_len)`, no `transform_abi_version` export) vs. new binaries (`process(samples_ptr, num_inputs, block_len, params_ptr, params_len) -> result_ptr`, always, including single-input transforms). Since `transform-sdk` is path-pinned per compile job (not version-pinned), the worklet will load a mix of both indefinitely once this ships. Still needs: where the worklet checks for the export, and how `callWasm` branches per node. Still editor-agent's to design.
2. ~~Two-trait ABI design~~ — **resolved.** Collapsed to one signature during direct discussion with the user (see "Contract shape" above); no second trait, no dispatch-by-trait question remains.
3. **Creator-side republish warning** — creator-agent's suggestion (non-blocking confirm dialog at Publish time if input port shape changes on an already-published transform) is new scope beyond decisions 1–4. Needs a yes/no.
4. **Auto-validate graphs on load** — should opening a saved graph run `validateGraph()` immediately (cheap, no binary fetch) so a broken-by-republish graph surfaces on open rather than waiting for the next Save/Compile/Activate? Editor-agent's call to make, flagged as a scope decision.
5. **Edge-replace-on-drop** for a full `Single`-cardinality port (drag a new edge onto an occupied handle → replace, vs. block and require manual delete-first) — a UX convenience call, not a correctness requirement.
6. **`export.rs`'s arena wrap-safety comment** ("at most two pointers live at once") needs updating for N inputs + output + params live per quantum. Still safe at realistic port counts against the 1 MiB arena; just a doc-correctness gap per `agents/transforms.md`'s "ABI changes must stay reflected across SDK/introspector/worklet by convention" rule.

## Deferred / out of scope

- **Multi-output** (e.g. a stereo splitter) — this design further commits to single-output (dedicated `output_ptr`); a symmetric future gap, not solved here.
- **A third `PortKind`/`PortCardinality` variant** (e.g. an "aux send" kind floated in earlier scoping conversation but unconfirmed) — the enums ship closed at exactly what decisions 1–2 asked for. Worth asking the user if a near-term third case is already anticipated, since adding one later is another CHECK-constraint migration of the same shape.
- **Consolidating the dead `ActiveGraphState` store** onto what `canvas-panel.tsx` actually uses — real dead code, but out of scope for this feature; noted so a future reviewer isn't confused about why the "obviously right" store isn't wired up.

## Sequencing

1. Resolve "Open coordination" items above (needs the user + a short creator-agent/editor-agent joint pass, primarily on ABI version dispatch and the two-trait confirmation).
2. `transform-sdk` ABI + `PortMetadata` schema + `metadata_introspector.rs` validation (creator-agent) — this is a strict dependency for everything else, since the editor can't build port-aware UI or worklet routing until the metadata shape is final.
3. In parallel once (2) lands: `GraphCompiler.ts`/persistence fixes and canvas UI (editor-agent) — largely independent of each other internally, both depend on (2).
4. Cross-check by `regression-review-agent` before merge, since this crosses the creator-write/editor-read boundary on `transform_ports` that `agents/ownership.md`'s escalation rule calls out explicitly.
5. Once implemented, this becomes an `agents/decisions/000N-*.md` entry (ABI shape decision, port-kind/cardinality schema decision, and the fail-closed backward-compat stance are all consequential enough per `agents/decisions/README.md`'s bar), and `agents/transforms.md`/`agents/invariants.md` get updated in the same change per the ownership escalation rule.
