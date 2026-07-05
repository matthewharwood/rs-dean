use leptos::prelude::*;
use rs_dean_ui::HealthCard;

#[component]
fn Stories() -> impl IntoView {
    view! {
        <main class="stories-shell">
            <div class="stories-shell__inner">
                <header class="stories-header">
                    <p class="stories-header__eyebrow">
                        "Developer workbench"
                    </p>
                    <h1 class="stories-header__title">"rs-dean stories"</h1>
                </header>
                <section data-story-id="ui-health-card" class="story-frame">
                    <HealthCard
                        title="HealthCard"
                        body="A minimal shared component rendered through the same Leptos code path as the app."
                    />
                </section>
            </div>
        </main>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(Stories);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
