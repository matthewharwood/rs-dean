# Component Implementation Sweep Log

Sweep Process: implement one component, then re-audit every previously
implemented component before moving to the next issue. Any learning is folded
back into the shared recipe, renderer, or tests, then the sweep restarts at the
first implemented component.

Completed pass: Accordion through Typography.

Concrete component pass: Accordion complete. The first issue now has a
component-specific Rust model, state transition API, render nodes, token-only
Leptos component, dedicated story proof, and Bevy primitive derivation from the
same model. Follow-up validation pass added `garde::Validate` to the shared
Accordion model and item types so component props are checked before rendering.

Concrete component pass: Alert complete. The second issue now has a typed
stateless Rust model, optional action contract, semantic tone and density
variants, `garde` validation, render nodes, token-only Leptos component,
dedicated story proof, and Bevy primitive derivation from the same model.
Accordion was re-audited against this instrumentation; no Accordion API change
was required because its model/state split, validator boundary, render nodes,
story proof, and Bevy adapter already match the shared setup. Alert was updated
during the sweep so its root render node identifies the component while
`AlertTitle` carries user copy, matching Accordion's root-node convention.

Concrete component pass: Alert Dialog complete. The third issue now has a typed
ephemeral Rust model, button contract, renderer-local state transition API,
`garde` validation, render nodes, token-only Leptos component, dedicated story
proof, and Bevy primitive derivation from the same model. The first sweep found
shared DOM id slugging was duplicated between Accordion and Alert Dialog, so the
id algorithm moved into an internal `dom` helper and the sweep restarted at
Accordion. The second sweep found no further Accordion, Alert, or Alert Dialog
changes were needed because their validator boundary, render-node root
convention, story proof, and Bevy adapter shape now match.

All-issue sweep pass: Aspect Ratio through Typography complete. The shared
literal widget path now validates every catalog entry with `garde`, then
projects validated render nodes into the Leptos story/component renderer and
the generic Bevy primitive adapter. The sweep found that shadcn anatomy can
intentionally repeat item-like parts, such as `ButtonGroupItem`, so the shared
validator rejects unknown and missing anatomy while allowing repeated catalog
parts.

## Consolidated Learnings

- Bespoke one-off APIs would drift across Leptos and Bevy, so the implementation source of truth is the combination of `ComponentImplementation` in `crates/ui/src/kit.rs` and literal `UiWidget` constructors in `crates/ui/src/widgets.rs`.
- Leptos exports named shadcn-style components that render from the same literal widget slots and token classes used by the story gallery.
- Bevy consumes the same literal widget slots through primitive derivation, keeping the game tree Leptos-free.
- Consumer-durable state is documented as app-owned and persisted through `crates/state` / `rs-dean-idb`; component renderers only emit typed intent boundaries.
- The token-class guard scans every `crates/ui/src` Rust file so later component work cannot reintroduce stock Tailwind design-scale utilities.
- Concrete components use `garde` at the shared Rust model boundary before a renderer accepts props, so Leptos and Bevy consume the same validated component contract.
- Generic catalog components use `garde` at the literal `UiWidget` boundary before Leptos or Bevy renders them.
- Repeatable shadcn anatomy parts are valid component structure; validation enforces catalog membership and full coverage instead of uniqueness.
- The Sweep Process is the standing rule for every next issue: implement the current component, audit all earlier concrete components, apply shared learnings, and restart at issue 01 until the implemented set is stable.
- DOM ids for concrete components route through one internal helper, with component-specific public wrappers only where consumer or renderer code benefits from named APIs.

## Current Result

All 64 shadcn-inspired issues are implemented by shared recipes, literal Rust widget constructors, named Leptos components, the story renderer, Bevy primitive adapters, `garde` validation, and issue-file status checklists. The repeated sweep found three cross-cutting improvements: keep per-component APIs thin, centralize durable state, renderer coverage, accessibility, variants, end-user outcomes, slots, and typed intents in Rust, and validate generic widgets before framework-specific rendering.
