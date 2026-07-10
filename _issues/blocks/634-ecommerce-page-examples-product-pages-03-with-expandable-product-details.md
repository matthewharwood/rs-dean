# With expandable product details

Source: https://tailwindcss.com/plus/ui-blocks/ecommerce/page-examples/product-pages#component-bed7a8c94a54fcd9e9b214b95b71d5d5

Catalog path: Ecommerce / page-examples / Product Pages / variant 3 of 5

## TPM Outcome

Ship `ecommerce-page-examples-product-pages-03-with-expandable-product-details` as a registered Rust-first block in the Product Pages family. Preserve
the source block's Page composition intent while using original fixture
content, `rs-dean-ui` components, and the shared token vocabulary. The outcome
must render from one validated block instance in Leptos and Bevy.

## Design Considerations

- Compose only `rs-dean-ui` components and layout primitives; do not copy or
  embed Tailwind Plus source markup.
- Use the shared Section -> Container -> Grid/GridItem -> Stack/Cluster box
  model with the `Page` preset.
- Apply the typed `Product media` media mode, `Plain` surface treatment, and
  `Actions` interaction contract without adding one-off classes.
- Use only token spacing, typography, color, radius, shadow, and motion classes.
- Keep component choices finite and typed. Button variant and size changes must
  use the existing `ButtonVariant` and `ButtonSize` enums.
- Preserve responsive hierarchy, readable line length, stable media geometry,
  keyboard access, reduced motion, and every theme's semantic contrast.

## Engineering Considerations

- Block index `633` and `ecommerce-page-examples-product-pages-03-with-expandable-product-details` are the stable registry identity.
- The canonical source is `crates/blocks`; renderer-specific code consumes the
  same `BlockPlan` and must not fork content or layout rules.
- Validate serialized instances with `garde` before catalog lookup or rendering.
- The family maps to the `Page composition` composition pattern and the `Card`
  primary UI component.
- Variant behavior is constrained by `Product media`, `Plain`, and `Actions`
  enums shared by the Leptos and Bevy plans.
- Keep transient interaction state local to the owning UI component. Persisted
  authoring data belongs in `crates/state` through `rs-dean-idb`.

## Consumer Implementation

- Resolve `BlockId::from_index(633)`, or look up `ecommerce-page-examples-product-pages-03-with-expandable-product-details` with
  `block_by_slug`, then construct the instance with `BlockInstance::new`.
  The catalog fixture is an editable starting point for sample content.
- Override typed `BlockContent` slots for eyebrow, title, body, media label,
  actions, and repeated items without changing the layout contract.
- Register vertical pages as ordered `BlockPage.blocks`; duplicate instance keys
  and unsupported schema versions must fail validation.
- Treat emitted component intents as application events. The block remains a
  composition and does not become an alternate state store.

## Acceptance Criteria

- The registry resolves both block id `633` and slug `ecommerce-page-examples-product-pages-03-with-expandable-product-details` one-to-one.
- The default fixture and shared plan pass `garde` validation.
- Leptos uses only `rs-dean-ui` components and token utilities.
- Bevy derives primitives from the same plan, component id, palette, spacing,
  and layout preset without depending on Leptos.
- An isolated `block-ecommerce-page-examples-product-pages-03-with-expandable-product-details` story proves the fixture in both renderers.
- The sweep process returns to every earlier block after a shared API change.
- `cargo xtask five-phase-pass` passes.

## Implementation Status

- [x] Tailwind Plus source identity and one-to-one catalog entry recorded.
- [x] Typed family, composition pattern, primary UI component, and layout preset.
- [x] Typed media, surface chrome, and interaction contracts.
- [x] Original validated fixture content generated from the shared schema.
- [x] Shared renderer-neutral `BlockPlan` implemented.
- [x] Leptos composition path implemented from `rs-dean-ui` primitives.
- [x] Bevy primitive adapter implemented from the same plan.
- [x] Isolated Leptos and Bevy routes generated; all-catalog behavior and
  geometry checks plus representative browser parity review passed.
- [x] Recursive sweep complete through the full catalog with no remaining
  shared API drift.
