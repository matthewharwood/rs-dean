# Contained in panel

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/marketing/sections/feature-sections#component-a17f8a661b0c18a30ef74897cdcdd168). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Contained in panel Leptos block fixture" src="../../../stories/?story=block-marketing-sections-feature-sections-07-contained-in-panel" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Contained in panel Bevy block fixture" src="../../../ui-bevy-stories/?story=block-marketing-sections-feature-sections-07-contained-in-panel" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-marketing-sections-feature-sections-07-contained-in-panel) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-marketing-sections-feature-sections-07-contained-in-panel) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `18` |
| Slug | `marketing-sections-feature-sections-07-contained-in-panel` |
| Surface | Marketing |
| Family | Feature Sections |
| Pattern | Feature |
| Layout | Split |
| Media | No media |
| Chrome | Panel |
| Interaction | Actions |
| Primary UI component | Card |

## Consumer Implementation

Resolve `marketing-sections-feature-sections-07-contained-in-panel` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(18)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
