# Left-aligned in card

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/application-ui/data-display/description-lists#component-e1b5917b21bbe76a73a96c5ca876225f). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Left-aligned in card Leptos block fixture" src="../../../stories/?story=block-application-ui-data-display-description-lists-02-left-aligned-in-card" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Left-aligned in card Bevy block fixture" src="../../../ui-bevy-stories/?story=block-application-ui-data-display-description-lists-02-left-aligned-in-card" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-application-ui-data-display-description-lists-02-left-aligned-in-card) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-application-ui-data-display-description-lists-02-left-aligned-in-card) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `228` |
| Slug | `application-ui-data-display-description-lists-02-left-aligned-in-card` |
| Surface | Application UI |
| Family | Description Lists |
| Pattern | Data display |
| Layout | Three column |
| Media | No media |
| Chrome | Panel |
| Interaction | Static |
| Primary UI component | Item |

## Consumer Implementation

Resolve `application-ui-data-display-description-lists-02-left-aligned-in-card` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(228)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
