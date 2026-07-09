# Button

The primary action primitive for commands, links, and submit actions.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Button examples.

<iframe title="Button live story fixtures" src="../../../stories/?story=ui-button" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-button) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Action |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Action row |

## Variants

- primary
- secondary
- quiet
- destructive
- icon

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- Button
- ButtonIcon
- ButtonLabel

## Accessibility

- Use semantic buttons or links.
- Expose disabled and pressed state to assistive tech.
- Keep visible focus from the focus-ring token.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can trigger a clear command with strong focus, disabled, loading, and destructive states.
