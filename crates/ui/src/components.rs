use leptos::prelude::*;

use crate::{
    AccordionIntent, AccordionItem, AccordionMode, AccordionModel, ComponentImplementation,
    ThemeChoice, ThemeId, UiBlock, UiBlockTone, UiComponentId, UiWidget, UiWidgetPattern,
    UiWidgetSlot, UiWidgetSlotKind, accordion_dom_id, component_implementation, component_spec,
    default_accordion_items, widget_for_component,
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
        _ => render_widget(widget_for_component(id)).into_any(),
    }
}

fn render_widget(widget: UiWidget) -> impl IntoView {
    let slug = widget.id.definition().slug;
    let pattern = widget.pattern.label();
    let root = *widget
        .slots
        .first()
        .expect("invariant: literal widgets always include a root slot");
    let content_class = content_class(widget.pattern);
    let slots = widget.slots.into_iter().skip(1).collect::<Vec<_>>();

    view! {
        <section class=widget_class(widget.pattern) data-ui-widget=slug data-ui-pattern=pattern>
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
}

fn slot_view(slot: UiWidgetSlot) -> AnyView {
    let part = slot.part;
    let kind = slot.kind.label();
    let intent = slot.intent.label();
    let selected = slot.selected.to_string();
    let disabled = slot.disabled;
    let slot_class = slot_class(slot);

    match slot.kind {
        UiWidgetSlotKind::Avatar => view! {
            <div class=WIDGET_SLOT data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_AVATAR aria-hidden="true">{slot.label}</span>
                <p class=WIDGET_VALUE>{slot.value}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Badge => view! {
            <span class=WIDGET_BADGE data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                {slot.label}
            </span>
        }.into_any(),
        UiWidgetSlotKind::Button | UiWidgetSlotKind::IconButton => view! {
            <button type="button" class=button_class(slot) data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-pressed=selected disabled=disabled>
                {slot.label}
            </button>
        }.into_any(),
        UiWidgetSlotKind::Cell => view! {
            <div role="cell" class=WIDGET_TABLE_CELL data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                {slot.label}
            </div>
        }.into_any(),
        UiWidgetSlotKind::Chart => view! {
            <div class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <p class=WIDGET_LABEL>{slot.label}</p>
                <div class="flex items-end gap-2xs" aria-hidden="true">
                    <span class="h-xs w-s rounded-field bg-brand"></span>
                    <span class="h-m w-s rounded-field bg-success"></span>
                    <span class="h-l w-s rounded-field bg-accent"></span>
                </div>
                <p class=WIDGET_VALUE>{slot.value}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Checkbox => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label}</span>
                <input type="checkbox" class="size-s rounded-selector border border-border-strong accent-brand" checked=slot.selected disabled=disabled />
                <span class=WIDGET_VALUE>{slot.value}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Description | UiWidgetSlotKind::Text => view! {
            <div class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <p class=WIDGET_LABEL>{slot.label}</p>
                <p class=WIDGET_VALUE>{slot.value}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Header | UiWidgetSlotKind::Title => view! {
            <div class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <h4 class=WIDGET_TITLE>{slot.label}</h4>
                <p class=WIDGET_VALUE>{slot.value}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Input => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label}</span>
                <input class=WIDGET_INPUT value=slot.value aria-label=slot.label disabled=disabled />
            </label>
        }.into_any(),
        UiWidgetSlotKind::Key => view! {
            <kbd class=WIDGET_KEY data-ui-part=part data-ui-kind=kind data-ui-intent=intent>{slot.label}</kbd>
        }.into_any(),
        UiWidgetSlotKind::Link => view! {
            <a class=button_class(slot) href="#" data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-current=selected>
                {slot.label}
            </a>
        }.into_any(),
        UiWidgetSlotKind::List => view! {
            <div role="list" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <p class=WIDGET_LABEL>{slot.label}</p>
                <p class=WIDGET_VALUE>{slot.value}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::ListItem | UiWidgetSlotKind::Option => view! {
            <div role="option" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                <p class=WIDGET_LABEL>{slot.label}</p>
                <p class=WIDGET_VALUE>{slot.value}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Marker => view! {
            <span class=WIDGET_MARKER data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                {slot.label}
            </span>
        }.into_any(),
        UiWidgetSlotKind::Media => view! {
            <figure class=WIDGET_MEDIA data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span>{slot.label}</span>
                <figcaption class=WIDGET_VALUE>{slot.value}</figcaption>
            </figure>
        }.into_any(),
        UiWidgetSlotKind::Overlay | UiWidgetSlotKind::Panel | UiWidgetSlotKind::Section => view! {
            <section class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                <p class=WIDGET_LABEL>{slot.label}</p>
                <p class=WIDGET_VALUE>{slot.value}</p>
            </section>
        }.into_any(),
        UiWidgetSlotKind::Progress => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label}</span>
                <progress class=WIDGET_PROGRESS value="64" max="100">{slot.value}</progress>
                <span class=WIDGET_VALUE>{slot.value}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Radio => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label}</span>
                <input type="radio" class="size-s border border-border-strong accent-brand" checked=slot.selected disabled=disabled />
                <span class=WIDGET_VALUE>{slot.value}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Row => view! {
            <div role="row" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                <span class=WIDGET_LABEL>{slot.label}</span>
                <span class=WIDGET_VALUE>{slot.value}</span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Select => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label}</span>
                <select class=WIDGET_INPUT aria-label=slot.label disabled=disabled>
                    <option selected=slot.selected>{slot.value}</option>
                    <option>"Light"</option>
                    <option>"Dark"</option>
                </select>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Separator => view! {
            <div class="grid gap-2xs" data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_SEPARATOR role="separator" aria-label=slot.value></span>
                <span class=WIDGET_LABEL>{slot.label}</span>
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
                <span class=WIDGET_LABEL>{slot.label}</span>
                <input type="range" class=WIDGET_RANGE min="0" max="100" value="72" aria-label=slot.label disabled=disabled />
                <span class=WIDGET_VALUE>{slot.value}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Spinner => view! {
            <div role="status" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_SPINNER aria-hidden="true"></span>
                <span class=WIDGET_VALUE>{slot.label}</span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Switch => view! {
            <button type="button" role="switch" class=button_class(slot) data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-checked=selected disabled=disabled>
                {slot.label}
            </button>
        }.into_any(),
        UiWidgetSlotKind::Table => view! {
            <div role="table" class=WIDGET_TABLE data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <div role="row" class="grid grid-cols-2">
                    <span role="cell" class=WIDGET_TABLE_CELL>{slot.label}</span>
                    <span role="cell" class=WIDGET_TABLE_CELL>{slot.value}</span>
                </div>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Textarea => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label}</span>
                <textarea class=WIDGET_TEXTAREA aria-label=slot.label disabled=disabled>{slot.value}</textarea>
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

fn slot_class(slot: UiWidgetSlot) -> &'static str {
    if slot.selected {
        WIDGET_SLOT_ACTIVE
    } else {
        WIDGET_SLOT
    }
}

fn button_class(slot: UiWidgetSlot) -> &'static str {
    if slot.selected {
        WIDGET_BUTTON_ACTIVE
    } else {
        WIDGET_BUTTON
    }
}

macro_rules! literal_component {
    ($name:ident, $id:ident) => {
        #[component]
        pub fn $name() -> impl IntoView {
            view! { <ComponentDemo id=UiComponentId::$id /> }
        }
    };
}

#[component]
pub fn Accordion(
    #[prop(optional, default = default_accordion_items())] items: Vec<AccordionItem>,
    #[prop(optional, default = AccordionMode::Single)] mode: AccordionMode,
    #[prop(optional, default = vec!["tokens".to_owned()])] default_open: Vec<String>,
) -> impl IntoView {
    let model = AccordionModel::new(mode, items).with_default_open(default_open);
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
}

literal_component!(Alert, Alert);
literal_component!(AlertDialog, AlertDialog);
literal_component!(AspectRatio, AspectRatio);
literal_component!(Attachment, Attachment);
literal_component!(Avatar, Avatar);
literal_component!(Badge, Badge);
literal_component!(Breadcrumb, Breadcrumb);
literal_component!(Bubble, Bubble);
literal_component!(Button, Button);
literal_component!(ButtonGroup, ButtonGroup);
literal_component!(Calendar, Calendar);
literal_component!(Card, Card);
literal_component!(Carousel, Carousel);
literal_component!(Chart, Chart);
literal_component!(Checkbox, Checkbox);
literal_component!(Collapsible, Collapsible);
literal_component!(Combobox, Combobox);
literal_component!(Command, Command);
literal_component!(ContextMenu, ContextMenu);
literal_component!(DataTable, DataTable);
literal_component!(DatePicker, DatePicker);
literal_component!(Dialog, Dialog);
literal_component!(Direction, Direction);
literal_component!(Drawer, Drawer);
literal_component!(DropdownMenu, DropdownMenu);
literal_component!(Empty, Empty);
literal_component!(Field, Field);
literal_component!(HoverCard, HoverCard);
literal_component!(Input, Input);
literal_component!(InputGroup, InputGroup);
literal_component!(InputOtp, InputOtp);
literal_component!(Item, Item);
literal_component!(Kbd, Kbd);
literal_component!(Label, Label);
literal_component!(Marker, Marker);
literal_component!(Menubar, Menubar);
literal_component!(Message, Message);
literal_component!(MessageScroller, MessageScroller);
literal_component!(NativeSelect, NativeSelect);
literal_component!(NavigationMenu, NavigationMenu);
literal_component!(Pagination, Pagination);
literal_component!(Popover, Popover);
literal_component!(Progress, Progress);
literal_component!(RadioGroup, RadioGroup);
literal_component!(Resizable, Resizable);
literal_component!(ScrollArea, ScrollArea);
literal_component!(Select, Select);
literal_component!(Separator, Separator);
literal_component!(Sheet, Sheet);
literal_component!(Sidebar, Sidebar);
literal_component!(Skeleton, Skeleton);
literal_component!(Slider, Slider);
literal_component!(Sonner, Sonner);
literal_component!(Spinner, Spinner);
literal_component!(Switch, Switch);
literal_component!(Table, Table);
literal_component!(Tabs, Tabs);
literal_component!(Textarea, Textarea);
literal_component!(Toast, Toast);
literal_component!(Toggle, Toggle);
literal_component!(ToggleGroup, ToggleGroup);
literal_component!(Tooltip, Tooltip);
literal_component!(Typography, Typography);

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
