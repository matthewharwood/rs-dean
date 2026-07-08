# Date Picker

A composed date input, popover, and calendar selection control.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Date Picker live story fixtures" src="../../../stories/#ui-date-picker" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-date-picker) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Form |
| Framework | Shared Spec |
| State | Consumer Durable |
| Render contract | Shared Leptos/Bevy |
| State contract | Consumer durable |
| Layout contract | Form control |

## Variants

- default
- filled
- invalid
- disabled
- readonly

## States

- controlled
- pending persistence
- hydrated
- invalid boundary input
- disabled

## Anatomy

- DatePicker
- DatePickerTrigger
- DatePickerPopover
- DatePickerCalendar
- DatePickerValue

## Accessibility

- Bind labels, descriptions, and errors to controls.
- Expose invalid and required state explicitly.
- Do not persist field values inside renderer-local state.

## Consumer Implementation

Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents.

## End User Outcome

Users can open a calendar from a field, pick a date, and return the chosen value to the consumer.
