# Scroll Area

A custom scroll viewport with tokenized scrollbars and fade affordances.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Scroll Area examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Scroll Area Leptos live story fixtures" src="../../../stories/?story=ui-scroll-area" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Scroll Area Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-scroll-area" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-scroll-area) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-scroll-area) when a wider
canvas is needed.

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
