use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CollapsibleDensity {
    Standard,
    Dense,
}

impl CollapsibleDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl CollapsiblePart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Trigger, Self::Content];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Collapsible",
            Self::Trigger => "CollapsibleTrigger",
            Self::Content => "CollapsibleContent",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CollapsibleModel {
    #[garde(skip)]
    pub density: CollapsibleDensity,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 2_000))]
    pub content: String,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollapsibleState {
    open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsibleIntent {
    Toggle,
    Open,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsibleChange {
    Opened,
    Closed,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CollapsiblePart {
    Root,
    Trigger,
    Content,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollapsibleRenderNode {
    pub part: CollapsiblePart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: CollapsibleDensity,
    pub open: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollapsibleLayoutMetrics {
    pub max_width: f32,
    pub trigger_min_height: f32,
    pub trigger_padding_inline: f32,
    pub trigger_padding_block: f32,
    pub trigger_gap: f32,
    pub trigger_font_size: f32,
    pub trigger_line_height: f32,
    pub indicator_size: f32,
    pub indicator_font_size: f32,
    pub indicator_line_height: f32,
    pub content_padding: f32,
    pub content_font_size: f32,
    pub content_line_height: f32,
}

impl CollapsibleModel {
    pub fn new(
        value: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            density: CollapsibleDensity::Standard,
            value: value.into(),
            title: title.into(),
            content: content.into(),
            default_open: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: CollapsibleDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub const fn open(mut self) -> Self {
        self.default_open = true;
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

    pub const fn state(&self) -> CollapsibleState {
        CollapsibleState::new(self.default_open)
    }
}

impl CollapsibleState {
    pub const fn new(open: bool) -> Self {
        Self { open }
    }

    pub const fn is_open(self) -> bool {
        self.open
    }

    pub fn apply(&mut self, intent: CollapsibleIntent) -> CollapsibleChange {
        match intent {
            CollapsibleIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            CollapsibleIntent::Open => self.open(),
            CollapsibleIntent::Close => self.close(),
        }
    }

    fn open(&mut self) -> CollapsibleChange {
        if self.open {
            CollapsibleChange::Unchanged
        } else {
            self.open = true;
            CollapsibleChange::Opened
        }
    }

    fn close(&mut self) -> CollapsibleChange {
        if self.open {
            self.open = false;
            CollapsibleChange::Closed
        } else {
            CollapsibleChange::Unchanged
        }
    }
}

pub fn validate_collapsible_model(model: &CollapsibleModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn collapsible_layout_metrics(
    density: CollapsibleDensity,
    disabled: bool,
    open: bool,
    inline_size: f32,
) -> CollapsibleLayoutMetrics {
    let dense_trigger = density == CollapsibleDensity::Dense && !disabled && !open;
    let (trigger_padding_inline, trigger_padding_block, trigger_gap, trigger_font_size) =
        if dense_trigger {
            (
                scale::space::xs(inline_size),
                scale::space::xs2(inline_size),
                scale::space::xs2(inline_size),
                scale::font_size::f00(inline_size),
            )
        } else {
            (
                scale::space::s(inline_size),
                scale::space::xs(inline_size),
                scale::space::xs(inline_size),
                scale::font_size::f0(inline_size),
            )
        };
    let dense_content = density == CollapsibleDensity::Dense;

    CollapsibleLayoutMetrics {
        max_width: scale::container::CONTROL,
        trigger_min_height: scale::space::FIELD,
        trigger_padding_inline,
        trigger_padding_block,
        trigger_gap,
        trigger_font_size,
        trigger_line_height: scale::line_height::LH0,
        indicator_size: scale::space::s(inline_size),
        indicator_font_size: scale::font_size::f00(inline_size),
        indicator_line_height: scale::line_height::LH00,
        content_padding: if dense_content {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        content_font_size: if dense_content {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        content_line_height: scale::line_height::LH0,
    }
}

pub fn collapsible_render_nodes(
    model: &CollapsibleModel,
    state: &CollapsibleState,
) -> Vec<CollapsibleRenderNode> {
    let open = state.is_open();
    let blocked = model.loading || model.disabled;
    vec![
        CollapsibleRenderNode {
            part: CollapsiblePart::Root,
            value: model.value.clone(),
            label: model.title.clone(),
            detail: format!(
                "{} {} collapsible",
                model.density.label(),
                if open { "open" } else { "closed" },
            ),
            density: model.density,
            open,
            loading: model.loading,
            disabled: blocked,
        },
        CollapsibleRenderNode {
            part: CollapsiblePart::Trigger,
            value: model.value.clone(),
            label: model.title.clone(),
            detail: if open {
                "Collapse content".to_owned()
            } else {
                "Expand content".to_owned()
            },
            density: model.density,
            open,
            loading: model.loading,
            disabled: blocked,
        },
        CollapsibleRenderNode {
            part: CollapsiblePart::Content,
            value: model.value.clone(),
            label: "Collapsible content".to_owned(),
            detail: model.content.clone(),
            density: model.density,
            open,
            loading: model.loading,
            disabled: blocked || !open,
        },
    ]
}

pub fn default_collapsible_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "token-stack",
        "Shared token stack",
        "The same Rust disclosure state feeds the Leptos DOM surface and Bevy primitive adapter.",
    )
    .open()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_collapsible_model(&default_collapsible_model()).is_ok());
    }

    #[test]
    fn layout_metrics_share_fluid_tailwind_tokens_and_state_rules() {
        let compact = collapsible_layout_metrics(CollapsibleDensity::Standard, false, false, 320.0);
        let wide = collapsible_layout_metrics(CollapsibleDensity::Standard, false, false, 1_000.0);
        let dense_closed =
            collapsible_layout_metrics(CollapsibleDensity::Dense, false, false, 1_000.0);
        let dense_open =
            collapsible_layout_metrics(CollapsibleDensity::Dense, false, true, 1_000.0);
        let dense_disabled =
            collapsible_layout_metrics(CollapsibleDensity::Dense, true, false, 1_000.0);

        assert!(compact.trigger_padding_inline < wide.trigger_padding_inline);
        assert!(dense_closed.trigger_padding_inline < wide.trigger_padding_inline);
        assert_eq!(
            dense_open.trigger_padding_inline,
            wide.trigger_padding_inline
        );
        assert_eq!(
            dense_disabled.trigger_padding_inline,
            wide.trigger_padding_inline
        );
        assert!(dense_closed.content_padding < wide.content_padding);
        assert_eq!(wide.max_width, scale::container::CONTROL);
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = CollapsibleModel::new("", "Shared token stack", "Content");
        assert!(validate_collapsible_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = CollapsibleModel::new("token-stack", "", "Content");
        assert!(validate_collapsible_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_content() {
        let model = CollapsibleModel::new("token-stack", "Shared token stack", "");
        assert!(validate_collapsible_model(&model).is_err());
    }

    #[test]
    fn state_opens_closes_and_toggles() {
        let mut state = CollapsibleState::new(false);
        assert!(!state.is_open());
        assert_eq!(
            state.apply(CollapsibleIntent::Open),
            CollapsibleChange::Opened
        );
        assert!(state.is_open());
        assert_eq!(
            state.apply(CollapsibleIntent::Toggle),
            CollapsibleChange::Closed
        );
        assert!(!state.is_open());
        assert_eq!(
            state.apply(CollapsibleIntent::Close),
            CollapsibleChange::Unchanged
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_collapsible_model();
        let nodes = collapsible_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(CollapsiblePart::Root)
        );
        for part in CollapsiblePart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn closed_content_node_is_disabled() {
        let model = default_collapsible_model().default_open(false);
        let nodes = collapsible_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == CollapsiblePart::Content && node.disabled)
        );
    }

    #[test]
    fn loading_disables_trigger_node() {
        let model = default_collapsible_model().loading();
        let nodes = collapsible_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == CollapsiblePart::Trigger && node.disabled)
        );
    }
}
