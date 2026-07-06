# Checkbox

Source: https://ui.shadcn.com/docs/components/checkbox

## TPM Outcome

Ship a Rust-first `Checkbox` component that follows the shadcn composition model while using `rs-dean-ui` design tokens, shared component specs, and local-first state boundaries. A binary or indeterminate form control.

## Design Considerations

- Use only the shared Tailwind token vocabulary for color, type, space, radius, shadow, and motion.
- Preserve shadcn's compositional anatomy: Checkbox, CheckboxIndicator, CheckboxLabel, CheckboxDescription.
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

- `Checkbox` appears in the story gallery and `_issues` index.
- The shared spec exposes all anatomy parts listed above.
- Leptos output uses only `rs-dean-ui` token utilities for design scales.
- Bevy adapter can derive scene primitives from the same component spec.
- `cargo xtask five-phase-pass` passes.
