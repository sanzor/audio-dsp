# 0009: Temporary frontend compile package

- **Status:** Accepted
- **Date:** 2026-08-30

## Context

The Creator needs a successful compile result to be previewed before Save,
while ticket resources can have a lifecycle independent from a draft. The
previous resource-reference Save design required Save to read ticket-resource
state, which is intentionally deferred for the current implementation.

## Decision

For now, the successful compile-resource endpoint returns the exact ticket
source and compiled WASM as one package. The Creator retains that package only
in memory and returns it to primitive Save only while its source equals the
current editor buffer. Save does not load a ticket resource; it validates the
submitted WASM through the existing fuel-limited, zero-import metadata
introspection and atomically stores the source, WASM, and derived metadata in
the draft.

There is no receipt, durable build record, package persistence, or versioning
in this iteration. A page refresh before Save may require recompilation or
resource retrieval. This deliberately means draft owners can submit arbitrary
valid WASM paired with source; it does not prove compiler provenance.

## Consequences

Compile remains independent from Save: a worker only writes ticket/resource
state and never mutates a draft. Save remains independent from Compile and
Publish, but now accepts the optional frontend-held binary package. The
living compile/save contract is updated in `agents/transforms.md` and the
integrity limitation is recorded in `agents/invariants.md`.
