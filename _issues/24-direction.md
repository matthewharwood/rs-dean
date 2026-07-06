# Direction

Source: https://ui.shadcn.com/docs/components/direction

## TPM Outcome

Ship a Rust-first `Direction` component that follows the shadcn composition model while using `rs-dean-ui` design tokens, shared component specs, and local-first state boundaries. A direction provider for left-to-right and right-to-left composition.

## Design Considerations

- Use only the shared Tailwind token vocabulary for color, type, space, radius, shadow, and motion.
- Preserve shadcn's compositional anatomy: DirectionProvider, DirectionScope, DirectionAwareContent.
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

- `Direction` appears in the story gallery and `_issues` index.
- The shared spec exposes all anatomy parts listed above.
- Leptos output uses only `rs-dean-ui` token utilities for design scales.
- Bevy adapter can derive scene primitives from the same component spec.
- `cargo xtask five-phase-pass` passes.

## Implementation Status

- [x] Shared Rust implementation recipe.
- [x] Token-only Leptos rendering path.
- [x] Bevy primitive derivation path.
- [x] Sweep review complete through the current implemented catalog.
