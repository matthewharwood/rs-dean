# Spinner

A compact activity indicator.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Spinner live story fixtures" src="../../../stories/#ui-spinner" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-spinner) when a wider canvas is
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

- Spinner
- SpinnerTrack
- SpinnerIndicator
- SpinnerLabel

## Accessibility

- Use status or alert semantics based on urgency.
- Keep recovery actions reachable by keyboard.
- Avoid motion-only status communication.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users see compact activity feedback when a region is busy.
