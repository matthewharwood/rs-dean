---
name: five-phase-pass
description: "Use for any rs-dean stack, gate, template, docs, or shared-pattern change. Runs the Rust five-phase pass: app/crate, template, generated proof, docs/skill, and stale-reference sweep."
argument-hint: "[change description]"
---

## Objective

Keep `rs-dean` coherent when a change affects more than one surface. A change
is not complete until the source, template, generated proof app, durable docs,
and local skills all agree.

## Phases

### P1 — App Or Crate

Implement the change in the owning app or crate:

- `apps/marketing` for the required Leptos marketing app.
- `apps/game` for the required Bevy-only game app.
- `apps/stories` for isolated UI or scene proofs.
- `crates/*` for reusable Rust logic.
- `xtask` for commands, gates, generated templates, and artifact checks.

Leptos app and template styling uses Tailwind through Trunk's
`rel="tailwind-css"` asset type. Keep that link and the Tailwind import in
every Leptos scaffold; `xtask` owns the policy check and standalone CLI
installer.

Shared app theme work belongs in `crates/ui`: keep the Rust token/theme model,
the Tailwind token stylesheet, Leptos components, and any Bevy-facing feature
usage in sync. Bevy consumers must enable `rs-dean-ui` with
`default-features = false` and `features = ["bevy"]` so the game tree stays
Leptos-free.

Shared Leptos UI examples must use the `rs-dean-ui` Tailwind token vocabulary
for design scales: `text-0`, `gap-m`, `p-s`, `rounded-box`, `font-7`,
`leading-0`, `shadow-2`, and similar tokens. Do not mix stock Tailwind
design-scale utilities such as `text-sm`, `px-6`, `gap-4`, `rounded-lg`, or
`font-bold` into reusable UI examples; `xtask` enforces this on the app,
story, template, and shared component surfaces.

The shadcn-inspired catalog lives in `crates/ui`. Keep its Rust definitions,
implementation recipes, Leptos previews, Bevy primitive adapters,
`apps/stories` gallery, `_issues/` component task files, and sweep log in sync.

Use the local Bevy and modern-Rust skills before changing their owned surfaces.

### P2 — Template

Mirror any scaffold-impacting change into `templates/app`. New apps must be
born with the same gate and browser assumptions as the shipped app.

### P3 — Generated Proof And Gate

For a stale local checkout or a new machine, run the fast preflight first:

```bash
cargo xtask doctor
```

Run:

```bash
cargo xtask five-phase-pass
```

This regenerates ignored `apps/test-project`, runs the one-pass Rust gate, and
sweeps docs. `apps/test-project` is proof output only; never commit it.

### P4 — Docs And Owning Skill

Update `AGENTS.md`, `README.md`, and the owning skill under `.agents/skills/`.
Docs should describe the current Rust/WASM stack directly. Remove references to
retired tools instead of keeping historical explanations.

### P5 — Stale-Reference Sweep

Run:

```bash
cargo xtask docs-sweep
```

The sweep fails if docs or local skills mention retired non-Rust stack terms.
If a new stack retirement happens, add the term to the sweep in `xtask` and
remove the old wording from docs/skills.

## Acceptance Criteria

- `cargo xtask gate` passes.
- `cargo xtask doctor` passes when the task changes local tooling or gate
  prerequisites.
- `cargo xtask docs-sweep` passes.
- Template regeneration succeeds and `apps/test-project` remains untracked.
- `apps/marketing`, `apps/game`, and `apps/stories` produce Trunk `.wasm`,
  glue, and CSS artifacts; Leptos CSS artifacts are compiled through Tailwind.
- Leptos app, story, template, and shared component examples use
  `rs-dean-ui` token utilities for design scales instead of stock Tailwind
  scale classes.
- The shadcn-inspired catalog has one `_issues/` task file per component, and
  every catalog component can build a shared implementation recipe and spec for
  Leptos and Bevy consumers.
- Shared UI themes switch through Tailwind tokens in Leptos and through the
  same Rust palette in Bevy without adding Leptos to `rs-dean-game`.
- Required app packages keep persistent-state wiring through `rs-dean-state`.
- Generated `apps/test-project/cube-smoke` verifies the WebGPU smoke surface.
- The Bevy wasm feature tree contains WebGPU and no WebGL.
