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
        BreadcrumbPart, RenderContract, StateContract, Theme, UiBlockRole, UiBlockTone,
        UiComponentId, UiWidgetIntent, UiWidgetSlotKind, accordion_render_nodes,
        alert_dialog_render_nodes, alert_render_nodes, aspect_ratio_render_nodes,
        attachment_render_nodes, avatar_render_nodes, badge_render_nodes, breadcrumb_render_nodes,
        catalog_component_any_render_nodes_for_component, component_implementation,
        default_accordion_items, default_alert_dialog_model, default_alert_model,
        default_aspect_ratio_model, default_attachment_model, default_avatar_model,
        default_badge_model, default_breadcrumb_model, scale,
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
