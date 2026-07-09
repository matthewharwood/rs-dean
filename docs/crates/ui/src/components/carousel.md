# Carousel

A paged horizontal media or content strip with previous/next controls.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Carousel examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Carousel Leptos live story fixtures" src="../../../stories/?story=ui-carousel" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Carousel Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-carousel" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-carousel) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-carousel) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Display |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Display surface |

## Variants

- default
- compact
- media
- selected

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- Carousel
- CarouselContent
- CarouselItem
- CarouselPrevious
- CarouselNext
- CarouselIndicator

## Accessibility

- Prefer semantic content order over visual order.
- Provide text alternatives for media.
- Keep selected and unavailable states perceivable without color alone.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can browse paged content while position and navigation remain understandable.
