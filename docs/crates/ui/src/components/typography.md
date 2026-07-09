# Typography

A typographic recipe set for headings, body copy, lists, and code.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Typography examples.

<iframe title="Typography live story fixtures" src="../../../stories/?story=ui-typography" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-typography) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Typography |
| Framework | Shared Spec |
| State | Stateless |
| Render contract | Shared Leptos/Bevy |
| State contract | Stateless |
| Layout contract | Typography flow |

## Variants

- prose
- heading
- inline
- code

## States

- default
- loading
- empty
- disabled
- themed

## Anatomy

- Typography
- TypographyH1
- TypographyH2
- TypographyP
- TypographyList
- TypographyBlockquote

## Accessibility

- Keep heading hierarchy valid.
- Use readable line-height tokens.
- Do not encode meaning through size alone.

## Consumer Implementation

Consumer passes display data; renderer is a pure projection of props, theme, and tokens.

## End User Outcome

Users can read headings, paragraphs, lists, quotes, and code with consistent hierarchy.
