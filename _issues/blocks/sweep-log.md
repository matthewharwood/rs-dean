# Block Sweep Process Log

The block sweep process is the recursive first-to-current audit used for the
entire `rs-dean-blocks` catalog. After implementing or changing block N, audit
blocks 1 through N for shared schema, layout, token, component API, accessibility,
Leptos, and Bevy learnings. If any earlier block changes, restart at block 1 and
repeat until a complete pass produces no changes before moving to N + 1.

## Current Baseline

- Catalog entries: 657
- Families: 93
- Composition patterns: 26
- Source surfaces: Marketing, Application UI, Ecommerce
- Shared hierarchy: Section -> Container -> Grid/GridItem -> Stack/Cluster
- Data boundary: serde plus garde-validated `BlockPage` and `BlockInstance`
- Renderer contract: one `BlockPlan`, consumed by Leptos and Bevy

## Required Audit Per Sweep

- Registry ids, slugs, family counts, and source anchors remain one-to-one.
- Block content stays original and typed; no subscription source code is copied.
- Layout changes use shared presets and token scales, not per-block CSS.
- Component variants use existing finite `rs-dean-ui` enums.
- Theme switching changes semantic tokens in both renderers without changing data.
- Local interaction state stays in the component; resumable authoring state goes
  through `crates/state` and `rs-dean-idb`.
- Isolated stories render the same fixture and plan in Leptos and Bevy.
- The five-phase pass and one-pass gate remain green.

## Sweep Status

- [x] Catalog and family count audit: 657 / 657 source entries.
- [x] Shared layout, schema, plan, and renderer adapter foundation.
- [x] Isolated route coverage for all 657 entries, representative browser
  parity review, and exhaustive desktop/mobile geometry checks.
- [x] Final no-change recursive sweep across schema, layout, renderers, docs,
  issues, and theme behavior.
