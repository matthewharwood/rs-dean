# Separator

A horizontal or vertical divider between related content.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Separator examples.

<iframe title="Separator live story fixtures" src="../../../stories/?story=ui-separator" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-separator) when a wider canvas is
needed.

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

- Separator
- SeparatorLine
- SeparatorLabel

## Accessibility

- Preserve source order when visual layout changes.
- Keep handles and scroll affordances keyboard reachable.
- Avoid layout shift when content hydrates.

## Consumer Implementation

Consumer passes display data; renderer is a pure projection of props, theme, and tokens.

## End User Outcome

Users can distinguish related regions without mistaking decoration for content.
