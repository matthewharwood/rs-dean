use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{
    StateContract, UiBlockRole, UiBlockTone, UiComponentCategory, UiComponentId,
    component_implementation, detail_for_part, role_for_part, tone_for_category, tone_for_role,
};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum UiWidgetPattern {
    Action,
    Callout,
    Data,
    Disclosure,
    Display,
    Feedback,
    Form,
    Layout,
    Messaging,
    Navigation,
    Overlay,
    Typography,
    Utility,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum UiWidgetIntent {
    None,
    Activate,
    Close,
    Dismiss,
    Input,
    Navigate,
    Open,
    Resize,
    Scroll,
    Select,
    Toggle,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum UiWidgetSlotKind {
    Avatar,
    Badge,
    Button,
    Cell,
    Chart,
    Checkbox,
    Description,
    Header,
    IconButton,
    Input,
    Key,
    Link,
    List,
    ListItem,
    Marker,
    Media,
    Option,
    Overlay,
    Panel,
    Progress,
    Radio,
    Row,
    Section,
    Select,
    Separator,
    Skeleton,
    Slider,
    Spinner,
    Switch,
    Table,
    Text,
    Textarea,
    Title,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Validate)]
pub struct UiWidgetSlot {
    #[garde(length(min = 1, max = 128))]
    pub part: &'static str,
    #[garde(skip)]
    pub kind: UiWidgetSlotKind,
    #[garde(skip)]
    pub role: UiBlockRole,
    #[garde(skip)]
    pub tone: UiBlockTone,
    #[garde(length(min = 1, max = 160))]
    pub label: &'static str,
    #[garde(length(min = 1, max = 2_000))]
    pub value: &'static str,
    #[garde(length(min = 1, max = 240))]
    pub detail: &'static str,
    #[garde(skip)]
    pub intent: UiWidgetIntent,
    #[garde(skip)]
    pub selected: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct UiWidget {
    #[garde(skip)]
    pub id: UiComponentId,
    #[garde(skip)]
    pub pattern: UiWidgetPattern,
    #[garde(skip)]
    pub state: StateContract,
    #[garde(length(min = 1), dive, custom(widget_slots_match_catalog(&self.id)))]
    pub slots: Vec<UiWidgetSlot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiWidgetRenderNode {
    pub part: &'static str,
    pub kind: UiWidgetSlotKind,
    pub role: UiBlockRole,
    pub tone: UiBlockTone,
    pub label: &'static str,
    pub value: &'static str,
    pub detail: &'static str,
    pub intent: UiWidgetIntent,
    pub selected: bool,
    pub disabled: bool,
}

pub type UiWidgetConstructor = fn() -> UiWidget;

pub const UI_WIDGET_CONSTRUCTOR_COUNT: usize = crate::SHADCN_COMPONENT_COUNT;

pub const UI_WIDGET_CONSTRUCTORS: [UiWidgetConstructor; UI_WIDGET_CONSTRUCTOR_COUNT] = [
    accordion_widget,
    alert_widget,
    alert_dialog_widget,
    aspect_ratio_widget,
    attachment_widget,
    avatar_widget,
    badge_widget,
    breadcrumb_widget,
    bubble_widget,
    button_widget,
    button_group_widget,
    calendar_widget,
    card_widget,
    carousel_widget,
    chart_widget,
    checkbox_widget,
    collapsible_widget,
    combobox_widget,
    command_widget,
    context_menu_widget,
    data_table_widget,
    date_picker_widget,
    dialog_widget,
    direction_widget,
    drawer_widget,
    dropdown_menu_widget,
    empty_widget,
    field_widget,
    hover_card_widget,
    input_widget,
    input_group_widget,
    input_otp_widget,
    item_widget,
    kbd_widget,
    label_widget,
    marker_widget,
    menubar_widget,
    message_widget,
    message_scroller_widget,
    native_select_widget,
    navigation_menu_widget,
    pagination_widget,
    popover_widget,
    progress_widget,
    radio_group_widget,
    resizable_widget,
    scroll_area_widget,
    select_widget,
    separator_widget,
    sheet_widget,
    sidebar_widget,
    skeleton_widget,
    slider_widget,
    sonner_widget,
    spinner_widget,
    switch_widget,
    table_widget,
    tabs_widget,
    textarea_widget,
    toast_widget,
    toggle_widget,
    toggle_group_widget,
    tooltip_widget,
    typography_widget,
];

pub fn widget_for_component(id: UiComponentId) -> UiWidget {
    UI_WIDGET_CONSTRUCTORS[id.index()]()
}

pub fn implemented_widgets() -> impl Iterator<Item = UiWidget> + use<> {
    UI_WIDGET_CONSTRUCTORS
        .into_iter()
        .map(|constructor| constructor())
}

pub fn validate_widget(widget: &UiWidget) -> Result<(), garde::Report> {
    widget.validate()
}

pub fn widget_render_nodes(widget: &UiWidget) -> Result<Vec<UiWidgetRenderNode>, garde::Report> {
    validate_widget(widget)?;
    Ok(widget
        .slots
        .iter()
        .map(|slot| UiWidgetRenderNode {
            part: slot.part,
            kind: slot.kind,
            role: slot.role,
            tone: slot.tone,
            label: slot.label,
            value: slot.value,
            detail: slot.detail,
            intent: slot.intent,
            selected: slot.selected,
            disabled: slot.disabled,
        })
        .collect())
}

fn widget_slots_match_catalog<'a>(
    id: &'a UiComponentId,
) -> impl FnOnce(&[UiWidgetSlot], &()) -> garde::Result + 'a {
    move |slots, _context| {
        let anatomy = id.anatomy_parts();
        for slot in slots {
            if !anatomy.contains(&slot.part) {
                return Err(garde::Error::new(format!(
                    "{id:?} widget has unknown anatomy part `{}`",
                    slot.part
                )));
            }
        }

        for part in anatomy {
            if !slots.iter().any(|slot| slot.part == *part) {
                return Err(garde::Error::new(format!(
                    "{id:?} widget is missing anatomy part `{part}`"
                )));
            }
        }

        Ok(())
    }
}

pub fn accordion_widget() -> UiWidget {
    let id = UiComponentId::Accordion;
    widget(
        id,
        UiWidgetPattern::Disclosure,
        vec![
            slot(
                id,
                "AccordionItem",
                UiWidgetSlotKind::Panel,
                "Billing",
                "Expanded section",
                UiWidgetIntent::None,
            )
            .selected(),
            slot(
                id,
                "AccordionTrigger",
                UiWidgetSlotKind::Button,
                "Payment settings",
                "Open section",
                UiWidgetIntent::Toggle,
            )
            .selected(),
            slot(
                id,
                "AccordionContent",
                UiWidgetSlotKind::Description,
                "Invoices are emailed every Friday.",
                "Visible content",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn alert_widget() -> UiWidget {
    let id = UiComponentId::Alert;
    widget(
        id,
        UiWidgetPattern::Callout,
        vec![
            slot(
                id,
                "AlertTitle",
                UiWidgetSlotKind::Title,
                "Build completed",
                "Success",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AlertDescription",
                UiWidgetSlotKind::Description,
                "The latest design-token bundle is ready for review.",
                "Status copy",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AlertAction",
                UiWidgetSlotKind::Button,
                "Open report",
                "Action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn alert_dialog_widget() -> UiWidget {
    let id = UiComponentId::AlertDialog;
    widget(
        id,
        UiWidgetPattern::Overlay,
        vec![
            slot(
                id,
                "AlertDialogTrigger",
                UiWidgetSlotKind::Button,
                "Delete draft",
                "Open confirmation",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "AlertDialogContent",
                UiWidgetSlotKind::Overlay,
                "Confirm deletion",
                "Blocking overlay",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AlertDialogHeader",
                UiWidgetSlotKind::Header,
                "This action cannot be undone.",
                "Warning",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AlertDialogFooter",
                UiWidgetSlotKind::Panel,
                "Choose how to continue.",
                "Action row",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AlertDialogAction",
                UiWidgetSlotKind::Button,
                "Delete",
                "Destructive action",
                UiWidgetIntent::Activate,
            ),
            slot(
                id,
                "AlertDialogCancel",
                UiWidgetSlotKind::Button,
                "Cancel",
                "Dismiss",
                UiWidgetIntent::Close,
            ),
        ],
    )
}

pub fn aspect_ratio_widget() -> UiWidget {
    let id = UiComponentId::AspectRatio;
    widget(
        id,
        UiWidgetPattern::Layout,
        vec![
            slot(
                id,
                "AspectRatioFrame",
                UiWidgetSlotKind::Media,
                "16:9 canvas frame",
                "Stable media frame",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AspectRatioMedia",
                UiWidgetSlotKind::Media,
                "Preview image",
                "Hydration-safe media",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn attachment_widget() -> UiWidget {
    let id = UiComponentId::Attachment;
    widget(
        id,
        UiWidgetPattern::Messaging,
        vec![
            slot(
                id,
                "AttachmentPreview",
                UiWidgetSlotKind::Media,
                "PDF",
                "File preview",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AttachmentTitle",
                UiWidgetSlotKind::Title,
                "roadmap-notes.pdf",
                "Filename",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AttachmentMeta",
                UiWidgetSlotKind::Text,
                "2.4 MB",
                "File metadata",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AttachmentAction",
                UiWidgetSlotKind::Button,
                "Download",
                "Attachment action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn avatar_widget() -> UiWidget {
    let id = UiComponentId::Avatar;
    widget(
        id,
        UiWidgetPattern::Display,
        vec![
            slot(
                id,
                "AvatarImage",
                UiWidgetSlotKind::Avatar,
                "MH",
                "Loaded user image",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "AvatarFallback",
                UiWidgetSlotKind::Text,
                "Matthew Harwood",
                "Fallback name",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn badge_widget() -> UiWidget {
    let id = UiComponentId::Badge;
    widget(
        id,
        UiWidgetPattern::Display,
        vec![
            slot(
                id,
                "BadgeIcon",
                UiWidgetSlotKind::Marker,
                "Live",
                "Status icon",
                UiWidgetIntent::None,
            )
            .selected(),
            slot(
                id,
                "BadgeText",
                UiWidgetSlotKind::Badge,
                "Ready",
                "Badge label",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn breadcrumb_widget() -> UiWidget {
    let id = UiComponentId::Breadcrumb;
    widget(
        id,
        UiWidgetPattern::Navigation,
        vec![
            slot(
                id,
                "BreadcrumbList",
                UiWidgetSlotKind::List,
                "Location trail",
                "Ancestor list",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "BreadcrumbItem",
                UiWidgetSlotKind::ListItem,
                "Library",
                "Trail item",
                UiWidgetIntent::Navigate,
            ),
            slot(
                id,
                "BreadcrumbLink",
                UiWidgetSlotKind::Link,
                "Components",
                "Ancestor link",
                UiWidgetIntent::Navigate,
            ),
            slot(
                id,
                "BreadcrumbSeparator",
                UiWidgetSlotKind::Separator,
                "/",
                "Separator",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "BreadcrumbPage",
                UiWidgetSlotKind::Text,
                "Accordion",
                "Current page",
                UiWidgetIntent::None,
            )
            .selected(),
        ],
    )
}

pub fn bubble_widget() -> UiWidget {
    let id = UiComponentId::Bubble;
    widget(
        id,
        UiWidgetPattern::Messaging,
        vec![
            slot(
                id,
                "BubbleAvatar",
                UiWidgetSlotKind::Avatar,
                "AI",
                "Sender",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "BubbleContent",
                UiWidgetSlotKind::Description,
                "The sweep is ready for review.",
                "Message body",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "BubbleMeta",
                UiWidgetSlotKind::Text,
                "Delivered now",
                "Delivery metadata",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "BubbleActions",
                UiWidgetSlotKind::Button,
                "Reply",
                "Message action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn button_widget() -> UiWidget {
    let id = UiComponentId::Button;
    widget(
        id,
        UiWidgetPattern::Action,
        vec![
            slot(
                id,
                "ButtonIcon",
                UiWidgetSlotKind::Marker,
                "->",
                "Leading icon",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ButtonLabel",
                UiWidgetSlotKind::Button,
                "Save changes",
                "Primary command",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn button_group_widget() -> UiWidget {
    let id = UiComponentId::ButtonGroup;
    widget(
        id,
        UiWidgetPattern::Action,
        vec![
            slot(
                id,
                "ButtonGroupItem",
                UiWidgetSlotKind::Button,
                "Day",
                "First grouped action",
                UiWidgetIntent::Select,
            )
            .selected(),
            slot(
                id,
                "ButtonGroupSeparator",
                UiWidgetSlotKind::Separator,
                "divider",
                "Group divider",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ButtonGroupItem",
                UiWidgetSlotKind::Button,
                "Week",
                "Second grouped action",
                UiWidgetIntent::Select,
            ),
        ],
    )
}

pub fn calendar_widget() -> UiWidget {
    let id = UiComponentId::Calendar;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "CalendarHeader",
                UiWidgetSlotKind::Header,
                "July 2026",
                "Visible month",
                UiWidgetIntent::Navigate,
            ),
            slot(
                id,
                "CalendarGrid",
                UiWidgetSlotKind::List,
                "Date grid",
                "Calendar body",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "CalendarDay",
                UiWidgetSlotKind::Button,
                "6",
                "Selected date",
                UiWidgetIntent::Select,
            )
            .selected(),
            slot(
                id,
                "CalendarRange",
                UiWidgetSlotKind::Marker,
                "Jul 6-10",
                "Selected range",
                UiWidgetIntent::Select,
            ),
        ],
    )
}

pub fn card_widget() -> UiWidget {
    let id = UiComponentId::Card;
    widget(
        id,
        UiWidgetPattern::Display,
        vec![
            slot(
                id,
                "CardHeader",
                UiWidgetSlotKind::Header,
                "Weekly review",
                "Card header",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "CardTitle",
                UiWidgetSlotKind::Title,
                "Design system",
                "Card title",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "CardDescription",
                UiWidgetSlotKind::Description,
                "64 components share one token contract.",
                "Card description",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "CardContent",
                UiWidgetSlotKind::Panel,
                "Implementation notes",
                "Card content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "CardFooter",
                UiWidgetSlotKind::Button,
                "Open checklist",
                "Card footer action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn carousel_widget() -> UiWidget {
    let id = UiComponentId::Carousel;
    widget(
        id,
        UiWidgetPattern::Display,
        vec![
            slot(
                id,
                "CarouselContent",
                UiWidgetSlotKind::List,
                "Slides",
                "Paged content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "CarouselItem",
                UiWidgetSlotKind::Media,
                "Theme preview",
                "Current slide",
                UiWidgetIntent::None,
            )
            .selected(),
            slot(
                id,
                "CarouselPrevious",
                UiWidgetSlotKind::IconButton,
                "Previous",
                "Previous slide",
                UiWidgetIntent::Navigate,
            ),
            slot(
                id,
                "CarouselNext",
                UiWidgetSlotKind::IconButton,
                "Next",
                "Next slide",
                UiWidgetIntent::Navigate,
            ),
            slot(
                id,
                "CarouselIndicator",
                UiWidgetSlotKind::Marker,
                "1 of 3",
                "Position indicator",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn chart_widget() -> UiWidget {
    let id = UiComponentId::Chart;
    widget(
        id,
        UiWidgetPattern::Data,
        vec![
            slot(
                id,
                "ChartSeries",
                UiWidgetSlotKind::Chart,
                "Completion",
                "Series bars",
                UiWidgetIntent::Select,
            ),
            slot(
                id,
                "ChartLegend",
                UiWidgetSlotKind::Badge,
                "Implemented",
                "Legend",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ChartTooltip",
                UiWidgetSlotKind::Text,
                "64 components",
                "Tooltip copy",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ChartAxis",
                UiWidgetSlotKind::Separator,
                "Components",
                "Axis label",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn checkbox_widget() -> UiWidget {
    let id = UiComponentId::Checkbox;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "CheckboxIndicator",
                UiWidgetSlotKind::Checkbox,
                "Use shared tokens",
                "Checked control",
                UiWidgetIntent::Toggle,
            )
            .selected(),
            slot(
                id,
                "CheckboxLabel",
                UiWidgetSlotKind::Title,
                "Token-only UI",
                "Control label",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "CheckboxDescription",
                UiWidgetSlotKind::Description,
                "Class names come from rs-dean-ui.",
                "Helper text",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn collapsible_widget() -> UiWidget {
    let id = UiComponentId::Collapsible;
    widget(
        id,
        UiWidgetPattern::Disclosure,
        vec![
            slot(
                id,
                "CollapsibleTrigger",
                UiWidgetSlotKind::Button,
                "Show details",
                "Disclosure trigger",
                UiWidgetIntent::Toggle,
            )
            .selected(),
            slot(
                id,
                "CollapsibleContent",
                UiWidgetSlotKind::Description,
                "Renderer-local state keeps this content open.",
                "Disclosure content",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn combobox_widget() -> UiWidget {
    let id = UiComponentId::Combobox;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "ComboboxInput",
                UiWidgetSlotKind::Input,
                "Search framework",
                "Filter input",
                UiWidgetIntent::Input,
            ),
            slot(
                id,
                "ComboboxList",
                UiWidgetSlotKind::List,
                "Options",
                "Filtered list",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ComboboxOption",
                UiWidgetSlotKind::Option,
                "Leptos",
                "Selected option",
                UiWidgetIntent::Select,
            )
            .selected(),
            slot(
                id,
                "ComboboxEmpty",
                UiWidgetSlotKind::Text,
                "No matches",
                "Empty result",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn command_widget() -> UiWidget {
    let id = UiComponentId::Command;
    widget(
        id,
        UiWidgetPattern::Navigation,
        vec![
            slot(
                id,
                "CommandInput",
                UiWidgetSlotKind::Input,
                "Type a command",
                "Command search",
                UiWidgetIntent::Input,
            ),
            slot(
                id,
                "CommandList",
                UiWidgetSlotKind::List,
                "Commands",
                "Result list",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "CommandGroup",
                UiWidgetSlotKind::Header,
                "Workspace",
                "Command group",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "CommandItem",
                UiWidgetSlotKind::Option,
                "Open stories",
                "Command item",
                UiWidgetIntent::Activate,
            )
            .selected(),
            slot(
                id,
                "CommandShortcut",
                UiWidgetSlotKind::Key,
                "Cmd+K",
                "Shortcut",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn context_menu_widget() -> UiWidget {
    let id = UiComponentId::ContextMenu;
    widget(
        id,
        UiWidgetPattern::Overlay,
        vec![
            slot(
                id,
                "ContextMenuTrigger",
                UiWidgetSlotKind::Button,
                "Right click target",
                "Menu trigger",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "ContextMenuContent",
                UiWidgetSlotKind::Overlay,
                "Object actions",
                "Menu content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ContextMenuItem",
                UiWidgetSlotKind::Option,
                "Duplicate",
                "Menu item",
                UiWidgetIntent::Activate,
            ),
            slot(
                id,
                "ContextMenuSeparator",
                UiWidgetSlotKind::Separator,
                "separator",
                "Menu separator",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ContextMenuSubmenu",
                UiWidgetSlotKind::Panel,
                "Move to",
                "Nested submenu",
                UiWidgetIntent::Open,
            ),
        ],
    )
}

pub fn data_table_widget() -> UiWidget {
    let id = UiComponentId::DataTable;
    widget(
        id,
        UiWidgetPattern::Data,
        vec![
            slot(
                id,
                "DataTableToolbar",
                UiWidgetSlotKind::Input,
                "Filter tasks",
                "Toolbar",
                UiWidgetIntent::Input,
            ),
            slot(
                id,
                "DataTableHeader",
                UiWidgetSlotKind::Header,
                "Component",
                "Sortable header",
                UiWidgetIntent::Select,
            ),
            slot(
                id,
                "DataTableRow",
                UiWidgetSlotKind::Row,
                "Button",
                "Table row",
                UiWidgetIntent::Select,
            ),
            slot(
                id,
                "DataTableCell",
                UiWidgetSlotKind::Cell,
                "Implemented",
                "Table cell",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DataTablePagination",
                UiWidgetSlotKind::Button,
                "Page 1",
                "Pagination",
                UiWidgetIntent::Navigate,
            ),
        ],
    )
}

pub fn date_picker_widget() -> UiWidget {
    let id = UiComponentId::DatePicker;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "DatePickerTrigger",
                UiWidgetSlotKind::Button,
                "Jul 6, 2026",
                "Open date picker",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "DatePickerPopover",
                UiWidgetSlotKind::Overlay,
                "Pick a date",
                "Calendar popover",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DatePickerCalendar",
                UiWidgetSlotKind::List,
                "Month grid",
                "Calendar grid",
                UiWidgetIntent::Select,
            ),
            slot(
                id,
                "DatePickerValue",
                UiWidgetSlotKind::Text,
                "2026-07-06",
                "Selected value",
                UiWidgetIntent::None,
            )
            .selected(),
        ],
    )
}

pub fn dialog_widget() -> UiWidget {
    let id = UiComponentId::Dialog;
    widget(
        id,
        UiWidgetPattern::Overlay,
        vec![
            slot(
                id,
                "DialogTrigger",
                UiWidgetSlotKind::Button,
                "Edit profile",
                "Open dialog",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "DialogContent",
                UiWidgetSlotKind::Overlay,
                "Profile settings",
                "Dialog content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DialogHeader",
                UiWidgetSlotKind::Header,
                "Account",
                "Dialog header",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DialogTitle",
                UiWidgetSlotKind::Title,
                "Update profile",
                "Dialog title",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DialogDescription",
                UiWidgetSlotKind::Description,
                "Changes are saved by the consumer.",
                "Dialog description",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DialogFooter",
                UiWidgetSlotKind::Button,
                "Save",
                "Dialog action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn direction_widget() -> UiWidget {
    let id = UiComponentId::Direction;
    widget(
        id,
        UiWidgetPattern::Utility,
        vec![
            slot(
                id,
                "DirectionScope",
                UiWidgetSlotKind::Panel,
                "dir=rtl",
                "Scoped direction",
                UiWidgetIntent::Select,
            ),
            slot(
                id,
                "DirectionAwareContent",
                UiWidgetSlotKind::Text,
                "Mirrored content",
                "Direction-aware child",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn drawer_widget() -> UiWidget {
    let id = UiComponentId::Drawer;
    widget(
        id,
        UiWidgetPattern::Overlay,
        vec![
            slot(
                id,
                "DrawerTrigger",
                UiWidgetSlotKind::Button,
                "Open drawer",
                "Drawer trigger",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "DrawerContent",
                UiWidgetSlotKind::Overlay,
                "Mobile task panel",
                "Drawer content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DrawerHeader",
                UiWidgetSlotKind::Header,
                "Quick actions",
                "Drawer header",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DrawerFooter",
                UiWidgetSlotKind::Button,
                "Done",
                "Drawer footer",
                UiWidgetIntent::Close,
            ),
            slot(
                id,
                "DrawerHandle",
                UiWidgetSlotKind::Marker,
                "drag",
                "Drawer handle",
                UiWidgetIntent::Resize,
            ),
        ],
    )
}

pub fn dropdown_menu_widget() -> UiWidget {
    let id = UiComponentId::DropdownMenu;
    widget(
        id,
        UiWidgetPattern::Overlay,
        vec![
            slot(
                id,
                "DropdownMenuTrigger",
                UiWidgetSlotKind::Button,
                "Actions",
                "Menu trigger",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "DropdownMenuContent",
                UiWidgetSlotKind::Overlay,
                "Menu",
                "Menu content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DropdownMenuItem",
                UiWidgetSlotKind::Option,
                "Rename",
                "Menu item",
                UiWidgetIntent::Activate,
            ),
            slot(
                id,
                "DropdownMenuLabel",
                UiWidgetSlotKind::Header,
                "File",
                "Menu label",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "DropdownMenuSeparator",
                UiWidgetSlotKind::Separator,
                "separator",
                "Menu separator",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn empty_widget() -> UiWidget {
    let id = UiComponentId::Empty;
    widget(
        id,
        UiWidgetPattern::Feedback,
        vec![
            slot(
                id,
                "EmptyHeader",
                UiWidgetSlotKind::Header,
                "No components queued",
                "Empty header",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "EmptyTitle",
                UiWidgetSlotKind::Title,
                "All clear",
                "Empty title",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "EmptyDescription",
                UiWidgetSlotKind::Description,
                "There is nothing else to process.",
                "Empty description",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "EmptyContent",
                UiWidgetSlotKind::Panel,
                "Try changing filters.",
                "Recovery content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "EmptyAction",
                UiWidgetSlotKind::Button,
                "Reset filters",
                "Recovery action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn field_widget() -> UiWidget {
    let id = UiComponentId::Field;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "FieldLabel",
                UiWidgetSlotKind::Title,
                "Repository name",
                "Field label",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "FieldControl",
                UiWidgetSlotKind::Input,
                "rs-dean",
                "Field control",
                UiWidgetIntent::Input,
            ),
            slot(
                id,
                "FieldDescription",
                UiWidgetSlotKind::Description,
                "Use a unique project slug.",
                "Field hint",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "FieldError",
                UiWidgetSlotKind::Text,
                "Name is available.",
                "Validation message",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn hover_card_widget() -> UiWidget {
    let id = UiComponentId::HoverCard;
    widget(
        id,
        UiWidgetPattern::Overlay,
        vec![
            slot(
                id,
                "HoverCardTrigger",
                UiWidgetSlotKind::Link,
                "Design token",
                "Hover trigger",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "HoverCardContent",
                UiWidgetSlotKind::Overlay,
                "Shared across Leptos and Bevy.",
                "Preview content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "HoverCardArrow",
                UiWidgetSlotKind::Marker,
                "arrow",
                "Overlay arrow",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn input_widget() -> UiWidget {
    let id = UiComponentId::Input;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "InputPrefix",
                UiWidgetSlotKind::Text,
                "https://",
                "Prefix",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "InputControl",
                UiWidgetSlotKind::Input,
                "engmanager.xyz",
                "Text input",
                UiWidgetIntent::Input,
            ),
            slot(
                id,
                "InputSuffix",
                UiWidgetSlotKind::Button,
                "Copy",
                "Suffix action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn input_group_widget() -> UiWidget {
    let id = UiComponentId::InputGroup;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "InputGroupAddon",
                UiWidgetSlotKind::Text,
                "$",
                "Leading addon",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "InputGroupInput",
                UiWidgetSlotKind::Input,
                "42",
                "Grouped input",
                UiWidgetIntent::Input,
            ),
            slot(
                id,
                "InputGroupButton",
                UiWidgetSlotKind::Button,
                "Apply",
                "Inline action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn input_otp_widget() -> UiWidget {
    let id = UiComponentId::InputOtp;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "InputOtpGroup",
                UiWidgetSlotKind::List,
                "Code slots",
                "OTP group",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "InputOtpSlot",
                UiWidgetSlotKind::Input,
                "7",
                "OTP slot",
                UiWidgetIntent::Input,
            )
            .selected(),
            slot(
                id,
                "InputOtpSeparator",
                UiWidgetSlotKind::Separator,
                "-",
                "OTP separator",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn item_widget() -> UiWidget {
    let id = UiComponentId::Item;
    widget(
        id,
        UiWidgetPattern::Display,
        vec![
            slot(
                id,
                "ItemMedia",
                UiWidgetSlotKind::Avatar,
                "UI",
                "Item media",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ItemContent",
                UiWidgetSlotKind::Panel,
                "Component task",
                "Item content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ItemTitle",
                UiWidgetSlotKind::Title,
                "Build widgets",
                "Item title",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ItemDescription",
                UiWidgetSlotKind::Description,
                "Shared contracts are in place.",
                "Item description",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ItemActions",
                UiWidgetSlotKind::Button,
                "Open",
                "Item action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn kbd_widget() -> UiWidget {
    let id = UiComponentId::Kbd;
    widget(
        id,
        UiWidgetPattern::Typography,
        vec![
            slot(
                id,
                "KbdKey",
                UiWidgetSlotKind::Key,
                "K",
                "Keyboard key",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "KbdChord",
                UiWidgetSlotKind::Text,
                "Command + K",
                "Shortcut chord",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn label_widget() -> UiWidget {
    let id = UiComponentId::Label;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "LabelText",
                UiWidgetSlotKind::Title,
                "Email",
                "Label text",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "LabelRequirement",
                UiWidgetSlotKind::Badge,
                "Required",
                "Requirement marker",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn marker_widget() -> UiWidget {
    let id = UiComponentId::Marker;
    widget(
        id,
        UiWidgetPattern::Messaging,
        vec![
            slot(
                id,
                "MarkerDot",
                UiWidgetSlotKind::Marker,
                "Unread",
                "Marker dot",
                UiWidgetIntent::None,
            )
            .selected(),
            slot(
                id,
                "MarkerLabel",
                UiWidgetSlotKind::Text,
                "3 new",
                "Marker label",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "MarkerAnchor",
                UiWidgetSlotKind::Link,
                "Jump",
                "Marker anchor",
                UiWidgetIntent::Navigate,
            ),
        ],
    )
}

pub fn menubar_widget() -> UiWidget {
    let id = UiComponentId::Menubar;
    widget(
        id,
        UiWidgetPattern::Navigation,
        vec![
            slot(
                id,
                "MenubarMenu",
                UiWidgetSlotKind::List,
                "Application menu",
                "Menu group",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "MenubarTrigger",
                UiWidgetSlotKind::Button,
                "File",
                "Menu trigger",
                UiWidgetIntent::Open,
            )
            .selected(),
            slot(
                id,
                "MenubarContent",
                UiWidgetSlotKind::Overlay,
                "File commands",
                "Menu content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "MenubarItem",
                UiWidgetSlotKind::Option,
                "New project",
                "Menu item",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn message_widget() -> UiWidget {
    let id = UiComponentId::Message;
    widget(
        id,
        UiWidgetPattern::Messaging,
        vec![
            slot(
                id,
                "MessageHeader",
                UiWidgetSlotKind::Header,
                "Codex",
                "Message header",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "MessageContent",
                UiWidgetSlotKind::Description,
                "All component contracts are now real widgets.",
                "Message body",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "MessageFooter",
                UiWidgetSlotKind::Text,
                "Sent now",
                "Message footer",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "MessageActions",
                UiWidgetSlotKind::Button,
                "Resolve",
                "Message action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn message_scroller_widget() -> UiWidget {
    let id = UiComponentId::MessageScroller;
    widget(
        id,
        UiWidgetPattern::Messaging,
        vec![
            slot(
                id,
                "MessageViewport",
                UiWidgetSlotKind::Panel,
                "Thread viewport",
                "Scroll viewport",
                UiWidgetIntent::Scroll,
            ),
            slot(
                id,
                "MessageList",
                UiWidgetSlotKind::List,
                "Message list",
                "Scrollable messages",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "MessageAnchor",
                UiWidgetSlotKind::Marker,
                "Latest",
                "Scroll anchor",
                UiWidgetIntent::Scroll,
            ),
            slot(
                id,
                "MessageJumpButton",
                UiWidgetSlotKind::Button,
                "Jump to latest",
                "Jump action",
                UiWidgetIntent::Scroll,
            ),
        ],
    )
}

pub fn native_select_widget() -> UiWidget {
    let id = UiComponentId::NativeSelect;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "NativeSelectTrigger",
                UiWidgetSlotKind::Select,
                "Theme",
                "Native select",
                UiWidgetIntent::Select,
            ),
            slot(
                id,
                "NativeSelectOption",
                UiWidgetSlotKind::Option,
                "Catppuccin",
                "Native option",
                UiWidgetIntent::Select,
            )
            .selected(),
            slot(
                id,
                "NativeSelectValue",
                UiWidgetSlotKind::Text,
                "catppuccin",
                "Selected value",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn navigation_menu_widget() -> UiWidget {
    let id = UiComponentId::NavigationMenu;
    widget(
        id,
        UiWidgetPattern::Navigation,
        vec![
            slot(
                id,
                "NavigationMenuList",
                UiWidgetSlotKind::List,
                "Primary navigation",
                "Navigation list",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "NavigationMenuItem",
                UiWidgetSlotKind::ListItem,
                "Docs",
                "Navigation item",
                UiWidgetIntent::Navigate,
            )
            .selected(),
            slot(
                id,
                "NavigationMenuTrigger",
                UiWidgetSlotKind::Button,
                "Components",
                "Panel trigger",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "NavigationMenuContent",
                UiWidgetSlotKind::Panel,
                "Component links",
                "Navigation panel",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "NavigationMenuLink",
                UiWidgetSlotKind::Link,
                "Button",
                "Navigation link",
                UiWidgetIntent::Navigate,
            ),
        ],
    )
}

pub fn pagination_widget() -> UiWidget {
    let id = UiComponentId::Pagination;
    widget(
        id,
        UiWidgetPattern::Navigation,
        vec![
            slot(
                id,
                "PaginationContent",
                UiWidgetSlotKind::List,
                "Pages",
                "Page list",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "PaginationItem",
                UiWidgetSlotKind::ListItem,
                "1",
                "Page item",
                UiWidgetIntent::Navigate,
            )
            .selected(),
            slot(
                id,
                "PaginationPrevious",
                UiWidgetSlotKind::Button,
                "Previous",
                "Previous page",
                UiWidgetIntent::Navigate,
            ),
            slot(
                id,
                "PaginationLink",
                UiWidgetSlotKind::Link,
                "2",
                "Page link",
                UiWidgetIntent::Navigate,
            ),
            slot(
                id,
                "PaginationNext",
                UiWidgetSlotKind::Button,
                "Next",
                "Next page",
                UiWidgetIntent::Navigate,
            ),
        ],
    )
}

pub fn popover_widget() -> UiWidget {
    let id = UiComponentId::Popover;
    widget(
        id,
        UiWidgetPattern::Overlay,
        vec![
            slot(
                id,
                "PopoverTrigger",
                UiWidgetSlotKind::Button,
                "Open popover",
                "Popover trigger",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "PopoverContent",
                UiWidgetSlotKind::Overlay,
                "Secondary settings",
                "Popover content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "PopoverArrow",
                UiWidgetSlotKind::Marker,
                "arrow",
                "Popover arrow",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn progress_widget() -> UiWidget {
    let id = UiComponentId::Progress;
    widget(
        id,
        UiWidgetPattern::Feedback,
        vec![
            slot(
                id,
                "ProgressTrack",
                UiWidgetSlotKind::Progress,
                "Upload",
                "Progress track",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ProgressIndicator",
                UiWidgetSlotKind::Marker,
                "64%",
                "Progress indicator",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ProgressLabel",
                UiWidgetSlotKind::Text,
                "64 percent complete",
                "Accessible label",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn radio_group_widget() -> UiWidget {
    let id = UiComponentId::RadioGroup;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "RadioGroupItem",
                UiWidgetSlotKind::Radio,
                "Light",
                "Radio item",
                UiWidgetIntent::Select,
            )
            .selected(),
            slot(
                id,
                "RadioGroupIndicator",
                UiWidgetSlotKind::Marker,
                "selected",
                "Selection indicator",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "RadioGroupLabel",
                UiWidgetSlotKind::Title,
                "Theme",
                "Radio label",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn resizable_widget() -> UiWidget {
    let id = UiComponentId::Resizable;
    widget(
        id,
        UiWidgetPattern::Layout,
        vec![
            slot(
                id,
                "ResizablePanel",
                UiWidgetSlotKind::Panel,
                "Primary panel",
                "Resizable panel",
                UiWidgetIntent::Resize,
            ),
            slot(
                id,
                "ResizableHandle",
                UiWidgetSlotKind::Separator,
                "drag",
                "Resize handle",
                UiWidgetIntent::Resize,
            ),
        ],
    )
}

pub fn scroll_area_widget() -> UiWidget {
    let id = UiComponentId::ScrollArea;
    widget(
        id,
        UiWidgetPattern::Layout,
        vec![
            slot(
                id,
                "ScrollViewport",
                UiWidgetSlotKind::Panel,
                "Scrollable viewport",
                "Scroll viewport",
                UiWidgetIntent::Scroll,
            ),
            slot(
                id,
                "ScrollContent",
                UiWidgetSlotKind::Description,
                "Long token documentation content.",
                "Scroll content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ScrollBar",
                UiWidgetSlotKind::Separator,
                "bar",
                "Scrollbar",
                UiWidgetIntent::Scroll,
            ),
            slot(
                id,
                "ScrollCorner",
                UiWidgetSlotKind::Marker,
                "corner",
                "Scroll corner",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn select_widget() -> UiWidget {
    let id = UiComponentId::Select;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "SelectTrigger",
                UiWidgetSlotKind::Button,
                "Select theme",
                "Custom select trigger",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "SelectValue",
                UiWidgetSlotKind::Text,
                "Dark",
                "Selected value",
                UiWidgetIntent::None,
            )
            .selected(),
            slot(
                id,
                "SelectContent",
                UiWidgetSlotKind::Overlay,
                "Theme options",
                "Select content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SelectItem",
                UiWidgetSlotKind::Option,
                "Dark",
                "Select item",
                UiWidgetIntent::Select,
            )
            .selected(),
            slot(
                id,
                "SelectGroup",
                UiWidgetSlotKind::Header,
                "Appearance",
                "Select group",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn separator_widget() -> UiWidget {
    let id = UiComponentId::Separator;
    widget(
        id,
        UiWidgetPattern::Layout,
        vec![
            slot(
                id,
                "SeparatorLine",
                UiWidgetSlotKind::Separator,
                "line",
                "Divider line",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SeparatorLabel",
                UiWidgetSlotKind::Text,
                "Section",
                "Optional label",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn sheet_widget() -> UiWidget {
    let id = UiComponentId::Sheet;
    widget(
        id,
        UiWidgetPattern::Overlay,
        vec![
            slot(
                id,
                "SheetTrigger",
                UiWidgetSlotKind::Button,
                "Open sheet",
                "Sheet trigger",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "SheetContent",
                UiWidgetSlotKind::Overlay,
                "Inspector",
                "Sheet content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SheetHeader",
                UiWidgetSlotKind::Header,
                "Properties",
                "Sheet header",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SheetFooter",
                UiWidgetSlotKind::Button,
                "Apply",
                "Sheet footer",
                UiWidgetIntent::Activate,
            ),
            slot(
                id,
                "SheetClose",
                UiWidgetSlotKind::IconButton,
                "Close",
                "Close action",
                UiWidgetIntent::Close,
            ),
        ],
    )
}

pub fn sidebar_widget() -> UiWidget {
    let id = UiComponentId::Sidebar;
    widget(
        id,
        UiWidgetPattern::Navigation,
        vec![
            slot(
                id,
                "SidebarHeader",
                UiWidgetSlotKind::Header,
                "rs-dean",
                "Sidebar header",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SidebarContent",
                UiWidgetSlotKind::Panel,
                "Navigation content",
                "Sidebar content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SidebarGroup",
                UiWidgetSlotKind::List,
                "Workspace",
                "Sidebar group",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SidebarMenu",
                UiWidgetSlotKind::Option,
                "Stories",
                "Menu item",
                UiWidgetIntent::Navigate,
            )
            .selected(),
            slot(
                id,
                "SidebarFooter",
                UiWidgetSlotKind::Button,
                "Settings",
                "Sidebar footer",
                UiWidgetIntent::Navigate,
            ),
            slot(
                id,
                "SidebarRail",
                UiWidgetSlotKind::Separator,
                "rail",
                "Collapse rail",
                UiWidgetIntent::Toggle,
            ),
        ],
    )
}

pub fn skeleton_widget() -> UiWidget {
    let id = UiComponentId::Skeleton;
    widget(
        id,
        UiWidgetPattern::Feedback,
        vec![
            slot(
                id,
                "SkeletonBlock",
                UiWidgetSlotKind::Skeleton,
                "Block placeholder",
                "Loading block",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SkeletonText",
                UiWidgetSlotKind::Skeleton,
                "Text placeholder",
                "Loading text",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SkeletonMedia",
                UiWidgetSlotKind::Skeleton,
                "Media placeholder",
                "Loading media",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn slider_widget() -> UiWidget {
    let id = UiComponentId::Slider;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "SliderTrack",
                UiWidgetSlotKind::Slider,
                "Volume",
                "Slider track",
                UiWidgetIntent::Input,
            ),
            slot(
                id,
                "SliderRange",
                UiWidgetSlotKind::Progress,
                "72",
                "Selected range",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SliderThumb",
                UiWidgetSlotKind::Marker,
                "thumb",
                "Slider thumb",
                UiWidgetIntent::Input,
            ),
            slot(
                id,
                "SliderValue",
                UiWidgetSlotKind::Text,
                "72",
                "Current value",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn sonner_widget() -> UiWidget {
    let id = UiComponentId::Sonner;
    widget(
        id,
        UiWidgetPattern::Feedback,
        vec![
            slot(
                id,
                "SonnerViewport",
                UiWidgetSlotKind::Panel,
                "Notification viewport",
                "Toast viewport",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SonnerToast",
                UiWidgetSlotKind::Overlay,
                "Saved",
                "Toast item",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SonnerAction",
                UiWidgetSlotKind::Button,
                "Undo",
                "Toast action",
                UiWidgetIntent::Activate,
            ),
            slot(
                id,
                "SonnerDismiss",
                UiWidgetSlotKind::IconButton,
                "Dismiss",
                "Dismiss toast",
                UiWidgetIntent::Dismiss,
            ),
        ],
    )
}

pub fn spinner_widget() -> UiWidget {
    let id = UiComponentId::Spinner;
    widget(
        id,
        UiWidgetPattern::Feedback,
        vec![
            slot(
                id,
                "SpinnerTrack",
                UiWidgetSlotKind::Spinner,
                "Loading",
                "Spinner track",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SpinnerIndicator",
                UiWidgetSlotKind::Marker,
                "busy",
                "Spinner indicator",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "SpinnerLabel",
                UiWidgetSlotKind::Text,
                "Loading components",
                "Spinner label",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn switch_widget() -> UiWidget {
    let id = UiComponentId::Switch;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "SwitchTrack",
                UiWidgetSlotKind::Switch,
                "Enable sync",
                "Switch track",
                UiWidgetIntent::Toggle,
            )
            .selected(),
            slot(
                id,
                "SwitchThumb",
                UiWidgetSlotKind::Marker,
                "on",
                "Switch thumb",
                UiWidgetIntent::None,
            )
            .selected(),
            slot(
                id,
                "SwitchLabel",
                UiWidgetSlotKind::Title,
                "Sync state",
                "Switch label",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn table_widget() -> UiWidget {
    let id = UiComponentId::Table;
    widget(
        id,
        UiWidgetPattern::Data,
        vec![
            slot(
                id,
                "TableHeader",
                UiWidgetSlotKind::Header,
                "Component",
                "Table header",
                UiWidgetIntent::Select,
            ),
            slot(
                id,
                "TableBody",
                UiWidgetSlotKind::Table,
                "Rows",
                "Table body",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TableRow",
                UiWidgetSlotKind::Row,
                "Accordion",
                "Table row",
                UiWidgetIntent::Select,
            ),
            slot(
                id,
                "TableHead",
                UiWidgetSlotKind::Cell,
                "Status",
                "Column header",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TableCell",
                UiWidgetSlotKind::Cell,
                "Implemented",
                "Table cell",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TableCaption",
                UiWidgetSlotKind::Description,
                "Component implementation status.",
                "Table caption",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn tabs_widget() -> UiWidget {
    let id = UiComponentId::Tabs;
    widget(
        id,
        UiWidgetPattern::Navigation,
        vec![
            slot(
                id,
                "TabsList",
                UiWidgetSlotKind::List,
                "Views",
                "Tab list",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TabsTrigger",
                UiWidgetSlotKind::Button,
                "Preview",
                "Active tab",
                UiWidgetIntent::Select,
            )
            .selected(),
            slot(
                id,
                "TabsContent",
                UiWidgetSlotKind::Panel,
                "Preview content",
                "Active panel",
                UiWidgetIntent::None,
            )
            .selected(),
        ],
    )
}

pub fn textarea_widget() -> UiWidget {
    let id = UiComponentId::Textarea;
    widget(
        id,
        UiWidgetPattern::Form,
        vec![
            slot(
                id,
                "TextareaControl",
                UiWidgetSlotKind::Textarea,
                "Implementation notes",
                "Text area",
                UiWidgetIntent::Input,
            ),
            slot(
                id,
                "TextareaCounter",
                UiWidgetSlotKind::Text,
                "42 / 280",
                "Character counter",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TextareaHint",
                UiWidgetSlotKind::Description,
                "Keep notes short and actionable.",
                "Textarea hint",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn toast_widget() -> UiWidget {
    let id = UiComponentId::Toast;
    widget(
        id,
        UiWidgetPattern::Feedback,
        vec![
            slot(
                id,
                "ToastViewport",
                UiWidgetSlotKind::Panel,
                "Toast region",
                "Toast viewport",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "Toast",
                UiWidgetSlotKind::Overlay,
                "Branch pushed",
                "Toast surface",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ToastTitle",
                UiWidgetSlotKind::Title,
                "PR updated",
                "Toast title",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ToastDescription",
                UiWidgetSlotKind::Description,
                "GitHub checks are running.",
                "Toast description",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "ToastAction",
                UiWidgetSlotKind::Button,
                "View",
                "Toast action",
                UiWidgetIntent::Activate,
            ),
        ],
    )
}

pub fn toggle_widget() -> UiWidget {
    let id = UiComponentId::Toggle;
    widget(
        id,
        UiWidgetPattern::Action,
        vec![
            slot(
                id,
                "ToggleIndicator",
                UiWidgetSlotKind::Marker,
                "on",
                "Pressed indicator",
                UiWidgetIntent::None,
            )
            .selected(),
            slot(
                id,
                "ToggleLabel",
                UiWidgetSlotKind::Button,
                "Bold",
                "Toggle label",
                UiWidgetIntent::Toggle,
            )
            .selected(),
        ],
    )
}

pub fn toggle_group_widget() -> UiWidget {
    let id = UiComponentId::ToggleGroup;
    widget(
        id,
        UiWidgetPattern::Action,
        vec![
            slot(
                id,
                "ToggleGroupItem",
                UiWidgetSlotKind::Button,
                "Left",
                "Toggle item",
                UiWidgetIntent::Select,
            )
            .selected(),
            slot(
                id,
                "ToggleGroupIndicator",
                UiWidgetSlotKind::Marker,
                "selected",
                "Toggle indicator",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn tooltip_widget() -> UiWidget {
    let id = UiComponentId::Tooltip;
    widget(
        id,
        UiWidgetPattern::Overlay,
        vec![
            slot(
                id,
                "TooltipTrigger",
                UiWidgetSlotKind::Button,
                "Info",
                "Tooltip trigger",
                UiWidgetIntent::Open,
            ),
            slot(
                id,
                "TooltipContent",
                UiWidgetSlotKind::Overlay,
                "Uses shared theme tokens.",
                "Tooltip content",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TooltipArrow",
                UiWidgetSlotKind::Marker,
                "arrow",
                "Tooltip arrow",
                UiWidgetIntent::None,
            ),
        ],
    )
}

pub fn typography_widget() -> UiWidget {
    let id = UiComponentId::Typography;
    widget(
        id,
        UiWidgetPattern::Typography,
        vec![
            slot(
                id,
                "TypographyH1",
                UiWidgetSlotKind::Title,
                "Readable systems",
                "Heading one",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TypographyH2",
                UiWidgetSlotKind::Header,
                "Token scales",
                "Heading two",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TypographyP",
                UiWidgetSlotKind::Description,
                "Text sizes, leading, and spacing come from the shared scale.",
                "Paragraph",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TypographyList",
                UiWidgetSlotKind::List,
                "Headings, body, code",
                "List",
                UiWidgetIntent::None,
            ),
            slot(
                id,
                "TypographyBlockquote",
                UiWidgetSlotKind::Text,
                "Theme once, render everywhere.",
                "Blockquote",
                UiWidgetIntent::None,
            ),
        ],
    )
}

fn widget(id: UiComponentId, pattern: UiWidgetPattern, slots: Vec<UiWidgetSlot>) -> UiWidget {
    let implementation = component_implementation(id);
    let mut all_slots = Vec::with_capacity(slots.len() + 1);
    let root_part = id
        .anatomy_parts()
        .first()
        .copied()
        .expect("invariant: every catalog component has a root anatomy part");
    all_slots.push(root_slot(id, root_part));
    all_slots.extend(slots);
    UiWidget {
        id,
        pattern,
        state: implementation.state,
        slots: all_slots,
    }
}

fn root_slot(id: UiComponentId, part: &'static str) -> UiWidgetSlot {
    let definition = id.definition();
    UiWidgetSlot {
        part,
        kind: UiWidgetSlotKind::Section,
        role: UiBlockRole::Root,
        tone: tone_for_category(definition.category),
        label: definition.name,
        value: definition.summary,
        detail: detail_for_part(part, definition.state),
        intent: UiWidgetIntent::None,
        selected: false,
        disabled: false,
    }
}

fn slot(
    id: UiComponentId,
    part: &'static str,
    kind: UiWidgetSlotKind,
    label: &'static str,
    value: &'static str,
    intent: UiWidgetIntent,
) -> UiWidgetSlot {
    let definition = id.definition();
    let role = role_for_part(part);
    UiWidgetSlot {
        part,
        kind,
        role,
        tone: tone_for_role(role, definition.category),
        label,
        value,
        detail: detail_for_part(part, definition.state),
        intent,
        selected: false,
        disabled: false,
    }
}

impl UiWidgetSlot {
    pub const fn selected(mut self) -> Self {
        self.selected = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl UiWidgetPattern {
    pub const fn for_category(category: UiComponentCategory) -> Self {
        match category {
            UiComponentCategory::Action => Self::Action,
            UiComponentCategory::Data => Self::Data,
            UiComponentCategory::Disclosure => Self::Disclosure,
            UiComponentCategory::Display => Self::Display,
            UiComponentCategory::Feedback => Self::Feedback,
            UiComponentCategory::Form => Self::Form,
            UiComponentCategory::Layout => Self::Layout,
            UiComponentCategory::Messaging => Self::Messaging,
            UiComponentCategory::Navigation => Self::Navigation,
            UiComponentCategory::Overlay => Self::Overlay,
            UiComponentCategory::Typography => Self::Typography,
            UiComponentCategory::Utility => Self::Utility,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Action => "Action",
            Self::Callout => "Callout",
            Self::Data => "Data",
            Self::Disclosure => "Disclosure",
            Self::Display => "Display",
            Self::Feedback => "Feedback",
            Self::Form => "Form",
            Self::Layout => "Layout",
            Self::Messaging => "Messaging",
            Self::Navigation => "Navigation",
            Self::Overlay => "Overlay",
            Self::Typography => "Typography",
            Self::Utility => "Utility",
        }
    }
}

impl UiWidgetIntent {
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Activate => "Activate",
            Self::Close => "Close",
            Self::Dismiss => "Dismiss",
            Self::Input => "Input",
            Self::Navigate => "Navigate",
            Self::Open => "Open",
            Self::Resize => "Resize",
            Self::Scroll => "Scroll",
            Self::Select => "Select",
            Self::Toggle => "Toggle",
        }
    }
}

impl UiWidgetSlotKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Avatar => "Avatar",
            Self::Badge => "Badge",
            Self::Button => "Button",
            Self::Cell => "Cell",
            Self::Chart => "Chart",
            Self::Checkbox => "Checkbox",
            Self::Description => "Description",
            Self::Header => "Header",
            Self::IconButton => "Icon button",
            Self::Input => "Input",
            Self::Key => "Key",
            Self::Link => "Link",
            Self::List => "List",
            Self::ListItem => "List item",
            Self::Marker => "Marker",
            Self::Media => "Media",
            Self::Option => "Option",
            Self::Overlay => "Overlay",
            Self::Panel => "Panel",
            Self::Progress => "Progress",
            Self::Radio => "Radio",
            Self::Row => "Row",
            Self::Section => "Section",
            Self::Select => "Select",
            Self::Separator => "Separator",
            Self::Skeleton => "Skeleton",
            Self::Slider => "Slider",
            Self::Spinner => "Spinner",
            Self::Switch => "Switch",
            Self::Table => "Table",
            Self::Text => "Text",
            Self::Textarea => "Textarea",
            Self::Title => "Title",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{SHADCN_COMPONENT_COUNT, UiComponentId};

    #[test]
    fn every_catalog_component_has_a_literal_widget_constructor() {
        assert_eq!(UI_WIDGET_CONSTRUCTORS.len(), SHADCN_COMPONENT_COUNT);
        let ids = implemented_widgets()
            .map(|widget| widget.id)
            .collect::<HashSet<_>>();
        assert_eq!(ids.len(), SHADCN_COMPONENT_COUNT);
        for id in UiComponentId::ALL {
            assert!(ids.contains(&id), "missing literal widget for {id:?}");
        }
    }

    #[test]
    fn widget_for_component_preserves_catalog_order() {
        for id in UiComponentId::ALL {
            let widget = widget_for_component(id);
            assert_eq!(widget.id, id);
            assert_eq!(UI_WIDGET_CONSTRUCTORS[id.index()]().id, id);
        }
    }

    #[test]
    fn literal_widgets_cover_the_catalog_anatomy() {
        for widget in implemented_widgets() {
            let part_names = widget
                .slots
                .iter()
                .map(|slot| slot.part)
                .collect::<HashSet<_>>();
            for part in widget.id.anatomy_parts() {
                assert!(
                    part_names.contains(part),
                    "{:?} widget is missing anatomy part {part}",
                    widget.id
                );
            }
        }
    }

    #[test]
    fn literal_widgets_share_the_implementation_contract() {
        for widget in implemented_widgets() {
            let implementation = component_implementation(widget.id);
            assert_eq!(widget.state, implementation.state);
            assert!(
                !widget.slots.is_empty(),
                "{:?} should expose concrete render slots",
                widget.id
            );
        }
    }

    #[test]
    fn literal_widgets_validate_with_garde() {
        for widget in implemented_widgets() {
            validate_widget(&widget).expect("literal widget should validate");
        }
    }

    #[test]
    fn render_nodes_cover_every_literal_widget() {
        for widget in implemented_widgets() {
            let nodes = widget_render_nodes(&widget).expect("literal widget should validate");
            assert_eq!(nodes.len(), widget.slots.len());
            assert_eq!(
                nodes.first().map(|node| node.part),
                widget.id.anatomy_parts().first().copied()
            );
        }
    }
}
