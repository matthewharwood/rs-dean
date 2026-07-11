use leptos::prelude::*;
#[cfg(test)]
use rs_dean_blocks::BLOCKS;
use rs_dean_blocks::{Block, BlockInstance, block_by_slug};
use rs_dean_ui::story_fixtures::*;
use rs_dean_ui::{
    ComponentDefinition, HealthCard, SHADCN_COMPONENTS, ShadcnComponentGallery, ThemeCycleButton,
    ThemeId, ThemeScope, TooltipModel, TooltipPlacement, UiStoryModelView,
    tooltip_effective_placement,
};

const STORIES_SHELL: &str = "min-h-screen bg-surface-1 px-m py-l text-text-1";
const STORIES_SHELL_ISOLATED: &str = "min-h-screen bg-surface-1 p-s text-text-1";
const STORIES_SHELL_BLOCK: &str = "min-h-screen bg-surface-1 text-text-1";
const STORIES_SHELL_INNER: &str = "mx-auto max-w-5xl";
const STORIES_SHELL_INNER_BLOCK: &str = "w-full";
const STORIES_HEADER: &str = "mb-m flex flex-col gap-s sm:flex-row sm:items-end sm:justify-between";
const STORIES_HEADER_COPY: &str = "grid gap-2xs";
const STORIES_EYEBROW: &str = "m-0 text-00 font-7 uppercase text-brand";
const STORIES_TITLE: &str = "m-0 text-3 font-7 text-text-1";
const STORIES_GRID: &str = "grid gap-m";
const STORY_FRAME: &str = "max-w-control";
const STORY_SECTION: &str = "grid gap-s";
const STORY_SECTION_HEADER: &str = "grid gap-2xs";
const STORY_SECTION_TITLE: &str = "m-0 text-2 font-7 leading-2 text-text-1";
const STORY_SECTION_BODY: &str = "m-0 max-w-reading text-0 leading-0 text-text-2";
const ALERT_STORY_GRID: &str = "grid gap-s md:grid-cols-2";
const TOOLTIP_STORY_STAGE_TOP: &str =
    "relative flex min-h-4xl w-full items-end justify-center overflow-visible p-s";
const TOOLTIP_STORY_STAGE_RIGHT: &str =
    "relative flex min-h-4xl w-full items-center justify-start overflow-visible p-s";
const TOOLTIP_STORY_STAGE_BOTTOM: &str =
    "relative flex min-h-4xl w-full items-start justify-center overflow-visible p-s";
const TOOLTIP_STORY_STAGE_LEFT: &str =
    "relative flex min-h-4xl w-full items-center justify-end overflow-visible p-s";
const TOOLTIP_STORY_STAGE_CLOSED: &str =
    "relative flex min-h-4xl w-full items-center justify-center overflow-visible p-s";
const POPOVER_STORY_STAGE: &str = "relative mb-xl pb-4xl";
const THEME_GALLERY: &str = "grid grid-cols-1 gap-s sm:grid-cols-2 lg:grid-cols-3";
const THEME_CARD: &str =
    "min-h-4xl rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const THEME_NAME: &str = "m-0 text-00 font-7 uppercase text-brand";
const THEME_BODY: &str = "m-0 mt-xs text-0 leading-0 text-text-2";
const THEME_SWATCH_ROW: &str = "mt-s flex gap-2xs";
const THEME_SWATCH: &str = "size-l rounded-field border border-border-subtle";

#[component]
fn Stories() -> AnyView {
    let isolated_story_id = isolated_story_id();
    let isolated_block = isolated_story_id
        .as_deref()
        .and_then(|story_id| story_id.strip_prefix("block-"))
        .and_then(block_by_slug);
    if let Some(definition) = isolated_block {
        let story_id = format!("block-{}", definition.slug);
        let story_id_for_data = story_id.clone();
        let instance = BlockInstance::fixture(definition);
        return view! {
            <main class=STORIES_SHELL_BLOCK data-story-shell="isolated-block">
                <div class=STORIES_SHELL_INNER_BLOCK>
                    <section id=story_id data-story-id=story_id_for_data class="w-full">
                        <Block instance />
                    </section>
                </div>
            </main>
        }
        .into_any();
    }

    let shell_class = if isolated_story_id.is_some() {
        STORIES_SHELL_ISOLATED
    } else {
        STORIES_SHELL
    };
    let shell_inner_class = STORIES_SHELL_INNER;
    let shell_mode = if isolated_story_id.is_some() {
        "isolated"
    } else {
        "catalog"
    };
    let isolated_style = isolated_story_style(isolated_story_id.as_deref());
    view! {
        <main class=shell_class data-story-shell=shell_mode>
            <style>{isolated_style}</style>
            <div class=shell_inner_class>
                <header class=STORIES_HEADER data-story-shell-header="true">
                    <div class=STORIES_HEADER_COPY>
                        <p class=STORIES_EYEBROW>
                            "Developer workbench"
                        </p>
                        <h1 class=STORIES_TITLE>"rs-dean stories"</h1>
                    </div>
                    <ThemeCycleButton />
                </header>
                <div class=STORIES_GRID data-story-grid="true">
                    <section id="ui-health-card" data-story-id="ui-health-card" class=STORY_FRAME>
                        <HealthCard
                            title="HealthCard"
                            body="A minimal shared component rendered through the same Leptos code path as the app."
                        />
                    </section>
                    {SHADCN_COMPONENTS
                        .into_iter()
                        .map(component_story_section)
                        .collect_view()}
                    <section data-story-id="shadcn-components" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"shadcn component catalog"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Every official shadcn component has a literal Rust widget constructor, a concrete typed model, a token-only Leptos component, and a Bevy primitive adapter over the same render nodes."
                            </p>
                        </header>
                        <ShadcnComponentGallery />
                    </section>
                    <section id="ui-theme-gallery" data-story-id="ui-theme-gallery" class=THEME_GALLERY>
                        {ThemeId::ALL.into_iter().map(theme_card).collect_view()}
                    </section>
                </div>
            </div>
        </main>
    }
    .into_any()
}

fn component_story_section(definition: ComponentDefinition) -> impl IntoView {
    let story_id = format!("ui-{}", definition.slug);
    let fixture_class = if definition.id == rs_dean_ui::UiComponentId::Accordion {
        STORY_FRAME
    } else {
        ALERT_STORY_GRID
    };
    let fixtures = ui_story_fixtures(definition.id);

    view! {
        <section id=story_id.clone() data-story-id=story_id class=STORY_SECTION>
            <header class=STORY_SECTION_HEADER>
                <h2 class=STORY_SECTION_TITLE>{definition.name}</h2>
                <p class=STORY_SECTION_BODY>{definition.summary}</p>
            </header>
            <div class=fixture_class>
                {fixtures.into_iter().map(story_fixture_view).collect_view()}
            </div>
        </section>
    }
}

fn story_fixture_view(fixture: UiStoryFixture) -> AnyView {
    let theme_id = fixture.theme_id;
    let themed = fixture.kind == UiStoryVariantKind::Themed;
    let tooltip_stage = match &fixture.model {
        UiStoryModel::Tooltip(model) => Some(tooltip_story_stage_class(model)),
        _ => None,
    };
    let popover_stage = matches!(&fixture.model, UiStoryModel::Popover(_));
    let component =
        view! { <UiStoryModelView model=fixture.model default_open=fixture.default_open /> }
            .into_any();

    let component = if themed {
        view! {
            <ThemeScope theme=theme_id>{component}</ThemeScope>
        }
        .into_any()
    } else {
        component
    };

    if let Some(stage_class) = tooltip_stage {
        view! { <div class=stage_class>{component}</div> }.into_any()
    } else if popover_stage {
        view! { <div class=POPOVER_STORY_STAGE>{component}</div> }.into_any()
    } else {
        component
    }
}

fn tooltip_story_stage_class(model: &TooltipModel) -> &'static str {
    if model.disabled || !model.default_open {
        return TOOLTIP_STORY_STAGE_CLOSED;
    }
    match tooltip_effective_placement(model) {
        TooltipPlacement::Top => TOOLTIP_STORY_STAGE_TOP,
        TooltipPlacement::Right => TOOLTIP_STORY_STAGE_RIGHT,
        TooltipPlacement::Bottom => TOOLTIP_STORY_STAGE_BOTTOM,
        TooltipPlacement::Left => TOOLTIP_STORY_STAGE_LEFT,
    }
}

#[cfg(target_arch = "wasm32")]
fn isolated_story_id() -> Option<String> {
    web_sys::window()
        .and_then(|window| window.location().search().ok())
        .and_then(|search| isolated_story_id_from_search(&search))
}

#[cfg(not(target_arch = "wasm32"))]
fn isolated_story_id() -> Option<String> {
    isolated_story_id_from_search("")
}

fn isolated_story_id_from_search(search: &str) -> Option<String> {
    let query = search.strip_prefix('?').unwrap_or(search);
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        (key == "story")
            .then_some(value)
            .and_then(valid_catalog_story_id)
    })
}

fn valid_catalog_story_id(value: &str) -> Option<String> {
    let (kind, slug) = if let Some(slug) = value.strip_prefix("ui-") {
        ("ui", slug)
    } else {
        ("block", value.strip_prefix("block-")?)
    };
    if slug.is_empty()
        || !slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return None;
    }
    let exists = if kind == "ui" {
        SHADCN_COMPONENTS
            .iter()
            .any(|definition| definition.slug == slug)
    } else {
        block_by_slug(slug).is_some()
    };
    exists.then(|| value.to_owned())
}

fn isolated_story_style(story_id: Option<&str>) -> String {
    story_id
        .map(|story_id| {
            format!(
                r#"
body {{ margin: 0; }}
[data-story-shell="isolated"] [data-story-shell-header] {{ display: none !important; }}
[data-story-shell="isolated"] [data-story-grid] {{ gap: 0; }}
[data-story-shell="isolated"] [data-story-id]:not([data-story-id="{story_id}"]) {{ display: none !important; }}
[data-story-shell="isolated"] [data-story-id="{story_id}"] > header {{ display: none !important; }}
"#
            )
        })
        .unwrap_or_default()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isolated_story_id_accepts_catalog_story_query() {
        assert_eq!(
            isolated_story_id_from_search("?story=ui-button"),
            Some("ui-button".to_owned())
        );
        assert_eq!(
            isolated_story_id_from_search("foo=bar&story=ui-alert-dialog"),
            Some("ui-alert-dialog".to_owned())
        );
        assert_eq!(
            isolated_story_id_from_search(
                "?story=block-marketing-sections-heroes-01-simple-centered"
            ),
            Some("block-marketing-sections-heroes-01-simple-centered".to_owned())
        );
    }

    #[test]
    fn every_block_catalog_entry_has_an_isolated_story_route() {
        for definition in BLOCKS {
            let story_id = format!("block-{}", definition.slug);
            let search = format!("?story={story_id}");
            assert_eq!(isolated_story_id_from_search(&search), Some(story_id));
        }
    }

    #[test]
    fn isolated_story_id_rejects_non_catalog_or_malformed_values() {
        assert_eq!(isolated_story_id_from_search("?story=ui-health-card"), None);
        assert_eq!(isolated_story_id_from_search("?story=button"), None);
        assert_eq!(isolated_story_id_from_search("?story=ui-Button"), None);
        assert_eq!(isolated_story_id_from_search("?story=ui-button%20"), None);
    }

    #[test]
    fn isolated_story_style_hides_everything_except_requested_story() {
        let style = isolated_story_style(Some("ui-button"));
        assert!(style.contains(r#"[data-story-shell-header]"#));
        assert!(style.contains(r#"[data-story-id]:not([data-story-id="ui-button"])"#));
        assert!(style.contains(r#"[data-story-id="ui-button"] > header"#));
    }

    #[test]
    fn stories_do_not_render_bad_fixtures() {
        let source = include_str!("main.rs");
        let bad_fixture_name = ["invalid", "_"].concat();
        let bad_fixture_binding = ["model=", "invalid", "_"].concat();

        assert!(!source.contains(&bad_fixture_name));
        assert!(!source.contains(&bad_fixture_binding));
    }

    #[test]
    fn tooltip_stories_reserve_space_for_the_effective_placement() {
        let right =
            TooltipModel::new("Sync", "sync", "Sync now.").with_placement(TooltipPlacement::Right);
        let loading = right.clone().loading();
        let disabled = right.clone().disabled();

        assert_eq!(tooltip_story_stage_class(&right), TOOLTIP_STORY_STAGE_RIGHT);
        assert_eq!(
            tooltip_story_stage_class(&loading),
            TOOLTIP_STORY_STAGE_BOTTOM
        );
        assert_eq!(
            tooltip_story_stage_class(&disabled),
            TOOLTIP_STORY_STAGE_CLOSED
        );
    }
}
