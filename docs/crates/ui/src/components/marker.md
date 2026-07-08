# Marker

A small position, unread, or annotation marker in message surfaces.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Marker live story fixtures" src="../../../stories/#ui-marker" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-marker) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Messaging |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Messaging thread |

## Variants

- incoming
- outgoing
- system
- compact

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- Marker
- MarkerDot
- MarkerLabel
- MarkerAnchor

## Accessibility

- Keep sender, timestamp, and delivery state available to assistive tech.
- Do not trap focus inside live message regions.
- Announce new messages only when consumer opts in.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can notice unread, positional, or annotation state without disrupting layout.
