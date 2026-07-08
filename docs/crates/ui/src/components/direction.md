# Direction

A direction provider for left-to-right and right-to-left composition.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Direction live story fixtures" src="../../../stories/#ui-direction" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-direction) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Utility |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Utility scope |

## Variants

- ltr
- rtl
- nested

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- DirectionProvider
- DirectionScope
- DirectionAwareContent

## Accessibility

- Apply direction or utility scope without changing semantic order.
- Keep nested scopes explicit.
- Do not hide framework-specific assumptions in utility components.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can render nested left-to-right or right-to-left scopes without duplicating components.
