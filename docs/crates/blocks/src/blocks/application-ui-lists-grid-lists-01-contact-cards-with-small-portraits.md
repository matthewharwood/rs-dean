# Contact cards with small portraits

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/application-ui/lists/grid-lists#component-ce021f02f586e0e6a5f8de2ca2ee537b). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Contact cards with small portraits Leptos block fixture" src="../../../stories/?story=block-application-ui-lists-grid-lists-01-contact-cards-with-small-portraits" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Contact cards with small portraits Bevy block fixture" src="../../../ui-bevy-stories/?story=block-application-ui-lists-grid-lists-01-contact-cards-with-small-portraits" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-application-ui-lists-grid-lists-01-contact-cards-with-small-portraits) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-application-ui-lists-grid-lists-01-contact-cards-with-small-portraits) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `280` |
| Slug | `application-ui-lists-grid-lists-01-contact-cards-with-small-portraits` |
| Surface | Application UI |
| Family | Grid Lists |
| Pattern | Collection |
| Layout | Three column |
| Media | No media |
| Chrome | Panel |
| Interaction | Static |
| Primary UI component | Card |

## Consumer Implementation

Resolve `application-ui-lists-grid-lists-01-contact-cards-with-small-portraits` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(280)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
