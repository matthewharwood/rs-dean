# 3-column with illustrations and split header

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/ecommerce/components/incentives#component-8872b12a99b750e189f99b92c726ab75). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="3-column with illustrations and split header Leptos block fixture" src="../../../stories/?story=block-ecommerce-components-incentives-01-3-column-with-illustrations-and-split-header" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="3-column with illustrations and split header Bevy block fixture" src="../../../ui-bevy-stories/?story=block-ecommerce-components-incentives-01-3-column-with-illustrations-and-split-header" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-ecommerce-components-incentives-01-3-column-with-illustrations-and-split-header) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-ecommerce-components-incentives-01-3-column-with-illustrations-and-split-header) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `619` |
| Slug | `ecommerce-components-incentives-01-3-column-with-illustrations-and-split-header` |
| Surface | Ecommerce |
| Family | Incentives |
| Pattern | Metrics |
| Layout | Split |
| Media | No media |
| Chrome | Plain |
| Interaction | Static |
| Primary UI component | Badge |

## Consumer Implementation

Resolve `ecommerce-components-incentives-01-3-column-with-illustrations-and-split-header` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(619)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
