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
    DataTableDensity, DataTableModel, DataTableRow, DataTableSortDirection, DatePicker,
    DatePickerDensity, DatePickerModel, Dialog, DialogAction, DialogMode, DialogModel, DialogSize,
    Direction, DirectionModel, DirectionValue, Drawer, DrawerAction, DrawerModel, DrawerSide,
    DropdownMenu, DropdownMenuDensity, DropdownMenuEntry, DropdownMenuItem, DropdownMenuModel,
    Empty, EmptyAction, EmptyDensity, EmptyModel, Field, FieldDensity, FieldInputKind, FieldModel,
    HealthCard, HoverCard, HoverCardDensity, HoverCardModel, Input, InputAction, InputDensity,
    InputGroup, InputGroupModel, InputKind, InputModel, InputOtp, InputOtpModel, Item, ItemAction,
    ItemDensity, ItemModel, Kbd, KbdDensity, KbdKey, KbdModel, Label, LabelDensity, LabelModel,
    LabelRequirement, Marker, MarkerAnchor, MarkerDensity, MarkerModel, MarkerTone, Menubar,
    MenubarDensity, MenubarItem, MenubarMenu, MenubarModel, Message, MessageAction, MessageDensity,
    MessageModel, MessageScroller, MessageScrollerDensity, MessageScrollerEntry,
    MessageScrollerModel, MessageSide, NativeSelect, NativeSelectDensity, NativeSelectModel,
    NativeSelectOption, NavigationMenu, NavigationMenuDensity, NavigationMenuItem,
    NavigationMenuLink, NavigationMenuModel, Pagination, PaginationDensity, PaginationModel,
    Popover, PopoverDensity, PopoverModel, Progress, ProgressDensity, ProgressModel, RadioGroup,
    RadioGroupDensity, RadioGroupModel, RadioGroupOption, RadioGroupOrientation, Resizable,
    ResizableDensity, ResizableModel, ResizableOrientation, ResizablePanel, ScrollArea,
    ScrollAreaDensity, ScrollAreaItem, ScrollAreaModel, ScrollAreaOverflow, Select, SelectDensity,
    SelectGroup, SelectModel, SelectOption, Separator, SeparatorDensity, SeparatorModel,
    SeparatorOrientation, ShadcnComponentGallery, Sheet, SheetAction, SheetDensity, SheetModel,
    SheetSide, Sidebar, SidebarDensity, SidebarGroup, SidebarItem, SidebarModel, Skeleton,
    SkeletonDensity, SkeletonModel, Slider, SliderDensity, SliderModel, SliderOrientation, Sonner,
    SonnerAction, SonnerDensity, SonnerModel, SonnerPosition, SonnerToast, SonnerTone, Spinner,
    SpinnerDensity, SpinnerModel, SpinnerSize, SpinnerTone, Switch, SwitchDensity, SwitchModel,
    Table, TableColumn, TableDensity, TableModel, TableRow, Tabs, TabsDensity, TabsItem, TabsModel,
    TabsOrientation, ThemeCycleButton, ThemeId, ThemeScope,
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
                    <section data-story-id="ui-date-picker" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Date Picker"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 22 implemented as a composed date trigger, popover, calendar grid, and value contract backed by shared Rust date state and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <DatePicker model=default_date_picker_story_model() />
                            <DatePicker model=dense_open_date_picker_story_model() />
                            <DatePicker model=loading_date_picker_story_model() />
                            <DatePicker model=disabled_date_picker_story_model() />
                            <DatePicker model=invalid_date_picker_story_model() />
                            <ThemeScope theme=ThemeId::Catppuccin>
                                <DatePicker model=themed_date_picker_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-dialog" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Dialog"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 23 implemented as a modal or non-modal workflow overlay backed by validated shared Rust copy/actions, renderer-local open/focus/action state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Dialog model=default_dialog_story_model() />
                            <Dialog model=open_small_dialog_story_model() />
                            <Dialog model=loading_dialog_story_model() />
                            <Dialog model=disabled_dialog_story_model() />
                            <Dialog model=invalid_dialog_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <Dialog model=themed_dialog_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-direction" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Direction"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 24 implemented as a direction provider, nested scope, and aware-content utility backed by validated shared Rust direction state and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Direction model=default_direction_story_model() />
                            <Direction model=rtl_direction_story_model() />
                            <Direction model=scoped_direction_story_model() />
                            <Direction model=loading_direction_story_model() />
                            <Direction model=disabled_direction_story_model() />
                            <Direction model=invalid_direction_story_model() />
                            <ThemeScope theme=ThemeId::Forest>
                                <Direction model=themed_direction_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-drawer" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Drawer"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 25 implemented as a side-aware drawer overlay backed by validated shared Rust copy/actions, renderer-local open/drag/action state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Drawer model=default_drawer_story_model() />
                            <Drawer model=right_drawer_story_model() />
                            <Drawer model=loading_drawer_story_model() />
                            <Drawer model=disabled_drawer_story_model() />
                            <Drawer model=invalid_drawer_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Drawer model=themed_drawer_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-dropdown-menu" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Dropdown Menu"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 26 implemented as a trigger-attached action menu backed by validated shared Rust entries, renderer-local open/focus/select state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <DropdownMenu model=default_dropdown_menu_story_model() />
                            <DropdownMenu model=dense_dropdown_menu_story_model() />
                            <DropdownMenu model=loading_dropdown_menu_story_model() />
                            <DropdownMenu model=disabled_dropdown_menu_story_model() />
                            <DropdownMenu model=invalid_dropdown_menu_story_model() />
                            <ThemeScope theme=ThemeId::Cyberpunk>
                                <DropdownMenu model=themed_dropdown_menu_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-empty" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Empty"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 27 implemented as a structured empty-state primitive backed by validated shared Rust copy/action state, renderer-local activation, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Empty model=default_empty_story_model() />
                            <Empty model=dense_empty_story_model() />
                            <Empty model=loading_empty_story_model() />
                            <Empty model=disabled_empty_story_model() />
                            <Empty model=invalid_empty_story_model() />
                            <ThemeScope theme=ThemeId::Forest>
                                <Empty model=themed_empty_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-field" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Field"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 28 implemented as a form-field composition wrapper backed by validated shared Rust label/control/error state, renderer-local focus/input draft state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Field model=default_field_story_model() />
                            <Field model=dense_field_story_model() />
                            <Field model=loading_field_story_model() />
                            <Field model=disabled_field_story_model() />
                            <Field model=invalid_field_story_model() />
                            <ThemeScope theme=ThemeId::Catppuccin>
                                <Field model=themed_field_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-hover-card" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Hover Card"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 29 implemented as a trigger-attached preview overlay backed by validated shared Rust copy, renderer-local hover/focus state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <HoverCard model=default_hover_card_story_model() />
                            <HoverCard model=dense_hover_card_story_model() />
                            <HoverCard model=loading_hover_card_story_model() />
                            <HoverCard model=disabled_hover_card_story_model() />
                            <HoverCard model=invalid_hover_card_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <HoverCard model=themed_hover_card_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-input" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Input"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 30 implemented as a single-line form control backed by validated shared Rust prefix/control/suffix state, renderer-local draft input state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Input model=default_input_story_model() />
                            <Input model=dense_input_story_model() />
                            <Input model=loading_input_story_model() />
                            <Input model=disabled_input_story_model() />
                            <Input model=invalid_input_story_model() />
                            <ThemeScope theme=ThemeId::Lofi>
                                <Input model=themed_input_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-input-group" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Input Group"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 31 implemented as a composed input primitive backed by validated shared Rust addon/input/button state, renderer-local draft input state, and Bevy-readable render nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <InputGroup model=default_input_group_story_model() />
                            <InputGroup model=dense_input_group_story_model() />
                            <InputGroup model=loading_input_group_story_model() />
                            <InputGroup model=disabled_input_group_story_model() />
                            <InputGroup model=invalid_input_group_story_model() />
                            <ThemeScope theme=ThemeId::Synthwave>
                                <InputGroup model=themed_input_group_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-input-otp" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Input OTP"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 32 implemented as a fixed-length code control backed by validated shared Rust slot/group/separator state, renderer-local focus and paste intents, and Bevy-readable repeatable slot nodes."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <InputOtp model=default_input_otp_story_model() />
                            <InputOtp model=dense_input_otp_story_model() />
                            <InputOtp model=loading_input_otp_story_model() />
                            <InputOtp model=disabled_input_otp_story_model() />
                            <InputOtp model=invalid_input_otp_story_model() />
                            <ThemeScope theme=ThemeId::Cyberpunk>
                                <InputOtp model=themed_input_otp_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-item" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Item"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 33 implemented as a flexible content row backed by validated shared Rust media/content/title/description/action nodes, renderer-local action state, and Bevy-readable repeatable action primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Item model=default_item_story_model() />
                            <Item model=dense_item_story_model() />
                            <Item model=loading_item_story_model() />
                            <Item model=disabled_item_story_model() />
                            <Item model=invalid_item_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Item model=themed_item_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-kbd" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Kbd"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 34 implemented as a keyboard shortcut token backed by validated shared Rust key/chord nodes, renderer-local key focus state, and Bevy-readable repeatable key primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Kbd model=default_kbd_story_model() />
                            <Kbd model=dense_kbd_story_model() />
                            <Kbd model=loading_kbd_story_model() />
                            <Kbd model=disabled_kbd_story_model() />
                            <Kbd model=invalid_kbd_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <Kbd model=themed_kbd_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-label" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Label"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 35 implemented as a form label primitive backed by validated shared Rust text/requirement nodes, renderer-local hover/focus state, and Bevy-readable label primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Label model=default_label_story_model() />
                            <Label model=dense_label_story_model() />
                            <Label model=loading_label_story_model() />
                            <Label model=disabled_label_story_model() />
                            <Label model=invalid_label_story_model() />
                            <ThemeScope theme=ThemeId::Catppuccin>
                                <Label model=themed_label_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-marker" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Marker"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 36 implemented as a status marker backed by validated shared Rust dot/label/anchor nodes, renderer-local hover/focus/navigation state, and Bevy-readable marker primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Marker model=default_marker_story_model() />
                            <Marker model=dense_marker_story_model() />
                            <Marker model=loading_marker_story_model() />
                            <Marker model=disabled_marker_story_model() />
                            <Marker model=invalid_marker_story_model() />
                            <ThemeScope theme=ThemeId::Forest>
                                <Marker model=themed_marker_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-menubar" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Menubar"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 37 implemented as a horizontal application menu backed by validated shared Rust menu/trigger/content/item nodes, renderer-local open/focus/activation state, and Bevy-readable indexed menu primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Menubar model=default_menubar_story_model() />
                            <Menubar model=dense_menubar_story_model() />
                            <Menubar model=loading_menubar_story_model() />
                            <Menubar model=disabled_menubar_story_model() />
                            <Menubar model=invalid_menubar_story_model() />
                            <ThemeScope theme=ThemeId::Synthwave>
                                <Menubar model=themed_menubar_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-message" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Message"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 38 implemented as a durable message surface backed by validated shared Rust header/content/footer/action nodes, renderer-local action state, and Bevy-readable message primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Message model=default_message_story_model() />
                            <Message model=dense_outgoing_message_story_model() />
                            <Message model=loading_message_story_model() />
                            <Message model=disabled_message_story_model() />
                            <Message model=invalid_message_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Message model=themed_message_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-message-scroller" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Message Scroller"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 39 implemented as a live transcript viewport backed by validated shared Rust message entries, renderer-local scroll/jump state, and Bevy-readable scroller primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <MessageScroller model=default_message_scroller_story_model() />
                            <MessageScroller model=dense_latest_message_scroller_story_model() />
                            <MessageScroller model=loading_message_scroller_story_model() />
                            <MessageScroller model=disabled_message_scroller_story_model() />
                            <MessageScroller model=invalid_message_scroller_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <MessageScroller model=themed_message_scroller_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-native-select" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Native Select"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 40 implemented as a browser-native select backed by validated shared Rust options, renderer-local focus/selection state, and Bevy-readable select primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <NativeSelect model=default_native_select_story_model() />
                            <NativeSelect model=dense_native_select_story_model() />
                            <NativeSelect model=loading_native_select_story_model() />
                            <NativeSelect model=disabled_native_select_story_model() />
                            <NativeSelect model=invalid_native_select_story_model() />
                            <ThemeScope theme=ThemeId::Catppuccin>
                                <NativeSelect model=themed_native_select_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-navigation-menu" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Navigation Menu"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 41 implemented as a responsive top-level navigation contract backed by validated shared Rust items, renderer-local open/focus state, and Bevy-readable menu primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <NavigationMenu model=default_navigation_menu_story_model() />
                            <NavigationMenu model=dense_navigation_menu_story_model() />
                            <NavigationMenu model=loading_navigation_menu_story_model() />
                            <NavigationMenu model=disabled_navigation_menu_story_model() />
                            <NavigationMenu model=invalid_navigation_menu_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <NavigationMenu model=themed_navigation_menu_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-pagination" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Pagination"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 42 implemented as a page navigation contract backed by validated shared Rust page math, renderer-local current/focus state, and Bevy-readable page primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Pagination model=default_pagination_story_model() />
                            <Pagination model=dense_pagination_story_model() />
                            <Pagination model=loading_pagination_story_model() />
                            <Pagination model=disabled_pagination_story_model() />
                            <Pagination model=invalid_pagination_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Pagination model=themed_pagination_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-popover" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Popover"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 43 implemented as a trigger-attached overlay backed by validated shared Rust copy, renderer-local open/focus state, and Bevy-readable overlay primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Popover model=default_popover_story_model() />
                            <Popover model=dense_popover_story_model() />
                            <Popover model=loading_popover_story_model() />
                            <Popover model=disabled_popover_story_model() />
                            <Popover model=invalid_popover_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <Popover model=themed_popover_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-progress" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Progress"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 44 implemented as a consumer-owned progress value backed by validated shared Rust state, renderer-local highlight state, and Bevy-readable progress primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Progress model=default_progress_story_model() />
                            <Progress model=dense_progress_story_model() />
                            <Progress model=loading_progress_story_model() />
                            <Progress model=disabled_progress_story_model() />
                            <Progress model=invalid_progress_story_model() />
                            <Progress model=indeterminate_progress_story_model() />
                            <ThemeScope theme=ThemeId::Catppuccin>
                                <Progress model=themed_progress_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-radio-group" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Radio Group"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 45 implemented as a mutually exclusive option group backed by validated shared Rust options, renderer-local focus/selection state, and Bevy-readable radio primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <RadioGroup model=default_radio_group_story_model() />
                            <RadioGroup model=dense_radio_group_story_model() />
                            <RadioGroup model=loading_radio_group_story_model() />
                            <RadioGroup model=disabled_radio_group_story_model() />
                            <RadioGroup model=invalid_radio_group_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <RadioGroup model=themed_radio_group_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-resizable" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Resizable"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 46 implemented as a split-panel layout backed by validated shared Rust panel bounds, renderer-local resize state, and Bevy-readable panel primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Resizable model=default_resizable_story_model() />
                            <Resizable model=dense_resizable_story_model() />
                            <Resizable model=vertical_resizable_story_model() />
                            <Resizable model=loading_resizable_story_model() />
                            <Resizable model=disabled_resizable_story_model() />
                            <Resizable model=invalid_resizable_story_model() />
                            <ThemeScope theme=ThemeId::Cyberpunk>
                                <Resizable model=themed_resizable_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-scroll-area" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Scroll Area"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 47 implemented as a scrollable content region backed by validated shared Rust items, renderer-local viewport/bar state, and Bevy-readable scroll primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <ScrollArea model=default_scroll_area_story_model() />
                            <ScrollArea model=dense_scroll_area_story_model() />
                            <ScrollArea model=horizontal_scroll_area_story_model() />
                            <ScrollArea model=loading_scroll_area_story_model() />
                            <ScrollArea model=disabled_scroll_area_story_model() />
                            <ScrollArea model=invalid_scroll_area_story_model() />
                            <ThemeScope theme=ThemeId::Lofi>
                                <ScrollArea model=themed_scroll_area_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-select" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Select"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 48 implemented as a trigger-based listbox backed by validated shared Rust groups/options, renderer-local open/focus/selection state, and Bevy-readable select primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Select model=default_select_story_model() />
                            <Select model=dense_select_story_model() />
                            <Select model=loading_select_story_model() />
                            <Select model=disabled_select_story_model() />
                            <Select model=invalid_select_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Select model=themed_select_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-separator" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Separator"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 49 implemented as a divider primitive backed by validated shared Rust orientation/label state, renderer-local focus/hover state, and Bevy-readable separator primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Separator model=default_separator_story_model() />
                            <Separator model=dense_separator_story_model() />
                            <Separator model=vertical_separator_story_model() />
                            <Separator model=loading_separator_story_model() />
                            <Separator model=disabled_separator_story_model() />
                            <Separator model=invalid_separator_story_model() />
                            <ThemeScope theme=ThemeId::Cyberpunk>
                                <Separator model=themed_separator_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-sheet" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Sheet"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 50 implemented as an edge-attached workflow panel backed by validated shared Rust side/action state, renderer-local open/focus state, and Bevy-readable overlay primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Sheet model=default_sheet_story_model() />
                            <Sheet model=dense_sheet_story_model() />
                            <Sheet model=left_sheet_story_model() />
                            <Sheet model=loading_sheet_story_model() />
                            <Sheet model=disabled_sheet_story_model() />
                            <Sheet model=invalid_sheet_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Sheet model=themed_sheet_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-sidebar" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Sidebar"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 51 implemented as a grouped navigation rail backed by validated shared Rust groups/items, renderer-local collapse/focus/active state, and Bevy-readable navigation primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Sidebar model=default_sidebar_story_model() />
                            <Sidebar model=dense_sidebar_story_model() />
                            <Sidebar model=collapsed_sidebar_story_model() />
                            <Sidebar model=loading_sidebar_story_model() />
                            <Sidebar model=disabled_sidebar_story_model() />
                            <Sidebar model=invalid_sidebar_story_model() />
                            <ThemeScope theme=ThemeId::Cyberpunk>
                                <Sidebar model=themed_sidebar_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-skeleton" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Skeleton"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 52 implemented as a layout-preserving placeholder backed by validated shared Rust geometry, renderer-local focus/pause state, and Bevy-readable loading primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Skeleton model=default_skeleton_story_model() />
                            <Skeleton model=dense_skeleton_story_model() />
                            <Skeleton model=ready_skeleton_story_model() />
                            <Skeleton model=static_skeleton_story_model() />
                            <Skeleton model=disabled_skeleton_story_model() />
                            <Skeleton model=invalid_skeleton_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Skeleton model=themed_skeleton_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-slider" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Slider"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 53 implemented as a numeric range control backed by validated shared Rust min/max/step state, renderer-local focus/drag/value state, and Bevy-readable range primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Slider model=default_slider_story_model() />
                            <Slider model=dense_slider_story_model() />
                            <Slider model=vertical_slider_story_model() />
                            <Slider model=loading_slider_story_model() />
                            <Slider model=disabled_slider_story_model() />
                            <Slider model=invalid_slider_story_model() />
                            <ThemeScope theme=ThemeId::Cyberpunk>
                                <Slider model=themed_slider_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-sonner" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Sonner"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 54 implemented as a toast viewport backed by validated shared Rust notification state, renderer-local pause/action/dismiss state, and Bevy-readable toast primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Sonner model=default_sonner_story_model() />
                            <Sonner model=dense_sonner_story_model() />
                            <Sonner model=centered_sonner_story_model() />
                            <Sonner model=loading_sonner_story_model() />
                            <Sonner model=disabled_sonner_story_model() />
                            <Sonner model=invalid_sonner_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Sonner model=themed_sonner_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-spinner" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Spinner"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 55 implemented as a compact activity indicator backed by validated shared Rust motion state, renderer-local pause/focus state, and Bevy-readable spinner primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Spinner model=default_spinner_story_model() />
                            <Spinner model=dense_spinner_story_model() />
                            <Spinner model=large_spinner_story_model() />
                            <Spinner model=ready_spinner_story_model() />
                            <Spinner model=disabled_spinner_story_model() />
                            <Spinner model=invalid_spinner_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <Spinner model=themed_spinner_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-switch" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Switch"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 56 implemented as a binary setting control backed by validated shared Rust state, renderer-local focus/toggle state, and Bevy-readable switch primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Switch model=default_switch_story_model() />
                            <Switch model=dense_switch_story_model() />
                            <Switch model=off_switch_story_model() />
                            <Switch model=loading_switch_story_model() />
                            <Switch model=disabled_switch_story_model() />
                            <Switch model=invalid_switch_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Switch model=themed_switch_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-table" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Table"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 57 implemented as a semantic table backed by validated shared Rust row and column data, renderer-local row selection, and Bevy-readable table primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Table model=default_table_story_model() />
                            <Table model=dense_table_story_model() />
                            <Table model=selected_table_story_model() />
                            <Table model=loading_table_story_model() />
                            <Table model=disabled_table_story_model() />
                            <Table model=invalid_table_story_model() />
                            <ThemeScope theme=ThemeId::Luxury>
                                <Table model=themed_table_story_model() />
                            </ThemeScope>
                        </div>
                    </section>
                    <section data-story-id="ui-tabs" class=STORY_SECTION>
                        <header class=STORY_SECTION_HEADER>
                            <h2 class=STORY_SECTION_TITLE>"Tabs"</h2>
                            <p class=STORY_SECTION_BODY>
                                "Issue 58 implemented as a tabbed panel contract backed by validated shared Rust items, renderer-local selected/focus state, and Bevy-readable tab primitives."
                            </p>
                        </header>
                        <div class=ALERT_STORY_GRID>
                            <Tabs model=default_tabs_story_model() />
                            <Tabs model=dense_tabs_story_model() />
                            <Tabs model=vertical_tabs_story_model() />
                            <Tabs model=loading_tabs_story_model() />
                            <Tabs model=disabled_tabs_story_model() />
                            <Tabs model=invalid_tabs_story_model() />
                            <ThemeScope theme=ThemeId::Dracula>
                                <Tabs model=themed_tabs_story_model() />
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

fn default_date_picker_story_model() -> DatePickerModel {
    DatePickerModel::new(2026, 7)
        .with_label("Release date")
        .with_placeholder("Choose release date")
        .with_selected(CalendarDate::new(2026, 7, 7))
}

fn dense_open_date_picker_story_model() -> DatePickerModel {
    DatePickerModel::new(2026, 8)
        .with_density(DatePickerDensity::Dense)
        .with_label("Dense picker")
        .with_placeholder("Pick day")
        .with_default_open(true)
}

fn loading_date_picker_story_model() -> DatePickerModel {
    default_date_picker_story_model()
        .with_default_open(true)
        .loading()
}

fn disabled_date_picker_story_model() -> DatePickerModel {
    DatePickerModel::new(2026, 9)
        .with_label("Locked date")
        .with_placeholder("Unavailable")
        .with_selected(CalendarDate::new(2026, 9, 15))
        .disabled()
}

fn invalid_date_picker_story_model() -> DatePickerModel {
    DatePickerModel::new(2026, 7).with_label("")
}

fn themed_date_picker_story_model() -> DatePickerModel {
    DatePickerModel::new(2026, 10)
        .with_label("Theme scoped date")
        .with_placeholder("Choose themed date")
        .with_selected(CalendarDate::new(2026, 10, 21))
        .with_default_open(true)
}

fn default_dialog_story_model() -> DialogModel {
    DialogModel::new(
        "Edit profile",
        "Edit profile",
        "Make changes to the shared profile.",
        "This story exercises trigger, content, header, title, description, body, and footer action nodes.",
    )
    .with_actions(vec![
        DialogAction::new("Save", "save-profile"),
        DialogAction::new("Cancel", "cancel-profile"),
    ])
}

fn open_small_dialog_story_model() -> DialogModel {
    DialogModel::new(
        "Open small dialog",
        "Small workflow",
        "A compact non-modal dialog stays open after preview.",
        "The primary footer action records local action state without closing the overlay.",
    )
    .with_size(DialogSize::Small)
    .with_mode(DialogMode::NonModal)
    .with_default_open(true)
    .with_actions(vec![
        DialogAction::new("Preview", "preview").keep_open(),
        DialogAction::new("Close", "close-preview"),
    ])
}

fn loading_dialog_story_model() -> DialogModel {
    default_dialog_story_model()
        .with_default_open(true)
        .loading()
}

fn disabled_dialog_story_model() -> DialogModel {
    DialogModel::new(
        "Locked dialog",
        "Locked workflow",
        "This dialog is disabled.",
        "The trigger and footer actions are disabled through the shared model.",
    )
    .with_default_open(true)
    .disabled()
}

fn invalid_dialog_story_model() -> DialogModel {
    DialogModel::new("Broken dialog", "", "Description", "Body")
}

fn themed_dialog_story_model() -> DialogModel {
    DialogModel::new(
        "Theme dialog",
        "Theme scoped dialog",
        "Semantic tokens resolve through the active theme.",
        "The same Dialog model drives Leptos DOM and Bevy primitive projections.",
    )
    .with_default_open(true)
    .with_actions(vec![
        DialogAction::new("Apply", "apply-theme"),
        DialogAction::new("Dismiss", "dismiss-theme"),
    ])
}

fn default_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "Application direction",
        "RTL article scope",
        "The default provider starts left-to-right while the nested scope can opt into right-to-left flow.",
    )
}

fn rtl_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "Arabic locale",
        "LTR code sample",
        "مرحبا بكم في واجهة تدعم اتجاه النص من اليمين إلى اليسار.",
    )
    .with_direction(DirectionValue::Rtl)
    .with_scope_direction(DirectionValue::Ltr)
}

fn scoped_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "LTR shell",
        "RTL preview pane",
        "A nested scope can override the provider direction without rebuilding the component tree.",
    )
    .with_default_scope_active(true)
}

fn loading_direction_story_model() -> DirectionModel {
    default_direction_story_model().loading()
}

fn disabled_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "Locked locale",
        "Locked scope",
        "Direction controls are disabled while app-level locale hydration is unavailable.",
    )
    .with_direction(DirectionValue::Rtl)
    .with_default_scope_active(true)
    .disabled()
}

fn invalid_direction_story_model() -> DirectionModel {
    DirectionModel::new("", "Broken scope", "Missing provider label")
}

fn themed_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "Theme direction",
        "Theme RTL scope",
        "Semantic tokens and direction attributes stay independent so theme switching does not break RTL rendering.",
    )
    .with_default_scope_active(true)
}

fn default_drawer_story_model() -> DrawerModel {
    DrawerModel::new(
        "Open task drawer",
        "Mobile task drawer",
        "Review a focused workflow in a bottom drawer.",
        "This story exercises trigger, content, header, footer, and handle nodes from one shared Drawer model.",
    )
    .with_default_open(true)
    .with_actions(vec![
        DrawerAction::new("Submit", "submit-task"),
        DrawerAction::new("Cancel", "cancel-task"),
    ])
}

fn right_drawer_story_model() -> DrawerModel {
    DrawerModel::new(
        "Open side drawer",
        "Side panel",
        "A right-side drawer uses the same model with side-specific primitive sizing.",
        "Side drawers are useful for filters, settings, or mobile navigation.",
    )
    .with_side(DrawerSide::Right)
    .with_default_open(true)
    .with_actions(vec![
        DrawerAction::new("Preview", "preview").keep_open(),
        DrawerAction::new("Close", "close-side"),
    ])
}

fn loading_drawer_story_model() -> DrawerModel {
    default_drawer_story_model().loading()
}

fn disabled_drawer_story_model() -> DrawerModel {
    DrawerModel::new(
        "Locked drawer",
        "Locked workflow",
        "The drawer is disabled while durable app state hydrates.",
        "Trigger, handle, and footer actions are disabled through the shared model.",
    )
    .with_default_open(true)
    .disabled()
}

fn invalid_drawer_story_model() -> DrawerModel {
    DrawerModel::new("Broken drawer", "", "Description", "Body")
}

fn themed_drawer_story_model() -> DrawerModel {
    DrawerModel::new(
        "Theme drawer",
        "Theme scoped drawer",
        "Drawer surfaces resolve semantic tokens through the active theme.",
        "The same Drawer model drives Leptos DOM and Bevy primitive projections.",
    )
    .with_side(DrawerSide::Left)
    .with_default_open(true)
    .with_actions(vec![
        DrawerAction::new("Apply", "apply-drawer"),
        DrawerAction::new("Dismiss", "dismiss-drawer"),
    ])
}

fn dropdown_menu_story_entries() -> Vec<DropdownMenuEntry> {
    vec![
        DropdownMenuEntry::label("File", "file"),
        DropdownMenuEntry::item(
            DropdownMenuItem::new("Rename", "rename")
                .with_detail("Update the component name.")
                .with_shortcut("R"),
        ),
        DropdownMenuEntry::item(
            DropdownMenuItem::new("Duplicate", "duplicate")
                .with_detail("Create a copy.")
                .with_shortcut("D"),
        ),
        DropdownMenuEntry::separator("file-separator"),
        DropdownMenuEntry::item(
            DropdownMenuItem::new("Delete", "delete")
                .with_detail("Remove this component.")
                .destructive(),
        ),
    ]
}

fn default_dropdown_menu_story_model() -> DropdownMenuModel {
    DropdownMenuModel::new(dropdown_menu_story_entries())
        .with_trigger_label("Open actions")
        .with_selected_value("rename")
        .with_active_value("duplicate")
}

fn dense_dropdown_menu_story_model() -> DropdownMenuModel {
    DropdownMenuModel::new(dropdown_menu_story_entries())
        .with_density(DropdownMenuDensity::Dense)
        .with_trigger_label("Dense actions")
        .with_selected_value("duplicate")
}

fn loading_dropdown_menu_story_model() -> DropdownMenuModel {
    default_dropdown_menu_story_model().loading()
}

fn disabled_dropdown_menu_story_model() -> DropdownMenuModel {
    DropdownMenuModel::new(dropdown_menu_story_entries())
        .with_trigger_label("Locked actions")
        .disabled()
}

fn invalid_dropdown_menu_story_model() -> DropdownMenuModel {
    DropdownMenuModel::new(vec![DropdownMenuEntry::separator("only-separator")])
}

fn themed_dropdown_menu_story_model() -> DropdownMenuModel {
    DropdownMenuModel::new(dropdown_menu_story_entries())
        .with_trigger_label("Theme actions")
        .with_content_label("Theme menu actions")
        .with_selected_value("delete")
}

fn default_empty_story_model() -> EmptyModel {
    EmptyModel::new(
        "No lessons queued",
        "The current filters do not match any scheduled review cards.",
    )
    .with_content("Clear the filter or create a new lesson to keep the queue moving.")
    .with_action(EmptyAction::new("Create lesson", "create-lesson"))
}

fn dense_empty_story_model() -> EmptyModel {
    EmptyModel::new(
        "No matches",
        "Dense empty states preserve the same anatomy with tighter spacing.",
    )
    .with_density(EmptyDensity::Dense)
    .with_illustration_label("0")
    .with_content("Try a broader search term.")
    .with_action(EmptyAction::new("Reset search", "reset-search"))
}

fn loading_empty_story_model() -> EmptyModel {
    default_empty_story_model().loading()
}

fn disabled_empty_story_model() -> EmptyModel {
    EmptyModel::new(
        "State locked",
        "The empty-state action is unavailable until durable state hydrates.",
    )
    .with_content("The copy remains visible while action affordances are disabled.")
    .with_action(EmptyAction::new("Create", "create-locked").disabled())
    .disabled()
}

fn invalid_empty_story_model() -> EmptyModel {
    EmptyModel::new("", "The validation boundary rejects empty titles.")
}

fn themed_empty_story_model() -> EmptyModel {
    EmptyModel::new(
        "Theme scoped empty",
        "The same semantic empty-state tokens resolve through a nested Forest theme.",
    )
    .with_illustration_label("UI")
    .with_content("Leptos DOM and Bevy primitives consume the same Empty render nodes.")
    .with_action(EmptyAction::new("Inspect", "inspect-empty-theme"))
}

fn default_field_story_model() -> FieldModel {
    FieldModel::new(
        "Project name",
        "This value stays renderer-local until a consumer persists form state.",
    )
    .with_placeholder("rs-dean-ui")
    .with_value("rs-dean")
    .required()
}

fn dense_field_story_model() -> FieldModel {
    FieldModel::new(
        "Search",
        "Dense field spacing keeps label, control, hint, and error anatomy stable.",
    )
    .with_density(FieldDensity::Dense)
    .with_input_kind(FieldInputKind::Search)
    .with_placeholder("Search components")
}

fn loading_field_story_model() -> FieldModel {
    default_field_story_model().loading()
}

fn disabled_field_story_model() -> FieldModel {
    FieldModel::new(
        "Locked slug",
        "Disabled fields keep copy visible while the control is unavailable.",
    )
    .with_value("stable-id")
    .disabled()
}

fn invalid_field_story_model() -> FieldModel {
    FieldModel::new("Email", "Use a reachable address for invitations.")
        .with_input_kind(FieldInputKind::Email)
        .with_placeholder("name@example.com")
        .with_error("Email address is required.")
        .required()
}

fn themed_field_story_model() -> FieldModel {
    FieldModel::new(
        "Theme token",
        "The same Field model drives Leptos control styling and Bevy primitive projection.",
    )
    .with_placeholder("surface-elevated")
    .with_value("brand")
}

fn default_hover_card_story_model() -> HoverCardModel {
    HoverCardModel::new(
        "Theme token",
        "Shared semantic preview",
        "The preview opens from local hover or focus state while durable app data stays outside the component.",
    )
    .with_metadata("Issue 29")
    .default_open()
}

fn dense_hover_card_story_model() -> HoverCardModel {
    HoverCardModel::new(
        "Dense preview",
        "Compact overlay",
        "Dense spacing keeps the trigger, content, and arrow anatomy stable across themes.",
    )
    .with_density(HoverCardDensity::Dense)
    .with_metadata("Dense")
    .default_open()
}

fn loading_hover_card_story_model() -> HoverCardModel {
    default_hover_card_story_model().loading()
}

fn disabled_hover_card_story_model() -> HoverCardModel {
    HoverCardModel::new(
        "Locked preview",
        "Unavailable content",
        "Disabled hover cards keep trigger copy visible while suppressing renderer-local open changes.",
    )
    .with_metadata("Disabled")
    .disabled()
}

fn invalid_hover_card_story_model() -> HoverCardModel {
    HoverCardModel::new(
        "",
        "Preview",
        "The validation boundary rejects empty trigger labels.",
    )
}

fn themed_hover_card_story_model() -> HoverCardModel {
    HoverCardModel::new(
        "Palette preview",
        "Theme scoped overlay",
        "The same Hover Card model resolves semantic tokens through the nested Luxury theme.",
    )
    .with_metadata("Luxury")
    .with_arrow_label("Theme arrow")
    .default_open()
}

fn default_input_story_model() -> InputModel {
    InputModel::new("engmanager.xyz")
        .with_input_kind(InputKind::Url)
        .with_value("engmanager.xyz")
        .with_prefix("https://")
        .with_suffix(InputAction::new("Copy", "copy-url"))
        .required()
}

fn dense_input_story_model() -> InputModel {
    InputModel::new("Search components")
        .with_density(InputDensity::Dense)
        .with_input_kind(InputKind::Search)
        .with_prefix("UI")
        .with_suffix(InputAction::new("Go", "search-components"))
}

fn loading_input_story_model() -> InputModel {
    default_input_story_model().loading()
}

fn disabled_input_story_model() -> InputModel {
    InputModel::new("Locked value")
        .with_value("stable-id")
        .with_prefix("id:")
        .with_suffix(InputAction::new("Copy", "copy-locked").disabled())
        .disabled()
}

fn invalid_input_story_model() -> InputModel {
    InputModel::new("Email")
        .with_input_kind(InputKind::Email)
        .with_prefix("@")
        .with_error("Email address is required.")
        .required()
}

fn themed_input_story_model() -> InputModel {
    InputModel::new("theme token")
        .with_input_kind(InputKind::Text)
        .with_value("surface-elevated")
        .with_prefix("token:")
        .with_suffix(InputAction::new("Apply", "apply-token"))
}

fn default_input_group_story_model() -> InputGroupModel {
    InputGroupModel::new("42")
        .with_value("42")
        .with_addon("$")
        .with_button(InputAction::new("Apply", "apply-amount"))
        .required()
}

fn dense_input_group_story_model() -> InputGroupModel {
    InputGroupModel::new("lesson slug")
        .with_density(InputDensity::Dense)
        .with_value("ui-input-group")
        .with_addon("slug")
        .with_button(InputAction::new("Save", "save-slug"))
}

fn loading_input_group_story_model() -> InputGroupModel {
    default_input_group_story_model().loading()
}

fn disabled_input_group_story_model() -> InputGroupModel {
    InputGroupModel::new("Locked")
        .with_value("read-only")
        .with_addon("id")
        .with_button(InputAction::new("Copy", "copy-read-only").disabled())
        .disabled()
}

fn invalid_input_group_story_model() -> InputGroupModel {
    InputGroupModel::new("Amount")
        .with_addon("$")
        .with_button(InputAction::new("Apply", "apply-invalid"))
        .with_error("Amount is required.")
        .required()
}

fn themed_input_group_story_model() -> InputGroupModel {
    InputGroupModel::new("theme token")
        .with_value("brand")
        .with_addon("token")
        .with_button(InputAction::new("Apply", "apply-theme-token"))
}

fn default_input_otp_story_model() -> InputOtpModel {
    InputOtpModel::new(6)
        .with_value("123")
        .with_group_size(3)
        .required()
}

fn dense_input_otp_story_model() -> InputOtpModel {
    InputOtpModel::new(6)
        .with_density(InputDensity::Dense)
        .with_value("42")
        .with_group_size(2)
}

fn loading_input_otp_story_model() -> InputOtpModel {
    default_input_otp_story_model().loading()
}

fn disabled_input_otp_story_model() -> InputOtpModel {
    InputOtpModel::new(6)
        .with_value("987654")
        .with_group_size(3)
        .disabled()
}

fn invalid_input_otp_story_model() -> InputOtpModel {
    InputOtpModel::new(6)
        .with_value("12")
        .with_error("Enter the full six digit code.")
        .required()
}

fn themed_input_otp_story_model() -> InputOtpModel {
    InputOtpModel::new(8)
        .alphanumeric()
        .with_value("A1B2")
        .with_group_size(4)
        .with_separator(" ")
        .with_label("Recovery code")
}

fn default_item_story_model() -> ItemModel {
    ItemModel::new(
        "Token migration",
        "Theme-aware rows use the same shared palette in Leptos and Bevy.",
    )
    .with_media("UI")
    .with_actions(vec![
        ItemAction::new("Open", "open-token-migration"),
        ItemAction::new("Queue", "queue-token-migration"),
    ])
}

fn dense_item_story_model() -> ItemModel {
    ItemModel::new(
        "Bevy primitive",
        "The same render node can become a scene item.",
    )
    .with_density(ItemDensity::Dense)
    .with_media("B")
    .with_actions(vec![ItemAction::new("Inspect", "inspect-bevy-primitive")])
}

fn loading_item_story_model() -> ItemModel {
    default_item_story_model().loading()
}

fn disabled_item_story_model() -> ItemModel {
    ItemModel::new("Locked item", "This row is visible but unavailable.")
        .with_media("L")
        .with_actions(vec![ItemAction::new("Open", "open-locked-item").disabled()])
        .disabled()
}

fn invalid_item_story_model() -> ItemModel {
    ItemModel::new(
        "Missing owner",
        "Assign an owner before this item can advance.",
    )
    .with_media("!")
    .with_actions(vec![ItemAction::new("Resolve", "resolve-missing-owner")])
    .with_error("Owner is required.")
}

fn themed_item_story_model() -> ItemModel {
    ItemModel::new(
        "Theme scoped row",
        "Semantic tokens keep the action surface portable.",
    )
    .with_media("R")
    .with_actions(vec![
        ItemAction::new("Open", "open-theme-row"),
        ItemAction::new("Pin", "pin-theme-row"),
    ])
}

fn default_kbd_story_model() -> KbdModel {
    KbdModel::new(vec![KbdKey::new("Cmd"), KbdKey::new("K")])
        .with_separator(" + ")
        .with_aria_label("Open command menu")
}

fn dense_kbd_story_model() -> KbdModel {
    KbdModel::new(vec![KbdKey::new("Shift"), KbdKey::new("P")])
        .with_density(KbdDensity::Dense)
        .with_separator(" + ")
        .with_aria_label("Open palette")
}

fn loading_kbd_story_model() -> KbdModel {
    default_kbd_story_model().loading()
}

fn disabled_kbd_story_model() -> KbdModel {
    KbdModel::new(vec![
        KbdKey::new("Ctrl"),
        KbdKey::new("S").with_value("save"),
    ])
    .with_separator(" + ")
    .with_aria_label("Save disabled")
    .disabled()
}

fn invalid_kbd_story_model() -> KbdModel {
    KbdModel::new(vec![KbdKey::new("Alt"), KbdKey::new("Enter")])
        .with_separator(" + ")
        .with_error("Shortcut is already assigned.")
}

fn themed_kbd_story_model() -> KbdModel {
    KbdModel::new(vec![
        KbdKey::new("Ctrl"),
        KbdKey::new("Shift"),
        KbdKey::new("D"),
    ])
    .with_separator(" + ")
    .with_aria_label("Toggle diagnostics")
}

fn default_label_story_model() -> LabelModel {
    LabelModel::new("Email").with_for("email").required()
}

fn dense_label_story_model() -> LabelModel {
    LabelModel::new("Username")
        .with_density(LabelDensity::Dense)
        .with_for("username")
        .optional()
}

fn loading_label_story_model() -> LabelModel {
    default_label_story_model().loading()
}

fn disabled_label_story_model() -> LabelModel {
    LabelModel::new("Archived email")
        .with_for("archived_email")
        .required()
        .disabled()
}

fn invalid_label_story_model() -> LabelModel {
    LabelModel::new("Workspace")
        .with_for("workspace")
        .required()
        .with_error("Workspace is required.")
}

fn themed_label_story_model() -> LabelModel {
    LabelModel::new("Theme token").with_requirement(LabelRequirement::Optional)
}

fn default_marker_story_model() -> MarkerModel {
    MarkerModel::new("3 new")
        .with_value("unread")
        .with_dot_label("Unread messages")
        .with_anchor(MarkerAnchor::new("Jump", "#latest-message"))
}

fn dense_marker_story_model() -> MarkerModel {
    MarkerModel::new("Pinned")
        .with_density(MarkerDensity::Dense)
        .with_tone(MarkerTone::Success)
        .with_value("pinned")
        .with_dot_label("Pinned marker")
        .without_anchor()
}

fn loading_marker_story_model() -> MarkerModel {
    default_marker_story_model().loading()
}

fn disabled_marker_story_model() -> MarkerModel {
    MarkerModel::new("Archived")
        .with_tone(MarkerTone::Neutral)
        .with_value("archived")
        .with_dot_label("Archived marker")
        .with_anchor(MarkerAnchor::new("Open", "#archived").disabled())
        .disabled()
}

fn invalid_marker_story_model() -> MarkerModel {
    MarkerModel::new("Stale")
        .with_tone(MarkerTone::Danger)
        .with_value("stale")
        .with_dot_label("Stale marker")
        .with_error("Marker target is stale.")
}

fn themed_marker_story_model() -> MarkerModel {
    MarkerModel::new("2 notes")
        .with_tone(MarkerTone::Warning)
        .with_value("notes")
        .with_dot_label("Annotation marker")
        .with_anchor(MarkerAnchor::new("Review", "#review-notes"))
}

fn default_menubar_story_model() -> MenubarModel {
    MenubarModel::new(vec![
        MenubarMenu::new(
            "File",
            "file",
            vec![
                MenubarItem::new("New project", "new-project").with_shortcut("N"),
                MenubarItem::new("Import deck", "import-deck").with_shortcut("I"),
            ],
        ),
        MenubarMenu::new(
            "Edit",
            "edit",
            vec![
                MenubarItem::new("Undo", "undo").with_shortcut("Z"),
                MenubarItem::new("Redo", "redo").with_shortcut("R"),
            ],
        ),
    ])
}

fn dense_menubar_story_model() -> MenubarModel {
    default_menubar_story_model().with_density(MenubarDensity::Dense)
}

fn loading_menubar_story_model() -> MenubarModel {
    default_menubar_story_model().loading()
}

fn disabled_menubar_story_model() -> MenubarModel {
    default_menubar_story_model().disabled()
}

fn invalid_menubar_story_model() -> MenubarModel {
    default_menubar_story_model().with_error("Menu commands are unavailable.")
}

fn themed_menubar_story_model() -> MenubarModel {
    MenubarModel::new(vec![
        MenubarMenu::new(
            "View",
            "view",
            vec![
                MenubarItem::new("Focus mode", "focus-mode").with_shortcut("F"),
                MenubarItem::new("Command log", "command-log").with_shortcut("L"),
            ],
        ),
        MenubarMenu::new(
            "Tools",
            "tools",
            vec![
                MenubarItem::new("Sync", "sync").with_shortcut("S"),
                MenubarItem::new("Inspect", "inspect").with_shortcut("I"),
            ],
        ),
    ])
    .with_default_open("tools")
}

fn default_message_story_model() -> MessageModel {
    MessageModel::new(
        "Codex",
        "Today at 9:41",
        "The next component is ready for sweep review.",
        "Delivered",
    )
    .with_actions(vec![
        MessageAction::new("Reply", "reply"),
        MessageAction::new("Resolve", "resolve"),
    ])
}

fn dense_outgoing_message_story_model() -> MessageModel {
    MessageModel::new(
        "Matthew",
        "Today at 9:42",
        "Ship the contract first, then sweep the earlier components.",
        "Read",
    )
    .with_density(MessageDensity::Dense)
    .with_side(MessageSide::Outgoing)
    .with_actions(vec![MessageAction::new("Edit", "edit")])
}

fn loading_message_story_model() -> MessageModel {
    default_message_story_model().loading()
}

fn disabled_message_story_model() -> MessageModel {
    MessageModel::new(
        "Archive",
        "Yesterday",
        "This transcript entry is locked by the durable state owner.",
        "Archived",
    )
    .with_side(MessageSide::System)
    .with_actions(vec![MessageAction::new("Open", "open-archive").disabled()])
    .disabled()
}

fn invalid_message_story_model() -> MessageModel {
    MessageModel::new(
        "Sync",
        "Retry required",
        "The persisted transcript did not accept this update.",
        "Failed",
    )
    .with_side(MessageSide::Outgoing)
    .with_error("Message failed to persist.")
}

fn themed_message_story_model() -> MessageModel {
    MessageModel::new(
        "Theme runner",
        "Token preview",
        "Message colors, type, spacing, radius, and actions resolve through the shared theme.",
        "Ready",
    )
    .with_side(MessageSide::Incoming)
    .with_actions(vec![MessageAction::new("Inspect", "inspect-theme")])
}

fn default_message_scroller_story_model() -> MessageScrollerModel {
    MessageScrollerModel::new(vec![
        MessageScrollerEntry::new(
            "codex-ready",
            MessageModel::new(
                "Codex",
                "Today at 9:41",
                "Message Scroller is ready for sweep review.",
                "Delivered",
            )
            .with_actions(vec![MessageAction::new("Reply", "reply")]),
        ),
        MessageScrollerEntry::new(
            "matthew-plan",
            MessageModel::new(
                "Matthew",
                "Today at 9:42",
                "Keep transcript ownership durable and scroll position renderer-local.",
                "Read",
            )
            .with_side(MessageSide::Outgoing)
            .with_actions(vec![MessageAction::new("Edit", "edit")]),
        ),
    ])
}

fn dense_latest_message_scroller_story_model() -> MessageScrollerModel {
    default_message_scroller_story_model()
        .with_density(MessageScrollerDensity::Dense)
        .with_at_latest(true)
        .manual_scroll()
}

fn loading_message_scroller_story_model() -> MessageScrollerModel {
    default_message_scroller_story_model().loading()
}

fn disabled_message_scroller_story_model() -> MessageScrollerModel {
    default_message_scroller_story_model()
        .with_anchor_label("Transcript locked")
        .with_jump_label("Locked")
        .disabled()
}

fn invalid_message_scroller_story_model() -> MessageScrollerModel {
    default_message_scroller_story_model().with_error("Transcript hydration failed.")
}

fn themed_message_scroller_story_model() -> MessageScrollerModel {
    MessageScrollerModel::new(vec![
        MessageScrollerEntry::new(
            "theme-runner",
            MessageModel::new(
                "Theme runner",
                "Token preview",
                "Scroller chrome, nested messages, and jump controls all resolve through the shared theme.",
                "Ready",
            )
            .with_actions(vec![MessageAction::new("Inspect", "inspect")]),
        ),
        MessageScrollerEntry::new(
            "designer",
            MessageModel::new(
                "Designer",
                "Follow-up",
                "The latest anchor should stay visible across theme switches.",
                "Queued",
            )
            .with_side(MessageSide::Outgoing)
            .with_actions(vec![MessageAction::new("Pin", "pin")]),
        ),
    ])
    .with_jump_label("Latest")
}

fn default_native_select_story_model() -> NativeSelectModel {
    NativeSelectModel::new(vec![
        NativeSelectOption::new("Leptos DOM", "leptos")
            .with_detail("Render the component through the Leptos browser surface."),
        NativeSelectOption::new("Bevy WebGPU", "bevy")
            .with_detail("Render the same shared slots as Bevy UI primitives."),
        NativeSelectOption::new("Shared contract", "shared")
            .with_detail("Keep durable selection ownership above the component."),
    ])
    .with_label("Renderer")
    .with_placeholder("Choose renderer")
    .with_selected_value("leptos")
}

fn dense_native_select_story_model() -> NativeSelectModel {
    default_native_select_story_model()
        .with_density(NativeSelectDensity::Dense)
        .with_label("Dense renderer")
        .with_selected_value("bevy")
}

fn loading_native_select_story_model() -> NativeSelectModel {
    default_native_select_story_model().loading()
}

fn disabled_native_select_story_model() -> NativeSelectModel {
    default_native_select_story_model()
        .with_label("Locked renderer")
        .disabled()
}

fn invalid_native_select_story_model() -> NativeSelectModel {
    default_native_select_story_model()
        .without_selected_value()
        .required()
        .with_error("Select a renderer before this state can persist.")
}

fn themed_native_select_story_model() -> NativeSelectModel {
    NativeSelectModel::new(vec![
        NativeSelectOption::new("Catppuccin", "catppuccin")
            .with_detail("Preview the select in the Catppuccin theme scope."),
        NativeSelectOption::new("Dracula", "dracula")
            .with_detail("Preview the select in the Dracula theme scope."),
        NativeSelectOption::new("Luxury", "luxury")
            .with_detail("Preview the select in the Luxury theme scope.")
            .disabled(),
    ])
    .with_label("Theme")
    .with_placeholder("Choose theme")
    .with_selected_value("catppuccin")
}

fn default_navigation_menu_story_model() -> NavigationMenuModel {
    NavigationMenuModel::new(vec![
        NavigationMenuItem::link(
            "Docs",
            "docs",
            "/docs",
            "Implementation guides and architecture notes.",
        ),
        NavigationMenuItem::panel(
            "Components",
            "components",
            "Browse shared token-only UI primitives.",
            vec![
                NavigationMenuLink::new(
                    "Accordion",
                    "accordion",
                    "/components/accordion",
                    "Disclosure state rendered through shared Rust nodes.",
                ),
                NavigationMenuLink::new(
                    "Native Select",
                    "native-select",
                    "/components/native-select",
                    "Browser-native select with shared option validation.",
                ),
                NavigationMenuLink::new(
                    "Navigation Menu",
                    "navigation-menu",
                    "/components/navigation-menu",
                    "Top-level menus with local panel state.",
                ),
            ],
        ),
        NavigationMenuItem::link(
            "Stories",
            "stories",
            "/stories",
            "Open the reusable UI story harness.",
        ),
    ])
    .with_default_open("components")
    .with_selected_value("native-select")
}

fn dense_navigation_menu_story_model() -> NavigationMenuModel {
    default_navigation_menu_story_model()
        .with_density(NavigationMenuDensity::Dense)
        .with_selected_value("docs")
}

fn loading_navigation_menu_story_model() -> NavigationMenuModel {
    default_navigation_menu_story_model().loading()
}

fn disabled_navigation_menu_story_model() -> NavigationMenuModel {
    default_navigation_menu_story_model()
        .with_label("Locked navigation")
        .disabled()
}

fn invalid_navigation_menu_story_model() -> NavigationMenuModel {
    default_navigation_menu_story_model().with_error("Navigation hydration failed.")
}

fn themed_navigation_menu_story_model() -> NavigationMenuModel {
    NavigationMenuModel::new(vec![
        NavigationMenuItem::link(
            "Overview",
            "overview",
            "/overview",
            "Theme scoped navigation overview.",
        ),
        NavigationMenuItem::panel(
            "Themes",
            "themes",
            "Preview palettes through the shared token surface.",
            vec![
                NavigationMenuLink::new(
                    "Luxury",
                    "luxury",
                    "/themes/luxury",
                    "Warm high-contrast palette.",
                ),
                NavigationMenuLink::new(
                    "Dracula",
                    "dracula",
                    "/themes/dracula",
                    "Dark saturated palette.",
                ),
                NavigationMenuLink::new(
                    "Catppuccin",
                    "catppuccin",
                    "/themes/catppuccin",
                    "Soft balanced palette.",
                )
                .disabled(),
            ],
        ),
    ])
    .with_label("Theme navigation")
    .with_default_open("themes")
    .with_selected_value("luxury")
}

fn default_pagination_story_model() -> PaginationModel {
    PaginationModel::new(12, 5)
}

fn dense_pagination_story_model() -> PaginationModel {
    PaginationModel::new(6, 2)
        .with_density(PaginationDensity::Dense)
        .with_sibling_count(2)
}

fn loading_pagination_story_model() -> PaginationModel {
    default_pagination_story_model().loading()
}

fn disabled_pagination_story_model() -> PaginationModel {
    PaginationModel::new(3, 1)
        .with_previous_label("Back")
        .with_next_label("Forward")
        .disabled()
}

fn invalid_pagination_story_model() -> PaginationModel {
    default_pagination_story_model().with_error("Page range failed to hydrate.")
}

fn themed_pagination_story_model() -> PaginationModel {
    PaginationModel::new(9, 7)
        .with_sibling_count(1)
        .with_previous_label("Earlier")
        .with_next_label("Later")
}

fn default_popover_story_model() -> PopoverModel {
    PopoverModel::new(
        "Open controls",
        "Shared overlay controls",
        "Popover state stays renderer-local while durable choices remain with the consuming app.",
    )
    .with_eyebrow("Issue 43")
}

fn dense_popover_story_model() -> PopoverModel {
    PopoverModel::new(
        "Dense controls",
        "Compact overlay",
        "Dense spacing keeps the trigger, content, and arrow anatomy stable across themes.",
    )
    .with_density(PopoverDensity::Dense)
    .with_eyebrow("Dense")
}

fn loading_popover_story_model() -> PopoverModel {
    default_popover_story_model().loading()
}

fn disabled_popover_story_model() -> PopoverModel {
    PopoverModel::new(
        "Locked controls",
        "Unavailable overlay",
        "Disabled popovers keep trigger copy visible while suppressing local open changes.",
    )
    .with_eyebrow("Disabled")
    .disabled()
}

fn invalid_popover_story_model() -> PopoverModel {
    default_popover_story_model().with_error("Popover content failed validation upstream.")
}

fn themed_popover_story_model() -> PopoverModel {
    PopoverModel::new(
        "Palette controls",
        "Theme scoped overlay",
        "The same Popover model resolves semantic tokens through the nested Luxury theme.",
    )
    .with_eyebrow("Luxury")
    .with_arrow_label("Theme arrow")
}

fn default_progress_story_model() -> ProgressModel {
    ProgressModel::new(64)
        .with_label("Upload")
        .with_detail("64 percent complete")
}

fn dense_progress_story_model() -> ProgressModel {
    ProgressModel::new(38)
        .with_density(ProgressDensity::Dense)
        .with_label("Sync")
        .with_detail("38 percent complete")
}

fn loading_progress_story_model() -> ProgressModel {
    default_progress_story_model().loading()
}

fn disabled_progress_story_model() -> ProgressModel {
    ProgressModel::new(22)
        .with_label("Locked task")
        .with_detail("Progress is visible but not focus-highlightable.")
        .disabled()
}

fn invalid_progress_story_model() -> ProgressModel {
    ProgressModel::new(72)
        .with_label("Import")
        .with_detail("Import progress is paused.")
        .with_error("The consumer reported a recoverable progress error.")
}

fn indeterminate_progress_story_model() -> ProgressModel {
    ProgressModel::indeterminate()
        .with_label("Preparing")
        .with_detail("The consumer has not provided a determinate value yet.")
}

fn themed_progress_story_model() -> ProgressModel {
    ProgressModel::new(86)
        .with_label("Theme build")
        .with_detail("86 percent complete inside the Catppuccin theme scope.")
}

fn radio_group_story_options() -> Vec<RadioGroupOption> {
    vec![
        RadioGroupOption::new("Light", "light").with_detail("Use light semantic tokens."),
        RadioGroupOption::new("Dark", "dark").with_detail("Use dark semantic tokens."),
        RadioGroupOption::new("System", "system").with_detail("Follow the device setting."),
    ]
}

fn default_radio_group_story_model() -> RadioGroupModel {
    RadioGroupModel::new(radio_group_story_options())
        .with_label("Theme preference")
        .with_selected_value("light")
        .required()
}

fn dense_radio_group_story_model() -> RadioGroupModel {
    RadioGroupModel::new(radio_group_story_options())
        .with_density(RadioGroupDensity::Dense)
        .with_label("Compact theme")
        .with_selected_value("dark")
}

fn loading_radio_group_story_model() -> RadioGroupModel {
    default_radio_group_story_model().loading()
}

fn disabled_radio_group_story_model() -> RadioGroupModel {
    RadioGroupModel::new(radio_group_story_options())
        .with_label("Locked rollout")
        .with_selected_value("system")
        .disabled()
}

fn invalid_radio_group_story_model() -> RadioGroupModel {
    default_radio_group_story_model().with_error("A selected value must be persisted upstream.")
}

fn themed_radio_group_story_model() -> RadioGroupModel {
    RadioGroupModel::new(radio_group_story_options())
        .with_label("Theme scope")
        .with_orientation(RadioGroupOrientation::Horizontal)
        .with_selected_value("dark")
}

fn resizable_story_panels() -> Vec<ResizablePanel> {
    vec![
        ResizablePanel::new("Outline", "outline", 34)
            .with_detail("Navigation and lesson structure.")
            .with_bounds(20, 70),
        ResizablePanel::new("Workspace", "workspace", 66)
            .with_detail("Primary activity and preview surface.")
            .with_bounds(30, 80),
    ]
}

fn three_panel_resizable_story_panels() -> Vec<ResizablePanel> {
    vec![
        ResizablePanel::new("Queue", "queue", 24)
            .with_detail("Local-first pending items.")
            .with_bounds(15, 45),
        ResizablePanel::new("Editor", "editor", 46)
            .with_detail("Focused work panel.")
            .with_bounds(30, 65),
        ResizablePanel::new("Inspector", "inspector", 30)
            .with_detail("Context and metadata.")
            .with_bounds(20, 45),
    ]
}

fn default_resizable_story_model() -> ResizableModel {
    ResizableModel::new(resizable_story_panels())
        .with_label("Workspace split")
        .with_active_panel("workspace")
}

fn dense_resizable_story_model() -> ResizableModel {
    ResizableModel::new(resizable_story_panels())
        .with_density(ResizableDensity::Dense)
        .with_label("Compact split")
}

fn vertical_resizable_story_model() -> ResizableModel {
    ResizableModel::new(three_panel_resizable_story_panels())
        .with_orientation(ResizableOrientation::Vertical)
        .with_label("Stacked panels")
        .with_resizing_handle(1)
}

fn loading_resizable_story_model() -> ResizableModel {
    default_resizable_story_model().loading()
}

fn disabled_resizable_story_model() -> ResizableModel {
    ResizableModel::new(resizable_story_panels())
        .with_label("Locked split")
        .disabled()
}

fn invalid_resizable_story_model() -> ResizableModel {
    default_resizable_story_model().with_error("Panel layout must be reviewed before persistence.")
}

fn themed_resizable_story_model() -> ResizableModel {
    ResizableModel::new(three_panel_resizable_story_panels()).with_label("Theme scoped split")
}

fn scroll_area_story_items() -> Vec<ScrollAreaItem> {
    vec![
        ScrollAreaItem::new("Queue lesson", "queue")
            .with_detail("Prepare the next lesson from local state."),
        ScrollAreaItem::new("Hydrate app", "hydrate")
            .with_detail("Read durable state before rendering progress."),
        ScrollAreaItem::new("Render story", "render")
            .with_detail("Prove the component in the isolated story harness."),
        ScrollAreaItem::new("Verify gate", "verify")
            .with_detail("Run the one-pass Rust quality gate."),
        ScrollAreaItem::new("Publish artifact", "publish")
            .with_detail("Emit static Pages output after the gate passes."),
    ]
}

fn default_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_label("Activity feed")
        .with_active_item("hydrate")
}

fn dense_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_density(ScrollAreaDensity::Dense)
        .with_label("Compact feed")
}

fn horizontal_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_label("Overflow lanes")
        .with_overflow(ScrollAreaOverflow::Both)
        .with_active_item("render")
}

fn loading_scroll_area_story_model() -> ScrollAreaModel {
    default_scroll_area_story_model().loading()
}

fn disabled_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_label("Locked feed")
        .disabled()
}

fn invalid_scroll_area_story_model() -> ScrollAreaModel {
    default_scroll_area_story_model().with_error("Scroll content failed validation upstream.")
}

fn themed_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_label("Theme scoped feed")
        .with_overflow(ScrollAreaOverflow::Horizontal)
}

fn select_story_groups() -> Vec<SelectGroup> {
    vec![
        SelectGroup::new(
            "Application surfaces",
            "surfaces",
            vec![
                SelectOption::new("Marketing app", "marketing")
                    .with_detail("Leptos static marketing surface."),
                SelectOption::new("Story harness", "stories")
                    .with_detail("Isolated reusable component proof."),
            ],
        ),
        SelectGroup::new(
            "Renderers",
            "renderers",
            vec![
                SelectOption::new("Leptos DOM", "leptos")
                    .with_detail("Token-only DOM component rendering."),
                SelectOption::new("Bevy WebGPU", "bevy").with_detail("Scene primitive derivation."),
            ],
        ),
    ]
}

fn default_select_story_model() -> SelectModel {
    SelectModel::new(select_story_groups())
        .with_label("Renderer target")
        .with_placeholder("Choose target")
        .with_selected_value("leptos")
        .required()
}

fn dense_select_story_model() -> SelectModel {
    default_select_story_model()
        .with_density(SelectDensity::Dense)
        .with_selected_value("stories")
}

fn loading_select_story_model() -> SelectModel {
    default_select_story_model().loading()
}

fn disabled_select_story_model() -> SelectModel {
    default_select_story_model()
        .with_label("Locked target")
        .with_selected_value("marketing")
        .disabled()
}

fn invalid_select_story_model() -> SelectModel {
    default_select_story_model()
        .without_selected_value()
        .with_error("A renderer target is required before this state can persist.")
}

fn themed_select_story_model() -> SelectModel {
    SelectModel::new(select_story_groups())
        .with_label("Theme scoped target")
        .with_selected_value("bevy")
}

fn default_separator_story_model() -> SeparatorModel {
    SeparatorModel::new("Content boundary")
        .with_detail("Separates related component regions with shared theme tokens.")
}

fn dense_separator_story_model() -> SeparatorModel {
    default_separator_story_model()
        .with_density(SeparatorDensity::Dense)
        .with_label("Compact boundary")
}

fn vertical_separator_story_model() -> SeparatorModel {
    SeparatorModel::new("Rail divider")
        .with_orientation(SeparatorOrientation::Vertical)
        .with_detail("A vertical divider for split navigation and content rails.")
}

fn loading_separator_story_model() -> SeparatorModel {
    default_separator_story_model()
        .with_label("Publishing boundary")
        .loading()
}

fn disabled_separator_story_model() -> SeparatorModel {
    default_separator_story_model()
        .with_label("Locked boundary")
        .disabled()
}

fn invalid_separator_story_model() -> SeparatorModel {
    default_separator_story_model().with_error("Separator metadata failed validation upstream.")
}

fn themed_separator_story_model() -> SeparatorModel {
    SeparatorModel::new("Theme scoped boundary")
        .with_density(SeparatorDensity::Dense)
        .decorative()
}

fn default_sheet_story_model() -> SheetModel {
    SheetModel::new(
        "Open settings",
        "Project settings",
        "Update scoped preferences without leaving the current workflow.",
        "The sheet keeps transient panel state local while consumers own persisted settings.",
    )
    .with_default_open(true)
}

fn dense_sheet_story_model() -> SheetModel {
    default_sheet_story_model()
        .with_density(SheetDensity::Dense)
        .with_actions(vec![
            SheetAction::new("Apply", "apply"),
            SheetAction::new("Close", "close").close_sheet(),
        ])
}

fn left_sheet_story_model() -> SheetModel {
    SheetModel::new(
        "Open navigation",
        "Navigation rail",
        "A left-attached sheet for secondary navigation workflows.",
        "Edge placement is part of the shared Rust model so Leptos and Bevy agree.",
    )
    .with_side(SheetSide::Left)
    .with_default_open(true)
}

fn loading_sheet_story_model() -> SheetModel {
    default_sheet_story_model().loading()
}

fn disabled_sheet_story_model() -> SheetModel {
    default_sheet_story_model()
        .with_close_label("Locked")
        .disabled()
}

fn invalid_sheet_story_model() -> SheetModel {
    SheetModel::new("", "Missing trigger", "Invalid sheet metadata", "Invalid")
}

fn themed_sheet_story_model() -> SheetModel {
    SheetModel::new(
        "Open theme sheet",
        "Theme scoped sheet",
        "The same sheet tokens resolve inside the nested Dracula theme.",
        "Theme switching stays token-driven across the overlay surface.",
    )
    .with_side(SheetSide::Bottom)
    .with_default_open(true)
}

fn sidebar_story_groups() -> Vec<SidebarGroup> {
    vec![
        SidebarGroup::new(
            "Workspace",
            "workspace",
            vec![
                SidebarItem::new("Overview", "overview"),
                SidebarItem::new("Stories", "stories").with_badge("64"),
                SidebarItem::new("Gate", "gate"),
            ],
        ),
        SidebarGroup::new(
            "Build",
            "build",
            vec![
                SidebarItem::new("Components", "components"),
                SidebarItem::new("Themes", "themes"),
                SidebarItem::new("Bevy scenes", "bevy").with_badge("GPU"),
            ],
        ),
    ]
}

fn default_sidebar_story_model() -> SidebarModel {
    SidebarModel::new(sidebar_story_groups())
        .with_header("rs-dean", "Rust/WASM workspace")
        .with_active_value("stories")
}

fn dense_sidebar_story_model() -> SidebarModel {
    default_sidebar_story_model()
        .with_density(SidebarDensity::Dense)
        .with_active_value("components")
}

fn collapsed_sidebar_story_model() -> SidebarModel {
    default_sidebar_story_model().collapsed()
}

fn loading_sidebar_story_model() -> SidebarModel {
    default_sidebar_story_model().loading()
}

fn disabled_sidebar_story_model() -> SidebarModel {
    default_sidebar_story_model()
        .with_footer("Locked", "Navigation waits for durable state hydration.")
        .disabled()
}

fn invalid_sidebar_story_model() -> SidebarModel {
    SidebarModel::new(Vec::new())
}

fn themed_sidebar_story_model() -> SidebarModel {
    SidebarModel::new(sidebar_story_groups())
        .with_label("Theme scoped navigation")
        .with_active_value("themes")
}

fn default_skeleton_story_model() -> SkeletonModel {
    SkeletonModel::new("Loading component summary")
        .with_block_label("Title area")
        .with_text_label("Summary copy")
        .with_media_label("Preview frame")
        .with_detail("A placeholder keeps the final layout stable while data hydrates.")
}

fn dense_skeleton_story_model() -> SkeletonModel {
    default_skeleton_story_model()
        .with_density(SkeletonDensity::Dense)
        .with_text_lines(2)
        .with_block_label("Compact title")
}

fn ready_skeleton_story_model() -> SkeletonModel {
    default_skeleton_story_model()
        .with_detail("Content is ready, so placeholder parts stay hidden without layout churn.")
        .ready()
}

fn static_skeleton_story_model() -> SkeletonModel {
    default_skeleton_story_model()
        .with_detail("Motion can be disabled while preserving the same placeholder geometry.")
        .with_text_lines(4)
        .static_placeholder()
}

fn disabled_skeleton_story_model() -> SkeletonModel {
    default_skeleton_story_model()
        .with_detail("The loading surface is locked while the owner resolves durable state.")
        .disabled()
}

fn invalid_skeleton_story_model() -> SkeletonModel {
    default_skeleton_story_model()
        .with_error("Skeleton layout metadata failed validation before render.")
}

fn themed_skeleton_story_model() -> SkeletonModel {
    SkeletonModel::new("Theme scoped placeholder")
        .with_density(SkeletonDensity::Dense)
        .with_text_lines(3)
        .with_detail("Semantic placeholder colors resolve through the nested theme.")
}

fn default_slider_story_model() -> SliderModel {
    SliderModel::new(0, 100, 64)
        .with_label("Completion")
        .with_step(4)
        .with_detail("Adjust a local value while the app decides whether to persist it.")
}

fn dense_slider_story_model() -> SliderModel {
    default_slider_story_model()
        .with_density(SliderDensity::Dense)
        .with_value(40)
}

fn vertical_slider_story_model() -> SliderModel {
    SliderModel::new(0, 12, 8)
        .with_label("Columns")
        .with_unit(" columns")
        .with_step(1)
        .with_orientation(SliderOrientation::Vertical)
        .with_detail("Vertical orientation remains part of the shared Rust model.")
}

fn loading_slider_story_model() -> SliderModel {
    default_slider_story_model()
        .with_label("Hydrating value")
        .loading()
}

fn disabled_slider_story_model() -> SliderModel {
    default_slider_story_model()
        .with_label("Locked threshold")
        .with_value(32)
        .disabled()
}

fn invalid_slider_story_model() -> SliderModel {
    default_slider_story_model()
        .with_error("Slider value cannot be persisted until the range is reconciled.")
}

fn themed_slider_story_model() -> SliderModel {
    SliderModel::new(0, 100, 80)
        .with_label("Theme scoped range")
        .with_step(5)
        .with_detail("Slider colors resolve from the active theme tokens.")
}

fn default_sonner_story_model() -> SonnerModel {
    SonnerModel::new(vec![
        SonnerToast::new(
            "saved",
            "Project saved",
            "The local draft is ready for the next gate run.",
        )
        .with_tone(SonnerTone::Success)
        .with_action(SonnerAction::new("Undo", "undo-save")),
        SonnerToast::new(
            "queued",
            "Gate queued",
            "The branch is waiting on the full validation pass.",
        )
        .with_tone(SonnerTone::Info),
    ])
}

fn dense_sonner_story_model() -> SonnerModel {
    default_sonner_story_model()
        .with_density(SonnerDensity::Dense)
        .with_label("Dense notifications")
}

fn centered_sonner_story_model() -> SonnerModel {
    SonnerModel::new(vec![
        SonnerToast::new(
            "centered",
            "Centered viewport",
            "Position is shared state, while actual placement belongs to the renderer.",
        )
        .with_tone(SonnerTone::Warning)
        .with_action(SonnerAction::new("Review", "review-position")),
    ])
    .with_position(SonnerPosition::BottomCenter)
}

fn loading_sonner_story_model() -> SonnerModel {
    default_sonner_story_model()
        .with_label("Publishing notifications")
        .loading()
}

fn disabled_sonner_story_model() -> SonnerModel {
    SonnerModel::new(vec![
        SonnerToast::new(
            "locked",
            "Notifications locked",
            "Actions are disabled while durable app state is reconciling.",
        )
        .with_tone(SonnerTone::Destructive)
        .with_action(SonnerAction::new("Retry", "retry-locked").disabled()),
    ])
    .disabled()
}

fn invalid_sonner_story_model() -> SonnerModel {
    default_sonner_story_model()
        .with_error("Toast delivery failed validation before the renderer accepted it.")
}

fn themed_sonner_story_model() -> SonnerModel {
    SonnerModel::new(vec![
        SonnerToast::new(
            "theme",
            "Theme scoped toast",
            "Toast severity colors resolve through the nested Dracula theme.",
        )
        .with_tone(SonnerTone::Success)
        .with_action(SonnerAction::new("Inspect", "inspect-theme-toast")),
    ])
    .with_position(SonnerPosition::TopRight)
}

fn default_spinner_story_model() -> SpinnerModel {
    SpinnerModel::new("Loading components")
        .with_detail("A compact busy indicator that keeps motion state renderer-local.")
}

fn dense_spinner_story_model() -> SpinnerModel {
    default_spinner_story_model()
        .with_density(SpinnerDensity::Dense)
        .with_size(SpinnerSize::Small)
        .with_tone(SpinnerTone::Info)
        .with_detail("Dense spinners preserve the same shared anatomy in less space.")
}

fn large_spinner_story_model() -> SpinnerModel {
    default_spinner_story_model()
        .with_size(SpinnerSize::Large)
        .with_tone(SpinnerTone::Warning)
        .with_speed_ms(1_200)
        .with_detail("Size and rotation speed are validated in the shared Rust model.")
}

fn ready_spinner_story_model() -> SpinnerModel {
    default_spinner_story_model()
        .with_tone(SpinnerTone::Success)
        .with_detail("Ready state hides the active indicator while preserving label anatomy.")
        .ready()
}

fn disabled_spinner_story_model() -> SpinnerModel {
    default_spinner_story_model()
        .with_tone(SpinnerTone::Destructive)
        .with_detail("Disabled spinners pause all renderer-local motion controls.")
        .disabled()
}

fn invalid_spinner_story_model() -> SpinnerModel {
    default_spinner_story_model()
        .with_error("Spinner metadata failed validation before the renderer accepted it.")
}

fn themed_spinner_story_model() -> SpinnerModel {
    SpinnerModel::new("Theme scoped loading")
        .with_size(SpinnerSize::Large)
        .with_tone(SpinnerTone::Brand)
        .with_detail("Spinner border colors resolve through the nested Luxury theme.")
}

fn default_switch_story_model() -> SwitchModel {
    SwitchModel::new("Persist theme preference", "theme-preference")
        .with_detail("The renderer can toggle locally; consumers persist the accepted setting.")
        .with_on_label("Synced")
        .with_off_label("Local")
        .checked()
}

fn dense_switch_story_model() -> SwitchModel {
    default_switch_story_model()
        .with_density(SwitchDensity::Dense)
        .with_detail("Dense switches keep the same track, thumb, and label anatomy.")
}

fn off_switch_story_model() -> SwitchModel {
    SwitchModel::new("Use local motion setting", "motion-setting")
        .with_detail("Unchecked switches keep the same Bevy primitive contract.")
        .with_on_label("Motion")
        .with_off_label("Static")
        .unchecked()
}

fn loading_switch_story_model() -> SwitchModel {
    default_switch_story_model()
        .with_detail("Loading switches block interaction while the app reconciles durable state.")
        .loading()
}

fn disabled_switch_story_model() -> SwitchModel {
    default_switch_story_model()
        .with_detail("Disabled switches expose a stable read-only state to every renderer.")
        .disabled()
}

fn invalid_switch_story_model() -> SwitchModel {
    default_switch_story_model().with_error("Persisted setting failed validation at the edge.")
}

fn themed_switch_story_model() -> SwitchModel {
    SwitchModel::new("Theme scoped switch", "theme-scoped-switch")
        .with_detail("Switch track and status colors resolve through the nested Dracula theme.")
        .with_on_label("Dracula")
        .with_off_label("Default")
        .checked()
}

fn table_story_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Component", "component"),
        TableColumn::new("Surface", "surface"),
        TableColumn::new("Score", "score").numeric(),
    ]
}

fn table_story_rows() -> Vec<TableRow> {
    vec![
        TableRow::new(
            "accordion",
            vec![
                "Accordion".to_owned(),
                "Disclosure".to_owned(),
                "100".to_owned(),
            ],
        ),
        TableRow::new(
            "spinner",
            vec![
                "Spinner".to_owned(),
                "Feedback".to_owned(),
                "100".to_owned(),
            ],
        ),
        TableRow::new(
            "switch",
            vec!["Switch".to_owned(), "Control".to_owned(), "100".to_owned()],
        ),
        TableRow::new(
            "table",
            vec!["Table".to_owned(), "Data".to_owned(), "100".to_owned()],
        ),
    ]
}

fn default_table_story_model() -> TableModel {
    TableModel::new(table_story_columns(), table_story_rows())
        .with_caption("Semantic table rows render from shared Rust data.")
}

fn dense_table_story_model() -> TableModel {
    default_table_story_model()
        .with_density(TableDensity::Dense)
        .with_caption(
            "Dense tables preserve the same header, body, row, cell, and caption anatomy.",
        )
}

fn selected_table_story_model() -> TableModel {
    default_table_story_model()
        .with_caption("Renderer-local row selection stays separate from durable collection state.")
        .with_selected_row("switch")
}

fn loading_table_story_model() -> TableModel {
    default_table_story_model()
        .with_caption("Loading tables keep structure visible while blocking row interaction.")
        .loading()
}

fn disabled_table_story_model() -> TableModel {
    default_table_story_model()
        .with_caption("Disabled tables expose read-only row and cell primitives.")
        .disabled()
}

fn invalid_table_story_model() -> TableModel {
    default_table_story_model().with_error("Table rows failed validation before persistence.")
}

fn themed_table_story_model() -> TableModel {
    TableModel::new(table_story_columns(), table_story_rows())
        .with_caption("Table surfaces resolve through the nested Luxury theme.")
        .with_selected_row("spinner")
}

fn tabs_story_items() -> Vec<TabsItem> {
    vec![
        TabsItem::new(
            "Tokens",
            "tokens",
            "Trigger, panel, focus, and selected states all resolve through shared Tailwind tokens.",
        ),
        TabsItem::new(
            "Leptos",
            "leptos",
            "The DOM renderer owns selected and focused state locally unless the app persists a preference.",
        ),
        TabsItem::new(
            "Bevy",
            "bevy",
            "Scene primitives consume the same trigger and content render nodes without a Leptos dependency.",
        ),
    ]
}

fn default_tabs_story_model() -> TabsModel {
    TabsModel::new(tabs_story_items())
        .with_label("Framework surfaces")
        .with_selected_value("tokens")
}

fn dense_tabs_story_model() -> TabsModel {
    default_tabs_story_model()
        .with_density(TabsDensity::Dense)
        .with_selected_value("leptos")
}

fn vertical_tabs_story_model() -> TabsModel {
    default_tabs_story_model()
        .with_orientation(TabsOrientation::Vertical)
        .with_selected_value("bevy")
}

fn loading_tabs_story_model() -> TabsModel {
    default_tabs_story_model()
        .with_label("Hydrating tabs")
        .loading()
}

fn disabled_tabs_story_model() -> TabsModel {
    default_tabs_story_model()
        .with_label("Locked tabs")
        .disabled()
}

fn invalid_tabs_story_model() -> TabsModel {
    default_tabs_story_model().with_error("Selected tab failed validation before render.")
}

fn themed_tabs_story_model() -> TabsModel {
    TabsModel::new(tabs_story_items())
        .with_label("Theme scoped tabs")
        .with_density(TabsDensity::Dense)
        .with_selected_value("bevy")
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
