# Chart

A themed data visualization frame with legend, tooltip, and series metadata.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths. The frame uses the
isolated story route so this page only shows Chart examples.

<iframe title="Chart live story fixtures" src="../../../stories/?story=ui-chart" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-chart) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Data |
| Framework | Shared Spec |
| State | Consumer Durable |
| Render contract | Shared Leptos/Bevy |
| State contract | Consumer durable |
| Layout contract | Data surface |

## Variants

- default
- dense
- sortable
- loading
- empty

## States

- controlled
- pending persistence
- hydrated
- invalid boundary input
- disabled

## Anatomy

- ChartContainer
- ChartSeries
- ChartLegend
- ChartTooltip
- ChartAxis

## Accessibility

- Preserve table, grid, or figure semantics.
- Announce sorting, filtering, and loading changes.
- Keep row and cell labels stable across hydration.

## Consumer Implementation

Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents.

## End User Outcome

Users can interpret themed data series, legends, and tooltips without bespoke color assumptions.
