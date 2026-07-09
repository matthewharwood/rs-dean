# Aspect Ratio

A fixed-ratio media frame that keeps images, canvases, and embeds stable.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Aspect Ratio examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Aspect Ratio Leptos live story fixtures" src="../../../stories/?story=ui-aspect-ratio" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Aspect Ratio Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-aspect-ratio" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-aspect-ratio) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-aspect-ratio) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Layout |
| Framework | Shared Spec |
| State | Stateless |
| Render contract | Shared Leptos/Bevy |
| State contract | Stateless |
| Layout contract | Layout frame |

## Variants

- default
- horizontal
- vertical
- contained

## States

- default
- loading
- empty
- disabled
- themed

## Anatomy

- AspectRatio
- AspectRatioFrame
- AspectRatioMedia

## Accessibility

- Preserve source order when visual layout changes.
- Keep handles and scroll affordances keyboard reachable.
- Avoid layout shift when content hydrates.

## Consumer Implementation

Consumer passes display data; renderer is a pure projection of props, theme, and tokens.

## End User Outcome

Users see media and canvas content in a stable frame before and after hydration.
