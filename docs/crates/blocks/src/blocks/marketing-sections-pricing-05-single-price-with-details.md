# Single price with details

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/marketing/sections/pricing#component-56cbd4f191ac0d54e5a5c0287481d5b9). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Single price with details Leptos block fixture" src="../../../stories/?story=block-marketing-sections-pricing-05-single-price-with-details" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Single price with details Bevy block fixture" src="../../../ui-bevy-stories/?story=block-marketing-sections-pricing-05-single-price-with-details" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-marketing-sections-pricing-05-single-price-with-details) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-marketing-sections-pricing-05-single-price-with-details) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `45` |
| Slug | `marketing-sections-pricing-05-single-price-with-details` |
| Surface | Marketing |
| Family | Pricing Sections |
| Pattern | Pricing |
| Layout | Split |
| Media | No media |
| Chrome | Plain |
| Interaction | Actions |
| Primary UI component | Card |

## Consumer Implementation

Resolve `marketing-sections-pricing-05-single-price-with-details` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(45)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
