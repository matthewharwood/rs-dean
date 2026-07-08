# Carousel

A paged horizontal media or content strip with previous/next controls.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Carousel live story fixtures" src="../../../stories/#ui-carousel" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-carousel) when a wider canvas is
needed.

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
