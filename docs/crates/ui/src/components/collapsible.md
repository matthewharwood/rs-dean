# Collapsible

A single disclosure region that can hide or reveal content.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Collapsible examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Collapsible Leptos live story fixtures" src="../../../stories/?story=ui-collapsible" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Collapsible Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-collapsible" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-collapsible) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-collapsible) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Disclosure |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Disclosure panel |

## Variants

- single
- multiple
- compact
- disabled

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- Collapsible
- CollapsibleTrigger
- CollapsibleContent

## Accessibility

- Connect trigger and content with aria-controls.
- Expose expanded state from renderer-local state.
- Keep keyboard activation on Enter and Space.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can show or hide a single region while preserving focus and layout stability.
