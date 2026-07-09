# Message

A durable chat or activity message with content and metadata.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Message examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Message Leptos live story fixtures" src="../../../stories/?story=ui-message" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Message Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-message" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-message) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-message) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Messaging |
| Framework | Shared Spec |
| State | Consumer Durable |
| Render contract | Shared Leptos/Bevy |
| State contract | Consumer durable |
| Layout contract | Messaging thread |

## Variants

- incoming
- outgoing
- system
- compact

## States

- controlled
- pending persistence
- hydrated
- invalid boundary input
- disabled

## Anatomy

- Message
- MessageHeader
- MessageContent
- MessageFooter
- MessageActions

## Accessibility

- Keep sender, timestamp, and delivery state available to assistive tech.
- Do not trap focus inside live message regions.
- Announce new messages only when consumer opts in.

## Consumer Implementation

Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents.

## End User Outcome

Users can read durable message content with sender, metadata, and actions preserved.
