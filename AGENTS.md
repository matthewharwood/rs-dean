# AGENTS.md

`rs-dean` is a Rust/WASM browser-game workspace. It is static-only, targets
GitHub Pages, stores durable state in IndexedDB, and renders browser scenes
with Leptos plus Bevy WebGPU.

## Non-Negotiables

1. **Rust-first**: source, gates, generated templates, and local orchestration
   stay in Rust/Cargo unless a browser standard requires a Rust binding crate.
2. **Story-harness first**: reusable UI and scene work gets a matching route in
   `apps/stories` before being wired into `apps/web`.
3. **Serde + validation first**: persisted records, browser messages, and
   generated data cross typed Rust boundaries and validate at the edge.
4. **IDB-first state**: `rs-dean-idb` is the only durable storage boundary.
   It uses IndexedDB in wasm browser bundles and a native embedded store in
   native bundles. Leptos signals and Bevy resources are hydrated caches, not
   alternate sources of truth.
5. **One-pass gate first**: `cargo xtask gate` is the only quality gate. It must
   be green before any task is considered complete. `just check` is an alias.

## Stack

- Rust 2024 with Cargo workspace resolver 3.
- Leptos CSR mounted by Trunk.
- Bevy `0.19.0` for browser scenes, WebGPU-only. The gate fails if a WebGL
  feature appears in the Bevy wasm feature tree.
- `apps/cube-smoke` is the browser render smoke surface: one centered square
  canvas, one lit green cube, WebGPU startup verification, and a green material
  scene assertion.
- `crates/idb` owns the shared async storage API. It exposes one object-store
  interface for wasm IndexedDB and native bundles, so Leptos UI code, Bevy
  systems, and state crates do not import storage backends directly.
- Durable app state belongs in `crates/state` and persists through
  `rs-dean-idb`.
- `garde` for validation and `serde` for boundary serialization.
- `cargo xtask` owns commands, gates, template generation, artifact checks, and
  docs drift checks.

Before writing Bevy code, read `.agents/skills/bevy-019/SKILL.md`. Before
changing Rust patterns, read `.agents/skills/rust-modern/SKILL.md`. Before
changing persistence, read `.agents/skills/idb/SKILL.md`. For any cross-cutting
change, use `.agents/skills/five-phase-pass/SKILL.md`.

## One-Pass Gate

Run:

```bash
cargo xtask gate
```

The pass runs, in order:

1. `cargo fmt --all -- --check`
2. Bevy WebGPU-only feature-tree check
3. native `cargo clippy` for workspace crates except browser-only Bevy crates
4. wasm `cargo clippy` for app, story harness, Bevy scene, storage, and state
   crates
5. native `cargo nextest` for workspace crates except browser-only Bevy crates
6. native `cargo test --doc` for workspace crates except browser-only Bevy
   crates
7. wasm `cargo check` for browser crates
8. wasm compile of the browser refresh hydration regression
9. strict rustdoc build
10. `cargo deny check`
11. `cargo machete`
12. regenerate `apps/test-project` from `templates/app`
13. assert the generated template keeps the shared schema/state contract
14. build and verify generated template output
15. build and verify `apps/web` static output, including Pages artifacts
16. build and verify `apps/stories` static output
17. build `apps/cube-smoke`, verify the centered canvas, WebGPU renderer, and
    green cube scene contract
18. docs and skill sweep for stale non-Rust stack references

Warnings fail. Missing expected artifacts fail. Missing gate tools fail with a
clear install message. Do not bypass a failing step; fix the source of the
failure and rerun the whole pass.

## Five-Phase Pass

For any stack, template, gate, or shared-pattern change, run:

```bash
cargo xtask five-phase-pass
```

The Rust five-phase pass means:

1. **P1 app/crate**: implement the source change in the owning app or crate.
2. **P2 template**: mirror scaffold-impacting changes into `templates/app`.
3. **P3 generated proof**: regenerate ignored `apps/test-project` and run the
   one-pass gate.
4. **P4 docs/skill**: update `AGENTS.md`, `README.md`, and the owning skill.
5. **P5 sweep**: run the stale-reference sweep across docs and local skills.

`apps/test-project` is generated proof only. It stays ignored and must not be
committed.

## Commands

| Command | Purpose |
|---|---|
| `just dev` | Run the web app with Trunk on LAN-friendly host/port. |
| `just stories` | Run the Rust story harness. |
| `just cube-smoke` | Build the green-cube page and verify the WebGPU scene contract. |
| `just doctor` | Run the fast local environment preflight. |
| `just build` | Build static app output and Pages artifacts. |
| `just gate` | Run the one-pass Rust gate. |
| `just check` | Alias for the one-pass Rust gate. |
| `just five-phase-pass` | Run the Rust five-phase pass. |
| `just docs-sweep` | Run the stale-reference sweep only. |
| `just bootstrap` | Install local gate tools. |
