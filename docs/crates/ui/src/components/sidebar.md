# Sidebar

A persistent or collapsible app navigation rail.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Sidebar examples.

<iframe title="Sidebar live story fixtures" src="../../../stories/?story=ui-sidebar" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-sidebar) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Navigation |
| Framework | Shared Spec |
| State | Consumer Durable |
| Render contract | Shared Leptos/Bevy |
| State contract | Consumer durable |
| Layout contract | Navigation region |

## Variants

- default
- active
- collapsed
- overflow

## States

- controlled
- pending persistence
- hydrated
- invalid boundary input
- disabled

## Anatomy

- Sidebar
- SidebarHeader
- SidebarContent
- SidebarGroup
- SidebarMenu
- SidebarFooter
- SidebarRail

## Accessibility

- Use navigation landmarks where appropriate.
- Expose current page or active item.
- Keep roving focus predictable in composite menus.

## Consumer Implementation

Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents.

## End User Outcome

Users can navigate persistent app structure and collapse state without losing context.
