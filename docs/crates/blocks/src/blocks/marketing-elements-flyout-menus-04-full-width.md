# Full-width

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/marketing/elements/flyout-menus#component-51e7bd07cdcacb755fd1f4ad6fde01ab). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Full-width Leptos block fixture" src="../../../stories/?story=block-marketing-elements-flyout-menus-04-full-width" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Full-width Bevy block fixture" src="../../../ui-bevy-stories/?story=block-marketing-elements-flyout-menus-04-full-width" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-marketing-elements-flyout-menus-04-full-width) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-marketing-elements-flyout-menus-04-full-width) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `147` |
| Slug | `marketing-elements-flyout-menus-04-full-width` |
| Surface | Marketing |
| Family | Flyout Menus |
| Pattern | Menu |
| Layout | Four column |
| Media | No media |
| Chrome | Plain |
| Interaction | Menu |
| Primary UI component | Dropdown Menu |

## Consumer Implementation

Resolve `marketing-elements-flyout-menus-04-full-width` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(147)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
