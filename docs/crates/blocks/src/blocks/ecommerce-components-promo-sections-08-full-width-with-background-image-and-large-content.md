# Full-width with background image and large content

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/ecommerce/components/promo-sections#component-2c7fddc98add76710894f66c325afcd7). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Full-width with background image and large content Leptos block fixture" src="../../../stories/?story=block-ecommerce-components-promo-sections-08-full-width-with-background-image-and-large-content" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Full-width with background image and large content Bevy block fixture" src="../../../ui-bevy-stories/?story=block-ecommerce-components-promo-sections-08-full-width-with-background-image-and-large-content" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-ecommerce-components-promo-sections-08-full-width-with-background-image-and-large-content) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-ecommerce-components-promo-sections-08-full-width-with-background-image-and-large-content) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `601` |
| Slug | `ecommerce-components-promo-sections-08-full-width-with-background-image-and-large-content` |
| Surface | Ecommerce |
| Family | Promo Sections |
| Pattern | Commerce feature |
| Layout | Full width |
| Media | Background image |
| Chrome | Plain |
| Interaction | Actions |
| Primary UI component | Card |

## Consumer Implementation

Resolve `ecommerce-components-promo-sections-08-full-width-with-background-image-and-large-content` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(601)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
