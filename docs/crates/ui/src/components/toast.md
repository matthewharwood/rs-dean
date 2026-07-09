# Toast

A transient notification item with optional action.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Toast examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Toast Leptos live story fixtures" src="../../../stories/?story=ui-toast" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Toast Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-toast" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-toast) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-toast) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Feedback |
| Framework | Leptos Only |
| State | Ephemeral |
| Render contract | Leptos DOM only |
| State contract | Renderer ephemeral |
| Layout contract | Feedback callout |

## Variants

- default
- success
- warning
- danger
- loading

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- ToastProvider
- ToastViewport
- Toast
- ToastTitle
- ToastDescription
- ToastAction

## Accessibility

- Use status or alert semantics based on urgency.
- Keep recovery actions reachable by keyboard.
- Avoid motion-only status communication.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users receive transient feedback with optional action and non-blocking dismissal.
