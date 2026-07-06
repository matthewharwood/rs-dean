use leptos::prelude::*;

use crate::{ThemeChoice, ThemeId, UiBlock, UiBlockTone, UiComponentId, component_spec};

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
const BLOCK_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const BLOCK_DETAIL: &str = "m-0 text-0 leading-0 text-text-2";

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
        </article>
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
