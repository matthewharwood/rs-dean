# List with inline description

Source: https://tailwindcss.com/plus/ui-blocks/application-ui/forms/radio-groups#component-0f35a14dd9e3a48fa7b5fbe43874f079

Catalog path: Application UI / forms / Radio Groups / variant 4 of 12

## TPM Outcome

Ship `application-ui-forms-radio-groups-04-list-with-inline-description` as a registered Rust-first block in the Radio Groups family. Preserve
the source block's Stack composition intent while using original fixture
content, `rs-dean-ui` components, and the shared token vocabulary. The outcome
must render from one validated block instance in Leptos and Bevy.

## Design Considerations

- Compose only `rs-dean-ui` components and layout primitives; do not copy or
  embed Tailwind Plus source markup.
- Use the shared Section -> Container -> Grid/GridItem -> Stack/Cluster box
  model with the `Stack` preset.
- Apply the typed `No media` media mode, `Plain` surface treatment, and
  `Form` interaction contract without adding one-off classes.
- Use only token spacing, typography, color, radius, shadow, and motion classes.
- Keep component choices finite and typed. Button variant and size changes must
  use the existing `ButtonVariant` and `ButtonSize` enums.
- Preserve responsive hierarchy, readable line length, stable media geometry,
  keyboard access, reduced motion, and every theme's semantic contrast.

## Engineering Considerations

- Block index `334` and `application-ui-forms-radio-groups-04-list-with-inline-description` are the stable registry identity.
- The canonical source is `crates/blocks`; renderer-specific code consumes the
  same `BlockPlan` and must not fork content or layout rules.
- Validate serialized instances with `garde` before catalog lookup or rendering.
- The family maps to the `Form` composition pattern and the `Radio Group`
  primary UI component.
- Variant behavior is constrained by `No media`, `Plain`, and `Form`
  enums shared by the Leptos and Bevy plans.
- Keep transient interaction state local to the owning UI component. Persisted
  authoring data belongs in `crates/state` through `rs-dean-idb`.

## Consumer Implementation

- Resolve `BlockId::from_index(334)`, or look up `application-ui-forms-radio-groups-04-list-with-inline-description` with
  `block_by_slug`, then construct the instance with `BlockInstance::new`.
  The catalog fixture is an editable starting point for sample content.
- Override typed `BlockContent` slots for eyebrow, title, body, media label,
  actions, and repeated items without changing the layout contract.
- Register vertical pages as ordered `BlockPage.blocks`; duplicate instance keys
  and unsupported schema versions must fail validation.
- Treat emitted component intents as application events. The block remains a
  composition and does not become an alternate state store.

## Acceptance Criteria

- The registry resolves both block id `334` and slug `application-ui-forms-radio-groups-04-list-with-inline-description` one-to-one.
- The default fixture and shared plan pass `garde` validation.
- Leptos uses only `rs-dean-ui` components and token utilities.
- Bevy derives primitives from the same plan, component id, palette, spacing,
  and layout preset without depending on Leptos.
- An isolated `block-application-ui-forms-radio-groups-04-list-with-inline-description` story proves the fixture in both renderers.
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
