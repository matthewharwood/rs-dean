# Message Scroller

A scroll container optimized for live message history and jump-to-latest.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Message Scroller live story fixtures" src="../../../stories/#ui-message-scroller" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-message-scroller) when a wider canvas is
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

- MessageScroller
- MessageViewport
- MessageList
- MessageAnchor
- MessageJumpButton

## Accessibility

- Keep sender, timestamp, and delivery state available to assistive tech.
- Do not trap focus inside live message regions.
- Announce new messages only when consumer opts in.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can review message history and jump to the latest item when appropriate.
