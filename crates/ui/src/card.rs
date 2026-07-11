use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CardVariant {
    Elevated,
    Outline,
    Ghost,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CardDensity {
    Standard,
    Dense,
}

impl CardVariant {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Elevated => "elevated",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
        }
    }
}

impl CardDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl CardPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Header,
        Self::Title,
        Self::Description,
        Self::Content,
        Self::Footer,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Card",
            Self::Header => "CardHeader",
            Self::Title => "CardTitle",
            Self::Description => "CardDescription",
            Self::Content => "CardContent",
            Self::Footer => "CardFooter",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CardAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CardModel {
    #[garde(skip)]
    pub variant: CardVariant,
    #[garde(skip)]
    pub density: CardDensity,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 320))]
    pub description: String,
    #[garde(length(min = 1, max = 2_000))]
    pub content: String,
    #[garde(length(min = 1, max = 240))]
    pub footer: String,
    #[garde(custom(validate_optional_card_action))]
    pub action: Option<CardAction>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardState {
    active_part: Option<CardPart>,
    activated_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardIntent {
    Focus(CardPart),
    Blur(CardPart),
    ActivateFooter(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardChange {
    Focused(CardPart),
    Blurred(CardPart),
    Activated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CardPart {
    Root,
    Header,
    Title,
    Description,
    Content,
    Footer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardRenderNode {
    pub part: CardPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub variant: CardVariant,
    pub density: CardDensity,
    pub active: bool,
    pub actionable: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CardLayoutMetrics {
    pub width: f32,
    pub height: f32,
    pub border_width: f32,
    pub padding: f32,
    pub gap: f32,
    pub header_gap: f32,
    pub header_height: f32,
    pub title_height: f32,
    pub title_font_size: f32,
    pub title_line_height: f32,
    pub description_height: f32,
    pub copy_font_size: f32,
    pub copy_line_height: f32,
    pub content_padding: f32,
    pub content_height: f32,
    pub content_text_height: f32,
    pub footer_padding_top: f32,
    pub footer_gap: f32,
    pub footer_height: f32,
    pub footer_text_width: f32,
    pub footer_text_height: f32,
    pub footer_font_size: f32,
    pub footer_line_height: f32,
    pub footer_wraps: bool,
    pub action_width: f32,
    pub action_height: f32,
    pub action_font_size: f32,
    pub action_line_height: f32,
    pub action_padding_inline: f32,
    pub action_padding_block: f32,
}

impl CardAction {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl CardModel {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        content: impl Into<String>,
        footer: impl Into<String>,
    ) -> Self {
        Self {
            variant: CardVariant::Elevated,
            density: CardDensity::Standard,
            title: title.into(),
            description: description.into(),
            content: content.into(),
            footer: footer.into(),
            action: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_density(mut self, density: CardDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_action(mut self, action: CardAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn without_action(mut self) -> Self {
        self.action = None;
        self
    }

    pub const fn loading(mut self) -> Self {
        self.loading = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn state(&self) -> CardState {
        let _ = self;
        CardState::resting()
    }
}

impl CardState {
    pub const fn resting() -> Self {
        Self {
            active_part: None,
            activated_value: None,
        }
    }

    pub const fn is_active(&self, part: CardPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn activated_value(&self) -> Option<&str> {
        self.activated_value.as_deref()
    }

    pub fn apply(&mut self, intent: CardIntent) -> CardChange {
        match intent {
            CardIntent::Focus(part) => self.focus(part),
            CardIntent::Blur(part) => self.blur(part),
            CardIntent::ActivateFooter(value) => {
                if value.is_empty() {
                    CardChange::Unchanged
                } else {
                    self.active_part = Some(CardPart::Footer);
                    self.activated_value = Some(value.clone());
                    CardChange::Activated(value)
                }
            }
            CardIntent::Clear => {
                if self.active_part.is_none() && self.activated_value.is_none() {
                    CardChange::Unchanged
                } else {
                    self.active_part = None;
                    self.activated_value = None;
                    CardChange::Cleared
                }
            }
        }
    }

    fn focus(&mut self, part: CardPart) -> CardChange {
        if self.is_active(part) {
            CardChange::Unchanged
        } else {
            self.active_part = Some(part);
            CardChange::Focused(part)
        }
    }

    fn blur(&mut self, part: CardPart) -> CardChange {
        if self.is_active(part) {
            self.active_part = None;
            CardChange::Blurred(part)
        } else {
            CardChange::Unchanged
        }
    }
}

pub fn validate_card_model(model: &CardModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn card_layout_metrics(
    model: &CardModel,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> CardLayoutMetrics {
    let border_width = border_width.max(0.0);
    let width = available_width.clamp(1.0, scale::container::CONTROL);
    let dense = model.density == CardDensity::Dense;
    let padding = if dense {
        scale::space::xs(inline_size)
    } else {
        scale::space::s(inline_size)
    };
    let gap = padding;
    let header_gap = if dense {
        scale::space::xs3(inline_size)
    } else {
        scale::space::xs2(inline_size)
    };
    let content_padding = padding;
    let footer_padding_top = padding;
    let footer_gap = if dense {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let inner_width = (width - border_width * 2.0 - padding * 2.0).max(1.0);

    let (title_font_size, title_line_height, copy_font_size) = if dense {
        (
            scale::font_size::f0(inline_size),
            scale::line_height::LH0,
            scale::font_size::f00(inline_size),
        )
    } else {
        (
            scale::font_size::f1(inline_size),
            scale::line_height::LH2,
            scale::font_size::f0(inline_size),
        )
    };
    let title_height = scale::estimate_text_block_height(
        &model.title,
        inner_width,
        title_font_size,
        title_line_height,
        0.52,
    );
    let description_height = scale::estimate_text_block_height(
        &model.description,
        inner_width,
        copy_font_size,
        scale::line_height::LH0,
        0.52,
    );
    let header_height = title_height + header_gap + description_height;

    let content_text_width = (inner_width - border_width * 2.0 - content_padding * 2.0).max(1.0);
    let content_text_height = scale::estimate_text_block_height(
        &model.content,
        content_text_width,
        copy_font_size,
        scale::line_height::LH0,
        0.52,
    );
    let content_height = content_text_height + content_padding * 2.0 + border_width * 2.0;

    let copy_line_height = scale::line_height::LH0;
    let footer_font_size = scale::font_size::f00(inline_size);
    let footer_line_height = scale::line_height::LH00;
    let footer_text_width =
        scale::estimate_inline_text_width(&model.footer.to_uppercase(), footer_font_size, 0.08);
    let footer_text_height = footer_font_size * footer_line_height;
    let action_font_size = scale::font_size::f0(inline_size);
    let action_line_height = scale::line_height::LH0;
    let action_padding_inline = scale::space::xs(inline_size);
    let action_padding_block = scale::space::xs2(inline_size);
    let (action_width, action_height) = model.action.as_ref().map_or((0.0, 0.0), |action| {
        (
            scale::estimate_inline_text_width(&action.label, action_font_size, 0.0)
                + action_padding_inline * 2.0
                + border_width * 2.0,
            (action_font_size * action_line_height
                + action_padding_block * 2.0
                + border_width * 2.0)
                .max(40.0),
        )
    });
    let footer_wraps =
        action_width > 0.0 && footer_text_width + footer_gap + action_width > inner_width;
    let footer_content_height = if footer_wraps {
        footer_text_height + footer_gap + action_height
    } else {
        footer_text_height.max(action_height)
    };
    let footer_height = border_width + footer_padding_top + footer_content_height;
    let height = border_width * 2.0
        + padding * 2.0
        + header_height
        + content_height
        + footer_height
        + gap * 2.0;

    CardLayoutMetrics {
        width,
        height,
        border_width,
        padding,
        gap,
        header_gap,
        header_height,
        title_height,
        title_font_size,
        title_line_height,
        description_height,
        copy_font_size,
        copy_line_height,
        content_padding,
        content_height,
        content_text_height,
        footer_padding_top,
        footer_gap,
        footer_height,
        footer_text_width,
        footer_text_height,
        footer_font_size,
        footer_line_height,
        footer_wraps,
        action_width,
        action_height,
        action_font_size,
        action_line_height,
        action_padding_inline,
        action_padding_block,
    }
}

pub fn card_render_nodes(model: &CardModel, state: &CardState) -> Vec<CardRenderNode> {
    let action = model.action.as_ref();
    let action_disabled = action.is_some_and(|action| action.disabled);
    vec![
        card_node(
            model,
            state,
            CardNodeDraft::new(
                CardPart::Root,
                "card",
                "Card",
                &model.title,
                false,
                model.disabled,
            ),
        ),
        card_node(
            model,
            state,
            CardNodeDraft::new(
                CardPart::Header,
                "card-header",
                "Card header",
                &model.title,
                false,
                model.disabled,
            ),
        ),
        card_node(
            model,
            state,
            CardNodeDraft::new(
                CardPart::Title,
                "card-title",
                &model.title,
                "Card title",
                false,
                model.disabled,
            ),
        ),
        card_node(
            model,
            state,
            CardNodeDraft::new(
                CardPart::Description,
                "card-description",
                "Card description",
                &model.description,
                false,
                model.disabled,
            ),
        ),
        card_node(
            model,
            state,
            CardNodeDraft::new(
                CardPart::Content,
                "card-content",
                "Card content",
                &model.content,
                false,
                model.disabled,
            ),
        ),
        card_node(
            model,
            state,
            CardNodeDraft::new(
                CardPart::Footer,
                action
                    .map(|action| action.value.as_str())
                    .unwrap_or("card-footer"),
                &model.footer,
                action
                    .map(|action| action.label.as_str())
                    .unwrap_or("Card footer"),
                action.is_some(),
                model.disabled || model.loading || action_disabled,
            ),
        ),
    ]
}

pub fn default_card_model() -> CardModel {
    CardModel::new(
        "Design system",
        "64 components share one token contract.",
        "Implementation notes stay portable across Leptos DOM and Bevy scene primitives.",
        "Sweep process ready",
    )
    .with_action(CardAction::new("Open checklist", "open-checklist"))
}

struct CardNodeDraft<'a> {
    part: CardPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    actionable: bool,
    disabled: bool,
}

impl<'a> CardNodeDraft<'a> {
    const fn new(
        part: CardPart,
        value: &'a str,
        label: &'a str,
        detail: &'a str,
        actionable: bool,
        disabled: bool,
    ) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            actionable,
            disabled,
        }
    }
}

fn card_node(model: &CardModel, state: &CardState, draft: CardNodeDraft<'_>) -> CardRenderNode {
    CardRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        variant: model.variant,
        density: model.density,
        active: state.is_active(draft.part),
        actionable: draft.actionable,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn validate_optional_card_action(action: &Option<CardAction>, _context: &()) -> garde::Result {
    if let Some(action) = action {
        action.validate().map_err(|report| {
            garde::Error::new(format!("invalid card action contract: {report}"))
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_card_model(&default_card_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = CardModel::new("", "Description", "Content", "Footer");
        assert!(validate_card_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_action_contract() {
        let model = default_card_model().with_action(CardAction::new("", "invalid-empty-action"));
        assert!(validate_card_model(&model).is_err());
    }

    #[test]
    fn layout_metrics_preserve_the_token_control_cap_and_intrinsic_height() {
        let metrics = card_layout_metrics(&default_card_model(), 960.0, 1_000.0, 1.0);
        let thick_metrics = card_layout_metrics(&default_card_model(), 960.0, 1_000.0, 2.0);

        assert_eq!(metrics.width, scale::container::CONTROL);
        assert!(metrics.height > scale::space::xl3(1_000.0) * 2.0);
        assert!(metrics.content_height > metrics.content_text_height);
        assert!(!metrics.footer_wraps);
        assert!(
            ((thick_metrics.height - metrics.height) - 7.0).abs() <= f32::EPSILON,
            "the root, content, footer, and footer action borders must all contribute to intrinsic height",
        );
    }

    #[test]
    fn state_tracks_focus_and_footer_activation() {
        let mut state = CardState::resting();
        assert!(!state.is_active(CardPart::Footer));
        assert_eq!(
            state.apply(CardIntent::Focus(CardPart::Footer)),
            CardChange::Focused(CardPart::Footer)
        );
        assert!(state.is_active(CardPart::Footer));
        assert_eq!(
            state.apply(CardIntent::ActivateFooter("open-checklist".to_owned())),
            CardChange::Activated("open-checklist".to_owned())
        );
        assert_eq!(state.activated_value(), Some("open-checklist"));
        assert_eq!(
            state.apply(CardIntent::Blur(CardPart::Footer)),
            CardChange::Blurred(CardPart::Footer)
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_card_model();
        let nodes = card_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), CardPart::ALL.len());
        assert_eq!(nodes.first().map(|node| node.part), Some(CardPart::Root));
        for part in CardPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert!(
            nodes
                .iter()
                .any(|node| node.part == CardPart::Footer && node.actionable)
        );
    }

    #[test]
    fn disabled_action_disables_footer_node() {
        let model = default_card_model().with_action(CardAction::new("Open", "open").disabled());
        let nodes = card_render_nodes(&model, &model.state());
        let footer = nodes
            .into_iter()
            .find(|node| node.part == CardPart::Footer)
            .expect("card render nodes include footer");
        assert!(footer.disabled);
    }
}
