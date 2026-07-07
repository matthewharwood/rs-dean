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
        EmptyPart, FieldPart, HoverCardPart, RenderContract, StateContract, Theme, UiBlockRole,
        UiBlockTone, UiComponentId, UiWidgetIntent, UiWidgetSlotKind, accordion_render_nodes,
        alert_dialog_render_nodes, alert_render_nodes, aspect_ratio_render_nodes,
        attachment_render_nodes, avatar_render_nodes, badge_render_nodes, breadcrumb_render_nodes,
        bubble_render_nodes, button_group_render_nodes, button_render_nodes, calendar_render_nodes,
        card_render_nodes, carousel_render_nodes, catalog_component_any_render_nodes_for_component,
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
        default_empty_model, default_field_model, default_hover_card_model, dialog_render_nodes,
        direction_render_nodes, drawer_render_nodes, dropdown_menu_render_nodes,
        empty_render_nodes, field_render_nodes, hover_card_render_nodes, scale,
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
                    part: node.part.label().to_owned(),
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
                    part: node.part.label().to_owned(),
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
                    part: node.part.label().to_owned(),
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
                    part: node.part.label().to_owned(),
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
}
