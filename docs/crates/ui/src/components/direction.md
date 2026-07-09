# Direction

A direction provider for left-to-right and right-to-left composition.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Direction examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Direction Leptos live story fixtures" src="../../../stories/?story=ui-direction" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Direction Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-direction" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-direction) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-direction) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Utility |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Utility scope |

## Variants

- ltr
- rtl
- nested

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- DirectionProvider
- DirectionScope
- DirectionAwareContent

## Accessibility

- Apply direction or utility scope without changing semantic order.
- Keep nested scopes explicit.
- Do not hide framework-specific assumptions in utility components.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can render nested left-to-right or right-to-left scopes without duplicating components.
