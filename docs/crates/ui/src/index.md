# rs-dean-ui

`rs-dean-ui` owns the shared design tokens, semantic themes, shadcn-inspired
component catalog, Leptos renderers, and Bevy primitive adapters.

This book is generated from the Rust catalog. The component pages link back to
the live stories harness, so each page shows the same pre-filled fixtures used
by local component development.

## Pages Structure

- [Marketing app](../../marketing/)
- [Game app](../../game/)
- [Stories app](../../stories/)
- [Crate index](../)

## Component Coverage

- Components documented: 64
- Source of truth: `crates/ui/src/catalog.rs`
- Implementation contracts: `crates/ui/src/kit.rs`
- Live fixtures: `apps/stories/src/main.rs`

Run `cargo xtask gen-ui-book` after adding, removing, or renaming a component.
`cargo xtask gate` verifies this book stays in sync with the catalog and stories.
