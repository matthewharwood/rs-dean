use leptos::prelude::*;
use rs_dean_ui::HealthCard;

const STORIES_SHELL: &str = "min-h-screen bg-slate-50 px-6 py-8 text-slate-950";
const STORIES_SHELL_INNER: &str = "mx-auto max-w-5xl";
const STORIES_HEADER: &str = "mb-6";
const STORIES_EYEBROW: &str = "m-0 text-sm font-bold uppercase tracking-widest text-sky-700";
const STORIES_TITLE: &str = "m-0 mt-2 text-3xl font-bold";
const STORY_FRAME: &str = "max-w-md";

#[component]
fn Stories() -> impl IntoView {
    view! {
        <main class=STORIES_SHELL>
            <div class=STORIES_SHELL_INNER>
                <header class=STORIES_HEADER>
                    <p class=STORIES_EYEBROW>
                        "Developer workbench"
                    </p>
                    <h1 class=STORIES_TITLE>"rs-dean stories"</h1>
                </header>
                <section data-story-id="ui-health-card" class=STORY_FRAME>
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
