# Simple dark

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/application-ui/navigation/navbars#component-1f0fd3139f66c75457637ca3746e3384). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Simple dark Leptos block fixture" src="../../../stories/?story=block-application-ui-navigation-navbars-03-simple-dark" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Simple dark Bevy block fixture" src="../../../ui-bevy-stories/?story=block-application-ui-navigation-navbars-03-simple-dark" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-application-ui-navigation-navbars-03-simple-dark) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-application-ui-navigation-navbars-03-simple-dark) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `378` |
| Slug | `application-ui-navigation-navbars-03-simple-dark` |
| Surface | Application UI |
| Family | Navbars |
| Pattern | Navigation |
| Layout | Stack |
| Media | No media |
| Chrome | Dark |
| Interaction | Menu |
| Primary UI component | Navigation Menu |

## Consumer Implementation

Resolve `application-ui-navigation-navbars-03-simple-dark` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(378)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
