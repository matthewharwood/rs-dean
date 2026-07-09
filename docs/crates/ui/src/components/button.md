# Button

The primary action primitive for commands, links, and submit actions.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Button examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Button Leptos live story fixtures" src="../../../stories/?story=ui-button" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Button Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-button" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-button) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-button) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Action |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Action row |

## Variants

- primary
- secondary
- quiet
- destructive
- icon

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- Button
- ButtonIcon
- ButtonLabel

## Accessibility

- Use semantic buttons or links.
- Expose disabled and pressed state to assistive tech.
- Keep visible focus from the focus-ring token.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can trigger a clear command with strong focus, disabled, loading, and destructive states.
