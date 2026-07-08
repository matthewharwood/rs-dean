# Command

A command palette for searchable actions and route jumps.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Command live story fixtures" src="../../../stories/#ui-command" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-command) when a wider canvas is
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

- Command
- CommandInput
- CommandList
- CommandGroup
- CommandItem
- CommandShortcut

## Accessibility

- Use navigation landmarks where appropriate.
- Expose current page or active item.
- Keep roving focus predictable in composite menus.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can search available commands and execute a selected action with keyboard-first flow.
