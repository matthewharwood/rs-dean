# Resizable

A split panel layout with draggable handles.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Resizable live story fixtures" src="../../../stories/#ui-resizable" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-resizable) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Layout |
| Framework | Shared Spec |
| State | Consumer Durable |
| Render contract | Shared Leptos/Bevy |
| State contract | Consumer durable |
| Layout contract | Layout frame |

## Variants

- default
- horizontal
- vertical
- contained

## States

- controlled
- pending persistence
- hydrated
- invalid boundary input
- disabled

## Anatomy

- ResizablePanelGroup
- ResizablePanel
- ResizableHandle

## Accessibility

- Preserve source order when visual layout changes.
- Keep handles and scroll affordances keyboard reachable.
- Avoid layout shift when content hydrates.

## Consumer Implementation

Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents.

## End User Outcome

Users can adjust panel proportions while the consumer decides whether to persist layout.
