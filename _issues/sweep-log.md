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

Concrete component pass: Dialog complete. The twenty-third issue now has a
typed modal/non-modal model, trigger/content/header/title/description/footer
contracts, footer action contracts, renderer-local open/focus/footer-action
state, `garde` validation, token-only Leptos dialog rendering, dedicated story
proof, and Bevy primitive derivation from the same model. The sweep found that
workflow overlays need their own action contract and close-on-activate behavior
instead of generic footer slots so Leptos and Bevy agree on modal state and
footer intent. Issues 01 through 22 were re-audited against this learning; no
earlier API changes were needed because Alert Dialog owns destructive
confirmation semantics while Dialog owns general workflow overlays.

Concrete component pass: Direction complete. The twenty-fourth issue now has a
typed left-to-right/right-to-left provider model, nested scope contract,
direction-aware content node, renderer-local provider/scope/focus state,
`garde` validation, token-only Leptos direction rendering, dedicated story
proof, and Bevy primitive derivation from the same model. The sweep found that
directionality should be modeled as a provider/scope utility with explicit
`dir` values rather than as generic selected widget slots, so both Leptos and
Bevy can agree on inherited and effective flow without persisting locale state
inside the component. Issues 01 through 23 were re-audited against this
learning; no earlier API changes were needed because overlay, data, and form
components already keep their own interaction state separate from document
flow.

Concrete component pass: Drawer complete. The twenty-fifth issue now has a
typed side-aware overlay model, trigger/content/header/footer/handle contracts,
footer action semantics, renderer-local open/focus/drag/action state, `garde`
validation, token-only Leptos drawer rendering, dedicated story proof, and
Bevy primitive derivation from the same model. The sweep found that edge-
attached overlays need side-specific geometry and handle drag state distinct
from general Dialog workflow state, while retaining the same close-on-action
footer pattern. Issues 01 through 24 were re-audited against this learning; no
earlier API changes were needed because Dialog owns centered workflows and
Direction owns document flow rather than overlay placement.

Concrete component pass: Dropdown Menu complete. The twenty-sixth issue now has
a typed trigger-attached menu model, grouped label/item/separator entry
contract, renderer-local open/focus/select state, `garde` validation,
token-only Leptos dropdown rendering, dedicated story proof, and Bevy primitive
derivation from the same model. The sweep found that command-like dropdowns
should share the menu action vocabulary with Context Menu while keeping trigger
ownership and selection state in their own contract. Issues 01 through 25 were
re-audited against this learning; no earlier API changes were needed because
Context Menu still owns pointer-target and submenu semantics while Dropdown
Menu owns trigger-attached grouped actions.

Concrete component pass: Empty complete. The twenty-seventh issue now has a
typed empty-state model, optional illustration label, recovery action contract,
renderer-local focus/activation state, `garde` validation, token-only Leptos
empty-state rendering, dedicated story proof, and Bevy primitive derivation from
the same model. The sweep found that Empty should expose the full shadcn anatomy
even when a consumer omits a recovery action, so the shared render-node path
keeps a disabled action node instead of dropping the part. Issues 01 through 26
were re-audited against this learning; no earlier API changes were needed
because prior optional-action components already preserve their component
anatomy while keeping disabled action affordances renderer-local.

Concrete component pass: Field complete. The twenty-eighth issue now has a
typed form-field model, input kind vocabulary, label/control/description/error
anatomy, renderer-local focus/input draft state, `garde` validation, token-only
Leptos field rendering, dedicated story proof, and Bevy primitive derivation
from the same model. The sweep found that form values can be edited locally for
component behavior, but durable form submission and saved field values must
remain consumer-owned through `crates/state` / `rs-dean-idb`. Issues 01 through
27 were re-audited against this learning; no earlier API changes were needed
because picker query state, command filtering, and checkbox toggles already
model renderer-local drafts separately from durable app state.

Concrete component pass: Hover Card complete. The twenty-ninth issue now has a
typed preview-overlay model, trigger/content/arrow anatomy, renderer-local
hover/focus open state, `garde` validation, token-only Leptos hover-card
rendering, dedicated story proof, and Bevy primitive derivation from the same
model. The sweep found that trigger-attached preview overlays need an explicit
local-open state and can expose opened content in Bevy proofs without promoting
hover state into durable app storage. Issues 01 through 28 were re-audited
against this learning; no earlier API changes were needed because disclosure,
dialog, drawer, dropdown, and field draft states already keep ephemeral
renderer state separate from persisted app data.

Concrete component pass: Input complete. The thirtieth issue now has a typed
single-line input model, input kind vocabulary, prefix/control/suffix anatomy,
optional suffix action and error state, renderer-local draft value state,
`garde` validation, token-only Leptos input rendering, dedicated story proof,
and Bevy primitive derivation from the same model. The sweep found that bare
form controls follow Field's local-draft rule while preserving optional prefix
and suffix anatomy as disabled render nodes when omitted. Issues 01 through 29
were re-audited against this learning; no earlier API changes were needed
because Field already separates draft control state from durable form
submission, while Button and menu actions already cover activation semantics.

Concrete component pass: Input Group complete. The thirty-first issue now has a
typed grouped-input model, Input-shared density/kind/action vocabulary,
addon/input/button anatomy, optional button and error state, renderer-local
draft value state, `garde` validation, token-only Leptos input-group rendering,
dedicated story proof, and Bevy primitive derivation from the same model. The
sweep found that composed form controls should reuse the bare Input vocabulary
for density, input kind, and suffix/button actions instead of adding parallel
types. Issues 01 through 30 were re-audited against this learning; Input was
already the canonical source for those primitives, and no earlier component API
needed to change.

Concrete component pass: Input OTP complete. The thirty-second issue now has a
typed fixed-length OTP model, Input-shared density vocabulary, repeatable
slot/group/separator anatomy, numeric and alphanumeric value validation,
renderer-local focus/input/paste state, `garde` validation, token-only Leptos
input-OTP rendering, dedicated story proof, and Bevy primitive derivation from
the same repeatable slot nodes. The sweep found that repeatable control parts
need stable per-index primitive names for Bevy while preserving the canonical
shadcn anatomy label for Leptos data attributes. Issues 01 through 31 were
re-audited against this learning; Button Group, Calendar, Data Table, and Date
Picker Bevy primitive adapters were upgraded to use stable value/date/index
suffixes for repeatable nodes, while their public Rust and Leptos APIs stayed
unchanged.

Concrete component pass: Item complete. The thirty-third issue now has a typed
content-row model, density/media/action contracts, media/content/title/
description/actions anatomy, repeatable action render nodes, renderer-local
action focus and activation state, `garde` validation, token-only Leptos item
rendering, dedicated story proof, and Bevy primitive derivation from the same
model. The sweep found that display rows can have multiple actions while still
presenting one canonical `ItemActions` anatomy part; Bevy gets stable indexed
primitive names and Leptos keeps the shadcn data attribute. Issues 01 through
32 were re-audited against this learning; no earlier API changes were needed
because repeatable action and repeatable primitive naming had already been
settled by Bubble, Button Group, and Input OTP.

Concrete component pass: Kbd complete. The thirty-fourth issue now has a typed
keyboard-shortcut model, density/key/separator contracts, repeatable `KbdKey`
render nodes, renderer-local key focus state, `garde` validation, token-only
Leptos `<kbd>` rendering, dedicated story proof, and Bevy primitive derivation
from the same model. The sweep found that typography tokens can still need
repeatable domain data and local focus styling while remaining non-durable
display state. Issues 01 through 33 were re-audited against this learning; no
earlier API changes were needed because repeated-key primitive naming follows
the same stable indexed convention established for Item, Input OTP, and other
repeatable components.

Concrete component pass: Label complete. The thirty-fifth issue now has a
typed form-label model, density/requirement contracts, root/text/requirement
render nodes, renderer-local hover/focus state, `garde` validation, token-only
Leptos `<label>` rendering, dedicated story proof, and Bevy primitive
derivation from the same model. The sweep found that form-adjacent typography
can expose stateful requirement metadata without owning durable form data.
Issues 01 through 34 were re-audited against this learning; no earlier API
changes were needed because optional visible/hidden anatomy and non-durable
renderer-local focus state were already covered by Field, Input, Item, and Kbd.

Concrete component pass: Marker complete. The thirty-sixth issue now has a
typed status-marker model, density/tone/anchor contracts, root/dot/label/
anchor render nodes, renderer-local hover/focus/navigation state, `garde`
validation, token-only Leptos marker rendering, dedicated story proof, and
Bevy primitive derivation from the same model. The sweep found that tiny
messaging primitives still need explicit visible/hidden anchor anatomy so
Leptos can stay stable while Bevy receives deterministic primitive slots.
Issues 01 through 35 were re-audited against this learning; no earlier API
changes were needed because hidden optional anatomy and local navigation state
were already covered by Breadcrumb, Button, Item, Kbd, and Label.

Concrete component pass: Menubar complete. The thirty-seventh issue now has a
typed application-menu model, density/menu/item contracts, repeatable menu/
trigger/content/item render nodes, renderer-local open/focus/activation state,
`garde` validation, token-only Leptos menubar rendering, dedicated story proof,
and Bevy primitive derivation from the same model. The sweep found that
menu-like components need hidden/visible render state separated from hard
disabled state so DOM renderers can open content after mount while Bevy still
gets deterministic initial primitives. Issues 01 through 36 were re-audited
against this learning; no earlier API changes were needed because existing
menu components already isolate renderer-local open state from durable app
state.

Concrete component pass: Message complete. The thirty-eighth issue now has a
typed durable-message model, density/side/action contracts, header/content/
footer/action render nodes, renderer-local focus/hover/action state, `garde`
validation, token-only Leptos message rendering, dedicated story proof, and
Bevy primitive derivation from the same model. The sweep found that messaging
components should keep transcript ownership app-durable while modeling side
and action feedback as renderer-local UI state. Issues 01 through 37 were
re-audited against this learning; no earlier API changes were needed because
Bubble, Attachment, Marker, and Menubar already keep durable workflow state
outside renderer-local interaction caches.

Concrete component pass: Message Scroller complete. The thirty-ninth issue now
has a typed transcript-viewport model, message-entry composition over
`MessageModel`, viewport/list/anchor/jump-button render nodes, renderer-local
scroll position and jump state, `garde` validation, token-only Leptos scroller
rendering, dedicated story proof, and Bevy primitive derivation from the same
model. The sweep found that scroll affordances need local position state while
the transcript entries remain consumer-owned durable data. Issues 01 through
38 were re-audited against this learning; no earlier API changes were needed
because Message and the existing navigation components already separate local
UI position from durable app state.

Concrete component pass: Native Select complete. The component now has a
typed Rust option/value model, `garde` validation for duplicate and disabled
selections, token-only native `<select>` rendering, dedicated story proof, and
Bevy primitive derivation from the same render nodes. The sweep found that the
native DOM control should stay browser-native while the shared contract names
the shadcn trigger/option/value anatomy for Bevy and Leptos. Issues 01 through
39 were re-audited against this learning; no earlier API changes were needed
because existing form and messaging components already keep durable ownership
above renderer-local interaction state.

Concrete component pass: Navigation Menu complete. The component now has a
typed Rust item/link model, `garde` validation for direct-link versus panel
items, token-only Leptos navigation rendering, dedicated story proof, and Bevy
primitive derivation from the same render nodes. The sweep found that
top-level navigation needs local open/focus state while selected routes remain
consumer-owned durable state. Issues 01 through 40 were re-audited against
this learning; no earlier API changes were needed because Menubar, Breadcrumb,
and Native Select already separate local navigation affordances from durable
selection ownership.

Concrete component pass: Pagination complete. The component now has a typed
Rust page-count/current-page model, `garde` validation for page ranges,
token-only Leptos page controls, dedicated story proof, and Bevy primitive
derivation from the same render nodes. The sweep found that pagination needs
renderer-local page/focus transitions while durable page ownership remains
with the consuming table or collection state. Issues 01 through 41 were
re-audited against this learning; no earlier API changes were needed because
Data Table already keeps pagination state in the collection owner and
Navigation Menu already exposes local navigation affordances without durable
storage.

Concrete component pass: Popover complete. The component now has a typed Rust
trigger/content model, `garde` validation for overlay copy and error text,
token-only Leptos trigger/content/arrow rendering, dedicated story proof, and
Bevy primitive derivation from the same render nodes. The sweep found that
click-open overlays share Hover Card's anchored anatomy but need a distinct
renderer-local toggle/focus contract so hover previews and explicit popovers
do not collapse into one API. Issues 01 through 42 were re-audited against
this learning; no earlier API changes were needed because Hover Card already
owns hover/focus preview state separately and Dialog/Dropdown Menu keep their
workflow and selection semantics in their own contracts.

Concrete component pass: Progress complete. The component now has a typed Rust
determinate/indeterminate value model, `garde` validation for value range and
status copy, token-only Leptos progressbar rendering, dedicated story proof,
and Bevy primitive derivation from the same render nodes. The sweep found that
Progress value ownership is consumer durable while only highlight/focus
feedback belongs in renderer-local state. Issues 01 through 43 were
re-audited against this learning; no earlier API changes were needed because
Data Table and Pagination already keep durable collection position with the
consumer and expose only renderer-local focus affordances in their components.

Concrete component pass: Radio Group complete. The component now has a typed
Rust option model, `garde` validation for unique values and selected-value
membership, token-only Leptos radio controls, dedicated story proof, and Bevy
primitive derivation from the same render nodes. The sweep found that
mutually exclusive form controls need the same selected-value validation as
pickers while keeping renderer-local focus separate from durable submitted
form state. Issues 01 through 44 were re-audited against this learning; no
earlier API changes were needed because Checkbox, Combobox, Native Select, and
Progress already keep consumer-owned values distinct from renderer-local
focus/highlight affordances.

Concrete component pass: Resizable complete. The component now has a typed
Rust panel model with per-panel min/max bounds, `garde` validation for unique
panels and 100 percent layouts, renderer-local handle resize state, token-only
Leptos range controls, dedicated story proof, and Bevy primitive derivation
from the same render nodes. The sweep found that layout controls need shared
clamping rules rather than renderer-specific drag math; earlier components were
re-audited against that learning, and no changes were required because their
local state machines already keep interaction math inside shared Rust.

Concrete component pass: Scroll Area complete. The component now has a typed
Rust item model, overflow-axis contract, `garde` validation for unique item
values and active-item membership, renderer-local viewport/bar state,
token-only Leptos scroll content, dedicated story proof, and Bevy primitive
derivation from the same render nodes. The sweep found that scroll affordances
should expose axis state without owning durable scroll position; Resizable was
re-audited against that learning and required no change because its durable
layout sizes remain consumer-owned while local handle state stays ephemeral.

Concrete component pass: Select complete. The component now has typed Rust
groups/options, `garde` validation for unique group/option values and
selected-value membership, renderer-local open/focus state, token-only Leptos
trigger/listbox rendering, dedicated story proof, and Bevy primitive derivation
from the same render nodes. The sweep found that custom select controls need
their own open/focus contract instead of reusing Native Select's platform
control state. Native Select, Combobox, Radio Group, and Scroll Area were
re-audited against that learning; no earlier changes were required because each
keeps its renderer-local affordances separate from durable selected values.

Concrete component pass: Separator complete. The component now has typed Rust
orientation, density, decorative-label semantics, `garde` validation for
optional labels and error copy, renderer-local focus/hover state, token-only
Leptos divider rendering, dedicated story proof, and Bevy primitive derivation
from the same render nodes. The sweep found that non-interactive layout
primitives still need explicit renderer-local state for focus/hover proofing,
but should not persist layout affordance state. Progress, Resizable, Scroll
Area, and Select were re-audited against that learning; no earlier changes were
required because their durable values remain consumer-owned and renderer
affordances stay local.

Concrete component pass: Sheet complete. The component now has a typed Rust
side/density/action model, `garde` validation for trigger/header/content/action
copy, renderer-local open/focus/footer activation state, token-only Leptos
edge-panel rendering, dedicated story proof, and Bevy primitive derivation from
the same render nodes. The sweep found that Sheet should share workflow-overlay
vocabulary with Dialog and Drawer while keeping edge placement and close action
semantics component-specific. Dialog, Drawer, Dropdown Menu, Popover, and
Separator were re-audited against that learning; no earlier changes were
required because their overlay, edge, and non-interactive layout semantics are
already modeled in their own typed contracts.

Concrete component pass: Sidebar complete. The component now has typed Rust
navigation groups/items, active-item validation, optional badges, renderer-local
collapse/focus/active state, token-only Leptos rail/menu rendering, dedicated
story proof, and Bevy primitive derivation from the same render nodes. The
sweep found that app navigation components need grouped domain data rather than
generic menu slots so collapsed rail state, active item, and badges stay
portable across Leptos and Bevy. Navigation Menu, Menubar, Breadcrumb, Sheet,
and Drawer were re-audited against that learning; no earlier changes were
required because each already owns its navigation or overlay semantics in a
component-specific Rust contract.

Concrete component pass: Skeleton complete. The component now has typed Rust
placeholder geometry, validated text-line count, loading/ready/static/disabled
state, renderer-local focus and animation-pause state, token-only Leptos
placeholder surfaces, dedicated story proof, and Bevy primitive derivation from
the same render nodes. The sweep found that non-interactive loading components
still need a concrete local state model so reduced-motion and hover/focus
previews can remain renderer-local without becoming persisted app progress.
Progress, Separator, Aspect Ratio, Avatar, and Empty were re-audited against
that learning; no earlier changes were required because each already separates
ephemeral renderer state from durable app state.

Concrete component pass: Slider complete. The component now has a typed Rust
numeric range model, min/max/step/value validation, renderer-local focus,
drag, and value state, token-only Leptos range rendering, dedicated story
proof, and Bevy primitive derivation from the same render nodes. The sweep
found that numeric controls need range and step alignment validated at the Rust
edge so generated DOM input values, Bevy primitives, and durable app handoff
cannot disagree. Progress, Input, Field, Resizable, and Pagination were
re-audited against that learning; no earlier changes were required because
each already validates numeric or user-entered boundaries before rendering.

Concrete component pass: Sonner complete. The component now has a typed Rust
toast stack model, unique toast value validation, renderer-local pause,
focus, action, and dismiss state, token-only Leptos toast viewport rendering,
dedicated story proof, and Bevy primitive derivation from the same notification
render nodes. The sweep found that transient notification dismissal belongs in
renderer-local state unless the consuming app chooses to persist delivery or
read receipts. Toast, Alert, Message, Message Scroller, and Popover were
re-audited against that learning; no earlier changes were required because
they already separate ephemeral renderer state from durable app-owned records.

All-issue sweep pass: Spinner through Typography complete. The shared
literal widget path now validates every catalog entry with `garde`, then
projects validated render nodes into the Leptos story/component renderer and
the generic Bevy primitive adapter. The sweep found that shadcn anatomy can
intentionally repeat item-like parts, such as `ButtonGroupItem`, so the shared
validator rejects unknown and missing anatomy while allowing repeated catalog
parts.

Concrete all-issue sweep pass: Spinner through Typography now have
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
- Direction components model inherited and effective flow explicitly, because `dir` is provider/scope state rather than a visual selected slot.
- Drawer components keep edge placement and handle drag state in the shared model, because mobile overlays need side-specific geometry that Dialog should not inherit.
- Dropdown Menu components share item/shortcut/destructive action vocabulary with Context Menu while keeping trigger-attached open and selection state separate.
- Empty components preserve a disabled action node when no recovery action exists so Leptos and Bevy still expose the full empty-state anatomy.
- Field components may keep focus and draft input local for renderer behavior, while durable submitted form data remains consumer-owned state.
- Trigger-attached preview overlays keep hover/focus open state renderer-local while Bevy primitive proofs can start opened to expose content and arrow anatomy.
- Bare input controls preserve optional prefix/suffix anatomy as disabled render nodes while keeping draft text renderer-local until consumers persist submitted values.
- Composed input controls reuse Input's density, input kind, and action vocabulary so grouped controls do not create parallel form-control APIs.
- Repeatable fixed-length controls expose stable indexed primitive names to Bevy while keeping canonical shadcn anatomy labels in Leptos data attributes.
- Display rows expose repeatable action nodes through one canonical anatomy part, with stable Bevy primitive indexes and renderer-local activation state.
- Typography tokens can own repeatable domain data and local focus styling without becoming durable app state.
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
- Workflow overlays need a typed footer action contract with close-on-activate semantics so general Dialog behavior stays distinct from destructive Alert Dialog confirmation.
- The Sweep Process is the standing rule for every next issue: implement the current component, audit all earlier concrete components, apply shared learnings, and restart at issue 01 until the implemented set is stable.
- DOM ids for concrete components route through one internal helper, with component-specific public wrappers only where consumer or renderer code benefits from named APIs.

## Current Result

All 64 shadcn-inspired issues are implemented by shared recipes, literal Rust widget constructors, concrete typed Rust models, named Leptos components, the story renderer, Bevy primitive adapters, `garde` validation, and issue-file status checklists. The repeated sweep found twenty-eight cross-cutting improvements: keep per-component APIs thin, centralize durable state, renderer coverage, accessibility, variants, end-user outcomes, slots, and typed intents in Rust, validate generic widgets before framework-specific rendering, use owned render nodes where consumer-provided copy enters the renderer, keep message/media activation state ephemeral unless the app persists a real workflow decision, keep image load/error fallback local to the renderer, express compact status variants through the typed component API rather than generated catalog slots, model repeatable concrete anatomy in domain data when the component's semantics depend on item order, keep messaging side/action state renderer-local unless the app persists a real transcript decision, promote primary action primitives into bespoke typed APIs so generated catalog slots cannot drift from canonical Button styling, reuse Button's action vocabulary for grouped action components, keep date-grid arithmetic in the shared Rust contract, encode tri-state form controls through a shared enum instead of renderer-local booleans, model single-region disclosure with a shared boolean open state instead of generated root-part toggles, put input-backed filtering in the shared picker contract instead of renderer-local list logic, model searchable command palettes with grouped action and shortcut contracts instead of generated catalog slots, model pointer-triggered nested menus with shared entry, separator, submenu, and destructive-action contracts, model tabular data with typed columns, rows, selected-row validation, and shared page math instead of generic table slots, compose date pickers from shared calendar date math plus picker-owned trigger, popover, and selected-value semantics, keep general workflow Dialog footer actions distinct from destructive Alert Dialog confirmation, model inherited/effective direction flow explicitly instead of treating `dir` as a generic selected slot, keep drawer side placement and handle drag state out of general dialog workflow contracts, keep dropdown trigger/selection state separate from context-menu pointer and submenu semantics, keep empty-state action anatomy present even when recovery actions are omitted, keep form field drafts renderer-local while durable submitted values stay app-owned, keep trigger-attached preview overlay state renderer-local while Bevy proofs can open content explicitly, preserve bare input prefix/suffix anatomy as disabled nodes while durable submitted values stay consumer-owned, and reuse Input's density/kind/action vocabulary for composed input controls.
