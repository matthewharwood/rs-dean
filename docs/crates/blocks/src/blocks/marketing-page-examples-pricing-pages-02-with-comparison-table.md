# With comparison table

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/marketing/page-examples/pricing-pages#component-5f3268c0a01e789d4578d1f0f505594e). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="With comparison table Leptos block fixture" src="../../../stories/?story=block-marketing-page-examples-pricing-pages-02-with-comparison-table" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="With comparison table Bevy block fixture" src="../../../ui-bevy-stories/?story=block-marketing-page-examples-pricing-pages-02-with-comparison-table" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-marketing-page-examples-pricing-pages-02-with-comparison-table) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-marketing-page-examples-pricing-pages-02-with-comparison-table) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `174` |
| Slug | `marketing-page-examples-pricing-pages-02-with-comparison-table` |
| Surface | Marketing |
| Family | Pricing Pages |
| Pattern | Page composition |
| Layout | Page |
| Media | No media |
| Chrome | Plain |
| Interaction | Actions |
| Primary UI component | Card |

## Consumer Implementation

Resolve `marketing-page-examples-pricing-pages-02-with-comparison-table` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(174)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
