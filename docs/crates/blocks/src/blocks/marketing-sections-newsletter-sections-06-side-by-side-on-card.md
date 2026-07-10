# Side-by-side on card

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/marketing/sections/newsletter-sections#component-545e4f870cc7bc80a882f6ea2f33fb9f). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Side-by-side on card Leptos block fixture" src="../../../stories/?story=block-marketing-sections-newsletter-sections-06-side-by-side-on-card" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Side-by-side on card Bevy block fixture" src="../../../ui-bevy-stories/?story=block-marketing-sections-newsletter-sections-06-side-by-side-on-card" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-marketing-sections-newsletter-sections-06-side-by-side-on-card) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-marketing-sections-newsletter-sections-06-side-by-side-on-card) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `66` |
| Slug | `marketing-sections-newsletter-sections-06-side-by-side-on-card` |
| Surface | Marketing |
| Family | Newsletter Sections |
| Pattern | Form |
| Layout | Stack |
| Media | No media |
| Chrome | Panel |
| Interaction | Form |
| Primary UI component | Input |

## Consumer Implementation

Resolve `marketing-sections-newsletter-sections-06-side-by-side-on-card` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(66)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
