# Empty

A structured empty state with optional illustration and recovery action.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Empty live story fixtures" src="../../../stories/#ui-empty" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-empty) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Feedback |
| Framework | Shared Spec |
| State | Stateless |
| Render contract | Shared Leptos/Bevy |
| State contract | Stateless |
| Layout contract | Feedback callout |

## Variants

- default
- success
- warning
- danger
- loading

## States

- default
- loading
- empty
- disabled
- themed

## Anatomy

- Empty
- EmptyHeader
- EmptyTitle
- EmptyDescription
- EmptyContent
- EmptyAction

## Accessibility

- Use status or alert semantics based on urgency.
- Keep recovery actions reachable by keyboard.
- Avoid motion-only status communication.

## Consumer Implementation

Consumer passes display data; renderer is a pure projection of props, theme, and tokens.

## End User Outcome

Users can understand why content is absent and what action can recover the flow.
