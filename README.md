# rs-dean

Rust/WASM browser-game scaffold for Dean.

The app is static-only: Leptos CSR for DOM UI, Bevy 0.19 WebGPU-only scenes,
`rs-dean-idb` for isomorphic durable state, and GitHub Pages-compatible output.

## First Run

```bash
rustup show
just bootstrap
just gate
just dev
```

## Quality Gate

`cargo xtask gate` is the single pass for this repo. It formats, lints, tests,
checks wasm builds, audits dependencies, regenerates the template proof app,
verifies Trunk artifacts, verifies Pages artifacts, and sweeps docs for stale
stack references. The gate runs the native refresh hydration regression and
compiles the browser refresh hydration regression for wasm. It also runs the
`apps/cube-smoke` browser render check, which screenshots a centered square
canvas and asserts that the lit cube rendered green pixels.

## Durable State

`crates/idb` exposes the shared async object-store API. Browser wasm builds use
IndexedDB, native bundles use the native backend, and callers use the same
`Database` methods in both targets. Leptos and Bevy state flows should depend
on `crates/state` or `rs-dean-idb`, not on backend-specific storage crates.

The refresh contract is covered by `crates/state/tests/refresh_hydration.rs`.
It persists an app snapshot, hydrates separate Leptos-style and Bevy-style
caches, drops those caches, reopens durable storage, and asserts both caches
resume from the same snapshot.

## Bevy Render Smoke

`just cube-smoke` builds `apps/cube-smoke`, serves the generated files, launches
headless Chrome, captures the centered square canvas page, and fails unless the
screenshot contains a centered green-dominant cube region.

## Skill Docs

- `.agents/skills/bevy-019/SKILL.md`
- `.agents/skills/rust-modern/SKILL.md`
- `.agents/skills/idb/SKILL.md`
- `.agents/skills/five-phase-pass/SKILL.md`

Read the relevant skill before changing that part of the stack.
