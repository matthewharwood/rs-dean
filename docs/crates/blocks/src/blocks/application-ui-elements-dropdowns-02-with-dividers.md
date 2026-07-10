# With dividers

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/application-ui/elements/dropdowns#component-1f10562f6266c9b3ef15ff792edd508e). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="With dividers Leptos block fixture" src="../../../stories/?story=block-application-ui-elements-dropdowns-02-with-dividers" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="With dividers Bevy block fixture" src="../../../ui-bevy-stories/?story=block-application-ui-elements-dropdowns-02-with-dividers" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-application-ui-elements-dropdowns-02-with-dividers) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-application-ui-elements-dropdowns-02-with-dividers) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `482` |
| Slug | `application-ui-elements-dropdowns-02-with-dividers` |
| Surface | Application UI |
| Family | Dropdowns |
| Pattern | Menu |
| Layout | Stack |
| Media | No media |
| Chrome | Bordered |
| Interaction | Menu |
| Primary UI component | Dropdown Menu |

## Consumer Implementation

Resolve `application-ui-elements-dropdowns-02-with-dividers` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(482)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
