use crate::{ComponentDefinition, UiComponentCategory, UiComponentId, UiStateModel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        RenderContract, StateContract, Theme, UiBlockRole, UiBlockTone, UiComponentId,
        UiWidgetIntent, UiWidgetSlotKind, accordion_render_nodes, alert_dialog_render_nodes,
        alert_render_nodes, component_implementation, default_accordion_items,
        default_alert_dialog_model, default_alert_model, scale, widget_for_component,
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
        widget_for_component(id)
            .slots
            .into_iter()
            .map(|slot| BevyUiPrimitive {
                part: slot.part.to_owned(),
                kind: slot.kind,
                role: slot.role,
                label: slot.label.to_owned(),
                value: slot.value.to_owned(),
                size: size_for_role(slot.role),
                fill: fill_for_tone(slot.tone, theme),
                text: theme.text_1().to_bevy(),
                render: implementation.render,
                state: implementation.state,
                intent: slot.intent,
                selected: slot.selected,
                disabled: slot.disabled,
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
