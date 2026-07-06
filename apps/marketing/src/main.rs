use leptos::prelude::*;
use rs_dean_ui::HealthCard;

#[cfg(target_arch = "wasm32")]
use rs_dean_state::ensure_durable_snapshot;

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="app-shell">
            <div class="app-shell__inner">
                <header class="hero">
                    <p class="hero__eyebrow">
                        "Leptos marketing"
                    </p>
                    <h1 class="hero__title">"Hello world"</h1>
                    <p class="hero__body">
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
