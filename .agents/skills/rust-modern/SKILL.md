---
name: rust-modern
description: >-
  Modern Rust 2024 rules for rs-dean. Use when writing, editing, reviewing, or
  scaffolding Rust code, especially browser-WASM, Bevy-adjacent game logic,
  deterministic simulation, Cargo workspace setup, clippy-clean APIs, error
  handling, tests, and code that should follow current stable Rust idioms.
---

# Rust Modern

Write Rust 2024 code that is clear, deterministic where game logic needs it,
and friendly to strict gates. When Bevy APIs are involved, read the `bevy-019`
skill first; that skill owns Bevy 0.19 mechanics. This skill owns Rust language
and code-judgment rules around those APIs.

## Baseline

- Use `edition = "2024"` for new crates.
- Set `resolver = "3"` explicitly in a virtual workspace root.
- Prefer stable Rust unless the repo has explicitly opted into nightly for a
  specific crate. Do not introduce `#![feature(...)]` casually.
- Keep logic crates native-testable when possible. Browser-only APIs belong
  behind wasm-specific adapter modules.
- For workspace commands, prefer deterministic scripts or `xtask` code over
  shell fragments copied into multiple places.

## Rust 2024 Rules

### Return-position `impl Trait`

Edition 2024 captures all in-scope generics and lifetimes by default.

Use `+ use<>` when a helper returns `impl Iterator`, `impl Fn`, or another
opaque type that does not borrow its inputs:

```rust
fn ids(&self) -> impl Iterator<Item = u32> + use<> {
    0..4
}
```

Add only the params that are truly captured, such as `use<'a, T>`.

### Temporary scopes

Do not assume a guard or borrow created in an `if let` scrutinee or a block tail
expression lives to the end of the block. Bind it first if it must stay alive.

### Let chains

Flatten nested optional lookups:

```rust
if let Some(player) = player
    && let Some(target) = player.target
    && target.alive
{
    attack(player, target);
}
```

This is usually clearer and keeps clippy's collapsible-if checks quiet.

### Match guards

Use `if let` guards for fallible secondary lookups in a match arm:

```rust
match action {
    Action::Use(id) if let Some(item) = inventory.get(id) => use_item(item),
    _ => {}
}
```

## Current Stable Idioms

- Use `get_disjoint_mut` for two or more mutable references into one slice,
  `Vec`, or `HashMap` by index/key. Do not use repeated indexing or unsafe
  split workarounds.
- Use `Vec::extract_if` when removing and collecting matching elements.
- Use `as_chunks::<N>()` for fixed-size slice groups such as RGBA pixels or quad
  vertices.
- Use `[value; _]` inside function bodies when the return type or const already
  fixes the array length.
- Use `LazyLock` for compute-once global tables. Avoid hand-rolled `OnceLock`
  plumbing unless initialization really needs custom error behavior.
- Use `core::hint::cold_path()` at the top of rare branches in hot loops.
- Use `core::range::Range` for ranges stored in `Copy` components/resources.
- Use `assert_matches!` in tests instead of `assert!(matches!(...))`.

## Deterministic Game Logic

- Run recorded or replayable simulation on fixed ticks, not variable frame
  deltas.
- Do not let deterministic state depend on wall-clock time, unseeded RNG, or
  `HashMap` iteration order.
- Pick an explicit overflow policy in deterministic math:
  - `strict_*` when overflow is a bug and should fail loudly.
  - `wrapping_*` when wraparound is intentional and replay-stable.
- Avoid bare `+`, `-`, and `*` for tick counters, grid indices, resource totals,
  replay streams, and deterministic accumulators.
- Store intent and inputs, derive results late. A replay should be a pure
  function of recorded input plus tick.
- Keep transition handlers idempotent: running them twice should either be
  impossible by construction or produce the same final state.

## Error Handling

- Propagate errors with `Result` and `?`; do not use `.unwrap()` in application
  or system code.
- Use `.expect("invariant: ...")` only for a real invariant and explain it in
  the message.
- Use `thiserror` for domain/library error enums.
- Use `anyhow` or miette-style reports only at binary, `xtask`, or CLI edges
  where the caller wants context rather than a stable error type.
- Do not silently drop a `Result`. If fire-and-forget is intentional, make that
  intent explicit and handle/log the failure path where it matters.

## API Shape

- Prefer plain data types with public fields when there is no invariant to
  protect.
- Keep fields private only when the type enforces a real invariant.
- Use finite enums for compile-time-known sets. Prefer exhaustive `match` arms
  over catch-all `_` when future variants should force a code review.
- Consume `self` for one-way transforms and builders that should not leave the
  source usable.
- Return views, iterators, slices, or references rather than allocating a `Vec`.
  If a function allocates, make that behavior obvious from the name or docs.
- Do not suffix types with `Component`, `Data`, or `Tag` unless two distinct
  concepts need disambiguation.
- Import common types at module scope. Avoid long inline paths in function
  bodies unless they clarify a rare conflict.

## Bevy-Adjacent Rust Judgment

- Use the `bevy-019` skill for all Bevy APIs before touching Bevy code.
- Mutate stored entity state directly through narrow queries when decoupling is
  not buying anything.
- Use messages/events/observers only when the communication boundary earns its
  keep.
- Shape query signatures as scheduling contracts: fetch only what is needed and
  ask for `&mut T` only when the system mutates `T`.
- Gate systems with run conditions instead of guard-and-return when a scheduler
  skip is possible.
- Model global state as resources and per-entity state as components. Avoid
  global mutable state.
- Prefer small cohesive systems by behavior, not tiny fragments that all mutate
  the same component and force ordering.
- Do not put async work inside normal ECS systems. Keep async at asset loading,
  task pool, browser API, or boundary adapters.

## WASM And Browser Code

- Put browser-only APIs behind `#[cfg(target_arch = "wasm32")]` adapters.
- Keep domain crates free of direct `web_sys` dependencies unless the crate's
  purpose is browser integration.
- Serialize boundary data with `serde`; validate persisted/browser inputs before
  they enter typed state.
- Treat browser storage and messages as untrusted boundaries, even though they
  are local-first.
- For wasm tests, use browser-level tests only where native tests cannot cover
  the behavior.

## Tests

- Unit-test pure domain logic natively with `cargo test`.
- Use `wasm-bindgen-test` only for code that genuinely needs wasm/browser APIs.
- Prefer property tests for math, scheduling, parser, and deterministic replay
  invariants when the state space is broad.
- In tests, assert the behavior and boundary contract rather than private helper
  details.
- Keep examples in rustdoc compiling when they are public API documentation.

## Avoid

- Do not use `generic_const_exprs` in shipping code.
- Do not use `specialization`.
- Do not add `static mut`.
- Do not rely on `HashMap` iteration order for gameplay outcomes.
- Do not introduce nightly features unless the repo policy and crate target
  explicitly require them.
- Do not hand-roll iterator state machines before considering current stable
  iterator tools. If nightly `gen` blocks are not explicitly enabled, do not use
  them.
