# Narrow with hidden labels

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/application-ui/data-display/description-lists#component-abf967b89e0df5cf9648be5a30d7660e). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Narrow with hidden labels Leptos block fixture" src="../../../stories/?story=block-application-ui-data-display-description-lists-06-narrow-with-hidden-labels" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Narrow with hidden labels Bevy block fixture" src="../../../ui-bevy-stories/?story=block-application-ui-data-display-description-lists-06-narrow-with-hidden-labels" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-application-ui-data-display-description-lists-06-narrow-with-hidden-labels) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-application-ui-data-display-description-lists-06-narrow-with-hidden-labels) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `232` |
| Slug | `application-ui-data-display-description-lists-06-narrow-with-hidden-labels` |
| Surface | Application UI |
| Family | Description Lists |
| Pattern | Data display |
| Layout | Centered narrow |
| Media | No media |
| Chrome | Plain |
| Interaction | Static |
| Primary UI component | Item |

## Consumer Implementation

Resolve `application-ui-data-display-description-lists-06-narrow-with-hidden-labels` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(232)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
