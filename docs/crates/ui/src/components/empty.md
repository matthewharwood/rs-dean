# Empty

A structured empty state with optional illustration and recovery action.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Empty examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Empty Leptos live story fixtures" src="../../../stories/?story=ui-empty" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Empty Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-empty" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-empty) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-empty) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Feedback |
| Framework | Shared Spec |
| State | Stateless |
| Render contract | Shared Leptos/Bevy |
| State contract | Stateless |
| Layout contract | Feedback callout |

## Variants

- default
- success
- warning
- danger
- loading

## States

- default
- loading
- empty
- disabled
- themed

## Anatomy

- Empty
- EmptyHeader
- EmptyTitle
- EmptyDescription
- EmptyContent
- EmptyAction

## Accessibility

- Use status or alert semantics based on urgency.
- Keep recovery actions reachable by keyboard.
- Avoid motion-only status communication.

## Consumer Implementation

Consumer passes display data; renderer is a pure projection of props, theme, and tokens.

## End User Outcome

Users can understand why content is absent and what action can recover the flow.
