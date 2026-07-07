use leptos::prelude::*;
use rs_dean_ui::{
    Accordion, AccordionItem, AccordionMode, Alert, AlertAction, AlertDensity, AlertDialog,
    AlertDialogButton, AlertDialogModel, AlertDialogSize, AlertModel, AlertTone, AspectRatio,
    AspectRatioFit, AspectRatioModel, Attachment, AttachmentAction, AttachmentKind,
    AttachmentModel, Avatar, AvatarModel, AvatarSize, Badge, BadgeModel, BadgeSize, BadgeTone,
    BadgeVariant, Breadcrumb, BreadcrumbDensity, BreadcrumbEntry, BreadcrumbModel, Bubble,
    BubbleAction, BubbleModel, BubbleSide, Button, ButtonGroup, ButtonGroupItem, ButtonGroupModel,
    ButtonGroupOrientation, ButtonKind, ButtonModel, ButtonSize, ButtonVariant, Calendar,
    CalendarDate, CalendarModel, CalendarRange, CalendarSelectionMode, Card, CardAction,
    CardDensity, CardModel, CardVariant, Carousel, CarouselDensity, CarouselModel, CarouselSlide,
    Chart, ChartDensity, ChartModel, ChartSeries, ChartTone, Checkbox, CheckboxChecked,
    CheckboxDensity, CheckboxModel, Collapsible, CollapsibleDensity, CollapsibleModel, Combobox,
    ComboboxDensity, ComboboxModel, ComboboxOption, Command, CommandDensity, CommandGroup,
    CommandItem, CommandModel, ContextMenu, ContextMenuAction, ContextMenuDensity,
    ContextMenuEntry, ContextMenuModel, ContextMenuSubmenu, DataTable, DataTableColumn,
    DataTableDensity, DataTableModel, DataTableRow, DataTableSortDirection, HealthCard,
    ShadcnComponentGallery, ThemeCycleButton, ThemeId, ThemeScope,
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
                    <section data-story-id="ui-alert-dialog" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Alert Dialog"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 03 implemented as a renderer-local confirmation flow backed by a validated shared Rust model and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <AlertDialog model=default_alert_dialog_story_model() />
                            <AlertDialog model=open_destructive_alert_dialog_story_model() default_open=true />
                            <AlertDialog model=small_loading_alert_dialog_story_model() default_open=true />
                            <AlertDialog model=disabled_alert_dialog_story_model() />
                            <AlertDialog model=invalid_alert_dialog_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <AlertDialog model=themed_alert_dialog_story_model() default_open=true />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-aspect-ratio" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Aspect Ratio"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 04 implemented as a stateless media-frame contract backed by a validated shared Rust model and Bevy-readable ratio primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <AspectRatio model=default_aspect_ratio_story_model() />
                            <AspectRatio model=contain_aspect_ratio_story_model() />
                            <AspectRatio model=loading_aspect_ratio_story_model() />
                            <AspectRatio model=disabled_aspect_ratio_story_model() />
                            <AspectRatio model=invalid_aspect_ratio_story_model() />
                            <ThemeScope theme=ThemeId::Cyberpunk>
                                <AspectRatio model=themed_aspect_ratio_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-attachment" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Attachment"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 05 implemented as a message attachment contract backed by a validated shared Rust model, local activation state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Attachment model=default_attachment_story_model() />
                            <Attachment model=image_attachment_story_model() />
                            <Attachment model=loading_attachment_story_model() />
                            <Attachment model=disabled_attachment_story_model() />
                            <Attachment model=invalid_attachment_story_model() />
                            <ThemeScope theme=ThemeId::Forest>
                                <Attachment model=themed_attachment_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-avatar" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Avatar"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 06 implemented as an identity image/fallback contract backed by a validated shared Rust model, local image fallback state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Avatar model=default_avatar_story_model() />
                            <Avatar model=fallback_avatar_story_model() />
                            <Avatar model=loading_avatar_story_model() />
                            <Avatar model=disabled_avatar_story_model() />
                            <Avatar model=invalid_avatar_story_model() />
                            <ThemeScope theme=ThemeId::Synthwave>
                                <Avatar model=themed_avatar_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-badge" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Badge"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 07 implemented as a compact status contract backed by a validated shared Rust model, renderer-local highlight state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Badge model=default_badge_story_model() />
                            <Badge model=no_icon_badge_story_model() />
                            <Badge model=loading_badge_story_model() />
                            <Badge model=disabled_badge_story_model() />
                            <Badge model=invalid_badge_story_model() />
                            <ThemeScope theme=ThemeId::Lofi>
                                <Badge model=themed_badge_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-breadcrumb" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Breadcrumb"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 08 implemented as a repeatable route-trail contract backed by a validated shared Rust model, renderer-local navigation focus state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Breadcrumb model=default_breadcrumb_story_model() />
                            <Breadcrumb model=dense_breadcrumb_story_model() />
                            <Breadcrumb model=loading_breadcrumb_story_model() />
                            <Breadcrumb model=disabled_breadcrumb_story_model() />
                            <Breadcrumb model=invalid_breadcrumb_story_model() />
                            <ThemeScope theme=ThemeId::Catppuccin>
                                <Breadcrumb model=themed_breadcrumb_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-bubble" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Bubble"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 09 implemented as a sender-aware message contract backed by a validated shared Rust model, renderer-local action state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Bubble model=default_bubble_story_model() />
                            <Bubble model=outgoing_bubble_story_model() />
                            <Bubble model=loading_bubble_story_model() />
                            <Bubble model=disabled_bubble_story_model() />
                            <Bubble model=invalid_bubble_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Bubble model=themed_bubble_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-button" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Button"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 10 implemented as a primary action primitive backed by a validated shared Rust model, renderer-local press state, link/submit semantics, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Button model=default_button_story_model() />
                            <Button model=secondary_button_story_model() />
                            <Button model=link_button_story_model() />
                            <Button model=loading_button_story_model() />
                            <Button model=disabled_button_story_model() />
                            <Button model=invalid_button_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <Button model=themed_button_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-button-group" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Button Group"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 11 implemented as a grouped action primitive backed by a validated shared Rust model, renderer-local selected state, repeated item/separator anatomy, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <ButtonGroup model=default_button_group_story_model() />
                            <ButtonGroup model=vertical_button_group_story_model() />
                            <ButtonGroup model=loading_button_group_story_model() />
                            <ButtonGroup model=disabled_button_group_story_model() />
                            <ButtonGroup model=invalid_button_group_story_model() />
                            <ThemeScope theme=ThemeId::Cyberpunk>
                                <ButtonGroup model=themed_button_group_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-calendar" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Calendar"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 12 implemented as a date grid primitive backed by a validated shared Rust model, local single/range selection state, month navigation, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Calendar model=default_calendar_story_model() />
                            <Calendar model=range_calendar_story_model() />
                            <Calendar model=loading_calendar_story_model() />
                            <Calendar model=disabled_calendar_story_model() />
                            <Calendar model=invalid_calendar_story_model() />
                            <ThemeScope theme=ThemeId::Forest>
                                <Calendar model=themed_calendar_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-card" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Card"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 13 implemented as a framed content surface backed by a validated shared Rust model, renderer-local footer action state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Card model=default_card_story_model() />
                            <Card model=dense_card_story_model() />
                            <Card model=loading_card_story_model() />
                            <Card model=disabled_card_story_model() />
                            <Card model=invalid_card_story_model() />
                            <ThemeScope theme=ThemeId::Synthwave>
                                <Card model=themed_card_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-carousel" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Carousel"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 14 implemented as a paged content strip backed by a validated shared Rust slide model, renderer-local index state, repeated item anatomy, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Carousel model=default_carousel_story_model() />
                            <Carousel model=dense_carousel_story_model() />
                            <Carousel model=loading_carousel_story_model() />
                            <Carousel model=disabled_carousel_story_model() />
                            <Carousel model=invalid_carousel_story_model() />
                            <ThemeScope theme=ThemeId::Catppuccin>
                                <Carousel model=themed_carousel_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-chart" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Chart"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 15 implemented as a themed data visualization frame backed by a validated shared Rust series model, renderer-local selected-series state, repeated series/legend anatomy, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Chart model=default_chart_story_model() />
                            <Chart model=dense_chart_story_model() />
                            <Chart model=loading_chart_story_model() />
                            <Chart model=disabled_chart_story_model() />
                            <Chart model=invalid_chart_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Chart model=themed_chart_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-checkbox" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Checkbox"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 16 implemented as a tri-state form control backed by a validated shared Rust model, renderer-local checked state, checkbox anatomy, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Checkbox model=default_checkbox_story_model() />
                            <Checkbox model=dense_checkbox_story_model() />
                            <Checkbox model=loading_checkbox_story_model() />
                            <Checkbox model=disabled_checkbox_story_model() />
                            <Checkbox model=invalid_checkbox_story_model() />
                            <ThemeScope theme=ThemeId::Forest>
                                <Checkbox model=themed_checkbox_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-collapsible" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Collapsible"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 17 implemented as a single disclosure region backed by a validated shared Rust model, renderer-local open state, trigger/content anatomy, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Collapsible model=default_collapsible_story_model() />
                            <Collapsible model=dense_collapsible_story_model() />
                            <Collapsible model=loading_collapsible_story_model() />
                            <Collapsible model=disabled_collapsible_story_model() />
                            <Collapsible model=invalid_collapsible_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <Collapsible model=themed_collapsible_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-combobox" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Combobox"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 18 implemented as an input-backed picker backed by a validated shared Rust option model, renderer-local query/open/selected state, filtered option anatomy, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Combobox model=default_combobox_story_model() />
                            <Combobox model=dense_combobox_story_model() />
                            <Combobox model=loading_combobox_story_model() />
                            <Combobox model=disabled_combobox_story_model() />
                            <Combobox model=invalid_combobox_story_model() />
                            <ThemeScope theme=ThemeId::Cyberpunk>
                                <Combobox model=themed_combobox_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-command" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Command"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 19 implemented as a searchable command palette backed by validated shared Rust groups/items, renderer-local query/open/highlight/selected state, shortcut anatomy, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Command model=default_command_story_model() />
                            <Command model=dense_command_story_model() />
                            <Command model=loading_command_story_model() />
                            <Command model=disabled_command_story_model() />
                            <Command model=invalid_command_story_model() />
                            <ThemeScope theme=ThemeId::Lofi>
                                <Command model=themed_command_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-context-menu" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Context Menu"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 20 implemented as a pointer-triggered menu backed by validated shared Rust entries/submenus, renderer-local open/active/selected/submenu state, separator anatomy, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <ContextMenu model=default_context_menu_story_model() />
                            <ContextMenu model=dense_context_menu_story_model() />
                            <ContextMenu model=loading_context_menu_story_model() />
                            <ContextMenu model=disabled_context_menu_story_model() />
                            <ContextMenu model=invalid_context_menu_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <ContextMenu model=themed_context_menu_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-data-table" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Data Table"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 21 implemented as a filterable, sortable, paginated table backed by validated shared Rust columns/rows, renderer-local table state, repeatable cell anatomy, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <DataTable model=default_data_table_story_model() />
                            <DataTable model=dense_data_table_story_model() />
                            <DataTable model=loading_data_table_story_model() />
                            <DataTable model=disabled_data_table_story_model() />
                            <DataTable model=invalid_data_table_story_model() />
                            <ThemeScope theme=ThemeId::Synthwave>
                                <DataTable model=themed_data_table_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="shadcn-components" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"shadcn component catalog"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Every official shadcn component has a literal Rust widget constructor, a concrete typed model, a token-only Leptos component, and a Bevy primitive adapter over the same render nodes."
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

fn default_alert_dialog_story_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "Archive deck",
        "Archive this deck?",
        "The deck will be hidden from the active queue until a durable state restore reactivates it.",
        AlertDialogButton::new("Archive", "archive-deck"),
        AlertDialogButton::new("Cancel", "cancel-archive"),
    )
}

fn open_destructive_alert_dialog_story_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "Delete draft",
        "Delete this draft?",
        "This cannot be undone. The draft and its local review state will be removed.",
        AlertDialogButton::new("Delete", "delete-draft"),
        AlertDialogButton::new("Cancel", "cancel-delete"),
    )
    .destructive()
}

fn small_loading_alert_dialog_story_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "Publish",
        "Publish static bundle?",
        "The gate is already creating the release artifact for Pages.",
        AlertDialogButton::new("Publish", "publish-pages"),
        AlertDialogButton::new("Cancel", "cancel-publish"),
    )
    .with_size(AlertDialogSize::Small)
    .loading()
}

fn disabled_alert_dialog_story_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "Reset progress",
        "Reset progress?",
        "The app must hydrate durable state before this destructive action is available.",
        AlertDialogButton::new("Reset", "reset-progress"),
        AlertDialogButton::new("Cancel", "cancel-reset"),
    )
    .destructive()
    .disabled()
}

fn invalid_alert_dialog_story_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "",
        "Missing title",
        "The validation boundary renders an invalid state instead of accepting an empty trigger label.",
        AlertDialogButton::new("Confirm", "confirm-invalid"),
        AlertDialogButton::new("Cancel", "cancel-invalid"),
    )
}

fn themed_alert_dialog_story_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "Share project",
        "Share this project?",
        "The confirmation surface resolves through the nested Luxury theme scope.",
        AlertDialogButton::new("Share", "share-project"),
        AlertDialogButton::new("Cancel", "cancel-share"),
    )
    .with_size(AlertDialogSize::Small)
}

fn default_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Course preview",
        "A stable 16:9 frame for images, canvases, and embedded lesson media.",
    )
}

fn contain_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Avatar crop",
        "A square frame that preserves the full media bounds with contain behavior.",
    )
    .with_ratio(1, 1)
    .with_fit(AspectRatioFit::Contain)
}

fn loading_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Generating preview",
        "Loading keeps the frame height stable while media data resolves.",
    )
    .with_ratio(4, 3)
    .loading()
}

fn disabled_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Locked recording",
        "Disabled media keeps its frame but removes interactive affordance.",
    )
    .with_ratio(21, 9)
    .disabled()
}

fn invalid_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Invalid media",
        "The validation boundary renders an invalid state instead of accepting a zero ratio side.",
    )
    .with_ratio(0, 9)
}

fn themed_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Theme scoped media",
        "The same semantic tokens resolve through a nested Cyberpunk theme scope.",
    )
    .with_ratio(3, 2)
    .with_fit(AspectRatioFit::Cover)
}

fn default_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("roadmap-notes.pdf", "2.4 MB")
        .with_action(AttachmentAction::new("Download", "download-roadmap"))
}

fn image_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("lesson-cover.png", "842 KB")
        .with_kind(AttachmentKind::Image)
        .with_action(AttachmentAction::new("Open", "open-lesson-cover"))
}

fn loading_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("uploading-transcript.txt", "Preparing")
        .with_kind(AttachmentKind::Data)
        .with_action(AttachmentAction::new("Open", "open-transcript"))
        .loading()
}

fn disabled_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("locked-export.zip", "12.8 MB")
        .with_kind(AttachmentKind::Archive)
        .with_action(AttachmentAction::new("Download", "download-export").disabled())
        .disabled()
}

fn invalid_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("", "The validation boundary rejects empty filenames.")
}

fn themed_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("theme-audit.csv", "18 rows")
        .with_kind(AttachmentKind::Data)
        .with_preview_label("CSV")
        .with_action(AttachmentAction::new("Inspect", "inspect-theme-audit"))
}

fn default_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Matthew Harwood", "MH")
}

fn fallback_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Design System", "DS")
        .with_size(AvatarSize::Small)
        .without_image()
}

fn loading_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Hydrating User", "HU").loading()
}

fn disabled_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Locked User", "LU").disabled()
}

fn invalid_avatar_story_model() -> AvatarModel {
    AvatarModel::new("", "??")
}

fn themed_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Theme Scope", "TS").with_size(AvatarSize::Large)
}

fn default_badge_story_model() -> BadgeModel {
    BadgeModel::new("Ready")
}

fn no_icon_badge_story_model() -> BadgeModel {
    BadgeModel::new("Beta")
        .with_tone(BadgeTone::Info)
        .with_size(BadgeSize::Small)
        .without_icon()
}

fn loading_badge_story_model() -> BadgeModel {
    BadgeModel::new("Syncing")
        .with_tone(BadgeTone::Brand)
        .loading()
}

fn disabled_badge_story_model() -> BadgeModel {
    BadgeModel::new("Paused")
        .with_tone(BadgeTone::Muted)
        .with_variant(BadgeVariant::Outline)
        .disabled()
}

fn invalid_badge_story_model() -> BadgeModel {
    BadgeModel::new("")
}

fn themed_badge_story_model() -> BadgeModel {
    BadgeModel::new("Critical")
        .with_tone(BadgeTone::Destructive)
        .with_variant(BadgeVariant::Solid)
        .with_icon("High")
}

fn default_breadcrumb_story_model() -> BreadcrumbModel {
    BreadcrumbModel::new(vec![
        BreadcrumbEntry::link("Library", "#library"),
        BreadcrumbEntry::link("Components", "#components"),
        BreadcrumbEntry::page("Breadcrumb"),
    ])
}

fn dense_breadcrumb_story_model() -> BreadcrumbModel {
    BreadcrumbModel::new(vec![
        BreadcrumbEntry::link("Docs", "#docs"),
        BreadcrumbEntry::link("UI", "#ui"),
        BreadcrumbEntry::page("Tokens"),
    ])
    .with_density(BreadcrumbDensity::Dense)
}

fn loading_breadcrumb_story_model() -> BreadcrumbModel {
    default_breadcrumb_story_model().loading()
}

fn disabled_breadcrumb_story_model() -> BreadcrumbModel {
    BreadcrumbModel::new(vec![
        BreadcrumbEntry::link("Workspace", "#workspace"),
        BreadcrumbEntry::link("Project", "#project").disabled(),
        BreadcrumbEntry::page("Locked route"),
    ])
    .disabled()
}

fn invalid_breadcrumb_story_model() -> BreadcrumbModel {
    BreadcrumbModel::new(Vec::new())
}

fn themed_breadcrumb_story_model() -> BreadcrumbModel {
    BreadcrumbModel::new(vec![
        BreadcrumbEntry::link("Theme", "#theme"),
        BreadcrumbEntry::link("Palette", "#palette"),
        BreadcrumbEntry::page("Catppuccin"),
    ])
    .with_separator(">")
}

fn default_bubble_story_model() -> BubbleModel {
    BubbleModel::new(
        "Codex",
        "AI",
        "The sweep is ready for review.",
        "Delivered now",
    )
}

fn outgoing_bubble_story_model() -> BubbleModel {
    BubbleModel::new(
        "Matthew",
        "MH",
        "Ship the next component when the gate is green.",
        "Sent now",
    )
    .with_side(BubbleSide::Outgoing)
    .with_actions(vec![BubbleAction::new("Edit", "edit-message")])
}

fn loading_bubble_story_model() -> BubbleModel {
    BubbleModel::new("Codex", "AI", "Hydrating response", "Pending").loading()
}

fn disabled_bubble_story_model() -> BubbleModel {
    BubbleModel::new(
        "Archive",
        "AR",
        "This message is locked by the transcript.",
        "Read only",
    )
    .with_actions(vec![BubbleAction::new("Reply", "reply").disabled()])
    .disabled()
}

fn invalid_bubble_story_model() -> BubbleModel {
    BubbleModel::new("", "AI", "Missing sender", "Invalid")
}

fn themed_bubble_story_model() -> BubbleModel {
    BubbleModel::new("System", "SYS", "Theme-scoped audit note.", "Pinned")
        .with_side(BubbleSide::System)
        .with_actions(vec![BubbleAction::new("Resolve", "resolve-note")])
}

fn default_button_story_model() -> ButtonModel {
    ButtonModel::new("Continue", "continue")
}

fn secondary_button_story_model() -> ButtonModel {
    ButtonModel::new("Preview", "preview")
        .with_variant(ButtonVariant::Secondary)
        .with_size(ButtonSize::Small)
        .without_icon()
}

fn link_button_story_model() -> ButtonModel {
    ButtonModel::new("Open docs", "open-docs")
        .with_variant(ButtonVariant::Link)
        .as_link("#docs")
}

fn loading_button_story_model() -> ButtonModel {
    ButtonModel::new("Saving", "save").loading()
}

fn disabled_button_story_model() -> ButtonModel {
    ButtonModel::new("Locked", "locked")
        .with_kind(ButtonKind::Submit)
        .with_variant(ButtonVariant::Outline)
        .disabled()
}

fn invalid_button_story_model() -> ButtonModel {
    ButtonModel::new("", "invalid")
}

fn themed_button_story_model() -> ButtonModel {
    ButtonModel::new("Delete", "delete")
        .with_variant(ButtonVariant::Destructive)
        .with_size(ButtonSize::Large)
        .with_icon("Del")
}

fn default_button_group_story_model() -> ButtonGroupModel {
    ButtonGroupModel::new(vec![
        ButtonGroupItem::new("Day", "day").with_icon("D"),
        ButtonGroupItem::new("Week", "week").with_icon("W"),
        ButtonGroupItem::new("Month", "month").with_icon("M"),
    ])
    .with_selected("week")
}

fn vertical_button_group_story_model() -> ButtonGroupModel {
    ButtonGroupModel::new(vec![
        ButtonGroupItem::new("Draft", "draft"),
        ButtonGroupItem::new("Review", "review"),
        ButtonGroupItem::new("Publish", "publish"),
    ])
    .with_orientation(ButtonGroupOrientation::Vertical)
    .with_variant(ButtonVariant::Outline)
    .with_selected("review")
}

fn loading_button_group_story_model() -> ButtonGroupModel {
    default_button_group_story_model().loading()
}

fn disabled_button_group_story_model() -> ButtonGroupModel {
    ButtonGroupModel::new(vec![
        ButtonGroupItem::new("A", "a").with_icon("A"),
        ButtonGroupItem::new("B", "b").with_icon("B").disabled(),
        ButtonGroupItem::new("C", "c").with_icon("C"),
    ])
    .with_size(ButtonSize::Icon)
    .with_selected("a")
    .disabled()
}

fn invalid_button_group_story_model() -> ButtonGroupModel {
    ButtonGroupModel::new(Vec::new())
}

fn themed_button_group_story_model() -> ButtonGroupModel {
    ButtonGroupModel::new(vec![
        ButtonGroupItem::new("Left", "left").with_icon("L"),
        ButtonGroupItem::new("Center", "center").with_icon("C"),
        ButtonGroupItem::new("Right", "right").with_icon("R"),
    ])
    .with_variant(ButtonVariant::Primary)
    .with_size(ButtonSize::Small)
    .with_selected("center")
}

fn default_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 7).with_selected(CalendarDate::new(2026, 7, 7))
}

fn range_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 7)
        .with_mode(CalendarSelectionMode::Range)
        .with_range(CalendarRange::new(
            CalendarDate::new(2026, 7, 6),
            CalendarDate::new(2026, 7, 10),
        ))
}

fn loading_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 8)
        .with_selected(CalendarDate::new(2026, 8, 14))
        .loading()
}

fn disabled_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 9)
        .with_selected(CalendarDate::new(2026, 9, 21))
        .disabled()
}

fn invalid_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 13)
}

fn themed_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 10)
        .with_mode(CalendarSelectionMode::Range)
        .with_range(CalendarRange::new(
            CalendarDate::new(2026, 10, 12),
            CalendarDate::new(2026, 10, 16),
        ))
}

fn default_card_story_model() -> CardModel {
    CardModel::new(
        "Design system",
        "64 components share one token contract.",
        "Implementation notes stay portable across Leptos DOM and Bevy scene primitives.",
        "Sweep process ready",
    )
    .with_action(CardAction::new("Open checklist", "open-checklist"))
}

fn dense_card_story_model() -> CardModel {
    CardModel::new(
        "Compact review",
        "Dense card preserves the same anatomy with tighter token spacing.",
        "Header, content, and footer spacing all resolve through shared scale tokens.",
        "No drift found",
    )
    .with_density(CardDensity::Dense)
    .with_variant(CardVariant::Outline)
    .without_action()
}

fn loading_card_story_model() -> CardModel {
    CardModel::new(
        "Publishing artifact",
        "The card keeps layout stable while the action is blocked.",
        "The one-pass gate is still building Pages and story artifacts.",
        "Waiting on CI",
    )
    .with_action(CardAction::new("View run", "view-run"))
    .loading()
}

fn disabled_card_story_model() -> CardModel {
    CardModel::new(
        "Locked surface",
        "Disabled cards keep their content visible without interactive affordance.",
        "Consumer state must hydrate before this checklist can be opened.",
        "Hydration required",
    )
    .with_variant(CardVariant::Ghost)
    .with_action(CardAction::new("Open", "open-locked").disabled())
    .disabled()
}

fn invalid_card_story_model() -> CardModel {
    CardModel::new(
        "",
        "The validation boundary renders an invalid state instead of accepting an empty title.",
        "Invalid card content remains outside the typed render-node path.",
        "Invalid",
    )
}

fn themed_card_story_model() -> CardModel {
    CardModel::new(
        "Theme scoped card",
        "The same semantic card tokens resolve through a nested Synthwave theme scope.",
        "Variant, density, text, border, and action state stay token-driven.",
        "Theme preview",
    )
    .with_variant(CardVariant::Elevated)
    .with_action(CardAction::new("Inspect", "inspect-themed-card"))
}

fn default_carousel_story_model() -> CarouselModel {
    CarouselModel::new(vec![
        CarouselSlide::new(
            "Theme preview",
            "theme-preview",
            "Semantic tokens resolve through Leptos and Bevy from the same palette.",
        ),
        CarouselSlide::new(
            "Component contract",
            "component-contract",
            "Typed render nodes keep content, controls, and indicators portable.",
        ),
        CarouselSlide::new(
            "Story proof",
            "story-proof",
            "The story harness validates the component before app integration.",
        ),
    ])
    .looping()
}

fn dense_carousel_story_model() -> CarouselModel {
    CarouselModel::new(vec![
        CarouselSlide::new(
            "Compact slide",
            "compact-slide",
            "Dense mode preserves the carousel anatomy with tighter token spacing.",
        ),
        CarouselSlide::new(
            "Focused item",
            "focused-item",
            "The selected item reads from renderer-local state only.",
        ),
    ])
    .with_density(CarouselDensity::Dense)
    .with_default_index(1)
}

fn loading_carousel_story_model() -> CarouselModel {
    default_carousel_story_model().loading()
}

fn disabled_carousel_story_model() -> CarouselModel {
    CarouselModel::new(vec![
        CarouselSlide::new(
            "Locked deck",
            "locked-deck",
            "Navigation is blocked until the app hydrates durable state.",
        ),
        CarouselSlide::new(
            "Disabled slide",
            "disabled-slide",
            "Individual slides can also remove interaction affordance.",
        )
        .disabled(),
    ])
    .disabled()
}

fn invalid_carousel_story_model() -> CarouselModel {
    CarouselModel::new(Vec::new())
}

fn themed_carousel_story_model() -> CarouselModel {
    CarouselModel::new(vec![
        CarouselSlide::new(
            "Theme scope",
            "theme-scope",
            "The same semantic carousel tokens resolve through Catppuccin.",
        ),
        CarouselSlide::new(
            "Looped controls",
            "looped-controls",
            "Previous and next controls derive disabled state from shared CarouselState.",
        ),
        CarouselSlide::new(
            "Bevy primitive",
            "bevy-primitive",
            "Scene adapters consume the same selected item and indicator nodes.",
        ),
    ])
    .with_density(CarouselDensity::Dense)
    .looping()
}

fn default_chart_story_model() -> ChartModel {
    ChartModel::new(
        "Component progress",
        "Implementation status",
        "%",
        vec![
            ChartSeries::new("Implemented", "implemented", 23).with_tone(ChartTone::Success),
            ChartSeries::new("In sweep", "in-sweep", 8).with_tone(ChartTone::Info),
            ChartSeries::new("Remaining", "remaining", 69).with_tone(ChartTone::Muted),
        ],
    )
    .with_selected_value("implemented")
}

fn dense_chart_story_model() -> ChartModel {
    ChartModel::new(
        "Gate timing",
        "Minutes",
        "m",
        vec![
            ChartSeries::new("Local", "local", 7).with_tone(ChartTone::Brand),
            ChartSeries::new("Remote", "remote", 13).with_tone(ChartTone::Info),
        ],
    )
    .with_density(ChartDensity::Dense)
    .with_selected_value("remote")
}

fn loading_chart_story_model() -> ChartModel {
    default_chart_story_model().loading()
}

fn disabled_chart_story_model() -> ChartModel {
    ChartModel::new(
        "Hydration status",
        "Availability",
        "%",
        vec![
            ChartSeries::new("Ready", "ready", 75).with_tone(ChartTone::Success),
            ChartSeries::new("Blocked", "blocked", 25)
                .with_tone(ChartTone::Warning)
                .disabled(),
        ],
    )
    .with_selected_value("ready")
    .disabled()
}

fn invalid_chart_story_model() -> ChartModel {
    ChartModel::new("Invalid chart", "Axis", "%", Vec::new())
}

fn themed_chart_story_model() -> ChartModel {
    ChartModel::new(
        "Theme scoped chart",
        "Palette coverage",
        "%",
        vec![
            ChartSeries::new("Brand", "brand", 40).with_tone(ChartTone::Brand),
            ChartSeries::new("Danger", "danger", 18).with_tone(ChartTone::Danger),
            ChartSeries::new("Muted", "muted", 42).with_tone(ChartTone::Muted),
        ],
    )
    .with_density(ChartDensity::Dense)
    .with_selected_value("brand")
}

fn default_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Use shared theme tokens", "shared-theme")
        .with_description("Leptos and Bevy read the same semantic checkbox state.")
        .checked()
}

fn dense_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Compact preference", "compact-preference")
        .with_density(CheckboxDensity::Dense)
        .with_description("Dense layout keeps the same state contract.")
}

fn loading_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Syncing setting", "syncing-setting")
        .with_description("The renderer blocks local toggles while hydration is pending.")
        .loading()
}

fn disabled_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Locked rollout", "locked-rollout")
        .with_description("Disabled checkboxes preserve the shared checked value.")
        .with_checked(CheckboxChecked::Indeterminate)
        .disabled()
}

fn invalid_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Accept the state contract", "accept-state-contract")
        .with_description("The consumer owns durable choices through rs-dean-state.")
        .required()
        .with_error("This required option has not been accepted.")
}

fn themed_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Theme scoped checkbox", "theme-scoped-checkbox")
        .with_description("The same control resolves through a nested Forest theme scope.")
        .indeterminate()
}

fn default_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "token-stack",
        "Shared token stack",
        "A single disclosure region can expose implementation notes without creating durable state.",
    )
    .open()
}

fn dense_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "compact-disclosure",
        "Compact disclosure",
        "Dense mode keeps the same trigger and content anatomy with tighter token spacing.",
    )
    .with_density(CollapsibleDensity::Dense)
}

fn loading_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "loading-disclosure",
        "Hydrating disclosure",
        "The trigger is blocked while renderer-local state hydrates from the consumer boundary.",
    )
    .loading()
}

fn disabled_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "locked-disclosure",
        "Locked disclosure",
        "Disabled disclosure keeps the content contract visible to Bevy primitives.",
    )
    .open()
    .disabled()
}

fn invalid_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "invalid-disclosure",
        "",
        "Missing title should fail validation.",
    )
}

fn themed_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "theme-disclosure",
        "Theme scoped disclosure",
        "The same semantic classes resolve through the nested Luxury theme scope.",
    )
    .open()
}

fn default_combobox_story_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Leptos DOM", "leptos"),
        ComboboxOption::new("Bevy scene", "bevy"),
        ComboboxOption::new("Shared state", "state"),
    ])
    .with_placeholder("Search UI surface")
    .with_selected_value("leptos")
}

fn dense_combobox_story_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Accordion", "accordion"),
        ComboboxOption::new("Checkbox", "checkbox"),
        ComboboxOption::new("Collapsible", "collapsible"),
    ])
    .with_density(ComboboxDensity::Dense)
    .with_placeholder("Filter component")
    .with_default_query("co")
}

fn loading_combobox_story_model() -> ComboboxModel {
    default_combobox_story_model().loading()
}

fn disabled_combobox_story_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Stable", "stable"),
        ComboboxOption::new("Blocked", "blocked").disabled(),
    ])
    .with_selected_value("stable")
    .disabled()
}

fn invalid_combobox_story_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Duplicate", "same"),
        ComboboxOption::new("Duplicate again", "same"),
    ])
}

fn themed_combobox_story_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Brand", "brand"),
        ComboboxOption::new("Success", "success"),
        ComboboxOption::new("Danger", "danger"),
    ])
    .with_placeholder("Search tone")
    .with_selected_value("success")
}

fn default_command_story_model() -> CommandModel {
    CommandModel::new(vec![
        CommandGroup::new(
            "Workspace",
            "workspace",
            vec![
                CommandItem::new("Run gate", "gate")
                    .with_detail("Run cargo xtask gate.")
                    .with_shortcut("G")
                    .with_keywords(vec!["check".to_owned(), "ci".to_owned()]),
                CommandItem::new("Open stories", "stories")
                    .with_detail("Launch the UI story harness.")
                    .with_shortcut("S"),
            ],
        ),
        CommandGroup::new(
            "Surfaces",
            "surfaces",
            vec![
                CommandItem::new("Marketing app", "marketing")
                    .with_detail("Open the Leptos marketing surface.")
                    .with_shortcut("M"),
                CommandItem::new("Bevy game", "game")
                    .with_detail("Open the WebGPU game surface.")
                    .with_shortcut("B"),
            ],
        ),
    ])
    .with_placeholder("Search command")
    .with_selected_value("gate")
    .with_highlighted_value("stories")
}

fn dense_command_story_model() -> CommandModel {
    CommandModel::new(vec![CommandGroup::new(
        "Components",
        "components",
        vec![
            CommandItem::new("Accordion", "accordion").with_shortcut("A"),
            CommandItem::new("Combobox", "combobox").with_shortcut("C"),
            CommandItem::new("Command", "command").with_shortcut("K"),
        ],
    )])
    .with_density(CommandDensity::Dense)
    .with_placeholder("Filter component")
    .with_default_query("co")
    .with_highlighted_value("combobox")
}

fn loading_command_story_model() -> CommandModel {
    default_command_story_model().loading()
}

fn disabled_command_story_model() -> CommandModel {
    CommandModel::new(vec![CommandGroup::new(
        "Locked",
        "locked",
        vec![
            CommandItem::new("Available", "available").with_shortcut("A"),
            CommandItem::new("Blocked", "blocked")
                .with_shortcut("B")
                .disabled(),
        ],
    )])
    .with_selected_value("available")
    .disabled()
}

fn invalid_command_story_model() -> CommandModel {
    CommandModel::new(vec![CommandGroup::new(
        "Duplicate",
        "duplicate",
        vec![
            CommandItem::new("First", "same").with_shortcut("1"),
            CommandItem::new("Second", "same").with_shortcut("2"),
        ],
    )])
}

fn themed_command_story_model() -> CommandModel {
    CommandModel::new(vec![
        CommandGroup::new(
            "Theme",
            "theme",
            vec![
                CommandItem::new("Brand route", "brand").with_shortcut("R"),
                CommandItem::new("Success route", "success").with_shortcut("S"),
                CommandItem::new("Danger route", "danger").with_shortcut("D"),
            ],
        ),
        CommandGroup::new(
            "Runtime",
            "runtime",
            vec![
                CommandItem::new("Leptos DOM", "leptos").with_shortcut("L"),
                CommandItem::new("Bevy scene", "bevy").with_shortcut("B"),
            ],
        ),
    ])
    .with_placeholder("Search themed command")
    .with_selected_value("success")
    .with_highlighted_value("brand")
}

fn default_context_menu_story_model() -> ContextMenuModel {
    ContextMenuModel::new(vec![
        ContextMenuEntry::item(
            ContextMenuAction::new("Back", "back")
                .with_detail("Return to the previous route.")
                .with_shortcut("Cmd+["),
        ),
        ContextMenuEntry::item(
            ContextMenuAction::new("Reload", "reload")
                .with_detail("Refresh the current surface.")
                .with_shortcut("Cmd+R"),
        ),
        ContextMenuEntry::separator("navigation-separator"),
        ContextMenuEntry::submenu(ContextMenuSubmenu::new(
            "Insert",
            "insert",
            vec![
                ContextMenuAction::new("Card", "insert-card").with_shortcut("C"),
                ContextMenuAction::new("Chart", "insert-chart").with_shortcut("H"),
            ],
        )),
        ContextMenuEntry::separator("danger-separator"),
        ContextMenuEntry::item(
            ContextMenuAction::new("Delete", "delete")
                .with_detail("Remove the focused object.")
                .destructive(),
        ),
    ])
    .with_trigger_label("Right click surface")
    .with_selected_value("reload")
    .with_active_value("insert")
    .with_open_submenu("insert")
}

fn dense_context_menu_story_model() -> ContextMenuModel {
    ContextMenuModel::new(vec![
        ContextMenuEntry::item(ContextMenuAction::new("Open", "open").with_shortcut("O")),
        ContextMenuEntry::item(ContextMenuAction::new("Rename", "rename").with_shortcut("R")),
        ContextMenuEntry::separator("dense-separator"),
        ContextMenuEntry::submenu(ContextMenuSubmenu::new(
            "Move to",
            "move",
            vec![
                ContextMenuAction::new("Inbox", "move-inbox"),
                ContextMenuAction::new("Archive", "move-archive"),
            ],
        )),
    ])
    .with_density(ContextMenuDensity::Dense)
    .with_trigger_label("Dense menu")
    .with_active_value("move")
    .with_open_submenu("move")
}

fn loading_context_menu_story_model() -> ContextMenuModel {
    default_context_menu_story_model().loading()
}

fn disabled_context_menu_story_model() -> ContextMenuModel {
    ContextMenuModel::new(vec![
        ContextMenuEntry::item(ContextMenuAction::new("Available", "available")),
        ContextMenuEntry::item(ContextMenuAction::new("Blocked", "blocked").disabled()),
        ContextMenuEntry::separator("disabled-separator"),
        ContextMenuEntry::submenu(
            ContextMenuSubmenu::new(
                "Locked submenu",
                "locked-submenu",
                vec![ContextMenuAction::new("Nested", "nested")],
            )
            .disabled(),
        ),
    ])
    .with_selected_value("available")
    .disabled()
}

fn invalid_context_menu_story_model() -> ContextMenuModel {
    ContextMenuModel::new(vec![
        ContextMenuEntry::item(ContextMenuAction::new("Duplicate", "same")),
        ContextMenuEntry::submenu(ContextMenuSubmenu::new(
            "Nested duplicate",
            "nested-duplicate",
            vec![ContextMenuAction::new("Duplicate again", "same")],
        )),
    ])
}

fn themed_context_menu_story_model() -> ContextMenuModel {
    ContextMenuModel::new(vec![
        ContextMenuEntry::item(ContextMenuAction::new("Inspect token", "inspect-token")),
        ContextMenuEntry::item(ContextMenuAction::new("Copy class", "copy-class")),
        ContextMenuEntry::separator("theme-separator"),
        ContextMenuEntry::submenu(ContextMenuSubmenu::new(
            "Theme scope",
            "theme-scope",
            vec![
                ContextMenuAction::new("Leptos DOM", "leptos-dom"),
                ContextMenuAction::new("Bevy primitive", "bevy-primitive"),
            ],
        )),
    ])
    .with_trigger_label("Theme scoped menu")
    .with_selected_value("copy-class")
    .with_active_value("theme-scope")
    .with_open_submenu("theme-scope")
}

fn data_table_story_columns() -> Vec<DataTableColumn> {
    vec![
        DataTableColumn::new("Component", "component"),
        DataTableColumn::new("Surface", "surface"),
        DataTableColumn::new("Health", "health").numeric(),
    ]
}

fn data_table_story_rows() -> Vec<DataTableRow> {
    vec![
        DataTableRow::new(
            "accordion",
            vec![
                "Accordion".to_owned(),
                "Disclosure".to_owned(),
                "98".to_owned(),
            ],
        ),
        DataTableRow::new(
            "combobox",
            vec!["Combobox".to_owned(), "Form".to_owned(), "95".to_owned()],
        ),
        DataTableRow::new(
            "command",
            vec!["Command".to_owned(), "Overlay".to_owned(), "93".to_owned()],
        ),
        DataTableRow::new(
            "context-menu",
            vec![
                "Context Menu".to_owned(),
                "Overlay".to_owned(),
                "91".to_owned(),
            ],
        )
        .disabled(),
        DataTableRow::new(
            "data-table",
            vec!["Data Table".to_owned(), "Data".to_owned(), "89".to_owned()],
        ),
    ]
}

fn default_data_table_story_model() -> DataTableModel {
    DataTableModel::new(data_table_story_columns(), data_table_story_rows())
        .with_title("Component health")
        .with_filter_placeholder("Filter components")
        .with_sort("component", DataTableSortDirection::Ascending)
        .with_selected_row("command")
        .with_page_size(3)
}

fn dense_data_table_story_model() -> DataTableModel {
    DataTableModel::new(data_table_story_columns(), data_table_story_rows())
        .with_density(DataTableDensity::Dense)
        .with_title("Dense health")
        .with_filter_placeholder("Filter dense rows")
        .with_sort("health", DataTableSortDirection::Descending)
        .with_default_filter("co")
        .with_page_size(2)
}

fn loading_data_table_story_model() -> DataTableModel {
    default_data_table_story_model().loading()
}

fn disabled_data_table_story_model() -> DataTableModel {
    DataTableModel::new(data_table_story_columns(), data_table_story_rows())
        .with_title("Disabled table")
        .with_selected_row("accordion")
        .disabled()
}

fn invalid_data_table_story_model() -> DataTableModel {
    DataTableModel::new(
        data_table_story_columns(),
        vec![DataTableRow::new(
            "bad-row",
            vec!["Missing cells".to_owned(), "Data".to_owned()],
        )],
    )
    .with_title("Invalid table")
}

fn themed_data_table_story_model() -> DataTableModel {
    DataTableModel::new(data_table_story_columns(), data_table_story_rows())
        .with_title("Theme scoped table")
        .with_filter_placeholder("Filter theme rows")
        .with_sort("surface", DataTableSortDirection::Ascending)
        .with_selected_row("data-table")
        .with_page_size(4)
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
