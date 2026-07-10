# With title and pill actions

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/application-ui/forms/textareas#component-c8189b6993ca18e9955b370f741b763b). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="With title and pill actions Leptos block fixture" src="../../../stories/?story=block-application-ui-forms-textareas-04-with-title-and-pill-actions" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="With title and pill actions Bevy block fixture" src="../../../ui-bevy-stories/?story=block-application-ui-forms-textareas-04-with-title-and-pill-actions" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-application-ui-forms-textareas-04-with-title-and-pill-actions) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-application-ui-forms-textareas-04-with-title-and-pill-actions) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `329` |
| Slug | `application-ui-forms-textareas-04-with-title-and-pill-actions` |
| Surface | Application UI |
| Family | Textareas |
| Pattern | Form |
| Layout | Stack |
| Media | No media |
| Chrome | Plain |
| Interaction | Form |
| Primary UI component | Textarea |

## Consumer Implementation

Resolve `application-ui-forms-textareas-04-with-title-and-pill-actions` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(329)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
