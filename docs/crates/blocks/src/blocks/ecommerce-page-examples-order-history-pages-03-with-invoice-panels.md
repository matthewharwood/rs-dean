# With invoice panels

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/ecommerce/page-examples/order-history-pages#component-036712a88c6db4f5399b6387356e119a). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="With invoice panels Leptos block fixture" src="../../../stories/?story=block-ecommerce-page-examples-order-history-pages-03-with-invoice-panels" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="With invoice panels Bevy block fixture" src="../../../ui-bevy-stories/?story=block-ecommerce-page-examples-order-history-pages-03-with-invoice-panels" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-ecommerce-page-examples-order-history-pages-03-with-invoice-panels) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-ecommerce-page-examples-order-history-pages-03-with-invoice-panels) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `654` |
| Slug | `ecommerce-page-examples-order-history-pages-03-with-invoice-panels` |
| Surface | Ecommerce |
| Family | Order History Pages |
| Pattern | Page composition |
| Layout | Page |
| Media | No media |
| Chrome | Panel |
| Interaction | Actions |
| Primary UI component | Table |

## Consumer Implementation

Resolve `ecommerce-page-examples-order-history-pages-03-with-invoice-panels` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(654)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
