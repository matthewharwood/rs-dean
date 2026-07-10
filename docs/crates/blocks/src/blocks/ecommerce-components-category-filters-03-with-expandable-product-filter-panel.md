# With expandable product filter panel

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/ecommerce/components/category-filters#component-8dbbe78f722647bcb452fab9ba8a3b9a). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="With expandable product filter panel Leptos block fixture" src="../../../stories/?story=block-ecommerce-components-category-filters-03-with-expandable-product-filter-panel" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="With expandable product filter panel Bevy block fixture" src="../../../ui-bevy-stories/?story=block-ecommerce-components-category-filters-03-with-expandable-product-filter-panel" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-ecommerce-components-category-filters-03-with-expandable-product-filter-panel) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-ecommerce-components-category-filters-03-with-expandable-product-filter-panel) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `573` |
| Slug | `ecommerce-components-category-filters-03-with-expandable-product-filter-panel` |
| Surface | Ecommerce |
| Family | Category Filters |
| Pattern | Form |
| Layout | Split |
| Media | Product media |
| Chrome | Panel |
| Interaction | Filter |
| Primary UI component | Checkbox |

## Consumer Implementation

Resolve `ecommerce-components-category-filters-03-with-expandable-product-filter-panel` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(573)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
