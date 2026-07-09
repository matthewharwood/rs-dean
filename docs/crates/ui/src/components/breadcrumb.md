# Breadcrumb

A hierarchical location trail for route and document navigation.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Breadcrumb examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Breadcrumb Leptos live story fixtures" src="../../../stories/?story=ui-breadcrumb" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Breadcrumb Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-breadcrumb" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-breadcrumb) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-breadcrumb) when a wider
canvas is needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Navigation |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Navigation region |

## Variants

- default
- active
- collapsed
- overflow

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- Breadcrumb
- BreadcrumbList
- BreadcrumbItem
- BreadcrumbLink
- BreadcrumbSeparator
- BreadcrumbPage

## Accessibility

- Use navigation landmarks where appropriate.
- Expose current page or active item.
- Keep roving focus predictable in composite menus.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can understand location and move to an ancestor level predictably.
