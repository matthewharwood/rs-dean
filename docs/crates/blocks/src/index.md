# rs-dean-blocks

`rs-dean-blocks` is the typed composition layer over `rs-dean-ui`. It owns the
one-to-one block registry, validated authoring schema, constrained layout plans,
and shared Leptos/Bevy renderer inputs.

This book is generated from the Rust catalog. Every page embeds the same
pre-filled `BlockInstance` in the isolated Leptos and Bevy story harnesses.

## Pages Structure

- [Marketing app](../../marketing/)
- [Game app](../../game/)
- [Stories app](../../stories/)
- [UI Bevy stories app](../../ui-bevy-stories/)
- [Crate index](../)
- [rs-dean-ui book](../ui/)

## Catalog Coverage

- Blocks documented: 657
- Families: 93
- Source surfaces: Marketing, Application UI, Ecommerce
- Source of truth: `crates/blocks/src/catalog_data.rs`
- Authoring boundary: serde plus garde-validated `BlockPage` and `BlockInstance`
- Renderer boundary: one `BlockPlan` consumed by Leptos and Bevy

Run `cargo xtask gen-block-book` after catalog, fixture, or story-route changes.
`cargo xtask gate` verifies the book, block issues, and isolated routes remain
in sync.
