# Calendar

Source: https://ui.shadcn.com/docs/components/calendar

## TPM Outcome

Ship a Rust-first `Calendar` component that follows the shadcn composition model while using `rs-dean-ui` design tokens, shared component specs, and local-first state boundaries. A date grid for single-date and range selection.

## Design Considerations

- Use only the shared Tailwind token vocabulary for color, type, space, radius, shadow, and motion.
- Preserve shadcn's compositional anatomy: Calendar, CalendarHeader, CalendarGrid, CalendarDay, CalendarRange.
- Keep focus, hover, disabled, loading, and empty states visually distinct across every theme.
- Respect reduced-motion and avoid layout shift when content, loading state, or validation changes.

## Engineering Considerations

- The canonical contract lives in `crates/ui` as Rust data, not framework-specific markup.
- Leptos renders the DOM version from the shared spec. Bevy consumes the same spec through Bevy primitives or a specialized scene adapter when needed.
- Avoid adding dependencies unless a browser standard or proven interaction primitive requires one.
- Keep persistent user choices out of component-local state; route resumable state through `crates/state` and `rs-dean-idb`.

## Consumer Implementation

- Consumers pass typed props or controlled state handles and subscribe to emitted intents.
- Stateless displays render directly from props. Ephemeral interactions stay renderer-local. Durable selections, filters, dismissed messages, and table state must be owned by the app/state layer.
- Stories must cover default, dense, disabled, invalid/error, loading, and themed variants where applicable.

## Acceptance Criteria

- `Calendar` appears in the story gallery and `_issues` index.
- The shared spec exposes all anatomy parts listed above.
- Leptos output uses only `rs-dean-ui` token utilities for design scales.
- Bevy adapter can derive scene primitives from the same literal widget slots.
- `cargo xtask five-phase-pass` passes.

## Implementation Status

- [x] Shared Rust implementation recipe.
- [x] Literal Rust widget constructor exported from `rs-dean-ui`.
- [x] Named token-only Leptos component exported for consumer code.
- [x] Bevy primitive derivation path consumes the same widget slots and typed intents.
- [x] Shared widget validation/render-node contract covers the catalog entry, with concrete renderers kept in sync where specialized.
- [x] Concrete typed date/range model, selection mode, part enum, repeatable day render nodes, validation, local month/selection state, and named token-only Leptos component implemented.
- [x] Dedicated story coverage includes single-date, range, loading, disabled, invalid, and theme-scoped examples.
- [x] Sweep review complete through the current implemented catalog.
