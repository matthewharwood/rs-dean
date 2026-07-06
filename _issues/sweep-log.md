# Component Implementation Sweep Log

Protocol: implement one component, then re-audit every previously implemented component before moving to the next issue. Any learning is folded back into the shared recipe, renderer, or tests, then the sweep restarts at the first implemented component.

Completed pass: Accordion through Typography.

## Consolidated Learnings

- Bespoke one-off APIs would drift across Leptos and Bevy, so the implementation source of truth is the combination of `ComponentImplementation` in `crates/ui/src/kit.rs` and literal `UiWidget` constructors in `crates/ui/src/widgets.rs`.
- Leptos exports named shadcn-style components that render from the same literal widget slots and token classes used by the story gallery.
- Bevy consumes the same literal widget slots through primitive derivation, keeping the game tree Leptos-free.
- Consumer-durable state is documented as app-owned and persisted through `crates/state` / `rs-dean-idb`; component renderers only emit typed intent boundaries.
- The token-class guard scans every `crates/ui/src` Rust file so later component work cannot reintroduce stock Tailwind design-scale utilities.

## Current Result

All 64 shadcn-inspired issues are implemented by shared recipes, literal Rust widget constructors, named Leptos components, the story renderer, Bevy primitive adapters, and issue-file status checklists. The repeated sweep found two cross-cutting improvements: keep per-component APIs thin and centralize durable state, renderer coverage, accessibility, variants, end-user outcomes, slots, and typed intents in Rust.
