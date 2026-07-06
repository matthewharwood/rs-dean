# Component Implementation Sweep Log

Protocol: implement one component, then re-audit every previously implemented component before moving to the next issue. Any learning is folded back into the shared recipe, renderer, or tests, then the sweep restarts at the first implemented component.

Completed pass: Accordion through Typography.

## Consolidated Learnings

- Bespoke one-off APIs would drift across Leptos and Bevy, so the implementation source of truth is `ComponentImplementation` in `crates/ui/src/kit.rs`.
- Leptos renders from the same implementation recipe and token classes used by the story gallery.
- Bevy consumes the same implementation through primitive derivation, keeping the game tree Leptos-free.
- Consumer-durable state is documented as app-owned and persisted through `crates/state` / `rs-dean-idb`; component renderers only emit typed intent boundaries.
- The token-class guard scans every `crates/ui/src` Rust file so later component work cannot reintroduce stock Tailwind design-scale utilities.

## Current Result

All 64 shadcn-inspired issues are implemented by the shared recipe layer, story renderer, Bevy primitive adapter, and issue-file status checklist. The repeated sweep found one cross-cutting improvement: keep per-component APIs thin and centralize durable state, renderer coverage, accessibility, variants, and end-user outcomes in the Rust implementation recipe.
