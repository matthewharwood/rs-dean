# Card

A framed content surface with header, body, and footer slots.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Card live story fixtures" src="../../../stories/#ui-card" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-card) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Display |
| Framework | Shared Spec |
| State | Stateless |
| Render contract | Shared Leptos/Bevy |
| State contract | Stateless |
| Layout contract | Display surface |

## Variants

- default
- compact
- media
- selected

## States

- default
- loading
- empty
- disabled
- themed

## Anatomy

- Card
- CardHeader
- CardTitle
- CardDescription
- CardContent
- CardFooter

## Accessibility

- Prefer semantic content order over visual order.
- Provide text alternatives for media.
- Keep selected and unavailable states perceivable without color alone.

## Consumer Implementation

Consumer passes display data; renderer is a pure projection of props, theme, and tokens.

## End User Outcome

Users can scan a grouped content surface with predictable header, content, and action zones.
