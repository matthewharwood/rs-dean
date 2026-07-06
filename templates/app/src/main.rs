use leptos::prelude::*;
use rs_dean_schema::{AppSnapshot, validate_snapshot};
use rs_dean_state::{APP_SNAPSHOT_KEY, DB_NAME, SNAPSHOTS_STORE};

#[component]
fn App() -> impl IntoView {
    let snapshot = AppSnapshot::default();
    validate_snapshot(&snapshot).expect("generated app snapshot validates");
    let lesson_count = snapshot.progress.len();
    let first_lesson = snapshot
        .progress
        .first()
        .map(|progress| progress.id.clone())
        .unwrap_or_else(|| "none".to_owned());

    view! {
        <main class="generated-shell">
            <div class="generated-shell__inner">
                <p class="generated-shell__eyebrow">
                    "rs-dean generated app"
                </p>
                <h1 class="generated-shell__title">"{{name}}"</h1>
                <p class="generated-shell__body">
                    "This app starts with a validated state snapshot and the shared durable storage contract."
                </p>
                <dl class="generated-state">
                    <div>
                        <dt>"database"</dt>
                        <dd>{DB_NAME}</dd>
                    </div>
                    <div>
                        <dt>"store"</dt>
                        <dd>{SNAPSHOTS_STORE}</dd>
                    </div>
                    <div>
                        <dt>"key"</dt>
                        <dd>{APP_SNAPSHOT_KEY}</dd>
                    </div>
                    <div>
                        <dt>"lessons"</dt>
                        <dd>{lesson_count}</dd>
                    </div>
                    <div>
                        <dt>"first"</dt>
                        <dd>{first_lesson}</dd>
                    </div>
                </dl>
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
