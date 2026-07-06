# AGENTS.md

`rs-dean` is a Rust/WASM browser-game scaffold workspace. It is static-only,
targets GitHub Pages, stores durable state in IndexedDB, and ships every clone
with a Leptos marketing app plus a Bevy-only game app.

## Non-Negotiables

1. **Rust-first**: source, gates, generated templates, and local orchestration
   stay in Rust/Cargo unless a browser standard requires a Rust binding crate.
2. **Story-harness first**: reusable UI and scene work gets a matching route in
   `apps/stories` before being wired into `apps/marketing` or `apps/game`.
3. **Serde + validation first**: persisted records, browser messages, and
   generated data cross typed Rust boundaries and validate at the edge.
4. **IDB-first state**: `rs-dean-idb` is the only durable storage boundary.
   It uses IndexedDB in wasm browser bundles and a native embedded store in
   native bundles. Leptos signals and Bevy resources are hydrated caches, not
   alternate sources of truth.
5. **Persistent updates first**: every app update that represents user progress,
   settings, unlocks, completion, or other resumable state must go through
   `crates/state` and persist through `rs-dean-idb`. In-memory Leptos signals
   and Bevy resources are only caches over durable state.
6. **One-pass gate first**: `cargo xtask gate` is the only quality gate. It must
   be green before any task is considered complete. `just check` is an alias.
7. **Shared-theme first**: design tokens and theme palettes belong in
   `crates/ui`. Leptos consumes them through Tailwind theme variables; Bevy
   consumes the same Rust palette through the `bevy` feature without depending
   on Leptos.

## Stack

- Rust 2024 with Cargo workspace resolver 3.
- `apps/marketing` is the required Leptos marketing app. It may use Bevy for
  browser canvas moments, but it owns the marketing DOM surface.
- Leptos app and template styles use Tailwind through Trunk's
  `rel="tailwind-css"` asset type. Keep the standalone `tailwindcss` CLI on
  `PATH`; `just bootstrap` installs it.
- `crates/ui` owns shared design tokens, semantic colors, theme cycling, and
  all theme palettes. The Tailwind token stylesheet lives at
  `crates/ui/styles/theme.css`; Bevy callers use `rs-dean-ui` with
  `default-features = false` and `features = ["bevy"]`.
- `apps/game` is the required Bevy WebGPU game binary. It must not depend on
  Leptos.
- `apps/stories` is the required independent story harness for reusable UI and
  scene proofs.
- Bevy `0.19.0` for browser scenes, WebGPU-only. The gate fails if a WebGL
  feature appears in the Bevy wasm feature tree.
- `templates/app/cube-smoke` is copied into generated `apps/test-project` as
  the browser render smoke surface: one centered square canvas, one lit green
  cube, WebGPU startup verification, and a green material scene assertion.
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
3. required app persistent-state wiring check
4. Leptos Tailwind asset wiring check for apps and generated templates
5. native `cargo clippy` for workspace crates except browser-only Bevy crates
6. wasm `cargo clippy` for app, story harness, Bevy scene, storage, and state
   crates
7. native `cargo nextest` for workspace crates except browser-only Bevy crates
8. native `cargo test --doc` for workspace crates except browser-only Bevy
   crates
9. wasm `cargo check` for browser crates
10. wasm compile of the browser refresh hydration regression
11. strict rustdoc build
12. `cargo deny check`
13. `cargo machete`
14. regenerate `apps/test-project` from `templates/app`
15. assert the generated template keeps the shared schema/state contract
16. build and verify generated template output
17. build and verify `apps/marketing` static output, including Pages artifacts
18. build and verify `apps/game` static output
19. build and verify `apps/stories` static output
20. build generated `apps/test-project/cube-smoke`, verify the centered canvas,
    WebGPU renderer, and green cube scene contract
21. docs and skill sweep for stale non-Rust stack references

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
| `just dev` | Run the marketing app with Trunk on LAN-friendly host/port. |
| `just game` | Run the Bevy game app with Trunk on LAN-friendly host/port. |
| `just stories` | Run the Rust story harness. |
| `just cube-smoke` | Generate the test project, build its green-cube page, and verify the WebGPU scene contract. |
| `just doctor` | Run the fast local environment preflight. |
| `just build` | Build static marketing/game output and Pages artifacts. |
| `just pages` | Build the aggregate GitHub Pages artifact under `target/pages`. |
| `just gate` | Run the one-pass Rust gate. |
| `just check` | Alias for the one-pass Rust gate. |
| `just five-phase-pass` | Run the Rust five-phase pass. |
| `just docs-sweep` | Run the stale-reference sweep only. |
| `just bootstrap` | Install local gate tools. |
