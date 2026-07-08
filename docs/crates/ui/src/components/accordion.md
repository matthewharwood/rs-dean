# Accordion

A vertically stacked disclosure set for expanding one or more sections.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Accordion live story fixtures" src="../../../stories/#ui-accordion" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-accordion) when a wider canvas is
needed.

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

- Accordion
- AccordionItem
- AccordionTrigger
- AccordionContent

## Accessibility

- Connect trigger and content with aria-controls.
- Expose expanded state from renderer-local state.
- Keep keyboard activation on Enter and Space.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users can scan headings and reveal the exact section they need without leaving context.
