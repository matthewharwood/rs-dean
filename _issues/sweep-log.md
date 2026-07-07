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

Concrete component pass: Bubble complete. The ninth issue now has a typed Rust
model, sender side/avatar/content/meta/action contracts, repeatable action
render nodes, renderer-local action state, `garde` validation, token-only
Leptos component, dedicated story proof, and Bevy primitive derivation from the
same model. The sweep found that messaging surfaces need side-aware layout and
action feedback in the shared domain contract while durable transcript updates
stay with the app/state layer. Issues 01 through 08 were re-audited against
this learning; no earlier API changes were needed because their renderer-local
state does not imply durable message or navigation ownership.

Concrete component pass: Button complete. The tenth issue now has a typed Rust
model, variant/size/kind contracts, optional icon and href contracts,
renderer-local press state, `garde` validation, token-only Leptos button/link
rendering, dedicated story proof, and Bevy primitive derivation from the same
model. The sweep found that the primary action primitive must leave the generic
catalog path so AI and consumer code do not mix generated Button slot classes
with the canonical typed Button API. Issues 01 through 09 were re-audited
against this learning; no earlier API changes were needed because their
activation, navigation, and renderer-local states are already represented in
their component-specific contracts.

Concrete component pass: Button Group complete. The eleventh issue now has a
typed Rust model, item/orientation contracts, Button-shared variant and size
contracts, repeatable item/separator render nodes, renderer-local selected
state, `garde` validation, token-only Leptos grouped action rendering,
dedicated story proof, and Bevy primitive derivation from the same model. The
sweep found that grouped action surfaces should reuse the primary Button action
vocabulary instead of introducing a parallel variant or size enum. Issues 01
through 10 were re-audited against this learning; Button already owns the
canonical action vocabulary, and earlier components do not need grouped
selection semantics.

Concrete component pass: Calendar complete. The twelfth issue now has typed
date and range models, single/range selection mode, six-week month-grid
generation, repeatable day render nodes, renderer-local month and selection
state, `garde` validation, token-only Leptos calendar rendering, dedicated
story proof, and Bevy primitive derivation from the same model. The sweep found
that date-grid components should keep calendar math inside the shared Rust
contract instead of pulling in a renderer-specific date dependency or encoding
dates as strings. Issues 01 through 11 were re-audited against this learning;
no earlier API changes were needed because none of them encode calendar
arithmetic or durable date selection semantics.

Concrete component pass: Card complete. The thirteenth issue now has a typed
Rust model, variant/density/action contracts, six-part shadcn anatomy render
nodes, renderer-local footer action state, `garde` validation, token-only
Leptos card rendering, dedicated story proof, and Bevy primitive derivation
from the same model. The sweep found that framed display surfaces should keep
footer actions in the component-specific model instead of relying on generic
catalog slots for action behavior. Issues 01 through 12 were re-audited
against this learning; no earlier API changes were needed because their action
state is already represented in component-specific contracts.

Concrete component pass: Carousel complete. The fourteenth issue now has a
typed Rust slide model, density and loop contracts, repeated item render nodes,
renderer-local current-index navigation state, `garde` validation, token-only
Leptos carousel rendering, dedicated story proof, and Bevy primitive derivation
from the same model. The sweep found that paged content components need typed
collections and shared navigation state instead of generic navigate slots.
Issues 01 through 13 were re-audited against this learning; no earlier API
changes were needed because their repeatable collections and action states are
already represented in component-specific contracts.

Concrete component pass: Chart complete. The fifteenth issue now has a typed
Rust series model, density/tone/value contracts, repeated series and legend
render nodes, renderer-local selected-series state, `garde` validation,
token-only Leptos chart rendering, dedicated story proof, and Bevy primitive
derivation from the same model. The sweep found that data visualization
components need value-range validation and a typed selected-series boundary
instead of generic chart slots. Issues 01 through 14 were re-audited against
this learning; no earlier API changes were needed because their numeric data
contracts are either absent or already component-specific.

Concrete component pass: Checkbox complete. The sixteenth issue now has a
typed tri-state model, density/value/required/error contracts, four-part
checkbox anatomy render nodes, renderer-local checked state, `garde`
validation, token-only Leptos checkbox rendering, dedicated story proof, and
Bevy primitive derivation from the same model. The sweep found that form
controls with mixed state need an explicit shared enum instead of boolean-only
renderer state or generic checkbox slots. Issues 01 through 15 were re-audited
against this learning; no earlier API changes were needed because their local
state machines already expose component-specific enums or value contracts.

Concrete component pass: Collapsible complete. The seventeenth issue now has a
typed disclosure model, density/value/default-open contracts, trigger/content
render nodes, renderer-local open state, `garde` validation, token-only Leptos
collapsible rendering, dedicated story proof, and Bevy primitive derivation
from the same model. The sweep found that single-region disclosure should own a
boolean open state in its shared contract rather than using generated root-part
toggle slots. Issues 01 through 16 were re-audited against this learning; no
earlier API changes were needed because multi-item disclosure, selection, and
tri-state controls already expose component-specific state transitions.

Concrete component pass: Combobox complete. The eighteenth issue now has a
typed option model, placeholder/empty-label/query contracts, repeated filtered
option render nodes, renderer-local open/query/selected state, `garde`
validation, token-only Leptos combobox rendering, dedicated story proof, and
Bevy primitive derivation from the same model. The sweep found that
input-backed pickers need shared filtering and selected-value validation rather
than generic input/list/option slots. Issues 01 through 17 were re-audited
against this learning; no earlier API changes were needed because previous
components either do not filter collections or already validate selected values
against typed item collections.

Concrete component pass: Command complete. The nineteenth issue now has a
typed group/item model, placeholder/empty-label/query contracts, repeated
group/item/shortcut render nodes, renderer-local open/query/highlight/selected
state, `garde` validation, token-only Leptos command rendering, dedicated
story proof, and Bevy primitive derivation from the same model. The sweep found
that searchable action palettes need shared grouping, shortcut, and highlighted
value contracts instead of generic command slots. Issues 01 through 18 were
re-audited against this learning; no earlier API changes were needed because
only Command combines input-backed filtering with grouped action shortcuts.

Concrete component pass: Context Menu complete. The twentieth issue now has a
typed entry/submenu model, trigger/content contracts, repeated
item/separator/submenu render nodes, renderer-local open/active/selected
submenu state, `garde` validation, token-only Leptos context-menu rendering,
dedicated story proof, and Bevy primitive derivation from the same model. The
sweep found that pointer-triggered nested menus need shared separator,
submenu, and destructive-action contracts instead of generic context-menu
slots. Issues 01 through 19 were re-audited against this learning; no earlier
API changes were needed because only Context Menu combines pointer-opened
overlay state with nested menu entries.

Concrete component pass: Data Table complete. The twenty-first issue now has a
typed column/row model, filter/sort/page/selected-row contracts, repeated
header/row/cell render nodes, renderer-local filter/sort/pagination/selection
state, `garde` validation, token-only Leptos data-table rendering, dedicated
story proof, and Bevy primitive derivation from the same model. The sweep found
that tabular data surfaces need shared column identity, row identity, and page
math instead of generic table slots so Leptos and Bevy agree on sorting,
filtering, pagination, empty state, and selected rows. Issues 01 through 20
were re-audited against this learning; no earlier API changes were needed
because none of them combine typed tabular cells with filter/sort/page state.

Concrete component pass: Date Picker complete. The twenty-second issue now has
a typed date-picker model, trigger/popover/calendar/value contracts, repeated
calendar-day render nodes, renderer-local open/month/selected-value state,
`garde` validation, token-only Leptos date-picker rendering, dedicated story
proof, and Bevy primitive derivation from the same model. The sweep found that
composed date-input controls should reuse the shared calendar date arithmetic
while owning picker-specific popover and value semantics in their own contract.
Issues 01 through 21 were re-audited against this learning; no earlier API
changes were needed because Calendar already owns month math and Data Table's
page math remains a separate tabular concern.

All-issue sweep pass: Dialog through Typography complete. The shared
literal widget path now validates every catalog entry with `garde`, then
projects validated render nodes into the Leptos story/component renderer and
the generic Bevy primitive adapter. The sweep found that shadcn anatomy can
intentionally repeat item-like parts, such as `ButtonGroupItem`, so the shared
validator rejects unknown and missing anatomy while allowing repeated catalog
parts.

Concrete all-issue sweep pass: Dialog through Typography now have
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
- Messaging components can own renderer-local action feedback and side-aware layout while durable transcript/message updates stay app-owned.
- Identity media components can own renderer-local load/error fallback state while keeping durable profile identity in app/state models.
- Repeatable shadcn anatomy parts are valid component structure; validation enforces catalog membership and full coverage instead of uniqueness, while bespoke components can model repeated anatomy directly in typed domain data.
- Action primitives need a bespoke typed API for variant, size, kind, optional icon, and optional href semantics so consumer code does not mix generated Button slots with canonical action styling.
- Grouped action components reuse the Button variant and size vocabulary so consumers do not learn or mix parallel action class systems.
- Date-grid components keep date arithmetic, range ordering, and month navigation in the shared Rust contract so Leptos and Bevy do not invent separate date string conventions.
- Framed display surfaces keep footer actions in a component-specific model so generic catalog slots do not become the source of consumer action semantics.
- Paged content components need typed item collections and shared navigation state so Leptos and Bevy derive previous/next disabled state from the same model.
- Data visualization components need value-range validation and typed selected-series state so renderer tooltips, legends, and scene primitives agree.
- Tri-state form controls need an explicit checked-state enum so unchecked, checked, and indeterminate render identically across Leptos and Bevy.
- Single-region disclosure controls need a shared boolean open-state contract so trigger disabled state and hidden content match across Leptos and Bevy.
- Input-backed pickers need shared filtering and selected-value validation so Leptos input behavior and Bevy option primitives agree.
- Searchable action palettes need shared grouping, shortcut, highlighted-value, and selected-value contracts so Leptos command behavior and Bevy item primitives agree.
- Pointer-triggered nested menus need shared entry, separator, submenu, and destructive-action contracts so Leptos overlay behavior and Bevy menu primitives agree.
- Tabular data surfaces need typed columns, typed rows, selected-row validation, and shared page math so filtering, sorting, pagination, empty state, and row selection do not drift across Leptos and Bevy.
- Composed date-input controls reuse the shared calendar date arithmetic while owning picker-specific trigger, popover, selected-value, and close-on-select semantics in their own contract.
- The Sweep Process is the standing rule for every next issue: implement the current component, audit all earlier concrete components, apply shared learnings, and restart at issue 01 until the implemented set is stable.
- DOM ids for concrete components route through one internal helper, with component-specific public wrappers only where consumer or renderer code benefits from named APIs.

## Current Result

All 64 shadcn-inspired issues are implemented by shared recipes, literal Rust widget constructors, concrete typed Rust models, named Leptos components, the story renderer, Bevy primitive adapters, `garde` validation, and issue-file status checklists. The repeated sweep found nineteen cross-cutting improvements: keep per-component APIs thin, centralize durable state, renderer coverage, accessibility, variants, end-user outcomes, slots, and typed intents in Rust, validate generic widgets before framework-specific rendering, use owned render nodes where consumer-provided copy enters the renderer, keep message/media activation state ephemeral unless the app persists a real workflow decision, keep image load/error fallback local to the renderer, express compact status variants through the typed component API rather than generated catalog slots, model repeatable concrete anatomy in domain data when the component's semantics depend on item order, keep messaging side/action state renderer-local unless the app persists a real transcript decision, promote primary action primitives into bespoke typed APIs so generated catalog slots cannot drift from canonical Button styling, reuse Button's action vocabulary for grouped action components, keep date-grid arithmetic in the shared Rust contract, encode tri-state form controls through a shared enum instead of renderer-local booleans, model single-region disclosure with a shared boolean open state instead of generated root-part toggles, put input-backed filtering in the shared picker contract instead of renderer-local list logic, model searchable command palettes with grouped action and shortcut contracts instead of generated catalog slots, model pointer-triggered nested menus with shared entry, separator, submenu, and destructive-action contracts, model tabular data with typed columns, rows, selected-row validation, and shared page math instead of generic table slots, and compose date pickers from shared calendar date math plus picker-owned trigger, popover, and selected-value semantics.
