use crate::scale;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BadgeTone {
    Default,
    Brand,
    Info,
    Success,
    Warning,
    Destructive,
    Muted,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BadgeVariant {
    Soft,
    Solid,
    Outline,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BadgeSize {
    Small,
    Medium,
}

impl BadgeTone {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Brand => "brand",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Destructive => "destructive",
            Self::Muted => "muted",
        }
    }
}

impl BadgeVariant {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Soft => "soft",
            Self::Solid => "solid",
            Self::Outline => "outline",
        }
    }
}

impl BadgeSize {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "md",
        }
    }
}

impl BadgePart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Badge",
            Self::Icon => "BadgeIcon",
            Self::Text => "BadgeText",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BadgeModel {
    #[garde(skip)]
    pub tone: BadgeTone,
    #[garde(skip)]
    pub variant: BadgeVariant,
    #[garde(skip)]
    pub size: BadgeSize,
    #[garde(length(min = 1, max = 96))]
    pub text: String,
    #[garde(custom(validate_optional_badge_icon))]
    pub icon: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadgeState {
    highlighted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadgeIntent {
    Highlight,
    ClearHighlight,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadgeChange {
    Highlighted,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgePart {
    Root,
    Icon,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadgeRenderNode {
    pub part: BadgePart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub tone: BadgeTone,
    pub variant: BadgeVariant,
    pub size: BadgeSize,
    pub highlighted: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BadgeLayoutMetrics {
    pub min_height: f32,
    pub gap: f32,
    pub padding_inline: f32,
    pub padding_block: f32,
    pub font_size: f32,
    pub line_height: f32,
    pub icon_min_width: f32,
}

impl BadgeModel {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            tone: BadgeTone::Success,
            variant: BadgeVariant::Soft,
            size: BadgeSize::Medium,
            text: text.into(),
            icon: Some("Live".to_owned()),
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_tone(mut self, tone: BadgeTone) -> Self {
        self.tone = tone;
        self
    }

    pub const fn with_variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn without_icon(mut self) -> Self {
        self.icon = None;
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

    pub const fn state(&self) -> BadgeState {
        let _ = self;
        BadgeState::resting()
    }
}

impl BadgeState {
    pub const fn resting() -> Self {
        Self { highlighted: false }
    }

    pub const fn highlighted() -> Self {
        Self { highlighted: true }
    }

    pub const fn is_highlighted(self) -> bool {
        self.highlighted
    }

    pub fn apply(&mut self, intent: BadgeIntent) -> BadgeChange {
        match intent {
            BadgeIntent::Highlight => self.highlight(),
            BadgeIntent::ClearHighlight => self.clear(),
        }
    }

    fn highlight(&mut self) -> BadgeChange {
        if self.highlighted {
            BadgeChange::Unchanged
        } else {
            self.highlighted = true;
            BadgeChange::Highlighted
        }
    }

    fn clear(&mut self) -> BadgeChange {
        if self.highlighted {
            self.highlighted = false;
            BadgeChange::Cleared
        } else {
            BadgeChange::Unchanged
        }
    }
}

pub fn validate_badge_model(model: &BadgeModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn badge_icon_copy(model: &BadgeModel) -> Option<&str> {
    model
        .icon
        .as_deref()
        .map(|icon| if model.loading { "..." } else { icon })
}

pub fn badge_text_copy(model: &BadgeModel) -> &str {
    if model.loading {
        "Loading"
    } else {
        &model.text
    }
}

pub fn badge_layout_metrics(size: BadgeSize, inline_size: f32) -> BadgeLayoutMetrics {
    match size {
        BadgeSize::Small => BadgeLayoutMetrics {
            min_height: scale::space::s(inline_size),
            gap: scale::space::xs3(inline_size),
            padding_inline: scale::space::xs2(inline_size),
            padding_block: scale::space::xs3(inline_size),
            font_size: scale::font_size::f00(inline_size),
            line_height: scale::line_height::LH0,
            icon_min_width: scale::space::s(inline_size),
        },
        BadgeSize::Medium => BadgeLayoutMetrics {
            min_height: 40.0,
            gap: scale::space::xs2(inline_size),
            padding_inline: scale::space::xs(inline_size),
            padding_block: scale::space::xs2(inline_size),
            font_size: scale::font_size::f0(inline_size),
            line_height: scale::line_height::LH0,
            icon_min_width: scale::space::s(inline_size),
        },
    }
}

pub fn badge_render_nodes(model: &BadgeModel, state: BadgeState) -> Vec<BadgeRenderNode> {
    let highlighted = state.is_highlighted();
    let icon_value = model
        .icon
        .clone()
        .unwrap_or_else(|| "badge-icon-missing".to_owned());

    vec![
        BadgeRenderNode {
            part: BadgePart::Root,
            value: "badge".to_owned(),
            label: model.text.clone(),
            detail: format!("{} {} badge", model.variant.label(), model.tone.label()),
            tone: model.tone,
            variant: model.variant,
            size: model.size,
            highlighted,
            loading: model.loading,
            disabled: model.disabled,
        },
        BadgeRenderNode {
            part: BadgePart::Icon,
            value: icon_value.clone(),
            label: icon_value,
            detail: "Badge icon".to_owned(),
            tone: model.tone,
            variant: model.variant,
            size: model.size,
            highlighted,
            loading: model.loading,
            disabled: model.disabled || model.loading || model.icon.is_none(),
        },
        BadgeRenderNode {
            part: BadgePart::Text,
            value: model.text.clone(),
            label: model.text.clone(),
            detail: "Badge text".to_owned(),
            tone: model.tone,
            variant: model.variant,
            size: model.size,
            highlighted,
            loading: model.loading,
            disabled: model.disabled,
        },
    ]
}

pub fn default_badge_model() -> BadgeModel {
    BadgeModel::new("Ready")
}

fn validate_optional_badge_icon(icon: &Option<String>, _context: &()) -> garde::Result {
    if let Some(icon) = icon
        && !(1..=16).contains(&icon.chars().count())
    {
        return Err(garde::Error::new("badge icon must be 1 to 16 characters"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_badge_model(&default_badge_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_text() {
        let model = BadgeModel::new("");
        assert!(validate_badge_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_icon() {
        let model = BadgeModel::new("Ready").with_icon("");
        assert!(validate_badge_model(&model).is_err());
    }

    #[test]
    fn state_tracks_renderer_local_highlight() {
        let mut state = BadgeState::resting();
        assert!(!state.is_highlighted());
        assert_eq!(
            state.apply(BadgeIntent::Highlight),
            BadgeChange::Highlighted
        );
        assert!(state.is_highlighted());
        assert_eq!(
            state.apply(BadgeIntent::ClearHighlight),
            BadgeChange::Cleared
        );
        assert!(!state.is_highlighted());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_badge_model();
        let nodes = badge_render_nodes(&model, model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(BadgePart::Root));
        assert!(nodes.iter().any(|node| node.part == BadgePart::Icon));
        assert!(nodes.iter().any(|node| node.part == BadgePart::Text));
    }

    #[test]
    fn missing_icon_keeps_disabled_icon_node() {
        let model = default_badge_model().without_icon();
        let nodes = badge_render_nodes(&model, model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == BadgePart::Icon && node.disabled)
        );
    }

    #[test]
    fn display_copy_and_layout_follow_shared_state() {
        let loading = BadgeModel::new("Syncing").loading();
        let compact = badge_layout_metrics(BadgeSize::Medium, 320.0);
        let wide = badge_layout_metrics(BadgeSize::Medium, 1_000.0);

        assert_eq!(badge_icon_copy(&loading), Some("..."));
        assert_eq!(badge_text_copy(&loading), "Loading");
        assert!(wide.padding_inline > compact.padding_inline);
        assert!(wide.font_size > compact.font_size);
    }
}
