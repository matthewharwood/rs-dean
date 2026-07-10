# With image grid

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/ecommerce/components/product-features#component-6bf262ecea63105e5f1fc57ec12057f9). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="With image grid Leptos block fixture" src="../../../stories/?story=block-ecommerce-components-product-features-03-with-image-grid" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="With image grid Bevy block fixture" src="../../../ui-bevy-stories/?story=block-ecommerce-components-product-features-03-with-image-grid" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-ecommerce-components-product-features-03-with-image-grid) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-ecommerce-components-product-features-03-with-image-grid) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `582` |
| Slug | `ecommerce-components-product-features-03-with-image-grid` |
| Surface | Ecommerce |
| Family | Product Features |
| Pattern | Commerce feature |
| Layout | Split |
| Media | Image |
| Chrome | Plain |
| Interaction | Actions |
| Primary UI component | Card |

## Consumer Implementation

Resolve `ecommerce-components-product-features-03-with-image-grid` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(582)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
