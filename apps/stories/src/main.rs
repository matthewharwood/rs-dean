use leptos::prelude::*;
use rs_dean_ui::{HealthCard, ThemeCycleButton, ThemeId, ThemeScope};

const STORIES_SHELL: &str = "min-h-screen bg-surface-1 px-6 py-8 text-text-1";
const STORIES_SHELL_INNER: &str = "mx-auto max-w-5xl";
const STORIES_HEADER: &str = "mb-6 flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between";
const STORIES_HEADER_COPY: &str = "grid gap-2";
const STORIES_EYEBROW: &str = "m-0 text-sm font-bold uppercase text-brand";
const STORIES_TITLE: &str = "m-0 text-3xl font-bold text-text-1";
const STORIES_GRID: &str = "grid gap-6";
const STORY_FRAME: &str = "max-w-md";
const THEME_GALLERY: &str = "grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3";
const THEME_CARD: &str =
    "min-h-40 rounded-box border border-border-subtle bg-surface-1 p-4 text-text-1 shadow-1";
const THEME_NAME: &str = "m-0 text-sm font-bold uppercase text-brand";
const THEME_BODY: &str = "m-0 mt-3 text-sm leading-6 text-text-2";
const THEME_SWATCH_ROW: &str = "mt-4 flex gap-2";
const THEME_SWATCH: &str = "h-8 w-8 rounded-field border border-border-subtle";

#[component]
fn Stories() -> impl IntoView {
    view! {
        <main class=STORIES_SHELL>
            <div class=STORIES_SHELL_INNER>
                <header class=STORIES_HEADER>
                    <div class=STORIES_HEADER_COPY>
                        <p class=STORIES_EYEBROW>
                            "Developer workbench"
                        </p>
                        <h1 class=STORIES_TITLE>"rs-dean stories"</h1>
                    </div>
                    <ThemeCycleButton />
                </header>
                <div class=STORIES_GRID>
                    <section data-story-id="ui-health-card" class=STORY_FRAME>
                        <HealthCard
                            title="HealthCard"
                            body="A minimal shared component rendered through the same Leptos code path as the app."
                        />
                    </section>
                    <section data-story-id="ui-theme-gallery" class=THEME_GALLERY>
                        {ThemeId::ALL.into_iter().map(theme_card).collect_view()}
                    </section>
                </div>
            </div>
        </main>
    }
}

fn theme_card(theme: ThemeId) -> impl IntoView {
    view! {
        <ThemeScope theme=theme>
            <article class=THEME_CARD>
                <h2 class=THEME_NAME>{theme.label()}</h2>
                <p class=THEME_BODY>
                    "Semantic Tailwind tokens resolve from the shared Rust palette."
                </p>
                <div class=THEME_SWATCH_ROW aria-hidden="true">
                    <span class=format!("{THEME_SWATCH} bg-surface-1")></span>
                    <span class=format!("{THEME_SWATCH} bg-surface-2")></span>
                    <span class=format!("{THEME_SWATCH} bg-brand")></span>
                    <span class=format!("{THEME_SWATCH} bg-success")></span>
                    <span class=format!("{THEME_SWATCH} bg-danger")></span>
                </div>
            </article>
        </ThemeScope>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(Stories);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
