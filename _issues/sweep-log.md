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

Concrete component pass: Aspect Ratio complete. The fourth issue now has a
typed stateless Rust model, fit and ratio contract, `garde` validation, render
nodes, token-only Leptos component, dedicated story proof, and Bevy primitive
derivation from the same model. The sweep found that generated concrete catalog
coverage was not enough for issue 04 because consumers need an owned ratio
contract and renderer-specific frame semantics. Aspect Ratio moved out of the
generic catalog component set and now joins Accordion, Alert, and Alert Dialog
as a bespoke component. Issues 01 through 03 were re-audited against this
learning; no API changes were needed because their bespoke contracts already
own their domain-specific state or anatomy.

Concrete component pass: Attachment complete. The fifth issue now has a typed
Rust model, preview kind contract, optional action contract, renderer-local
activation state, `garde` validation, render nodes, token-only Leptos
component, dedicated story proof, and Bevy primitive derivation from the same
model. The sweep found that message-surface components can still need local
ephemeral state even when durable file ownership stays in the app/state layer.
Issues 01 through 04 were re-audited against this learning; no earlier API
changes were needed because Accordion already owns renderer-local disclosure
state, Alert and Aspect Ratio stay stateless by design, and Alert Dialog owns
ephemeral open/confirm state.

Concrete component pass: Avatar complete. The sixth issue now has a typed Rust
model, optional image contract, fallback initials contract, size variants,
renderer-local image/fallback state, `garde` validation, render nodes,
token-only Leptos component, dedicated story proof, and Bevy primitive
derivation from the same model. The sweep found that identity media needs a
local failure path for image errors without persisting renderer-only load
events. Issues 01 through 05 were re-audited against this learning; no earlier
API changes were needed because their local states already represent explicit
user interaction or valid renderer fallback behavior.

Concrete component pass: Badge complete. The seventh issue now has a typed Rust
model, tone/variant/size contracts, optional icon contract, renderer-local
highlight state, `garde` validation, render nodes, token-only Leptos component,
dedicated story proof, and Bevy primitive derivation from the same model. The
sweep found that compact status surfaces still need a stable domain API for
visual variants so consumers do not mix generated catalog slots with bespoke
Tailwind token classes. Issues 01 through 06 were re-audited against this
learning; no earlier API changes were needed because their domain variants are
already expressed directly in their typed models.

Concrete component pass: Breadcrumb complete. The eighth issue now has a typed
Rust model, route entry contract, density/separator contracts, repeatable
render nodes, renderer-local navigation focus state, `garde` validation,
token-only Leptos component, dedicated story proof, and Bevy primitive
derivation from the same model. The sweep found that concrete components can
own repeatable shadcn anatomy directly instead of flattening repeated item
parts through the generic catalog model. Issues 01 through 07 were re-audited
against this learning; no earlier API changes were needed because their
repeatable surfaces are either not present or already represented in typed
domain data.

All-issue sweep pass: Bubble through Typography complete. The shared
literal widget path now validates every catalog entry with `garde`, then
projects validated render nodes into the Leptos story/component renderer and
the generic Bevy primitive adapter. The sweep found that shadcn anatomy can
intentionally repeat item-like parts, such as `ButtonGroupItem`, so the shared
validator rejects unknown and missing anatomy while allowing repeated catalog
parts.

Concrete all-issue sweep pass: Bubble through Typography now have
component-specific public Rust APIs generated from one shared contract. Each
non-bespoke component exposes its own model, part enum, local state type,
intent/change aliases, render node alias, default model, validator, and render
node function. The story gallery routes every catalog ID through the named
Leptos component, and Bevy primitive derivation consumes the same concrete
render-node layer. This sweep found that consumer-provided models need owned
render node copy instead of static literals, so the Leptos slot renderer now
uses a renderer-neutral owned view node.

## Consolidated Learnings

- Bespoke one-off APIs would drift across Leptos and Bevy, so the implementation source of truth is the combination of `ComponentImplementation` in `crates/ui/src/kit.rs` and literal `UiWidget` constructors in `crates/ui/src/widgets.rs`.
- Leptos exports named shadcn-style components that render from the same literal widget slots and token classes used by the story gallery.
- Bevy consumes the same literal widget slots through primitive derivation, keeping the game tree Leptos-free.
- Consumer-durable state is documented as app-owned and persisted through `crates/state` / `rs-dean-idb`; component renderers only emit typed intent boundaries.
- The token-class guard scans every `crates/ui/src` Rust file so later component work cannot reintroduce stock Tailwind design-scale utilities.
- Concrete components use `garde` at the shared Rust model boundary before a renderer accepts props, so Leptos and Bevy consume the same validated component contract.
- Generic catalog components use `garde` at the literal `UiWidget` boundary before Leptos or Bevy renders them.
- Non-bespoke catalog components now expose concrete generated Rust APIs over the shared implementation so consumer code gets named model, part, state, intent, validation, and render-node types without duplicating behavior.
- The shared Leptos slot renderer accepts owned view nodes, which keeps consumer-provided model copy valid while preserving token-only classes.
- Message and media display components can own renderer-local ephemeral state for activation feedback while keeping durable file selections and uploads in `crates/state` / `rs-dean-idb`.
- Identity media components can own renderer-local load/error fallback state while keeping durable profile identity in app/state models.
- Repeatable shadcn anatomy parts are valid component structure; validation enforces catalog membership and full coverage instead of uniqueness, while bespoke components can model repeated anatomy directly in typed domain data.
- The Sweep Process is the standing rule for every next issue: implement the current component, audit all earlier concrete components, apply shared learnings, and restart at issue 01 until the implemented set is stable.
- DOM ids for concrete components route through one internal helper, with component-specific public wrappers only where consumer or renderer code benefits from named APIs.

## Current Result

All 64 shadcn-inspired issues are implemented by shared recipes, literal Rust widget constructors, concrete typed Rust models, named Leptos components, the story renderer, Bevy primitive adapters, `garde` validation, and issue-file status checklists. The repeated sweep found eight cross-cutting improvements: keep per-component APIs thin, centralize durable state, renderer coverage, accessibility, variants, end-user outcomes, slots, and typed intents in Rust, validate generic widgets before framework-specific rendering, use owned render nodes where consumer-provided copy enters the renderer, keep message/media activation state ephemeral unless the app persists a real workflow decision, keep image load/error fallback local to the renderer, express compact status variants through the typed component API rather than generated catalog slots, and model repeatable concrete anatomy in domain data when the component’s semantics depend on item order.
