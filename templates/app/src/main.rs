use leptos::prelude::*;
use rs_dean_schema::{AppSnapshot, validate_snapshot};
use rs_dean_state::{APP_SNAPSHOT_KEY, DB_NAME, SNAPSHOTS_STORE};

#[cfg(target_arch = "wasm32")]
use rs_dean_state::ensure_durable_snapshot;

const GENERATED_SHELL: &str = "min-h-screen bg-teal-50 px-6 py-8 text-slate-950";
const GENERATED_SHELL_INNER: &str = "mx-auto max-w-3xl";
const GENERATED_EYEBROW: &str =
    "m-0 text-sm font-bold uppercase tracking-widest text-emerald-700";
const GENERATED_TITLE: &str = "m-0 mt-3 text-5xl font-bold leading-none md:text-6xl";
const GENERATED_BODY: &str = "m-0 mt-4 max-w-2xl text-base leading-7 text-slate-600";
const GENERATED_STATE: &str = "mt-6 grid max-w-2xl grid-cols-1 gap-3 sm:grid-cols-2";
const GENERATED_STATE_ITEM: &str = "min-w-0 rounded-lg border border-slate-200 bg-white p-4";
const GENERATED_STATE_TERM: &str = "m-0 text-xs font-bold uppercase text-slate-500";
const GENERATED_STATE_VALUE: &str = "m-0 mt-2 break-words text-sm text-slate-950";

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
                <p class=GENERATED_EYEBROW>
                    "rs-dean generated app"
                </p>
                <h1 class=GENERATED_TITLE>"{{name}}"</h1>
                <p class=GENERATED_BODY>
                    "This app starts with a validated state snapshot and the shared durable storage contract."
                </p>
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
