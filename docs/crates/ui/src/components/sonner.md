# Sonner

A toast notification viewport modeled after Sonner semantics.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Sonner examples.

<iframe title="Sonner live story fixtures" src="../../../stories/?story=ui-sonner" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-sonner) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Feedback |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
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

- SonnerProvider
- SonnerViewport
- SonnerToast
- SonnerAction
- SonnerDismiss

## Accessibility

- Use status or alert semantics based on urgency.
- Keep recovery actions reachable by keyboard.
- Avoid motion-only status communication.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users receive a stack of transient notifications with consistent action and dismiss behavior.
