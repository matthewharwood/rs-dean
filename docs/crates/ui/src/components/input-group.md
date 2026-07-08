# Input Group

A composed input with addons, buttons, and inline affordances.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Input Group live story fixtures" src="../../../stories/#ui-input-group" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-input-group) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Form |
| Framework | Shared Spec |
| State | Consumer Durable |
| Render contract | Shared Leptos/Bevy |
| State contract | Consumer durable |
| Layout contract | Form control |

## Variants

- default
- filled
- invalid
- disabled
- readonly

## States

- controlled
- pending persistence
- hydrated
- invalid boundary input
- disabled

## Anatomy

- InputGroup
- InputGroupAddon
- InputGroupInput
- InputGroupButton

## Accessibility

- Bind labels, descriptions, and errors to controls.
- Expose invalid and required state explicitly.
- Do not persist field values inside renderer-local state.

## Consumer Implementation

Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents.

## End User Outcome

Users can complete text entry with addons and inline actions in one stable control row.
