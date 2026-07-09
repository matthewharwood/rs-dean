# Table

A semantic table for structured rows and columns.

## Live Fixtures

The embedded Leptos surface renders pre-filled DOM fixtures for this
component's variants, states, themed rendering, and validation paths. The Bevy
surface renders the same shared `rs-dean-ui` component contract through its
Bevy primitive adapter. Both frames use isolated story routes so this page only
shows Table examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos DOM Story</h3>
    <iframe title="Table Leptos live story fixtures" src="../../../stories/?story=ui-table" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Primitive Story</h3>
    <iframe title="Table Bevy primitive story fixtures" src="../../../ui-bevy-stories/?story=ui-table" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos stories page](../../../stories/#ui-table) or the
[full Bevy story page](../../../ui-bevy-stories/?story=ui-table) when a wider
canvas is needed.

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

- Table
- TableHeader
- TableBody
- TableRow
- TableHead
- TableCell
- TableCaption

## Accessibility

- Preserve table, grid, or figure semantics.
- Announce sorting, filtering, and loading changes.
- Keep row and cell labels stable across hydration.

## Consumer Implementation

Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents.

## End User Outcome

Users can scan structured rows and columns with semantic headers.
