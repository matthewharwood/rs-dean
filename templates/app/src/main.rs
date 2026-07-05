use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="generated-shell">
            <div class="generated-shell__inner">
                <p class="generated-shell__eyebrow">
                    "rs-dean generated app"
                </p>
                <h1 class="generated-shell__title">"{{name}}"</h1>
                <p class="generated-shell__body">
                    "This app was generated from templates/app and is ready for Leptos CSR work."
                </p>
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
