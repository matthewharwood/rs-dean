# Toast

A transient notification item with optional action.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Toast examples.

<iframe title="Toast live story fixtures" src="../../../stories/?story=ui-toast" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-toast) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Feedback |
| Framework | Leptos Only |
| State | Ephemeral |
| Render contract | Leptos DOM only |
| State contract | Renderer ephemeral |
| Layout contract | Feedback callout |

## Variants

- default
- success
- warning
- danger
- loading

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- ToastProvider
- ToastViewport
- Toast
- ToastTitle
- ToastDescription
- ToastAction

## Accessibility

- Use status or alert semantics based on urgency.
- Keep recovery actions reachable by keyboard.
- Avoid motion-only status communication.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users receive transient feedback with optional action and non-blocking dismissal.
