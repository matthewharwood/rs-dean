use leptos::prelude::*;
use rs_dean_curriculum::starter_curriculum;
use rs_dean_ui::HealthCard;

#[component]
fn App() -> impl IntoView {
    let curriculum = starter_curriculum();

    view! {
        <main class="app-shell">
            <div class="app-shell__inner">
                <header class="hero">
                    <p class="hero__eyebrow">
                        "Rust WASM scaffold"
                    </p>
                    <h1 class="hero__title">"rs-dean"</h1>
                    <p class="hero__body">
                        "Leptos owns the DOM shell. Bevy owns canvas scenes. IndexedDB remains the source of truth."
                    </p>
                </header>
                <HealthCard
                    title="Stack contract"
                    body="Rust 2024, Leptos CSR, Bevy 0.19, IDB-first state, and static GitHub Pages output."
                />
                <section class="panel">
                    <h2 class="panel__title">{curriculum.title}</h2>
                    <p class="panel__body">
                        {format!("{} starter lesson(s) wired through a native Rust crate.", curriculum.lessons.len())}
                    </p>
                </section>
            </div>
        </main>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
