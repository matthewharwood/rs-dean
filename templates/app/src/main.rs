use leptos::prelude::*;
use rs_dean_schema::{AppSnapshot, validate_snapshot};
use rs_dean_state::{APP_SNAPSHOT_KEY, DB_NAME, SNAPSHOTS_STORE};
use rs_dean_ui::{HealthCard, ThemeCycleButton};

#[cfg(target_arch = "wasm32")]
use rs_dean_state::ensure_durable_snapshot;

const GENERATED_SHELL: &str = "min-h-screen bg-surface-1 px-6 py-8 text-text-1";
const GENERATED_SHELL_INNER: &str = "mx-auto grid max-w-3xl gap-6";
const GENERATED_TOPBAR: &str = "flex items-center justify-end";
const GENERATED_HEADER: &str = "grid gap-3";
const GENERATED_EYEBROW: &str = "m-0 text-sm font-bold uppercase text-brand";
const GENERATED_TITLE: &str = "m-0 text-5 font-bold leading-none text-text-1 md:text-6";
const GENERATED_BODY: &str = "m-0 max-w-2xl text-base leading-7 text-text-2";
const GENERATED_STATE: &str = "mt-6 grid max-w-2xl grid-cols-1 gap-3 sm:grid-cols-2";
const GENERATED_STATE_ITEM: &str =
    "min-w-0 rounded-box border border-border-subtle bg-surface-elevated p-4";
const GENERATED_STATE_TERM: &str = "m-0 text-xs font-bold uppercase text-text-muted";
const GENERATED_STATE_VALUE: &str = "m-0 mt-2 break-words text-sm text-text-1";

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
        <main class=GENERATED_SHELL>
            <div class=GENERATED_SHELL_INNER>
                <div class=GENERATED_TOPBAR>
                    <ThemeCycleButton />
                </div>
                <header class=GENERATED_HEADER>
                    <p class=GENERATED_EYEBROW>
                        "rs-dean generated app"
                    </p>
                    <h1 class=GENERATED_TITLE>"{{name}}"</h1>
                    <p class=GENERATED_BODY>
                        "This app starts with a validated state snapshot and the shared durable storage contract."
                    </p>
                </header>
                <HealthCard
                    title="Shared UI contract"
                    body="The scaffold uses the same token-backed Leptos components as the repo apps."
                />
                <dl class=GENERATED_STATE>
                    <div class=GENERATED_STATE_ITEM>
                        <dt class=GENERATED_STATE_TERM>"database"</dt>
                        <dd class=GENERATED_STATE_VALUE>{DB_NAME}</dd>
                    </div>
                    <div class=GENERATED_STATE_ITEM>
                        <dt class=GENERATED_STATE_TERM>"store"</dt>
                        <dd class=GENERATED_STATE_VALUE>{SNAPSHOTS_STORE}</dd>
                    </div>
                    <div class=GENERATED_STATE_ITEM>
                        <dt class=GENERATED_STATE_TERM>"key"</dt>
                        <dd class=GENERATED_STATE_VALUE>{APP_SNAPSHOT_KEY}</dd>
                    </div>
                    <div class=GENERATED_STATE_ITEM>
                        <dt class=GENERATED_STATE_TERM>"lessons"</dt>
                        <dd class=GENERATED_STATE_VALUE>{lesson_count}</dd>
                    </div>
                    <div class=GENERATED_STATE_ITEM>
                        <dt class=GENERATED_STATE_TERM>"first"</dt>
                        <dd class=GENERATED_STATE_VALUE>{first_lesson}</dd>
                    </div>
                </dl>
            </div>
        </main>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::task::spawn_local(async {
        if let Err(error) = ensure_durable_snapshot().await {
            leptos::logging::error!("failed to hydrate persistent app state: {error}");
        }
    });
    leptos::mount::mount_to_body(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
