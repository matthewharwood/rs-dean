use leptos::prelude::*;
use rs_dean_ui::{HealthCard, ThemeCycleButton};

#[cfg(target_arch = "wasm32")]
use rs_dean_state::ensure_durable_snapshot;

const APP_SHELL: &str = "min-h-screen bg-surface-1 px-6 py-8 text-text-1";
const APP_SHELL_INNER: &str = "mx-auto grid max-w-4xl gap-6";
const APP_TOPBAR: &str = "flex items-center justify-end";
const HERO: &str = "grid gap-3";
const HERO_EYEBROW: &str = "m-0 text-sm font-bold uppercase text-brand";
const HERO_TITLE: &str = "m-0 text-5 font-bold leading-none text-text-1 md:text-6";
const HERO_BODY: &str = "m-0 max-w-2xl text-base leading-7 text-text-2";

#[component]
fn App() -> impl IntoView {
    view! {
        <main class=APP_SHELL>
            <div class=APP_SHELL_INNER>
                <div class=APP_TOPBAR>
                    <ThemeCycleButton />
                </div>
                <header class=HERO>
                    <p class=HERO_EYEBROW>
                        "Leptos marketing"
                    </p>
                    <h1 class=HERO_TITLE>"Hello world"</h1>
                    <p class=HERO_BODY>
                        "Static GitHub Pages output with durable local state."
                    </p>
                </header>
                <HealthCard
                    title="Marketing contract"
                    body="Leptos renders this hello-world marketing surface and hydrates durable state on boot."
                />
            </div>
        </main>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::task::spawn_local(async {
        if let Err(error) = ensure_durable_snapshot().await {
            leptos::logging::error!("failed to hydrate persistent marketing state: {error}");
        }
    });
    leptos::mount::mount_to_body(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
