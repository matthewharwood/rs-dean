# Bubble

A chat or activity message container with sender-aware alignment.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Bubble examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Bubble Leptos live story fixtures" src="../../../stories/?story=ui-bubble" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Bubble Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-bubble" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-bubble) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-bubble) when a wider
canvas is needed.

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

- Bubble
- BubbleAvatar
- BubbleContent
- BubbleMeta
- BubbleActions

## Accessibility

- Keep sender, timestamp, and delivery state available to assistive tech.
- Do not trap focus inside live message regions.
- Announce new messages only when consumer opts in.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can distinguish sender, message body, and delivery context in a conversation.
