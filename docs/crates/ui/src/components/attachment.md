# Attachment

A file or resource chip shown inside message and compose surfaces.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Attachment examples.

<iframe title="Attachment live story fixtures" src="../../../stories/?story=ui-attachment" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-attachment) when a wider canvas is
needed.

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

- Attachment
- AttachmentPreview
- AttachmentTitle
- AttachmentMeta
- AttachmentAction

## Accessibility

- Keep sender, timestamp, and delivery state available to assistive tech.
- Do not trap focus inside live message regions.
- Announce new messages only when consumer opts in.

## Consumer Implementation

Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents.

## End User Outcome

Users can inspect attached files or resources with clear name, type, size, and action affordances.
