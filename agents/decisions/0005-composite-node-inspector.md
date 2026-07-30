# 0005: Composite canvas node inspector (bottom panel), ephemeral preview param editing, node enable/disable

- **Status:** Accepted
- **Date:** 2026-07-30

## Context

The composite canvas (`frontend/src/components/creator/composite-canvas.tsx`) lets a creator wire existing primitive transforms into a graph. Clicking a node originally opened `TransformDetailsModal` — a generic read-only popup (name/description/ports/params, no source) shared with the dashboard's transform list. The user rejected the modal in favor of a Monaco-tab-bar-style bottom panel, matching `code-editor.tsx`'s `impl.rs`/`output` tab pattern, and raised two further ideas in the same conversation: adjusting a node's params during preview, and temporarily disabling individual nodes while editing a composite.

Scoped by `product-owner`, with UI-pattern research delegated to `creator-agent` (composite-canvas.tsx, code-editor.tsx, transform-properties-panel.tsx) and two feasibility checks also delegated to `creator-agent`:
- Whether per-node param overrides exist anywhere in the composite data model today (`CompositeNode`/`CompositeGraphDefinition` in `backend/domain/src/db/transform_snapshot.rs` and its TS mirror `frontend/src/domain/Transform/CompositeGraphDefinition.ts`) — confirmed they don't; both sides carry only `node_id`/`transform_id`.
- Whether any per-node runtime bypass exists in `graph-worklet.js` — confirmed only a whole-graph `SET_BYPASS` exists, no per-node concept.

Mid-scoping, the user clarified that "adjust params" meant the same ephemeral, frontend-only preview tweak that already exists for primitive transforms (`CreatorPreviewStore.updateParam` → `CreatorTransformPreview.updateParams` → worklet `onUpdateParams`), not a persisted per-node override — which removed what would otherwise have been a real backend feature from scope.

## Decision

Ship all three pieces as frontend-only work, no backend/schema changes:

**1. Bottom panel replaces the modal.** Clicking a `CompositeTransformNode` sets new per-node selection state (net-new — no such state exists today) instead of calling `openModal({type:"transformDetails", ...})`. A bottom-docked panel appears with two tabs:
   - **Source** — read-only Monaco (`readOnly: true`, first use of this option in the codebase), showing the selected node's `source_code` (already loaded client-side via `useResolveTransformDefinitions`, no new fetch).
   - **Details** — ports/params, via a prop-driven extraction of `transform-properties-panel.tsx`'s existing `PortsList`/`ParamRow` (currently hardwired to `useCreatorStore.selectedTransformId`) so it can be driven by the selected node instead.
   Only this one call site changes. `TransformDetailsModal` itself and its other call site (`TransformItem.tsx`) are untouched.

**2. Ephemeral per-node param editing, in the Details tab, during an active preview.** `CreatorTransformPreview.updateParams` gains a `nodeIndex` argument (was hardcoded to `0`, correct only for the single-node primitive preview graph — primitive call sites now pass `0` explicitly). The selected node's `nodeIndex` in the *currently compiled* preview graph must be looked up dynamically by `node_id` at call time, not cached, because decision 3 changes which nodes are even present in that compiled graph. State lives only for the preview session's lifetime: never written into `editingGraph`, never part of Save/Publish payloads.

**3. On-node enable/disable, ephemeral, with reachability-based coloring built in from v1 (not deferred).** A small on-node control (not double-click, not a context menu) toggles a node's disabled state, held in new frontend-only state scoped to the current editing session (resets on reopen — no persistence, no new field on `CompositeNode`). Disabled nodes and their incident edges are filtered out of the graph before both Save/Publish and Preview/Play compile. A new client-side reachability check (does removing this node break connectivity from any exposed input to any exposed output — no such traversal exists in `composite_validator.rs` today, which only does flat set-membership checks) drives per-node coloring (safe-to-disable / load-bearing / currently-disabled), recomputed on every graph edit.

**Explicit non-goals:** no changes to `CompositeNode`/`CompositeGraphDefinition` (Rust or TS) or `composite_validator.rs`; no persisted per-node param overrides or disabled-node state; no per-node runtime bypass in `graph-worklet.js`; no new API endpoints; no server-side reachability check.

**Suggested build order:** Phase 1 (panel) → Phase 2 (param editing, needs the Details tab to exist) → Phase 3 (enable/disable). Phase 3's on-node UI is otherwise independent of 1/2 and could be parallelized, but it shares the compiled-preview-graph touchpoint with Phase 2 — Phase 2's dynamic index lookup must hold regardless of build order.

## Consequences

- No changes to `agents/architecture.md` — this doesn't change either surface's boundary, data ownership, or the Creator/Editor split; it's additive UI within the Creator's existing composite-canvas ownership.
- Hand-off target: `creator-agent` (owns the Creator surface end-to-end, including composite-canvas.tsx, code-editor.tsx, transform-properties-panel.tsx, creatorTransformPreview.ts — all files this touches).
- Files most relevant to implementation: `frontend/src/components/creator/composite-canvas.tsx`, `frontend/src/components/creator/creatorTransformPreview.ts`, `frontend/src/components/creator/transform-properties-panel.tsx`; `frontend/src/components/creator/code-editor.tsx` and `frontend/src/audio/worklet/graph-worklet.js` as pattern/behavior references only (no changes expected in the latter).
