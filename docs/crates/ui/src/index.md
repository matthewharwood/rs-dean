# rs-dean-ui

`rs-dean-ui` owns the shared design tokens, semantic themes, shadcn-inspired
component catalog, canonical typed story fixtures, Leptos renderers, and Bevy
primitive adapters.

This book is generated from the Rust catalog. The component pages link back to
the live Leptos and Bevy story harnesses with isolated story routes, so each
page shows only that component's pre-filled DOM fixtures and matching Bevy
primitive adapter used by local component development.

## Pages Structure

- [Marketing app](../../marketing/)
- [Game app](../../game/)
- [Stories app](../../stories/)
- [UI Bevy stories app](../../ui-bevy-stories/)
- [Crate index](../)

## Component Coverage

- Components documented: 64
- Source of truth: `crates/ui/src/catalog.rs`
- Implementation contracts: `crates/ui/src/kit.rs`
- Canonical story fixtures: `crates/ui/src/story_fixtures.rs`
- Leptos live fixtures: `apps/stories/src/main.rs`
- Bevy primitive fixtures: `apps/ui-bevy-stories/src/main.rs`

Run `cargo xtask gen-ui-book` after adding, removing, or renaming a component.
`cargo xtask gate` verifies this book stays in sync with the catalog and story
harnesses.
