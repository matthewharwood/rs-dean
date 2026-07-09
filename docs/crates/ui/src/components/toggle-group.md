# Toggle Group

A grouped single-select or multi-select toggle set.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Toggle Group examples.

<iframe title="Toggle Group live story fixtures" src="../../../stories/?story=ui-toggle-group" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-toggle-group) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Action |
| Framework | Shared Spec |
| State | Consumer Durable |
| Render contract | Shared Leptos/Bevy |
| State contract | Consumer durable |
| Layout contract | Action row |

## Variants

- primary
- secondary
- quiet
- destructive
- icon

## States

- controlled
- pending persistence
- hydrated
- invalid boundary input
- disabled

## Anatomy

- ToggleGroup
- ToggleGroupItem
- ToggleGroupIndicator

## Accessibility

- Use semantic buttons or links.
- Expose disabled and pressed state to assistive tech.
- Keep visible focus from the focus-ring token.

## Consumer Implementation

Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents.

## End User Outcome

Users can switch one or more related toggles while the group state remains coherent.
