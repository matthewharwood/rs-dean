# Tooltip

A focus and hover popup that describes a control.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Tooltip examples.

<iframe title="Tooltip live story fixtures" src="../../../stories/?story=ui-tooltip" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-tooltip) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Overlay |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Overlay surface |

## Variants

- modal
- nonmodal
- anchored
- danger

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- Tooltip
- TooltipTrigger
- TooltipContent
- TooltipArrow

## Accessibility

- Trap focus only for modal surfaces.
- Restore focus to the trigger on close.
- Expose title and description for dialog-like content.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can discover a concise description for a focused or hovered control.
