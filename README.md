# rs-dean

Rust/WASM browser-game scaffold for Dean.

Every clone has a static GitHub Pages shape: `apps/marketing` for the Leptos
marketing surface, `apps/game` for the Bevy-only WebGPU game, `apps/stories` for
independent UI and scene proofs, `rs-dean-ui` for shared Leptos/Bevy design
tokens and themes, `rs-dean-idb` for isomorphic durable state, and generated
template proof under ignored `apps/test-project`.

## First Run

```bash
rustup show
just bootstrap
just doctor
just gate
just dev
just game
```

## Quality Gate

`cargo xtask gate` is the single pass for this repo. It formats, lints, tests,
checks wasm builds, audits dependencies, regenerates the template proof app,
verifies Trunk artifacts, verifies Pages artifacts, and sweeps docs for stale
stack references. The gate runs the native refresh hydration regression and
compiles the browser refresh hydration regression for wasm. It also runs the
generated `apps/test-project/cube-smoke` browser render check, which verifies a
centered square canvas, WebGPU startup, and the lit green-cube scene contract.

## App Surfaces

- `apps/marketing`: required Leptos marketing app. It may host Bevy canvas
  moments, but owns the marketing DOM surface. Styling is compiled by Trunk
  from the Tailwind entrypoint at `apps/marketing/styles/index.css`. It
  currently renders a hello-world page and boots through `rs-dean-state`
  so browser refreshes resume from durable local state.
- `apps/game`: required Bevy-only game app. The gate fails if it pulls in
  Leptos or drops the persistent-state wiring. It currently renders a Bevy
  hello-world scene.
- `apps/stories`: required independent story harness for reusable UI and scene
  proofs. Leptos stories use the same Trunk Tailwind asset path as marketing
  and include a theme gallery for every `rs-dean-ui` theme plus the full
  shadcn-inspired component catalog.
- `apps/test-project`: ignored generated proof from `templates/app`; it contains
  a generated Leptos app with Tailwind already wired, plus the generated
  cube-smoke app used by the render gate.

## Design Tokens

`crates/ui` owns the shared theme contract. Its Rust model contains the
EngManager-style primitive palettes, semantic aliases, theme cycle, and Bevy
color conversion. Its Tailwind v4 stylesheet at `crates/ui/styles/theme.css`
exports those semantics as theme variables, so Leptos components use normal
utilities such as `bg-surface-1`, `text-text-1`, `rounded-box`, and `shadow-2`.
Shared Leptos examples use the `rs-dean-ui` token scales for type, space,
radii, weight, leading, and elevation: `text-0`, `text-5`, `gap-m`, `p-s`,
`rounded-box`, `font-7`, `leading-0`, and `shadow-2`. Avoid mixing stock
Tailwind design-scale utilities like `text-sm`, `px-6`, `gap-4`, `rounded-lg`,
or `font-bold` in reusable UI so generated components follow the token system.

Bevy consumers depend on `rs-dean-ui` with `default-features = false` and
`features = ["bevy"]`, which keeps `apps/game` Bevy-only while sharing the same
palette and semantic token methods.

The shadcn-inspired component catalog is Rust data in `crates/ui`. Each entry
has a component definition, shared anatomy/spec blocks, an implementation
recipe, a literal Rust widget constructor, a named token-only Leptos component,
and Bevy primitive derivation from the same widget slots for scene consumers.
`_issues/` mirrors the catalog with one technical-program-management task per
component plus a sweep log for the repeated first-to-current audit.
`cargo xtask gen-ui-book` regenerates the UI crate mdBook from that Rust
catalog. The book has one page per component and embeds the matching live story
fixture so variant and state samples stay tied to the component harness.

## Doctor

`just doctor` is the fast local preflight. It checks tool availability,
including the standalone Tailwind CLI, the wasm target, Chrome discovery,
WebGPU feature wiring, common local ports, ignored generated outputs, and
required repo files. Use it before a long gate when a machine or checkout may be
stale.

## Durable State

`crates/idb` exposes the shared async object-store API. Browser wasm builds use
IndexedDB, native bundles use the native backend, and callers use the same
`Database` methods in both targets. Leptos and Bevy state flows should depend
on `crates/state` or `rs-dean-idb`, not on backend-specific storage crates.

The refresh contract is covered by `crates/state/tests/refresh_hydration.rs`.
It persists an app snapshot, hydrates separate Leptos-style and Bevy-style
caches, drops those caches, reopens durable storage, and asserts both caches
resume from the same snapshot.

All app updates that represent user progress, settings, unlocks, completion, or
other resumable state should use `rs-dean-state` update APIs. The required
marketing and game packages both declare persistent-state constraints, depend on
`rs-dean-state`, and hydrate durable state on browser boot.

## Bevy Render Smoke

`just cube-smoke` regenerates `apps/test-project`, builds
`apps/test-project/cube-smoke`, serves the generated files, launches headless
Chrome, verifies the centered square canvas page, attempts green-pixel readback,
and fails unless the WebGPU renderer and green material scene marker are
confirmed.

## GitHub Pages

`cargo xtask pages` builds an aggregate static artifact under `target/pages`.
The root index links binaries at `/marketing/`, `/game/`, and `/stories/`.
It also links `/crates/`, the workspace crate index. The crate index lists each
workspace crate and links `rs-dean-ui` to its book at `/crates/ui/`. The UI
crate book embeds `/stories/#ui-{component}` for live component fixtures. The
deploy workflow sets `RS_DEAN_PAGES_BASE=/rs-dean/` so Trunk emits
project-page-safe asset URLs.

## Skill Docs

- `.agents/skills/bevy-019/SKILL.md`
- `.agents/skills/rust-modern/SKILL.md`
- `.agents/skills/idb/SKILL.md`
- `.agents/skills/five-phase-pass/SKILL.md`

Read the relevant skill before changing that part of the stack.
