use leptos::prelude::*;
use rs_dean_ui::story_fixtures::*;
use rs_dean_ui::{
    Accordion, Alert, AlertDialog, AspectRatio, Attachment, Avatar, Badge, Breadcrumb, Bubble,
    Button, ButtonGroup, Calendar, Card, Carousel, Chart, Checkbox, Collapsible, Combobox, Command,
    ComponentDefinition, ContextMenu, DataTable, DatePicker, Dialog, Direction, Drawer,
    DropdownMenu, Empty, Field, HealthCard, HoverCard, Input, InputGroup, InputOtp, Item, Kbd,
    Label, Marker, Menubar, Message, MessageScroller, NativeSelect, NavigationMenu, Pagination,
    Popover, Progress, RadioGroup, Resizable, SHADCN_COMPONENTS, ScrollArea, Select, Separator,
    ShadcnComponentGallery, Sheet, Sidebar, Skeleton, Slider, Sonner, Spinner, Switch, Table, Tabs,
    Textarea, ThemeCycleButton, ThemeId, ThemeScope, Toast, Toggle, ToggleGroup, Tooltip,
    Typography,
};

const STORIES_SHELL: &str = "min-h-screen bg-surface-1 px-m py-l text-text-1";
const STORIES_SHELL_ISOLATED: &str = "min-h-screen bg-surface-1 p-s text-text-1";
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
    let isolated_story_id = isolated_story_id();
    let shell_class = if isolated_story_id.is_some() {
        STORIES_SHELL_ISOLATED
    } else {
        STORIES_SHELL
    };
    let shell_mode = if isolated_story_id.is_some() {
        "isolated"
    } else {
        "catalog"
    };
    let isolated_style = isolated_story_style(isolated_story_id.as_deref());

    view! {
        <main class=shell_class data-story-shell=shell_mode>
            <style>{isolated_style}</style>
            <div class=STORIES_SHELL_INNER>
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
    let component = leptos_story_model_view(fixture.model, fixture.default_open);

    if themed {
        view! {
            <ThemeScope theme=theme_id>{component}</ThemeScope>
        }
        .into_any()
    } else {
        component
    }
}

fn leptos_story_model_view(model: UiStoryModel, default_open: bool) -> AnyView {
    match model {
        UiStoryModel::Accordion(model) => view! {
            <Accordion
                items=model.items
                mode=model.mode
                default_open=model.default_open
            />
        }
        .into_any(),
        UiStoryModel::Alert(model) => view! { <Alert model=model /> }.into_any(),
        UiStoryModel::AlertDialog(model) => {
            view! { <AlertDialog model=model default_open=default_open /> }.into_any()
        }
        UiStoryModel::AspectRatio(model) => view! { <AspectRatio model=model /> }.into_any(),
        UiStoryModel::Attachment(model) => view! { <Attachment model=model /> }.into_any(),
        UiStoryModel::Avatar(model) => view! { <Avatar model=model /> }.into_any(),
        UiStoryModel::Badge(model) => view! { <Badge model=model /> }.into_any(),
        UiStoryModel::Breadcrumb(model) => view! { <Breadcrumb model=model /> }.into_any(),
        UiStoryModel::Bubble(model) => view! { <Bubble model=model /> }.into_any(),
        UiStoryModel::Button(model) => view! { <Button model=model /> }.into_any(),
        UiStoryModel::ButtonGroup(model) => view! { <ButtonGroup model=model /> }.into_any(),
        UiStoryModel::Calendar(model) => view! { <Calendar model=model /> }.into_any(),
        UiStoryModel::Card(model) => view! { <Card model=model /> }.into_any(),
        UiStoryModel::Carousel(model) => view! { <Carousel model=model /> }.into_any(),
        UiStoryModel::Chart(model) => view! { <Chart model=model /> }.into_any(),
        UiStoryModel::Checkbox(model) => view! { <Checkbox model=model /> }.into_any(),
        UiStoryModel::Collapsible(model) => view! { <Collapsible model=model /> }.into_any(),
        UiStoryModel::Combobox(model) => view! { <Combobox model=model /> }.into_any(),
        UiStoryModel::Command(model) => view! { <Command model=model /> }.into_any(),
        UiStoryModel::ContextMenu(model) => view! { <ContextMenu model=model /> }.into_any(),
        UiStoryModel::DataTable(model) => view! { <DataTable model=model /> }.into_any(),
        UiStoryModel::DatePicker(model) => view! { <DatePicker model=model /> }.into_any(),
        UiStoryModel::Dialog(model) => view! { <Dialog model=model /> }.into_any(),
        UiStoryModel::Direction(model) => view! { <Direction model=model /> }.into_any(),
        UiStoryModel::Drawer(model) => view! { <Drawer model=model /> }.into_any(),
        UiStoryModel::DropdownMenu(model) => view! { <DropdownMenu model=model /> }.into_any(),
        UiStoryModel::Empty(model) => view! { <Empty model=model /> }.into_any(),
        UiStoryModel::Field(model) => view! { <Field model=model /> }.into_any(),
        UiStoryModel::HoverCard(model) => view! { <HoverCard model=model /> }.into_any(),
        UiStoryModel::Input(model) => view! { <Input model=model /> }.into_any(),
        UiStoryModel::InputGroup(model) => view! { <InputGroup model=model /> }.into_any(),
        UiStoryModel::InputOtp(model) => view! { <InputOtp model=model /> }.into_any(),
        UiStoryModel::Item(model) => view! { <Item model=model /> }.into_any(),
        UiStoryModel::Kbd(model) => view! { <Kbd model=model /> }.into_any(),
        UiStoryModel::Label(model) => view! { <Label model=model /> }.into_any(),
        UiStoryModel::Marker(model) => view! { <Marker model=model /> }.into_any(),
        UiStoryModel::Menubar(model) => view! { <Menubar model=model /> }.into_any(),
        UiStoryModel::Message(model) => view! { <Message model=model /> }.into_any(),
        UiStoryModel::MessageScroller(model) => {
            view! { <MessageScroller model=model /> }.into_any()
        }
        UiStoryModel::NativeSelect(model) => view! { <NativeSelect model=model /> }.into_any(),
        UiStoryModel::NavigationMenu(model) => view! { <NavigationMenu model=model /> }.into_any(),
        UiStoryModel::Pagination(model) => view! { <Pagination model=model /> }.into_any(),
        UiStoryModel::Popover(model) => view! { <Popover model=model /> }.into_any(),
        UiStoryModel::Progress(model) => view! { <Progress model=model /> }.into_any(),
        UiStoryModel::RadioGroup(model) => view! { <RadioGroup model=model /> }.into_any(),
        UiStoryModel::Resizable(model) => view! { <Resizable model=model /> }.into_any(),
        UiStoryModel::ScrollArea(model) => view! { <ScrollArea model=model /> }.into_any(),
        UiStoryModel::Select(model) => view! { <Select model=model /> }.into_any(),
        UiStoryModel::Separator(model) => view! { <Separator model=model /> }.into_any(),
        UiStoryModel::Sheet(model) => view! { <Sheet model=model /> }.into_any(),
        UiStoryModel::Sidebar(model) => view! { <Sidebar model=model /> }.into_any(),
        UiStoryModel::Skeleton(model) => view! { <Skeleton model=model /> }.into_any(),
        UiStoryModel::Slider(model) => view! { <Slider model=model /> }.into_any(),
        UiStoryModel::Sonner(model) => view! { <Sonner model=model /> }.into_any(),
        UiStoryModel::Spinner(model) => view! { <Spinner model=model /> }.into_any(),
        UiStoryModel::Switch(model) => view! { <Switch model=model /> }.into_any(),
        UiStoryModel::Table(model) => view! { <Table model=model /> }.into_any(),
        UiStoryModel::Tabs(model) => view! { <Tabs model=model /> }.into_any(),
        UiStoryModel::Textarea(model) => view! { <Textarea model=model /> }.into_any(),
        UiStoryModel::Toast(model) => view! { <Toast model=model /> }.into_any(),
        UiStoryModel::Toggle(model) => view! { <Toggle model=model /> }.into_any(),
        UiStoryModel::ToggleGroup(model) => view! { <ToggleGroup model=model /> }.into_any(),
        UiStoryModel::Tooltip(model) => view! { <Tooltip model=model /> }.into_any(),
        UiStoryModel::Typography(model) => view! { <Typography model=model /> }.into_any(),
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
    let slug = value.strip_prefix("ui-")?;
    if slug.is_empty()
        || !slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return None;
    }
    SHADCN_COMPONENTS
        .iter()
        .any(|definition| definition.slug == slug)
        .then(|| value.to_owned())
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
}
