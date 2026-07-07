use leptos::prelude::*;

use crate::{
    AccordionIntent, AccordionItem, AccordionMode, AccordionModel, AlertDensity, AlertDialogIntent,
    AlertDialogModel, AlertDialogSize, AlertDialogState, AlertModel, AlertTone, AspectRatioFit,
    AspectRatioModel, AspectRatioPart, AttachmentIntent, AttachmentKind, AttachmentModel,
    AttachmentPart, CatalogComponentModel, CatalogComponentPart, CatalogComponentRenderNode,
    ComponentImplementation, ThemeChoice, ThemeId, UiBlock, UiBlockTone, UiComponentId,
    UiWidgetIntent, UiWidgetPattern, UiWidgetSlotKind, accordion_dom_id, alert_dialog_dom_id,
    aspect_ratio_render_nodes, attachment_render_nodes, catalog_component_render_nodes,
    component_implementation, component_spec, default_accordion_items, default_alert_dialog_model,
    default_alert_model, default_aspect_ratio_model, default_attachment_model,
    validate_accordion_model, validate_alert_dialog_model, validate_alert_model,
    validate_aspect_ratio_model, validate_attachment_model,
};

const HEALTH_CARD: &str =
    "rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-2";
const HEALTH_CARD_EYEBROW: &str = "m-0 text-00 font-7 uppercase text-brand";
const HEALTH_CARD_TITLE: &str = "m-0 mt-2xs text-1 font-7 text-text-1";
const HEALTH_CARD_BODY: &str = "m-0 mt-xs text-0 leading-0 text-text-2";
const THEME_BUTTON: &str = "inline-flex items-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const THEME_BUTTON_LABEL: &str = "text-text-muted";
const THEME_SCOPE: &str = "contents";
const COMPONENT_GALLERY: &str = "grid gap-s sm:grid-cols-2 lg:grid-cols-3";
const COMPONENT_CARD: &str = "grid min-w-0 gap-s rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-2";
const COMPONENT_HEADER: &str = "grid gap-2xs";
const COMPONENT_META: &str = "flex flex-wrap items-center gap-2xs";
const COMPONENT_PILL: &str = "rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-6 uppercase tracking-label text-text-muted";
const COMPONENT_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const COMPONENT_SUMMARY: &str = "m-0 text-0 leading-0 text-text-2";
const COMPONENT_BLOCKS: &str = "grid gap-2xs";
const COMPONENT_RECIPE: &str = "grid gap-2xs border-t border-border-subtle pt-s";
const COMPONENT_RECIPE_TITLE: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const COMPONENT_TAG_ROW: &str = "flex flex-wrap gap-2xs";
const COMPONENT_TAG: &str = "rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-2";
const COMPONENT_DEMO: &str = "grid gap-2xs border-t border-border-subtle pt-s";
const BLOCK_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const BLOCK_DETAIL: &str = "m-0 text-0 leading-0 text-text-2";
const WIDGET_SHELL: &str = "grid min-w-0 gap-s rounded-field bg-surface-1 p-s text-text-1";
const WIDGET_HEADER: &str = "grid gap-2xs";
const WIDGET_EYEBROW: &str = "m-0 text-00 font-7 uppercase tracking-label text-brand";
const WIDGET_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const WIDGET_SUMMARY: &str = "m-0 text-0 leading-0 text-text-2";
const WIDGET_CONTENT: &str = "grid gap-2xs";
const WIDGET_CONTENT_INLINE: &str = "flex flex-wrap items-center gap-2xs";
const WIDGET_SLOT: &str =
    "grid gap-3xs rounded-field border border-border-subtle bg-surface-2 p-xs text-text-1";
const WIDGET_SLOT_ACTIVE: &str =
    "grid gap-3xs rounded-field border border-brand bg-primary-soft p-xs text-text-1";
const WIDGET_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const WIDGET_VALUE: &str = "m-0 text-0 leading-0 text-text-2";
const WIDGET_BUTTON: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const WIDGET_BUTTON_ACTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const WIDGET_INPUT: &str = "min-h-field w-full rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 text-text-1 outline-none focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const WIDGET_TEXTAREA: &str = "min-h-2xl w-full resize-y rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const WIDGET_BADGE: &str = "inline-flex items-center gap-2xs rounded-pill border border-border-subtle bg-surface-2 px-xs py-3xs text-00 font-7 text-text-1";
const WIDGET_MEDIA: &str = "grid min-h-xl place-items-center rounded-field border border-border-subtle bg-surface-2 p-s text-0 font-7 text-text-2";
const WIDGET_AVATAR: &str = "grid size-l place-items-center rounded-pill border border-border-subtle bg-primary-soft text-0 font-7 text-text-1";
const WIDGET_MARKER: &str = "inline-flex size-s items-center justify-center rounded-pill bg-brand text-00 font-7 text-text-on-brand";
const WIDGET_KEY: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-muted bg-surface-2 px-2xs py-3xs font-mono text-00 text-text-1 shadow-1";
const WIDGET_SEPARATOR: &str = "min-h-selector rounded-pill bg-border-subtle";
const WIDGET_TABLE: &str =
    "w-full overflow-hidden rounded-field border border-border-subtle text-left text-0 text-text-1";
const WIDGET_TABLE_CELL: &str = "border-t border-border-subtle px-xs py-2xs";
const WIDGET_PROGRESS: &str = "h-xs w-full rounded-pill accent-brand";
const WIDGET_RANGE: &str = "w-full accent-brand";
const WIDGET_SKELETON: &str = "h-s rounded-field bg-surface-3";
const WIDGET_SPINNER: &str = "size-s rounded-pill border border-border-muted border-t-brand";
const WIDGET_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ACCORDION_ROOT: &str =
    "grid w-full gap-2xs rounded-box border border-border-subtle bg-surface-1 p-2xs text-text-1";
const ACCORDION_ITEM: &str = "grid gap-0 rounded-field border border-border-faint bg-surface-1";
const ACCORDION_ITEM_OPEN: &str = "grid gap-0 rounded-field border border-brand bg-primary-soft";
const ACCORDION_TRIGGER: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field px-xs py-2xs text-left text-0 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled";
const ACCORDION_TRIGGER_OPEN: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field bg-selected-tint px-xs py-2xs text-left text-0 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled";
const ACCORDION_CONTENT: &str = "grid gap-2xs px-xs pb-xs text-0 leading-0 text-text-2";
const ACCORDION_INDICATOR: &str =
    "grid size-s place-items-center rounded-pill bg-surface-2 text-00 font-7 text-text-muted";
const ACCORDION_EMPTY: &str =
    "rounded-field border border-border-subtle bg-surface-2 p-s text-0 leading-0 text-text-muted";
const ACCORDION_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ALERT_STANDARD_DEFAULT: &str = "relative grid w-full gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const ALERT_STANDARD_INFO: &str = "relative grid w-full gap-xs rounded-box border border-info bg-info-soft p-s text-text-1 shadow-1";
const ALERT_STANDARD_SUCCESS: &str = "relative grid w-full gap-xs rounded-box border border-success bg-success-soft p-s text-text-1 shadow-1";
const ALERT_STANDARD_WARNING: &str = "relative grid w-full gap-xs rounded-box border border-warning bg-warning-soft p-s text-text-1 shadow-1";
const ALERT_STANDARD_DESTRUCTIVE: &str = "relative grid w-full gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const ALERT_DENSE_DEFAULT: &str = "relative grid w-full gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const ALERT_DENSE_INFO: &str = "relative grid w-full gap-2xs rounded-field border border-info bg-info-soft p-xs text-text-1 shadow-1";
const ALERT_DENSE_SUCCESS: &str = "relative grid w-full gap-2xs rounded-field border border-success bg-success-soft p-xs text-text-1 shadow-1";
const ALERT_DENSE_WARNING: &str = "relative grid w-full gap-2xs rounded-field border border-warning bg-warning-soft p-xs text-text-1 shadow-1";
const ALERT_DENSE_DESTRUCTIVE: &str = "relative grid w-full gap-2xs rounded-field border border-danger bg-error-soft p-xs text-text-1 shadow-1";
const ALERT_ROW: &str = "flex min-w-0 flex-wrap items-start gap-xs";
const ALERT_BODY: &str = "grid min-w-0 flex-1 gap-2xs";
const ALERT_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const ALERT_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const ALERT_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const ALERT_DESCRIPTION_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const ALERT_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_MARKER_DEFAULT: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-surface-2 text-00 font-7 text-text-muted";
const ALERT_MARKER_INFO: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-info text-00 font-7 text-text-on-brand";
const ALERT_MARKER_SUCCESS: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-success text-00 font-7 text-text-on-brand";
const ALERT_MARKER_WARNING: &str =
    "grid size-s shrink-0 place-items-center rounded-pill bg-warning text-00 font-7 text-text-1";
const ALERT_MARKER_DESTRUCTIVE: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-danger text-00 font-7 text-text-on-brand";
const ALERT_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ALERT_DIALOG_ROOT: &str = "grid w-full gap-xs text-text-1";
const ALERT_DIALOG_TRIGGER: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_DIALOG_OVERLAY: &str =
    "grid w-full place-items-center rounded-box bg-surface-overlay p-s";
const ALERT_DIALOG_CONTENT: &str = "grid w-full gap-s rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-3";
const ALERT_DIALOG_CONTENT_SMALL: &str = "grid w-full gap-xs rounded-box border border-border-subtle bg-surface-elevated p-xs text-text-1 shadow-2";
const ALERT_DIALOG_CONTENT_DESTRUCTIVE: &str = "grid w-full gap-s rounded-box border border-danger bg-surface-elevated p-s text-text-1 shadow-3";
const ALERT_DIALOG_CONTENT_SMALL_DESTRUCTIVE: &str = "grid w-full gap-xs rounded-box border border-danger bg-surface-elevated p-xs text-text-1 shadow-2";
const ALERT_DIALOG_HEADER: &str = "grid gap-2xs";
const ALERT_DIALOG_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const ALERT_DIALOG_TITLE_SMALL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const ALERT_DIALOG_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const ALERT_DIALOG_DESCRIPTION_SMALL: &str = "m-0 text-00 leading-0 text-text-2";
const ALERT_DIALOG_FOOTER: &str = "flex flex-wrap items-center justify-end gap-2xs";
const ALERT_DIALOG_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_DIALOG_ACTION_DESTRUCTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-danger bg-error-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-press-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_DIALOG_CANCEL: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_DIALOG_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ASPECT_RATIO_ROOT: &str = "grid w-full gap-2xs text-text-1";
const ASPECT_RATIO_FIGURE: &str = "m-0 grid gap-2xs";
const ASPECT_RATIO_FRAME: &str = "relative grid w-full overflow-hidden rounded-box border border-border-subtle bg-surface-2 p-2xs shadow-1";
const ASPECT_RATIO_MEDIA_COVER: &str =
    "grid h-full w-full place-items-center rounded-field bg-primary-soft p-s text-center";
const ASPECT_RATIO_MEDIA_CONTAIN: &str = "grid h-full w-full place-items-center rounded-field border border-border-faint bg-surface-1 p-s text-center";
const ASPECT_RATIO_MEDIA_LOADING: &str =
    "grid h-full w-full place-items-center rounded-field bg-surface-3 p-s text-center";
const ASPECT_RATIO_MEDIA_DISABLED: &str =
    "grid h-full w-full place-items-center rounded-field bg-surface-2 p-s text-center";
const ASPECT_RATIO_MEDIA_STACK: &str = "grid gap-2xs";
const ASPECT_RATIO_MEDIA_MARKER: &str = "mx-auto grid size-xl place-items-center rounded-field border border-border-subtle bg-surface-elevated text-1 font-7 text-brand shadow-1";
const ASPECT_RATIO_MEDIA_LABEL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const ASPECT_RATIO_MEDIA_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const ASPECT_RATIO_MEDIA_MUTED: &str = "m-0 text-0 font-6 leading-0 text-text-muted";
const ASPECT_RATIO_CAPTION: &str = "flex flex-wrap items-start justify-between gap-2xs";
const ASPECT_RATIO_CAPTION_TEXT: &str = "grid min-w-0 gap-3xs";
const ASPECT_RATIO_LABEL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const ASPECT_RATIO_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const ASPECT_RATIO_BADGE: &str = "inline-flex rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-7 uppercase tracking-label text-text-muted";
const ASPECT_RATIO_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ATTACHMENT_ROOT: &str = "flex min-w-0 items-center gap-xs rounded-box border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const ATTACHMENT_ROOT_LOADING: &str = "flex min-w-0 items-center gap-xs rounded-box border border-info bg-info-soft p-xs text-text-1 shadow-1";
const ATTACHMENT_ROOT_DISABLED: &str = "flex min-w-0 items-center gap-xs rounded-box border border-border-muted bg-surface-2 p-xs text-text-disabled";
const ATTACHMENT_PREVIEW_PDF: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-error-soft text-00 font-7 text-text-1";
const ATTACHMENT_PREVIEW_IMAGE: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-success-soft text-00 font-7 text-text-1";
const ATTACHMENT_PREVIEW_ARCHIVE: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-warning-soft text-00 font-7 text-text-1";
const ATTACHMENT_PREVIEW_DATA: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-info-soft text-00 font-7 text-text-1";
const ATTACHMENT_PREVIEW_MUTED: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-surface-3 text-00 font-7 text-text-muted";
const ATTACHMENT_BODY: &str = "grid min-w-0 flex-1 gap-3xs";
const ATTACHMENT_TITLE: &str =
    "m-0 overflow-hidden text-ellipsis whitespace-nowrap text-0 font-7 leading-0 text-text-1";
const ATTACHMENT_TITLE_DISABLED: &str = "m-0 overflow-hidden text-ellipsis whitespace-nowrap text-0 font-7 leading-0 text-text-disabled";
const ATTACHMENT_META: &str = "m-0 text-00 leading-0 text-text-2";
const ATTACHMENT_META_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const ATTACHMENT_ACTION: &str = "inline-flex min-h-field shrink-0 items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ATTACHMENT_ACTION_ACTIVE: &str = "inline-flex min-h-field shrink-0 items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ATTACHMENT_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";

#[derive(Clone)]
struct WidgetViewNode {
    part: String,
    kind: UiWidgetSlotKind,
    label: String,
    value: String,
    intent: UiWidgetIntent,
    selected: bool,
    disabled: bool,
}

impl<P: CatalogComponentPart> From<CatalogComponentRenderNode<P>> for WidgetViewNode {
    fn from(node: CatalogComponentRenderNode<P>) -> Self {
        Self {
            part: node.part.label().to_owned(),
            kind: node.kind,
            label: node.label,
            value: node.value,
            intent: node.intent,
            selected: node.selected,
            disabled: node.disabled,
        }
    }
}

#[component]
pub fn HealthCard(title: &'static str, body: &'static str) -> impl IntoView {
    view! {
        <section class=HEALTH_CARD>
            <p class=HEALTH_CARD_EYEBROW>"rs-dean"</p>
            <h2 class=HEALTH_CARD_TITLE>{title}</h2>
            <p class=HEALTH_CARD_BODY>{body}</p>
        </section>
    }
}

#[component]
pub fn ShadcnComponentGallery() -> impl IntoView {
    view! {
        <section class=COMPONENT_GALLERY data-story-id="shadcn-component-gallery">
            {UiComponentId::ALL
                .into_iter()
                .map(|id| view! { <ShadcnComponentPreview id /> })
                .collect_view()}
        </section>
    }
}

#[component]
pub fn ShadcnComponentPreview(id: UiComponentId) -> impl IntoView {
    let spec = component_spec(id);
    let implementation = component_implementation(id);
    let definition = spec.definition;
    let category = definition.category.label();
    let framework = definition.framework.label();
    let state = definition.state.label();
    let slug = definition.slug;
    let name = definition.name;
    let summary = definition.summary;

    view! {
        <article class=COMPONENT_CARD data-component-id=slug>
            <header class=COMPONENT_HEADER>
                <div class=COMPONENT_META>
                    <span class=COMPONENT_PILL>{category}</span>
                    <span class=COMPONENT_PILL>{framework}</span>
                    <span class=COMPONENT_PILL>{state}</span>
                </div>
                <h2 class=COMPONENT_TITLE>{name}</h2>
                <p class=COMPONENT_SUMMARY>{summary}</p>
            </header>
            <div class=COMPONENT_BLOCKS>
                {spec.blocks.into_iter().map(component_block).collect_view()}
            </div>
            {implementation_recipe(implementation)}
            <section class=COMPONENT_DEMO>
                <h3 class=COMPONENT_RECIPE_TITLE>"Live component"</h3>
                <ComponentDemo id />
            </section>
        </article>
    }
}

fn implementation_recipe(implementation: ComponentImplementation) -> impl IntoView {
    view! {
        <section class=COMPONENT_RECIPE>
            <h3 class=COMPONENT_RECIPE_TITLE>"Implementation"</h3>
            <p class=BLOCK_DETAIL>{implementation.end_user_outcome}</p>
            <p class=BLOCK_DETAIL>{implementation.consumer_contract}</p>
            <div class=COMPONENT_META>
                <span class=COMPONENT_PILL>{implementation.maturity.label()}</span>
                <span class=COMPONENT_PILL>{implementation.render.label()}</span>
                <span class=COMPONENT_PILL>{implementation.state.label()}</span>
                <span class=COMPONENT_PILL>{implementation.layout.label()}</span>
            </div>
            <TokenList title="Variants" values=implementation.variants />
            <TokenList title="States" values=implementation.states />
            <TokenList title="Accessibility" values=implementation.accessibility />
        </section>
    }
}

#[component]
fn TokenList(title: &'static str, values: &'static [&'static str]) -> impl IntoView {
    view! {
        <div class="grid gap-2xs">
            <h4 class=COMPONENT_RECIPE_TITLE>{title}</h4>
            <div class=COMPONENT_TAG_ROW>
                {values
                    .iter()
                    .map(|value| view! { <span class=COMPONENT_TAG>{*value}</span> })
                    .collect_view()}
            </div>
        </div>
    }
}

fn component_block(block: UiBlock) -> impl IntoView {
    view! {
        <div class=block_class(block.tone) data-ui-role=block.role.slug()>
            <p class=BLOCK_LABEL>{block.label}</p>
            <p class=BLOCK_DETAIL>{block.detail}</p>
        </div>
    }
}

fn block_class(tone: UiBlockTone) -> &'static str {
    match tone {
        UiBlockTone::Accent => {
            "grid min-h-field gap-3xs rounded-field border border-accent bg-accent-soft p-s"
        }
        UiBlockTone::Brand => {
            "grid min-h-field gap-3xs rounded-field border border-brand bg-primary-soft p-s"
        }
        UiBlockTone::Danger => {
            "grid min-h-field gap-3xs rounded-field border border-danger bg-error-soft p-s"
        }
        UiBlockTone::Info => {
            "grid min-h-field gap-3xs rounded-field border border-info bg-info-soft p-s"
        }
        UiBlockTone::Muted => {
            "grid min-h-field gap-3xs rounded-field border border-border-subtle bg-surface-2 p-s"
        }
        UiBlockTone::Surface => {
            "grid min-h-field gap-3xs rounded-field border border-border-subtle bg-surface-1 p-s"
        }
        UiBlockTone::Success => {
            "grid min-h-field gap-3xs rounded-field border border-success bg-success-soft p-s"
        }
        UiBlockTone::Warning => {
            "grid min-h-field gap-3xs rounded-field border border-warning bg-warning-soft p-s"
        }
    }
}

#[component]
pub fn ComponentDemo(id: UiComponentId) -> AnyView {
    match id {
        UiComponentId::Accordion => view! { <Accordion /> }.into_any(),
        UiComponentId::Alert => view! { <Alert /> }.into_any(),
        UiComponentId::AlertDialog => view! { <AlertDialog /> }.into_any(),
        UiComponentId::AspectRatio => view! { <AspectRatio /> }.into_any(),
        UiComponentId::Attachment => view! { <Attachment /> }.into_any(),
        UiComponentId::Avatar => view! { <Avatar /> }.into_any(),
        UiComponentId::Badge => view! { <Badge /> }.into_any(),
        UiComponentId::Breadcrumb => view! { <Breadcrumb /> }.into_any(),
        UiComponentId::Bubble => view! { <Bubble /> }.into_any(),
        UiComponentId::Button => view! { <Button /> }.into_any(),
        UiComponentId::ButtonGroup => view! { <ButtonGroup /> }.into_any(),
        UiComponentId::Calendar => view! { <Calendar /> }.into_any(),
        UiComponentId::Card => view! { <Card /> }.into_any(),
        UiComponentId::Carousel => view! { <Carousel /> }.into_any(),
        UiComponentId::Chart => view! { <Chart /> }.into_any(),
        UiComponentId::Checkbox => view! { <Checkbox /> }.into_any(),
        UiComponentId::Collapsible => view! { <Collapsible /> }.into_any(),
        UiComponentId::Combobox => view! { <Combobox /> }.into_any(),
        UiComponentId::Command => view! { <Command /> }.into_any(),
        UiComponentId::ContextMenu => view! { <ContextMenu /> }.into_any(),
        UiComponentId::DataTable => view! { <DataTable /> }.into_any(),
        UiComponentId::DatePicker => view! { <DatePicker /> }.into_any(),
        UiComponentId::Dialog => view! { <Dialog /> }.into_any(),
        UiComponentId::Direction => view! { <Direction /> }.into_any(),
        UiComponentId::Drawer => view! { <Drawer /> }.into_any(),
        UiComponentId::DropdownMenu => view! { <DropdownMenu /> }.into_any(),
        UiComponentId::Empty => view! { <Empty /> }.into_any(),
        UiComponentId::Field => view! { <Field /> }.into_any(),
        UiComponentId::HoverCard => view! { <HoverCard /> }.into_any(),
        UiComponentId::Input => view! { <Input /> }.into_any(),
        UiComponentId::InputGroup => view! { <InputGroup /> }.into_any(),
        UiComponentId::InputOtp => view! { <InputOtp /> }.into_any(),
        UiComponentId::Item => view! { <Item /> }.into_any(),
        UiComponentId::Kbd => view! { <Kbd /> }.into_any(),
        UiComponentId::Label => view! { <Label /> }.into_any(),
        UiComponentId::Marker => view! { <Marker /> }.into_any(),
        UiComponentId::Menubar => view! { <Menubar /> }.into_any(),
        UiComponentId::Message => view! { <Message /> }.into_any(),
        UiComponentId::MessageScroller => view! { <MessageScroller /> }.into_any(),
        UiComponentId::NativeSelect => view! { <NativeSelect /> }.into_any(),
        UiComponentId::NavigationMenu => view! { <NavigationMenu /> }.into_any(),
        UiComponentId::Pagination => view! { <Pagination /> }.into_any(),
        UiComponentId::Popover => view! { <Popover /> }.into_any(),
        UiComponentId::Progress => view! { <Progress /> }.into_any(),
        UiComponentId::RadioGroup => view! { <RadioGroup /> }.into_any(),
        UiComponentId::Resizable => view! { <Resizable /> }.into_any(),
        UiComponentId::ScrollArea => view! { <ScrollArea /> }.into_any(),
        UiComponentId::Select => view! { <Select /> }.into_any(),
        UiComponentId::Separator => view! { <Separator /> }.into_any(),
        UiComponentId::Sheet => view! { <Sheet /> }.into_any(),
        UiComponentId::Sidebar => view! { <Sidebar /> }.into_any(),
        UiComponentId::Skeleton => view! { <Skeleton /> }.into_any(),
        UiComponentId::Slider => view! { <Slider /> }.into_any(),
        UiComponentId::Sonner => view! { <Sonner /> }.into_any(),
        UiComponentId::Spinner => view! { <Spinner /> }.into_any(),
        UiComponentId::Switch => view! { <Switch /> }.into_any(),
        UiComponentId::Table => view! { <Table /> }.into_any(),
        UiComponentId::Tabs => view! { <Tabs /> }.into_any(),
        UiComponentId::Textarea => view! { <Textarea /> }.into_any(),
        UiComponentId::Toast => view! { <Toast /> }.into_any(),
        UiComponentId::Toggle => view! { <Toggle /> }.into_any(),
        UiComponentId::ToggleGroup => view! { <ToggleGroup /> }.into_any(),
        UiComponentId::Tooltip => view! { <Tooltip /> }.into_any(),
        UiComponentId::Typography => view! { <Typography /> }.into_any(),
    }
}

fn render_catalog_component<P: CatalogComponentPart>(model: CatalogComponentModel<P>) -> AnyView {
    let id = P::ID;
    let slug = id.definition().slug;
    let pattern = model.pattern.label();
    let nodes = match catalog_component_render_nodes(&model) {
        Ok(nodes) => nodes,
        Err(report) => {
            let message = format!("{} validation failed: {report}", id.definition().name);
            return view! {
                <section class=WIDGET_SHELL data-ui-widget=slug data-ui-state="invalid">
                    <p class=WIDGET_ERROR role="alert">{message}</p>
                </section>
            }
            .into_any();
        }
    };
    let root = nodes
        .first()
        .cloned()
        .map(WidgetViewNode::from)
        .expect("invariant: validated catalog components always include a root node");
    let content_class = content_class(model.pattern);
    let slots = nodes
        .into_iter()
        .skip(1)
        .map(WidgetViewNode::from)
        .collect::<Vec<_>>();

    view! {
        <section class=widget_class(model.pattern) data-ui-widget=slug data-ui-pattern=pattern>
            <header class=WIDGET_HEADER>
                <p class=WIDGET_EYEBROW>{pattern}</p>
                <h3 class=WIDGET_TITLE>{root.label}</h3>
                <p class=WIDGET_SUMMARY>{root.value}</p>
            </header>
            <div class=content_class>
                {slots.into_iter().map(slot_view).collect_view()}
            </div>
        </section>
    }
    .into_any()
}

fn slot_view(slot: WidgetViewNode) -> AnyView {
    let part = slot.part.clone();
    let kind = slot.kind.label();
    let intent = slot.intent.label();
    let selected = slot.selected.to_string();
    let disabled = slot.disabled;
    let slot_class = slot_class(&slot);

    match slot.kind {
        UiWidgetSlotKind::Avatar => view! {
            <div class=WIDGET_SLOT data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_AVATAR aria-hidden="true">{slot.label.clone()}</span>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Badge => view! {
            <span class=WIDGET_BADGE data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                {slot.label.clone()}
            </span>
        }.into_any(),
        UiWidgetSlotKind::Button | UiWidgetSlotKind::IconButton => view! {
            <button type="button" class=button_class(&slot) data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-pressed=selected disabled=disabled>
                {slot.label.clone()}
            </button>
        }.into_any(),
        UiWidgetSlotKind::Cell => view! {
            <div role="cell" class=WIDGET_TABLE_CELL data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                {slot.label.clone()}
            </div>
        }.into_any(),
        UiWidgetSlotKind::Chart => view! {
            <div class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <div class="flex items-end gap-2xs" aria-hidden="true">
                    <span class="h-xs w-s rounded-field bg-brand"></span>
                    <span class="h-m w-s rounded-field bg-success"></span>
                    <span class="h-l w-s rounded-field bg-accent"></span>
                </div>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Checkbox => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <input type="checkbox" class="size-s rounded-selector border border-border-strong accent-brand" checked=slot.selected disabled=disabled />
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Description | UiWidgetSlotKind::Text => view! {
            <div class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Header | UiWidgetSlotKind::Title => view! {
            <div class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <h4 class=WIDGET_TITLE>{slot.label.clone()}</h4>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Input => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <input class=WIDGET_INPUT value=slot.value.clone() aria-label=slot.label.clone() disabled=disabled />
            </label>
        }.into_any(),
        UiWidgetSlotKind::Key => view! {
            <kbd class=WIDGET_KEY data-ui-part=part data-ui-kind=kind data-ui-intent=intent>{slot.label.clone()}</kbd>
        }.into_any(),
        UiWidgetSlotKind::Link => view! {
            <a class=button_class(&slot) href="#" data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-current=selected>
                {slot.label.clone()}
            </a>
        }.into_any(),
        UiWidgetSlotKind::List => view! {
            <div role="list" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::ListItem | UiWidgetSlotKind::Option => view! {
            <div role="option" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Marker => view! {
            <span class=WIDGET_MARKER data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                {slot.label.clone()}
            </span>
        }.into_any(),
        UiWidgetSlotKind::Media => view! {
            <figure class=WIDGET_MEDIA data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span>{slot.label.clone()}</span>
                <figcaption class=WIDGET_VALUE>{slot.value.clone()}</figcaption>
            </figure>
        }.into_any(),
        UiWidgetSlotKind::Overlay | UiWidgetSlotKind::Panel | UiWidgetSlotKind::Section => view! {
            <section class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </section>
        }.into_any(),
        UiWidgetSlotKind::Progress => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <progress class=WIDGET_PROGRESS value="64" max="100">{slot.value.clone()}</progress>
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Radio => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <input type="radio" class="size-s border border-border-strong accent-brand" checked=slot.selected disabled=disabled />
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Row => view! {
            <div role="row" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Select => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <select class=WIDGET_INPUT aria-label=slot.label.clone() disabled=disabled>
                    <option selected=slot.selected>{slot.value.clone()}</option>
                    <option>"Light"</option>
                    <option>"Dark"</option>
                </select>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Separator => view! {
            <div class="grid gap-2xs" data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_SEPARATOR role="separator" aria-label=slot.value.clone()></span>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Skeleton => view! {
            <div class="grid gap-2xs" data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-busy="true">
                <span class=WIDGET_SKELETON></span>
                <span class="h-xs rounded-field bg-surface-3"></span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Slider => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <input type="range" class=WIDGET_RANGE min="0" max="100" value="72" aria-label=slot.label.clone() disabled=disabled />
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Spinner => view! {
            <div role="status" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_SPINNER aria-hidden="true"></span>
                <span class=WIDGET_VALUE>{slot.label.clone()}</span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Switch => view! {
            <button type="button" role="switch" class=button_class(&slot) data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-checked=selected disabled=disabled>
                {slot.label.clone()}
            </button>
        }.into_any(),
        UiWidgetSlotKind::Table => view! {
            <div role="table" class=WIDGET_TABLE data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <div role="row" class="grid grid-cols-2">
                    <span role="cell" class=WIDGET_TABLE_CELL>{slot.label.clone()}</span>
                    <span role="cell" class=WIDGET_TABLE_CELL>{slot.value.clone()}</span>
                </div>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Textarea => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <textarea class=WIDGET_TEXTAREA aria-label=slot.label.clone() disabled=disabled>{slot.value.clone()}</textarea>
            </label>
        }.into_any(),
    }
}

fn widget_class(pattern: UiWidgetPattern) -> &'static str {
    match pattern {
        UiWidgetPattern::Action | UiWidgetPattern::Navigation => {
            "grid min-w-0 gap-s rounded-field bg-surface-1 p-s text-text-1"
        }
        UiWidgetPattern::Overlay => {
            "grid min-w-0 gap-s rounded-field bg-surface-elevated p-s text-text-1 shadow-2"
        }
        UiWidgetPattern::Callout | UiWidgetPattern::Feedback => {
            "grid min-w-0 gap-s rounded-field bg-warning-soft p-s text-text-1"
        }
        UiWidgetPattern::Data
        | UiWidgetPattern::Disclosure
        | UiWidgetPattern::Display
        | UiWidgetPattern::Form
        | UiWidgetPattern::Layout
        | UiWidgetPattern::Messaging
        | UiWidgetPattern::Typography
        | UiWidgetPattern::Utility => WIDGET_SHELL,
    }
}

fn content_class(pattern: UiWidgetPattern) -> &'static str {
    match pattern {
        UiWidgetPattern::Action | UiWidgetPattern::Navigation => WIDGET_CONTENT_INLINE,
        UiWidgetPattern::Data => "grid gap-2xs overflow-hidden",
        _ => WIDGET_CONTENT,
    }
}

fn slot_class(slot: &WidgetViewNode) -> &'static str {
    if slot.selected {
        WIDGET_SLOT_ACTIVE
    } else {
        WIDGET_SLOT
    }
}

fn button_class(slot: &WidgetViewNode) -> &'static str {
    if slot.selected {
        WIDGET_BUTTON_ACTIVE
    } else {
        WIDGET_BUTTON
    }
}

macro_rules! catalog_component {
    ($name:ident, $model:ty, $default:path) => {
        #[component]
        pub fn $name(#[prop(optional, default = $default())] model: $model) -> AnyView {
            render_catalog_component(model)
        }
    };
}

#[component]
pub fn Alert(#[prop(optional, default = default_alert_model())] model: AlertModel) -> AnyView {
    if let Err(report) = validate_alert_model(&model) {
        let message = format!("Alert validation failed: {report}");
        return view! {
            <section class=ALERT_STANDARD_DESTRUCTIVE data-ui-component="alert" data-ui-state="invalid" role="alert">
                <p class=ALERT_ERROR>{message}</p>
            </section>
        }
        .into_any();
    }

    let root_class = alert_root_class(model.tone, model.density);
    let marker_class = alert_marker_class(model.tone);
    let marker_label = alert_marker_label(model.tone);
    let title_class = alert_title_class(model.density);
    let description_class = alert_description_class(model.density);
    let role = alert_role(model.tone);
    let state = alert_state(&model);
    let title = model.title;
    let description = model.description;
    let tone = model.tone.label();
    let density = model.density.label();
    let loading = model.loading;
    let disabled = model.disabled;
    let action = model.action;

    view! {
        <section
            class=root_class
            data-ui-component="alert"
            data-ui-part="Alert"
            data-ui-tone=tone
            data-ui-density=density
            data-ui-state=state
            role=role
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
        >
            <div class=ALERT_ROW>
                <span class=marker_class aria-hidden="true">{marker_label}</span>
                <div class=ALERT_BODY>
                    <h3 class=title_class data-ui-part="AlertTitle">{title}</h3>
                    <p class=description_class data-ui-part="AlertDescription">{description}</p>
                </div>
                {if let Some(action) = action {
                    let action_disabled = disabled || loading || action.disabled;
                    let action_state = if action_disabled { "disabled" } else { "ready" };
                    let label = if loading {
                        "Loading".to_owned()
                    } else {
                        action.label
                    };

                    view! {
                        <button
                            type="button"
                            class=ALERT_ACTION
                            data-ui-part="AlertAction"
                            data-ui-intent="activate"
                            data-ui-action=action.value
                            data-ui-state=action_state
                            disabled=action_disabled
                        >
                            {label}
                        </button>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }}
            </div>
        </section>
    }
    .into_any()
}

fn alert_root_class(tone: AlertTone, density: AlertDensity) -> &'static str {
    match (density, tone) {
        (AlertDensity::Standard, AlertTone::Default) => ALERT_STANDARD_DEFAULT,
        (AlertDensity::Standard, AlertTone::Info) => ALERT_STANDARD_INFO,
        (AlertDensity::Standard, AlertTone::Success) => ALERT_STANDARD_SUCCESS,
        (AlertDensity::Standard, AlertTone::Warning) => ALERT_STANDARD_WARNING,
        (AlertDensity::Standard, AlertTone::Destructive) => ALERT_STANDARD_DESTRUCTIVE,
        (AlertDensity::Dense, AlertTone::Default) => ALERT_DENSE_DEFAULT,
        (AlertDensity::Dense, AlertTone::Info) => ALERT_DENSE_INFO,
        (AlertDensity::Dense, AlertTone::Success) => ALERT_DENSE_SUCCESS,
        (AlertDensity::Dense, AlertTone::Warning) => ALERT_DENSE_WARNING,
        (AlertDensity::Dense, AlertTone::Destructive) => ALERT_DENSE_DESTRUCTIVE,
    }
}

const fn alert_marker_class(tone: AlertTone) -> &'static str {
    match tone {
        AlertTone::Default => ALERT_MARKER_DEFAULT,
        AlertTone::Info => ALERT_MARKER_INFO,
        AlertTone::Success => ALERT_MARKER_SUCCESS,
        AlertTone::Warning => ALERT_MARKER_WARNING,
        AlertTone::Destructive => ALERT_MARKER_DESTRUCTIVE,
    }
}

const fn alert_marker_label(tone: AlertTone) -> &'static str {
    match tone {
        AlertTone::Default | AlertTone::Info => "i",
        AlertTone::Success => "+",
        AlertTone::Warning | AlertTone::Destructive => "!",
    }
}

const fn alert_title_class(density: AlertDensity) -> &'static str {
    match density {
        AlertDensity::Standard => ALERT_TITLE,
        AlertDensity::Dense => ALERT_TITLE_DENSE,
    }
}

const fn alert_description_class(density: AlertDensity) -> &'static str {
    match density {
        AlertDensity::Standard => ALERT_DESCRIPTION,
        AlertDensity::Dense => ALERT_DESCRIPTION_DENSE,
    }
}

const fn alert_role(tone: AlertTone) -> &'static str {
    match tone {
        AlertTone::Warning | AlertTone::Destructive => "alert",
        AlertTone::Default | AlertTone::Info | AlertTone::Success => "status",
    }
}

const fn alert_state(model: &AlertModel) -> &'static str {
    if model.loading {
        "loading"
    } else if model.disabled {
        "disabled"
    } else {
        "ready"
    }
}

#[component]
pub fn AlertDialog(
    #[prop(optional, default = default_alert_dialog_model())] model: AlertDialogModel,
    #[prop(optional, default = false)] default_open: bool,
) -> AnyView {
    if let Err(report) = validate_alert_dialog_model(&model) {
        let message = format!("AlertDialog validation failed: {report}");
        return view! {
            <section class=ALERT_DIALOG_ROOT data-ui-component="alert-dialog" data-ui-state="invalid">
                <p class=ALERT_DIALOG_ERROR role="alert">{message}</p>
            </section>
        }
        .into_any();
    }

    let initial_state = if default_open {
        AlertDialogState::open()
    } else {
        AlertDialogState::closed()
    };
    let (state, set_state) = signal(initial_state);
    let is_open = Memo::new(move |_| state.with(|state| state.is_open()));
    let trigger_label = model.trigger_label;
    let title = model.title;
    let description = model.description;
    let title_id = alert_dialog_dom_id("alert-dialog-title", &model.action.value);
    let description_id = alert_dialog_dom_id("alert-dialog-description", &model.action.value);
    let content_class = alert_dialog_content_class(model.size, model.destructive);
    let title_class = alert_dialog_title_class(model.size);
    let description_class = alert_dialog_description_class(model.size);
    let action_class = alert_dialog_action_class(model.destructive);
    let action_value = model.action.value;
    let action_label = if model.loading {
        "Working".to_owned()
    } else {
        model.action.label
    };
    let action_disabled = model.disabled || model.loading || model.action.disabled;
    let cancel_value = model.cancel.value;
    let cancel_label = model.cancel.label;
    let cancel_disabled = model.disabled || model.loading || model.cancel.disabled;
    let disabled = model.disabled;
    let loading = model.loading;
    let size = model.size.label();
    let tone = if model.destructive {
        "destructive"
    } else {
        "default"
    };
    let action_state = if action_disabled { "disabled" } else { "ready" };
    let cancel_state = if cancel_disabled { "disabled" } else { "ready" };

    let open_dialog = move |_| {
        if disabled {
            return;
        }
        set_state.update(|state| {
            let _ = state.apply(AlertDialogIntent::Open);
        });
    };
    let is_open_for_trigger = is_open;
    let is_open_for_root = is_open;
    let is_open_for_overlay = is_open;

    view! {
        <section
            class=ALERT_DIALOG_ROOT
            data-ui-component="alert-dialog"
            data-ui-size=size
            data-ui-tone=tone
            data-ui-state=move || {
                if is_open_for_root.get() {
                    "open"
                } else if disabled {
                    "disabled"
                } else {
                    "closed"
                }
            }
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=ALERT_DIALOG_TRIGGER
                data-ui-part="AlertDialogTrigger"
                aria-haspopup="dialog"
                aria-expanded=move || is_open_for_trigger.get().to_string()
                disabled=disabled
                on:click=open_dialog
            >
                {trigger_label}
            </button>
            {move || {
                if is_open_for_overlay.get() {
                    let action_value_for_click = action_value.clone();
                    let confirm_dialog = move |_| {
                        if action_disabled {
                            return;
                        }
                        set_state.update(|state| {
                            let _ = state.apply(AlertDialogIntent::Confirm(
                                action_value_for_click.clone(),
                            ));
                        });
                    };
                    let cancel_dialog = move |_| {
                        if cancel_disabled {
                            return;
                        }
                        set_state.update(|state| {
                            let _ = state.apply(AlertDialogIntent::Cancel);
                        });
                    };

                    view! {
                        <div class=ALERT_DIALOG_OVERLAY data-ui-part="AlertDialog">
                            <section
                                class=content_class
                                role="dialog"
                                aria-modal="true"
                                aria-labelledby=title_id.clone()
                                aria-describedby=description_id.clone()
                                data-ui-part="AlertDialogContent"
                            >
                                <header class=ALERT_DIALOG_HEADER data-ui-part="AlertDialogHeader">
                                    <h3 id=title_id.clone() class=title_class>{title.clone()}</h3>
                                    <p id=description_id.clone() class=description_class>{description.clone()}</p>
                                </header>
                                <footer class=ALERT_DIALOG_FOOTER data-ui-part="AlertDialogFooter">
                                    <button
                                        type="button"
                                        class=ALERT_DIALOG_CANCEL
                                        data-ui-part="AlertDialogCancel"
                                        data-ui-intent="close"
                                        data-ui-action=cancel_value.clone()
                                        data-ui-state=cancel_state
                                        disabled=cancel_disabled
                                        on:click=cancel_dialog
                                    >
                                        {cancel_label.clone()}
                                    </button>
                                    <button
                                        type="button"
                                        class=action_class
                                        data-ui-part="AlertDialogAction"
                                        data-ui-intent="activate"
                                        data-ui-action=action_value.clone()
                                        data-ui-state=action_state
                                        disabled=action_disabled
                                        on:click=confirm_dialog
                                    >
                                        {action_label.clone()}
                                    </button>
                                </footer>
                            </section>
                        </div>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}
        </section>
    }
    .into_any()
}

const fn alert_dialog_content_class(size: AlertDialogSize, destructive: bool) -> &'static str {
    match (size, destructive) {
        (AlertDialogSize::Default, false) => ALERT_DIALOG_CONTENT,
        (AlertDialogSize::Default, true) => ALERT_DIALOG_CONTENT_DESTRUCTIVE,
        (AlertDialogSize::Small, false) => ALERT_DIALOG_CONTENT_SMALL,
        (AlertDialogSize::Small, true) => ALERT_DIALOG_CONTENT_SMALL_DESTRUCTIVE,
    }
}

const fn alert_dialog_title_class(size: AlertDialogSize) -> &'static str {
    match size {
        AlertDialogSize::Default => ALERT_DIALOG_TITLE,
        AlertDialogSize::Small => ALERT_DIALOG_TITLE_SMALL,
    }
}

const fn alert_dialog_description_class(size: AlertDialogSize) -> &'static str {
    match size {
        AlertDialogSize::Default => ALERT_DIALOG_DESCRIPTION,
        AlertDialogSize::Small => ALERT_DIALOG_DESCRIPTION_SMALL,
    }
}

const fn alert_dialog_action_class(destructive: bool) -> &'static str {
    if destructive {
        ALERT_DIALOG_ACTION_DESTRUCTIVE
    } else {
        ALERT_DIALOG_ACTION
    }
}

#[component]
pub fn Accordion(
    #[prop(optional, default = default_accordion_items())] items: Vec<AccordionItem>,
    #[prop(optional, default = AccordionMode::Single)] mode: AccordionMode,
    #[prop(optional, default = vec!["tokens".to_owned()])] default_open: Vec<String>,
) -> AnyView {
    let model = AccordionModel::new(mode, items).with_default_open(default_open);
    if let Err(report) = validate_accordion_model(&model) {
        let message = format!("Accordion validation failed: {report}");
        return view! {
            <section class=ACCORDION_ROOT data-ui-component="accordion" data-ui-state="invalid">
                <p class=ACCORDION_ERROR role="alert">{message}</p>
            </section>
        }
        .into_any();
    }

    let (state, set_state) = signal(model.state());
    let items = model.items;

    view! {
        <section class=ACCORDION_ROOT data-ui-component="accordion" data-ui-mode=mode.label()>
            {if items.is_empty() {
                view! {
                    <p class=ACCORDION_EMPTY data-ui-part="AccordionEmpty">
                        "No accordion sections are available."
                    </p>
                }
                    .into_any()
            } else {
                items
                    .into_iter()
                    .map(move |item| {
                        let value = item.value.clone();
                        let title = item.title;
                        let content = item.content;
                        let disabled = item.disabled;
                        let trigger_id = accordion_dom_id("accordion-trigger", &value);
                        let panel_id = accordion_dom_id("accordion-panel", &value);
                        let open_value = value.clone();
                        let is_open = Memo::new(move |_| {
                            state.with(|state| state.is_open(&open_value))
                        });
                        let toggle_value = value.clone();
                        let on_click = move |_| {
                            if disabled {
                                return;
                            }
                            set_state.update(|state| {
                                let _ = state.apply(AccordionIntent::Toggle(toggle_value.clone()));
                            });
                        };
                        let is_open_for_item = is_open;
                        let is_open_for_trigger = is_open;
                        let is_open_for_indicator = is_open;
                        let is_open_for_hidden = is_open;

                        view! {
                            <article
                                class=move || {
                                    if is_open_for_item.get() {
                                        ACCORDION_ITEM_OPEN
                                    } else {
                                        ACCORDION_ITEM
                                    }
                                }
                                data-ui-part="AccordionItem"
                                data-value=value
                                data-state=move || {
                                    if is_open_for_trigger.get() {
                                        "open"
                                    } else {
                                        "closed"
                                    }
                                }
                            >
                                <h3 class="m-0">
                                    <button
                                        id=trigger_id.clone()
                                        type="button"
                                        class=move || {
                                            if is_open_for_trigger.get() {
                                                ACCORDION_TRIGGER_OPEN
                                            } else {
                                                ACCORDION_TRIGGER
                                            }
                                        }
                                        aria-expanded=move || {
                                            is_open_for_trigger.get().to_string()
                                        }
                                        aria-controls=panel_id.clone()
                                        aria-disabled=disabled.to_string()
                                        disabled=disabled
                                        on:click=on_click
                                    >
                                        <span>{title}</span>
                                        <span class=ACCORDION_INDICATOR aria-hidden="true">
                                            {move || if is_open_for_indicator.get() { "-" } else { "+" }}
                                        </span>
                                    </button>
                                </h3>
                                <div
                                    id=panel_id
                                    role="region"
                                    class=ACCORDION_CONTENT
                                    aria-labelledby=trigger_id
                                    hidden=move || !is_open_for_hidden.get()
                                    data-ui-part="AccordionContent"
                                >
                                    <p class="m-0">{content}</p>
                                </div>
                            </article>
                        }
                    })
                    .collect_view()
                    .into_any()
            }}
        </section>
    }
    .into_any()
}

#[component]
pub fn AspectRatio(
    #[prop(optional, default = default_aspect_ratio_model())] model: AspectRatioModel,
) -> AnyView {
    if let Err(report) = validate_aspect_ratio_model(&model) {
        let message = format!("AspectRatio validation failed: {report}");
        return view! {
            <section class=ASPECT_RATIO_ROOT data-ui-component="aspect-ratio" data-ui-state="invalid">
                <p class=ASPECT_RATIO_ERROR role="alert">{message}</p>
            </section>
        }
        .into_any();
    }

    let nodes = aspect_ratio_render_nodes(&model);
    let frame = nodes
        .iter()
        .find(|node| node.part == AspectRatioPart::Frame)
        .expect("invariant: aspect ratio render nodes include frame");
    let media = nodes
        .iter()
        .find(|node| node.part == AspectRatioPart::Media)
        .expect("invariant: aspect ratio render nodes include media");
    let frame_label = frame.label.clone();
    let frame_detail = frame.detail.clone();
    let media_label = media.label.clone();
    let media_detail = media.detail.clone();
    let ratio_label = model.ratio_label();
    let frame_style = model.aspect_ratio_style();
    let fit = model.fit.label();
    let media_class = aspect_ratio_media_class(model.fit, model.loading, model.disabled);
    let state = aspect_ratio_state(&model);
    let loading = model.loading;
    let disabled = model.disabled;

    view! {
        <section
            class=ASPECT_RATIO_ROOT
            data-ui-component="aspect-ratio"
            data-ui-state=state
            data-ui-ratio=ratio_label.clone()
            data-ui-fit=fit
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
        >
            <figure class=ASPECT_RATIO_FIGURE data-ui-part="AspectRatio">
                <div
                    class=ASPECT_RATIO_FRAME
                    style=frame_style
                    data-ui-part="AspectRatioFrame"
                    data-ui-ratio=ratio_label.clone()
                    aria-label=frame_label.clone()
                >
                    <div class=media_class data-ui-part="AspectRatioMedia" data-ui-fit=fit>
                        {if loading {
                            view! {
                                <p class=ASPECT_RATIO_MEDIA_MUTED role="status">
                                    "Loading media"
                                </p>
                            }
                                .into_any()
                        } else {
                            view! {
                                <div class=ASPECT_RATIO_MEDIA_STACK>
                                    <span class=ASPECT_RATIO_MEDIA_MARKER aria-hidden="true">
                                        {ratio_label.clone()}
                                    </span>
                                    <p class=ASPECT_RATIO_MEDIA_LABEL>{media_label}</p>
                                    <p class=ASPECT_RATIO_MEDIA_DETAIL>{media_detail}</p>
                                </div>
                            }
                                .into_any()
                        }}
                    </div>
                </div>
                <figcaption class=ASPECT_RATIO_CAPTION>
                    <span class=ASPECT_RATIO_CAPTION_TEXT>
                        <span class=ASPECT_RATIO_LABEL>{frame_label}</span>
                        <span class=ASPECT_RATIO_DETAIL>{frame_detail}</span>
                    </span>
                    <span class=ASPECT_RATIO_BADGE>{ratio_label.clone()}</span>
                </figcaption>
            </figure>
        </section>
    }
    .into_any()
}

const fn aspect_ratio_media_class(
    fit: AspectRatioFit,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        ASPECT_RATIO_MEDIA_DISABLED
    } else if loading {
        ASPECT_RATIO_MEDIA_LOADING
    } else {
        match fit {
            AspectRatioFit::Cover => ASPECT_RATIO_MEDIA_COVER,
            AspectRatioFit::Contain => ASPECT_RATIO_MEDIA_CONTAIN,
        }
    }
}

const fn aspect_ratio_state(model: &AspectRatioModel) -> &'static str {
    if model.disabled {
        "disabled"
    } else if model.loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Attachment(
    #[prop(optional, default = default_attachment_model())] model: AttachmentModel,
) -> AnyView {
    if let Err(report) = validate_attachment_model(&model) {
        let message = format!("Attachment validation failed: {report}");
        return view! {
            <section class=ATTACHMENT_ROOT data-ui-component="attachment" data-ui-state="invalid">
                <p class=ATTACHMENT_ERROR role="alert">{message}</p>
            </section>
        }
        .into_any();
    }

    let nodes = attachment_render_nodes(&model);
    let preview = nodes
        .iter()
        .find(|node| node.part == AttachmentPart::Preview)
        .expect("invariant: attachment render nodes include preview");
    let title = nodes
        .iter()
        .find(|node| node.part == AttachmentPart::Title)
        .expect("invariant: attachment render nodes include title");
    let meta = nodes
        .iter()
        .find(|node| node.part == AttachmentPart::Meta)
        .expect("invariant: attachment render nodes include metadata");
    let preview_label = preview.label.clone();
    let title_value = title.value.clone();
    let meta_value = meta.value.clone();
    let root_class = attachment_root_class(model.loading, model.disabled);
    let preview_class = attachment_preview_class(model.kind, model.loading, model.disabled);
    let title_class = attachment_title_class(model.disabled);
    let meta_class = attachment_meta_class(model.disabled);
    let state_label = attachment_state(&model);
    let kind = model.kind.label();
    let loading = model.loading;
    let disabled = model.disabled;
    let (state, set_state) = signal(model.state());
    let action = model.action;

    view! {
        <section
            class=root_class
            data-ui-component="attachment"
            data-ui-part="Attachment"
            data-ui-kind=kind
            data-ui-state=state_label
            role="group"
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
        >
            <span class=preview_class data-ui-part="AttachmentPreview" aria-hidden="true">
                {if loading { "...".to_owned() } else { preview_label }}
            </span>
            <span class=ATTACHMENT_BODY>
                <span class=title_class data-ui-part="AttachmentTitle">{title_value}</span>
                <span class=meta_class data-ui-part="AttachmentMeta">
                    {if loading { "Preparing attachment".to_owned() } else { meta_value }}
                </span>
            </span>
            {if let Some(action) = action {
                let action_disabled = disabled || loading || action.disabled;
                let action_label = if loading {
                    "Loading".to_owned()
                } else {
                    action.label
                };
                let action_state = if action_disabled { "disabled" } else { "ready" };
                let action_value = action.value;
                let action_value_for_memo = action_value.clone();
                let action_value_for_click = action_value.clone();
                let is_active = Memo::new(move |_| {
                    state.with(|state| {
                        state.activated_value() == Some(action_value_for_memo.as_str())
                    })
                });
                let is_active_for_class = is_active;
                let is_active_for_pressed = is_active;
                let on_click = move |_| {
                    if action_disabled {
                        return;
                    }
                    set_state.update(|state| {
                        let _ = state.apply(AttachmentIntent::Activate(
                            action_value_for_click.clone(),
                        ));
                    });
                };

                view! {
                    <button
                        type="button"
                        class=move || {
                            if is_active_for_class.get() {
                                ATTACHMENT_ACTION_ACTIVE
                            } else {
                                ATTACHMENT_ACTION
                            }
                        }
                        data-ui-part="AttachmentAction"
                        data-ui-intent="activate"
                        data-ui-action=action_value
                        data-ui-state=action_state
                        aria-pressed=move || is_active_for_pressed.get().to_string()
                        disabled=action_disabled
                        on:click=on_click
                    >
                        {action_label}
                    </button>
                }
                    .into_any()
            } else {
                ().into_any()
            }}
        </section>
    }
    .into_any()
}

const fn attachment_root_class(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        ATTACHMENT_ROOT_DISABLED
    } else if loading {
        ATTACHMENT_ROOT_LOADING
    } else {
        ATTACHMENT_ROOT
    }
}

const fn attachment_preview_class(
    kind: AttachmentKind,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if loading || disabled {
        ATTACHMENT_PREVIEW_MUTED
    } else {
        match kind {
            AttachmentKind::Pdf => ATTACHMENT_PREVIEW_PDF,
            AttachmentKind::Image => ATTACHMENT_PREVIEW_IMAGE,
            AttachmentKind::Archive => ATTACHMENT_PREVIEW_ARCHIVE,
            AttachmentKind::Data => ATTACHMENT_PREVIEW_DATA,
        }
    }
}

const fn attachment_title_class(disabled: bool) -> &'static str {
    if disabled {
        ATTACHMENT_TITLE_DISABLED
    } else {
        ATTACHMENT_TITLE
    }
}

const fn attachment_meta_class(disabled: bool) -> &'static str {
    if disabled {
        ATTACHMENT_META_DISABLED
    } else {
        ATTACHMENT_META
    }
}

const fn attachment_state(model: &AttachmentModel) -> &'static str {
    if model.disabled {
        "disabled"
    } else if model.loading {
        "loading"
    } else {
        "ready"
    }
}

catalog_component!(Avatar, crate::AvatarModel, crate::default_avatar_model);
catalog_component!(Badge, crate::BadgeModel, crate::default_badge_model);
catalog_component!(
    Breadcrumb,
    crate::BreadcrumbModel,
    crate::default_breadcrumb_model
);
catalog_component!(Bubble, crate::BubbleModel, crate::default_bubble_model);
catalog_component!(Button, crate::ButtonModel, crate::default_button_model);
catalog_component!(
    ButtonGroup,
    crate::ButtonGroupModel,
    crate::default_button_group_model
);
catalog_component!(
    Calendar,
    crate::CalendarModel,
    crate::default_calendar_model
);
catalog_component!(Card, crate::CardModel, crate::default_card_model);
catalog_component!(
    Carousel,
    crate::CarouselModel,
    crate::default_carousel_model
);
catalog_component!(Chart, crate::ChartModel, crate::default_chart_model);
catalog_component!(
    Checkbox,
    crate::CheckboxModel,
    crate::default_checkbox_model
);
catalog_component!(
    Collapsible,
    crate::CollapsibleModel,
    crate::default_collapsible_model
);
catalog_component!(
    Combobox,
    crate::ComboboxModel,
    crate::default_combobox_model
);
catalog_component!(Command, crate::CommandModel, crate::default_command_model);
catalog_component!(
    ContextMenu,
    crate::ContextMenuModel,
    crate::default_context_menu_model
);
catalog_component!(
    DataTable,
    crate::DataTableModel,
    crate::default_data_table_model
);
catalog_component!(
    DatePicker,
    crate::DatePickerModel,
    crate::default_date_picker_model
);
catalog_component!(Dialog, crate::DialogModel, crate::default_dialog_model);
catalog_component!(
    Direction,
    crate::DirectionModel,
    crate::default_direction_model
);
catalog_component!(Drawer, crate::DrawerModel, crate::default_drawer_model);
catalog_component!(
    DropdownMenu,
    crate::DropdownMenuModel,
    crate::default_dropdown_menu_model
);
catalog_component!(Empty, crate::EmptyModel, crate::default_empty_model);
catalog_component!(Field, crate::FieldModel, crate::default_field_model);
catalog_component!(
    HoverCard,
    crate::HoverCardModel,
    crate::default_hover_card_model
);
catalog_component!(Input, crate::InputModel, crate::default_input_model);
catalog_component!(
    InputGroup,
    crate::InputGroupModel,
    crate::default_input_group_model
);
catalog_component!(
    InputOtp,
    crate::InputOtpModel,
    crate::default_input_otp_model
);
catalog_component!(Item, crate::ItemModel, crate::default_item_model);
catalog_component!(Kbd, crate::KbdModel, crate::default_kbd_model);
catalog_component!(Label, crate::LabelModel, crate::default_label_model);
catalog_component!(Marker, crate::MarkerModel, crate::default_marker_model);
catalog_component!(Menubar, crate::MenubarModel, crate::default_menubar_model);
catalog_component!(Message, crate::MessageModel, crate::default_message_model);
catalog_component!(
    MessageScroller,
    crate::MessageScrollerModel,
    crate::default_message_scroller_model
);
catalog_component!(
    NativeSelect,
    crate::NativeSelectModel,
    crate::default_native_select_model
);
catalog_component!(
    NavigationMenu,
    crate::NavigationMenuModel,
    crate::default_navigation_menu_model
);
catalog_component!(
    Pagination,
    crate::PaginationModel,
    crate::default_pagination_model
);
catalog_component!(Popover, crate::PopoverModel, crate::default_popover_model);
catalog_component!(
    Progress,
    crate::ProgressModel,
    crate::default_progress_model
);
catalog_component!(
    RadioGroup,
    crate::RadioGroupModel,
    crate::default_radio_group_model
);
catalog_component!(
    Resizable,
    crate::ResizableModel,
    crate::default_resizable_model
);
catalog_component!(
    ScrollArea,
    crate::ScrollAreaModel,
    crate::default_scroll_area_model
);
catalog_component!(Select, crate::SelectModel, crate::default_select_model);
catalog_component!(
    Separator,
    crate::SeparatorModel,
    crate::default_separator_model
);
catalog_component!(Sheet, crate::SheetModel, crate::default_sheet_model);
catalog_component!(Sidebar, crate::SidebarModel, crate::default_sidebar_model);
catalog_component!(
    Skeleton,
    crate::SkeletonModel,
    crate::default_skeleton_model
);
catalog_component!(Slider, crate::SliderModel, crate::default_slider_model);
catalog_component!(Sonner, crate::SonnerModel, crate::default_sonner_model);
catalog_component!(Spinner, crate::SpinnerModel, crate::default_spinner_model);
catalog_component!(Switch, crate::SwitchModel, crate::default_switch_model);
catalog_component!(Table, crate::TableModel, crate::default_table_model);
catalog_component!(Tabs, crate::TabsModel, crate::default_tabs_model);
catalog_component!(
    Textarea,
    crate::TextareaModel,
    crate::default_textarea_model
);
catalog_component!(Toast, crate::ToastModel, crate::default_toast_model);
catalog_component!(Toggle, crate::ToggleModel, crate::default_toggle_model);
catalog_component!(
    ToggleGroup,
    crate::ToggleGroupModel,
    crate::default_toggle_group_model
);
catalog_component!(Tooltip, crate::TooltipModel, crate::default_tooltip_model);
catalog_component!(
    Typography,
    crate::TypographyModel,
    crate::default_typography_model
);

#[component]
pub fn ThemeCycleButton(
    #[prop(optional, default = ThemeChoice::Auto)] initial: ThemeChoice,
) -> impl IntoView {
    let (theme, set_theme) = signal(initial);

    Effect::new(move |_| {
        apply_document_theme(theme.get());
    });

    let cycle = move |_| {
        set_theme.update(|theme| *theme = theme.next());
    };
    let label = move || theme.get().label();
    let aria_label = move || format!("Cycle theme. Current theme: {}", label());

    view! {
        <button type="button" class=THEME_BUTTON aria-label=aria_label on:click=cycle>
            <span class=THEME_BUTTON_LABEL>"Theme"</span>
            <span>{label}</span>
        </button>
    }
}

#[component]
pub fn ThemeScope(theme: ThemeId, children: Children) -> impl IntoView {
    view! {
        <div class=THEME_SCOPE data-theme=theme.slug()>
            {children()}
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_document_theme(choice: ThemeChoice) {
    let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    else {
        return;
    };

    if let Some(theme) = choice.data_theme() {
        let _ = root.set_attribute("data-theme", theme);
    } else {
        let _ = root.remove_attribute("data-theme");
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_document_theme(_choice: ThemeChoice) {}
