# 3-column with icons

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/ecommerce/components/incentives#component-aeb0ee53368825ebb0be85ae6524cd6c). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="3-column with icons Leptos block fixture" src="../../../stories/?story=block-ecommerce-components-incentives-08-3-column-with-icons" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="3-column with icons Bevy block fixture" src="../../../ui-bevy-stories/?story=block-ecommerce-components-incentives-08-3-column-with-icons" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-ecommerce-components-incentives-08-3-column-with-icons) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-ecommerce-components-incentives-08-3-column-with-icons) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `626` |
| Slug | `ecommerce-components-incentives-08-3-column-with-icons` |
| Surface | Ecommerce |
| Family | Incentives |
| Pattern | Metrics |
| Layout | Stack |
| Media | Icon |
| Chrome | Plain |
| Interaction | Static |
| Primary UI component | Badge |

## Consumer Implementation

Resolve `ecommerce-components-incentives-08-3-column-with-icons` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(626)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
