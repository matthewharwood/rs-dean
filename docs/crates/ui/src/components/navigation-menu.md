# Navigation Menu

A responsive top-level navigation system with panels and links.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Navigation Menu live story fixtures" src="../../../stories/#ui-navigation-menu" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-navigation-menu) when a wider canvas is
needed.

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

- NavigationMenu
- NavigationMenuList
- NavigationMenuItem
- NavigationMenuTrigger
- NavigationMenuContent
- NavigationMenuLink

## Accessibility

- Use navigation landmarks where appropriate.
- Expose current page or active item.
- Keep roving focus predictable in composite menus.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can move across top-level sections and reveal grouped navigation content.
