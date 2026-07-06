use leptos::prelude::*;
use rs_dean_ui::{
    Accordion, AccordionItem, AccordionMode, Alert, AlertAction, AlertDensity, AlertModel,
    AlertTone, HealthCard, ShadcnComponentGallery, ThemeCycleButton, ThemeId, ThemeScope,
};

const STORIES_SHELL: &str = "min-h-screen bg-surface-1 px-m py-l text-text-1";
const STORIES_SHELL_INNER: &str = "mx-auto max-w-5xl";
const STORIES_HEADER: &str = "mb-m flex flex-col gap-s sm:flex-row sm:items-end sm:justify-between";
const STORIES_HEADER_COPY: &str = "grid gap-2xs";
const STORIES_EYEBROW: &str = "m-0 text-00 font-7 uppercase text-brand";
const STORIES_TITLE: &str = "m-0 text-3 font-7 text-text-1";
const STORIES_GRID: &str = "grid gap-m";
const STORY_FRAME: &str = "max-w-md";
const STORY_SECTION: &str = "grid gap-s";
const STORY_SECTION_HEADER: &str = "grid gap-2xs";
const STORY_SECTION_TITLE: &str = "m-0 text-2 font-7 leading-2 text-text-1";
const STORY_SECTION_BODY: &str = "m-0 max-w-2xl text-0 leading-0 text-text-2";
const ALERT_STORY_GRID: &str = "grid gap-s md:grid-cols-2";
const THEME_GALLERY: &str = "grid grid-cols-1 gap-s sm:grid-cols-2 lg:grid-cols-3";
const THEME_CARD: &str =
    "min-h-4xl rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const THEME_NAME: &str = "m-0 text-00 font-7 uppercase text-brand";
const THEME_BODY: &str = "m-0 mt-xs text-0 leading-0 text-text-2";
const THEME_SWATCH_ROW: &str = "mt-s flex gap-2xs";
const THEME_SWATCH: &str = "size-l rounded-field border border-border-subtle";

#[component]
fn Stories() -> impl IntoView {
    view! {
        <main class=STORIES_SHELL>
            <div class=STORIES_SHELL_INNER>
                <header class=STORIES_HEADER>
                    <div class=STORIES_HEADER_COPY>
                        <p class=STORIES_EYEBROW>
                            "Developer workbench"
                        </p>
                        <h1 class=STORIES_TITLE>"rs-dean stories"</h1>
                    </div>
                    <ThemeCycleButton />
                </header>
                <div class=STORIES_GRID>
                    <section data-story-id="ui-health-card" class=STORY_FRAME>
                        <HealthCard
                            title="HealthCard"
                            body="A minimal shared component rendered through the same Leptos code path as the app."
                        />
                    </section>
                    <section data-story-id="ui-accordion" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Accordion"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 01 implemented as a real token-only Leptos component backed by shared Rust state transitions."
                            </p>
                        </header>
                        <div class=STORY_FRAME>
                            <Accordion
                                items=accordion_story_items()
                                mode=AccordionMode::Multiple
                                default_open=vec!["tokens".to_owned(), "bevy".to_owned()]
                            />
                        </div>
                    </section>
                    <section data-story-id="ui-alert" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Alert"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 02 implemented as a stateless callout backed by a validated shared Rust model and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Alert model=default_alert_story_model() />
                            <Alert model=dense_warning_alert_story_model() />
                            <Alert model=loading_alert_story_model() />
                            <Alert model=disabled_error_alert_story_model() />
                            <Alert model=invalid_alert_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Alert model=themed_alert_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="shadcn-components" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"shadcn component catalog"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Every official shadcn component has a literal Rust widget constructor, a token-only Leptos component, and a Bevy primitive adapter over the same slots."
                            </p>
                        </header>
                        <ShadcnComponentGallery />
                    </section>
                    <section data-story-id="ui-theme-gallery" class=THEME_GALLERY>
                        {ThemeId::ALL.into_iter().map(theme_card).collect_view()}
                    </section>
                </div>
            </div>
        </main>
    }
}

fn accordion_story_items() -> Vec<AccordionItem> {
    vec![
        AccordionItem::new(
            "tokens",
            "Token-only styling",
            "The trigger, content, focus ring, border, and text all use rs-dean-ui Tailwind token utilities.",
        ),
        AccordionItem::new(
            "local-state",
            "Renderer-local state",
            "Accordion open state is ephemeral by default and does not bypass the durable app state layer.",
        )
        .disabled(),
        AccordionItem::new(
            "bevy",
            "Shared Rust contract",
            "The same model exposes render nodes that can be consumed outside the Leptos DOM path.",
        ),
    ]
}

fn default_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Build completed",
        "The design-token bundle is ready for review in the shared UI crate.",
    )
    .with_tone(AlertTone::Success)
    .with_action(AlertAction::new("Open report", "open-report"))
}

fn dense_warning_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Token drift detected",
        "Two stories reference the same semantic tone through different framework paths.",
    )
    .with_tone(AlertTone::Warning)
    .with_density(AlertDensity::Dense)
    .with_action(AlertAction::new("Review", "review-token-drift"))
}

fn loading_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Publishing artifacts",
        "The static Pages bundle is still being generated by the one-pass gate.",
    )
    .with_tone(AlertTone::Info)
    .with_action(AlertAction::new("View run", "view-run"))
    .loading()
}

fn disabled_error_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Destructive action blocked",
        "The app requires a durable state handoff before this action can be enabled.",
    )
    .with_tone(AlertTone::Destructive)
    .with_action(AlertAction::new("Retry", "retry-action").disabled())
    .disabled()
}

fn invalid_alert_story_model() -> AlertModel {
    AlertModel::new(
        "",
        "The validation boundary renders an invalid state instead of accepting empty title copy.",
    )
}

fn themed_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Theme scoped alert",
        "The same semantic tokens resolve through a nested Dracula theme scope.",
    )
    .with_tone(AlertTone::Default)
    .with_action(AlertAction::new("Inspect", "inspect-theme"))
}

fn theme_card(theme: ThemeId) -> impl IntoView {
    view! {
        <ThemeScope theme=theme>
            <article class=THEME_CARD>
                <h2 class=THEME_NAME>{theme.label()}</h2>
                <p class=THEME_BODY>
                    "Semantic Tailwind tokens resolve from the shared Rust palette."
                </p>
                <div class=THEME_SWATCH_ROW aria-hidden="true">
                    <span class=format!("{THEME_SWATCH} bg-surface-1")></span>
                    <span class=format!("{THEME_SWATCH} bg-surface-2")></span>
                    <span class=format!("{THEME_SWATCH} bg-brand")></span>
                    <span class=format!("{THEME_SWATCH} bg-success")></span>
                    <span class=format!("{THEME_SWATCH} bg-danger")></span>
                </div>
            </article>
        </ThemeScope>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(Stories);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
