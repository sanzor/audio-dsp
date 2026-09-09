# 0010: Remove ticket-based compile; compile runs synchronously on Save

- **Status:** Accepted (supersedes 0009)
- **Date:** 2026-09-09

## Context

Bucket 1 (Compile) was an async ticket pipeline: `POST /transforms/tickets`
created a `DbTicket`, a background `Worker` consumed a `TicketCreatedEvent`
off an in-process mpsc channel and ran `Processor::process` (compile → wasm
introspection → validate → store as a `transform_resource`), and the Creator
frontend polled ticket status before it could preview or Save. 0009 then
patched Save to accept an optional `wasm_base64` package the frontend held
in memory from that polled ticket result, specifically because Save
couldn't read ticket/resource state directly.

The ticket endpoints (`tickets` module, `ticket_controller`) were already
deleted in a prior commit, which left `main.rs`, `openapi.rs`, and the
now-orphaned `Worker`/`Processor` referencing types that no longer existed
— the async pipeline was fully dead code (nothing produced
`TicketCreatedEvent`, and even when it ran, `Worker::run_loop` discarded its
`ProcessResult`). The two-step "compile via ticket, then hand the binary
back on Save" flow this served is no longer wanted at all.

## Decision

Compile is no longer a separate bucket or an async step. `PUT
/draft_transforms/{id}/save-primitive` now takes only `{source_code}` and
runs `build_job::compile_transform_source` → `wasm_parser::parse_wasm` →
`validator::validate_primitive` synchronously, inline, on the request —
the same compile+introspect logic the ticket `Processor` used to run, just
called directly instead of through a queue.

**A failed compile rejects the whole save.** `source_code` is not persisted
and no prior binary/metadata is touched. This replaces bucket 2's old
"source-only save, compile is optional and never blocks" contract — there
is no longer a way to persist source that doesn't compile.

Removed entirely: the `ticket_worker` consumer/producer/worker/events
machinery, `TransformDraftsDataProvider::save_primitive_draft`'s `Option`
around a compiled artifact (now required, since a save that reaches
persistence always has one), the `wasm_base64` field on
`SavePrimitiveParams`, and the now-fully-unused `domain::db::ticket` types
(`DbTicket`, `DbResource`, `TicketStatus`, `UpdateTicketParams`,
`CreateTicketParams`/`CreateTransformDraftParams`) — nothing in the
workspace referenced them once the pipeline was gone.

`check_draft_source_code` (`POST
/draft_transforms/{id}/validate-source-code`, a fast `cargo check` with no
wasm artifact) is unaffected — it was already synchronous and independent
of the ticket pipeline.

**Not done in this change:** the `transform_ticket`/`transform_resource`
database tables and their migrations were left as-is (schema changes are a
separate, more consequential call). The Creator frontend's matching flow
(ticket polling, `TicketService.ts`, the `attachableCompiledDraft`/package
handoff in `code-editor.tsx` and `unsaved-creator-changes-modal.tsx`) still
expects the old two-step contract and needs a corresponding update — not
done here.

## Consequences

- `agents/transforms.md`'s "Three buckets" section is rewritten: Bucket 1
  as a separate async stage no longer exists; Save now owns compile.
- `agents/invariants.md`'s data-integrity section no longer needs the
  "source-only save never wipes a good build" rule (a save either compiles
  and saves everything, or fails and saves nothing) — and its references
  to `metadata_introspector.rs` are corrected to the actual current module
  split (`ticket_worker/processor/wasm/wasm_parser.rs` +
  `ticket_worker/processor/validator/validator.rs`), which predates this
  decision but was never fixed.
- Frontend Creator work (drop ticket polling, send `{source_code}` only,
  surface a rejected-save compile error inline) is tracked as follow-up,
  not done here — this decision only covers the backend.
