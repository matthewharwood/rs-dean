# Simple alert with left-aligned buttons

Original `rs-dean-blocks` fixture corresponding one-to-one with the
[Tailwind Plus source reference](https://tailwindcss.com/plus/ui-blocks/application-ui/overlays/modal-dialogs#component-91a802d0d690e1fab88003f4fe449f0b). No subscription source markup is
copied into this implementation.

## Live Fixtures

Both frames deserialize the same typed fixture, validate it with `garde`, and
derive the same `BlockPlan`. Leptos consumes token-only `rs-dean-ui` components;
Bevy consumes the same palette, spacing, grid, and component roles without a
Leptos dependency.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr)); gap: 1rem; align-items: start;">
  <section>
    <h3>Leptos Block Story</h3>
    <iframe title="Simple alert with left-aligned buttons Leptos block fixture" src="../../../stories/?story=block-application-ui-overlays-modal-dialogs-06-simple-alert-with-left-aligned-buttons" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
  <section>
    <h3>Bevy Block Story</h3>
    <iframe title="Simple alert with left-aligned buttons Bevy block fixture" src="../../../ui-bevy-stories/?story=block-application-ui-overlays-modal-dialogs-06-simple-alert-with-left-aligned-buttons" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>
  </section>
</div>

Open the [full Leptos block story](../../../stories/?story=block-application-ui-overlays-modal-dialogs-06-simple-alert-with-left-aligned-buttons) or the
[full Bevy block story](../../../ui-bevy-stories/?story=block-application-ui-overlays-modal-dialogs-06-simple-alert-with-left-aligned-buttons) for a wider
surface.

## Contract

| Field | Value |
| --- | --- |
| Registry id | `435` |
| Slug | `application-ui-overlays-modal-dialogs-06-simple-alert-with-left-aligned-buttons` |
| Surface | Application UI |
| Family | Modal Dialogs |
| Pattern | Overlay |
| Layout | Overlay |
| Media | No media |
| Chrome | Plain |
| Interaction | Overlay |
| Primary UI component | Dialog |

## Consumer Implementation

Resolve `application-ui-overlays-modal-dialogs-06-simple-alert-with-left-aligned-buttons` through `block_by_slug`, or obtain its stable id with
`BlockId::from_index(435)`. Start from `BlockInstance::fixture` for sample
content, then replace typed content slots. Place instances in `BlockPage.blocks`
in vertical order; schema versions and duplicate keys validate before render.

Transient interaction state remains local to the owning UI component. Persisted
authoring data belongs in `crates/state` through `rs-dean-idb`.
