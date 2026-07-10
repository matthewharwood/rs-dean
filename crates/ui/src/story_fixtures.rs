use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiStoryVariantKind {
    Default,
    Alternate(u8),
    Loading,
    Disabled,
    Themed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiStoryModel {
    Accordion(AccordionModel),
    Alert(AlertModel),
    AlertDialog(AlertDialogModel),
    AspectRatio(AspectRatioModel),
    Attachment(AttachmentModel),
    Avatar(AvatarModel),
    Badge(BadgeModel),
    Breadcrumb(BreadcrumbModel),
    Bubble(BubbleModel),
    Button(ButtonModel),
    ButtonGroup(ButtonGroupModel),
    Calendar(CalendarModel),
    Card(CardModel),
    Carousel(CarouselModel),
    Chart(ChartModel),
    Checkbox(CheckboxModel),
    Collapsible(CollapsibleModel),
    Combobox(ComboboxModel),
    Command(CommandModel),
    ContextMenu(ContextMenuModel),
    DataTable(DataTableModel),
    DatePicker(DatePickerModel),
    Dialog(DialogModel),
    Direction(DirectionModel),
    Drawer(DrawerModel),
    DropdownMenu(DropdownMenuModel),
    Empty(EmptyModel),
    Field(FieldModel),
    HoverCard(HoverCardModel),
    Input(InputModel),
    InputGroup(InputGroupModel),
    InputOtp(InputOtpModel),
    Item(ItemModel),
    Kbd(KbdModel),
    Label(LabelModel),
    Marker(MarkerModel),
    Menubar(MenubarModel),
    Message(MessageModel),
    MessageScroller(MessageScrollerModel),
    NativeSelect(NativeSelectModel),
    NavigationMenu(NavigationMenuModel),
    Pagination(PaginationModel),
    Popover(PopoverModel),
    Progress(ProgressModel),
    RadioGroup(RadioGroupModel),
    Resizable(ResizableModel),
    ScrollArea(ScrollAreaModel),
    Select(SelectModel),
    Separator(SeparatorModel),
    Sheet(SheetModel),
    Sidebar(SidebarModel),
    Skeleton(SkeletonModel),
    Slider(SliderModel),
    Sonner(SonnerModel),
    Spinner(SpinnerModel),
    Switch(SwitchModel),
    Table(TableModel),
    Tabs(TabsModel),
    Textarea(TextareaModel),
    Toast(ToastModel),
    Toggle(ToggleModel),
    ToggleGroup(ToggleGroupModel),
    Tooltip(TooltipModel),
    Typography(TypographyModel),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiStoryFixture {
    pub fixture_id: &'static str,
    pub kind: UiStoryVariantKind,
    pub theme_id: ThemeId,
    pub model: UiStoryModel,
    pub default_open: bool,
}

impl UiStoryModel {
    pub const fn component_id(&self) -> UiComponentId {
        match self {
            Self::Accordion(_) => UiComponentId::Accordion,
            Self::Alert(_) => UiComponentId::Alert,
            Self::AlertDialog(_) => UiComponentId::AlertDialog,
            Self::AspectRatio(_) => UiComponentId::AspectRatio,
            Self::Attachment(_) => UiComponentId::Attachment,
            Self::Avatar(_) => UiComponentId::Avatar,
            Self::Badge(_) => UiComponentId::Badge,
            Self::Breadcrumb(_) => UiComponentId::Breadcrumb,
            Self::Bubble(_) => UiComponentId::Bubble,
            Self::Button(_) => UiComponentId::Button,
            Self::ButtonGroup(_) => UiComponentId::ButtonGroup,
            Self::Calendar(_) => UiComponentId::Calendar,
            Self::Card(_) => UiComponentId::Card,
            Self::Carousel(_) => UiComponentId::Carousel,
            Self::Chart(_) => UiComponentId::Chart,
            Self::Checkbox(_) => UiComponentId::Checkbox,
            Self::Collapsible(_) => UiComponentId::Collapsible,
            Self::Combobox(_) => UiComponentId::Combobox,
            Self::Command(_) => UiComponentId::Command,
            Self::ContextMenu(_) => UiComponentId::ContextMenu,
            Self::DataTable(_) => UiComponentId::DataTable,
            Self::DatePicker(_) => UiComponentId::DatePicker,
            Self::Dialog(_) => UiComponentId::Dialog,
            Self::Direction(_) => UiComponentId::Direction,
            Self::Drawer(_) => UiComponentId::Drawer,
            Self::DropdownMenu(_) => UiComponentId::DropdownMenu,
            Self::Empty(_) => UiComponentId::Empty,
            Self::Field(_) => UiComponentId::Field,
            Self::HoverCard(_) => UiComponentId::HoverCard,
            Self::Input(_) => UiComponentId::Input,
            Self::InputGroup(_) => UiComponentId::InputGroup,
            Self::InputOtp(_) => UiComponentId::InputOtp,
            Self::Item(_) => UiComponentId::Item,
            Self::Kbd(_) => UiComponentId::Kbd,
            Self::Label(_) => UiComponentId::Label,
            Self::Marker(_) => UiComponentId::Marker,
            Self::Menubar(_) => UiComponentId::Menubar,
            Self::Message(_) => UiComponentId::Message,
            Self::MessageScroller(_) => UiComponentId::MessageScroller,
            Self::NativeSelect(_) => UiComponentId::NativeSelect,
            Self::NavigationMenu(_) => UiComponentId::NavigationMenu,
            Self::Pagination(_) => UiComponentId::Pagination,
            Self::Popover(_) => UiComponentId::Popover,
            Self::Progress(_) => UiComponentId::Progress,
            Self::RadioGroup(_) => UiComponentId::RadioGroup,
            Self::Resizable(_) => UiComponentId::Resizable,
            Self::ScrollArea(_) => UiComponentId::ScrollArea,
            Self::Select(_) => UiComponentId::Select,
            Self::Separator(_) => UiComponentId::Separator,
            Self::Sheet(_) => UiComponentId::Sheet,
            Self::Sidebar(_) => UiComponentId::Sidebar,
            Self::Skeleton(_) => UiComponentId::Skeleton,
            Self::Slider(_) => UiComponentId::Slider,
            Self::Sonner(_) => UiComponentId::Sonner,
            Self::Spinner(_) => UiComponentId::Spinner,
            Self::Switch(_) => UiComponentId::Switch,
            Self::Table(_) => UiComponentId::Table,
            Self::Tabs(_) => UiComponentId::Tabs,
            Self::Textarea(_) => UiComponentId::Textarea,
            Self::Toast(_) => UiComponentId::Toast,
            Self::Toggle(_) => UiComponentId::Toggle,
            Self::ToggleGroup(_) => UiComponentId::ToggleGroup,
            Self::Tooltip(_) => UiComponentId::Tooltip,
            Self::Typography(_) => UiComponentId::Typography,
        }
    }
}

impl UiStoryVariantKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Alternate(_) => "Alternate",
            Self::Loading => "Loading",
            Self::Disabled => "Disabled",
            Self::Themed => "Themed",
        }
    }

    pub const fn detail(self) -> &'static str {
        match self {
            Self::Default => "Validated default fixture",
            Self::Alternate(_) => "Interactive or alternate fixture state",
            Self::Loading => "Stable loading layout",
            Self::Disabled => "Read-only disabled state",
            Self::Themed => "Nested ThemeScope palette",
        }
    }
}

impl UiStoryModel {
    pub fn validate(&self) -> Result<(), garde::Report> {
        match self {
            Self::Accordion(model) => validate_accordion_model(model),
            Self::Alert(model) => validate_alert_model(model),
            Self::AlertDialog(model) => validate_alert_dialog_model(model),
            Self::AspectRatio(model) => validate_aspect_ratio_model(model),
            Self::Attachment(model) => validate_attachment_model(model),
            Self::Avatar(model) => validate_avatar_model(model),
            Self::Badge(model) => validate_badge_model(model),
            Self::Breadcrumb(model) => validate_breadcrumb_model(model),
            Self::Bubble(model) => validate_bubble_model(model),
            Self::Button(model) => validate_button_model(model),
            Self::ButtonGroup(model) => validate_button_group_model(model),
            Self::Calendar(model) => validate_calendar_model(model),
            Self::Card(model) => validate_card_model(model),
            Self::Carousel(model) => validate_carousel_model(model),
            Self::Chart(model) => validate_chart_model(model),
            Self::Checkbox(model) => validate_checkbox_model(model),
            Self::Collapsible(model) => validate_collapsible_model(model),
            Self::Combobox(model) => validate_combobox_model(model),
            Self::Command(model) => validate_command_model(model),
            Self::ContextMenu(model) => validate_context_menu_model(model),
            Self::DataTable(model) => validate_data_table_model(model),
            Self::DatePicker(model) => validate_date_picker_model(model),
            Self::Dialog(model) => validate_dialog_model(model),
            Self::Direction(model) => validate_direction_model(model),
            Self::Drawer(model) => validate_drawer_model(model),
            Self::DropdownMenu(model) => validate_dropdown_menu_model(model),
            Self::Empty(model) => validate_empty_model(model),
            Self::Field(model) => validate_field_model(model),
            Self::HoverCard(model) => validate_hover_card_model(model),
            Self::Input(model) => validate_input_model(model),
            Self::InputGroup(model) => validate_input_group_model(model),
            Self::InputOtp(model) => validate_input_otp_model(model),
            Self::Item(model) => validate_item_model(model),
            Self::Kbd(model) => validate_kbd_model(model),
            Self::Label(model) => validate_label_model(model),
            Self::Marker(model) => validate_marker_model(model),
            Self::Menubar(model) => validate_menubar_model(model),
            Self::Message(model) => validate_message_model(model),
            Self::MessageScroller(model) => validate_message_scroller_model(model),
            Self::NativeSelect(model) => validate_native_select_model(model),
            Self::NavigationMenu(model) => validate_navigation_menu_model(model),
            Self::Pagination(model) => validate_pagination_model(model),
            Self::Popover(model) => validate_popover_model(model),
            Self::Progress(model) => validate_progress_model(model),
            Self::RadioGroup(model) => validate_radio_group_model(model),
            Self::Resizable(model) => validate_resizable_model(model),
            Self::ScrollArea(model) => validate_scroll_area_model(model),
            Self::Select(model) => validate_select_model(model),
            Self::Separator(model) => validate_separator_model(model),
            Self::Sheet(model) => validate_sheet_model(model),
            Self::Sidebar(model) => validate_sidebar_model(model),
            Self::Skeleton(model) => validate_skeleton_model(model),
            Self::Slider(model) => validate_slider_model(model),
            Self::Sonner(model) => validate_sonner_model(model),
            Self::Spinner(model) => validate_spinner_model(model),
            Self::Switch(model) => validate_switch_model(model),
            Self::Table(model) => validate_table_model(model),
            Self::Tabs(model) => validate_tabs_model(model),
            Self::Textarea(model) => validate_textarea_model(model),
            Self::Toast(model) => validate_toast_model(model),
            Self::Toggle(model) => validate_toggle_model(model),
            Self::ToggleGroup(model) => validate_toggle_group_model(model),
            Self::Tooltip(model) => validate_tooltip_model(model),
            Self::Typography(model) => validate_typography_model(model),
        }
    }
}

impl UiStoryFixture {
    pub const fn new(
        fixture_id: &'static str,
        kind: UiStoryVariantKind,
        theme_id: ThemeId,
        model: UiStoryModel,
    ) -> Self {
        Self {
            fixture_id,
            kind,
            theme_id,
            model,
            default_open: false,
        }
    }

    pub const fn default_open(mut self) -> Self {
        self.default_open = true;
        self
    }
}

macro_rules! story_fixture {
    ($variant:ident, $fixture_id:literal, $kind:expr, $theme:expr, $model:expr) => {
        UiStoryFixture::new($fixture_id, $kind, $theme, UiStoryModel::$variant($model))
    };
}

pub fn ui_story_fixtures(id: UiComponentId) -> Vec<UiStoryFixture> {
    match id {
        UiComponentId::Accordion => vec![story_fixture!(
            Accordion,
            "default",
            UiStoryVariantKind::Default,
            ThemeId::Dark,
            AccordionModel::new(AccordionMode::Multiple, accordion_story_items())
                .with_default_open(vec!["tokens".to_owned(), "bevy".to_owned()])
        )],
        UiComponentId::Alert => vec![
            story_fixture!(
                Alert,
                "default_alert",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_alert_story_model()
            ),
            story_fixture!(
                Alert,
                "dense_warning_alert",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_warning_alert_story_model()
            ),
            story_fixture!(
                Alert,
                "loading_alert",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_alert_story_model()
            ),
            story_fixture!(
                Alert,
                "disabled_error_alert",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_error_alert_story_model()
            ),
            story_fixture!(
                Alert,
                "themed_alert",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_alert_story_model()
            ),
        ],
        UiComponentId::AlertDialog => vec![
            story_fixture!(
                AlertDialog,
                "default_alert_dialog",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_alert_dialog_story_model()
            ),
            story_fixture!(
                AlertDialog,
                "open_destructive_alert_dialog",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                open_destructive_alert_dialog_story_model()
            )
            .default_open(),
            story_fixture!(
                AlertDialog,
                "small_loading_alert_dialog",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                small_loading_alert_dialog_story_model()
            )
            .default_open(),
            story_fixture!(
                AlertDialog,
                "disabled_alert_dialog",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_alert_dialog_story_model()
            ),
            story_fixture!(
                AlertDialog,
                "themed_alert_dialog",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_alert_dialog_story_model()
            )
            .default_open(),
        ],
        UiComponentId::AspectRatio => vec![
            story_fixture!(
                AspectRatio,
                "default_aspect_ratio",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_aspect_ratio_story_model()
            ),
            story_fixture!(
                AspectRatio,
                "contain_aspect_ratio",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                contain_aspect_ratio_story_model()
            ),
            story_fixture!(
                AspectRatio,
                "loading_aspect_ratio",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_aspect_ratio_story_model()
            ),
            story_fixture!(
                AspectRatio,
                "disabled_aspect_ratio",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_aspect_ratio_story_model()
            ),
            story_fixture!(
                AspectRatio,
                "themed_aspect_ratio",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_aspect_ratio_story_model()
            ),
        ],
        UiComponentId::Attachment => vec![
            story_fixture!(
                Attachment,
                "default_attachment",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_attachment_story_model()
            ),
            story_fixture!(
                Attachment,
                "image_attachment",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                image_attachment_story_model()
            ),
            story_fixture!(
                Attachment,
                "loading_attachment",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_attachment_story_model()
            ),
            story_fixture!(
                Attachment,
                "disabled_attachment",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_attachment_story_model()
            ),
            story_fixture!(
                Attachment,
                "themed_attachment",
                UiStoryVariantKind::Themed,
                ThemeId::Forest,
                themed_attachment_story_model()
            ),
        ],
        UiComponentId::Avatar => vec![
            story_fixture!(
                Avatar,
                "default_avatar",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_avatar_story_model()
            ),
            story_fixture!(
                Avatar,
                "fallback_avatar",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                fallback_avatar_story_model()
            ),
            story_fixture!(
                Avatar,
                "loading_avatar",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_avatar_story_model()
            ),
            story_fixture!(
                Avatar,
                "disabled_avatar",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_avatar_story_model()
            ),
            story_fixture!(
                Avatar,
                "themed_avatar",
                UiStoryVariantKind::Themed,
                ThemeId::Synthwave,
                themed_avatar_story_model()
            ),
        ],
        UiComponentId::Badge => vec![
            story_fixture!(
                Badge,
                "default_badge",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_badge_story_model()
            ),
            story_fixture!(
                Badge,
                "no_icon_badge",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                no_icon_badge_story_model()
            ),
            story_fixture!(
                Badge,
                "loading_badge",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_badge_story_model()
            ),
            story_fixture!(
                Badge,
                "disabled_badge",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_badge_story_model()
            ),
            story_fixture!(
                Badge,
                "themed_badge",
                UiStoryVariantKind::Themed,
                ThemeId::Lofi,
                themed_badge_story_model()
            ),
        ],
        UiComponentId::Breadcrumb => vec![
            story_fixture!(
                Breadcrumb,
                "default_breadcrumb",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_breadcrumb_story_model()
            ),
            story_fixture!(
                Breadcrumb,
                "dense_breadcrumb",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_breadcrumb_story_model()
            ),
            story_fixture!(
                Breadcrumb,
                "loading_breadcrumb",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_breadcrumb_story_model()
            ),
            story_fixture!(
                Breadcrumb,
                "disabled_breadcrumb",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_breadcrumb_story_model()
            ),
            story_fixture!(
                Breadcrumb,
                "themed_breadcrumb",
                UiStoryVariantKind::Themed,
                ThemeId::Catppuccin,
                themed_breadcrumb_story_model()
            ),
        ],
        UiComponentId::Bubble => vec![
            story_fixture!(
                Bubble,
                "default_bubble",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_bubble_story_model()
            ),
            story_fixture!(
                Bubble,
                "outgoing_bubble",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                outgoing_bubble_story_model()
            ),
            story_fixture!(
                Bubble,
                "loading_bubble",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_bubble_story_model()
            ),
            story_fixture!(
                Bubble,
                "disabled_bubble",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_bubble_story_model()
            ),
            story_fixture!(
                Bubble,
                "themed_bubble",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_bubble_story_model()
            ),
        ],
        UiComponentId::Button => vec![
            story_fixture!(
                Button,
                "default_button",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_button_story_model()
            ),
            story_fixture!(
                Button,
                "secondary_button",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                secondary_button_story_model()
            ),
            story_fixture!(
                Button,
                "link_button",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                link_button_story_model()
            ),
            story_fixture!(
                Button,
                "loading_button",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_button_story_model()
            ),
            story_fixture!(
                Button,
                "disabled_button",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_button_story_model()
            ),
            story_fixture!(
                Button,
                "themed_button",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_button_story_model()
            ),
        ],
        UiComponentId::ButtonGroup => vec![
            story_fixture!(
                ButtonGroup,
                "default_button_group",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_button_group_story_model()
            ),
            story_fixture!(
                ButtonGroup,
                "vertical_button_group",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                vertical_button_group_story_model()
            ),
            story_fixture!(
                ButtonGroup,
                "loading_button_group",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_button_group_story_model()
            ),
            story_fixture!(
                ButtonGroup,
                "disabled_button_group",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_button_group_story_model()
            ),
            story_fixture!(
                ButtonGroup,
                "themed_button_group",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_button_group_story_model()
            ),
        ],
        UiComponentId::Calendar => vec![
            story_fixture!(
                Calendar,
                "default_calendar",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_calendar_story_model()
            ),
            story_fixture!(
                Calendar,
                "range_calendar",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                range_calendar_story_model()
            ),
            story_fixture!(
                Calendar,
                "loading_calendar",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_calendar_story_model()
            ),
            story_fixture!(
                Calendar,
                "disabled_calendar",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_calendar_story_model()
            ),
            story_fixture!(
                Calendar,
                "themed_calendar",
                UiStoryVariantKind::Themed,
                ThemeId::Forest,
                themed_calendar_story_model()
            ),
        ],
        UiComponentId::Card => vec![
            story_fixture!(
                Card,
                "default_card",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_card_story_model()
            ),
            story_fixture!(
                Card,
                "dense_card",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_card_story_model()
            ),
            story_fixture!(
                Card,
                "loading_card",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_card_story_model()
            ),
            story_fixture!(
                Card,
                "disabled_card",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_card_story_model()
            ),
            story_fixture!(
                Card,
                "themed_card",
                UiStoryVariantKind::Themed,
                ThemeId::Synthwave,
                themed_card_story_model()
            ),
        ],
        UiComponentId::Carousel => vec![
            story_fixture!(
                Carousel,
                "default_carousel",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_carousel_story_model()
            ),
            story_fixture!(
                Carousel,
                "dense_carousel",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_carousel_story_model()
            ),
            story_fixture!(
                Carousel,
                "loading_carousel",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_carousel_story_model()
            ),
            story_fixture!(
                Carousel,
                "disabled_carousel",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_carousel_story_model()
            ),
            story_fixture!(
                Carousel,
                "themed_carousel",
                UiStoryVariantKind::Themed,
                ThemeId::Catppuccin,
                themed_carousel_story_model()
            ),
        ],
        UiComponentId::Chart => vec![
            story_fixture!(
                Chart,
                "default_chart",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_chart_story_model()
            ),
            story_fixture!(
                Chart,
                "dense_chart",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_chart_story_model()
            ),
            story_fixture!(
                Chart,
                "loading_chart",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_chart_story_model()
            ),
            story_fixture!(
                Chart,
                "disabled_chart",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_chart_story_model()
            ),
            story_fixture!(
                Chart,
                "themed_chart",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_chart_story_model()
            ),
        ],
        UiComponentId::Checkbox => vec![
            story_fixture!(
                Checkbox,
                "default_checkbox",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_checkbox_story_model()
            ),
            story_fixture!(
                Checkbox,
                "dense_checkbox",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_checkbox_story_model()
            ),
            story_fixture!(
                Checkbox,
                "loading_checkbox",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_checkbox_story_model()
            ),
            story_fixture!(
                Checkbox,
                "disabled_checkbox",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_checkbox_story_model()
            ),
            story_fixture!(
                Checkbox,
                "themed_checkbox",
                UiStoryVariantKind::Themed,
                ThemeId::Forest,
                themed_checkbox_story_model()
            ),
        ],
        UiComponentId::Collapsible => vec![
            story_fixture!(
                Collapsible,
                "default_collapsible",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_collapsible_story_model()
            ),
            story_fixture!(
                Collapsible,
                "dense_collapsible",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_collapsible_story_model()
            ),
            story_fixture!(
                Collapsible,
                "loading_collapsible",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_collapsible_story_model()
            ),
            story_fixture!(
                Collapsible,
                "disabled_collapsible",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_collapsible_story_model()
            ),
            story_fixture!(
                Collapsible,
                "themed_collapsible",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_collapsible_story_model()
            ),
        ],
        UiComponentId::Combobox => vec![
            story_fixture!(
                Combobox,
                "default_combobox",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_combobox_story_model()
            ),
            story_fixture!(
                Combobox,
                "dense_combobox",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_combobox_story_model()
            ),
            story_fixture!(
                Combobox,
                "loading_combobox",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_combobox_story_model()
            ),
            story_fixture!(
                Combobox,
                "disabled_combobox",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_combobox_story_model()
            ),
            story_fixture!(
                Combobox,
                "themed_combobox",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_combobox_story_model()
            ),
        ],
        UiComponentId::Command => vec![
            story_fixture!(
                Command,
                "default_command",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_command_story_model()
            ),
            story_fixture!(
                Command,
                "dense_command",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_command_story_model()
            ),
            story_fixture!(
                Command,
                "loading_command",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_command_story_model()
            ),
            story_fixture!(
                Command,
                "disabled_command",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_command_story_model()
            ),
            story_fixture!(
                Command,
                "themed_command",
                UiStoryVariantKind::Themed,
                ThemeId::Lofi,
                themed_command_story_model()
            ),
        ],
        UiComponentId::ContextMenu => vec![
            story_fixture!(
                ContextMenu,
                "default_context_menu",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_context_menu_story_model()
            ),
            story_fixture!(
                ContextMenu,
                "dense_context_menu",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_context_menu_story_model()
            ),
            story_fixture!(
                ContextMenu,
                "loading_context_menu",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_context_menu_story_model()
            ),
            story_fixture!(
                ContextMenu,
                "disabled_context_menu",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_context_menu_story_model()
            ),
            story_fixture!(
                ContextMenu,
                "themed_context_menu",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_context_menu_story_model()
            ),
        ],
        UiComponentId::DataTable => vec![
            story_fixture!(
                DataTable,
                "default_data_table",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_data_table_story_model()
            ),
            story_fixture!(
                DataTable,
                "dense_data_table",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_data_table_story_model()
            ),
            story_fixture!(
                DataTable,
                "loading_data_table",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_data_table_story_model()
            ),
            story_fixture!(
                DataTable,
                "disabled_data_table",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_data_table_story_model()
            ),
            story_fixture!(
                DataTable,
                "themed_data_table",
                UiStoryVariantKind::Themed,
                ThemeId::Synthwave,
                themed_data_table_story_model()
            ),
        ],
        UiComponentId::DatePicker => vec![
            story_fixture!(
                DatePicker,
                "default_date_picker",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_date_picker_story_model()
            ),
            story_fixture!(
                DatePicker,
                "dense_open_date_picker",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_open_date_picker_story_model()
            ),
            story_fixture!(
                DatePicker,
                "loading_date_picker",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_date_picker_story_model()
            ),
            story_fixture!(
                DatePicker,
                "disabled_date_picker",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_date_picker_story_model()
            ),
            story_fixture!(
                DatePicker,
                "themed_date_picker",
                UiStoryVariantKind::Themed,
                ThemeId::Catppuccin,
                themed_date_picker_story_model()
            ),
        ],
        UiComponentId::Dialog => vec![
            story_fixture!(
                Dialog,
                "default_dialog",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_dialog_story_model()
            ),
            story_fixture!(
                Dialog,
                "open_small_dialog",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                open_small_dialog_story_model()
            ),
            story_fixture!(
                Dialog,
                "loading_dialog",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_dialog_story_model()
            ),
            story_fixture!(
                Dialog,
                "disabled_dialog",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_dialog_story_model()
            ),
            story_fixture!(
                Dialog,
                "themed_dialog",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_dialog_story_model()
            ),
        ],
        UiComponentId::Direction => vec![
            story_fixture!(
                Direction,
                "default_direction",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_direction_story_model()
            ),
            story_fixture!(
                Direction,
                "rtl_direction",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                rtl_direction_story_model()
            ),
            story_fixture!(
                Direction,
                "scoped_direction",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                scoped_direction_story_model()
            ),
            story_fixture!(
                Direction,
                "loading_direction",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_direction_story_model()
            ),
            story_fixture!(
                Direction,
                "disabled_direction",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_direction_story_model()
            ),
            story_fixture!(
                Direction,
                "themed_direction",
                UiStoryVariantKind::Themed,
                ThemeId::Forest,
                themed_direction_story_model()
            ),
        ],
        UiComponentId::Drawer => vec![
            story_fixture!(
                Drawer,
                "default_drawer",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_drawer_story_model()
            ),
            story_fixture!(
                Drawer,
                "right_drawer",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                right_drawer_story_model()
            ),
            story_fixture!(
                Drawer,
                "loading_drawer",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_drawer_story_model()
            ),
            story_fixture!(
                Drawer,
                "disabled_drawer",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_drawer_story_model()
            ),
            story_fixture!(
                Drawer,
                "themed_drawer",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_drawer_story_model()
            ),
        ],
        UiComponentId::DropdownMenu => vec![
            story_fixture!(
                DropdownMenu,
                "default_dropdown_menu",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_dropdown_menu_story_model()
            ),
            story_fixture!(
                DropdownMenu,
                "dense_dropdown_menu",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_dropdown_menu_story_model()
            ),
            story_fixture!(
                DropdownMenu,
                "loading_dropdown_menu",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_dropdown_menu_story_model()
            ),
            story_fixture!(
                DropdownMenu,
                "disabled_dropdown_menu",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_dropdown_menu_story_model()
            ),
            story_fixture!(
                DropdownMenu,
                "themed_dropdown_menu",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_dropdown_menu_story_model()
            ),
        ],
        UiComponentId::Empty => vec![
            story_fixture!(
                Empty,
                "default_empty",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_empty_story_model()
            ),
            story_fixture!(
                Empty,
                "dense_empty",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_empty_story_model()
            ),
            story_fixture!(
                Empty,
                "loading_empty",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_empty_story_model()
            ),
            story_fixture!(
                Empty,
                "disabled_empty",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_empty_story_model()
            ),
            story_fixture!(
                Empty,
                "themed_empty",
                UiStoryVariantKind::Themed,
                ThemeId::Forest,
                themed_empty_story_model()
            ),
        ],
        UiComponentId::Field => vec![
            story_fixture!(
                Field,
                "default_field",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_field_story_model()
            ),
            story_fixture!(
                Field,
                "dense_field",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_field_story_model()
            ),
            story_fixture!(
                Field,
                "loading_field",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_field_story_model()
            ),
            story_fixture!(
                Field,
                "disabled_field",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_field_story_model()
            ),
            story_fixture!(
                Field,
                "themed_field",
                UiStoryVariantKind::Themed,
                ThemeId::Catppuccin,
                themed_field_story_model()
            ),
        ],
        UiComponentId::HoverCard => vec![
            story_fixture!(
                HoverCard,
                "default_hover_card",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_hover_card_story_model()
            ),
            story_fixture!(
                HoverCard,
                "dense_hover_card",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_hover_card_story_model()
            ),
            story_fixture!(
                HoverCard,
                "loading_hover_card",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_hover_card_story_model()
            ),
            story_fixture!(
                HoverCard,
                "disabled_hover_card",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_hover_card_story_model()
            ),
            story_fixture!(
                HoverCard,
                "themed_hover_card",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_hover_card_story_model()
            ),
        ],
        UiComponentId::Input => vec![
            story_fixture!(
                Input,
                "default_input",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_input_story_model()
            ),
            story_fixture!(
                Input,
                "dense_input",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_input_story_model()
            ),
            story_fixture!(
                Input,
                "loading_input",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_input_story_model()
            ),
            story_fixture!(
                Input,
                "disabled_input",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_input_story_model()
            ),
            story_fixture!(
                Input,
                "themed_input",
                UiStoryVariantKind::Themed,
                ThemeId::Lofi,
                themed_input_story_model()
            ),
        ],
        UiComponentId::InputGroup => vec![
            story_fixture!(
                InputGroup,
                "default_input_group",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_input_group_story_model()
            ),
            story_fixture!(
                InputGroup,
                "dense_input_group",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_input_group_story_model()
            ),
            story_fixture!(
                InputGroup,
                "loading_input_group",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_input_group_story_model()
            ),
            story_fixture!(
                InputGroup,
                "disabled_input_group",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_input_group_story_model()
            ),
            story_fixture!(
                InputGroup,
                "themed_input_group",
                UiStoryVariantKind::Themed,
                ThemeId::Synthwave,
                themed_input_group_story_model()
            ),
        ],
        UiComponentId::InputOtp => vec![
            story_fixture!(
                InputOtp,
                "default_input_otp",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_input_otp_story_model()
            ),
            story_fixture!(
                InputOtp,
                "dense_input_otp",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_input_otp_story_model()
            ),
            story_fixture!(
                InputOtp,
                "loading_input_otp",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_input_otp_story_model()
            ),
            story_fixture!(
                InputOtp,
                "disabled_input_otp",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_input_otp_story_model()
            ),
            story_fixture!(
                InputOtp,
                "themed_input_otp",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_input_otp_story_model()
            ),
        ],
        UiComponentId::Item => vec![
            story_fixture!(
                Item,
                "default_item",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_item_story_model()
            ),
            story_fixture!(
                Item,
                "dense_item",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_item_story_model()
            ),
            story_fixture!(
                Item,
                "loading_item",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_item_story_model()
            ),
            story_fixture!(
                Item,
                "disabled_item",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_item_story_model()
            ),
            story_fixture!(
                Item,
                "themed_item",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_item_story_model()
            ),
        ],
        UiComponentId::Kbd => vec![
            story_fixture!(
                Kbd,
                "default_kbd",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_kbd_story_model()
            ),
            story_fixture!(
                Kbd,
                "dense_kbd",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_kbd_story_model()
            ),
            story_fixture!(
                Kbd,
                "loading_kbd",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_kbd_story_model()
            ),
            story_fixture!(
                Kbd,
                "disabled_kbd",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_kbd_story_model()
            ),
            story_fixture!(
                Kbd,
                "themed_kbd",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_kbd_story_model()
            ),
        ],
        UiComponentId::Label => vec![
            story_fixture!(
                Label,
                "default_label",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_label_story_model()
            ),
            story_fixture!(
                Label,
                "dense_label",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_label_story_model()
            ),
            story_fixture!(
                Label,
                "loading_label",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_label_story_model()
            ),
            story_fixture!(
                Label,
                "disabled_label",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_label_story_model()
            ),
            story_fixture!(
                Label,
                "themed_label",
                UiStoryVariantKind::Themed,
                ThemeId::Catppuccin,
                themed_label_story_model()
            ),
        ],
        UiComponentId::Marker => vec![
            story_fixture!(
                Marker,
                "default_marker",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_marker_story_model()
            ),
            story_fixture!(
                Marker,
                "dense_marker",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_marker_story_model()
            ),
            story_fixture!(
                Marker,
                "loading_marker",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_marker_story_model()
            ),
            story_fixture!(
                Marker,
                "disabled_marker",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_marker_story_model()
            ),
            story_fixture!(
                Marker,
                "themed_marker",
                UiStoryVariantKind::Themed,
                ThemeId::Forest,
                themed_marker_story_model()
            ),
        ],
        UiComponentId::Menubar => vec![
            story_fixture!(
                Menubar,
                "default_menubar",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_menubar_story_model()
            ),
            story_fixture!(
                Menubar,
                "dense_menubar",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_menubar_story_model()
            ),
            story_fixture!(
                Menubar,
                "loading_menubar",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_menubar_story_model()
            ),
            story_fixture!(
                Menubar,
                "disabled_menubar",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_menubar_story_model()
            ),
            story_fixture!(
                Menubar,
                "themed_menubar",
                UiStoryVariantKind::Themed,
                ThemeId::Synthwave,
                themed_menubar_story_model()
            ),
        ],
        UiComponentId::Message => vec![
            story_fixture!(
                Message,
                "default_message",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_message_story_model()
            ),
            story_fixture!(
                Message,
                "dense_outgoing_message",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_outgoing_message_story_model()
            ),
            story_fixture!(
                Message,
                "loading_message",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_message_story_model()
            ),
            story_fixture!(
                Message,
                "disabled_message",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_message_story_model()
            ),
            story_fixture!(
                Message,
                "themed_message",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_message_story_model()
            ),
        ],
        UiComponentId::MessageScroller => vec![
            story_fixture!(
                MessageScroller,
                "default_message_scroller",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_message_scroller_story_model()
            ),
            story_fixture!(
                MessageScroller,
                "dense_latest_message_scroller",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_latest_message_scroller_story_model()
            ),
            story_fixture!(
                MessageScroller,
                "loading_message_scroller",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_message_scroller_story_model()
            ),
            story_fixture!(
                MessageScroller,
                "disabled_message_scroller",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_message_scroller_story_model()
            ),
            story_fixture!(
                MessageScroller,
                "themed_message_scroller",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_message_scroller_story_model()
            ),
        ],
        UiComponentId::NativeSelect => vec![
            story_fixture!(
                NativeSelect,
                "default_native_select",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_native_select_story_model()
            ),
            story_fixture!(
                NativeSelect,
                "dense_native_select",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_native_select_story_model()
            ),
            story_fixture!(
                NativeSelect,
                "loading_native_select",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_native_select_story_model()
            ),
            story_fixture!(
                NativeSelect,
                "disabled_native_select",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_native_select_story_model()
            ),
            story_fixture!(
                NativeSelect,
                "themed_native_select",
                UiStoryVariantKind::Themed,
                ThemeId::Catppuccin,
                themed_native_select_story_model()
            ),
        ],
        UiComponentId::NavigationMenu => vec![
            story_fixture!(
                NavigationMenu,
                "default_navigation_menu",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_navigation_menu_story_model()
            ),
            story_fixture!(
                NavigationMenu,
                "dense_navigation_menu",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_navigation_menu_story_model()
            ),
            story_fixture!(
                NavigationMenu,
                "loading_navigation_menu",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_navigation_menu_story_model()
            ),
            story_fixture!(
                NavigationMenu,
                "disabled_navigation_menu",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_navigation_menu_story_model()
            ),
            story_fixture!(
                NavigationMenu,
                "themed_navigation_menu",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_navigation_menu_story_model()
            ),
        ],
        UiComponentId::Pagination => vec![
            story_fixture!(
                Pagination,
                "default_pagination",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_pagination_story_model()
            ),
            story_fixture!(
                Pagination,
                "dense_pagination",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_pagination_story_model()
            ),
            story_fixture!(
                Pagination,
                "loading_pagination",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_pagination_story_model()
            ),
            story_fixture!(
                Pagination,
                "disabled_pagination",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_pagination_story_model()
            ),
            story_fixture!(
                Pagination,
                "themed_pagination",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_pagination_story_model()
            ),
        ],
        UiComponentId::Popover => vec![
            story_fixture!(
                Popover,
                "default_popover",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_popover_story_model()
            ),
            story_fixture!(
                Popover,
                "dense_popover",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_popover_story_model()
            ),
            story_fixture!(
                Popover,
                "loading_popover",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_popover_story_model()
            ),
            story_fixture!(
                Popover,
                "disabled_popover",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_popover_story_model()
            ),
            story_fixture!(
                Popover,
                "themed_popover",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_popover_story_model()
            ),
        ],
        UiComponentId::Progress => vec![
            story_fixture!(
                Progress,
                "default_progress",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_progress_story_model()
            ),
            story_fixture!(
                Progress,
                "dense_progress",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_progress_story_model()
            ),
            story_fixture!(
                Progress,
                "loading_progress",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_progress_story_model()
            ),
            story_fixture!(
                Progress,
                "disabled_progress",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_progress_story_model()
            ),
            story_fixture!(
                Progress,
                "indeterminate_progress",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                indeterminate_progress_story_model()
            ),
            story_fixture!(
                Progress,
                "themed_progress",
                UiStoryVariantKind::Themed,
                ThemeId::Catppuccin,
                themed_progress_story_model()
            ),
        ],
        UiComponentId::RadioGroup => vec![
            story_fixture!(
                RadioGroup,
                "default_radio_group",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_radio_group_story_model()
            ),
            story_fixture!(
                RadioGroup,
                "dense_radio_group",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_radio_group_story_model()
            ),
            story_fixture!(
                RadioGroup,
                "loading_radio_group",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_radio_group_story_model()
            ),
            story_fixture!(
                RadioGroup,
                "disabled_radio_group",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_radio_group_story_model()
            ),
            story_fixture!(
                RadioGroup,
                "themed_radio_group",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_radio_group_story_model()
            ),
        ],
        UiComponentId::Resizable => vec![
            story_fixture!(
                Resizable,
                "default_resizable",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_resizable_story_model()
            ),
            story_fixture!(
                Resizable,
                "dense_resizable",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_resizable_story_model()
            ),
            story_fixture!(
                Resizable,
                "vertical_resizable",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                vertical_resizable_story_model()
            ),
            story_fixture!(
                Resizable,
                "loading_resizable",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_resizable_story_model()
            ),
            story_fixture!(
                Resizable,
                "disabled_resizable",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_resizable_story_model()
            ),
            story_fixture!(
                Resizable,
                "themed_resizable",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_resizable_story_model()
            ),
        ],
        UiComponentId::ScrollArea => vec![
            story_fixture!(
                ScrollArea,
                "default_scroll_area",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_scroll_area_story_model()
            ),
            story_fixture!(
                ScrollArea,
                "dense_scroll_area",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_scroll_area_story_model()
            ),
            story_fixture!(
                ScrollArea,
                "horizontal_scroll_area",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                horizontal_scroll_area_story_model()
            ),
            story_fixture!(
                ScrollArea,
                "loading_scroll_area",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_scroll_area_story_model()
            ),
            story_fixture!(
                ScrollArea,
                "disabled_scroll_area",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_scroll_area_story_model()
            ),
            story_fixture!(
                ScrollArea,
                "themed_scroll_area",
                UiStoryVariantKind::Themed,
                ThemeId::Lofi,
                themed_scroll_area_story_model()
            ),
        ],
        UiComponentId::Select => vec![
            story_fixture!(
                Select,
                "default_select",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_select_story_model()
            ),
            story_fixture!(
                Select,
                "dense_select",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_select_story_model()
            ),
            story_fixture!(
                Select,
                "loading_select",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_select_story_model()
            ),
            story_fixture!(
                Select,
                "disabled_select",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_select_story_model()
            ),
            story_fixture!(
                Select,
                "themed_select",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_select_story_model()
            ),
        ],
        UiComponentId::Separator => vec![
            story_fixture!(
                Separator,
                "default_separator",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_separator_story_model()
            ),
            story_fixture!(
                Separator,
                "dense_separator",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_separator_story_model()
            ),
            story_fixture!(
                Separator,
                "vertical_separator",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                vertical_separator_story_model()
            ),
            story_fixture!(
                Separator,
                "loading_separator",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_separator_story_model()
            ),
            story_fixture!(
                Separator,
                "disabled_separator",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_separator_story_model()
            ),
            story_fixture!(
                Separator,
                "themed_separator",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_separator_story_model()
            ),
        ],
        UiComponentId::Sheet => vec![
            story_fixture!(
                Sheet,
                "default_sheet",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_sheet_story_model()
            ),
            story_fixture!(
                Sheet,
                "dense_sheet",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_sheet_story_model()
            ),
            story_fixture!(
                Sheet,
                "left_sheet",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                left_sheet_story_model()
            ),
            story_fixture!(
                Sheet,
                "loading_sheet",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_sheet_story_model()
            ),
            story_fixture!(
                Sheet,
                "disabled_sheet",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_sheet_story_model()
            ),
            story_fixture!(
                Sheet,
                "themed_sheet",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_sheet_story_model()
            ),
        ],
        UiComponentId::Sidebar => vec![
            story_fixture!(
                Sidebar,
                "default_sidebar",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_sidebar_story_model()
            ),
            story_fixture!(
                Sidebar,
                "dense_sidebar",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_sidebar_story_model()
            ),
            story_fixture!(
                Sidebar,
                "collapsed_sidebar",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                collapsed_sidebar_story_model()
            ),
            story_fixture!(
                Sidebar,
                "loading_sidebar",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_sidebar_story_model()
            ),
            story_fixture!(
                Sidebar,
                "disabled_sidebar",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_sidebar_story_model()
            ),
            story_fixture!(
                Sidebar,
                "themed_sidebar",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_sidebar_story_model()
            ),
        ],
        UiComponentId::Skeleton => vec![
            story_fixture!(
                Skeleton,
                "default_skeleton",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_skeleton_story_model()
            ),
            story_fixture!(
                Skeleton,
                "dense_skeleton",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_skeleton_story_model()
            ),
            story_fixture!(
                Skeleton,
                "ready_skeleton",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                ready_skeleton_story_model()
            ),
            story_fixture!(
                Skeleton,
                "static_skeleton",
                UiStoryVariantKind::Alternate(3),
                ThemeId::Dark,
                static_skeleton_story_model()
            ),
            story_fixture!(
                Skeleton,
                "disabled_skeleton",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_skeleton_story_model()
            ),
            story_fixture!(
                Skeleton,
                "themed_skeleton",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_skeleton_story_model()
            ),
        ],
        UiComponentId::Slider => vec![
            story_fixture!(
                Slider,
                "default_slider",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_slider_story_model()
            ),
            story_fixture!(
                Slider,
                "dense_slider",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_slider_story_model()
            ),
            story_fixture!(
                Slider,
                "vertical_slider",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                vertical_slider_story_model()
            ),
            story_fixture!(
                Slider,
                "loading_slider",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_slider_story_model()
            ),
            story_fixture!(
                Slider,
                "disabled_slider",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_slider_story_model()
            ),
            story_fixture!(
                Slider,
                "themed_slider",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_slider_story_model()
            ),
        ],
        UiComponentId::Sonner => vec![
            story_fixture!(
                Sonner,
                "default_sonner",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_sonner_story_model()
            ),
            story_fixture!(
                Sonner,
                "dense_sonner",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_sonner_story_model()
            ),
            story_fixture!(
                Sonner,
                "centered_sonner",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                centered_sonner_story_model()
            ),
            story_fixture!(
                Sonner,
                "loading_sonner",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_sonner_story_model()
            ),
            story_fixture!(
                Sonner,
                "disabled_sonner",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_sonner_story_model()
            ),
            story_fixture!(
                Sonner,
                "themed_sonner",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_sonner_story_model()
            ),
        ],
        UiComponentId::Spinner => vec![
            story_fixture!(
                Spinner,
                "default_spinner",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_spinner_story_model()
            ),
            story_fixture!(
                Spinner,
                "dense_spinner",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_spinner_story_model()
            ),
            story_fixture!(
                Spinner,
                "large_spinner",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                large_spinner_story_model()
            ),
            story_fixture!(
                Spinner,
                "ready_spinner",
                UiStoryVariantKind::Alternate(3),
                ThemeId::Dark,
                ready_spinner_story_model()
            ),
            story_fixture!(
                Spinner,
                "disabled_spinner",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_spinner_story_model()
            ),
            story_fixture!(
                Spinner,
                "themed_spinner",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_spinner_story_model()
            ),
        ],
        UiComponentId::Switch => vec![
            story_fixture!(
                Switch,
                "default_switch",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_switch_story_model()
            ),
            story_fixture!(
                Switch,
                "dense_switch",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_switch_story_model()
            ),
            story_fixture!(
                Switch,
                "off_switch",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                off_switch_story_model()
            ),
            story_fixture!(
                Switch,
                "loading_switch",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_switch_story_model()
            ),
            story_fixture!(
                Switch,
                "disabled_switch",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_switch_story_model()
            ),
            story_fixture!(
                Switch,
                "themed_switch",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_switch_story_model()
            ),
        ],
        UiComponentId::Toggle => vec![
            story_fixture!(
                Toggle,
                "default_toggle",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_toggle_story_model()
            ),
            story_fixture!(
                Toggle,
                "dense_toggle",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_toggle_story_model()
            ),
            story_fixture!(
                Toggle,
                "outline_toggle",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                outline_toggle_story_model()
            ),
            story_fixture!(
                Toggle,
                "loading_toggle",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_toggle_story_model()
            ),
            story_fixture!(
                Toggle,
                "disabled_toggle",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_toggle_story_model()
            ),
            story_fixture!(
                Toggle,
                "themed_toggle",
                UiStoryVariantKind::Themed,
                ThemeId::Cyberpunk,
                themed_toggle_story_model()
            ),
        ],
        UiComponentId::ToggleGroup => vec![
            story_fixture!(
                ToggleGroup,
                "default_toggle_group",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_toggle_group_story_model()
            ),
            story_fixture!(
                ToggleGroup,
                "multiple_toggle_group",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                multiple_toggle_group_story_model()
            ),
            story_fixture!(
                ToggleGroup,
                "dense_toggle_group",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                dense_toggle_group_story_model()
            ),
            story_fixture!(
                ToggleGroup,
                "vertical_toggle_group",
                UiStoryVariantKind::Alternate(3),
                ThemeId::Dark,
                vertical_toggle_group_story_model()
            ),
            story_fixture!(
                ToggleGroup,
                "loading_toggle_group",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_toggle_group_story_model()
            ),
            story_fixture!(
                ToggleGroup,
                "disabled_toggle_group",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_toggle_group_story_model()
            ),
            story_fixture!(
                ToggleGroup,
                "themed_toggle_group",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_toggle_group_story_model()
            ),
        ],
        UiComponentId::Tooltip => vec![
            story_fixture!(
                Tooltip,
                "default_tooltip",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_tooltip_story_model()
            ),
            story_fixture!(
                Tooltip,
                "dense_tooltip",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_tooltip_story_model()
            ),
            story_fixture!(
                Tooltip,
                "placement_tooltip",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                placement_tooltip_story_model()
            ),
            story_fixture!(
                Tooltip,
                "loading_tooltip",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_tooltip_story_model()
            ),
            story_fixture!(
                Tooltip,
                "disabled_tooltip",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_tooltip_story_model()
            ),
            story_fixture!(
                Tooltip,
                "hidden_arrow_tooltip",
                UiStoryVariantKind::Alternate(3),
                ThemeId::Dark,
                hidden_arrow_tooltip_story_model()
            ),
            story_fixture!(
                Tooltip,
                "themed_tooltip",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_tooltip_story_model()
            ),
        ],
        UiComponentId::Table => vec![
            story_fixture!(
                Table,
                "default_table",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_table_story_model()
            ),
            story_fixture!(
                Table,
                "dense_table",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_table_story_model()
            ),
            story_fixture!(
                Table,
                "selected_table",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                selected_table_story_model()
            ),
            story_fixture!(
                Table,
                "loading_table",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_table_story_model()
            ),
            story_fixture!(
                Table,
                "disabled_table",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_table_story_model()
            ),
            story_fixture!(
                Table,
                "themed_table",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_table_story_model()
            ),
        ],
        UiComponentId::Tabs => vec![
            story_fixture!(
                Tabs,
                "default_tabs",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_tabs_story_model()
            ),
            story_fixture!(
                Tabs,
                "dense_tabs",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_tabs_story_model()
            ),
            story_fixture!(
                Tabs,
                "vertical_tabs",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                vertical_tabs_story_model()
            ),
            story_fixture!(
                Tabs,
                "loading_tabs",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_tabs_story_model()
            ),
            story_fixture!(
                Tabs,
                "disabled_tabs",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_tabs_story_model()
            ),
            story_fixture!(
                Tabs,
                "themed_tabs",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_tabs_story_model()
            ),
        ],
        UiComponentId::Textarea => vec![
            story_fixture!(
                Textarea,
                "default_textarea",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_textarea_story_model()
            ),
            story_fixture!(
                Textarea,
                "dense_textarea",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_textarea_story_model()
            ),
            story_fixture!(
                Textarea,
                "unlimited_textarea",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                unlimited_textarea_story_model()
            ),
            story_fixture!(
                Textarea,
                "loading_textarea",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_textarea_story_model()
            ),
            story_fixture!(
                Textarea,
                "disabled_textarea",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_textarea_story_model()
            ),
            story_fixture!(
                Textarea,
                "themed_textarea",
                UiStoryVariantKind::Themed,
                ThemeId::Luxury,
                themed_textarea_story_model()
            ),
        ],
        UiComponentId::Toast => vec![
            story_fixture!(
                Toast,
                "default_toast",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_toast_story_model()
            ),
            story_fixture!(
                Toast,
                "dense_toast",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_toast_story_model()
            ),
            story_fixture!(
                Toast,
                "centered_toast",
                UiStoryVariantKind::Alternate(2),
                ThemeId::Dark,
                centered_toast_story_model()
            ),
            story_fixture!(
                Toast,
                "loading_toast",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_toast_story_model()
            ),
            story_fixture!(
                Toast,
                "disabled_toast",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_toast_story_model()
            ),
            story_fixture!(
                Toast,
                "themed_toast",
                UiStoryVariantKind::Themed,
                ThemeId::Dracula,
                themed_toast_story_model()
            ),
        ],
        UiComponentId::Typography => vec![
            story_fixture!(
                Typography,
                "default_typography",
                UiStoryVariantKind::Default,
                ThemeId::Dark,
                default_typography_story_model()
            ),
            story_fixture!(
                Typography,
                "dense_typography",
                UiStoryVariantKind::Alternate(1),
                ThemeId::Dark,
                dense_typography_story_model()
            ),
            story_fixture!(
                Typography,
                "loading_typography",
                UiStoryVariantKind::Loading,
                ThemeId::Dark,
                loading_typography_story_model()
            ),
            story_fixture!(
                Typography,
                "disabled_typography",
                UiStoryVariantKind::Disabled,
                ThemeId::Dark,
                disabled_typography_story_model()
            ),
            story_fixture!(
                Typography,
                "themed_typography",
                UiStoryVariantKind::Themed,
                ThemeId::Catppuccin,
                themed_typography_story_model()
            ),
        ],
    }
}

pub fn accordion_story_items() -> Vec<AccordionItem> {
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

pub fn default_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Build completed",
        "The design-token bundle is ready for review in the shared UI crate.",
    )
    .with_tone(AlertTone::Success)
    .with_action(AlertAction::new("Open report", "open-report"))
}

pub fn dense_warning_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Token drift detected",
        "Two stories reference the same semantic tone through different framework paths.",
    )
    .with_tone(AlertTone::Warning)
    .with_density(AlertDensity::Dense)
    .with_action(AlertAction::new("Review", "review-token-drift"))
}

pub fn loading_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Publishing artifacts",
        "The static Pages bundle is still being generated by the one-pass gate.",
    )
    .with_tone(AlertTone::Info)
    .with_action(AlertAction::new("View run", "view-run"))
    .loading()
}

pub fn disabled_error_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Destructive action blocked",
        "The app requires a durable state handoff before this action can be enabled.",
    )
    .with_tone(AlertTone::Destructive)
    .with_action(AlertAction::new("Retry", "retry-action").disabled())
    .disabled()
}

pub fn themed_alert_story_model() -> AlertModel {
    AlertModel::new(
        "Theme scoped alert",
        "The same semantic tokens resolve through a nested Dracula theme scope.",
    )
    .with_tone(AlertTone::Default)
    .with_action(AlertAction::new("Inspect", "inspect-theme"))
}

pub fn default_alert_dialog_story_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "Archive deck",
        "Archive this deck?",
        "The deck will be hidden from the active queue until a durable state restore reactivates it.",
        AlertDialogButton::new("Archive", "archive-deck"),
        AlertDialogButton::new("Cancel", "cancel-archive"),
    )
}

pub fn open_destructive_alert_dialog_story_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "Delete draft",
        "Delete this draft?",
        "This cannot be undone. The draft and its local review state will be removed.",
        AlertDialogButton::new("Delete", "delete-draft"),
        AlertDialogButton::new("Cancel", "cancel-delete"),
    )
    .destructive()
}

pub fn small_loading_alert_dialog_story_model() -> AlertDialogModel {
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

pub fn disabled_alert_dialog_story_model() -> AlertDialogModel {
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

pub fn themed_alert_dialog_story_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "Share project",
        "Share this project?",
        "The confirmation surface resolves through the nested Luxury theme scope.",
        AlertDialogButton::new("Share", "share-project"),
        AlertDialogButton::new("Cancel", "cancel-share"),
    )
    .with_size(AlertDialogSize::Small)
}

pub fn default_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Course preview",
        "A stable 16:9 frame for images, canvases, and embedded lesson media.",
    )
}

pub fn contain_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Avatar crop",
        "A square frame that preserves the full media bounds with contain behavior.",
    )
    .with_ratio(1, 1)
    .with_fit(AspectRatioFit::Contain)
}

pub fn loading_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Generating preview",
        "Loading keeps the frame height stable while media data resolves.",
    )
    .with_ratio(4, 3)
    .loading()
}

pub fn disabled_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Locked recording",
        "Disabled media keeps its frame but removes interactive affordance.",
    )
    .with_ratio(21, 9)
    .disabled()
}

pub fn themed_aspect_ratio_story_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Theme scoped media",
        "The same semantic tokens resolve through a nested Cyberpunk theme scope.",
    )
    .with_ratio(3, 2)
    .with_fit(AspectRatioFit::Cover)
}

pub fn default_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("roadmap-notes.pdf", "2.4 MB")
        .with_action(AttachmentAction::new("Download", "download-roadmap"))
}

pub fn image_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("lesson-cover.png", "842 KB")
        .with_kind(AttachmentKind::Image)
        .with_action(AttachmentAction::new("Open", "open-lesson-cover"))
}

pub fn loading_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("uploading-transcript.txt", "Preparing")
        .with_kind(AttachmentKind::Data)
        .with_action(AttachmentAction::new("Open", "open-transcript"))
        .loading()
}

pub fn disabled_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("locked-export.zip", "12.8 MB")
        .with_kind(AttachmentKind::Archive)
        .with_action(AttachmentAction::new("Download", "download-export").disabled())
        .disabled()
}

pub fn themed_attachment_story_model() -> AttachmentModel {
    AttachmentModel::new("theme-audit.csv", "18 rows")
        .with_kind(AttachmentKind::Data)
        .with_preview_label("CSV")
        .with_action(AttachmentAction::new("Inspect", "inspect-theme-audit"))
}

pub fn default_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Matthew Harwood", "MH")
}

pub fn fallback_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Design System", "DS")
        .with_size(AvatarSize::Small)
        .without_image()
}

pub fn loading_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Hydrating User", "HU").loading()
}

pub fn disabled_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Locked User", "LU").disabled()
}

pub fn themed_avatar_story_model() -> AvatarModel {
    AvatarModel::new("Theme Scope", "TS").with_size(AvatarSize::Large)
}

pub fn default_badge_story_model() -> BadgeModel {
    BadgeModel::new("Ready")
}

pub fn no_icon_badge_story_model() -> BadgeModel {
    BadgeModel::new("Beta")
        .with_tone(BadgeTone::Info)
        .with_size(BadgeSize::Small)
        .without_icon()
}

pub fn loading_badge_story_model() -> BadgeModel {
    BadgeModel::new("Syncing")
        .with_tone(BadgeTone::Brand)
        .loading()
}

pub fn disabled_badge_story_model() -> BadgeModel {
    BadgeModel::new("Paused")
        .with_tone(BadgeTone::Muted)
        .with_variant(BadgeVariant::Outline)
        .disabled()
}

pub fn themed_badge_story_model() -> BadgeModel {
    BadgeModel::new("Critical")
        .with_tone(BadgeTone::Destructive)
        .with_variant(BadgeVariant::Solid)
        .with_icon("High")
}

pub fn default_breadcrumb_story_model() -> BreadcrumbModel {
    BreadcrumbModel::new(vec![
        BreadcrumbEntry::link("Library", "#library"),
        BreadcrumbEntry::link("Components", "#components"),
        BreadcrumbEntry::page("Breadcrumb"),
    ])
}

pub fn dense_breadcrumb_story_model() -> BreadcrumbModel {
    BreadcrumbModel::new(vec![
        BreadcrumbEntry::link("Docs", "#docs"),
        BreadcrumbEntry::link("UI", "#ui"),
        BreadcrumbEntry::page("Tokens"),
    ])
    .with_density(BreadcrumbDensity::Dense)
}

pub fn loading_breadcrumb_story_model() -> BreadcrumbModel {
    default_breadcrumb_story_model().loading()
}

pub fn disabled_breadcrumb_story_model() -> BreadcrumbModel {
    BreadcrumbModel::new(vec![
        BreadcrumbEntry::link("Workspace", "#workspace"),
        BreadcrumbEntry::link("Project", "#project").disabled(),
        BreadcrumbEntry::page("Locked route"),
    ])
    .disabled()
}

pub fn themed_breadcrumb_story_model() -> BreadcrumbModel {
    BreadcrumbModel::new(vec![
        BreadcrumbEntry::link("Theme", "#theme"),
        BreadcrumbEntry::link("Palette", "#palette"),
        BreadcrumbEntry::page("Catppuccin"),
    ])
    .with_separator(">")
}

pub fn default_bubble_story_model() -> BubbleModel {
    BubbleModel::new(
        "Codex",
        "AI",
        "The sweep is ready for review.",
        "Delivered now",
    )
}

pub fn outgoing_bubble_story_model() -> BubbleModel {
    BubbleModel::new(
        "Matthew",
        "MH",
        "Ship the next component when the gate is green.",
        "Sent now",
    )
    .with_side(BubbleSide::Outgoing)
    .with_actions(vec![BubbleAction::new("Edit", "edit-message")])
}

pub fn loading_bubble_story_model() -> BubbleModel {
    BubbleModel::new("Codex", "AI", "Hydrating response", "Pending").loading()
}

pub fn disabled_bubble_story_model() -> BubbleModel {
    BubbleModel::new(
        "Archive",
        "AR",
        "This message is locked by the transcript.",
        "Read only",
    )
    .with_actions(vec![BubbleAction::new("Reply", "reply").disabled()])
    .disabled()
}

pub fn themed_bubble_story_model() -> BubbleModel {
    BubbleModel::new("System", "SYS", "Theme-scoped audit note.", "Pinned")
        .with_side(BubbleSide::System)
        .with_actions(vec![BubbleAction::new("Resolve", "resolve-note")])
}

pub fn default_button_story_model() -> ButtonModel {
    ButtonModel::new("Continue", "continue")
}

pub fn secondary_button_story_model() -> ButtonModel {
    ButtonModel::new("Preview", "preview")
        .with_variant(ButtonVariant::Secondary)
        .with_size(ButtonSize::Small)
        .without_icon()
}

pub fn link_button_story_model() -> ButtonModel {
    ButtonModel::new("Open docs", "open-docs")
        .with_variant(ButtonVariant::Link)
        .as_link("#docs")
}

pub fn loading_button_story_model() -> ButtonModel {
    ButtonModel::new("Saving", "save").loading()
}

pub fn disabled_button_story_model() -> ButtonModel {
    ButtonModel::new("Locked", "locked")
        .with_kind(ButtonKind::Submit)
        .with_variant(ButtonVariant::Outline)
        .disabled()
}

pub fn themed_button_story_model() -> ButtonModel {
    ButtonModel::new("Delete", "delete")
        .with_variant(ButtonVariant::Destructive)
        .with_size(ButtonSize::Large)
        .with_icon("Del")
}

pub fn default_button_group_story_model() -> ButtonGroupModel {
    ButtonGroupModel::new(vec![
        ButtonGroupItem::new("Day", "day").with_icon("D"),
        ButtonGroupItem::new("Week", "week").with_icon("W"),
        ButtonGroupItem::new("Month", "month").with_icon("M"),
    ])
    .with_selected("week")
}

pub fn vertical_button_group_story_model() -> ButtonGroupModel {
    ButtonGroupModel::new(vec![
        ButtonGroupItem::new("Draft", "draft"),
        ButtonGroupItem::new("Review", "review"),
        ButtonGroupItem::new("Publish", "publish"),
    ])
    .with_orientation(ButtonGroupOrientation::Vertical)
    .with_variant(ButtonVariant::Outline)
    .with_selected("review")
}

pub fn loading_button_group_story_model() -> ButtonGroupModel {
    default_button_group_story_model().loading()
}

pub fn disabled_button_group_story_model() -> ButtonGroupModel {
    ButtonGroupModel::new(vec![
        ButtonGroupItem::new("A", "a").with_icon("A"),
        ButtonGroupItem::new("B", "b").with_icon("B").disabled(),
        ButtonGroupItem::new("C", "c").with_icon("C"),
    ])
    .with_size(ButtonSize::Icon)
    .with_selected("a")
    .disabled()
}

pub fn themed_button_group_story_model() -> ButtonGroupModel {
    ButtonGroupModel::new(vec![
        ButtonGroupItem::new("Left", "left").with_icon("L"),
        ButtonGroupItem::new("Center", "center").with_icon("C"),
        ButtonGroupItem::new("Right", "right").with_icon("R"),
    ])
    .with_variant(ButtonVariant::Primary)
    .with_size(ButtonSize::Small)
    .with_selected("center")
}

pub fn default_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 7).with_selected(CalendarDate::new(2026, 7, 7))
}

pub fn range_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 7)
        .with_mode(CalendarSelectionMode::Range)
        .with_range(CalendarRange::new(
            CalendarDate::new(2026, 7, 6),
            CalendarDate::new(2026, 7, 10),
        ))
}

pub fn loading_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 8)
        .with_selected(CalendarDate::new(2026, 8, 14))
        .loading()
}

pub fn disabled_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 9)
        .with_selected(CalendarDate::new(2026, 9, 21))
        .disabled()
}

pub fn themed_calendar_story_model() -> CalendarModel {
    CalendarModel::new(2026, 10)
        .with_mode(CalendarSelectionMode::Range)
        .with_range(CalendarRange::new(
            CalendarDate::new(2026, 10, 12),
            CalendarDate::new(2026, 10, 16),
        ))
}

pub fn default_card_story_model() -> CardModel {
    CardModel::new(
        "Design system",
        "64 components share one token contract.",
        "Implementation notes stay portable across Leptos DOM and Bevy scene primitives.",
        "Sweep process ready",
    )
    .with_action(CardAction::new("Open checklist", "open-checklist"))
}

pub fn dense_card_story_model() -> CardModel {
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

pub fn loading_card_story_model() -> CardModel {
    CardModel::new(
        "Publishing artifact",
        "The card keeps layout stable while the action is blocked.",
        "The one-pass gate is still building Pages and story artifacts.",
        "Waiting on CI",
    )
    .with_action(CardAction::new("View run", "view-run"))
    .loading()
}

pub fn disabled_card_story_model() -> CardModel {
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

pub fn themed_card_story_model() -> CardModel {
    CardModel::new(
        "Theme scoped card",
        "The same semantic card tokens resolve through a nested Synthwave theme scope.",
        "Variant, density, text, border, and action state stay token-driven.",
        "Theme preview",
    )
    .with_variant(CardVariant::Elevated)
    .with_action(CardAction::new("Inspect", "inspect-themed-card"))
}

pub fn default_carousel_story_model() -> CarouselModel {
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

pub fn dense_carousel_story_model() -> CarouselModel {
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

pub fn loading_carousel_story_model() -> CarouselModel {
    default_carousel_story_model().loading()
}

pub fn disabled_carousel_story_model() -> CarouselModel {
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

pub fn themed_carousel_story_model() -> CarouselModel {
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

pub fn default_chart_story_model() -> ChartModel {
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

pub fn dense_chart_story_model() -> ChartModel {
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

pub fn loading_chart_story_model() -> ChartModel {
    default_chart_story_model().loading()
}

pub fn disabled_chart_story_model() -> ChartModel {
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

pub fn themed_chart_story_model() -> ChartModel {
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

pub fn default_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Use shared theme tokens", "shared-theme")
        .with_description("Leptos and Bevy read the same semantic checkbox state.")
        .checked()
}

pub fn dense_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Compact preference", "compact-preference")
        .with_density(CheckboxDensity::Dense)
        .with_description("Dense layout keeps the same state contract.")
}

pub fn loading_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Syncing setting", "syncing-setting")
        .with_description("The renderer blocks local toggles while hydration is pending.")
        .loading()
}

pub fn disabled_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Locked rollout", "locked-rollout")
        .with_description("Disabled checkboxes preserve the shared checked value.")
        .with_checked(CheckboxChecked::Indeterminate)
        .disabled()
}

pub fn themed_checkbox_story_model() -> CheckboxModel {
    CheckboxModel::new("Theme scoped checkbox", "theme-scoped-checkbox")
        .with_description("The same control resolves through a nested Forest theme scope.")
        .indeterminate()
}

pub fn default_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "token-stack",
        "Shared token stack",
        "A single disclosure region can expose implementation notes without creating durable state.",
    )
    .open()
}

pub fn dense_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "compact-disclosure",
        "Compact disclosure",
        "Dense mode keeps the same trigger and content anatomy with tighter token spacing.",
    )
    .with_density(CollapsibleDensity::Dense)
}

pub fn loading_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "loading-disclosure",
        "Hydrating disclosure",
        "The trigger is blocked while renderer-local state hydrates from the consumer boundary.",
    )
    .loading()
}

pub fn disabled_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "locked-disclosure",
        "Locked disclosure",
        "Disabled disclosure keeps the content contract visible to Bevy primitives.",
    )
    .open()
    .disabled()
}

pub fn themed_collapsible_story_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "theme-disclosure",
        "Theme scoped disclosure",
        "The same semantic classes resolve through the nested Luxury theme scope.",
    )
    .open()
}

pub fn default_combobox_story_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Leptos DOM", "leptos"),
        ComboboxOption::new("Bevy scene", "bevy"),
        ComboboxOption::new("Shared state", "state"),
    ])
    .with_placeholder("Search UI surface")
    .with_selected_value("leptos")
}

pub fn dense_combobox_story_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Accordion", "accordion"),
        ComboboxOption::new("Checkbox", "checkbox"),
        ComboboxOption::new("Collapsible", "collapsible"),
    ])
    .with_density(ComboboxDensity::Dense)
    .with_placeholder("Filter component")
    .with_default_query("co")
}

pub fn loading_combobox_story_model() -> ComboboxModel {
    default_combobox_story_model().loading()
}

pub fn disabled_combobox_story_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Stable", "stable"),
        ComboboxOption::new("Blocked", "blocked").disabled(),
    ])
    .with_selected_value("stable")
    .disabled()
}

pub fn themed_combobox_story_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Brand", "brand"),
        ComboboxOption::new("Success", "success"),
        ComboboxOption::new("Danger", "danger"),
    ])
    .with_placeholder("Search tone")
    .with_selected_value("success")
}

pub fn default_command_story_model() -> CommandModel {
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

pub fn dense_command_story_model() -> CommandModel {
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

pub fn loading_command_story_model() -> CommandModel {
    default_command_story_model().loading()
}

pub fn disabled_command_story_model() -> CommandModel {
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

pub fn themed_command_story_model() -> CommandModel {
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

pub fn default_context_menu_story_model() -> ContextMenuModel {
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

pub fn dense_context_menu_story_model() -> ContextMenuModel {
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

pub fn loading_context_menu_story_model() -> ContextMenuModel {
    default_context_menu_story_model().loading()
}

pub fn disabled_context_menu_story_model() -> ContextMenuModel {
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

pub fn themed_context_menu_story_model() -> ContextMenuModel {
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

pub fn data_table_story_columns() -> Vec<DataTableColumn> {
    vec![
        DataTableColumn::new("Component", "component"),
        DataTableColumn::new("Surface", "surface"),
        DataTableColumn::new("Health", "health").numeric(),
    ]
}

pub fn data_table_story_rows() -> Vec<DataTableRow> {
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

pub fn default_data_table_story_model() -> DataTableModel {
    DataTableModel::new(data_table_story_columns(), data_table_story_rows())
        .with_title("Component health")
        .with_filter_placeholder("Filter components")
        .with_sort("component", DataTableSortDirection::Ascending)
        .with_selected_row("command")
        .with_page_size(3)
}

pub fn dense_data_table_story_model() -> DataTableModel {
    DataTableModel::new(data_table_story_columns(), data_table_story_rows())
        .with_density(DataTableDensity::Dense)
        .with_title("Dense health")
        .with_filter_placeholder("Filter dense rows")
        .with_sort("health", DataTableSortDirection::Descending)
        .with_default_filter("co")
        .with_page_size(2)
}

pub fn loading_data_table_story_model() -> DataTableModel {
    default_data_table_story_model().loading()
}

pub fn disabled_data_table_story_model() -> DataTableModel {
    DataTableModel::new(data_table_story_columns(), data_table_story_rows())
        .with_title("Disabled table")
        .with_selected_row("accordion")
        .disabled()
}

pub fn themed_data_table_story_model() -> DataTableModel {
    DataTableModel::new(data_table_story_columns(), data_table_story_rows())
        .with_title("Theme scoped table")
        .with_filter_placeholder("Filter theme rows")
        .with_sort("surface", DataTableSortDirection::Ascending)
        .with_selected_row("data-table")
        .with_page_size(4)
}

pub fn default_date_picker_story_model() -> DatePickerModel {
    DatePickerModel::new(2026, 7)
        .with_label("Release date")
        .with_placeholder("Choose release date")
        .with_selected(CalendarDate::new(2026, 7, 7))
}

pub fn dense_open_date_picker_story_model() -> DatePickerModel {
    DatePickerModel::new(2026, 8)
        .with_density(DatePickerDensity::Dense)
        .with_label("Dense picker")
        .with_placeholder("Pick day")
        .with_default_open(true)
}

pub fn loading_date_picker_story_model() -> DatePickerModel {
    default_date_picker_story_model()
        .with_default_open(true)
        .loading()
}

pub fn disabled_date_picker_story_model() -> DatePickerModel {
    DatePickerModel::new(2026, 9)
        .with_label("Locked date")
        .with_placeholder("Unavailable")
        .with_selected(CalendarDate::new(2026, 9, 15))
        .disabled()
}

pub fn themed_date_picker_story_model() -> DatePickerModel {
    DatePickerModel::new(2026, 10)
        .with_label("Theme scoped date")
        .with_placeholder("Choose themed date")
        .with_selected(CalendarDate::new(2026, 10, 21))
        .with_default_open(true)
}

pub fn default_dialog_story_model() -> DialogModel {
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

pub fn open_small_dialog_story_model() -> DialogModel {
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

pub fn loading_dialog_story_model() -> DialogModel {
    default_dialog_story_model()
        .with_default_open(true)
        .loading()
}

pub fn disabled_dialog_story_model() -> DialogModel {
    DialogModel::new(
        "Locked dialog",
        "Locked workflow",
        "This dialog is disabled.",
        "The trigger and footer actions are disabled through the shared model.",
    )
    .with_default_open(true)
    .disabled()
}

pub fn themed_dialog_story_model() -> DialogModel {
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

pub fn default_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "Application direction",
        "RTL article scope",
        "The default provider starts left-to-right while the nested scope can opt into right-to-left flow.",
    )
}

pub fn rtl_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "Arabic locale",
        "LTR code sample",
        "مرحبا بكم في واجهة تدعم اتجاه النص من اليمين إلى اليسار.",
    )
    .with_direction(DirectionValue::Rtl)
    .with_scope_direction(DirectionValue::Ltr)
}

pub fn scoped_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "LTR shell",
        "RTL preview pane",
        "A nested scope can override the provider direction without rebuilding the component tree.",
    )
    .with_default_scope_active(true)
}

pub fn loading_direction_story_model() -> DirectionModel {
    default_direction_story_model().loading()
}

pub fn disabled_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "Locked locale",
        "Locked scope",
        "Direction controls are disabled while app-level locale hydration is unavailable.",
    )
    .with_direction(DirectionValue::Rtl)
    .with_default_scope_active(true)
    .disabled()
}

pub fn themed_direction_story_model() -> DirectionModel {
    DirectionModel::new(
        "Theme direction",
        "Theme RTL scope",
        "Semantic tokens and direction attributes stay independent so theme switching does not break RTL rendering.",
    )
    .with_default_scope_active(true)
}

pub fn default_drawer_story_model() -> DrawerModel {
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

pub fn right_drawer_story_model() -> DrawerModel {
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

pub fn loading_drawer_story_model() -> DrawerModel {
    default_drawer_story_model().loading()
}

pub fn disabled_drawer_story_model() -> DrawerModel {
    DrawerModel::new(
        "Locked drawer",
        "Locked workflow",
        "The drawer is disabled while durable app state hydrates.",
        "Trigger, handle, and footer actions are disabled through the shared model.",
    )
    .with_default_open(true)
    .disabled()
}

pub fn themed_drawer_story_model() -> DrawerModel {
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

pub fn dropdown_menu_story_entries() -> Vec<DropdownMenuEntry> {
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

pub fn default_dropdown_menu_story_model() -> DropdownMenuModel {
    DropdownMenuModel::new(dropdown_menu_story_entries())
        .with_trigger_label("Open actions")
        .with_selected_value("rename")
        .with_active_value("duplicate")
}

pub fn dense_dropdown_menu_story_model() -> DropdownMenuModel {
    DropdownMenuModel::new(dropdown_menu_story_entries())
        .with_density(DropdownMenuDensity::Dense)
        .with_trigger_label("Dense actions")
        .with_selected_value("duplicate")
}

pub fn loading_dropdown_menu_story_model() -> DropdownMenuModel {
    default_dropdown_menu_story_model().loading()
}

pub fn disabled_dropdown_menu_story_model() -> DropdownMenuModel {
    DropdownMenuModel::new(dropdown_menu_story_entries())
        .with_trigger_label("Locked actions")
        .disabled()
}

pub fn themed_dropdown_menu_story_model() -> DropdownMenuModel {
    DropdownMenuModel::new(dropdown_menu_story_entries())
        .with_trigger_label("Theme actions")
        .with_content_label("Theme menu actions")
        .with_selected_value("delete")
}

pub fn default_empty_story_model() -> EmptyModel {
    EmptyModel::new(
        "No lessons queued",
        "The current filters do not match any scheduled review cards.",
    )
    .with_content("Clear the filter or create a new lesson to keep the queue moving.")
    .with_action(EmptyAction::new("Create lesson", "create-lesson"))
}

pub fn dense_empty_story_model() -> EmptyModel {
    EmptyModel::new(
        "No matches",
        "Dense empty states preserve the same anatomy with tighter spacing.",
    )
    .with_density(EmptyDensity::Dense)
    .with_illustration_label("0")
    .with_content("Try a broader search term.")
    .with_action(EmptyAction::new("Reset search", "reset-search"))
}

pub fn loading_empty_story_model() -> EmptyModel {
    default_empty_story_model().loading()
}

pub fn disabled_empty_story_model() -> EmptyModel {
    EmptyModel::new(
        "State locked",
        "The empty-state action is unavailable until durable state hydrates.",
    )
    .with_content("The copy remains visible while action affordances are disabled.")
    .with_action(EmptyAction::new("Create", "create-locked").disabled())
    .disabled()
}

pub fn themed_empty_story_model() -> EmptyModel {
    EmptyModel::new(
        "Theme scoped empty",
        "The same semantic empty-state tokens resolve through a nested Forest theme.",
    )
    .with_illustration_label("UI")
    .with_content("Leptos DOM and Bevy primitives consume the same Empty render nodes.")
    .with_action(EmptyAction::new("Inspect", "inspect-empty-theme"))
}

pub fn default_field_story_model() -> FieldModel {
    FieldModel::new(
        "Project name",
        "This value stays renderer-local until a consumer persists form state.",
    )
    .with_placeholder("rs-dean-ui")
    .with_value("rs-dean")
    .required()
}

pub fn dense_field_story_model() -> FieldModel {
    FieldModel::new(
        "Search",
        "Dense field spacing keeps label, control, hint, and error anatomy stable.",
    )
    .with_density(FieldDensity::Dense)
    .with_input_kind(FieldInputKind::Search)
    .with_placeholder("Search components")
}

pub fn loading_field_story_model() -> FieldModel {
    default_field_story_model().loading()
}

pub fn disabled_field_story_model() -> FieldModel {
    FieldModel::new(
        "Locked slug",
        "Disabled fields keep copy visible while the control is unavailable.",
    )
    .with_value("stable-id")
    .disabled()
}

pub fn themed_field_story_model() -> FieldModel {
    FieldModel::new(
        "Theme token",
        "The same Field model drives Leptos control styling and Bevy primitive projection.",
    )
    .with_placeholder("surface-elevated")
    .with_value("brand")
}

pub fn default_hover_card_story_model() -> HoverCardModel {
    HoverCardModel::new(
        "Theme token",
        "Shared semantic preview",
        "The preview opens from local hover or focus state while durable app data stays outside the component.",
    )
    .with_metadata("Issue 29")
    .default_open()
}

pub fn dense_hover_card_story_model() -> HoverCardModel {
    HoverCardModel::new(
        "Dense preview",
        "Compact overlay",
        "Dense spacing keeps the trigger, content, and arrow anatomy stable across themes.",
    )
    .with_density(HoverCardDensity::Dense)
    .with_metadata("Dense")
    .default_open()
}

pub fn loading_hover_card_story_model() -> HoverCardModel {
    default_hover_card_story_model().loading()
}

pub fn disabled_hover_card_story_model() -> HoverCardModel {
    HoverCardModel::new(
        "Locked preview",
        "Unavailable content",
        "Disabled hover cards keep trigger copy visible while suppressing renderer-local open changes.",
    )
    .with_metadata("Disabled")
    .disabled()
}

pub fn themed_hover_card_story_model() -> HoverCardModel {
    HoverCardModel::new(
        "Palette preview",
        "Theme scoped overlay",
        "The same Hover Card model resolves semantic tokens through the nested Luxury theme.",
    )
    .with_metadata("Luxury")
    .with_arrow_label("Theme arrow")
    .default_open()
}

pub fn default_input_story_model() -> InputModel {
    InputModel::new("engmanager.xyz")
        .with_input_kind(InputKind::Url)
        .with_value("engmanager.xyz")
        .with_prefix("https://")
        .with_suffix(InputAction::new("Copy", "copy-url"))
        .required()
}

pub fn dense_input_story_model() -> InputModel {
    InputModel::new("Search components")
        .with_density(InputDensity::Dense)
        .with_input_kind(InputKind::Search)
        .with_prefix("UI")
        .with_suffix(InputAction::new("Go", "search-components"))
}

pub fn loading_input_story_model() -> InputModel {
    default_input_story_model().loading()
}

pub fn disabled_input_story_model() -> InputModel {
    InputModel::new("Locked value")
        .with_value("stable-id")
        .with_prefix("id:")
        .with_suffix(InputAction::new("Copy", "copy-locked").disabled())
        .disabled()
}

pub fn themed_input_story_model() -> InputModel {
    InputModel::new("theme token")
        .with_input_kind(InputKind::Text)
        .with_value("surface-elevated")
        .with_prefix("token:")
        .with_suffix(InputAction::new("Apply", "apply-token"))
}

pub fn default_input_group_story_model() -> InputGroupModel {
    InputGroupModel::new("42")
        .with_value("42")
        .with_addon("$")
        .with_button(InputAction::new("Apply", "apply-amount"))
        .required()
}

pub fn dense_input_group_story_model() -> InputGroupModel {
    InputGroupModel::new("lesson slug")
        .with_density(InputDensity::Dense)
        .with_value("ui-input-group")
        .with_addon("slug")
        .with_button(InputAction::new("Save", "save-slug"))
}

pub fn loading_input_group_story_model() -> InputGroupModel {
    default_input_group_story_model().loading()
}

pub fn disabled_input_group_story_model() -> InputGroupModel {
    InputGroupModel::new("Locked")
        .with_value("read-only")
        .with_addon("id")
        .with_button(InputAction::new("Copy", "copy-read-only").disabled())
        .disabled()
}

pub fn themed_input_group_story_model() -> InputGroupModel {
    InputGroupModel::new("theme token")
        .with_value("brand")
        .with_addon("token")
        .with_button(InputAction::new("Apply", "apply-theme-token"))
}

pub fn default_input_otp_story_model() -> InputOtpModel {
    InputOtpModel::new(6)
        .with_value("123")
        .with_group_size(3)
        .required()
}

pub fn dense_input_otp_story_model() -> InputOtpModel {
    InputOtpModel::new(6)
        .with_density(InputDensity::Dense)
        .with_value("42")
        .with_group_size(2)
}

pub fn loading_input_otp_story_model() -> InputOtpModel {
    default_input_otp_story_model().loading()
}

pub fn disabled_input_otp_story_model() -> InputOtpModel {
    InputOtpModel::new(6)
        .with_value("987654")
        .with_group_size(3)
        .disabled()
}

pub fn themed_input_otp_story_model() -> InputOtpModel {
    InputOtpModel::new(8)
        .alphanumeric()
        .with_value("A1B2")
        .with_group_size(4)
        .with_separator(" ")
        .with_label("Recovery code")
}

pub fn default_item_story_model() -> ItemModel {
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

pub fn dense_item_story_model() -> ItemModel {
    ItemModel::new(
        "Bevy primitive",
        "The same render node can become a scene item.",
    )
    .with_density(ItemDensity::Dense)
    .with_media("B")
    .with_actions(vec![ItemAction::new("Inspect", "inspect-bevy-primitive")])
}

pub fn loading_item_story_model() -> ItemModel {
    default_item_story_model().loading()
}

pub fn disabled_item_story_model() -> ItemModel {
    ItemModel::new("Locked item", "This row is visible but unavailable.")
        .with_media("L")
        .with_actions(vec![ItemAction::new("Open", "open-locked-item").disabled()])
        .disabled()
}

pub fn themed_item_story_model() -> ItemModel {
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

pub fn default_kbd_story_model() -> KbdModel {
    KbdModel::new(vec![KbdKey::new("Cmd"), KbdKey::new("K")])
        .with_separator(" + ")
        .with_aria_label("Open command menu")
}

pub fn dense_kbd_story_model() -> KbdModel {
    KbdModel::new(vec![KbdKey::new("Shift"), KbdKey::new("P")])
        .with_density(KbdDensity::Dense)
        .with_separator(" + ")
        .with_aria_label("Open palette")
}

pub fn loading_kbd_story_model() -> KbdModel {
    default_kbd_story_model().loading()
}

pub fn disabled_kbd_story_model() -> KbdModel {
    KbdModel::new(vec![
        KbdKey::new("Ctrl"),
        KbdKey::new("S").with_value("save"),
    ])
    .with_separator(" + ")
    .with_aria_label("Save disabled")
    .disabled()
}

pub fn themed_kbd_story_model() -> KbdModel {
    KbdModel::new(vec![
        KbdKey::new("Ctrl"),
        KbdKey::new("Shift"),
        KbdKey::new("D"),
    ])
    .with_separator(" + ")
    .with_aria_label("Toggle diagnostics")
}

pub fn default_label_story_model() -> LabelModel {
    LabelModel::new("Email").with_for("email").required()
}

pub fn dense_label_story_model() -> LabelModel {
    LabelModel::new("Username")
        .with_density(LabelDensity::Dense)
        .with_for("username")
        .optional()
}

pub fn loading_label_story_model() -> LabelModel {
    default_label_story_model().loading()
}

pub fn disabled_label_story_model() -> LabelModel {
    LabelModel::new("Archived email")
        .with_for("archived_email")
        .required()
        .disabled()
}

pub fn themed_label_story_model() -> LabelModel {
    LabelModel::new("Theme token").with_requirement(LabelRequirement::Optional)
}

pub fn default_marker_story_model() -> MarkerModel {
    MarkerModel::new("3 new")
        .with_value("unread")
        .with_dot_label("Unread messages")
        .with_anchor(MarkerAnchor::new("Jump", "#latest-message"))
}

pub fn dense_marker_story_model() -> MarkerModel {
    MarkerModel::new("Pinned")
        .with_density(MarkerDensity::Dense)
        .with_tone(MarkerTone::Success)
        .with_value("pinned")
        .with_dot_label("Pinned marker")
        .without_anchor()
}

pub fn loading_marker_story_model() -> MarkerModel {
    default_marker_story_model().loading()
}

pub fn disabled_marker_story_model() -> MarkerModel {
    MarkerModel::new("Archived")
        .with_tone(MarkerTone::Neutral)
        .with_value("archived")
        .with_dot_label("Archived marker")
        .with_anchor(MarkerAnchor::new("Open", "#archived").disabled())
        .disabled()
}

pub fn themed_marker_story_model() -> MarkerModel {
    MarkerModel::new("2 notes")
        .with_tone(MarkerTone::Warning)
        .with_value("notes")
        .with_dot_label("Annotation marker")
        .with_anchor(MarkerAnchor::new("Review", "#review-notes"))
}

pub fn default_menubar_story_model() -> MenubarModel {
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

pub fn dense_menubar_story_model() -> MenubarModel {
    default_menubar_story_model().with_density(MenubarDensity::Dense)
}

pub fn loading_menubar_story_model() -> MenubarModel {
    default_menubar_story_model().loading()
}

pub fn disabled_menubar_story_model() -> MenubarModel {
    default_menubar_story_model().disabled()
}

pub fn themed_menubar_story_model() -> MenubarModel {
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

pub fn default_message_story_model() -> MessageModel {
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

pub fn dense_outgoing_message_story_model() -> MessageModel {
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

pub fn loading_message_story_model() -> MessageModel {
    default_message_story_model().loading()
}

pub fn disabled_message_story_model() -> MessageModel {
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

pub fn themed_message_story_model() -> MessageModel {
    MessageModel::new(
        "Theme runner",
        "Token preview",
        "Message colors, type, spacing, radius, and actions resolve through the shared theme.",
        "Ready",
    )
    .with_side(MessageSide::Incoming)
    .with_actions(vec![MessageAction::new("Inspect", "inspect-theme")])
}

pub fn default_message_scroller_story_model() -> MessageScrollerModel {
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

pub fn dense_latest_message_scroller_story_model() -> MessageScrollerModel {
    default_message_scroller_story_model()
        .with_density(MessageScrollerDensity::Dense)
        .with_at_latest(true)
        .manual_scroll()
}

pub fn loading_message_scroller_story_model() -> MessageScrollerModel {
    default_message_scroller_story_model().loading()
}

pub fn disabled_message_scroller_story_model() -> MessageScrollerModel {
    default_message_scroller_story_model()
        .with_anchor_label("Transcript locked")
        .with_jump_label("Locked")
        .disabled()
}

pub fn themed_message_scroller_story_model() -> MessageScrollerModel {
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

pub fn default_native_select_story_model() -> NativeSelectModel {
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

pub fn dense_native_select_story_model() -> NativeSelectModel {
    default_native_select_story_model()
        .with_density(NativeSelectDensity::Dense)
        .with_label("Dense renderer")
        .with_selected_value("bevy")
}

pub fn loading_native_select_story_model() -> NativeSelectModel {
    default_native_select_story_model().loading()
}

pub fn disabled_native_select_story_model() -> NativeSelectModel {
    default_native_select_story_model()
        .with_label("Locked renderer")
        .disabled()
}

pub fn themed_native_select_story_model() -> NativeSelectModel {
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

pub fn default_navigation_menu_story_model() -> NavigationMenuModel {
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

pub fn dense_navigation_menu_story_model() -> NavigationMenuModel {
    default_navigation_menu_story_model()
        .with_density(NavigationMenuDensity::Dense)
        .with_selected_value("docs")
}

pub fn loading_navigation_menu_story_model() -> NavigationMenuModel {
    default_navigation_menu_story_model().loading()
}

pub fn disabled_navigation_menu_story_model() -> NavigationMenuModel {
    default_navigation_menu_story_model()
        .with_label("Locked navigation")
        .disabled()
}

pub fn themed_navigation_menu_story_model() -> NavigationMenuModel {
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

pub fn default_pagination_story_model() -> PaginationModel {
    PaginationModel::new(12, 5)
}

pub fn dense_pagination_story_model() -> PaginationModel {
    PaginationModel::new(6, 2)
        .with_density(PaginationDensity::Dense)
        .with_sibling_count(2)
}

pub fn loading_pagination_story_model() -> PaginationModel {
    default_pagination_story_model().loading()
}

pub fn disabled_pagination_story_model() -> PaginationModel {
    PaginationModel::new(3, 1)
        .with_previous_label("Back")
        .with_next_label("Forward")
        .disabled()
}

pub fn themed_pagination_story_model() -> PaginationModel {
    PaginationModel::new(9, 7)
        .with_sibling_count(1)
        .with_previous_label("Earlier")
        .with_next_label("Later")
}

pub fn default_popover_story_model() -> PopoverModel {
    PopoverModel::new(
        "Open controls",
        "Shared overlay controls",
        "Popover state stays renderer-local while durable choices remain with the consuming app.",
    )
    .with_eyebrow("Issue 43")
}

pub fn dense_popover_story_model() -> PopoverModel {
    PopoverModel::new(
        "Dense controls",
        "Compact overlay",
        "Dense spacing keeps the trigger, content, and arrow anatomy stable across themes.",
    )
    .with_density(PopoverDensity::Dense)
    .with_eyebrow("Dense")
}

pub fn loading_popover_story_model() -> PopoverModel {
    default_popover_story_model().loading()
}

pub fn disabled_popover_story_model() -> PopoverModel {
    PopoverModel::new(
        "Locked controls",
        "Unavailable overlay",
        "Disabled popovers keep trigger copy visible while suppressing local open changes.",
    )
    .with_eyebrow("Disabled")
    .disabled()
}

pub fn themed_popover_story_model() -> PopoverModel {
    PopoverModel::new(
        "Palette controls",
        "Theme scoped overlay",
        "The same Popover model resolves semantic tokens through the nested Luxury theme.",
    )
    .with_eyebrow("Luxury")
    .with_arrow_label("Theme arrow")
}

pub fn default_progress_story_model() -> ProgressModel {
    ProgressModel::new(64)
        .with_label("Upload")
        .with_detail("64 percent complete")
}

pub fn dense_progress_story_model() -> ProgressModel {
    ProgressModel::new(38)
        .with_density(ProgressDensity::Dense)
        .with_label("Sync")
        .with_detail("38 percent complete")
}

pub fn loading_progress_story_model() -> ProgressModel {
    default_progress_story_model().loading()
}

pub fn disabled_progress_story_model() -> ProgressModel {
    ProgressModel::new(22)
        .with_label("Locked task")
        .with_detail("Progress is visible but not focus-highlightable.")
        .disabled()
}

pub fn indeterminate_progress_story_model() -> ProgressModel {
    ProgressModel::indeterminate()
        .with_label("Preparing")
        .with_detail("The consumer has not provided a determinate value yet.")
}

pub fn themed_progress_story_model() -> ProgressModel {
    ProgressModel::new(86)
        .with_label("Theme build")
        .with_detail("86 percent complete inside the Catppuccin theme scope.")
}

pub fn radio_group_story_options() -> Vec<RadioGroupOption> {
    vec![
        RadioGroupOption::new("Light", "light").with_detail("Use light semantic tokens."),
        RadioGroupOption::new("Dark", "dark").with_detail("Use dark semantic tokens."),
        RadioGroupOption::new("System", "system").with_detail("Follow the device setting."),
    ]
}

pub fn default_radio_group_story_model() -> RadioGroupModel {
    RadioGroupModel::new(radio_group_story_options())
        .with_label("Theme preference")
        .with_selected_value("light")
        .required()
}

pub fn dense_radio_group_story_model() -> RadioGroupModel {
    RadioGroupModel::new(radio_group_story_options())
        .with_density(RadioGroupDensity::Dense)
        .with_label("Compact theme")
        .with_selected_value("dark")
}

pub fn loading_radio_group_story_model() -> RadioGroupModel {
    default_radio_group_story_model().loading()
}

pub fn disabled_radio_group_story_model() -> RadioGroupModel {
    RadioGroupModel::new(radio_group_story_options())
        .with_label("Locked rollout")
        .with_selected_value("system")
        .disabled()
}

pub fn themed_radio_group_story_model() -> RadioGroupModel {
    RadioGroupModel::new(radio_group_story_options())
        .with_label("Theme scope")
        .with_orientation(RadioGroupOrientation::Horizontal)
        .with_selected_value("dark")
}

pub fn resizable_story_panels() -> Vec<ResizablePanel> {
    vec![
        ResizablePanel::new("Outline", "outline", 34)
            .with_detail("Navigation and lesson structure.")
            .with_bounds(20, 70),
        ResizablePanel::new("Workspace", "workspace", 66)
            .with_detail("Primary activity and preview surface.")
            .with_bounds(30, 80),
    ]
}

pub fn three_panel_resizable_story_panels() -> Vec<ResizablePanel> {
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

pub fn default_resizable_story_model() -> ResizableModel {
    ResizableModel::new(resizable_story_panels())
        .with_label("Workspace split")
        .with_active_panel("workspace")
}

pub fn dense_resizable_story_model() -> ResizableModel {
    ResizableModel::new(resizable_story_panels())
        .with_density(ResizableDensity::Dense)
        .with_label("Compact split")
}

pub fn vertical_resizable_story_model() -> ResizableModel {
    ResizableModel::new(three_panel_resizable_story_panels())
        .with_orientation(ResizableOrientation::Vertical)
        .with_label("Stacked panels")
        .with_resizing_handle(1)
}

pub fn loading_resizable_story_model() -> ResizableModel {
    default_resizable_story_model().loading()
}

pub fn disabled_resizable_story_model() -> ResizableModel {
    ResizableModel::new(resizable_story_panels())
        .with_label("Locked split")
        .disabled()
}

pub fn themed_resizable_story_model() -> ResizableModel {
    ResizableModel::new(three_panel_resizable_story_panels()).with_label("Theme scoped split")
}

pub fn scroll_area_story_items() -> Vec<ScrollAreaItem> {
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

pub fn default_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_label("Activity feed")
        .with_active_item("hydrate")
}

pub fn dense_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_density(ScrollAreaDensity::Dense)
        .with_label("Compact feed")
}

pub fn horizontal_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_label("Overflow lanes")
        .with_overflow(ScrollAreaOverflow::Both)
        .with_active_item("render")
}

pub fn loading_scroll_area_story_model() -> ScrollAreaModel {
    default_scroll_area_story_model().loading()
}

pub fn disabled_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_label("Locked feed")
        .disabled()
}

pub fn themed_scroll_area_story_model() -> ScrollAreaModel {
    ScrollAreaModel::new(scroll_area_story_items())
        .with_label("Theme scoped feed")
        .with_overflow(ScrollAreaOverflow::Horizontal)
}

pub fn select_story_groups() -> Vec<SelectGroup> {
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

pub fn default_select_story_model() -> SelectModel {
    SelectModel::new(select_story_groups())
        .with_label("Renderer target")
        .with_placeholder("Choose target")
        .with_selected_value("leptos")
        .required()
}

pub fn dense_select_story_model() -> SelectModel {
    default_select_story_model()
        .with_density(SelectDensity::Dense)
        .with_selected_value("stories")
}

pub fn loading_select_story_model() -> SelectModel {
    default_select_story_model().loading()
}

pub fn disabled_select_story_model() -> SelectModel {
    default_select_story_model()
        .with_label("Locked target")
        .with_selected_value("marketing")
        .disabled()
}

pub fn themed_select_story_model() -> SelectModel {
    SelectModel::new(select_story_groups())
        .with_label("Theme scoped target")
        .with_selected_value("bevy")
}

pub fn default_separator_story_model() -> SeparatorModel {
    SeparatorModel::new("Content boundary")
        .with_detail("Separates related component regions with shared theme tokens.")
}

pub fn dense_separator_story_model() -> SeparatorModel {
    default_separator_story_model()
        .with_density(SeparatorDensity::Dense)
        .with_label("Compact boundary")
}

pub fn vertical_separator_story_model() -> SeparatorModel {
    SeparatorModel::new("Rail divider")
        .with_orientation(SeparatorOrientation::Vertical)
        .with_detail("A vertical divider for split navigation and content rails.")
}

pub fn loading_separator_story_model() -> SeparatorModel {
    default_separator_story_model()
        .with_label("Publishing boundary")
        .loading()
}

pub fn disabled_separator_story_model() -> SeparatorModel {
    default_separator_story_model()
        .with_label("Locked boundary")
        .disabled()
}

pub fn themed_separator_story_model() -> SeparatorModel {
    SeparatorModel::new("Theme scoped boundary")
        .with_density(SeparatorDensity::Dense)
        .decorative()
}

pub fn default_sheet_story_model() -> SheetModel {
    SheetModel::new(
        "Open settings",
        "Project settings",
        "Update scoped preferences without leaving the current workflow.",
        "The sheet keeps transient panel state local while consumers own persisted settings.",
    )
    .with_default_open(true)
}

pub fn dense_sheet_story_model() -> SheetModel {
    default_sheet_story_model()
        .with_density(SheetDensity::Dense)
        .with_actions(vec![
            SheetAction::new("Apply", "apply"),
            SheetAction::new("Close", "close").close_sheet(),
        ])
}

pub fn left_sheet_story_model() -> SheetModel {
    SheetModel::new(
        "Open navigation",
        "Navigation rail",
        "A left-attached sheet for secondary navigation workflows.",
        "Edge placement is part of the shared Rust model so Leptos and Bevy agree.",
    )
    .with_side(SheetSide::Left)
    .with_default_open(true)
}

pub fn loading_sheet_story_model() -> SheetModel {
    default_sheet_story_model().loading()
}

pub fn disabled_sheet_story_model() -> SheetModel {
    default_sheet_story_model()
        .with_close_label("Locked")
        .disabled()
}

pub fn themed_sheet_story_model() -> SheetModel {
    SheetModel::new(
        "Open theme sheet",
        "Theme scoped sheet",
        "The same sheet tokens resolve inside the nested Dracula theme.",
        "Theme switching stays token-driven across the overlay surface.",
    )
    .with_side(SheetSide::Bottom)
    .with_default_open(true)
}

pub fn sidebar_story_groups() -> Vec<SidebarGroup> {
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

pub fn default_sidebar_story_model() -> SidebarModel {
    SidebarModel::new(sidebar_story_groups())
        .with_header("rs-dean", "Rust/WASM workspace")
        .with_active_value("stories")
}

pub fn dense_sidebar_story_model() -> SidebarModel {
    default_sidebar_story_model()
        .with_density(SidebarDensity::Dense)
        .with_active_value("components")
}

pub fn collapsed_sidebar_story_model() -> SidebarModel {
    default_sidebar_story_model().collapsed()
}

pub fn loading_sidebar_story_model() -> SidebarModel {
    default_sidebar_story_model().loading()
}

pub fn disabled_sidebar_story_model() -> SidebarModel {
    default_sidebar_story_model()
        .with_footer("Locked", "Navigation waits for durable state hydration.")
        .disabled()
}

pub fn themed_sidebar_story_model() -> SidebarModel {
    SidebarModel::new(sidebar_story_groups())
        .with_label("Theme scoped navigation")
        .with_active_value("themes")
}

pub fn default_skeleton_story_model() -> SkeletonModel {
    SkeletonModel::new("Loading component summary")
        .with_block_label("Title area")
        .with_text_label("Summary copy")
        .with_media_label("Preview frame")
        .with_detail("A placeholder keeps the final layout stable while data hydrates.")
}

pub fn dense_skeleton_story_model() -> SkeletonModel {
    default_skeleton_story_model()
        .with_density(SkeletonDensity::Dense)
        .with_text_lines(2)
        .with_block_label("Compact title")
}

pub fn ready_skeleton_story_model() -> SkeletonModel {
    default_skeleton_story_model()
        .with_detail("Content is ready, so placeholder parts stay hidden without layout churn.")
        .ready()
}

pub fn static_skeleton_story_model() -> SkeletonModel {
    default_skeleton_story_model()
        .with_detail("Motion can be disabled while preserving the same placeholder geometry.")
        .with_text_lines(4)
        .static_placeholder()
}

pub fn disabled_skeleton_story_model() -> SkeletonModel {
    default_skeleton_story_model()
        .with_detail("The loading surface is locked while the owner resolves durable state.")
        .disabled()
}

pub fn themed_skeleton_story_model() -> SkeletonModel {
    SkeletonModel::new("Theme scoped placeholder")
        .with_density(SkeletonDensity::Dense)
        .with_text_lines(3)
        .with_detail("Semantic placeholder colors resolve through the nested theme.")
}

pub fn default_slider_story_model() -> SliderModel {
    SliderModel::new(0, 100, 64)
        .with_label("Completion")
        .with_step(4)
        .with_detail("Adjust a local value while the app decides whether to persist it.")
}

pub fn dense_slider_story_model() -> SliderModel {
    default_slider_story_model()
        .with_density(SliderDensity::Dense)
        .with_value(40)
}

pub fn vertical_slider_story_model() -> SliderModel {
    SliderModel::new(0, 12, 8)
        .with_label("Columns")
        .with_unit(" columns")
        .with_step(1)
        .with_orientation(SliderOrientation::Vertical)
        .with_detail("Vertical orientation remains part of the shared Rust model.")
}

pub fn loading_slider_story_model() -> SliderModel {
    default_slider_story_model()
        .with_label("Hydrating value")
        .loading()
}

pub fn disabled_slider_story_model() -> SliderModel {
    default_slider_story_model()
        .with_label("Locked threshold")
        .with_value(32)
        .disabled()
}

pub fn themed_slider_story_model() -> SliderModel {
    SliderModel::new(0, 100, 80)
        .with_label("Theme scoped range")
        .with_step(5)
        .with_detail("Slider colors resolve from the active theme tokens.")
}

pub fn default_sonner_story_model() -> SonnerModel {
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

pub fn dense_sonner_story_model() -> SonnerModel {
    default_sonner_story_model()
        .with_density(SonnerDensity::Dense)
        .with_label("Dense notifications")
}

pub fn centered_sonner_story_model() -> SonnerModel {
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

pub fn loading_sonner_story_model() -> SonnerModel {
    default_sonner_story_model()
        .with_label("Publishing notifications")
        .loading()
}

pub fn disabled_sonner_story_model() -> SonnerModel {
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

pub fn themed_sonner_story_model() -> SonnerModel {
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

pub fn default_spinner_story_model() -> SpinnerModel {
    SpinnerModel::new("Loading components")
        .with_detail("A compact busy indicator that keeps motion state renderer-local.")
}

pub fn dense_spinner_story_model() -> SpinnerModel {
    default_spinner_story_model()
        .with_density(SpinnerDensity::Dense)
        .with_size(SpinnerSize::Small)
        .with_tone(SpinnerTone::Info)
        .with_detail("Dense spinners preserve the same shared anatomy in less space.")
}

pub fn large_spinner_story_model() -> SpinnerModel {
    default_spinner_story_model()
        .with_size(SpinnerSize::Large)
        .with_tone(SpinnerTone::Warning)
        .with_speed_ms(1_200)
        .with_detail("Size and rotation speed are validated in the shared Rust model.")
}

pub fn ready_spinner_story_model() -> SpinnerModel {
    default_spinner_story_model()
        .with_tone(SpinnerTone::Success)
        .with_detail("Ready state hides the active indicator while preserving label anatomy.")
        .ready()
}

pub fn disabled_spinner_story_model() -> SpinnerModel {
    default_spinner_story_model()
        .with_tone(SpinnerTone::Destructive)
        .with_detail("Disabled spinners pause all renderer-local motion controls.")
        .disabled()
}

pub fn themed_spinner_story_model() -> SpinnerModel {
    SpinnerModel::new("Theme scoped loading")
        .with_size(SpinnerSize::Large)
        .with_tone(SpinnerTone::Brand)
        .with_detail("Spinner border colors resolve through the nested Luxury theme.")
}

pub fn default_switch_story_model() -> SwitchModel {
    SwitchModel::new("Persist theme preference", "theme-preference")
        .with_detail("The renderer can toggle locally; consumers persist the accepted setting.")
        .with_on_label("Synced")
        .with_off_label("Local")
        .checked()
}

pub fn dense_switch_story_model() -> SwitchModel {
    default_switch_story_model()
        .with_density(SwitchDensity::Dense)
        .with_detail("Dense switches keep the same track, thumb, and label anatomy.")
}

pub fn off_switch_story_model() -> SwitchModel {
    SwitchModel::new("Use local motion setting", "motion-setting")
        .with_detail("Unchecked switches keep the same Bevy primitive contract.")
        .with_on_label("Motion")
        .with_off_label("Static")
        .unchecked()
}

pub fn loading_switch_story_model() -> SwitchModel {
    default_switch_story_model()
        .with_detail("Loading switches block interaction while the app reconciles durable state.")
        .loading()
}

pub fn disabled_switch_story_model() -> SwitchModel {
    default_switch_story_model()
        .with_detail("Disabled switches expose a stable read-only state to every renderer.")
        .disabled()
}

pub fn themed_switch_story_model() -> SwitchModel {
    SwitchModel::new("Theme scoped switch", "theme-scoped-switch")
        .with_detail("Switch track and status colors resolve through the nested Dracula theme.")
        .with_on_label("Dracula")
        .with_off_label("Default")
        .checked()
}

pub fn default_toggle_story_model() -> ToggleModel {
    ToggleModel::new("Bold", "bold")
        .with_detail("Renderer-local pressed state mirrors a toolbar command before persistence.")
        .with_pressed_label("Active")
        .with_unpressed_label("Inactive")
        .with_pressed_indicator("on")
        .with_unpressed_indicator("off")
        .pressed()
}

pub fn dense_toggle_story_model() -> ToggleModel {
    default_toggle_story_model()
        .with_density(ToggleDensity::Dense)
        .with_detail("Dense toggles preserve the same root, indicator, and label anatomy.")
}

pub fn outline_toggle_story_model() -> ToggleModel {
    ToggleModel::new("Italic", "italic")
        .with_variant(ToggleVariant::Outline)
        .with_pressed(TogglePressed::Unpressed)
        .with_detail("Outline toggles use the same state model with a quieter surface.")
        .with_pressed_label("Enabled")
        .with_unpressed_label("Disabled")
        .with_pressed_indicator("on")
        .with_unpressed_indicator("off")
}

pub fn loading_toggle_story_model() -> ToggleModel {
    default_toggle_story_model()
        .with_detail("Loading toggles block interaction while the editor reconciles durable state.")
        .loading()
}

pub fn disabled_toggle_story_model() -> ToggleModel {
    default_toggle_story_model()
        .with_detail("Disabled toggles expose a read-only pressed state to every renderer.")
        .disabled()
}

pub fn themed_toggle_story_model() -> ToggleModel {
    ToggleModel::new("Theme scoped toggle", "theme-scoped-toggle")
        .with_variant(ToggleVariant::Outline)
        .with_detail("Toggle colors resolve through the nested Cyberpunk theme.")
        .with_pressed_label("Cyber")
        .with_unpressed_label("Default")
        .with_pressed_indicator("on")
        .with_unpressed_indicator("off")
        .pressed()
}

pub fn default_toggle_group_story_model() -> ToggleGroupModel {
    ToggleGroupModel::new(vec![
        ToggleGroupItem::new("Left", "left").with_detail("Align content to the left edge."),
        ToggleGroupItem::new("Center", "center").with_detail("Center content in the container."),
        ToggleGroupItem::new("Right", "right").with_detail("Align content to the right edge."),
    ])
    .with_label("Text alignment")
    .with_selected_value("left")
}

pub fn multiple_toggle_group_story_model() -> ToggleGroupModel {
    ToggleGroupModel::new(vec![
        ToggleGroupItem::new("Bold", "bold").with_detail("Apply bold text weight."),
        ToggleGroupItem::new("Italic", "italic").with_detail("Apply italic text style."),
        ToggleGroupItem::new("Underline", "underline").with_detail("Apply underline decoration."),
    ])
    .with_label("Text style")
    .with_selection_mode(ToggleGroupSelectionMode::Multiple)
    .with_selected_values(vec!["bold".to_owned(), "italic".to_owned()])
}

pub fn dense_toggle_group_story_model() -> ToggleGroupModel {
    default_toggle_group_story_model()
        .with_density(ToggleDensity::Dense)
        .with_variant(ToggleVariant::Outline)
}

pub fn vertical_toggle_group_story_model() -> ToggleGroupModel {
    default_toggle_group_story_model()
        .with_label("Panel alignment")
        .with_orientation(ToggleGroupOrientation::Vertical)
        .with_selected_value("center")
}

pub fn loading_toggle_group_story_model() -> ToggleGroupModel {
    default_toggle_group_story_model().loading()
}

pub fn disabled_toggle_group_story_model() -> ToggleGroupModel {
    default_toggle_group_story_model().disabled()
}

pub fn themed_toggle_group_story_model() -> ToggleGroupModel {
    ToggleGroupModel::new(vec![
        ToggleGroupItem::new("Compact", "compact").with_detail("Compact density preset."),
        ToggleGroupItem::new("Roomy", "roomy").with_detail("Roomy density preset."),
        ToggleGroupItem::new("Focus", "focus").with_detail("Focus density preset."),
    ])
    .with_label("Theme scoped group")
    .with_variant(ToggleVariant::Outline)
    .with_selected_value("roomy")
}

pub fn default_tooltip_story_model() -> TooltipModel {
    TooltipModel::new(
        "Save",
        "save-command",
        "Writes the current draft to durable state once the consumer accepts the action.",
    )
}

pub fn dense_tooltip_story_model() -> TooltipModel {
    TooltipModel::new(
        "Preview",
        "preview-command",
        "Opens a renderer-local preview without changing persisted state.",
    )
    .with_density(TooltipDensity::Dense)
}

pub fn placement_tooltip_story_model() -> TooltipModel {
    TooltipModel::new(
        "Sync",
        "sync-command",
        "Tooltip placement is part of the shared model and renders through token classes.",
    )
    .with_placement(TooltipPlacement::Right)
}

pub fn loading_tooltip_story_model() -> TooltipModel {
    default_tooltip_story_model().loading()
}

pub fn disabled_tooltip_story_model() -> TooltipModel {
    TooltipModel::new(
        "Locked",
        "locked-command",
        "Disabled tooltips keep the trigger readable but block hover and focus transitions.",
    )
    .disabled()
}

pub fn hidden_arrow_tooltip_story_model() -> TooltipModel {
    TooltipModel::new(
        "Compact hint",
        "compact-hint",
        "The arrow node stays in the render contract even when a renderer hides it.",
    )
    .without_arrow()
}

pub fn themed_tooltip_story_model() -> TooltipModel {
    TooltipModel::new(
        "Theme hint",
        "theme-hint",
        "The same Tooltip model resolves semantic tokens through the nested Dracula theme.",
    )
    .with_placement(TooltipPlacement::Bottom)
}

pub fn table_story_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Component", "component"),
        TableColumn::new("Surface", "surface"),
        TableColumn::new("Score", "score").numeric(),
    ]
}

pub fn table_story_rows() -> Vec<TableRow> {
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

pub fn default_table_story_model() -> TableModel {
    TableModel::new(table_story_columns(), table_story_rows())
        .with_caption("Semantic table rows render from shared Rust data.")
}

pub fn dense_table_story_model() -> TableModel {
    default_table_story_model()
        .with_density(TableDensity::Dense)
        .with_caption(
            "Dense tables preserve the same header, body, row, cell, and caption anatomy.",
        )
}

pub fn selected_table_story_model() -> TableModel {
    default_table_story_model()
        .with_caption("Renderer-local row selection stays separate from durable collection state.")
        .with_selected_row("switch")
}

pub fn loading_table_story_model() -> TableModel {
    default_table_story_model()
        .with_caption("Loading tables keep structure visible while blocking row interaction.")
        .loading()
}

pub fn disabled_table_story_model() -> TableModel {
    default_table_story_model()
        .with_caption("Disabled tables expose read-only row and cell primitives.")
        .disabled()
}

pub fn themed_table_story_model() -> TableModel {
    TableModel::new(table_story_columns(), table_story_rows())
        .with_caption("Table surfaces resolve through the nested Luxury theme.")
        .with_selected_row("spinner")
}

pub fn tabs_story_items() -> Vec<TabsItem> {
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

pub fn default_tabs_story_model() -> TabsModel {
    TabsModel::new(tabs_story_items())
        .with_label("Framework surfaces")
        .with_selected_value("tokens")
}

pub fn dense_tabs_story_model() -> TabsModel {
    default_tabs_story_model()
        .with_density(TabsDensity::Dense)
        .with_selected_value("leptos")
}

pub fn vertical_tabs_story_model() -> TabsModel {
    default_tabs_story_model()
        .with_orientation(TabsOrientation::Vertical)
        .with_selected_value("bevy")
}

pub fn loading_tabs_story_model() -> TabsModel {
    default_tabs_story_model()
        .with_label("Hydrating tabs")
        .loading()
}

pub fn disabled_tabs_story_model() -> TabsModel {
    default_tabs_story_model()
        .with_label("Locked tabs")
        .disabled()
}

pub fn themed_tabs_story_model() -> TabsModel {
    TabsModel::new(tabs_story_items())
        .with_label("Theme scoped tabs")
        .with_density(TabsDensity::Dense)
        .with_selected_value("bevy")
}

pub fn default_textarea_story_model() -> TextareaModel {
    TextareaModel::new(
        "Implementation notes",
        "Draft locally, then persist the accepted text through app state.",
    )
    .with_value("Textarea state is local until the consumer accepts the draft.")
    .with_placeholder("Add implementation notes")
    .with_max_length(160)
}

pub fn dense_textarea_story_model() -> TextareaModel {
    default_textarea_story_model()
        .with_density(TextareaDensity::Dense)
        .with_rows(3)
        .with_value("Dense textareas use the same shared Rust contract.")
}

pub fn unlimited_textarea_story_model() -> TextareaModel {
    default_textarea_story_model()
        .with_hint("Counter remains present even when no consumer limit is configured.")
        .without_max_length()
}

pub fn loading_textarea_story_model() -> TextareaModel {
    default_textarea_story_model()
        .with_hint("Loading textareas block draft updates while hydration completes.")
        .loading()
}

pub fn disabled_textarea_story_model() -> TextareaModel {
    default_textarea_story_model()
        .with_hint("Disabled textareas expose a read-only draft across renderers.")
        .disabled()
}

pub fn themed_textarea_story_model() -> TextareaModel {
    TextareaModel::new(
        "Theme scoped notes",
        "Textarea borders, hints, and counters resolve through the nested theme.",
    )
    .with_density(TextareaDensity::Dense)
    .with_value("The same model feeds DOM and Bevy primitive renderers.")
    .with_max_length(120)
}

pub fn default_toast_story_model() -> ToastModel {
    ToastModel::new(
        "Branch pushed",
        "The UI sweep commit is on the PR branch and waiting for the gate.",
    )
    .with_tone(ToastTone::Success)
    .with_action(ToastAction::new("Open checks", "open-checks"))
}

pub fn dense_toast_story_model() -> ToastModel {
    default_toast_story_model()
        .with_density(ToastDensity::Dense)
        .with_title("Gate running")
        .with_description("Dense toasts keep the same local open and action state.")
        .with_tone(ToastTone::Info)
}

pub fn centered_toast_story_model() -> ToastModel {
    default_toast_story_model()
        .with_position(ToastPosition::BottomCenter)
        .with_title("Queued locally")
        .with_description("The viewport position is part of the shared Rust model.")
        .without_action()
}

pub fn loading_toast_story_model() -> ToastModel {
    default_toast_story_model()
        .with_title("Publishing")
        .with_description("Loading toasts keep copy visible while blocking action intent.")
        .loading()
}

pub fn disabled_toast_story_model() -> ToastModel {
    default_toast_story_model()
        .with_title("Notifications paused")
        .with_description("Disabled toasts preserve anatomy while muting action affordances.")
        .disabled()
}

pub fn themed_toast_story_model() -> ToastModel {
    ToastModel::new(
        "Theme scoped toast",
        "Toast severity colors resolve through the nested Dracula theme.",
    )
    .with_density(ToastDensity::Dense)
    .with_tone(ToastTone::Info)
    .with_action(ToastAction::new("Inspect", "inspect-toast-theme"))
}

pub fn typography_story_items() -> Vec<TypographyListItem> {
    vec![
        TypographyListItem::new("Shared type scale", "shared-type-scale")
            .with_detail("Headings, body copy, and list rhythm use rs-dean-ui tokens."),
        TypographyListItem::new("Renderer local active state", "renderer-local-active")
            .with_detail("Hover and focus styling stay local to the renderer."),
        TypographyListItem::new("Bevy primitive proof", "bevy-primitive-proof")
            .with_detail("Scene primitives consume the same render nodes without Leptos."),
    ]
}

pub fn default_typography_story_model() -> TypographyModel {
    TypographyModel::new(
        "Readable systems",
        "Token scales",
        "Typography uses shared text, leading, spacing, and color tokens without custom class names.",
        typography_story_items(),
        "Update the token once and every renderer receives the same typographic contract.",
    )
    .with_list_label("Typography contract")
    .with_cite("rs-dean-ui")
}

pub fn dense_typography_story_model() -> TypographyModel {
    default_typography_story_model()
        .with_density(TypographyDensity::Dense)
        .with_heading("Dense type flow")
        .with_subheading("Compact reading")
        .with_paragraph("Dense typography keeps the same anatomy with tighter token spacing.")
}

pub fn loading_typography_story_model() -> TypographyModel {
    default_typography_story_model()
        .with_heading("Loading copy")
        .with_paragraph("Loading typography keeps the layout stable while content hydrates.")
        .loading()
}

pub fn disabled_typography_story_model() -> TypographyModel {
    default_typography_story_model()
        .with_heading("Locked copy")
        .with_paragraph("Disabled typography exposes a read-only content flow to every renderer.")
        .disabled()
}

pub fn themed_typography_story_model() -> TypographyModel {
    TypographyModel::new(
        "Theme scoped typography",
        "Catppuccin text rhythm",
        "The same heading, body, list, and blockquote nodes resolve through the nested theme.",
        typography_story_items(),
        "Semantic colors and text scales come from the shared token contract.",
    )
    .with_density(TypographyDensity::Dense)
    .with_list_label("Themed typography contract")
    .with_cite("ThemeScope")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn every_component_has_valid_canonical_fixtures() {
        let mut total = 0;
        let mut themed = 0;
        let mut default_open = 0;

        for id in UiComponentId::ALL {
            let fixtures = ui_story_fixtures(id);
            assert!(
                !fixtures.is_empty(),
                "{} has no fixtures",
                id.definition().name
            );

            let fixture_ids = fixtures
                .iter()
                .map(|fixture| fixture.fixture_id)
                .collect::<HashSet<_>>();
            assert_eq!(
                fixture_ids.len(),
                fixtures.len(),
                "{} has duplicate fixture IDs",
                id.definition().name
            );

            for fixture in &fixtures {
                assert_eq!(fixture.model.component_id(), id);
                assert!(
                    fixture.model.validate().is_ok(),
                    "{} fixture {} is invalid",
                    id.definition().name,
                    fixture.fixture_id
                );
                themed += usize::from(fixture.kind == UiStoryVariantKind::Themed);
                default_open += usize::from(fixture.default_open);
            }
            total += fixtures.len();
        }

        assert_eq!(total, 338);
        assert_eq!(themed, UiComponentId::ALL.len() - 1);
        assert_eq!(default_open, 3);
    }

    #[test]
    fn alert_fixtures_preserve_the_reference_story_copy_and_theme() {
        let fixtures = ui_story_fixtures(UiComponentId::Alert);
        assert_eq!(fixtures.len(), 5);

        let UiStoryModel::Alert(default_alert) = &fixtures[0].model else {
            panic!("alert fixture registry returned a different component model");
        };
        assert_eq!(default_alert.title, "Build completed");
        assert_eq!(
            default_alert.description,
            "The design-token bundle is ready for review in the shared UI crate."
        );
        assert_eq!(
            default_alert
                .action
                .as_ref()
                .map(|action| action.label.as_str()),
            Some("Open report")
        );

        assert_eq!(fixtures[4].kind, UiStoryVariantKind::Themed);
        assert_eq!(fixtures[4].theme_id, ThemeId::Dracula);
    }
}
