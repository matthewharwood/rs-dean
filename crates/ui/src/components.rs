use leptos::prelude::*;

use crate::{ThemeChoice, ThemeId};

const HEALTH_CARD: &str =
    "rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-2";
const HEALTH_CARD_EYEBROW: &str = "m-0 text-00 font-7 uppercase text-brand";
const HEALTH_CARD_TITLE: &str = "m-0 mt-2xs text-1 font-7 text-text-1";
const HEALTH_CARD_BODY: &str = "m-0 mt-xs text-0 leading-0 text-text-2";
const THEME_BUTTON: &str = "inline-flex items-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const THEME_BUTTON_LABEL: &str = "text-text-muted";
const THEME_SCOPE: &str = "contents";

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
