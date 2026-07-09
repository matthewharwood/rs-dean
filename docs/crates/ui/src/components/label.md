# Label

Accessible text associated with a form control.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Label examples.

<iframe title="Label live story fixtures" src="../../../stories/?story=ui-label" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-label) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Form |
| Framework | Shared Spec |
| State | Stateless |
| Render contract | Shared Leptos/Bevy |
| State contract | Stateless |
| Layout contract | Form control |

## Variants

- default
- filled
- invalid
- disabled
- readonly

## States

- default
- loading
- empty
- disabled
- themed

## Anatomy

- Label
- LabelText
- LabelRequirement

## Accessibility

- Bind labels, descriptions, and errors to controls.
- Expose invalid and required state explicitly.
- Do not persist field values inside renderer-local state.

## Consumer Implementation

Consumer passes display data; renderer is a pure projection of props, theme, and tokens.

## End User Outcome

Users can associate control names and requirement state with the correct field.
