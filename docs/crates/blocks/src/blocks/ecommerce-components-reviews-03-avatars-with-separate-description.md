# Avatars with separate description

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/ecommerce/components/reviews#component-8507b37851ab6df2d36b6ae3f4e7c8cf). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Avatars with separate description Leptos block fixture" src="../../../stories/?story=block-ecommerce-components-reviews-03-avatars-with-separate-description" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Avatars with separate description Bevy block fixture" src="../../../ui-bevy-stories/?story=block-ecommerce-components-reviews-03-avatars-with-separate-description" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-ecommerce-components-reviews-03-avatars-with-separate-description) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-ecommerce-components-reviews-03-avatars-with-separate-description) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `609` |
| Slug | `ecommerce-components-reviews-03-avatars-with-separate-description` |
| Surface | Ecommerce |
| Family | Reviews |
| Pattern | Testimonial |
| Layout | Stack |
| Media | Avatar |
| Chrome | Plain |
| Interaction | Static |
| Primary UI component | Card |

## Consumer Implementation

Resolve `ecommerce-components-reviews-03-avatars-with-separate-description` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(609)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
