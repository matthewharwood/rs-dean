use crate::{ComponentDefinition, UiComponentCategory, UiComponentId, UiStateModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum UiBlockRole {
    Action,
    Content,
    Control,
    Data,
    Feedback,
    Header,
    Indicator,
    Item,
    Layout,
    Media,
    Navigation,
    Overlay,
    Root,
    Text,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum UiBlockTone {
    Accent,
    Brand,
    Danger,
    Info,
    Muted,
    Surface,
    Success,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiBlock {
    pub role: UiBlockRole,
    pub tone: UiBlockTone,
    pub label: &'static str,
    pub detail: &'static str,
}

impl UiBlock {
    pub const fn new(
        role: UiBlockRole,
        tone: UiBlockTone,
        label: &'static str,
        detail: &'static str,
    ) -> Self {
        Self {
            role,
            tone,
            label,
            detail,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiComponentSpec {
    pub definition: &'static ComponentDefinition,
    pub blocks: Vec<UiBlock>,
}

impl UiComponentSpec {
    pub fn new(id: UiComponentId) -> Self {
        let definition = id.definition();
        let mut blocks = Vec::with_capacity(id.anatomy_parts().len() + 1);
        blocks.push(UiBlock::new(
            UiBlockRole::Root,
            tone_for_category(definition.category),
            definition.name,
            definition.summary,
        ));
        for part in id.anatomy_parts() {
            blocks.push(UiBlock::new(
                role_for_part(part),
                tone_for_role(role_for_part(part), definition.category),
                part,
                detail_for_part(part, definition.state),
            ));
        }
        Self { definition, blocks }
    }
}

pub fn component_spec(id: UiComponentId) -> UiComponentSpec {
    UiComponentSpec::new(id)
}

pub fn role_for_part(part: &str) -> UiBlockRole {
    let lower = part.to_ascii_lowercase();
    if lower.contains("trigger") || lower.contains("action") || lower.contains("button") {
        UiBlockRole::Action
    } else if lower.contains("input")
        || lower.contains("select")
        || lower.contains("control")
        || lower.contains("checkbox")
        || lower.contains("radio")
        || lower.contains("slider")
        || lower.contains("switch")
        || lower.contains("thumb")
    {
        UiBlockRole::Control
    } else if lower.contains("content")
        || lower.contains("description")
        || lower.contains("body")
        || lower.contains("viewport")
    {
        UiBlockRole::Content
    } else if lower.contains("header") || lower.contains("title") || lower.contains("label") {
        UiBlockRole::Header
    } else if lower.contains("menu")
        || lower.contains("navigation")
        || lower.contains("breadcrumb")
        || lower.contains("pagination")
        || lower.contains("tabs")
    {
        UiBlockRole::Navigation
    } else if lower.contains("dialog")
        || lower.contains("popover")
        || lower.contains("drawer")
        || lower.contains("sheet")
        || lower.contains("tooltip")
        || lower.contains("cardcontent")
    {
        UiBlockRole::Overlay
    } else if lower.contains("table")
        || lower.contains("chart")
        || lower.contains("axis")
        || lower.contains("series")
        || lower.contains("cell")
        || lower.contains("row")
    {
        UiBlockRole::Data
    } else if lower.contains("image")
        || lower.contains("media")
        || lower.contains("avatar")
        || lower.contains("preview")
    {
        UiBlockRole::Media
    } else if lower.contains("indicator")
        || lower.contains("progress")
        || lower.contains("spinner")
        || lower.contains("skeleton")
        || lower.contains("marker")
    {
        UiBlockRole::Indicator
    } else if lower.contains("separator") || lower.contains("handle") || lower.contains("ratio") {
        UiBlockRole::Layout
    } else if lower.contains("toast")
        || lower.contains("alert")
        || lower.contains("empty")
        || lower.contains("sonner")
    {
        UiBlockRole::Feedback
    } else if lower.contains("text") || lower.contains("kbd") || lower.contains("typography") {
        UiBlockRole::Text
    } else {
        UiBlockRole::Item
    }
}

pub const fn tone_for_category(category: UiComponentCategory) -> UiBlockTone {
    match category {
        UiComponentCategory::Action => UiBlockTone::Brand,
        UiComponentCategory::Data => UiBlockTone::Info,
        UiComponentCategory::Disclosure => UiBlockTone::Muted,
        UiComponentCategory::Display => UiBlockTone::Surface,
        UiComponentCategory::Feedback => UiBlockTone::Warning,
        UiComponentCategory::Form => UiBlockTone::Accent,
        UiComponentCategory::Layout => UiBlockTone::Muted,
        UiComponentCategory::Messaging => UiBlockTone::Success,
        UiComponentCategory::Navigation => UiBlockTone::Brand,
        UiComponentCategory::Overlay => UiBlockTone::Danger,
        UiComponentCategory::Typography => UiBlockTone::Surface,
        UiComponentCategory::Utility => UiBlockTone::Muted,
    }
}

pub const fn tone_for_role(role: UiBlockRole, category: UiComponentCategory) -> UiBlockTone {
    match role {
        UiBlockRole::Action | UiBlockRole::Control => UiBlockTone::Brand,
        UiBlockRole::Feedback => UiBlockTone::Warning,
        UiBlockRole::Indicator => UiBlockTone::Accent,
        UiBlockRole::Overlay => UiBlockTone::Danger,
        UiBlockRole::Data => UiBlockTone::Info,
        UiBlockRole::Navigation => UiBlockTone::Brand,
        UiBlockRole::Media => UiBlockTone::Success,
        UiBlockRole::Root => tone_for_category(category),
        UiBlockRole::Content
        | UiBlockRole::Header
        | UiBlockRole::Item
        | UiBlockRole::Layout
        | UiBlockRole::Text => UiBlockTone::Surface,
    }
}

pub const fn detail_for_part(part: &'static str, state: UiStateModel) -> &'static str {
    match state {
        UiStateModel::ConsumerDurable => durable_detail(part),
        UiStateModel::Ephemeral => ephemeral_detail(part),
        UiStateModel::Stateless => stateless_detail(part),
    }
}

const fn durable_detail(part: &'static str) -> &'static str {
    let _ = part;
    "Expose controlled state and events; consumers persist meaningful state through rs-dean-state."
}

const fn ephemeral_detail(part: &'static str) -> &'static str {
    let _ = part;
    "Keep open, hover, focus, and transient interaction state local to the renderer."
}

const fn stateless_detail(part: &'static str) -> &'static str {
    let _ = part;
    "Render directly from props and the shared token/theme contract."
}

impl UiBlockRole {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Action => "Action",
            Self::Content => "Content",
            Self::Control => "Control",
            Self::Data => "Data",
            Self::Feedback => "Feedback",
            Self::Header => "Header",
            Self::Indicator => "Indicator",
            Self::Item => "Item",
            Self::Layout => "Layout",
            Self::Media => "Media",
            Self::Navigation => "Navigation",
            Self::Overlay => "Overlay",
            Self::Root => "Root",
            Self::Text => "Text",
        }
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Action => "action",
            Self::Content => "content",
            Self::Control => "control",
            Self::Data => "data",
            Self::Feedback => "feedback",
            Self::Header => "header",
            Self::Indicator => "indicator",
            Self::Item => "item",
            Self::Layout => "layout",
            Self::Media => "media",
            Self::Navigation => "navigation",
            Self::Overlay => "overlay",
            Self::Root => "root",
            Self::Text => "text",
        }
    }

    pub const fn aria_role(self) -> &'static str {
        match self {
            Self::Action => "button",
            Self::Control => "group",
            Self::Data => "table",
            Self::Feedback => "status",
            Self::Header => "heading",
            Self::Navigation => "navigation",
            Self::Overlay => "dialog",
            Self::Root
            | Self::Content
            | Self::Indicator
            | Self::Item
            | Self::Layout
            | Self::Media
            | Self::Text => "group",
        }
    }
}

#[cfg(feature = "bevy")]
pub mod bevy_adapter {
    use bevy::prelude::{Color, Vec2};

    use crate::{
        AccordionMode, AccordionModel, AccordionPart, AlertDialogPart, AlertDialogState, AlertPart,
        AspectRatioPart, AttachmentPart, AvatarPart, AvatarVisual, BadgePart, BadgeTone,
        BreadcrumbPart, BubblePart, BubbleSide, ButtonGroupOrientation, ButtonGroupPart,
        ButtonKind, ButtonPart, ButtonSize, ButtonVariant, CalendarPart, CalendarSelectionMode,
        CardPart, CardVariant, CarouselPart, ChartPart, ChartTone, CheckboxChecked, CheckboxPart,
        CollapsiblePart, ComboboxPart, CommandPart, ContextMenuPart, DataTablePart, DatePickerPart,
        DialogPart, DirectionPart, DirectionValue, DrawerPart, DrawerSide, DropdownMenuPart,
        EmptyPart, FieldPart, HoverCardPart, InputGroupPart, InputOtpPart, InputPart, ItemPart,
        KbdPart, LabelPart, LabelRequirement, MarkerPart, MarkerTone, MenubarPart, MessagePart,
        MessageScrollerPart, MessageSide, NativeSelectPart, NavigationMenuPart, PaginationPart,
        PopoverPart, ProgressPart, RadioGroupPart, RenderContract, ResizablePart, ScrollAreaPart,
        SelectPart, SeparatorPart, SheetPart, SidebarPart, SkeletonPart, SliderPart, SonnerPart,
        SpinnerPart, StateContract, SwitchChecked, SwitchPart, TablePart, TabsPart, TextareaPart,
        Theme, ThemeId, ToastPart, ToggleGroupPart, ToggleGroupSelectionMode, TogglePart,
        TogglePressed, TooltipPart, TypographyPart, UiBlockRole, UiBlockTone, UiComponentId,
        UiWidgetIntent, UiWidgetSlotKind, accordion_render_nodes, alert_dialog_render_nodes,
        alert_render_nodes, aspect_ratio_render_nodes, attachment_render_nodes,
        avatar_render_nodes, badge_render_nodes, breadcrumb_render_nodes, bubble_render_nodes,
        button_group_render_nodes, button_render_nodes, calendar_render_nodes, card_render_nodes,
        carousel_render_nodes, catalog_component_any_render_nodes_for_component,
        chart_render_nodes, checkbox_render_nodes, collapsible_render_nodes, combobox_render_nodes,
        command_render_nodes, component_implementation, context_menu_render_nodes,
        data_table_render_nodes, date_picker_render_nodes, default_accordion_items,
        default_alert_dialog_model, default_alert_model, default_aspect_ratio_model,
        default_attachment_model, default_avatar_model, default_badge_model,
        default_breadcrumb_model, default_bubble_model, default_button_group_model,
        default_button_model, default_calendar_model, default_card_model, default_carousel_model,
        default_chart_model, default_checkbox_model, default_collapsible_model,
        default_combobox_model, default_command_model, default_context_menu_model,
        default_data_table_model, default_date_picker_model, default_dialog_model,
        default_direction_model, default_drawer_model, default_dropdown_menu_model,
        default_empty_model, default_field_model, default_hover_card_model,
        default_input_group_model, default_input_model, default_input_otp_model,
        default_item_model, default_kbd_model, default_label_model, default_marker_model,
        default_menubar_model, default_message_model, default_message_scroller_model,
        default_native_select_model, default_navigation_menu_model, default_pagination_model,
        default_popover_model, default_progress_model, default_radio_group_model,
        default_resizable_model, default_scroll_area_model, default_select_model,
        default_separator_model, default_sheet_model, default_sidebar_model,
        default_skeleton_model, default_slider_model, default_sonner_model, default_spinner_model,
        default_switch_model, default_table_model, default_tabs_model, default_textarea_model,
        default_toast_model, default_toggle_group_model, default_toggle_model,
        default_tooltip_model, default_typography_model, dialog_render_nodes,
        direction_render_nodes, drawer_render_nodes, dropdown_menu_render_nodes,
        empty_render_nodes, field_render_nodes, hover_card_render_nodes, input_group_render_nodes,
        input_otp_render_nodes, input_render_nodes, item_render_nodes, kbd_render_nodes,
        label_render_nodes, marker_render_nodes, menubar_render_nodes, message_render_nodes,
        message_scroller_render_nodes, native_select_render_nodes, navigation_menu_render_nodes,
        pagination_render_nodes, popover_render_nodes, progress_render_nodes,
        radio_group_render_nodes, resizable_render_nodes, scale, scroll_area_render_nodes,
        select_render_nodes, separator_render_nodes, sheet_render_nodes, sidebar_render_nodes,
        skeleton_render_nodes, slider_render_nodes, sonner_render_nodes, spinner_render_nodes,
        switch_render_nodes, table_render_nodes, tabs_render_nodes, textarea_render_nodes,
        toast_render_nodes, toggle_group_render_nodes, toggle_render_nodes, tooltip_render_nodes,
        typography_render_nodes,
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct BevyUiPrimitive {
        pub part: String,
        pub kind: UiWidgetSlotKind,
        pub role: UiBlockRole,
        pub label: String,
        pub value: String,
        pub size: Vec2,
        pub fill: Color,
        pub text: Color,
        pub render: RenderContract,
        pub state: StateContract,
        pub intent: UiWidgetIntent,
        pub selected: bool,
        pub disabled: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BevyUiStoryVariantKind {
        Default,
        Alternate,
        Loading,
        Disabled,
        Themed,
    }

    impl BevyUiStoryVariantKind {
        pub const fn label(self) -> &'static str {
            match self {
                Self::Default => "Default",
                Self::Alternate => "Alternate",
                Self::Loading => "Loading",
                Self::Disabled => "Disabled",
                Self::Themed => "Themed",
            }
        }

        pub const fn detail(self) -> &'static str {
            match self {
                Self::Default => "Validated default fixture",
                Self::Alternate => "Interactive or dense fixture state",
                Self::Loading => "Stable loading layout",
                Self::Disabled => "Read-only disabled state",
                Self::Themed => "Nested ThemeScope palette",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BevyUiStoryVariant {
        pub kind: BevyUiStoryVariantKind,
        pub theme_id: ThemeId,
        pub primitives: Vec<BevyUiPrimitive>,
    }

    impl BevyUiStoryVariant {
        pub fn new(
            kind: BevyUiStoryVariantKind,
            theme_id: ThemeId,
            primitives: Vec<BevyUiPrimitive>,
        ) -> Self {
            Self {
                kind,
                theme_id,
                primitives,
            }
        }

        pub const fn label(&self) -> &'static str {
            self.kind.label()
        }

        pub const fn detail(&self) -> &'static str {
            self.kind.detail()
        }
    }

    pub fn bevy_story_variants_for_component(
        id: UiComponentId,
        base_theme_id: ThemeId,
    ) -> Vec<BevyUiStoryVariant> {
        let base_theme = base_theme_id.palette();
        let themed_theme_id = story_theme_for_component(id, base_theme_id);
        let themed_theme = themed_theme_id.palette();

        let default_primitives = bevy_primitives_for_component(id, &base_theme);
        let mut alternate_primitives = bevy_primitives_for_component(id, &base_theme);
        apply_alternate_story_state(&mut alternate_primitives, &base_theme);

        let mut loading_primitives = bevy_primitives_for_component(id, &base_theme);
        apply_loading_story_state(&mut loading_primitives, &base_theme);

        let mut disabled_primitives = bevy_primitives_for_component(id, &base_theme);
        apply_disabled_story_state(&mut disabled_primitives, &base_theme);

        let mut themed_primitives = bevy_primitives_for_component(id, &themed_theme);
        apply_themed_story_state(&mut themed_primitives, &themed_theme);

        vec![
            BevyUiStoryVariant::new(
                BevyUiStoryVariantKind::Default,
                base_theme_id,
                default_primitives,
            ),
            BevyUiStoryVariant::new(
                BevyUiStoryVariantKind::Alternate,
                base_theme_id,
                alternate_primitives,
            ),
            BevyUiStoryVariant::new(
                BevyUiStoryVariantKind::Loading,
                base_theme_id,
                loading_primitives,
            ),
            BevyUiStoryVariant::new(
                BevyUiStoryVariantKind::Disabled,
                base_theme_id,
                disabled_primitives,
            ),
            BevyUiStoryVariant::new(
                BevyUiStoryVariantKind::Themed,
                themed_theme_id,
                themed_primitives,
            ),
        ]
    }

    fn story_theme_for_component(id: UiComponentId, base_theme_id: ThemeId) -> ThemeId {
        let themed = ThemeId::ALL[(id.index() + 2) % ThemeId::ALL.len()];
        if themed == base_theme_id {
            base_theme_id.next()
        } else {
            themed
        }
    }

    fn apply_alternate_story_state(primitives: &mut [BevyUiPrimitive], theme: &Theme) {
        if let Some(primitive) = primitives.iter_mut().find(|primitive| {
            matches!(
                primitive.role,
                UiBlockRole::Action
                    | UiBlockRole::Control
                    | UiBlockRole::Data
                    | UiBlockRole::Navigation
                    | UiBlockRole::Overlay
            )
        }) {
            primitive.selected = true;
            primitive.fill = theme.selected_tint().to_bevy();
            primitive.text = theme.text_1().to_bevy();
        }
    }

    fn apply_loading_story_state(primitives: &mut [BevyUiPrimitive], theme: &Theme) {
        for (index, primitive) in primitives.iter_mut().enumerate() {
            primitive.selected = false;
            primitive.disabled = true;
            primitive.fill = theme.surface_3().to_bevy();
            primitive.text = theme.text_muted().to_bevy();
            if index == 0 {
                primitive.label = "Loading".to_owned();
                primitive.value = "Hydrating shared component state".to_owned();
            }
        }
    }

    fn apply_disabled_story_state(primitives: &mut [BevyUiPrimitive], theme: &Theme) {
        for primitive in primitives {
            primitive.selected = false;
            primitive.disabled = true;
            primitive.text = theme.text_disabled().to_bevy();
        }
    }

    fn apply_themed_story_state(primitives: &mut [BevyUiPrimitive], theme: &Theme) {
        if let Some(primitive) = primitives.first_mut() {
            primitive.selected = true;
            primitive.fill = theme.primary_soft().to_bevy();
        }
    }

    pub fn bevy_primitives_for_component(id: UiComponentId, theme: &Theme) -> Vec<BevyUiPrimitive> {
        let implementation = component_implementation(id);
        if id == UiComponentId::Accordion {
            return bevy_primitives_for_accordion(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Alert {
            return bevy_primitives_for_alert(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::AlertDialog {
            return bevy_primitives_for_alert_dialog(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::AspectRatio {
            return bevy_primitives_for_aspect_ratio(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Attachment {
            return bevy_primitives_for_attachment(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Avatar {
            return bevy_primitives_for_avatar(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Badge {
            return bevy_primitives_for_badge(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Breadcrumb {
            return bevy_primitives_for_breadcrumb(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Bubble {
            return bevy_primitives_for_bubble(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Button {
            return bevy_primitives_for_button(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::ButtonGroup {
            return bevy_primitives_for_button_group(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Calendar {
            return bevy_primitives_for_calendar(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Card {
            return bevy_primitives_for_card(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Carousel {
            return bevy_primitives_for_carousel(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Chart {
            return bevy_primitives_for_chart(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Checkbox {
            return bevy_primitives_for_checkbox(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Collapsible {
            return bevy_primitives_for_collapsible(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Combobox {
            return bevy_primitives_for_combobox(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Command {
            return bevy_primitives_for_command(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::ContextMenu {
            return bevy_primitives_for_context_menu(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::DropdownMenu {
            return bevy_primitives_for_dropdown_menu(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Empty {
            return bevy_primitives_for_empty(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Field {
            return bevy_primitives_for_field(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::HoverCard {
            return bevy_primitives_for_hover_card(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Input {
            return bevy_primitives_for_input(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::InputGroup {
            return bevy_primitives_for_input_group(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::InputOtp {
            return bevy_primitives_for_input_otp(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Item {
            return bevy_primitives_for_item(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Kbd {
            return bevy_primitives_for_kbd(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Label {
            return bevy_primitives_for_label(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Marker {
            return bevy_primitives_for_marker(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Menubar {
            return bevy_primitives_for_menubar(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Message {
            return bevy_primitives_for_message(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::MessageScroller {
            return bevy_primitives_for_message_scroller(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::NativeSelect {
            return bevy_primitives_for_native_select(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::NavigationMenu {
            return bevy_primitives_for_navigation_menu(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Pagination {
            return bevy_primitives_for_pagination(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Popover {
            return bevy_primitives_for_popover(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Progress {
            return bevy_primitives_for_progress(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::RadioGroup {
            return bevy_primitives_for_radio_group(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Resizable {
            return bevy_primitives_for_resizable(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::ScrollArea {
            return bevy_primitives_for_scroll_area(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Select {
            return bevy_primitives_for_select(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Separator {
            return bevy_primitives_for_separator(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Sheet {
            return bevy_primitives_for_sheet(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Sidebar {
            return bevy_primitives_for_sidebar(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Skeleton {
            return bevy_primitives_for_skeleton(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Slider {
            return bevy_primitives_for_slider(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Sonner {
            return bevy_primitives_for_sonner(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Spinner {
            return bevy_primitives_for_spinner(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Switch {
            return bevy_primitives_for_switch(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Toggle {
            return bevy_primitives_for_toggle(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::ToggleGroup {
            return bevy_primitives_for_toggle_group(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Tooltip {
            return bevy_primitives_for_tooltip(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Typography {
            return bevy_primitives_for_typography(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Table {
            return bevy_primitives_for_table(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Tabs {
            return bevy_primitives_for_tabs(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Textarea {
            return bevy_primitives_for_textarea(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Toast {
            return bevy_primitives_for_toast(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::DataTable {
            return bevy_primitives_for_data_table(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::DatePicker {
            return bevy_primitives_for_date_picker(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Dialog {
            return bevy_primitives_for_dialog(theme, implementation.render, implementation.state);
        }
        if id == UiComponentId::Direction {
            return bevy_primitives_for_direction(
                theme,
                implementation.render,
                implementation.state,
            );
        }
        if id == UiComponentId::Drawer {
            return bevy_primitives_for_drawer(theme, implementation.render, implementation.state);
        }
        catalog_component_any_render_nodes_for_component(id)
            .expect("invariant: non-bespoke component has generated concrete render nodes")
            .into_iter()
            .map(|node| BevyUiPrimitive {
                part: node.part,
                kind: node.kind,
                role: node.role,
                label: node.label,
                value: node.value,
                size: size_for_role(node.role),
                fill: fill_for_tone(node.tone, theme),
                text: theme.text_1().to_bevy(),
                render: implementation.render,
                state: implementation.state,
                intent: node.intent,
                selected: node.selected,
                disabled: node.disabled,
            })
            .collect()
    }

    fn bevy_primitives_for_checkbox(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_checkbox_model();
        let checkbox_state = model.state();
        checkbox_render_nodes(&model, &checkbox_state)
            .into_iter()
            .map(|node| {
                let role = checkbox_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: checkbox_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: checkbox_size_for_part(node.part),
                    fill: fill_for_tone(
                        checkbox_tone_for_part(node.part, node.checked, node.invalid),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: checkbox_intent_for_part(node.part),
                    selected: node.checked.is_active(),
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_collapsible(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_collapsible_model();
        let collapsible_state = model.state();
        collapsible_render_nodes(&model, &collapsible_state)
            .into_iter()
            .map(|node| {
                let role = collapsible_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: collapsible_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: collapsible_size_for_part(node.part),
                    fill: fill_for_tone(
                        collapsible_tone_for_part(node.part, node.open, node.disabled),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: collapsible_intent_for_part(node.part),
                    selected: node.open,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_combobox(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_combobox_model();
        let mut combobox_state = model.state();
        let _ = combobox_state.apply(crate::ComboboxIntent::Open);
        combobox_render_nodes(&model, &combobox_state)
            .into_iter()
            .map(|node| {
                let role = combobox_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: combobox_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: combobox_size_for_part(node.part),
                    fill: fill_for_tone(
                        combobox_tone_for_part(node.part, node.selected, node.visible),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: combobox_intent_for_part(node.part),
                    selected: node.selected,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_command(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_command_model();
        let command_state = model.state();
        command_render_nodes(&model, &command_state)
            .into_iter()
            .map(|node| {
                let role = command_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: command_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: command_size_for_part(node.part),
                    fill: fill_for_tone(
                        command_tone_for_part(
                            node.part,
                            node.highlighted,
                            node.selected,
                            node.visible,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: command_intent_for_part(node.part),
                    selected: node.selected || node.highlighted,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_context_menu(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_context_menu_model();
        let context_menu_state = model.state();
        context_menu_render_nodes(&model, &context_menu_state)
            .into_iter()
            .map(|node| {
                let role = context_menu_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: context_menu_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: context_menu_size_for_part(node.part),
                    fill: fill_for_tone(
                        context_menu_tone_for_part(
                            node.part,
                            node.active,
                            node.selected,
                            node.destructive,
                            node.visible,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: context_menu_intent_for_part(node.part),
                    selected: node.selected || node.active || node.submenu_open,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_dropdown_menu(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_dropdown_menu_model();
        let dropdown_menu_state = model.state();
        dropdown_menu_render_nodes(&model, &dropdown_menu_state)
            .into_iter()
            .map(|node| {
                let role = dropdown_menu_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: dropdown_menu_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: dropdown_menu_size_for_part(node.part),
                    fill: fill_for_tone(
                        dropdown_menu_tone_for_part(
                            node.part,
                            node.active,
                            node.selected,
                            node.destructive,
                            node.visible,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: dropdown_menu_intent_for_part(node.part),
                    selected: node.selected || node.active,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_empty(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_empty_model();
        let empty_state = model.state();
        empty_render_nodes(&model, &empty_state)
            .into_iter()
            .map(|node| {
                let role = empty_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: empty_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: empty_size_for_part(node.part),
                    fill: fill_for_tone(
                        empty_tone_for_part(node.part, node.active, node.actionable),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: empty_intent_for_part(node.part, node.actionable),
                    selected: node.active,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_field(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_field_model();
        let field_state = model.state();
        field_render_nodes(&model, &field_state)
            .into_iter()
            .map(|node| {
                let role = field_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: field_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: field_size_for_part(node.part),
                    fill: fill_for_tone(
                        field_tone_for_part(node.part, node.focused, node.invalid, node.visible),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: field_intent_for_part(node.part),
                    selected: node.focused || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_hover_card(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_hover_card_model().default_open();
        let hover_card_state = model.state();
        hover_card_render_nodes(&model, &hover_card_state)
            .into_iter()
            .map(|node| {
                let role = hover_card_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: hover_card_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: hover_card_size_for_part(node.part),
                    fill: fill_for_tone(
                        hover_card_tone_for_part(node.part, node.open, node.visible),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: hover_card_intent_for_part(node.part),
                    selected: node.open || node.active,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_input(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_input_model();
        let input_state = model.state();
        input_render_nodes(&model, &input_state)
            .into_iter()
            .map(|node| {
                let role = input_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: input_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: input_size_for_part(node.part),
                    fill: fill_for_tone(
                        input_tone_for_part(node.part, node.focused, node.invalid, node.visible),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: input_intent_for_part(node.part, node.actionable),
                    selected: node.focused || node.active || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_input_group(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_input_group_model();
        let input_group_state = model.state();
        input_group_render_nodes(&model, &input_group_state)
            .into_iter()
            .map(|node| {
                let role = input_group_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: input_group_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: input_group_size_for_part(node.part),
                    fill: fill_for_tone(
                        input_group_tone_for_part(
                            node.part,
                            node.focused,
                            node.invalid,
                            node.visible,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: input_group_intent_for_part(node.part, node.actionable),
                    selected: node.focused || node.active || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_input_otp(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_input_otp_model();
        let input_otp_state = model.state();
        input_otp_render_nodes(&model, &input_otp_state)
            .into_iter()
            .map(|node| {
                let role = input_otp_role_for_part(node.part);
                BevyUiPrimitive {
                    part: input_otp_primitive_part(&node),
                    kind: input_otp_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: input_otp_size_for_part(node.part),
                    fill: fill_for_tone(
                        input_otp_tone_for_part(
                            node.part,
                            node.focused,
                            node.filled,
                            node.invalid,
                            node.visible,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: input_otp_intent_for_part(node.part, node.actionable),
                    selected: node.focused || node.active || node.filled || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_item(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_item_model();
        let item_state = model.state();
        item_render_nodes(&model, &item_state)
            .into_iter()
            .map(|node| {
                let role = item_role_for_part(node.part);
                BevyUiPrimitive {
                    part: item_primitive_part(&node),
                    kind: item_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: item_size_for_part(node.part),
                    fill: fill_for_tone(
                        item_tone_for_part(
                            node.part,
                            node.active,
                            node.invalid,
                            node.visible,
                            node.actionable,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: item_intent_for_part(node.part, node.actionable),
                    selected: node.active || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_kbd(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_kbd_model();
        let kbd_state = model.state();
        kbd_render_nodes(&model, &kbd_state)
            .into_iter()
            .map(|node| {
                let role = kbd_role_for_part(node.part);
                BevyUiPrimitive {
                    part: kbd_primitive_part(&node),
                    kind: kbd_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: kbd_size_for_part(node.part),
                    fill: fill_for_tone(
                        kbd_tone_for_part(node.part, node.focused, node.invalid, node.disabled),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: kbd_intent_for_part(node.part),
                    selected: node.focused || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_label(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_label_model();
        let label_state = model.state();
        label_render_nodes(&model, &label_state)
            .into_iter()
            .map(|node| {
                let role = label_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: label_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: label_size_for_part(node.part),
                    fill: fill_for_tone(
                        label_tone_for_part(
                            node.part,
                            node.requirement,
                            node.active,
                            node.invalid,
                            node.visible,
                            node.disabled,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: label_intent_for_part(node.part),
                    selected: node.active || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_marker(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_marker_model();
        let marker_state = model.state();
        marker_render_nodes(&model, &marker_state)
            .into_iter()
            .map(|node| {
                let role = marker_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: marker_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: marker_size_for_part(node.part),
                    fill: fill_for_tone(
                        marker_tone_for_part(
                            node.part,
                            node.tone,
                            node.active,
                            node.invalid,
                            node.visible,
                            node.actionable,
                            node.disabled,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: marker_intent_for_part(node.part, node.actionable),
                    selected: node.active || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_menubar(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_menubar_model();
        let menubar_state = model.state();
        menubar_render_nodes(&model, &menubar_state)
            .into_iter()
            .map(|node| {
                let role = menubar_role_for_part(node.part);
                let tone = menubar_tone_for_node(&node);
                BevyUiPrimitive {
                    part: menubar_primitive_part(&node),
                    kind: menubar_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: menubar_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: menubar_intent_for_part(node.part, node.actionable),
                    selected: node.open || node.focused || node.selected || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_message(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_message_model();
        let message_state = model.state();
        message_render_nodes(&model, &message_state)
            .into_iter()
            .map(|node| {
                let role = message_role_for_part(node.part);
                let tone = message_tone_for_node(&node);
                BevyUiPrimitive {
                    part: message_primitive_part(&node),
                    kind: message_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: message_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: message_intent_for_part(node.part, node.actionable),
                    selected: node.active || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_message_scroller(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_message_scroller_model();
        let message_scroller_state = model.state();
        message_scroller_render_nodes(&model, &message_scroller_state)
            .into_iter()
            .map(|node| {
                let role = message_scroller_role_for_part(node.part);
                let tone = message_scroller_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: message_scroller_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: message_scroller_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: message_scroller_intent_for_part(node.part, node.actionable),
                    selected: node.active || node.invalid || !node.at_latest,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_native_select(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_native_select_model();
        let native_select_state = model.state();
        native_select_render_nodes(&model, &native_select_state)
            .into_iter()
            .map(|node| {
                let role = native_select_role_for_part(node.part);
                let tone = native_select_tone_for_node(&node);
                BevyUiPrimitive {
                    part: native_select_primitive_part(&node),
                    kind: native_select_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: native_select_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: native_select_intent_for_part(node.part, node.actionable),
                    selected: node.selected || node.focused || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_navigation_menu(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_navigation_menu_model();
        let navigation_menu_state = model.state();
        navigation_menu_render_nodes(&model, &navigation_menu_state)
            .into_iter()
            .map(|node| {
                let role = navigation_menu_role_for_part(node.part);
                let tone = navigation_menu_tone_for_node(&node);
                BevyUiPrimitive {
                    part: navigation_menu_primitive_part(&node),
                    kind: navigation_menu_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: navigation_menu_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: navigation_menu_intent_for_part(node.part, node.actionable),
                    selected: node.open || node.focused || node.selected || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_pagination(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_pagination_model();
        let pagination_state = model.state();
        pagination_render_nodes(&model, &pagination_state)
            .into_iter()
            .map(|node| {
                let role = pagination_role_for_part(node.part);
                let tone = pagination_tone_for_node(&node);
                BevyUiPrimitive {
                    part: pagination_primitive_part(&node),
                    kind: pagination_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: pagination_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: pagination_intent_for_part(node.part, node.actionable),
                    selected: node.current || node.focused || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_popover(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_popover_model();
        let popover_state = model.state();
        popover_render_nodes(&model, &popover_state)
            .into_iter()
            .map(|node| {
                let role = popover_role_for_part(node.part);
                let tone = popover_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: popover_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: popover_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: popover_intent_for_part(node.part, node.actionable),
                    selected: node.open || node.active || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_progress(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_progress_model();
        let progress_state = model.state();
        progress_render_nodes(&model, &progress_state)
            .into_iter()
            .map(|node| {
                let role = progress_role_for_part(node.part);
                let tone = progress_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: progress_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: progress_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: UiWidgetIntent::None,
                    selected: node.highlighted || node.invalid || node.loading,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_radio_group(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_radio_group_model();
        let radio_group_state = model.state();
        radio_group_render_nodes(&model, &radio_group_state)
            .into_iter()
            .map(|node| {
                let role = radio_group_role_for_part(node.part);
                let tone = radio_group_tone_for_node(&node);
                BevyUiPrimitive {
                    part: radio_group_primitive_part(&node),
                    kind: radio_group_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: radio_group_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: radio_group_intent_for_part(node.part, node.actionable),
                    selected: node.selected || node.focused || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_resizable(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_resizable_model();
        let resizable_state = model.state();
        resizable_render_nodes(&model, &resizable_state)
            .into_iter()
            .map(|node| {
                let role = resizable_role_for_part(node.part);
                let tone = resizable_tone_for_node(&node);
                BevyUiPrimitive {
                    part: resizable_primitive_part(&node),
                    kind: resizable_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: resizable_size_for_part(node.part, node.percent, node.orientation),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: resizable_intent_for_part(node.part, node.actionable),
                    selected: node.selected || node.resizing || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_scroll_area(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_scroll_area_model().with_overflow(crate::ScrollAreaOverflow::Both);
        let scroll_area_state = model.state();
        scroll_area_render_nodes(&model, &scroll_area_state)
            .into_iter()
            .map(|node| {
                let role = scroll_area_role_for_part(node.part);
                let tone = scroll_area_tone_for_node(&node);
                BevyUiPrimitive {
                    part: scroll_area_primitive_part(&node),
                    kind: scroll_area_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: scroll_area_size_for_part(node.part, node.axis),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: scroll_area_intent_for_part(node.part, node.actionable),
                    selected: node.active || node.focused || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_select(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_select_model();
        let mut select_state = model.state();
        let _ = select_state.apply(crate::SelectIntent::Open);
        select_render_nodes(&model, &select_state)
            .into_iter()
            .map(|node| {
                let role = select_role_for_part(node.part);
                let tone = select_tone_for_node(&node);
                BevyUiPrimitive {
                    part: select_primitive_part(&node),
                    kind: select_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: select_size_for_part(node.part),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: select_intent_for_part(node.part, node.actionable),
                    selected: node.selected || node.focused || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_separator(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_separator_model();
        let separator_state = model.state();
        separator_render_nodes(&model, &separator_state)
            .into_iter()
            .map(|node| {
                let role = separator_role_for_part(node.part);
                let tone = separator_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: separator_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: separator_size_for_part(node.part, node.orientation),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: separator_intent_for_part(node.actionable),
                    selected: node.active || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_sheet(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_sheet_model();
        let mut sheet_state = model.state();
        let _ = sheet_state.apply(crate::SheetIntent::Open);
        sheet_render_nodes(&model, &sheet_state)
            .into_iter()
            .map(|node| {
                let role = sheet_role_for_part(node.part);
                let tone = sheet_tone_for_node(&node);
                BevyUiPrimitive {
                    part: sheet_primitive_part(&node),
                    kind: sheet_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: sheet_size_for_part(node.part, node.side),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: sheet_intent_for_part(node.part, node.actionable),
                    selected: node.selected || node.focused,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_sidebar(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_sidebar_model();
        let sidebar_state = model.state();
        sidebar_render_nodes(&model, &sidebar_state)
            .into_iter()
            .map(|node| {
                let role = sidebar_role_for_part(node.part);
                let tone = sidebar_tone_for_node(&node);
                BevyUiPrimitive {
                    part: sidebar_primitive_part(&node),
                    kind: sidebar_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: sidebar_size_for_part(node.part, node.collapsed),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: sidebar_intent_for_part(node.part, node.actionable),
                    selected: node.selected || node.focused,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_skeleton(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_skeleton_model();
        let skeleton_state = model.state();
        skeleton_render_nodes(&model, &skeleton_state)
            .into_iter()
            .map(|node| {
                let role = skeleton_role_for_part(node.part);
                let tone = skeleton_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: skeleton_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: skeleton_size_for_part(node.part, node.text_lines),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: skeleton_intent_for_part(node.part, node.loading, node.disabled),
                    selected: node.active || node.animation_paused || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_slider(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_slider_model();
        let slider_state = model.state();
        slider_render_nodes(&model, &slider_state)
            .into_iter()
            .map(|node| {
                let role = slider_role_for_part(node.part);
                let tone = slider_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: slider_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: slider_size_for_part(node.part, node.orientation, node.percent),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: slider_intent_for_part(node.part, node.actionable),
                    selected: node.focused || node.dragging || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_sonner(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_sonner_model();
        let sonner_state = model.state();
        sonner_render_nodes(&model, &sonner_state)
            .into_iter()
            .map(|node| {
                let role = sonner_role_for_part(node.part);
                let tone = sonner_tone_for_node(&node);
                BevyUiPrimitive {
                    part: sonner_primitive_part(&node),
                    kind: sonner_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: sonner_size_for_part(node.part, node.density),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: sonner_intent_for_part(node.part, node.actionable),
                    selected: node.active || node.actioned || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_spinner(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_spinner_model();
        let spinner_state = model.state();
        spinner_render_nodes(&model, &spinner_state)
            .into_iter()
            .map(|node| {
                let role = spinner_role_for_part(node.part);
                let tone = spinner_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: spinner_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: spinner_size_for_part(node.part, node.size),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: spinner_intent_for_part(node.part, node.actionable),
                    selected: node.active || node.paused || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_switch(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_switch_model();
        let switch_state = model.state();
        switch_render_nodes(&model, &switch_state)
            .into_iter()
            .map(|node| {
                let role = switch_role_for_part(node.part);
                let tone = switch_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: switch_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: switch_size_for_part(node.part, node.density),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: switch_intent_for_part(node.part, node.actionable),
                    selected: node.checked.is_on() || node.active || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_toggle(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_toggle_model();
        let toggle_state = model.state();
        toggle_render_nodes(&model, &toggle_state)
            .into_iter()
            .map(|node| {
                let role = toggle_role_for_part(node.part);
                let tone = toggle_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: toggle_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: toggle_size_for_part(node.part, node.density),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: toggle_intent_for_part(node.part, node.actionable),
                    selected: node.pressed.is_pressed() || node.active || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_toggle_group(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_toggle_group_model();
        let toggle_group_state = model.state();
        toggle_group_render_nodes(&model, &toggle_group_state)
            .into_iter()
            .map(|node| {
                let role = toggle_group_role_for_part(node.part);
                let tone = toggle_group_tone_for_node(&node);
                BevyUiPrimitive {
                    part: toggle_group_primitive_part(&node),
                    kind: toggle_group_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: toggle_group_size_for_part(node.part, node.density),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: toggle_group_intent_for_part(node.part, node.actionable),
                    selected: node.pressed.is_pressed() || node.focused || node.invalid,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_tooltip(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_tooltip_model();
        let tooltip_state = model.state();
        tooltip_render_nodes(&model, &tooltip_state)
            .into_iter()
            .map(|node| {
                let role = tooltip_role_for_part(node.part);
                let tone = tooltip_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: tooltip_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: tooltip_size_for_part(node.part, node.density),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: tooltip_intent_for_part(node.part, node.actionable),
                    selected: node.open || node.active || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_typography(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_typography_model();
        let typography_state = model.state();
        typography_render_nodes(&model, &typography_state)
            .into_iter()
            .map(|node| {
                let role = typography_role_for_part(node.part);
                let tone = typography_tone_for_node(&node);
                BevyUiPrimitive {
                    part: typography_primitive_part(&node),
                    kind: typography_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: typography_size_for_part(node.part, node.density, node.index.is_some()),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: typography_intent_for_part(node.part, node.disabled),
                    selected: node.active || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_table(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_table_model();
        let table_state = model.state();
        table_render_nodes(&model, &table_state)
            .into_iter()
            .map(|node| {
                let role = table_role_for_part(node.part);
                let tone = table_tone_for_node(&node);
                BevyUiPrimitive {
                    part: table_primitive_part(&node),
                    kind: table_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: table_size_for_part(node.part, node.density),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: table_intent_for_part(node.part, node.actionable),
                    selected: node.selected || node.active || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_tabs(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_tabs_model();
        let tabs_state = model.state();
        tabs_render_nodes(&model, &tabs_state)
            .into_iter()
            .map(|node| {
                let role = tabs_role_for_part(node.part);
                let tone = tabs_tone_for_node(&node);
                BevyUiPrimitive {
                    part: tabs_primitive_part(&node),
                    kind: tabs_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: tabs_size_for_part(node.part, node.density, node.orientation),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: tabs_intent_for_part(node.part, node.actionable),
                    selected: node.selected || node.focused || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_textarea(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_textarea_model();
        let textarea_state = model.state();
        textarea_render_nodes(&model, &textarea_state)
            .into_iter()
            .map(|node| {
                let role = textarea_role_for_part(node.part);
                let tone = textarea_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: textarea_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: textarea_size_for_part(node.part, node.density, node.rows),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: textarea_intent_for_part(node.part, node.actionable),
                    selected: node.focused || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_toast(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_toast_model();
        let toast_state = model.state();
        toast_render_nodes(&model, &toast_state)
            .into_iter()
            .map(|node| {
                let role = toast_role_for_part(node.part);
                let tone = toast_tone_for_node(&node);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: toast_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: toast_size_for_part(node.part, node.density),
                    fill: fill_for_tone(tone, theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: toast_intent_for_part(node.part, node.actionable),
                    selected: node.focused || node.paused || node.actioned || node.invalid,
                    disabled: node.disabled || !node.visible,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_data_table(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_data_table_model();
        let data_table_state = model.state();
        data_table_render_nodes(&model, &data_table_state)
            .into_iter()
            .map(|node| {
                let role = data_table_role_for_part(node.part);
                BevyUiPrimitive {
                    part: data_table_primitive_part(&node),
                    kind: data_table_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: data_table_size_for_part(node.part),
                    fill: fill_for_tone(
                        data_table_tone_for_part(node.part, node.selected, node.disabled),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: data_table_intent_for_part(node.part),
                    selected: node.selected,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_date_picker(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_date_picker_model().with_default_open(true);
        let date_picker_state = model.state();
        date_picker_render_nodes(&model, &date_picker_state)
            .into_iter()
            .map(|node| {
                let role = date_picker_role_for_part(node.part);
                BevyUiPrimitive {
                    part: date_picker_primitive_part(&node),
                    kind: date_picker_kind_for_part(node.part, node.date.is_some()),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: date_picker_size_for_part(node.part, node.date.is_some()),
                    fill: fill_for_tone(
                        date_picker_tone_for_part(
                            node.part,
                            node.selected,
                            node.open,
                            node.current_month,
                            node.disabled,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: date_picker_intent_for_part(node.part, node.date.is_some()),
                    selected: node.selected || node.open,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_dialog(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_dialog_model().with_default_open(true);
        let dialog_state = model.state();
        dialog_render_nodes(&model, &dialog_state)
            .into_iter()
            .map(|node| {
                let role = dialog_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: dialog_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: dialog_size_for_part(node.part),
                    fill: fill_for_tone(
                        dialog_tone_for_part(node.part, node.selected, node.open, node.disabled),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: dialog_intent_for_part(node.part, node.actionable, node.close_dialog),
                    selected: node.selected || node.open,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_direction(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_direction_model().with_default_scope_active(true);
        let direction_state = model.state();
        direction_render_nodes(&model, &direction_state)
            .into_iter()
            .map(|node| {
                let role = direction_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: direction_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: direction_size_for_part(node.part),
                    fill: fill_for_tone(
                        direction_tone_for_part(
                            node.part,
                            node.direction,
                            node.scope_active,
                            node.disabled,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: direction_intent_for_part(node.part),
                    selected: node.selected || node.scope_active,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_drawer(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_drawer_model().with_default_open(true);
        let drawer_state = model.state();
        drawer_render_nodes(&model, &drawer_state)
            .into_iter()
            .map(|node| {
                let role = drawer_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: drawer_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: drawer_size_for_part(node.part, node.side),
                    fill: fill_for_tone(
                        drawer_tone_for_part(
                            node.part,
                            node.open,
                            node.dragging,
                            node.selected,
                            node.disabled,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: drawer_intent_for_part(node.part, node.actionable, node.close_drawer),
                    selected: node.selected || node.open || node.dragging,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_chart(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_chart_model();
        let chart_state = model.state();
        chart_render_nodes(&model, &chart_state)
            .into_iter()
            .map(|node| {
                let role = chart_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: chart_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: chart_size_for_part(node.part),
                    fill: fill_for_tone(chart_tone_for_part(node.part, node.tone), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: chart_intent_for_part(node.part),
                    selected: node.selected,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_carousel(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_carousel_model();
        let carousel_state = model.state();
        carousel_render_nodes(&model, &carousel_state)
            .into_iter()
            .map(|node| {
                let role = carousel_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: carousel_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: carousel_size_for_part(node.part),
                    fill: fill_for_tone(carousel_tone_for_part(node.part, node.selected), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: carousel_intent_for_part(node.part),
                    selected: node.selected,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_card(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_card_model();
        let card_state = model.state();
        card_render_nodes(&model, &card_state)
            .into_iter()
            .map(|node| {
                let role = card_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: card_kind_for_part(node.part, node.actionable),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: card_size_for_part(node.part),
                    fill: fill_for_tone(
                        card_tone_for_part(node.part, node.variant, node.active),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: card_intent_for_part(node.part, node.actionable),
                    selected: node.active,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_calendar(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_calendar_model();
        let calendar_state = model.state();
        calendar_render_nodes(&model, &calendar_state)
            .into_iter()
            .map(|node| {
                let role = calendar_role_for_part(node.part);
                BevyUiPrimitive {
                    part: calendar_primitive_part(&node),
                    kind: calendar_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: calendar_size_for_part(node.part),
                    fill: fill_for_tone(
                        calendar_tone_for_part(
                            node.part,
                            node.mode,
                            node.selected,
                            node.in_range,
                            node.current_month,
                        ),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: calendar_intent_for_part(node.part, node.mode),
                    selected: node.selected || node.in_range,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_button_group(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_button_group_model();
        let button_group_state = model.state();
        button_group_render_nodes(&model, &button_group_state)
            .into_iter()
            .map(|node| {
                let role = button_group_role_for_part(node.part);
                BevyUiPrimitive {
                    part: button_group_primitive_part(&node),
                    kind: button_group_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: button_group_size_for_part(node.part, node.size, node.orientation),
                    fill: fill_for_tone(
                        button_group_tone_for_part(node.part, node.variant, node.selected),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: button_group_intent_for_part(node.part),
                    selected: node.selected,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_button(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_button_model();
        button_render_nodes(&model, model.state())
            .into_iter()
            .map(|node| {
                let role = button_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: button_kind_for_part(node.part, node.kind),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: button_size_for_part(node.part, node.size),
                    fill: fill_for_tone(button_tone_for_part(node.part, node.variant), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: button_intent_for_part(node.part, node.kind),
                    selected: node.pressed,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_bubble(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_bubble_model();
        let bubble_state = model.state();
        bubble_render_nodes(&model, &bubble_state)
            .into_iter()
            .map(|node| {
                let role = bubble_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: bubble_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: bubble_size_for_part(node.part),
                    fill: fill_for_tone(bubble_tone_for_part(node.part, node.side), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: bubble_intent_for_part(node.part),
                    selected: node.active,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_breadcrumb(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_breadcrumb_model();
        let breadcrumb_state = model.state();
        breadcrumb_render_nodes(&model, &breadcrumb_state)
            .into_iter()
            .map(|node| {
                let role = breadcrumb_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: breadcrumb_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: breadcrumb_size_for_part(node.part),
                    fill: fill_for_tone(breadcrumb_tone_for_part(node.part, node.current), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: breadcrumb_intent_for_part(node.part),
                    selected: node.current || node.active,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_badge(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_badge_model();
        badge_render_nodes(&model, model.state())
            .into_iter()
            .map(|node| {
                let role = badge_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: badge_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: badge_size_for_part(node.part),
                    fill: fill_for_tone(badge_tone(node.tone), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: UiWidgetIntent::None,
                    selected: node.highlighted,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_avatar(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = default_avatar_model();
        avatar_render_nodes(&model, model.state())
            .into_iter()
            .map(|node| {
                let role = avatar_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: avatar_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: avatar_size_for_part(node.part),
                    fill: fill_for_tone(avatar_tone_for_part(node.part), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: avatar_intent_for_part(node.part),
                    selected: node.visual == AvatarVisual::Image,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_attachment(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        attachment_render_nodes(&default_attachment_model())
            .into_iter()
            .map(|node| {
                let role = attachment_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: attachment_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: size_for_role(role),
                    fill: fill_for_tone(attachment_tone_for_part(node.part), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: attachment_intent_for_part(node.part),
                    selected: node.loading,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_aspect_ratio(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        aspect_ratio_render_nodes(&default_aspect_ratio_model())
            .into_iter()
            .map(|node| {
                let role = aspect_ratio_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: aspect_ratio_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: aspect_ratio_size(node.width, node.height, role),
                    fill: fill_for_tone(aspect_ratio_tone_for_part(node.part), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: UiWidgetIntent::None,
                    selected: node.loading,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_alert_dialog(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        alert_dialog_render_nodes(&default_alert_dialog_model(), AlertDialogState::open())
            .into_iter()
            .map(|node| {
                let role = alert_dialog_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: alert_dialog_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: size_for_role(role),
                    fill: fill_for_tone(
                        alert_dialog_tone_for_part(node.part, node.destructive),
                        theme,
                    ),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: alert_dialog_intent_for_part(node.part),
                    selected: node.open,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_alert(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        alert_render_nodes(&default_alert_model())
            .into_iter()
            .map(|node| {
                let role = alert_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: alert_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: size_for_role(role),
                    fill: fill_for_tone(alert_tone_for_part(node.part, node.tone), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: alert_intent_for_part(node.part),
                    selected: node.loading,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    fn bevy_primitives_for_accordion(
        theme: &Theme,
        render: RenderContract,
        state: StateContract,
    ) -> Vec<BevyUiPrimitive> {
        let model = AccordionModel::new(AccordionMode::Single, default_accordion_items())
            .with_default_open(vec!["tokens".to_owned()]);
        let accordion_state = model.state();
        accordion_render_nodes(&model, &accordion_state)
            .into_iter()
            .map(|node| {
                let role = accordion_role_for_part(node.part);
                BevyUiPrimitive {
                    part: node.part.label().to_owned(),
                    kind: accordion_kind_for_part(node.part),
                    role,
                    label: node.label,
                    value: node.detail,
                    size: size_for_role(role),
                    fill: fill_for_tone(accordion_tone_for_part(node.part), theme),
                    text: theme.text_1().to_bevy(),
                    render,
                    state,
                    intent: accordion_intent_for_part(node.part),
                    selected: node.open,
                    disabled: node.disabled,
                }
            })
            .collect()
    }

    const fn alert_dialog_kind_for_part(part: AlertDialogPart) -> UiWidgetSlotKind {
        match part {
            AlertDialogPart::Root => UiWidgetSlotKind::Section,
            AlertDialogPart::Trigger => UiWidgetSlotKind::Button,
            AlertDialogPart::Content => UiWidgetSlotKind::Overlay,
            AlertDialogPart::Header => UiWidgetSlotKind::Header,
            AlertDialogPart::Footer => UiWidgetSlotKind::Panel,
            AlertDialogPart::Action | AlertDialogPart::Cancel => UiWidgetSlotKind::Button,
        }
    }

    const fn aspect_ratio_kind_for_part(part: AspectRatioPart) -> UiWidgetSlotKind {
        match part {
            AspectRatioPart::Root => UiWidgetSlotKind::Section,
            AspectRatioPart::Frame => UiWidgetSlotKind::Panel,
            AspectRatioPart::Media => UiWidgetSlotKind::Media,
        }
    }

    const fn aspect_ratio_role_for_part(part: AspectRatioPart) -> UiBlockRole {
        match part {
            AspectRatioPart::Root => UiBlockRole::Root,
            AspectRatioPart::Frame => UiBlockRole::Layout,
            AspectRatioPart::Media => UiBlockRole::Media,
        }
    }

    const fn aspect_ratio_tone_for_part(part: AspectRatioPart) -> UiBlockTone {
        match part {
            AspectRatioPart::Root | AspectRatioPart::Frame => UiBlockTone::Surface,
            AspectRatioPart::Media => UiBlockTone::Success,
        }
    }

    const fn attachment_kind_for_part(part: AttachmentPart) -> UiWidgetSlotKind {
        match part {
            AttachmentPart::Root => UiWidgetSlotKind::Section,
            AttachmentPart::Preview => UiWidgetSlotKind::Media,
            AttachmentPart::Title => UiWidgetSlotKind::Title,
            AttachmentPart::Meta => UiWidgetSlotKind::Text,
            AttachmentPart::Action => UiWidgetSlotKind::Button,
        }
    }

    const fn attachment_role_for_part(part: AttachmentPart) -> UiBlockRole {
        match part {
            AttachmentPart::Root => UiBlockRole::Root,
            AttachmentPart::Preview => UiBlockRole::Media,
            AttachmentPart::Title => UiBlockRole::Header,
            AttachmentPart::Meta => UiBlockRole::Text,
            AttachmentPart::Action => UiBlockRole::Action,
        }
    }

    const fn attachment_tone_for_part(part: AttachmentPart) -> UiBlockTone {
        match part {
            AttachmentPart::Root | AttachmentPart::Title | AttachmentPart::Meta => {
                UiBlockTone::Surface
            }
            AttachmentPart::Preview => UiBlockTone::Info,
            AttachmentPart::Action => UiBlockTone::Brand,
        }
    }

    const fn attachment_intent_for_part(part: AttachmentPart) -> UiWidgetIntent {
        match part {
            AttachmentPart::Action => UiWidgetIntent::Activate,
            AttachmentPart::Root
            | AttachmentPart::Preview
            | AttachmentPart::Title
            | AttachmentPart::Meta => UiWidgetIntent::None,
        }
    }

    const fn avatar_kind_for_part(part: AvatarPart) -> UiWidgetSlotKind {
        match part {
            AvatarPart::Root => UiWidgetSlotKind::Section,
            AvatarPart::Image => UiWidgetSlotKind::Avatar,
            AvatarPart::Fallback => UiWidgetSlotKind::Text,
        }
    }

    const fn avatar_role_for_part(part: AvatarPart) -> UiBlockRole {
        match part {
            AvatarPart::Root => UiBlockRole::Root,
            AvatarPart::Image => UiBlockRole::Media,
            AvatarPart::Fallback => UiBlockRole::Text,
        }
    }

    const fn avatar_tone_for_part(part: AvatarPart) -> UiBlockTone {
        match part {
            AvatarPart::Root | AvatarPart::Fallback => UiBlockTone::Surface,
            AvatarPart::Image => UiBlockTone::Success,
        }
    }

    const fn avatar_intent_for_part(part: AvatarPart) -> UiWidgetIntent {
        match part {
            AvatarPart::Image => UiWidgetIntent::None,
            AvatarPart::Root | AvatarPart::Fallback => UiWidgetIntent::None,
        }
    }

    fn avatar_size_for_part(part: AvatarPart) -> Vec2 {
        match part {
            AvatarPart::Root | AvatarPart::Image | AvatarPart::Fallback => {
                Vec2::new(scale::space::XL, scale::space::XL)
            }
        }
    }

    const fn badge_kind_for_part(part: BadgePart) -> UiWidgetSlotKind {
        match part {
            BadgePart::Root => UiWidgetSlotKind::Section,
            BadgePart::Icon => UiWidgetSlotKind::Marker,
            BadgePart::Text => UiWidgetSlotKind::Badge,
        }
    }

    const fn badge_role_for_part(part: BadgePart) -> UiBlockRole {
        match part {
            BadgePart::Root => UiBlockRole::Root,
            BadgePart::Icon => UiBlockRole::Indicator,
            BadgePart::Text => UiBlockRole::Text,
        }
    }

    const fn badge_tone(tone: BadgeTone) -> UiBlockTone {
        match tone {
            BadgeTone::Default => UiBlockTone::Surface,
            BadgeTone::Brand => UiBlockTone::Brand,
            BadgeTone::Info => UiBlockTone::Info,
            BadgeTone::Success => UiBlockTone::Success,
            BadgeTone::Warning => UiBlockTone::Warning,
            BadgeTone::Destructive => UiBlockTone::Danger,
            BadgeTone::Muted => UiBlockTone::Muted,
        }
    }

    fn badge_size_for_part(part: BadgePart) -> Vec2 {
        match part {
            BadgePart::Root => Vec2::new(scale::space::XL, scale::space::M),
            BadgePart::Icon => Vec2::new(scale::space::S, scale::space::S),
            BadgePart::Text => Vec2::new(scale::space::L, scale::space::S),
        }
    }

    const fn breadcrumb_kind_for_part(part: BreadcrumbPart) -> UiWidgetSlotKind {
        match part {
            BreadcrumbPart::Root => UiWidgetSlotKind::Section,
            BreadcrumbPart::List => UiWidgetSlotKind::List,
            BreadcrumbPart::Item => UiWidgetSlotKind::ListItem,
            BreadcrumbPart::Link => UiWidgetSlotKind::Link,
            BreadcrumbPart::Separator => UiWidgetSlotKind::Separator,
            BreadcrumbPart::Page => UiWidgetSlotKind::Text,
        }
    }

    const fn breadcrumb_role_for_part(part: BreadcrumbPart) -> UiBlockRole {
        match part {
            BreadcrumbPart::Root | BreadcrumbPart::List => UiBlockRole::Navigation,
            BreadcrumbPart::Item => UiBlockRole::Item,
            BreadcrumbPart::Link => UiBlockRole::Action,
            BreadcrumbPart::Separator => UiBlockRole::Indicator,
            BreadcrumbPart::Page => UiBlockRole::Text,
        }
    }

    const fn breadcrumb_tone_for_part(part: BreadcrumbPart, current: bool) -> UiBlockTone {
        match part {
            BreadcrumbPart::Root | BreadcrumbPart::List | BreadcrumbPart::Item => {
                UiBlockTone::Surface
            }
            BreadcrumbPart::Link => UiBlockTone::Brand,
            BreadcrumbPart::Separator => UiBlockTone::Muted,
            BreadcrumbPart::Page if current => UiBlockTone::Accent,
            BreadcrumbPart::Page => UiBlockTone::Surface,
        }
    }

    const fn breadcrumb_intent_for_part(part: BreadcrumbPart) -> UiWidgetIntent {
        match part {
            BreadcrumbPart::Link => UiWidgetIntent::Navigate,
            BreadcrumbPart::Root
            | BreadcrumbPart::List
            | BreadcrumbPart::Item
            | BreadcrumbPart::Separator
            | BreadcrumbPart::Page => UiWidgetIntent::None,
        }
    }

    fn breadcrumb_size_for_part(part: BreadcrumbPart) -> Vec2 {
        match part {
            BreadcrumbPart::Root | BreadcrumbPart::List => {
                Vec2::new(scale::space::XL, scale::space::M)
            }
            BreadcrumbPart::Item | BreadcrumbPart::Link | BreadcrumbPart::Page => {
                Vec2::new(scale::space::L, scale::space::S)
            }
            BreadcrumbPart::Separator => Vec2::new(scale::space::S, scale::space::S),
        }
    }

    const fn bubble_kind_for_part(part: BubblePart) -> UiWidgetSlotKind {
        match part {
            BubblePart::Root => UiWidgetSlotKind::Section,
            BubblePart::Avatar => UiWidgetSlotKind::Avatar,
            BubblePart::Content => UiWidgetSlotKind::Description,
            BubblePart::Meta => UiWidgetSlotKind::Text,
            BubblePart::Actions => UiWidgetSlotKind::Button,
        }
    }

    const fn bubble_role_for_part(part: BubblePart) -> UiBlockRole {
        match part {
            BubblePart::Root => UiBlockRole::Root,
            BubblePart::Avatar => UiBlockRole::Media,
            BubblePart::Content => UiBlockRole::Content,
            BubblePart::Meta => UiBlockRole::Text,
            BubblePart::Actions => UiBlockRole::Action,
        }
    }

    const fn bubble_tone_for_part(part: BubblePart, side: BubbleSide) -> UiBlockTone {
        match part {
            BubblePart::Avatar => UiBlockTone::Accent,
            BubblePart::Actions => UiBlockTone::Brand,
            BubblePart::Root | BubblePart::Content | BubblePart::Meta => match side {
                BubbleSide::Incoming => UiBlockTone::Surface,
                BubbleSide::Outgoing => UiBlockTone::Brand,
                BubbleSide::System => UiBlockTone::Muted,
            },
        }
    }

    const fn bubble_intent_for_part(part: BubblePart) -> UiWidgetIntent {
        match part {
            BubblePart::Actions => UiWidgetIntent::Activate,
            BubblePart::Root | BubblePart::Avatar | BubblePart::Content | BubblePart::Meta => {
                UiWidgetIntent::None
            }
        }
    }

    fn bubble_size_for_part(part: BubblePart) -> Vec2 {
        match part {
            BubblePart::Root | BubblePart::Content => Vec2::new(scale::space::XL, scale::space::L),
            BubblePart::Avatar => Vec2::new(scale::space::L, scale::space::L),
            BubblePart::Meta | BubblePart::Actions => Vec2::new(scale::space::L, scale::space::S),
        }
    }

    const fn button_kind_for_part(part: ButtonPart, kind: ButtonKind) -> UiWidgetSlotKind {
        match part {
            ButtonPart::Root => UiWidgetSlotKind::Section,
            ButtonPart::Icon => UiWidgetSlotKind::Marker,
            ButtonPart::Label => match kind {
                ButtonKind::Link => UiWidgetSlotKind::Link,
                ButtonKind::Button | ButtonKind::Submit | ButtonKind::Reset => {
                    UiWidgetSlotKind::Button
                }
            },
        }
    }

    const fn button_role_for_part(part: ButtonPart) -> UiBlockRole {
        match part {
            ButtonPart::Root => UiBlockRole::Root,
            ButtonPart::Icon => UiBlockRole::Indicator,
            ButtonPart::Label => UiBlockRole::Action,
        }
    }

    const fn button_tone_for_part(part: ButtonPart, variant: ButtonVariant) -> UiBlockTone {
        match part {
            ButtonPart::Icon => UiBlockTone::Accent,
            ButtonPart::Root | ButtonPart::Label => match variant {
                ButtonVariant::Primary | ButtonVariant::Link => UiBlockTone::Brand,
                ButtonVariant::Secondary | ButtonVariant::Outline | ButtonVariant::Ghost => {
                    UiBlockTone::Surface
                }
                ButtonVariant::Destructive => UiBlockTone::Danger,
            },
        }
    }

    const fn button_intent_for_part(part: ButtonPart, kind: ButtonKind) -> UiWidgetIntent {
        match part {
            ButtonPart::Root | ButtonPart::Label => match kind {
                ButtonKind::Link => UiWidgetIntent::Navigate,
                ButtonKind::Button | ButtonKind::Submit | ButtonKind::Reset => {
                    UiWidgetIntent::Activate
                }
            },
            ButtonPart::Icon => UiWidgetIntent::None,
        }
    }

    fn button_size_for_part(part: ButtonPart, size: ButtonSize) -> Vec2 {
        match part {
            ButtonPart::Root => match size {
                ButtonSize::Small => Vec2::new(scale::space::XL, scale::space::S),
                ButtonSize::Medium => Vec2::new(scale::space::XL2, scale::space::L),
                ButtonSize::Large => Vec2::new(scale::space::XL3, scale::space::XL),
                ButtonSize::Icon => Vec2::new(scale::space::L, scale::space::L),
            },
            ButtonPart::Icon => Vec2::new(scale::space::S, scale::space::S),
            ButtonPart::Label => Vec2::new(scale::space::XL, scale::space::S),
        }
    }

    fn button_group_primitive_part(node: &crate::ButtonGroupRenderNode) -> String {
        match node.part {
            ButtonGroupPart::Item | ButtonGroupPart::Separator => {
                format!("{}:{}", node.part.label(), node.value)
            }
            ButtonGroupPart::Root => node.part.label().to_owned(),
        }
    }

    const fn button_group_kind_for_part(part: ButtonGroupPart) -> UiWidgetSlotKind {
        match part {
            ButtonGroupPart::Root => UiWidgetSlotKind::Section,
            ButtonGroupPart::Item => UiWidgetSlotKind::Button,
            ButtonGroupPart::Separator => UiWidgetSlotKind::Separator,
        }
    }

    const fn button_group_role_for_part(part: ButtonGroupPart) -> UiBlockRole {
        match part {
            ButtonGroupPart::Root => UiBlockRole::Root,
            ButtonGroupPart::Item => UiBlockRole::Action,
            ButtonGroupPart::Separator => UiBlockRole::Layout,
        }
    }

    const fn button_group_tone_for_part(
        part: ButtonGroupPart,
        variant: ButtonVariant,
        selected: bool,
    ) -> UiBlockTone {
        match part {
            ButtonGroupPart::Root | ButtonGroupPart::Separator => UiBlockTone::Surface,
            ButtonGroupPart::Item if !selected => UiBlockTone::Surface,
            ButtonGroupPart::Item => match variant {
                ButtonVariant::Primary | ButtonVariant::Link => UiBlockTone::Brand,
                ButtonVariant::Secondary | ButtonVariant::Outline | ButtonVariant::Ghost => {
                    UiBlockTone::Accent
                }
                ButtonVariant::Destructive => UiBlockTone::Danger,
            },
        }
    }

    const fn button_group_intent_for_part(part: ButtonGroupPart) -> UiWidgetIntent {
        match part {
            ButtonGroupPart::Item => UiWidgetIntent::Select,
            ButtonGroupPart::Root | ButtonGroupPart::Separator => UiWidgetIntent::None,
        }
    }

    fn button_group_size_for_part(
        part: ButtonGroupPart,
        size: ButtonSize,
        orientation: ButtonGroupOrientation,
    ) -> Vec2 {
        match part {
            ButtonGroupPart::Root => match orientation {
                ButtonGroupOrientation::Horizontal => Vec2::new(scale::space::XL3, scale::space::L),
                ButtonGroupOrientation::Vertical => Vec2::new(scale::space::XL2, scale::space::XL2),
            },
            ButtonGroupPart::Item => match size {
                ButtonSize::Small => Vec2::new(scale::space::L, scale::space::S),
                ButtonSize::Medium => Vec2::new(scale::space::XL, scale::space::L),
                ButtonSize::Large => Vec2::new(scale::space::XL2, scale::space::XL),
                ButtonSize::Icon => Vec2::new(scale::space::L, scale::space::L),
            },
            ButtonGroupPart::Separator => match orientation {
                ButtonGroupOrientation::Horizontal => Vec2::new(scale::space::XS3, scale::space::L),
                ButtonGroupOrientation::Vertical => Vec2::new(scale::space::XL, scale::space::XS3),
            },
        }
    }

    fn calendar_primitive_part(node: &crate::CalendarRenderNode) -> String {
        match node.part {
            CalendarPart::Day => {
                let value = node
                    .date
                    .map_or_else(|| node.index.to_string(), crate::CalendarDate::value);
                format!("{}:{value}", node.part.label())
            }
            CalendarPart::Root
            | CalendarPart::Header
            | CalendarPart::Grid
            | CalendarPart::Range => node.part.label().to_owned(),
        }
    }

    const fn card_kind_for_part(part: CardPart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            CardPart::Root => UiWidgetSlotKind::Section,
            CardPart::Header => UiWidgetSlotKind::Header,
            CardPart::Title => UiWidgetSlotKind::Title,
            CardPart::Description => UiWidgetSlotKind::Description,
            CardPart::Content => UiWidgetSlotKind::Panel,
            CardPart::Footer if actionable => UiWidgetSlotKind::Button,
            CardPart::Footer => UiWidgetSlotKind::Panel,
        }
    }

    const fn card_role_for_part(part: CardPart) -> UiBlockRole {
        match part {
            CardPart::Root => UiBlockRole::Root,
            CardPart::Header | CardPart::Title => UiBlockRole::Header,
            CardPart::Description | CardPart::Content => UiBlockRole::Content,
            CardPart::Footer => UiBlockRole::Action,
        }
    }

    const fn card_tone_for_part(part: CardPart, variant: CardVariant, active: bool) -> UiBlockTone {
        match part {
            CardPart::Footer if active => UiBlockTone::Brand,
            CardPart::Footer => UiBlockTone::Accent,
            CardPart::Root => match variant {
                CardVariant::Elevated | CardVariant::Outline => UiBlockTone::Surface,
                CardVariant::Ghost => UiBlockTone::Muted,
            },
            CardPart::Header | CardPart::Title | CardPart::Description | CardPart::Content => {
                UiBlockTone::Surface
            }
        }
    }

    const fn card_intent_for_part(part: CardPart, actionable: bool) -> UiWidgetIntent {
        match part {
            CardPart::Footer if actionable => UiWidgetIntent::Activate,
            CardPart::Root
            | CardPart::Header
            | CardPart::Title
            | CardPart::Description
            | CardPart::Content
            | CardPart::Footer => UiWidgetIntent::None,
        }
    }

    fn card_size_for_part(part: CardPart) -> Vec2 {
        match part {
            CardPart::Root => Vec2::new(scale::space::XL3, scale::space::XL2),
            CardPart::Header => Vec2::new(scale::space::XL2, scale::space::L),
            CardPart::Title => Vec2::new(scale::space::XL2, scale::space::S),
            CardPart::Description => Vec2::new(scale::space::XL2, scale::space::S),
            CardPart::Content => Vec2::new(scale::space::XL2, scale::space::XL),
            CardPart::Footer => Vec2::new(scale::space::XL2, scale::space::L),
        }
    }

    const fn carousel_kind_for_part(part: CarouselPart) -> UiWidgetSlotKind {
        match part {
            CarouselPart::Root => UiWidgetSlotKind::Section,
            CarouselPart::Content => UiWidgetSlotKind::List,
            CarouselPart::Item => UiWidgetSlotKind::Media,
            CarouselPart::Previous | CarouselPart::Next => UiWidgetSlotKind::IconButton,
            CarouselPart::Indicator => UiWidgetSlotKind::Marker,
        }
    }

    const fn carousel_role_for_part(part: CarouselPart) -> UiBlockRole {
        match part {
            CarouselPart::Root => UiBlockRole::Root,
            CarouselPart::Content => UiBlockRole::Layout,
            CarouselPart::Item => UiBlockRole::Media,
            CarouselPart::Previous | CarouselPart::Next => UiBlockRole::Action,
            CarouselPart::Indicator => UiBlockRole::Indicator,
        }
    }

    const fn carousel_tone_for_part(part: CarouselPart, selected: bool) -> UiBlockTone {
        match part {
            CarouselPart::Item if selected => UiBlockTone::Brand,
            CarouselPart::Previous | CarouselPart::Next => UiBlockTone::Brand,
            CarouselPart::Indicator if selected => UiBlockTone::Accent,
            CarouselPart::Root | CarouselPart::Content | CarouselPart::Item => UiBlockTone::Surface,
            CarouselPart::Indicator => UiBlockTone::Muted,
        }
    }

    const fn carousel_intent_for_part(part: CarouselPart) -> UiWidgetIntent {
        match part {
            CarouselPart::Previous | CarouselPart::Next => UiWidgetIntent::Navigate,
            CarouselPart::Item => UiWidgetIntent::Select,
            CarouselPart::Root | CarouselPart::Content | CarouselPart::Indicator => {
                UiWidgetIntent::None
            }
        }
    }

    fn carousel_size_for_part(part: CarouselPart) -> Vec2 {
        match part {
            CarouselPart::Root | CarouselPart::Content => {
                Vec2::new(scale::space::XL3, scale::space::XL2)
            }
            CarouselPart::Item => Vec2::new(scale::space::XL2, scale::space::XL),
            CarouselPart::Previous | CarouselPart::Next => {
                Vec2::new(scale::space::L, scale::space::L)
            }
            CarouselPart::Indicator => Vec2::new(scale::space::XL, scale::space::S),
        }
    }

    const fn chart_kind_for_part(part: ChartPart) -> UiWidgetSlotKind {
        match part {
            ChartPart::Container => UiWidgetSlotKind::Section,
            ChartPart::Series => UiWidgetSlotKind::Chart,
            ChartPart::Legend => UiWidgetSlotKind::Badge,
            ChartPart::Tooltip => UiWidgetSlotKind::Text,
            ChartPart::Axis => UiWidgetSlotKind::Separator,
        }
    }

    const fn chart_role_for_part(part: ChartPart) -> UiBlockRole {
        match part {
            ChartPart::Container => UiBlockRole::Root,
            ChartPart::Series => UiBlockRole::Data,
            ChartPart::Legend => UiBlockRole::Indicator,
            ChartPart::Tooltip => UiBlockRole::Text,
            ChartPart::Axis => UiBlockRole::Layout,
        }
    }

    const fn chart_tone_for_part(part: ChartPart, tone: ChartTone) -> UiBlockTone {
        match part {
            ChartPart::Container | ChartPart::Axis | ChartPart::Tooltip => UiBlockTone::Surface,
            ChartPart::Legend | ChartPart::Series => match tone {
                ChartTone::Brand => UiBlockTone::Brand,
                ChartTone::Info => UiBlockTone::Info,
                ChartTone::Success => UiBlockTone::Success,
                ChartTone::Warning => UiBlockTone::Warning,
                ChartTone::Danger => UiBlockTone::Danger,
                ChartTone::Muted => UiBlockTone::Muted,
            },
        }
    }

    const fn chart_intent_for_part(part: ChartPart) -> UiWidgetIntent {
        match part {
            ChartPart::Series => UiWidgetIntent::Select,
            ChartPart::Container | ChartPart::Legend | ChartPart::Tooltip | ChartPart::Axis => {
                UiWidgetIntent::None
            }
        }
    }

    fn chart_size_for_part(part: ChartPart) -> Vec2 {
        match part {
            ChartPart::Container => Vec2::new(scale::space::XL3, scale::space::XL2),
            ChartPart::Series => Vec2::new(scale::space::XL2, scale::space::L),
            ChartPart::Legend => Vec2::new(scale::space::XL, scale::space::S),
            ChartPart::Tooltip => Vec2::new(scale::space::XL2, scale::space::S),
            ChartPart::Axis => Vec2::new(scale::space::XL2, scale::space::XS3),
        }
    }

    const fn checkbox_kind_for_part(part: CheckboxPart) -> UiWidgetSlotKind {
        match part {
            CheckboxPart::Root => UiWidgetSlotKind::Section,
            CheckboxPart::Indicator => UiWidgetSlotKind::Checkbox,
            CheckboxPart::Label => UiWidgetSlotKind::Text,
            CheckboxPart::Description => UiWidgetSlotKind::Description,
        }
    }

    const fn checkbox_role_for_part(part: CheckboxPart) -> UiBlockRole {
        match part {
            CheckboxPart::Root => UiBlockRole::Root,
            CheckboxPart::Indicator => UiBlockRole::Control,
            CheckboxPart::Label => UiBlockRole::Header,
            CheckboxPart::Description => UiBlockRole::Text,
        }
    }

    const fn checkbox_tone_for_part(
        part: CheckboxPart,
        checked: CheckboxChecked,
        invalid: bool,
    ) -> UiBlockTone {
        if invalid {
            return UiBlockTone::Danger;
        }
        match part {
            CheckboxPart::Indicator => {
                if checked.is_active() {
                    UiBlockTone::Brand
                } else {
                    UiBlockTone::Surface
                }
            }
            CheckboxPart::Root | CheckboxPart::Label | CheckboxPart::Description => {
                UiBlockTone::Surface
            }
        }
    }

    const fn checkbox_intent_for_part(part: CheckboxPart) -> UiWidgetIntent {
        match part {
            CheckboxPart::Indicator => UiWidgetIntent::Toggle,
            CheckboxPart::Root | CheckboxPart::Label | CheckboxPart::Description => {
                UiWidgetIntent::None
            }
        }
    }

    fn checkbox_size_for_part(part: CheckboxPart) -> Vec2 {
        match part {
            CheckboxPart::Root => Vec2::new(scale::space::XL3, scale::space::L),
            CheckboxPart::Indicator => Vec2::new(scale::space::S, scale::space::S),
            CheckboxPart::Label => Vec2::new(scale::space::XL2, scale::space::S),
            CheckboxPart::Description => Vec2::new(scale::space::XL2, scale::space::S),
        }
    }

    const fn collapsible_kind_for_part(part: CollapsiblePart) -> UiWidgetSlotKind {
        match part {
            CollapsiblePart::Root => UiWidgetSlotKind::Section,
            CollapsiblePart::Trigger => UiWidgetSlotKind::Button,
            CollapsiblePart::Content => UiWidgetSlotKind::Text,
        }
    }

    const fn collapsible_role_for_part(part: CollapsiblePart) -> UiBlockRole {
        match part {
            CollapsiblePart::Root => UiBlockRole::Root,
            CollapsiblePart::Trigger => UiBlockRole::Action,
            CollapsiblePart::Content => UiBlockRole::Content,
        }
    }

    const fn collapsible_tone_for_part(
        part: CollapsiblePart,
        open: bool,
        disabled: bool,
    ) -> UiBlockTone {
        if disabled {
            return UiBlockTone::Muted;
        }
        match part {
            CollapsiblePart::Trigger => {
                if open {
                    UiBlockTone::Brand
                } else {
                    UiBlockTone::Surface
                }
            }
            CollapsiblePart::Root | CollapsiblePart::Content => UiBlockTone::Surface,
        }
    }

    const fn collapsible_intent_for_part(part: CollapsiblePart) -> UiWidgetIntent {
        match part {
            CollapsiblePart::Trigger => UiWidgetIntent::Toggle,
            CollapsiblePart::Root | CollapsiblePart::Content => UiWidgetIntent::None,
        }
    }

    fn collapsible_size_for_part(part: CollapsiblePart) -> Vec2 {
        match part {
            CollapsiblePart::Root => Vec2::new(scale::space::XL3, scale::space::XL),
            CollapsiblePart::Trigger => Vec2::new(scale::space::XL3, scale::space::L),
            CollapsiblePart::Content => Vec2::new(scale::space::XL3, scale::space::XL),
        }
    }

    const fn combobox_kind_for_part(part: ComboboxPart) -> UiWidgetSlotKind {
        match part {
            ComboboxPart::Root => UiWidgetSlotKind::Section,
            ComboboxPart::Input => UiWidgetSlotKind::Input,
            ComboboxPart::List => UiWidgetSlotKind::List,
            ComboboxPart::Option => UiWidgetSlotKind::Button,
            ComboboxPart::Empty => UiWidgetSlotKind::Text,
        }
    }

    const fn combobox_role_for_part(part: ComboboxPart) -> UiBlockRole {
        match part {
            ComboboxPart::Root => UiBlockRole::Root,
            ComboboxPart::Input => UiBlockRole::Control,
            ComboboxPart::List => UiBlockRole::Navigation,
            ComboboxPart::Option => UiBlockRole::Item,
            ComboboxPart::Empty => UiBlockRole::Feedback,
        }
    }

    const fn combobox_tone_for_part(
        part: ComboboxPart,
        selected: bool,
        visible: bool,
    ) -> UiBlockTone {
        match part {
            ComboboxPart::Option => {
                if selected {
                    UiBlockTone::Brand
                } else {
                    UiBlockTone::Surface
                }
            }
            ComboboxPart::Empty => {
                if visible {
                    UiBlockTone::Warning
                } else {
                    UiBlockTone::Muted
                }
            }
            ComboboxPart::Input => UiBlockTone::Brand,
            ComboboxPart::List | ComboboxPart::Root => UiBlockTone::Surface,
        }
    }

    const fn combobox_intent_for_part(part: ComboboxPart) -> UiWidgetIntent {
        match part {
            ComboboxPart::Input => UiWidgetIntent::Input,
            ComboboxPart::Option => UiWidgetIntent::Select,
            ComboboxPart::Root | ComboboxPart::List | ComboboxPart::Empty => UiWidgetIntent::None,
        }
    }

    fn combobox_size_for_part(part: ComboboxPart) -> Vec2 {
        match part {
            ComboboxPart::Root => Vec2::new(scale::space::XL3, scale::space::XL2),
            ComboboxPart::Input => Vec2::new(scale::space::XL3, scale::space::L),
            ComboboxPart::List => Vec2::new(scale::space::XL3, scale::space::XL),
            ComboboxPart::Option => Vec2::new(scale::space::XL3, scale::space::L),
            ComboboxPart::Empty => Vec2::new(scale::space::XL3, scale::space::S),
        }
    }

    const fn command_kind_for_part(part: CommandPart) -> UiWidgetSlotKind {
        match part {
            CommandPart::Root => UiWidgetSlotKind::Section,
            CommandPart::Input => UiWidgetSlotKind::Input,
            CommandPart::List => UiWidgetSlotKind::List,
            CommandPart::Group => UiWidgetSlotKind::Header,
            CommandPart::Item => UiWidgetSlotKind::Button,
            CommandPart::Shortcut => UiWidgetSlotKind::Text,
        }
    }

    const fn command_role_for_part(part: CommandPart) -> UiBlockRole {
        match part {
            CommandPart::Root => UiBlockRole::Root,
            CommandPart::Input => UiBlockRole::Control,
            CommandPart::List => UiBlockRole::Navigation,
            CommandPart::Group => UiBlockRole::Header,
            CommandPart::Item => UiBlockRole::Item,
            CommandPart::Shortcut => UiBlockRole::Text,
        }
    }

    const fn command_tone_for_part(
        part: CommandPart,
        highlighted: bool,
        selected: bool,
        visible: bool,
    ) -> UiBlockTone {
        match part {
            CommandPart::Item => {
                if selected || highlighted {
                    UiBlockTone::Brand
                } else {
                    UiBlockTone::Surface
                }
            }
            CommandPart::List => {
                if visible {
                    UiBlockTone::Surface
                } else {
                    UiBlockTone::Muted
                }
            }
            CommandPart::Input => UiBlockTone::Brand,
            CommandPart::Shortcut => UiBlockTone::Muted,
            CommandPart::Group | CommandPart::Root => UiBlockTone::Surface,
        }
    }

    const fn command_intent_for_part(part: CommandPart) -> UiWidgetIntent {
        match part {
            CommandPart::Input => UiWidgetIntent::Input,
            CommandPart::Item => UiWidgetIntent::Select,
            CommandPart::Root | CommandPart::List | CommandPart::Group | CommandPart::Shortcut => {
                UiWidgetIntent::None
            }
        }
    }

    fn command_size_for_part(part: CommandPart) -> Vec2 {
        match part {
            CommandPart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            CommandPart::Input => Vec2::new(scale::space::XL3, scale::space::L),
            CommandPart::List => Vec2::new(scale::space::XL3, scale::space::XL2),
            CommandPart::Group => Vec2::new(scale::space::XL3, scale::space::S),
            CommandPart::Item => Vec2::new(scale::space::XL3, scale::space::L),
            CommandPart::Shortcut => Vec2::new(scale::space::M, scale::space::S),
        }
    }

    const fn context_menu_kind_for_part(part: ContextMenuPart) -> UiWidgetSlotKind {
        match part {
            ContextMenuPart::Root => UiWidgetSlotKind::Section,
            ContextMenuPart::Trigger => UiWidgetSlotKind::Button,
            ContextMenuPart::Content => UiWidgetSlotKind::List,
            ContextMenuPart::Item => UiWidgetSlotKind::Button,
            ContextMenuPart::Separator => UiWidgetSlotKind::Separator,
            ContextMenuPart::Submenu => UiWidgetSlotKind::Button,
        }
    }

    const fn context_menu_role_for_part(part: ContextMenuPart) -> UiBlockRole {
        match part {
            ContextMenuPart::Root => UiBlockRole::Root,
            ContextMenuPart::Trigger => UiBlockRole::Action,
            ContextMenuPart::Content => UiBlockRole::Overlay,
            ContextMenuPart::Item => UiBlockRole::Item,
            ContextMenuPart::Separator => UiBlockRole::Layout,
            ContextMenuPart::Submenu => UiBlockRole::Navigation,
        }
    }

    const fn context_menu_tone_for_part(
        part: ContextMenuPart,
        active: bool,
        selected: bool,
        destructive: bool,
        visible: bool,
    ) -> UiBlockTone {
        match part {
            ContextMenuPart::Item => {
                if destructive {
                    UiBlockTone::Danger
                } else if selected || active {
                    UiBlockTone::Brand
                } else {
                    UiBlockTone::Surface
                }
            }
            ContextMenuPart::Submenu => {
                if active {
                    UiBlockTone::Brand
                } else {
                    UiBlockTone::Surface
                }
            }
            ContextMenuPart::Content => {
                if visible {
                    UiBlockTone::Surface
                } else {
                    UiBlockTone::Muted
                }
            }
            ContextMenuPart::Trigger => UiBlockTone::Brand,
            ContextMenuPart::Separator => UiBlockTone::Muted,
            ContextMenuPart::Root => UiBlockTone::Surface,
        }
    }

    const fn context_menu_intent_for_part(part: ContextMenuPart) -> UiWidgetIntent {
        match part {
            ContextMenuPart::Trigger => UiWidgetIntent::Toggle,
            ContextMenuPart::Item => UiWidgetIntent::Select,
            ContextMenuPart::Submenu => UiWidgetIntent::Open,
            ContextMenuPart::Root | ContextMenuPart::Content | ContextMenuPart::Separator => {
                UiWidgetIntent::None
            }
        }
    }

    fn context_menu_size_for_part(part: ContextMenuPart) -> Vec2 {
        match part {
            ContextMenuPart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            ContextMenuPart::Trigger => Vec2::new(scale::space::XL2, scale::space::L),
            ContextMenuPart::Content => Vec2::new(scale::space::XL3, scale::space::XL2),
            ContextMenuPart::Item => Vec2::new(scale::space::XL3, scale::space::L),
            ContextMenuPart::Separator => Vec2::new(scale::space::XL3, scale::space::XS),
            ContextMenuPart::Submenu => Vec2::new(scale::space::XL3, scale::space::L),
        }
    }

    const fn dropdown_menu_kind_for_part(part: DropdownMenuPart) -> UiWidgetSlotKind {
        match part {
            DropdownMenuPart::Root => UiWidgetSlotKind::Section,
            DropdownMenuPart::Trigger => UiWidgetSlotKind::Button,
            DropdownMenuPart::Content => UiWidgetSlotKind::List,
            DropdownMenuPart::Item => UiWidgetSlotKind::Button,
            DropdownMenuPart::Label => UiWidgetSlotKind::Header,
            DropdownMenuPart::Separator => UiWidgetSlotKind::Separator,
        }
    }

    const fn dropdown_menu_role_for_part(part: DropdownMenuPart) -> UiBlockRole {
        match part {
            DropdownMenuPart::Root => UiBlockRole::Root,
            DropdownMenuPart::Trigger => UiBlockRole::Action,
            DropdownMenuPart::Content => UiBlockRole::Overlay,
            DropdownMenuPart::Item => UiBlockRole::Item,
            DropdownMenuPart::Label => UiBlockRole::Header,
            DropdownMenuPart::Separator => UiBlockRole::Layout,
        }
    }

    const fn dropdown_menu_tone_for_part(
        part: DropdownMenuPart,
        active: bool,
        selected: bool,
        destructive: bool,
        visible: bool,
    ) -> UiBlockTone {
        match part {
            DropdownMenuPart::Item => {
                if destructive {
                    UiBlockTone::Danger
                } else if selected || active {
                    UiBlockTone::Brand
                } else {
                    UiBlockTone::Surface
                }
            }
            DropdownMenuPart::Content => {
                if visible {
                    UiBlockTone::Surface
                } else {
                    UiBlockTone::Muted
                }
            }
            DropdownMenuPart::Trigger => UiBlockTone::Brand,
            DropdownMenuPart::Label | DropdownMenuPart::Separator => UiBlockTone::Muted,
            DropdownMenuPart::Root => UiBlockTone::Surface,
        }
    }

    const fn dropdown_menu_intent_for_part(part: DropdownMenuPart) -> UiWidgetIntent {
        match part {
            DropdownMenuPart::Trigger => UiWidgetIntent::Toggle,
            DropdownMenuPart::Item => UiWidgetIntent::Select,
            DropdownMenuPart::Root
            | DropdownMenuPart::Content
            | DropdownMenuPart::Label
            | DropdownMenuPart::Separator => UiWidgetIntent::None,
        }
    }

    fn dropdown_menu_size_for_part(part: DropdownMenuPart) -> Vec2 {
        match part {
            DropdownMenuPart::Root => Vec2::new(scale::space::XL3, scale::space::XL2),
            DropdownMenuPart::Trigger => Vec2::new(scale::space::XL2, scale::space::L),
            DropdownMenuPart::Content => Vec2::new(scale::space::XL3, scale::space::XL2),
            DropdownMenuPart::Item => Vec2::new(scale::space::XL3, scale::space::L),
            DropdownMenuPart::Label => Vec2::new(scale::space::XL3, scale::space::S),
            DropdownMenuPart::Separator => Vec2::new(scale::space::XL3, scale::space::XS),
        }
    }

    const fn empty_kind_for_part(part: EmptyPart) -> UiWidgetSlotKind {
        match part {
            EmptyPart::Root => UiWidgetSlotKind::Section,
            EmptyPart::Header => UiWidgetSlotKind::Header,
            EmptyPart::Title => UiWidgetSlotKind::Title,
            EmptyPart::Description => UiWidgetSlotKind::Description,
            EmptyPart::Content => UiWidgetSlotKind::Panel,
            EmptyPart::Action => UiWidgetSlotKind::Button,
        }
    }

    const fn empty_role_for_part(part: EmptyPart) -> UiBlockRole {
        match part {
            EmptyPart::Root => UiBlockRole::Root,
            EmptyPart::Header | EmptyPart::Title => UiBlockRole::Header,
            EmptyPart::Description => UiBlockRole::Text,
            EmptyPart::Content => UiBlockRole::Feedback,
            EmptyPart::Action => UiBlockRole::Action,
        }
    }

    const fn empty_tone_for_part(part: EmptyPart, active: bool, actionable: bool) -> UiBlockTone {
        match part {
            EmptyPart::Action => {
                if active || actionable {
                    UiBlockTone::Brand
                } else {
                    UiBlockTone::Muted
                }
            }
            EmptyPart::Content => UiBlockTone::Warning,
            EmptyPart::Root | EmptyPart::Header | EmptyPart::Title | EmptyPart::Description => {
                UiBlockTone::Surface
            }
        }
    }

    const fn empty_intent_for_part(part: EmptyPart, actionable: bool) -> UiWidgetIntent {
        match part {
            EmptyPart::Action if actionable => UiWidgetIntent::Activate,
            EmptyPart::Root
            | EmptyPart::Header
            | EmptyPart::Title
            | EmptyPart::Description
            | EmptyPart::Content
            | EmptyPart::Action => UiWidgetIntent::None,
        }
    }

    fn empty_size_for_part(part: EmptyPart) -> Vec2 {
        match part {
            EmptyPart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            EmptyPart::Header => Vec2::new(scale::space::XL3, scale::space::L),
            EmptyPart::Title => Vec2::new(scale::space::XL2, scale::space::M),
            EmptyPart::Description => Vec2::new(scale::space::XL3, scale::space::M),
            EmptyPart::Content => Vec2::new(scale::space::XL3, scale::space::XL),
            EmptyPart::Action => Vec2::new(scale::space::XL2, scale::space::L),
        }
    }

    const fn field_kind_for_part(part: FieldPart) -> UiWidgetSlotKind {
        match part {
            FieldPart::Root => UiWidgetSlotKind::Section,
            FieldPart::Label => UiWidgetSlotKind::Text,
            FieldPart::Control => UiWidgetSlotKind::Input,
            FieldPart::Description | FieldPart::Error => UiWidgetSlotKind::Text,
        }
    }

    const fn field_role_for_part(part: FieldPart) -> UiBlockRole {
        match part {
            FieldPart::Root => UiBlockRole::Root,
            FieldPart::Label => UiBlockRole::Header,
            FieldPart::Control => UiBlockRole::Control,
            FieldPart::Description => UiBlockRole::Text,
            FieldPart::Error => UiBlockRole::Feedback,
        }
    }

    const fn field_tone_for_part(
        part: FieldPart,
        focused: bool,
        invalid: bool,
        visible: bool,
    ) -> UiBlockTone {
        match part {
            FieldPart::Root | FieldPart::Control if invalid => UiBlockTone::Danger,
            FieldPart::Control if focused => UiBlockTone::Brand,
            FieldPart::Error if visible => UiBlockTone::Danger,
            FieldPart::Error => UiBlockTone::Muted,
            FieldPart::Label | FieldPart::Description | FieldPart::Root => UiBlockTone::Surface,
            FieldPart::Control => UiBlockTone::Brand,
        }
    }

    const fn field_intent_for_part(part: FieldPart) -> UiWidgetIntent {
        match part {
            FieldPart::Control => UiWidgetIntent::Input,
            FieldPart::Root | FieldPart::Label | FieldPart::Description | FieldPart::Error => {
                UiWidgetIntent::None
            }
        }
    }

    fn field_size_for_part(part: FieldPart) -> Vec2 {
        match part {
            FieldPart::Root => Vec2::new(scale::space::XL3, scale::space::XL2),
            FieldPart::Label => Vec2::new(scale::space::XL3, scale::space::S),
            FieldPart::Control => Vec2::new(scale::space::XL3, scale::space::L),
            FieldPart::Description => Vec2::new(scale::space::XL3, scale::space::M),
            FieldPart::Error => Vec2::new(scale::space::XL3, scale::space::S),
        }
    }

    const fn hover_card_kind_for_part(part: HoverCardPart) -> UiWidgetSlotKind {
        match part {
            HoverCardPart::Root => UiWidgetSlotKind::Section,
            HoverCardPart::Trigger => UiWidgetSlotKind::Button,
            HoverCardPart::Content => UiWidgetSlotKind::Overlay,
            HoverCardPart::Arrow => UiWidgetSlotKind::Marker,
        }
    }

    const fn hover_card_role_for_part(part: HoverCardPart) -> UiBlockRole {
        match part {
            HoverCardPart::Root => UiBlockRole::Root,
            HoverCardPart::Trigger => UiBlockRole::Action,
            HoverCardPart::Content => UiBlockRole::Overlay,
            HoverCardPart::Arrow => UiBlockRole::Indicator,
        }
    }

    const fn hover_card_tone_for_part(
        part: HoverCardPart,
        open: bool,
        visible: bool,
    ) -> UiBlockTone {
        match part {
            HoverCardPart::Trigger if open => UiBlockTone::Brand,
            HoverCardPart::Trigger => UiBlockTone::Surface,
            HoverCardPart::Content if visible => UiBlockTone::Surface,
            HoverCardPart::Arrow if visible => UiBlockTone::Accent,
            HoverCardPart::Content | HoverCardPart::Arrow => UiBlockTone::Muted,
            HoverCardPart::Root => UiBlockTone::Surface,
        }
    }

    const fn hover_card_intent_for_part(part: HoverCardPart) -> UiWidgetIntent {
        match part {
            HoverCardPart::Trigger => UiWidgetIntent::Open,
            HoverCardPart::Root | HoverCardPart::Content | HoverCardPart::Arrow => {
                UiWidgetIntent::None
            }
        }
    }

    fn hover_card_size_for_part(part: HoverCardPart) -> Vec2 {
        match part {
            HoverCardPart::Root => Vec2::new(scale::space::XL3, scale::space::XL2),
            HoverCardPart::Trigger => Vec2::new(scale::space::XL2, scale::space::L),
            HoverCardPart::Content => Vec2::new(scale::space::XL3, scale::space::XL2),
            HoverCardPart::Arrow => Vec2::new(scale::space::S, scale::space::S),
        }
    }

    const fn input_kind_for_part(part: InputPart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            InputPart::Root => UiWidgetSlotKind::Section,
            InputPart::Prefix => UiWidgetSlotKind::Text,
            InputPart::Control => UiWidgetSlotKind::Input,
            InputPart::Suffix if actionable => UiWidgetSlotKind::Button,
            InputPart::Suffix => UiWidgetSlotKind::Text,
        }
    }

    const fn input_role_for_part(part: InputPart) -> UiBlockRole {
        match part {
            InputPart::Root => UiBlockRole::Root,
            InputPart::Prefix | InputPart::Suffix => UiBlockRole::Action,
            InputPart::Control => UiBlockRole::Control,
        }
    }

    const fn input_tone_for_part(
        part: InputPart,
        focused: bool,
        invalid: bool,
        visible: bool,
    ) -> UiBlockTone {
        if invalid {
            return UiBlockTone::Danger;
        }
        match part {
            InputPart::Control if focused => UiBlockTone::Brand,
            InputPart::Suffix if visible => UiBlockTone::Brand,
            InputPart::Prefix if visible => UiBlockTone::Muted,
            InputPart::Prefix | InputPart::Suffix => UiBlockTone::Muted,
            InputPart::Root | InputPart::Control => UiBlockTone::Surface,
        }
    }

    const fn input_intent_for_part(part: InputPart, actionable: bool) -> UiWidgetIntent {
        match part {
            InputPart::Control => UiWidgetIntent::Input,
            InputPart::Suffix if actionable => UiWidgetIntent::Activate,
            InputPart::Root | InputPart::Prefix | InputPart::Suffix => UiWidgetIntent::None,
        }
    }

    fn input_size_for_part(part: InputPart) -> Vec2 {
        match part {
            InputPart::Root => Vec2::new(scale::space::XL3, scale::space::L),
            InputPart::Prefix => Vec2::new(scale::space::L, scale::space::M),
            InputPart::Control => Vec2::new(scale::space::XL3, scale::space::L),
            InputPart::Suffix => Vec2::new(scale::space::L, scale::space::M),
        }
    }

    const fn input_group_kind_for_part(part: InputGroupPart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            InputGroupPart::Root => UiWidgetSlotKind::Section,
            InputGroupPart::Addon => UiWidgetSlotKind::Text,
            InputGroupPart::Input => UiWidgetSlotKind::Input,
            InputGroupPart::Button if actionable => UiWidgetSlotKind::Button,
            InputGroupPart::Button => UiWidgetSlotKind::Text,
        }
    }

    const fn input_group_role_for_part(part: InputGroupPart) -> UiBlockRole {
        match part {
            InputGroupPart::Root => UiBlockRole::Root,
            InputGroupPart::Addon | InputGroupPart::Button => UiBlockRole::Action,
            InputGroupPart::Input => UiBlockRole::Control,
        }
    }

    const fn input_group_tone_for_part(
        part: InputGroupPart,
        focused: bool,
        invalid: bool,
        visible: bool,
    ) -> UiBlockTone {
        if invalid {
            return UiBlockTone::Danger;
        }
        match part {
            InputGroupPart::Input if focused => UiBlockTone::Brand,
            InputGroupPart::Button if visible => UiBlockTone::Brand,
            InputGroupPart::Addon if visible => UiBlockTone::Muted,
            InputGroupPart::Addon | InputGroupPart::Button => UiBlockTone::Muted,
            InputGroupPart::Root | InputGroupPart::Input => UiBlockTone::Surface,
        }
    }

    const fn input_group_intent_for_part(part: InputGroupPart, actionable: bool) -> UiWidgetIntent {
        match part {
            InputGroupPart::Input => UiWidgetIntent::Input,
            InputGroupPart::Button if actionable => UiWidgetIntent::Activate,
            InputGroupPart::Root | InputGroupPart::Addon | InputGroupPart::Button => {
                UiWidgetIntent::None
            }
        }
    }

    fn input_group_size_for_part(part: InputGroupPart) -> Vec2 {
        match part {
            InputGroupPart::Root => Vec2::new(scale::space::XL3, scale::space::L),
            InputGroupPart::Addon => Vec2::new(scale::space::L, scale::space::M),
            InputGroupPart::Input => Vec2::new(scale::space::XL3, scale::space::L),
            InputGroupPart::Button => Vec2::new(scale::space::L, scale::space::M),
        }
    }

    fn input_otp_primitive_part(node: &crate::InputOtpRenderNode) -> String {
        match (node.part, node.index) {
            (InputOtpPart::Slot, Some(index)) => format!("{}:{index}", node.part.label()),
            (InputOtpPart::Separator, Some(index)) => format!("{}:{index}", node.part.label()),
            _ => node.part.label().to_owned(),
        }
    }

    const fn input_otp_kind_for_part(part: InputOtpPart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            InputOtpPart::Root => UiWidgetSlotKind::Section,
            InputOtpPart::Group => UiWidgetSlotKind::List,
            InputOtpPart::Slot if actionable => UiWidgetSlotKind::Input,
            InputOtpPart::Slot => UiWidgetSlotKind::Text,
            InputOtpPart::Separator => UiWidgetSlotKind::Separator,
        }
    }

    const fn input_otp_role_for_part(part: InputOtpPart) -> UiBlockRole {
        match part {
            InputOtpPart::Root => UiBlockRole::Root,
            InputOtpPart::Group => UiBlockRole::Content,
            InputOtpPart::Slot => UiBlockRole::Control,
            InputOtpPart::Separator => UiBlockRole::Text,
        }
    }

    const fn input_otp_tone_for_part(
        part: InputOtpPart,
        focused: bool,
        filled: bool,
        invalid: bool,
        visible: bool,
    ) -> UiBlockTone {
        if invalid {
            return UiBlockTone::Danger;
        }
        match part {
            InputOtpPart::Slot if focused || filled => UiBlockTone::Brand,
            InputOtpPart::Separator if visible => UiBlockTone::Muted,
            InputOtpPart::Separator => UiBlockTone::Muted,
            InputOtpPart::Root | InputOtpPart::Group | InputOtpPart::Slot => UiBlockTone::Surface,
        }
    }

    const fn input_otp_intent_for_part(part: InputOtpPart, actionable: bool) -> UiWidgetIntent {
        match part {
            InputOtpPart::Slot if actionable => UiWidgetIntent::Input,
            InputOtpPart::Root
            | InputOtpPart::Group
            | InputOtpPart::Slot
            | InputOtpPart::Separator => UiWidgetIntent::None,
        }
    }

    fn input_otp_size_for_part(part: InputOtpPart) -> Vec2 {
        match part {
            InputOtpPart::Root => Vec2::new(scale::space::XL3, scale::space::L),
            InputOtpPart::Group => Vec2::new(scale::space::XL3, scale::space::L),
            InputOtpPart::Slot => Vec2::new(scale::space::L, scale::space::L),
            InputOtpPart::Separator => Vec2::new(scale::space::S, scale::space::M),
        }
    }

    fn item_primitive_part(node: &crate::ItemRenderNode) -> String {
        match (node.part, node.index) {
            (ItemPart::Actions, Some(index)) => format!("{}:{index}", node.part.label()),
            _ => node.part.label().to_owned(),
        }
    }

    const fn item_kind_for_part(part: ItemPart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            ItemPart::Root => UiWidgetSlotKind::Section,
            ItemPart::Media => UiWidgetSlotKind::Avatar,
            ItemPart::Content => UiWidgetSlotKind::Panel,
            ItemPart::Title => UiWidgetSlotKind::Title,
            ItemPart::Description => UiWidgetSlotKind::Description,
            ItemPart::Actions if actionable => UiWidgetSlotKind::Button,
            ItemPart::Actions => UiWidgetSlotKind::Text,
        }
    }

    const fn item_role_for_part(part: ItemPart) -> UiBlockRole {
        match part {
            ItemPart::Root => UiBlockRole::Root,
            ItemPart::Media => UiBlockRole::Media,
            ItemPart::Content => UiBlockRole::Content,
            ItemPart::Title => UiBlockRole::Header,
            ItemPart::Description => UiBlockRole::Text,
            ItemPart::Actions => UiBlockRole::Action,
        }
    }

    const fn item_tone_for_part(
        part: ItemPart,
        active: bool,
        invalid: bool,
        visible: bool,
        actionable: bool,
    ) -> UiBlockTone {
        if invalid {
            return UiBlockTone::Danger;
        }
        match part {
            ItemPart::Actions if active || actionable => UiBlockTone::Brand,
            ItemPart::Media if visible => UiBlockTone::Success,
            ItemPart::Media | ItemPart::Actions => UiBlockTone::Muted,
            ItemPart::Root | ItemPart::Content | ItemPart::Title | ItemPart::Description => {
                UiBlockTone::Surface
            }
        }
    }

    const fn item_intent_for_part(part: ItemPart, actionable: bool) -> UiWidgetIntent {
        match part {
            ItemPart::Actions if actionable => UiWidgetIntent::Activate,
            ItemPart::Root
            | ItemPart::Media
            | ItemPart::Content
            | ItemPart::Title
            | ItemPart::Description
            | ItemPart::Actions => UiWidgetIntent::None,
        }
    }

    fn item_size_for_part(part: ItemPart) -> Vec2 {
        match part {
            ItemPart::Root => Vec2::new(scale::space::XL3, scale::space::XL),
            ItemPart::Media => Vec2::new(scale::space::L, scale::space::L),
            ItemPart::Content => Vec2::new(scale::space::XL2, scale::space::L),
            ItemPart::Title => Vec2::new(scale::space::XL2, scale::space::S),
            ItemPart::Description => Vec2::new(scale::space::XL2, scale::space::S),
            ItemPart::Actions => Vec2::new(scale::space::XL, scale::space::M),
        }
    }

    fn kbd_primitive_part(node: &crate::KbdRenderNode) -> String {
        match (node.part, node.index) {
            (KbdPart::Key, Some(index)) => format!("{}:{index}", node.part.label()),
            _ => node.part.label().to_owned(),
        }
    }

    const fn kbd_kind_for_part(part: KbdPart) -> UiWidgetSlotKind {
        match part {
            KbdPart::Root => UiWidgetSlotKind::Section,
            KbdPart::Key => UiWidgetSlotKind::Key,
            KbdPart::Chord => UiWidgetSlotKind::Text,
        }
    }

    const fn kbd_role_for_part(part: KbdPart) -> UiBlockRole {
        match part {
            KbdPart::Root => UiBlockRole::Root,
            KbdPart::Key => UiBlockRole::Text,
            KbdPart::Chord => UiBlockRole::Content,
        }
    }

    const fn kbd_tone_for_part(
        part: KbdPart,
        focused: bool,
        invalid: bool,
        disabled: bool,
    ) -> UiBlockTone {
        if invalid {
            return UiBlockTone::Danger;
        }
        if disabled {
            return UiBlockTone::Muted;
        }
        match part {
            KbdPart::Key if focused => UiBlockTone::Brand,
            KbdPart::Key => UiBlockTone::Surface,
            KbdPart::Root | KbdPart::Chord => UiBlockTone::Surface,
        }
    }

    const fn kbd_intent_for_part(part: KbdPart) -> UiWidgetIntent {
        match part {
            KbdPart::Root | KbdPart::Key | KbdPart::Chord => UiWidgetIntent::None,
        }
    }

    fn kbd_size_for_part(part: KbdPart) -> Vec2 {
        match part {
            KbdPart::Root => Vec2::new(scale::space::XL, scale::space::M),
            KbdPart::Key => Vec2::new(scale::space::M, scale::space::S),
            KbdPart::Chord => Vec2::new(scale::space::XL, scale::space::S),
        }
    }

    const fn label_kind_for_part(part: LabelPart) -> UiWidgetSlotKind {
        match part {
            LabelPart::Root => UiWidgetSlotKind::Section,
            LabelPart::Text => UiWidgetSlotKind::Text,
            LabelPart::Requirement => UiWidgetSlotKind::Badge,
        }
    }

    const fn label_role_for_part(part: LabelPart) -> UiBlockRole {
        match part {
            LabelPart::Root => UiBlockRole::Root,
            LabelPart::Text => UiBlockRole::Header,
            LabelPart::Requirement => UiBlockRole::Indicator,
        }
    }

    const fn label_tone_for_part(
        part: LabelPart,
        requirement: LabelRequirement,
        active: bool,
        invalid: bool,
        visible: bool,
        disabled: bool,
    ) -> UiBlockTone {
        if !visible || disabled {
            return UiBlockTone::Muted;
        }
        if invalid {
            return UiBlockTone::Danger;
        }
        match (part, requirement, active) {
            (LabelPart::Requirement, LabelRequirement::Required, _) => UiBlockTone::Danger,
            (LabelPart::Requirement, LabelRequirement::Optional, _) => UiBlockTone::Muted,
            (_, _, true) => UiBlockTone::Brand,
            _ => UiBlockTone::Surface,
        }
    }

    const fn label_intent_for_part(part: LabelPart) -> UiWidgetIntent {
        match part {
            LabelPart::Root | LabelPart::Text | LabelPart::Requirement => UiWidgetIntent::None,
        }
    }

    fn label_size_for_part(part: LabelPart) -> Vec2 {
        match part {
            LabelPart::Root => Vec2::new(scale::space::XL, scale::space::S),
            LabelPart::Text => Vec2::new(scale::space::L, scale::space::S),
            LabelPart::Requirement => Vec2::new(scale::space::M, scale::space::S),
        }
    }

    const fn marker_kind_for_part(part: MarkerPart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            MarkerPart::Root => UiWidgetSlotKind::Section,
            MarkerPart::Dot => UiWidgetSlotKind::Marker,
            MarkerPart::Label => UiWidgetSlotKind::Text,
            MarkerPart::Anchor if actionable => UiWidgetSlotKind::Link,
            MarkerPart::Anchor => UiWidgetSlotKind::Text,
        }
    }

    const fn marker_role_for_part(part: MarkerPart) -> UiBlockRole {
        match part {
            MarkerPart::Root => UiBlockRole::Root,
            MarkerPart::Dot => UiBlockRole::Indicator,
            MarkerPart::Label => UiBlockRole::Text,
            MarkerPart::Anchor => UiBlockRole::Navigation,
        }
    }

    const fn marker_tone_for_part(
        part: MarkerPart,
        tone: MarkerTone,
        active: bool,
        invalid: bool,
        visible: bool,
        actionable: bool,
        disabled: bool,
    ) -> UiBlockTone {
        if !visible || disabled {
            return UiBlockTone::Muted;
        }
        if invalid {
            return UiBlockTone::Danger;
        }
        if active || actionable {
            return UiBlockTone::Brand;
        }
        match (part, tone) {
            (MarkerPart::Dot, MarkerTone::Neutral) => UiBlockTone::Muted,
            (MarkerPart::Dot, MarkerTone::Brand) => UiBlockTone::Brand,
            (MarkerPart::Dot, MarkerTone::Info) => UiBlockTone::Info,
            (MarkerPart::Dot, MarkerTone::Success) => UiBlockTone::Success,
            (MarkerPart::Dot, MarkerTone::Warning) => UiBlockTone::Warning,
            (MarkerPart::Dot, MarkerTone::Danger) => UiBlockTone::Danger,
            _ => UiBlockTone::Surface,
        }
    }

    const fn marker_intent_for_part(part: MarkerPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (MarkerPart::Anchor, true) => UiWidgetIntent::Navigate,
            _ => UiWidgetIntent::None,
        }
    }

    fn marker_size_for_part(part: MarkerPart) -> Vec2 {
        match part {
            MarkerPart::Root => Vec2::new(scale::space::XL, scale::space::S),
            MarkerPart::Dot => Vec2::new(scale::space::XS, scale::space::XS),
            MarkerPart::Label => Vec2::new(scale::space::L, scale::space::S),
            MarkerPart::Anchor => Vec2::new(scale::space::M, scale::space::S),
        }
    }

    fn menubar_primitive_part(node: &crate::MenubarRenderNode) -> String {
        match node.part {
            MenubarPart::Root => node.part.label().to_owned(),
            MenubarPart::Menu | MenubarPart::Trigger | MenubarPart::Content => {
                format!("{}:{}", node.part.label(), node.menu_value)
            }
            MenubarPart::Item => {
                format!("{}:{}:{}", node.part.label(), node.menu_value, node.value)
            }
        }
    }

    const fn menubar_kind_for_part(part: MenubarPart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            MenubarPart::Root => UiWidgetSlotKind::Section,
            MenubarPart::Menu => UiWidgetSlotKind::List,
            MenubarPart::Trigger => UiWidgetSlotKind::Button,
            MenubarPart::Content => UiWidgetSlotKind::Overlay,
            MenubarPart::Item if actionable => UiWidgetSlotKind::Option,
            MenubarPart::Item => UiWidgetSlotKind::Text,
        }
    }

    const fn menubar_role_for_part(part: MenubarPart) -> UiBlockRole {
        match part {
            MenubarPart::Root => UiBlockRole::Root,
            MenubarPart::Menu => UiBlockRole::Navigation,
            MenubarPart::Trigger => UiBlockRole::Action,
            MenubarPart::Content => UiBlockRole::Overlay,
            MenubarPart::Item => UiBlockRole::Item,
        }
    }

    fn menubar_tone_for_node(node: &crate::MenubarRenderNode) -> UiBlockTone {
        if !node.visible || node.disabled {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.open || node.focused || node.selected {
            return UiBlockTone::Brand;
        }
        match (node.part, node.actionable) {
            (MenubarPart::Trigger | MenubarPart::Item, true) => UiBlockTone::Surface,
            (MenubarPart::Content, _) => UiBlockTone::Surface,
            _ => UiBlockTone::Muted,
        }
    }

    const fn menubar_intent_for_part(part: MenubarPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (MenubarPart::Trigger, true) => UiWidgetIntent::Open,
            (MenubarPart::Item, true) => UiWidgetIntent::Activate,
            _ => UiWidgetIntent::None,
        }
    }

    fn menubar_size_for_part(part: MenubarPart) -> Vec2 {
        match part {
            MenubarPart::Root => Vec2::new(scale::space::XL3, scale::space::XL),
            MenubarPart::Menu => Vec2::new(scale::space::L, scale::space::M),
            MenubarPart::Trigger => Vec2::new(scale::space::M, scale::space::S),
            MenubarPart::Content => Vec2::new(scale::space::XL2, scale::space::XL),
            MenubarPart::Item => Vec2::new(scale::space::XL, scale::space::S),
        }
    }

    fn message_primitive_part(node: &crate::MessageRenderNode) -> String {
        match node.part {
            MessagePart::Actions => format!("{}:{}", node.part.label(), node.value),
            _ => node.part.label().to_owned(),
        }
    }

    const fn message_kind_for_part(part: MessagePart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            MessagePart::Root => UiWidgetSlotKind::Section,
            MessagePart::Header => UiWidgetSlotKind::Header,
            MessagePart::Content => UiWidgetSlotKind::Description,
            MessagePart::Footer => UiWidgetSlotKind::Text,
            MessagePart::Actions if actionable => UiWidgetSlotKind::Button,
            MessagePart::Actions => UiWidgetSlotKind::Text,
        }
    }

    const fn message_role_for_part(part: MessagePart) -> UiBlockRole {
        match part {
            MessagePart::Root => UiBlockRole::Root,
            MessagePart::Header => UiBlockRole::Header,
            MessagePart::Content => UiBlockRole::Content,
            MessagePart::Footer => UiBlockRole::Text,
            MessagePart::Actions => UiBlockRole::Action,
        }
    }

    fn message_tone_for_node(node: &crate::MessageRenderNode) -> UiBlockTone {
        if !node.visible || node.disabled {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.active || node.actionable {
            return UiBlockTone::Brand;
        }
        match (node.part, node.side) {
            (MessagePart::Root | MessagePart::Content, MessageSide::Outgoing) => UiBlockTone::Brand,
            (MessagePart::Root | MessagePart::Content, MessageSide::System) => UiBlockTone::Muted,
            (MessagePart::Root | MessagePart::Content, MessageSide::Incoming) => {
                UiBlockTone::Surface
            }
            (MessagePart::Header | MessagePart::Footer, _) => UiBlockTone::Surface,
            (MessagePart::Actions, _) => UiBlockTone::Brand,
        }
    }

    const fn message_intent_for_part(part: MessagePart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (MessagePart::Actions, true) => UiWidgetIntent::Activate,
            _ => UiWidgetIntent::None,
        }
    }

    fn message_size_for_part(part: MessagePart) -> Vec2 {
        match part {
            MessagePart::Root => Vec2::new(scale::space::XL3, scale::space::XL2),
            MessagePart::Header | MessagePart::Footer => {
                Vec2::new(scale::space::XL2, scale::space::S)
            }
            MessagePart::Content => Vec2::new(scale::space::XL2, scale::space::L),
            MessagePart::Actions => Vec2::new(scale::space::M, scale::space::S),
        }
    }

    const fn message_scroller_kind_for_part(
        part: MessageScrollerPart,
        actionable: bool,
    ) -> UiWidgetSlotKind {
        match part {
            MessageScrollerPart::Root => UiWidgetSlotKind::Section,
            MessageScrollerPart::Viewport => UiWidgetSlotKind::Panel,
            MessageScrollerPart::List => UiWidgetSlotKind::List,
            MessageScrollerPart::Anchor => UiWidgetSlotKind::Marker,
            MessageScrollerPart::JumpButton if actionable => UiWidgetSlotKind::Button,
            MessageScrollerPart::JumpButton => UiWidgetSlotKind::Text,
        }
    }

    const fn message_scroller_role_for_part(part: MessageScrollerPart) -> UiBlockRole {
        match part {
            MessageScrollerPart::Root => UiBlockRole::Root,
            MessageScrollerPart::Viewport => UiBlockRole::Layout,
            MessageScrollerPart::List => UiBlockRole::Content,
            MessageScrollerPart::Anchor => UiBlockRole::Indicator,
            MessageScrollerPart::JumpButton => UiBlockRole::Action,
        }
    }

    fn message_scroller_tone_for_node(node: &crate::MessageScrollerRenderNode) -> UiBlockTone {
        if !node.visible || node.disabled {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.active || (node.part == MessageScrollerPart::JumpButton && !node.at_latest) {
            return UiBlockTone::Brand;
        }
        match node.part {
            MessageScrollerPart::Root | MessageScrollerPart::Viewport => UiBlockTone::Surface,
            MessageScrollerPart::List => UiBlockTone::Surface,
            MessageScrollerPart::Anchor if node.at_latest => UiBlockTone::Success,
            MessageScrollerPart::Anchor => UiBlockTone::Muted,
            MessageScrollerPart::JumpButton => UiBlockTone::Brand,
        }
    }

    const fn message_scroller_intent_for_part(
        part: MessageScrollerPart,
        actionable: bool,
    ) -> UiWidgetIntent {
        match (part, actionable) {
            (MessageScrollerPart::JumpButton, true) => UiWidgetIntent::Activate,
            _ => UiWidgetIntent::None,
        }
    }

    fn message_scroller_size_for_part(part: MessageScrollerPart) -> Vec2 {
        match part {
            MessageScrollerPart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            MessageScrollerPart::Viewport => Vec2::new(scale::space::XL3, scale::space::XL2),
            MessageScrollerPart::List => Vec2::new(scale::space::XL2, scale::space::XL2),
            MessageScrollerPart::Anchor => Vec2::new(scale::space::XL2, scale::space::XS3),
            MessageScrollerPart::JumpButton => Vec2::new(scale::space::L, scale::space::S),
        }
    }

    fn native_select_primitive_part(node: &crate::NativeSelectRenderNode) -> String {
        match node.part {
            NativeSelectPart::Option => format!("{}:{}", node.part.label(), node.value),
            _ => node.part.label().to_owned(),
        }
    }

    const fn native_select_kind_for_part(part: NativeSelectPart) -> UiWidgetSlotKind {
        match part {
            NativeSelectPart::Root => UiWidgetSlotKind::Section,
            NativeSelectPart::Trigger => UiWidgetSlotKind::Select,
            NativeSelectPart::Option => UiWidgetSlotKind::Option,
            NativeSelectPart::Value => UiWidgetSlotKind::Text,
        }
    }

    const fn native_select_role_for_part(part: NativeSelectPart) -> UiBlockRole {
        match part {
            NativeSelectPart::Root => UiBlockRole::Root,
            NativeSelectPart::Trigger => UiBlockRole::Control,
            NativeSelectPart::Option => UiBlockRole::Item,
            NativeSelectPart::Value => UiBlockRole::Text,
        }
    }

    fn native_select_tone_for_node(node: &crate::NativeSelectRenderNode) -> UiBlockTone {
        if node.disabled {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.focused || node.selected {
            return UiBlockTone::Brand;
        }
        match node.part {
            NativeSelectPart::Root => UiBlockTone::Surface,
            NativeSelectPart::Trigger => UiBlockTone::Surface,
            NativeSelectPart::Option => UiBlockTone::Surface,
            NativeSelectPart::Value => UiBlockTone::Muted,
        }
    }

    const fn native_select_intent_for_part(
        part: NativeSelectPart,
        actionable: bool,
    ) -> UiWidgetIntent {
        match (part, actionable) {
            (NativeSelectPart::Trigger | NativeSelectPart::Option, true) => UiWidgetIntent::Select,
            _ => UiWidgetIntent::None,
        }
    }

    fn native_select_size_for_part(part: NativeSelectPart) -> Vec2 {
        match part {
            NativeSelectPart::Root => Vec2::new(scale::space::XL2, scale::space::L),
            NativeSelectPart::Trigger => Vec2::new(scale::space::XL2, scale::space::S),
            NativeSelectPart::Option => Vec2::new(scale::space::XL, scale::space::S),
            NativeSelectPart::Value => Vec2::new(scale::space::L, scale::space::XS),
        }
    }

    fn navigation_menu_primitive_part(node: &crate::NavigationMenuRenderNode) -> String {
        match node.part {
            NavigationMenuPart::Item => format!("{}:{}", node.part.label(), node.value),
            NavigationMenuPart::Trigger => format!("{}:{}", node.part.label(), node.value),
            NavigationMenuPart::Content => format!("{}:{}", node.part.label(), node.value),
            NavigationMenuPart::Link => format!("{}:{}", node.part.label(), node.value),
            _ => node.part.label().to_owned(),
        }
    }

    const fn navigation_menu_kind_for_part(part: NavigationMenuPart) -> UiWidgetSlotKind {
        match part {
            NavigationMenuPart::Root => UiWidgetSlotKind::Section,
            NavigationMenuPart::List => UiWidgetSlotKind::List,
            NavigationMenuPart::Item => UiWidgetSlotKind::ListItem,
            NavigationMenuPart::Trigger => UiWidgetSlotKind::Button,
            NavigationMenuPart::Content => UiWidgetSlotKind::Panel,
            NavigationMenuPart::Link => UiWidgetSlotKind::Link,
        }
    }

    const fn navigation_menu_role_for_part(part: NavigationMenuPart) -> UiBlockRole {
        match part {
            NavigationMenuPart::Root | NavigationMenuPart::List => UiBlockRole::Navigation,
            NavigationMenuPart::Item => UiBlockRole::Item,
            NavigationMenuPart::Trigger => UiBlockRole::Action,
            NavigationMenuPart::Content => UiBlockRole::Overlay,
            NavigationMenuPart::Link => UiBlockRole::Navigation,
        }
    }

    fn navigation_menu_tone_for_node(node: &crate::NavigationMenuRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.open || node.focused || node.selected {
            return UiBlockTone::Brand;
        }
        match node.part {
            NavigationMenuPart::Root | NavigationMenuPart::List => UiBlockTone::Surface,
            NavigationMenuPart::Item => UiBlockTone::Surface,
            NavigationMenuPart::Trigger => UiBlockTone::Brand,
            NavigationMenuPart::Content => UiBlockTone::Surface,
            NavigationMenuPart::Link => UiBlockTone::Surface,
        }
    }

    const fn navigation_menu_intent_for_part(
        part: NavigationMenuPart,
        actionable: bool,
    ) -> UiWidgetIntent {
        match (part, actionable) {
            (NavigationMenuPart::Trigger, true) => UiWidgetIntent::Open,
            (NavigationMenuPart::Item | NavigationMenuPart::Link, true) => UiWidgetIntent::Navigate,
            _ => UiWidgetIntent::None,
        }
    }

    fn navigation_menu_size_for_part(part: NavigationMenuPart) -> Vec2 {
        match part {
            NavigationMenuPart::Root => Vec2::new(scale::space::XL3, scale::space::XL),
            NavigationMenuPart::List => Vec2::new(scale::space::XL3, scale::space::M),
            NavigationMenuPart::Item => Vec2::new(scale::space::L, scale::space::S),
            NavigationMenuPart::Trigger => Vec2::new(scale::space::L, scale::space::S),
            NavigationMenuPart::Content => Vec2::new(scale::space::XL2, scale::space::L),
            NavigationMenuPart::Link => Vec2::new(scale::space::XL, scale::space::S),
        }
    }

    fn pagination_primitive_part(node: &crate::PaginationRenderNode) -> String {
        match node.part {
            PaginationPart::Item | PaginationPart::Link => {
                format!("{}:{}", node.part.label(), node.page)
            }
            _ => node.part.label().to_owned(),
        }
    }

    const fn pagination_kind_for_part(part: PaginationPart) -> UiWidgetSlotKind {
        match part {
            PaginationPart::Root => UiWidgetSlotKind::Section,
            PaginationPart::Content => UiWidgetSlotKind::List,
            PaginationPart::Item => UiWidgetSlotKind::ListItem,
            PaginationPart::Previous | PaginationPart::Next => UiWidgetSlotKind::Button,
            PaginationPart::Link => UiWidgetSlotKind::Link,
        }
    }

    const fn pagination_role_for_part(part: PaginationPart) -> UiBlockRole {
        match part {
            PaginationPart::Root | PaginationPart::Content => UiBlockRole::Navigation,
            PaginationPart::Item => UiBlockRole::Item,
            PaginationPart::Previous | PaginationPart::Next | PaginationPart::Link => {
                UiBlockRole::Action
            }
        }
    }

    fn pagination_tone_for_node(node: &crate::PaginationRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.current || node.focused {
            return UiBlockTone::Brand;
        }
        match node.part {
            PaginationPart::Root | PaginationPart::Content | PaginationPart::Item => {
                UiBlockTone::Surface
            }
            PaginationPart::Previous | PaginationPart::Link | PaginationPart::Next => {
                UiBlockTone::Brand
            }
        }
    }

    const fn pagination_intent_for_part(part: PaginationPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (PaginationPart::Previous | PaginationPart::Next | PaginationPart::Link, true) => {
                UiWidgetIntent::Navigate
            }
            _ => UiWidgetIntent::None,
        }
    }

    fn pagination_size_for_part(part: PaginationPart) -> Vec2 {
        match part {
            PaginationPart::Root => Vec2::new(scale::space::XL2, scale::space::L),
            PaginationPart::Content => Vec2::new(scale::space::XL2, scale::space::M),
            PaginationPart::Item => Vec2::new(scale::space::M, scale::space::S),
            PaginationPart::Previous | PaginationPart::Next => {
                Vec2::new(scale::space::M, scale::space::S)
            }
            PaginationPart::Link => Vec2::new(scale::space::S, scale::space::S),
        }
    }

    const fn popover_kind_for_part(part: PopoverPart) -> UiWidgetSlotKind {
        match part {
            PopoverPart::Root => UiWidgetSlotKind::Section,
            PopoverPart::Trigger => UiWidgetSlotKind::Button,
            PopoverPart::Content => UiWidgetSlotKind::Overlay,
            PopoverPart::Arrow => UiWidgetSlotKind::Marker,
        }
    }

    const fn popover_role_for_part(part: PopoverPart) -> UiBlockRole {
        match part {
            PopoverPart::Root => UiBlockRole::Root,
            PopoverPart::Trigger => UiBlockRole::Action,
            PopoverPart::Content => UiBlockRole::Overlay,
            PopoverPart::Arrow => UiBlockRole::Indicator,
        }
    }

    fn popover_tone_for_node(node: &crate::PopoverRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        match node.part {
            PopoverPart::Trigger if node.open || node.active => UiBlockTone::Brand,
            PopoverPart::Trigger => UiBlockTone::Surface,
            PopoverPart::Content => UiBlockTone::Surface,
            PopoverPart::Arrow => UiBlockTone::Accent,
            PopoverPart::Root => UiBlockTone::Surface,
        }
    }

    const fn popover_intent_for_part(part: PopoverPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (PopoverPart::Trigger, true) => UiWidgetIntent::Open,
            _ => UiWidgetIntent::None,
        }
    }

    fn popover_size_for_part(part: PopoverPart) -> Vec2 {
        match part {
            PopoverPart::Root => Vec2::new(scale::space::XL3, scale::space::XL2),
            PopoverPart::Trigger => Vec2::new(scale::space::XL2, scale::space::L),
            PopoverPart::Content => Vec2::new(scale::space::XL3, scale::space::XL2),
            PopoverPart::Arrow => Vec2::new(scale::space::S, scale::space::S),
        }
    }

    const fn progress_kind_for_part(part: ProgressPart) -> UiWidgetSlotKind {
        match part {
            ProgressPart::Root => UiWidgetSlotKind::Section,
            ProgressPart::Track => UiWidgetSlotKind::Progress,
            ProgressPart::Indicator => UiWidgetSlotKind::Marker,
            ProgressPart::Label => UiWidgetSlotKind::Text,
        }
    }

    const fn progress_role_for_part(part: ProgressPart) -> UiBlockRole {
        match part {
            ProgressPart::Root => UiBlockRole::Feedback,
            ProgressPart::Track | ProgressPart::Indicator => UiBlockRole::Indicator,
            ProgressPart::Label => UiBlockRole::Text,
        }
    }

    fn progress_tone_for_node(node: &crate::ProgressRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.loading {
            return UiBlockTone::Info;
        }
        if node.highlighted {
            return UiBlockTone::Accent;
        }
        match node.part {
            ProgressPart::Indicator => UiBlockTone::Brand,
            ProgressPart::Root | ProgressPart::Track | ProgressPart::Label => UiBlockTone::Surface,
        }
    }

    fn progress_size_for_part(part: ProgressPart) -> Vec2 {
        match part {
            ProgressPart::Root => Vec2::new(scale::space::XL3, scale::space::L),
            ProgressPart::Track => Vec2::new(scale::space::XL3, scale::space::XS),
            ProgressPart::Indicator => Vec2::new(scale::space::XL, scale::space::XS),
            ProgressPart::Label => Vec2::new(scale::space::XL3, scale::space::S),
        }
    }

    fn radio_group_primitive_part(node: &crate::RadioGroupRenderNode) -> String {
        match node.part {
            RadioGroupPart::Item | RadioGroupPart::Indicator | RadioGroupPart::Label => {
                format!("{}:{}", node.part.label(), node.value)
            }
            RadioGroupPart::Root => node.part.label().to_owned(),
        }
    }

    const fn radio_group_kind_for_part(part: RadioGroupPart) -> UiWidgetSlotKind {
        match part {
            RadioGroupPart::Root => UiWidgetSlotKind::Section,
            RadioGroupPart::Item => UiWidgetSlotKind::Radio,
            RadioGroupPart::Indicator => UiWidgetSlotKind::Marker,
            RadioGroupPart::Label => UiWidgetSlotKind::Text,
        }
    }

    const fn radio_group_role_for_part(part: RadioGroupPart) -> UiBlockRole {
        match part {
            RadioGroupPart::Root | RadioGroupPart::Item => UiBlockRole::Control,
            RadioGroupPart::Indicator => UiBlockRole::Indicator,
            RadioGroupPart::Label => UiBlockRole::Text,
        }
    }

    fn radio_group_tone_for_node(node: &crate::RadioGroupRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.selected {
            return UiBlockTone::Brand;
        }
        if node.focused {
            return UiBlockTone::Accent;
        }
        UiBlockTone::Surface
    }

    const fn radio_group_intent_for_part(part: RadioGroupPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (RadioGroupPart::Item | RadioGroupPart::Indicator, true) => UiWidgetIntent::Select,
            _ => UiWidgetIntent::None,
        }
    }

    fn radio_group_size_for_part(part: RadioGroupPart) -> Vec2 {
        match part {
            RadioGroupPart::Root => Vec2::new(scale::space::XL3, scale::space::XL),
            RadioGroupPart::Item => Vec2::new(scale::space::XL2, scale::space::M),
            RadioGroupPart::Indicator => Vec2::new(scale::space::S, scale::space::S),
            RadioGroupPart::Label => Vec2::new(scale::space::XL, scale::space::S),
        }
    }

    fn resizable_primitive_part(node: &crate::ResizableRenderNode) -> String {
        match node.part {
            ResizablePart::Panel | ResizablePart::Handle => {
                format!("{}:{}", node.part.label(), node.value)
            }
            ResizablePart::PanelGroup => node.part.label().to_owned(),
        }
    }

    const fn resizable_kind_for_part(part: ResizablePart) -> UiWidgetSlotKind {
        match part {
            ResizablePart::PanelGroup => UiWidgetSlotKind::Section,
            ResizablePart::Panel => UiWidgetSlotKind::Panel,
            ResizablePart::Handle => UiWidgetSlotKind::Separator,
        }
    }

    const fn resizable_role_for_part(part: ResizablePart) -> UiBlockRole {
        match part {
            ResizablePart::PanelGroup | ResizablePart::Handle => UiBlockRole::Layout,
            ResizablePart::Panel => UiBlockRole::Content,
        }
    }

    fn resizable_tone_for_node(node: &crate::ResizableRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.resizing {
            return UiBlockTone::Accent;
        }
        if node.selected {
            return UiBlockTone::Brand;
        }
        UiBlockTone::Surface
    }

    const fn resizable_intent_for_part(part: ResizablePart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (ResizablePart::Handle, true) => UiWidgetIntent::Resize,
            (ResizablePart::Panel, true) => UiWidgetIntent::Select,
            _ => UiWidgetIntent::None,
        }
    }

    fn resizable_size_for_part(
        part: ResizablePart,
        percent: u8,
        orientation: crate::ResizableOrientation,
    ) -> Vec2 {
        let basis = scale::space::XL3 * (f32::from(percent).max(20.0) / 100.0);
        match (part, orientation) {
            (ResizablePart::PanelGroup, crate::ResizableOrientation::Horizontal) => {
                Vec2::new(scale::space::XL3, scale::space::XL)
            }
            (ResizablePart::PanelGroup, crate::ResizableOrientation::Vertical) => {
                Vec2::new(scale::space::XL2, scale::space::XL3)
            }
            (ResizablePart::Panel, crate::ResizableOrientation::Horizontal) => {
                Vec2::new(basis, scale::space::XL)
            }
            (ResizablePart::Panel, crate::ResizableOrientation::Vertical) => {
                Vec2::new(scale::space::XL2, basis)
            }
            (ResizablePart::Handle, crate::ResizableOrientation::Horizontal) => {
                Vec2::new(scale::space::XS, scale::space::XL)
            }
            (ResizablePart::Handle, crate::ResizableOrientation::Vertical) => {
                Vec2::new(scale::space::XL2, scale::space::XS)
            }
        }
    }

    fn scroll_area_primitive_part(node: &crate::ScrollAreaRenderNode) -> String {
        match node.part {
            ScrollAreaPart::Content | ScrollAreaPart::Bar => {
                format!("{}:{}", node.part.label(), node.value)
            }
            ScrollAreaPart::Root | ScrollAreaPart::Viewport | ScrollAreaPart::Corner => {
                node.part.label().to_owned()
            }
        }
    }

    const fn scroll_area_kind_for_part(part: ScrollAreaPart) -> UiWidgetSlotKind {
        match part {
            ScrollAreaPart::Root => UiWidgetSlotKind::Section,
            ScrollAreaPart::Viewport => UiWidgetSlotKind::Panel,
            ScrollAreaPart::Content => UiWidgetSlotKind::Text,
            ScrollAreaPart::Bar | ScrollAreaPart::Corner => UiWidgetSlotKind::Separator,
        }
    }

    const fn scroll_area_role_for_part(part: ScrollAreaPart) -> UiBlockRole {
        match part {
            ScrollAreaPart::Root | ScrollAreaPart::Viewport => UiBlockRole::Layout,
            ScrollAreaPart::Content => UiBlockRole::Content,
            ScrollAreaPart::Bar | ScrollAreaPart::Corner => UiBlockRole::Indicator,
        }
    }

    fn scroll_area_tone_for_node(node: &crate::ScrollAreaRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.active {
            return UiBlockTone::Brand;
        }
        if node.focused {
            return UiBlockTone::Accent;
        }
        UiBlockTone::Surface
    }

    const fn scroll_area_intent_for_part(part: ScrollAreaPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (ScrollAreaPart::Viewport | ScrollAreaPart::Bar, true) => UiWidgetIntent::Scroll,
            (ScrollAreaPart::Content, true) => UiWidgetIntent::Select,
            _ => UiWidgetIntent::None,
        }
    }

    fn scroll_area_size_for_part(
        part: ScrollAreaPart,
        axis: Option<crate::ScrollAreaAxis>,
    ) -> Vec2 {
        match (part, axis) {
            (ScrollAreaPart::Root, _) => Vec2::new(scale::space::XL3, scale::space::XL2),
            (ScrollAreaPart::Viewport, _) => Vec2::new(scale::space::XL3, scale::space::XL),
            (ScrollAreaPart::Content, _) => Vec2::new(scale::space::XL2, scale::space::M),
            (ScrollAreaPart::Bar, Some(crate::ScrollAreaAxis::Vertical)) => {
                Vec2::new(scale::space::XS, scale::space::XL)
            }
            (ScrollAreaPart::Bar, Some(crate::ScrollAreaAxis::Horizontal)) => {
                Vec2::new(scale::space::XL2, scale::space::XS)
            }
            (ScrollAreaPart::Bar, None) | (ScrollAreaPart::Corner, _) => {
                Vec2::new(scale::space::XS, scale::space::XS)
            }
        }
    }

    fn select_primitive_part(node: &crate::SelectRenderNode) -> String {
        match node.part {
            SelectPart::Group | SelectPart::Item => {
                format!("{}:{}", node.part.label(), node.value)
            }
            SelectPart::Root | SelectPart::Trigger | SelectPart::Value | SelectPart::Content => {
                node.part.label().to_owned()
            }
        }
    }

    const fn select_kind_for_part(part: SelectPart) -> UiWidgetSlotKind {
        match part {
            SelectPart::Root => UiWidgetSlotKind::Section,
            SelectPart::Trigger => UiWidgetSlotKind::Select,
            SelectPart::Value => UiWidgetSlotKind::Text,
            SelectPart::Content | SelectPart::Group => UiWidgetSlotKind::List,
            SelectPart::Item => UiWidgetSlotKind::Option,
        }
    }

    const fn select_role_for_part(part: SelectPart) -> UiBlockRole {
        match part {
            SelectPart::Root => UiBlockRole::Root,
            SelectPart::Trigger => UiBlockRole::Control,
            SelectPart::Value => UiBlockRole::Text,
            SelectPart::Content | SelectPart::Group => UiBlockRole::Content,
            SelectPart::Item => UiBlockRole::Item,
        }
    }

    fn select_tone_for_node(node: &crate::SelectRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.selected {
            return UiBlockTone::Brand;
        }
        if node.focused || node.open {
            return UiBlockTone::Accent;
        }
        UiBlockTone::Surface
    }

    const fn select_intent_for_part(part: SelectPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (SelectPart::Trigger, true) => UiWidgetIntent::Toggle,
            (SelectPart::Item, true) => UiWidgetIntent::Select,
            _ => UiWidgetIntent::None,
        }
    }

    fn select_size_for_part(part: SelectPart) -> Vec2 {
        match part {
            SelectPart::Root => Vec2::new(scale::space::XL2, scale::space::XL),
            SelectPart::Trigger => Vec2::new(scale::space::XL2, scale::space::S),
            SelectPart::Value => Vec2::new(scale::space::L, scale::space::XS),
            SelectPart::Content => Vec2::new(scale::space::XL2, scale::space::XL),
            SelectPart::Group => Vec2::new(scale::space::XL, scale::space::M),
            SelectPart::Item => Vec2::new(scale::space::XL, scale::space::S),
        }
    }

    const fn separator_kind_for_part(part: SeparatorPart) -> UiWidgetSlotKind {
        match part {
            SeparatorPart::Root => UiWidgetSlotKind::Section,
            SeparatorPart::Line => UiWidgetSlotKind::Separator,
            SeparatorPart::Label => UiWidgetSlotKind::Text,
        }
    }

    const fn separator_role_for_part(part: SeparatorPart) -> UiBlockRole {
        match part {
            SeparatorPart::Root | SeparatorPart::Line => UiBlockRole::Layout,
            SeparatorPart::Label => UiBlockRole::Text,
        }
    }

    fn separator_tone_for_node(node: &crate::SeparatorRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.active {
            return UiBlockTone::Brand;
        }
        match node.part {
            SeparatorPart::Line => UiBlockTone::Muted,
            SeparatorPart::Root | SeparatorPart::Label => UiBlockTone::Surface,
        }
    }

    const fn separator_intent_for_part(_actionable: bool) -> UiWidgetIntent {
        UiWidgetIntent::None
    }

    fn separator_size_for_part(
        part: SeparatorPart,
        orientation: crate::SeparatorOrientation,
    ) -> Vec2 {
        match (part, orientation) {
            (SeparatorPart::Root, crate::SeparatorOrientation::Horizontal) => {
                Vec2::new(scale::space::XL2, scale::space::M)
            }
            (SeparatorPart::Root, crate::SeparatorOrientation::Vertical) => {
                Vec2::new(scale::space::M, scale::space::XL)
            }
            (SeparatorPart::Line, crate::SeparatorOrientation::Horizontal) => {
                Vec2::new(scale::space::XL2, scale::space::XS)
            }
            (SeparatorPart::Line, crate::SeparatorOrientation::Vertical) => {
                Vec2::new(scale::space::XS, scale::space::XL)
            }
            (SeparatorPart::Label, _) => Vec2::new(scale::space::L, scale::space::S),
        }
    }

    fn sheet_primitive_part(node: &crate::SheetRenderNode) -> String {
        match node.part {
            SheetPart::Footer => format!("{}:{}", node.part.label(), node.value),
            SheetPart::Root
            | SheetPart::Trigger
            | SheetPart::Content
            | SheetPart::Header
            | SheetPart::Close => node.part.label().to_owned(),
        }
    }

    const fn sheet_kind_for_part(part: SheetPart) -> UiWidgetSlotKind {
        match part {
            SheetPart::Root => UiWidgetSlotKind::Section,
            SheetPart::Trigger | SheetPart::Footer => UiWidgetSlotKind::Button,
            SheetPart::Content => UiWidgetSlotKind::Overlay,
            SheetPart::Header => UiWidgetSlotKind::Header,
            SheetPart::Close => UiWidgetSlotKind::IconButton,
        }
    }

    const fn sheet_role_for_part(part: SheetPart) -> UiBlockRole {
        match part {
            SheetPart::Root => UiBlockRole::Root,
            SheetPart::Trigger | SheetPart::Footer | SheetPart::Close => UiBlockRole::Action,
            SheetPart::Content => UiBlockRole::Overlay,
            SheetPart::Header => UiBlockRole::Header,
        }
    }

    fn sheet_tone_for_node(node: &crate::SheetRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.selected {
            return UiBlockTone::Brand;
        }
        if node.focused {
            return UiBlockTone::Accent;
        }
        match node.part {
            SheetPart::Trigger | SheetPart::Footer => UiBlockTone::Brand,
            SheetPart::Close => UiBlockTone::Muted,
            SheetPart::Root | SheetPart::Content | SheetPart::Header => UiBlockTone::Surface,
        }
    }

    const fn sheet_intent_for_part(part: SheetPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (SheetPart::Trigger, true) => UiWidgetIntent::Toggle,
            (SheetPart::Footer, true) => UiWidgetIntent::Activate,
            (SheetPart::Close, true) => UiWidgetIntent::Close,
            _ => UiWidgetIntent::None,
        }
    }

    fn sheet_size_for_part(part: SheetPart, side: crate::SheetSide) -> Vec2 {
        match (part, side) {
            (SheetPart::Root, _) => Vec2::new(scale::space::XL3, scale::space::XL2),
            (SheetPart::Trigger, _) => Vec2::new(scale::space::XL, scale::space::M),
            (SheetPart::Content, crate::SheetSide::Left | crate::SheetSide::Right) => {
                Vec2::new(scale::space::XL2, scale::space::XL3)
            }
            (SheetPart::Content, crate::SheetSide::Top | crate::SheetSide::Bottom) => {
                Vec2::new(scale::space::XL3, scale::space::XL2)
            }
            (SheetPart::Header, _) => Vec2::new(scale::space::XL2, scale::space::L),
            (SheetPart::Footer, _) => Vec2::new(scale::space::L, scale::space::M),
            (SheetPart::Close, _) => Vec2::new(scale::space::M, scale::space::M),
        }
    }

    fn sidebar_primitive_part(node: &crate::SidebarRenderNode) -> String {
        match node.part {
            SidebarPart::Group | SidebarPart::Menu => {
                format!("{}:{}", node.part.label(), node.value)
            }
            SidebarPart::Root
            | SidebarPart::Header
            | SidebarPart::Content
            | SidebarPart::Footer
            | SidebarPart::Rail => node.part.label().to_owned(),
        }
    }

    const fn sidebar_kind_for_part(part: SidebarPart) -> UiWidgetSlotKind {
        match part {
            SidebarPart::Root => UiWidgetSlotKind::Section,
            SidebarPart::Header => UiWidgetSlotKind::Header,
            SidebarPart::Content | SidebarPart::Group => UiWidgetSlotKind::List,
            SidebarPart::Menu => UiWidgetSlotKind::Button,
            SidebarPart::Footer => UiWidgetSlotKind::Text,
            SidebarPart::Rail => UiWidgetSlotKind::Button,
        }
    }

    const fn sidebar_role_for_part(part: SidebarPart) -> UiBlockRole {
        match part {
            SidebarPart::Root | SidebarPart::Rail => UiBlockRole::Navigation,
            SidebarPart::Header => UiBlockRole::Header,
            SidebarPart::Content | SidebarPart::Group => UiBlockRole::Layout,
            SidebarPart::Menu => UiBlockRole::Action,
            SidebarPart::Footer => UiBlockRole::Text,
        }
    }

    fn sidebar_tone_for_node(node: &crate::SidebarRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.selected {
            return UiBlockTone::Brand;
        }
        if node.focused {
            return UiBlockTone::Accent;
        }
        match node.part {
            SidebarPart::Rail => UiBlockTone::Brand,
            SidebarPart::Root
            | SidebarPart::Header
            | SidebarPart::Content
            | SidebarPart::Group
            | SidebarPart::Menu
            | SidebarPart::Footer => UiBlockTone::Surface,
        }
    }

    const fn sidebar_intent_for_part(part: SidebarPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (SidebarPart::Rail, true) => UiWidgetIntent::Toggle,
            (SidebarPart::Menu, true) => UiWidgetIntent::Navigate,
            _ => UiWidgetIntent::None,
        }
    }

    fn sidebar_size_for_part(part: SidebarPart, collapsed: bool) -> Vec2 {
        match (part, collapsed) {
            (SidebarPart::Root, true) => Vec2::new(scale::space::M, scale::space::XL3),
            (SidebarPart::Root, false) => Vec2::new(scale::space::XL3, scale::space::XL3),
            (SidebarPart::Header, _) => Vec2::new(scale::space::XL2, scale::space::L),
            (SidebarPart::Content, _) => Vec2::new(scale::space::XL2, scale::space::XL2),
            (SidebarPart::Group, _) => Vec2::new(scale::space::XL2, scale::space::M),
            (SidebarPart::Menu, _) => Vec2::new(scale::space::XL2, scale::space::M),
            (SidebarPart::Footer, _) => Vec2::new(scale::space::XL2, scale::space::M),
            (SidebarPart::Rail, _) => Vec2::new(scale::space::S, scale::space::XL3),
        }
    }

    const fn skeleton_kind_for_part(part: SkeletonPart) -> UiWidgetSlotKind {
        match part {
            SkeletonPart::Root => UiWidgetSlotKind::Section,
            SkeletonPart::Block | SkeletonPart::Text => UiWidgetSlotKind::Skeleton,
            SkeletonPart::Media => UiWidgetSlotKind::Media,
        }
    }

    const fn skeleton_role_for_part(part: SkeletonPart) -> UiBlockRole {
        match part {
            SkeletonPart::Root => UiBlockRole::Root,
            SkeletonPart::Block | SkeletonPart::Text => UiBlockRole::Indicator,
            SkeletonPart::Media => UiBlockRole::Media,
        }
    }

    fn skeleton_tone_for_node(node: &crate::SkeletonRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.animation_paused || node.active {
            return UiBlockTone::Accent;
        }
        match node.part {
            SkeletonPart::Root => UiBlockTone::Surface,
            SkeletonPart::Block | SkeletonPart::Text | SkeletonPart::Media => UiBlockTone::Muted,
        }
    }

    const fn skeleton_intent_for_part(
        part: SkeletonPart,
        loading: bool,
        disabled: bool,
    ) -> UiWidgetIntent {
        match (part, loading, disabled) {
            (SkeletonPart::Root, true, false) => UiWidgetIntent::Toggle,
            _ => UiWidgetIntent::None,
        }
    }

    fn skeleton_size_for_part(part: SkeletonPart, text_lines: u8) -> Vec2 {
        match part {
            SkeletonPart::Root => Vec2::new(scale::space::XL3, scale::space::XL2),
            SkeletonPart::Block => Vec2::new(scale::space::XL2, scale::space::L),
            SkeletonPart::Text => Vec2::new(
                scale::space::XL2,
                scale::space::XS * f32::from(text_lines.max(1)),
            ),
            SkeletonPart::Media => Vec2::new(scale::space::XL2, scale::space::XL),
        }
    }

    const fn slider_kind_for_part(part: SliderPart) -> UiWidgetSlotKind {
        match part {
            SliderPart::Root => UiWidgetSlotKind::Section,
            SliderPart::Track | SliderPart::Range | SliderPart::Thumb => UiWidgetSlotKind::Slider,
            SliderPart::Value => UiWidgetSlotKind::Badge,
        }
    }

    const fn slider_role_for_part(part: SliderPart) -> UiBlockRole {
        match part {
            SliderPart::Root => UiBlockRole::Root,
            SliderPart::Track | SliderPart::Range => UiBlockRole::Layout,
            SliderPart::Thumb => UiBlockRole::Control,
            SliderPart::Value => UiBlockRole::Text,
        }
    }

    fn slider_tone_for_node(node: &crate::SliderRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.dragging || node.focused {
            return UiBlockTone::Accent;
        }
        match node.part {
            SliderPart::Range | SliderPart::Thumb => UiBlockTone::Brand,
            SliderPart::Root | SliderPart::Track | SliderPart::Value => UiBlockTone::Surface,
        }
    }

    const fn slider_intent_for_part(part: SliderPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (SliderPart::Thumb, true) => UiWidgetIntent::Input,
            _ => UiWidgetIntent::None,
        }
    }

    fn slider_size_for_part(
        part: SliderPart,
        orientation: crate::SliderOrientation,
        percent: u8,
    ) -> Vec2 {
        let range_factor = f32::from(percent.max(1)) / 100.0;
        match (part, orientation) {
            (SliderPart::Root, crate::SliderOrientation::Horizontal) => {
                Vec2::new(scale::space::XL3, scale::space::L)
            }
            (SliderPart::Root, crate::SliderOrientation::Vertical) => {
                Vec2::new(scale::space::L, scale::space::XL3)
            }
            (SliderPart::Track, crate::SliderOrientation::Horizontal) => {
                Vec2::new(scale::space::XL2, scale::space::XS)
            }
            (SliderPart::Track, crate::SliderOrientation::Vertical) => {
                Vec2::new(scale::space::XS, scale::space::XL2)
            }
            (SliderPart::Range, crate::SliderOrientation::Horizontal) => {
                Vec2::new(scale::space::XL2 * range_factor, scale::space::XS)
            }
            (SliderPart::Range, crate::SliderOrientation::Vertical) => {
                Vec2::new(scale::space::XS, scale::space::XL2 * range_factor)
            }
            (SliderPart::Thumb, _) => Vec2::new(scale::space::M, scale::space::M),
            (SliderPart::Value, _) => Vec2::new(scale::space::L, scale::space::S),
        }
    }

    fn sonner_primitive_part(node: &crate::SonnerRenderNode) -> String {
        match node.part {
            SonnerPart::Toast | SonnerPart::Action | SonnerPart::Dismiss => {
                format!("{}:{}", node.part.label(), node.toast_value)
            }
            SonnerPart::Provider | SonnerPart::Viewport => node.part.label().to_owned(),
        }
    }

    const fn sonner_kind_for_part(part: SonnerPart) -> UiWidgetSlotKind {
        match part {
            SonnerPart::Provider => UiWidgetSlotKind::Section,
            SonnerPart::Viewport => UiWidgetSlotKind::Panel,
            SonnerPart::Toast => UiWidgetSlotKind::Overlay,
            SonnerPart::Action => UiWidgetSlotKind::Button,
            SonnerPart::Dismiss => UiWidgetSlotKind::IconButton,
        }
    }

    const fn sonner_role_for_part(part: SonnerPart) -> UiBlockRole {
        match part {
            SonnerPart::Provider => UiBlockRole::Root,
            SonnerPart::Viewport => UiBlockRole::Layout,
            SonnerPart::Toast => UiBlockRole::Feedback,
            SonnerPart::Action | SonnerPart::Dismiss => UiBlockRole::Action,
        }
    }

    fn sonner_tone_for_node(node: &crate::SonnerRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.active || node.actioned {
            return UiBlockTone::Accent;
        }
        match (node.part, node.tone) {
            (
                SonnerPart::Toast | SonnerPart::Action | SonnerPart::Dismiss,
                crate::SonnerTone::Info,
            ) => UiBlockTone::Info,
            (
                SonnerPart::Toast | SonnerPart::Action | SonnerPart::Dismiss,
                crate::SonnerTone::Success,
            ) => UiBlockTone::Success,
            (
                SonnerPart::Toast | SonnerPart::Action | SonnerPart::Dismiss,
                crate::SonnerTone::Warning,
            ) => UiBlockTone::Warning,
            (
                SonnerPart::Toast | SonnerPart::Action | SonnerPart::Dismiss,
                crate::SonnerTone::Destructive,
            ) => UiBlockTone::Danger,
            _ => UiBlockTone::Surface,
        }
    }

    const fn sonner_intent_for_part(part: SonnerPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (SonnerPart::Action, true) => UiWidgetIntent::Activate,
            (SonnerPart::Dismiss, true) => UiWidgetIntent::Dismiss,
            _ => UiWidgetIntent::None,
        }
    }

    fn sonner_size_for_part(part: SonnerPart, density: crate::SonnerDensity) -> Vec2 {
        match (part, density) {
            (SonnerPart::Provider | SonnerPart::Viewport, crate::SonnerDensity::Standard) => {
                Vec2::new(scale::space::XL3, scale::space::XL2)
            }
            (SonnerPart::Provider | SonnerPart::Viewport, crate::SonnerDensity::Dense) => {
                Vec2::new(scale::space::XL2, scale::space::XL)
            }
            (SonnerPart::Toast, crate::SonnerDensity::Standard) => {
                Vec2::new(scale::space::XL2, scale::space::L)
            }
            (SonnerPart::Toast, crate::SonnerDensity::Dense) => {
                Vec2::new(scale::space::XL2, scale::space::M)
            }
            (SonnerPart::Action, _) => Vec2::new(scale::space::L, scale::space::S),
            (SonnerPart::Dismiss, _) => Vec2::new(scale::space::M, scale::space::M),
        }
    }

    const fn spinner_kind_for_part(part: SpinnerPart) -> UiWidgetSlotKind {
        match part {
            SpinnerPart::Root => UiWidgetSlotKind::Section,
            SpinnerPart::Track => UiWidgetSlotKind::Spinner,
            SpinnerPart::Indicator => UiWidgetSlotKind::Marker,
            SpinnerPart::Label => UiWidgetSlotKind::Text,
        }
    }

    const fn spinner_role_for_part(part: SpinnerPart) -> UiBlockRole {
        match part {
            SpinnerPart::Root => UiBlockRole::Root,
            SpinnerPart::Track => UiBlockRole::Layout,
            SpinnerPart::Indicator => UiBlockRole::Indicator,
            SpinnerPart::Label => UiBlockRole::Text,
        }
    }

    fn spinner_tone_for_node(node: &crate::SpinnerRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.active || node.paused {
            return UiBlockTone::Accent;
        }
        match (node.part, node.tone) {
            (SpinnerPart::Indicator, crate::SpinnerTone::Brand) => UiBlockTone::Brand,
            (SpinnerPart::Indicator, crate::SpinnerTone::Info) => UiBlockTone::Info,
            (SpinnerPart::Indicator, crate::SpinnerTone::Success) => UiBlockTone::Success,
            (SpinnerPart::Indicator, crate::SpinnerTone::Warning) => UiBlockTone::Warning,
            (SpinnerPart::Indicator, crate::SpinnerTone::Destructive) => UiBlockTone::Danger,
            (SpinnerPart::Indicator, crate::SpinnerTone::Default) => UiBlockTone::Accent,
            _ => UiBlockTone::Surface,
        }
    }

    const fn spinner_intent_for_part(part: SpinnerPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (SpinnerPart::Root | SpinnerPart::Indicator, true) => UiWidgetIntent::Toggle,
            _ => UiWidgetIntent::None,
        }
    }

    fn spinner_size_for_part(part: SpinnerPart, size: crate::SpinnerSize) -> Vec2 {
        let edge = match size {
            crate::SpinnerSize::Small => scale::space::S,
            crate::SpinnerSize::Medium => scale::space::M,
            crate::SpinnerSize::Large => scale::space::L,
        };
        match part {
            SpinnerPart::Root => Vec2::new(scale::space::XL2, scale::space::L),
            SpinnerPart::Track | SpinnerPart::Indicator => Vec2::new(edge, edge),
            SpinnerPart::Label => Vec2::new(scale::space::XL, scale::space::S),
        }
    }

    const fn switch_kind_for_part(part: SwitchPart) -> UiWidgetSlotKind {
        match part {
            SwitchPart::Root => UiWidgetSlotKind::Section,
            SwitchPart::Track => UiWidgetSlotKind::Switch,
            SwitchPart::Thumb => UiWidgetSlotKind::Marker,
            SwitchPart::Label => UiWidgetSlotKind::Text,
        }
    }

    const fn switch_role_for_part(part: SwitchPart) -> UiBlockRole {
        match part {
            SwitchPart::Root => UiBlockRole::Root,
            SwitchPart::Track => UiBlockRole::Control,
            SwitchPart::Thumb => UiBlockRole::Indicator,
            SwitchPart::Label => UiBlockRole::Text,
        }
    }

    fn switch_tone_for_node(node: &crate::SwitchRenderNode) -> UiBlockTone {
        if node.disabled {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        match (node.part, node.checked) {
            (SwitchPart::Track | SwitchPart::Thumb, SwitchChecked::On) => UiBlockTone::Brand,
            (SwitchPart::Track | SwitchPart::Thumb, SwitchChecked::Off) => UiBlockTone::Surface,
            (SwitchPart::Root | SwitchPart::Label, _) => UiBlockTone::Surface,
        }
    }

    const fn switch_intent_for_part(part: SwitchPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (SwitchPart::Track, true) => UiWidgetIntent::Toggle,
            _ => UiWidgetIntent::None,
        }
    }

    fn switch_size_for_part(part: SwitchPart, density: crate::SwitchDensity) -> Vec2 {
        let control = match density {
            crate::SwitchDensity::Standard => Vec2::new(scale::space::XL, scale::space::M),
            crate::SwitchDensity::Dense => Vec2::new(scale::space::L, scale::space::S),
        };
        let thumb_edge = match density {
            crate::SwitchDensity::Standard => scale::space::S,
            crate::SwitchDensity::Dense => scale::space::XS,
        };
        match part {
            SwitchPart::Root => Vec2::new(scale::space::XL3, scale::space::L),
            SwitchPart::Track => control,
            SwitchPart::Thumb => Vec2::new(thumb_edge, thumb_edge),
            SwitchPart::Label => Vec2::new(scale::space::XL2, scale::space::S),
        }
    }

    const fn toggle_kind_for_part(part: TogglePart) -> UiWidgetSlotKind {
        match part {
            TogglePart::Root => UiWidgetSlotKind::Button,
            TogglePart::Indicator => UiWidgetSlotKind::Marker,
            TogglePart::Label => UiWidgetSlotKind::Text,
        }
    }

    const fn toggle_role_for_part(part: TogglePart) -> UiBlockRole {
        match part {
            TogglePart::Root => UiBlockRole::Action,
            TogglePart::Indicator => UiBlockRole::Indicator,
            TogglePart::Label => UiBlockRole::Text,
        }
    }

    fn toggle_tone_for_node(node: &crate::ToggleRenderNode) -> UiBlockTone {
        if node.disabled {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        match (node.part, node.pressed) {
            (TogglePart::Root | TogglePart::Indicator, TogglePressed::Pressed) => {
                UiBlockTone::Brand
            }
            (TogglePart::Root | TogglePart::Indicator, TogglePressed::Unpressed) => {
                UiBlockTone::Surface
            }
            (TogglePart::Label, _) => UiBlockTone::Surface,
        }
    }

    const fn toggle_intent_for_part(part: TogglePart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (TogglePart::Root, true) => UiWidgetIntent::Toggle,
            _ => UiWidgetIntent::None,
        }
    }

    fn toggle_size_for_part(part: TogglePart, density: crate::ToggleDensity) -> Vec2 {
        let root = match density {
            crate::ToggleDensity::Standard => Vec2::new(scale::space::XL2, scale::space::M),
            crate::ToggleDensity::Dense => Vec2::new(scale::space::XL, scale::space::S),
        };
        match part {
            TogglePart::Root => root,
            TogglePart::Indicator => Vec2::new(scale::space::S, scale::space::S),
            TogglePart::Label => Vec2::new(scale::space::L, scale::space::S),
        }
    }

    fn toggle_group_primitive_part(node: &crate::ToggleGroupRenderNode) -> String {
        match node.part {
            ToggleGroupPart::Item | ToggleGroupPart::Indicator => {
                format!("{}:{}", node.part.label(), node.value)
            }
            ToggleGroupPart::Root => node.part.label().to_owned(),
        }
    }

    const fn toggle_group_kind_for_part(part: ToggleGroupPart) -> UiWidgetSlotKind {
        match part {
            ToggleGroupPart::Root => UiWidgetSlotKind::Section,
            ToggleGroupPart::Item => UiWidgetSlotKind::Button,
            ToggleGroupPart::Indicator => UiWidgetSlotKind::Marker,
        }
    }

    const fn toggle_group_role_for_part(part: ToggleGroupPart) -> UiBlockRole {
        match part {
            ToggleGroupPart::Root => UiBlockRole::Root,
            ToggleGroupPart::Item => UiBlockRole::Action,
            ToggleGroupPart::Indicator => UiBlockRole::Indicator,
        }
    }

    fn toggle_group_tone_for_node(node: &crate::ToggleGroupRenderNode) -> UiBlockTone {
        if node.disabled {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        match (node.part, node.pressed, node.selection_mode) {
            (
                ToggleGroupPart::Item | ToggleGroupPart::Indicator,
                TogglePressed::Pressed,
                ToggleGroupSelectionMode::Single,
            ) => UiBlockTone::Brand,
            (
                ToggleGroupPart::Item | ToggleGroupPart::Indicator,
                TogglePressed::Pressed,
                ToggleGroupSelectionMode::Multiple,
            ) => UiBlockTone::Accent,
            (ToggleGroupPart::Root, TogglePressed::Pressed, _) => UiBlockTone::Brand,
            _ => UiBlockTone::Surface,
        }
    }

    const fn toggle_group_intent_for_part(
        part: ToggleGroupPart,
        actionable: bool,
    ) -> UiWidgetIntent {
        match (part, actionable) {
            (ToggleGroupPart::Item, true) => UiWidgetIntent::Toggle,
            _ => UiWidgetIntent::None,
        }
    }

    fn toggle_group_size_for_part(part: ToggleGroupPart, density: crate::ToggleDensity) -> Vec2 {
        let item = match density {
            crate::ToggleDensity::Standard => Vec2::new(scale::space::XL2, scale::space::M),
            crate::ToggleDensity::Dense => Vec2::new(scale::space::XL, scale::space::S),
        };
        match part {
            ToggleGroupPart::Root => Vec2::new(scale::space::XL4, scale::space::XL),
            ToggleGroupPart::Item => item,
            ToggleGroupPart::Indicator => Vec2::new(scale::space::S, scale::space::S),
        }
    }

    const fn tooltip_kind_for_part(part: TooltipPart) -> UiWidgetSlotKind {
        match part {
            TooltipPart::Root => UiWidgetSlotKind::Section,
            TooltipPart::Trigger => UiWidgetSlotKind::Button,
            TooltipPart::Content => UiWidgetSlotKind::Overlay,
            TooltipPart::Arrow => UiWidgetSlotKind::Marker,
        }
    }

    const fn tooltip_role_for_part(part: TooltipPart) -> UiBlockRole {
        match part {
            TooltipPart::Root => UiBlockRole::Root,
            TooltipPart::Trigger => UiBlockRole::Action,
            TooltipPart::Content => UiBlockRole::Overlay,
            TooltipPart::Arrow => UiBlockRole::Indicator,
        }
    }

    fn tooltip_tone_for_node(node: &crate::TooltipRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        match node.part {
            TooltipPart::Trigger if node.open || node.active => UiBlockTone::Brand,
            TooltipPart::Trigger => UiBlockTone::Surface,
            TooltipPart::Content => UiBlockTone::Surface,
            TooltipPart::Arrow => UiBlockTone::Accent,
            TooltipPart::Root => UiBlockTone::Surface,
        }
    }

    const fn tooltip_intent_for_part(part: TooltipPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (TooltipPart::Trigger, true) => UiWidgetIntent::Open,
            _ => UiWidgetIntent::None,
        }
    }

    fn tooltip_size_for_part(part: TooltipPart, density: crate::TooltipDensity) -> Vec2 {
        let trigger = match density {
            crate::TooltipDensity::Standard => Vec2::new(scale::space::XL, scale::space::M),
            crate::TooltipDensity::Dense => Vec2::new(scale::space::L, scale::space::S),
        };
        match part {
            TooltipPart::Root => Vec2::new(scale::space::XL3, scale::space::L),
            TooltipPart::Trigger => trigger,
            TooltipPart::Content => Vec2::new(scale::space::XL3, scale::space::S),
            TooltipPart::Arrow => Vec2::new(scale::space::XS, scale::space::XS),
        }
    }

    fn typography_primitive_part(node: &crate::TypographyRenderNode) -> String {
        match (node.part, node.index) {
            (TypographyPart::List, Some(index)) => format!("{}:{index}", node.part.label()),
            _ => node.part.label().to_owned(),
        }
    }

    const fn typography_kind_for_part(part: TypographyPart) -> UiWidgetSlotKind {
        match part {
            TypographyPart::Root => UiWidgetSlotKind::Section,
            TypographyPart::H1 => UiWidgetSlotKind::Title,
            TypographyPart::H2 => UiWidgetSlotKind::Header,
            TypographyPart::P | TypographyPart::Blockquote => UiWidgetSlotKind::Text,
            TypographyPart::List => UiWidgetSlotKind::List,
        }
    }

    const fn typography_role_for_part(part: TypographyPart) -> UiBlockRole {
        match part {
            TypographyPart::Root => UiBlockRole::Root,
            TypographyPart::H1 | TypographyPart::H2 => UiBlockRole::Header,
            TypographyPart::P | TypographyPart::List | TypographyPart::Blockquote => {
                UiBlockRole::Text
            }
        }
    }

    fn typography_tone_for_node(node: &crate::TypographyRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.active {
            return UiBlockTone::Brand;
        }
        match node.part {
            TypographyPart::H1 | TypographyPart::H2 | TypographyPart::Blockquote => {
                UiBlockTone::Surface
            }
            TypographyPart::Root | TypographyPart::P | TypographyPart::List => UiBlockTone::Muted,
        }
    }

    const fn typography_intent_for_part(part: TypographyPart, disabled: bool) -> UiWidgetIntent {
        match (part, disabled) {
            (_, true) => UiWidgetIntent::None,
            (TypographyPart::Root | TypographyPart::H1 | TypographyPart::H2, false) => {
                UiWidgetIntent::Select
            }
            (TypographyPart::P | TypographyPart::List | TypographyPart::Blockquote, false) => {
                UiWidgetIntent::Select
            }
        }
    }

    fn typography_size_for_part(
        part: TypographyPart,
        density: crate::TypographyDensity,
        list_item: bool,
    ) -> Vec2 {
        let dense = matches!(density, crate::TypographyDensity::Dense);
        match (part, dense, list_item) {
            (TypographyPart::Root, true, _) => Vec2::new(scale::space::XL3, scale::space::XL2),
            (TypographyPart::Root, false, _) => Vec2::new(scale::space::XL4, scale::space::XL3),
            (TypographyPart::H1, true, _) => Vec2::new(scale::space::XL3, scale::space::M),
            (TypographyPart::H1, false, _) => Vec2::new(scale::space::XL3, scale::space::L),
            (TypographyPart::H2, true, _) => Vec2::new(scale::space::XL2, scale::space::S),
            (TypographyPart::H2, false, _) => Vec2::new(scale::space::XL3, scale::space::M),
            (TypographyPart::P, true, _) => Vec2::new(scale::space::XL3, scale::space::M),
            (TypographyPart::P, false, _) => Vec2::new(scale::space::XL3, scale::space::L),
            (TypographyPart::List, true, true) => Vec2::new(scale::space::XL2, scale::space::S),
            (TypographyPart::List, false, true) => Vec2::new(scale::space::XL2, scale::space::M),
            (TypographyPart::List, true, false) => Vec2::new(scale::space::XL3, scale::space::L),
            (TypographyPart::List, false, false) => Vec2::new(scale::space::XL3, scale::space::XL),
            (TypographyPart::Blockquote, true, _) => Vec2::new(scale::space::XL2, scale::space::M),
            (TypographyPart::Blockquote, false, _) => Vec2::new(scale::space::XL3, scale::space::L),
        }
    }

    fn table_primitive_part(node: &crate::TableRenderNode) -> String {
        match node.part {
            TablePart::Head => format!("{}:{}", node.part.label(), node.column_value),
            TablePart::Row => format!("{}:{}", node.part.label(), node.row_value),
            TablePart::Cell => format!(
                "{}:{}:{}",
                node.part.label(),
                node.row_value,
                node.column_value
            ),
            TablePart::Root | TablePart::Header | TablePart::Body | TablePart::Caption => {
                node.part.label().to_owned()
            }
        }
    }

    const fn table_kind_for_part(part: TablePart) -> UiWidgetSlotKind {
        match part {
            TablePart::Root | TablePart::Body => UiWidgetSlotKind::Table,
            TablePart::Header => UiWidgetSlotKind::Header,
            TablePart::Row => UiWidgetSlotKind::Row,
            TablePart::Head | TablePart::Cell => UiWidgetSlotKind::Cell,
            TablePart::Caption => UiWidgetSlotKind::Text,
        }
    }

    const fn table_role_for_part(part: TablePart) -> UiBlockRole {
        match part {
            TablePart::Root => UiBlockRole::Root,
            TablePart::Header | TablePart::Head => UiBlockRole::Header,
            TablePart::Body | TablePart::Row | TablePart::Cell => UiBlockRole::Data,
            TablePart::Caption => UiBlockRole::Text,
        }
    }

    fn table_tone_for_node(node: &crate::TableRenderNode) -> UiBlockTone {
        if node.disabled {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        match node.part {
            TablePart::Row | TablePart::Cell if node.selected => UiBlockTone::Brand,
            TablePart::Header | TablePart::Head => UiBlockTone::Muted,
            TablePart::Root | TablePart::Body | TablePart::Row | TablePart::Cell => {
                UiBlockTone::Surface
            }
            TablePart::Caption => UiBlockTone::Muted,
        }
    }

    const fn table_intent_for_part(part: TablePart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (TablePart::Row, true) => UiWidgetIntent::Select,
            _ => UiWidgetIntent::None,
        }
    }

    fn table_size_for_part(part: TablePart, density: crate::TableDensity) -> Vec2 {
        let row_height = match density {
            crate::TableDensity::Standard => scale::space::M,
            crate::TableDensity::Dense => scale::space::S,
        };
        match part {
            TablePart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            TablePart::Header | TablePart::Body => Vec2::new(scale::space::XL3, row_height),
            TablePart::Row => Vec2::new(scale::space::XL3, row_height),
            TablePart::Head | TablePart::Cell => Vec2::new(scale::space::XL, row_height),
            TablePart::Caption => Vec2::new(scale::space::XL3, scale::space::S),
        }
    }

    fn tabs_primitive_part(node: &crate::TabsRenderNode) -> String {
        match node.part {
            TabsPart::Trigger | TabsPart::Content => {
                format!("{}:{}", node.part.label(), node.value)
            }
            TabsPart::Root | TabsPart::List => node.part.label().to_owned(),
        }
    }

    const fn tabs_kind_for_part(part: TabsPart) -> UiWidgetSlotKind {
        match part {
            TabsPart::Root => UiWidgetSlotKind::Section,
            TabsPart::List => UiWidgetSlotKind::List,
            TabsPart::Trigger => UiWidgetSlotKind::Button,
            TabsPart::Content => UiWidgetSlotKind::Panel,
        }
    }

    const fn tabs_role_for_part(part: TabsPart) -> UiBlockRole {
        match part {
            TabsPart::Root => UiBlockRole::Root,
            TabsPart::List => UiBlockRole::Navigation,
            TabsPart::Trigger => UiBlockRole::Action,
            TabsPart::Content => UiBlockRole::Content,
        }
    }

    fn tabs_tone_for_node(node: &crate::TabsRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.focused {
            return UiBlockTone::Accent;
        }
        match node.part {
            TabsPart::Trigger if node.selected => UiBlockTone::Brand,
            TabsPart::List => UiBlockTone::Muted,
            TabsPart::Root | TabsPart::Trigger | TabsPart::Content => UiBlockTone::Surface,
        }
    }

    const fn tabs_intent_for_part(part: TabsPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (TabsPart::Trigger, true) => UiWidgetIntent::Select,
            _ => UiWidgetIntent::None,
        }
    }

    fn tabs_size_for_part(
        part: TabsPart,
        density: crate::TabsDensity,
        orientation: crate::TabsOrientation,
    ) -> Vec2 {
        let trigger_height = match density {
            crate::TabsDensity::Standard => scale::space::M,
            crate::TabsDensity::Dense => scale::space::S,
        };
        match (part, orientation) {
            (TabsPart::Root, crate::TabsOrientation::Horizontal) => {
                Vec2::new(scale::space::XL3, scale::space::XL2)
            }
            (TabsPart::Root, crate::TabsOrientation::Vertical) => {
                Vec2::new(scale::space::XL3, scale::space::XL3)
            }
            (TabsPart::List, crate::TabsOrientation::Horizontal) => {
                Vec2::new(scale::space::XL3, trigger_height)
            }
            (TabsPart::List, crate::TabsOrientation::Vertical) => {
                Vec2::new(scale::space::L, scale::space::XL2)
            }
            (TabsPart::Trigger, crate::TabsOrientation::Horizontal) => {
                Vec2::new(scale::space::XL, trigger_height)
            }
            (TabsPart::Trigger, crate::TabsOrientation::Vertical) => {
                Vec2::new(scale::space::L, trigger_height)
            }
            (TabsPart::Content, _) => Vec2::new(scale::space::XL3, scale::space::XL),
        }
    }

    const fn textarea_kind_for_part(part: TextareaPart) -> UiWidgetSlotKind {
        match part {
            TextareaPart::Root => UiWidgetSlotKind::Section,
            TextareaPart::Control => UiWidgetSlotKind::Textarea,
            TextareaPart::Counter => UiWidgetSlotKind::Badge,
            TextareaPart::Hint => UiWidgetSlotKind::Description,
        }
    }

    const fn textarea_role_for_part(part: TextareaPart) -> UiBlockRole {
        match part {
            TextareaPart::Root => UiBlockRole::Root,
            TextareaPart::Control => UiBlockRole::Control,
            TextareaPart::Counter => UiBlockRole::Indicator,
            TextareaPart::Hint => UiBlockRole::Text,
        }
    }

    fn textarea_tone_for_node(node: &crate::TextareaRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.focused {
            return UiBlockTone::Accent;
        }
        match node.part {
            TextareaPart::Control => UiBlockTone::Brand,
            TextareaPart::Counter => {
                if node
                    .max_length
                    .is_some_and(|max_length| node.current_length >= usize::from(max_length))
                {
                    UiBlockTone::Warning
                } else {
                    UiBlockTone::Muted
                }
            }
            TextareaPart::Root | TextareaPart::Hint => UiBlockTone::Surface,
        }
    }

    const fn textarea_intent_for_part(part: TextareaPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (TextareaPart::Control, true) => UiWidgetIntent::Input,
            _ => UiWidgetIntent::None,
        }
    }

    fn textarea_size_for_part(
        part: TextareaPart,
        density: crate::TextareaDensity,
        rows: u8,
    ) -> Vec2 {
        let row_height = match density {
            crate::TextareaDensity::Standard => scale::space::S,
            crate::TextareaDensity::Dense => scale::space::XS,
        };
        match part {
            TextareaPart::Root => Vec2::new(scale::space::XL3, scale::space::XL2),
            TextareaPart::Control => {
                Vec2::new(scale::space::XL3, row_height * f32::from(rows.max(2)))
            }
            TextareaPart::Counter => Vec2::new(scale::space::M, scale::space::S),
            TextareaPart::Hint => Vec2::new(scale::space::XL2, scale::space::S),
        }
    }

    const fn toast_kind_for_part(part: ToastPart) -> UiWidgetSlotKind {
        match part {
            ToastPart::Provider => UiWidgetSlotKind::Section,
            ToastPart::Viewport => UiWidgetSlotKind::Panel,
            ToastPart::Toast => UiWidgetSlotKind::Overlay,
            ToastPart::Title => UiWidgetSlotKind::Text,
            ToastPart::Description => UiWidgetSlotKind::Description,
            ToastPart::Action => UiWidgetSlotKind::Button,
        }
    }

    const fn toast_role_for_part(part: ToastPart) -> UiBlockRole {
        match part {
            ToastPart::Provider => UiBlockRole::Root,
            ToastPart::Viewport => UiBlockRole::Layout,
            ToastPart::Toast => UiBlockRole::Feedback,
            ToastPart::Title => UiBlockRole::Header,
            ToastPart::Description => UiBlockRole::Text,
            ToastPart::Action => UiBlockRole::Action,
        }
    }

    fn toast_tone_for_node(node: &crate::ToastRenderNode) -> UiBlockTone {
        if node.disabled || !node.visible {
            return UiBlockTone::Muted;
        }
        if node.invalid {
            return UiBlockTone::Danger;
        }
        if node.focused || node.paused || node.actioned {
            return UiBlockTone::Accent;
        }
        match (node.part, node.tone) {
            (
                ToastPart::Toast | ToastPart::Title | ToastPart::Description | ToastPart::Action,
                crate::ToastTone::Info,
            ) => UiBlockTone::Info,
            (
                ToastPart::Toast | ToastPart::Title | ToastPart::Description | ToastPart::Action,
                crate::ToastTone::Success,
            ) => UiBlockTone::Success,
            (
                ToastPart::Toast | ToastPart::Title | ToastPart::Description | ToastPart::Action,
                crate::ToastTone::Warning,
            ) => UiBlockTone::Warning,
            (
                ToastPart::Toast | ToastPart::Title | ToastPart::Description | ToastPart::Action,
                crate::ToastTone::Destructive,
            ) => UiBlockTone::Danger,
            _ => UiBlockTone::Surface,
        }
    }

    const fn toast_intent_for_part(part: ToastPart, actionable: bool) -> UiWidgetIntent {
        match (part, actionable) {
            (ToastPart::Toast, true) => UiWidgetIntent::Open,
            (ToastPart::Action, true) => UiWidgetIntent::Activate,
            _ => UiWidgetIntent::None,
        }
    }

    fn toast_size_for_part(part: ToastPart, density: crate::ToastDensity) -> Vec2 {
        match (part, density) {
            (ToastPart::Provider | ToastPart::Viewport, crate::ToastDensity::Standard) => {
                Vec2::new(scale::space::XL3, scale::space::XL2)
            }
            (ToastPart::Provider | ToastPart::Viewport, crate::ToastDensity::Dense) => {
                Vec2::new(scale::space::XL2, scale::space::XL)
            }
            (ToastPart::Toast, crate::ToastDensity::Standard) => {
                Vec2::new(scale::space::XL2, scale::space::L)
            }
            (ToastPart::Toast, crate::ToastDensity::Dense) => {
                Vec2::new(scale::space::XL2, scale::space::M)
            }
            (ToastPart::Title, _) => Vec2::new(scale::space::XL, scale::space::S),
            (ToastPart::Description, _) => Vec2::new(scale::space::XL2, scale::space::S),
            (ToastPart::Action, _) => Vec2::new(scale::space::L, scale::space::S),
        }
    }

    fn data_table_primitive_part(node: &crate::DataTableRenderNode) -> String {
        match node.part {
            DataTablePart::Header => {
                format!("{}:{}", node.part.label(), node.column_value)
            }
            DataTablePart::Row => {
                let value = if node.row_value.is_empty() {
                    node.value.as_str()
                } else {
                    node.row_value.as_str()
                };
                format!("{}:{value}", node.part.label())
            }
            DataTablePart::Cell => {
                format!(
                    "{}:{}:{}",
                    node.part.label(),
                    node.row_value,
                    node.column_value
                )
            }
            DataTablePart::Root | DataTablePart::Toolbar | DataTablePart::Pagination => {
                node.part.label().to_owned()
            }
        }
    }

    const fn data_table_kind_for_part(part: DataTablePart) -> UiWidgetSlotKind {
        match part {
            DataTablePart::Root => UiWidgetSlotKind::Table,
            DataTablePart::Toolbar => UiWidgetSlotKind::Input,
            DataTablePart::Header => UiWidgetSlotKind::Header,
            DataTablePart::Row => UiWidgetSlotKind::Row,
            DataTablePart::Cell => UiWidgetSlotKind::Cell,
            DataTablePart::Pagination => UiWidgetSlotKind::Button,
        }
    }

    const fn data_table_role_for_part(part: DataTablePart) -> UiBlockRole {
        match part {
            DataTablePart::Root => UiBlockRole::Root,
            DataTablePart::Toolbar => UiBlockRole::Control,
            DataTablePart::Header => UiBlockRole::Header,
            DataTablePart::Row | DataTablePart::Cell => UiBlockRole::Data,
            DataTablePart::Pagination => UiBlockRole::Navigation,
        }
    }

    const fn data_table_tone_for_part(
        part: DataTablePart,
        selected: bool,
        disabled: bool,
    ) -> UiBlockTone {
        if disabled {
            return UiBlockTone::Muted;
        }
        match part {
            DataTablePart::Header | DataTablePart::Row if selected => UiBlockTone::Brand,
            DataTablePart::Toolbar => UiBlockTone::Brand,
            DataTablePart::Pagination => UiBlockTone::Surface,
            DataTablePart::Root
            | DataTablePart::Header
            | DataTablePart::Row
            | DataTablePart::Cell => UiBlockTone::Surface,
        }
    }

    const fn data_table_intent_for_part(part: DataTablePart) -> UiWidgetIntent {
        match part {
            DataTablePart::Toolbar => UiWidgetIntent::Input,
            DataTablePart::Header | DataTablePart::Row => UiWidgetIntent::Select,
            DataTablePart::Pagination => UiWidgetIntent::Navigate,
            DataTablePart::Root | DataTablePart::Cell => UiWidgetIntent::None,
        }
    }

    fn data_table_size_for_part(part: DataTablePart) -> Vec2 {
        match part {
            DataTablePart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            DataTablePart::Toolbar => Vec2::new(scale::space::XL3, scale::space::L),
            DataTablePart::Header => Vec2::new(scale::space::XL, scale::space::M),
            DataTablePart::Row => Vec2::new(scale::space::XL3, scale::space::L),
            DataTablePart::Cell => Vec2::new(scale::space::XL, scale::space::M),
            DataTablePart::Pagination => Vec2::new(scale::space::XL3, scale::space::L),
        }
    }

    fn date_picker_primitive_part(node: &crate::DatePickerRenderNode) -> String {
        match (node.part, node.index, node.date) {
            (DatePickerPart::Calendar, index, Some(date)) if index > 0 => {
                format!("{}:{}", node.part.label(), date.value())
            }
            (DatePickerPart::Calendar, index, None) if index > 0 => {
                format!("{}:{index}", node.part.label())
            }
            _ => node.part.label().to_owned(),
        }
    }

    const fn date_picker_kind_for_part(part: DatePickerPart, day_node: bool) -> UiWidgetSlotKind {
        match part {
            DatePickerPart::Root => UiWidgetSlotKind::Section,
            DatePickerPart::Trigger => UiWidgetSlotKind::Button,
            DatePickerPart::Popover => UiWidgetSlotKind::Overlay,
            DatePickerPart::Calendar if day_node => UiWidgetSlotKind::Button,
            DatePickerPart::Calendar => UiWidgetSlotKind::List,
            DatePickerPart::Value => UiWidgetSlotKind::Text,
        }
    }

    const fn date_picker_role_for_part(part: DatePickerPart) -> UiBlockRole {
        match part {
            DatePickerPart::Root => UiBlockRole::Root,
            DatePickerPart::Trigger => UiBlockRole::Action,
            DatePickerPart::Popover => UiBlockRole::Overlay,
            DatePickerPart::Calendar => UiBlockRole::Control,
            DatePickerPart::Value => UiBlockRole::Text,
        }
    }

    const fn date_picker_tone_for_part(
        part: DatePickerPart,
        selected: bool,
        open: bool,
        current_month: bool,
        disabled: bool,
    ) -> UiBlockTone {
        if disabled {
            return UiBlockTone::Muted;
        }
        match part {
            DatePickerPart::Trigger if open || selected => UiBlockTone::Brand,
            DatePickerPart::Calendar if selected => UiBlockTone::Brand,
            DatePickerPart::Calendar if !current_month => UiBlockTone::Muted,
            DatePickerPart::Popover if open => UiBlockTone::Surface,
            DatePickerPart::Value if selected => UiBlockTone::Brand,
            DatePickerPart::Root
            | DatePickerPart::Trigger
            | DatePickerPart::Popover
            | DatePickerPart::Calendar
            | DatePickerPart::Value => UiBlockTone::Surface,
        }
    }

    const fn date_picker_intent_for_part(part: DatePickerPart, day_node: bool) -> UiWidgetIntent {
        match part {
            DatePickerPart::Trigger => UiWidgetIntent::Toggle,
            DatePickerPart::Calendar if day_node => UiWidgetIntent::Select,
            DatePickerPart::Calendar => UiWidgetIntent::Navigate,
            DatePickerPart::Root | DatePickerPart::Popover | DatePickerPart::Value => {
                UiWidgetIntent::None
            }
        }
    }

    fn date_picker_size_for_part(part: DatePickerPart, day_node: bool) -> Vec2 {
        match part {
            DatePickerPart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            DatePickerPart::Trigger => Vec2::new(scale::space::XL3, scale::space::L),
            DatePickerPart::Popover => Vec2::new(scale::space::XL3, scale::space::XL3),
            DatePickerPart::Calendar if day_node => Vec2::new(scale::space::L, scale::space::L),
            DatePickerPart::Calendar => Vec2::new(scale::space::XL3, scale::space::XL2),
            DatePickerPart::Value => Vec2::new(scale::space::XL, scale::space::S),
        }
    }

    const fn dialog_kind_for_part(part: DialogPart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            DialogPart::Root => UiWidgetSlotKind::Section,
            DialogPart::Trigger => UiWidgetSlotKind::Button,
            DialogPart::Content => UiWidgetSlotKind::Overlay,
            DialogPart::Header => UiWidgetSlotKind::Header,
            DialogPart::Title => UiWidgetSlotKind::Title,
            DialogPart::Description => UiWidgetSlotKind::Description,
            DialogPart::Footer if actionable => UiWidgetSlotKind::Button,
            DialogPart::Footer => UiWidgetSlotKind::Panel,
        }
    }

    const fn dialog_role_for_part(part: DialogPart) -> UiBlockRole {
        match part {
            DialogPart::Root => UiBlockRole::Root,
            DialogPart::Trigger => UiBlockRole::Action,
            DialogPart::Content => UiBlockRole::Overlay,
            DialogPart::Header | DialogPart::Title => UiBlockRole::Header,
            DialogPart::Description => UiBlockRole::Text,
            DialogPart::Footer => UiBlockRole::Layout,
        }
    }

    const fn dialog_tone_for_part(
        part: DialogPart,
        selected: bool,
        open: bool,
        disabled: bool,
    ) -> UiBlockTone {
        if disabled {
            return UiBlockTone::Muted;
        }
        match part {
            DialogPart::Trigger if open || selected => UiBlockTone::Brand,
            DialogPart::Footer if selected => UiBlockTone::Brand,
            DialogPart::Content if open => UiBlockTone::Surface,
            DialogPart::Trigger | DialogPart::Footer => UiBlockTone::Brand,
            DialogPart::Root
            | DialogPart::Content
            | DialogPart::Header
            | DialogPart::Title
            | DialogPart::Description => UiBlockTone::Surface,
        }
    }

    const fn dialog_intent_for_part(
        part: DialogPart,
        actionable: bool,
        close_dialog: bool,
    ) -> UiWidgetIntent {
        match part {
            DialogPart::Trigger => UiWidgetIntent::Toggle,
            DialogPart::Footer if actionable && close_dialog => UiWidgetIntent::Close,
            DialogPart::Footer if actionable => UiWidgetIntent::Activate,
            DialogPart::Root
            | DialogPart::Content
            | DialogPart::Header
            | DialogPart::Title
            | DialogPart::Description
            | DialogPart::Footer => UiWidgetIntent::None,
        }
    }

    fn dialog_size_for_part(part: DialogPart) -> Vec2 {
        match part {
            DialogPart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            DialogPart::Trigger => Vec2::new(scale::space::XL2, scale::space::L),
            DialogPart::Content => Vec2::new(scale::space::XL3, scale::space::XL2),
            DialogPart::Header => Vec2::new(scale::space::XL3, scale::space::L),
            DialogPart::Title => Vec2::new(scale::space::XL2, scale::space::M),
            DialogPart::Description => Vec2::new(scale::space::XL3, scale::space::M),
            DialogPart::Footer => Vec2::new(scale::space::XL3, scale::space::L),
        }
    }

    const fn direction_kind_for_part(part: DirectionPart) -> UiWidgetSlotKind {
        match part {
            DirectionPart::Provider | DirectionPart::Scope => UiWidgetSlotKind::Button,
            DirectionPart::AwareContent => UiWidgetSlotKind::Text,
        }
    }

    const fn direction_role_for_part(part: DirectionPart) -> UiBlockRole {
        match part {
            DirectionPart::Provider => UiBlockRole::Root,
            DirectionPart::Scope => UiBlockRole::Layout,
            DirectionPart::AwareContent => UiBlockRole::Text,
        }
    }

    const fn direction_tone_for_part(
        part: DirectionPart,
        direction: DirectionValue,
        scope_active: bool,
        disabled: bool,
    ) -> UiBlockTone {
        if disabled {
            return UiBlockTone::Muted;
        }
        match part {
            DirectionPart::Provider if direction.is_rtl() => UiBlockTone::Brand,
            DirectionPart::Scope if scope_active => UiBlockTone::Brand,
            DirectionPart::AwareContent if direction.is_rtl() => UiBlockTone::Brand,
            DirectionPart::Provider | DirectionPart::Scope | DirectionPart::AwareContent => {
                UiBlockTone::Surface
            }
        }
    }

    const fn direction_intent_for_part(part: DirectionPart) -> UiWidgetIntent {
        match part {
            DirectionPart::Provider | DirectionPart::Scope => UiWidgetIntent::Toggle,
            DirectionPart::AwareContent => UiWidgetIntent::None,
        }
    }

    fn direction_size_for_part(part: DirectionPart) -> Vec2 {
        match part {
            DirectionPart::Provider => Vec2::new(scale::space::XL3, scale::space::L),
            DirectionPart::Scope => Vec2::new(scale::space::XL3, scale::space::XL),
            DirectionPart::AwareContent => Vec2::new(scale::space::XL3, scale::space::L),
        }
    }

    const fn drawer_kind_for_part(part: DrawerPart, actionable: bool) -> UiWidgetSlotKind {
        match part {
            DrawerPart::Root => UiWidgetSlotKind::Section,
            DrawerPart::Trigger => UiWidgetSlotKind::Button,
            DrawerPart::Content => UiWidgetSlotKind::Overlay,
            DrawerPart::Header => UiWidgetSlotKind::Header,
            DrawerPart::Handle => UiWidgetSlotKind::Button,
            DrawerPart::Footer if actionable => UiWidgetSlotKind::Button,
            DrawerPart::Footer => UiWidgetSlotKind::Panel,
        }
    }

    const fn drawer_role_for_part(part: DrawerPart) -> UiBlockRole {
        match part {
            DrawerPart::Root => UiBlockRole::Root,
            DrawerPart::Trigger | DrawerPart::Footer => UiBlockRole::Action,
            DrawerPart::Content => UiBlockRole::Overlay,
            DrawerPart::Header => UiBlockRole::Header,
            DrawerPart::Handle => UiBlockRole::Control,
        }
    }

    const fn drawer_tone_for_part(
        part: DrawerPart,
        open: bool,
        dragging: bool,
        selected: bool,
        disabled: bool,
    ) -> UiBlockTone {
        if disabled {
            return UiBlockTone::Muted;
        }
        match part {
            DrawerPart::Trigger if open => UiBlockTone::Brand,
            DrawerPart::Handle if dragging => UiBlockTone::Brand,
            DrawerPart::Footer if selected => UiBlockTone::Brand,
            DrawerPart::Content if open => UiBlockTone::Surface,
            DrawerPart::Trigger | DrawerPart::Footer | DrawerPart::Handle => UiBlockTone::Brand,
            DrawerPart::Root | DrawerPart::Content | DrawerPart::Header => UiBlockTone::Surface,
        }
    }

    const fn drawer_intent_for_part(
        part: DrawerPart,
        actionable: bool,
        close_drawer: bool,
    ) -> UiWidgetIntent {
        match part {
            DrawerPart::Trigger => UiWidgetIntent::Toggle,
            DrawerPart::Handle => UiWidgetIntent::Resize,
            DrawerPart::Footer if actionable && close_drawer => UiWidgetIntent::Close,
            DrawerPart::Footer if actionable => UiWidgetIntent::Activate,
            DrawerPart::Root | DrawerPart::Content | DrawerPart::Header | DrawerPart::Footer => {
                UiWidgetIntent::None
            }
        }
    }

    fn drawer_size_for_part(part: DrawerPart, side: DrawerSide) -> Vec2 {
        match part {
            DrawerPart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            DrawerPart::Trigger => Vec2::new(scale::space::XL2, scale::space::L),
            DrawerPart::Content => match side {
                DrawerSide::Top | DrawerSide::Bottom => {
                    Vec2::new(scale::space::XL3, scale::space::XL2)
                }
                DrawerSide::Right | DrawerSide::Left => {
                    Vec2::new(scale::space::XL2, scale::space::XL3)
                }
            },
            DrawerPart::Header => Vec2::new(scale::space::XL3, scale::space::L),
            DrawerPart::Footer => Vec2::new(scale::space::XL3, scale::space::L),
            DrawerPart::Handle => Vec2::new(scale::space::L, scale::space::XS),
        }
    }

    const fn calendar_kind_for_part(part: CalendarPart) -> UiWidgetSlotKind {
        match part {
            CalendarPart::Root => UiWidgetSlotKind::Section,
            CalendarPart::Header => UiWidgetSlotKind::Header,
            CalendarPart::Grid => UiWidgetSlotKind::List,
            CalendarPart::Day => UiWidgetSlotKind::Button,
            CalendarPart::Range => UiWidgetSlotKind::Marker,
        }
    }

    const fn calendar_role_for_part(part: CalendarPart) -> UiBlockRole {
        match part {
            CalendarPart::Root => UiBlockRole::Root,
            CalendarPart::Header => UiBlockRole::Header,
            CalendarPart::Grid => UiBlockRole::Layout,
            CalendarPart::Day => UiBlockRole::Action,
            CalendarPart::Range => UiBlockRole::Indicator,
        }
    }

    const fn calendar_tone_for_part(
        part: CalendarPart,
        mode: CalendarSelectionMode,
        selected: bool,
        in_range: bool,
        current_month: bool,
    ) -> UiBlockTone {
        match part {
            CalendarPart::Day if selected => UiBlockTone::Brand,
            CalendarPart::Day if in_range => UiBlockTone::Accent,
            CalendarPart::Day if !current_month => UiBlockTone::Muted,
            CalendarPart::Range if selected => match mode {
                CalendarSelectionMode::Single => UiBlockTone::Brand,
                CalendarSelectionMode::Range => UiBlockTone::Accent,
            },
            CalendarPart::Header => UiBlockTone::Brand,
            CalendarPart::Root | CalendarPart::Grid | CalendarPart::Range | CalendarPart::Day => {
                UiBlockTone::Surface
            }
        }
    }

    const fn calendar_intent_for_part(
        part: CalendarPart,
        mode: CalendarSelectionMode,
    ) -> UiWidgetIntent {
        match part {
            CalendarPart::Header => UiWidgetIntent::Navigate,
            CalendarPart::Day | CalendarPart::Range => match mode {
                CalendarSelectionMode::Single => UiWidgetIntent::Select,
                CalendarSelectionMode::Range => UiWidgetIntent::Select,
            },
            CalendarPart::Root | CalendarPart::Grid => UiWidgetIntent::None,
        }
    }

    fn calendar_size_for_part(part: CalendarPart) -> Vec2 {
        match part {
            CalendarPart::Root => Vec2::new(scale::space::XL3, scale::space::XL3),
            CalendarPart::Header => Vec2::new(scale::space::XL2, scale::space::L),
            CalendarPart::Grid => Vec2::new(scale::space::XL3, scale::space::XL2),
            CalendarPart::Day => Vec2::new(scale::space::L, scale::space::L),
            CalendarPart::Range => Vec2::new(scale::space::XL2, scale::space::S),
        }
    }

    const fn alert_dialog_role_for_part(part: AlertDialogPart) -> UiBlockRole {
        match part {
            AlertDialogPart::Root => UiBlockRole::Root,
            AlertDialogPart::Trigger | AlertDialogPart::Action | AlertDialogPart::Cancel => {
                UiBlockRole::Action
            }
            AlertDialogPart::Content => UiBlockRole::Overlay,
            AlertDialogPart::Header => UiBlockRole::Header,
            AlertDialogPart::Footer => UiBlockRole::Layout,
        }
    }

    const fn alert_dialog_tone_for_part(part: AlertDialogPart, destructive: bool) -> UiBlockTone {
        match part {
            AlertDialogPart::Root | AlertDialogPart::Content => {
                if destructive {
                    UiBlockTone::Danger
                } else {
                    UiBlockTone::Surface
                }
            }
            AlertDialogPart::Action => {
                if destructive {
                    UiBlockTone::Danger
                } else {
                    UiBlockTone::Brand
                }
            }
            AlertDialogPart::Trigger | AlertDialogPart::Cancel => UiBlockTone::Brand,
            AlertDialogPart::Header | AlertDialogPart::Footer => UiBlockTone::Surface,
        }
    }

    const fn alert_dialog_intent_for_part(part: AlertDialogPart) -> UiWidgetIntent {
        match part {
            AlertDialogPart::Trigger => UiWidgetIntent::Open,
            AlertDialogPart::Action => UiWidgetIntent::Activate,
            AlertDialogPart::Cancel => UiWidgetIntent::Close,
            AlertDialogPart::Root
            | AlertDialogPart::Content
            | AlertDialogPart::Header
            | AlertDialogPart::Footer => UiWidgetIntent::None,
        }
    }

    const fn alert_kind_for_part(part: AlertPart) -> UiWidgetSlotKind {
        match part {
            AlertPart::Root => UiWidgetSlotKind::Section,
            AlertPart::Title => UiWidgetSlotKind::Title,
            AlertPart::Description => UiWidgetSlotKind::Description,
            AlertPart::Action => UiWidgetSlotKind::Button,
        }
    }

    const fn alert_role_for_part(part: AlertPart) -> UiBlockRole {
        match part {
            AlertPart::Root => UiBlockRole::Root,
            AlertPart::Title => UiBlockRole::Header,
            AlertPart::Description => UiBlockRole::Content,
            AlertPart::Action => UiBlockRole::Action,
        }
    }

    const fn alert_tone_for_part(part: AlertPart, tone: crate::AlertTone) -> UiBlockTone {
        match part {
            AlertPart::Root => match tone {
                crate::AlertTone::Default => UiBlockTone::Surface,
                crate::AlertTone::Info => UiBlockTone::Info,
                crate::AlertTone::Success => UiBlockTone::Success,
                crate::AlertTone::Warning => UiBlockTone::Warning,
                crate::AlertTone::Destructive => UiBlockTone::Danger,
            },
            AlertPart::Title | AlertPart::Description => UiBlockTone::Surface,
            AlertPart::Action => UiBlockTone::Brand,
        }
    }

    const fn alert_intent_for_part(part: AlertPart) -> UiWidgetIntent {
        match part {
            AlertPart::Action => UiWidgetIntent::Activate,
            AlertPart::Root | AlertPart::Title | AlertPart::Description => UiWidgetIntent::None,
        }
    }

    const fn accordion_kind_for_part(part: AccordionPart) -> UiWidgetSlotKind {
        match part {
            AccordionPart::Root => UiWidgetSlotKind::Section,
            AccordionPart::Item => UiWidgetSlotKind::Panel,
            AccordionPart::Trigger => UiWidgetSlotKind::Button,
            AccordionPart::Content => UiWidgetSlotKind::Description,
        }
    }

    const fn accordion_role_for_part(part: AccordionPart) -> UiBlockRole {
        match part {
            AccordionPart::Root => UiBlockRole::Root,
            AccordionPart::Item => UiBlockRole::Item,
            AccordionPart::Trigger => UiBlockRole::Action,
            AccordionPart::Content => UiBlockRole::Content,
        }
    }

    const fn accordion_tone_for_part(part: AccordionPart) -> UiBlockTone {
        match part {
            AccordionPart::Root | AccordionPart::Item | AccordionPart::Content => {
                UiBlockTone::Surface
            }
            AccordionPart::Trigger => UiBlockTone::Brand,
        }
    }

    const fn accordion_intent_for_part(part: AccordionPart) -> UiWidgetIntent {
        match part {
            AccordionPart::Trigger => UiWidgetIntent::Toggle,
            AccordionPart::Root | AccordionPart::Item | AccordionPart::Content => {
                UiWidgetIntent::None
            }
        }
    }

    fn size_for_role(role: UiBlockRole) -> Vec2 {
        match role {
            UiBlockRole::Root => Vec2::new(scale::space::XL4, scale::space::XL2),
            UiBlockRole::Action | UiBlockRole::Control => {
                Vec2::new(scale::space::XL2, scale::space::L)
            }
            UiBlockRole::Indicator | UiBlockRole::Media => {
                Vec2::new(scale::space::L, scale::space::L)
            }
            UiBlockRole::Layout => Vec2::new(scale::space::XL2, scale::space::XS3),
            _ => Vec2::new(scale::space::XL3, scale::space::M),
        }
    }

    fn aspect_ratio_size(width: u16, height: u16, role: UiBlockRole) -> Vec2 {
        match role {
            UiBlockRole::Root | UiBlockRole::Layout | UiBlockRole::Media => {
                let width = f32::from(width);
                let height = f32::from(height);
                let display_width = scale::space::XL4;
                Vec2::new(display_width, display_width * height / width)
            }
            _ => size_for_role(role),
        }
    }

    fn fill_for_tone(tone: UiBlockTone, theme: &Theme) -> Color {
        match tone {
            UiBlockTone::Accent => theme.accent_soft().to_bevy(),
            UiBlockTone::Brand => theme.primary_soft().to_bevy(),
            UiBlockTone::Danger => theme.error_soft().to_bevy(),
            UiBlockTone::Info => theme.info_soft().to_bevy(),
            UiBlockTone::Muted => theme.surface_sunken().to_bevy(),
            UiBlockTone::Surface => theme.surface_elevated().to_bevy(),
            UiBlockTone::Success => theme.success_soft().to_bevy(),
            UiBlockTone::Warning => theme.warning_soft().to_bevy(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UiComponentId;

    #[test]
    fn every_component_builds_shared_spec() {
        for id in UiComponentId::ALL {
            let spec = component_spec(id);
            assert_eq!(spec.definition.id, id);
            assert!(
                spec.blocks.len() >= 2,
                "{} has too little spec",
                spec.definition.name
            );
        }
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn every_component_builds_bevy_story_variants() {
        use crate::BevyUiStoryVariantKind;

        let expected_kinds = [
            BevyUiStoryVariantKind::Default,
            BevyUiStoryVariantKind::Alternate,
            BevyUiStoryVariantKind::Loading,
            BevyUiStoryVariantKind::Disabled,
            BevyUiStoryVariantKind::Themed,
        ];

        for id in UiComponentId::ALL {
            let definition = id.definition();
            let variants =
                bevy_adapter::bevy_story_variants_for_component(id, crate::ThemeId::Dark);
            assert_eq!(
                variants.len(),
                expected_kinds.len(),
                "{} has the wrong Bevy story variant count",
                definition.name
            );

            for (variant, expected_kind) in variants.iter().zip(expected_kinds) {
                assert_eq!(
                    variant.kind, expected_kind,
                    "{} has an unexpected Bevy story variant order",
                    definition.name
                );
                assert!(
                    !variant.primitives.is_empty(),
                    "{} {} Bevy story has no primitives",
                    definition.name,
                    variant.label()
                );
                for primitive in &variant.primitives {
                    assert!(
                        !primitive.part.trim().is_empty(),
                        "{} {} Bevy primitive has no part label",
                        definition.name,
                        variant.label()
                    );
                }
            }
        }
    }
}
