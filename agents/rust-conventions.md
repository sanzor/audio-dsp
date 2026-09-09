# Rust Code Conventions

Applies to all Rust code in `backend/`. These are style/structure rules a reviewer should flag on sight — separate from `agents/invariants.md`, which covers behavioral rules.

## Error handling

- No `.expect()` or `.unwrap()` outside test code (`#[cfg(test)]` modules or files under `tests/`). Production paths must handle the `Err`/`None` case explicitly.
- No `.ok_or(...)`/`.ok_or_else(...)`. To turn an `Option` into an early `Err` at a `?` boundary, use a plain `match` with an early return in the `None` arm:
  ```rust
  let x = match maybe_x {
      Some(x) => x,
      None => return Err(...),
  };
  ```
- Avoid `if let Some(x) = maybe_x { ... } else { return Err(...); }`-style control flow — use the `match` form above instead.
- Prefer `?` over manual `match`/`if let` unwrapping wherever the value is already a `Result` (or already converted to one via the `match` above). Use `.map_err(...)` to convert error types at the `?` boundary rather than writing a bespoke `match` arm just to wrap an error.
- Reach for combinators (`.and_then`, `.map`, `.map_err`, `.unwrap_or_else`) to express a chain monadically instead of nesting `match`/`if let`. Don't force it if it hurts readability, but default to it.

## Imports

- Import specific items (`use module::Thing;` / `use module::{Thing, Other};`). No glob imports (`use module::*;`), including in tests.

## File organization

- Tests live in their own file (e.g. `foo.rs` + `foo_tests.rs`, or a `tests/` submodule file), not inline `#[cfg(test)] mod tests { ... }` blocks inside the implementation file.
- One DTO per file.
- Database DTOs (the ones mapping directly to a table row, typically named `Db<Something>`) must declare a type alias for each primary/foreign key column instead of using a bare `i32`/`i64`/`Uuid`, e.g.:
  ```rust
  pub type TrackId = i64;

  pub struct DbTrack {
      pub id: TrackId,
      pub graph_id: GraphId,
      ...
  }
  ```
