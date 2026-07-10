# With icons in mobile menu

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/marketing/elements/headers#component-3b5e12493074827bcd9d77db75787d78). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="With icons in mobile menu Leptos block fixture" src="../../../stories/?story=block-marketing-elements-headers-08-with-icons-in-mobile-menu" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="With icons in mobile menu Bevy block fixture" src="../../../ui-bevy-stories/?story=block-marketing-elements-headers-08-with-icons-in-mobile-menu" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-marketing-elements-headers-08-with-icons-in-mobile-menu) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-marketing-elements-headers-08-with-icons-in-mobile-menu) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `140` |
| Slug | `marketing-elements-headers-08-with-icons-in-mobile-menu` |
| Surface | Marketing |
| Family | Headers |
| Pattern | Navigation |
| Layout | Split |
| Media | Icon |
| Chrome | Plain |
| Interaction | Menu |
| Primary UI component | Item |

## Consumer Implementation

Resolve `marketing-elements-headers-08-with-icons-in-mobile-menu` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(140)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
