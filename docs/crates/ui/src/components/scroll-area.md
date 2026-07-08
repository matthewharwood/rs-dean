# Scroll Area

A custom scroll viewport with tokenized scrollbars and fade affordances.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Scroll Area live story fixtures" src="../../../stories/#ui-scroll-area" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-scroll-area) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Layout |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Layout frame |

## Variants

- default
- horizontal
- vertical
- contained

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- ScrollArea
- ScrollViewport
- ScrollContent
- ScrollBar
- ScrollCorner

## Accessibility

- Preserve source order when visual layout changes.
- Keep handles and scroll affordances keyboard reachable.
- Avoid layout shift when content hydrates.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can scroll contained content with visible overflow affordances.
