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

- `apps/web` for the shipped browser app.
- `apps/stories` for isolated UI or scene proofs.
- `crates/*` for reusable Rust logic.
- `xtask` for commands, gates, generated templates, and artifact checks.

Use the local Bevy and modern-Rust skills before changing their owned surfaces.

### P2 — Template

Mirror any scaffold-impacting change into `templates/app`. New apps must be
born with the same gate and browser assumptions as the shipped app.

### P3 — Generated Proof And Gate

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
- `cargo xtask docs-sweep` passes.
- Template regeneration succeeds and `apps/test-project` remains untracked.
- `apps/web` and `apps/stories` both produce Trunk `.wasm`, glue, and CSS
  artifacts.
- The Bevy wasm feature tree contains WebGPU and no WebGL.
