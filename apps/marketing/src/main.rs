use leptos::prelude::*;
use rs_dean_ui::HealthCard;

#[cfg(target_arch = "wasm32")]
use rs_dean_state::ensure_durable_snapshot;

const APP_SHELL: &str = "rs-dean-marketing-bg min-h-screen px-6 py-8 text-slate-50";
const APP_SHELL_INNER: &str = "mx-auto grid max-w-4xl gap-6";
const HERO: &str = "grid gap-3";
const HERO_EYEBROW: &str = "m-0 text-sm font-bold uppercase tracking-widest text-cyan-300";
const HERO_TITLE: &str = "m-0 text-5xl font-bold leading-none md:text-6xl";
const HERO_BODY: &str = "m-0 max-w-2xl text-base leading-7 text-slate-300";

#[component]
fn App() -> impl IntoView {
    view! {
        <main class=APP_SHELL>
            <div class=APP_SHELL_INNER>
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
