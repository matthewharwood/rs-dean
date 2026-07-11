use crate::scale;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BubbleSide {
    Incoming,
    Outgoing,
    System,
}

impl BubbleSide {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Incoming => "incoming",
            Self::Outgoing => "outgoing",
            Self::System => "system",
        }
    }
}

impl BubblePart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Bubble",
            Self::Avatar => "BubbleAvatar",
            Self::Content => "BubbleContent",
            Self::Meta => "BubbleMeta",
            Self::Actions => "BubbleActions",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BubbleAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BubbleModel {
    #[garde(skip)]
    pub side: BubbleSide,
    #[garde(length(min = 1, max = 160))]
    pub sender: String,
    #[garde(length(min = 1, max = 8))]
    pub avatar: String,
    #[garde(length(min = 1, max = 2_000))]
    pub content: String,
    #[garde(length(min = 1, max = 160))]
    pub meta: String,
    #[garde(length(max = 4), dive)]
    pub actions: Vec<BubbleAction>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BubbleState {
    active_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BubbleIntent {
    Activate(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BubbleChange {
    Activated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BubblePart {
    Root,
    Avatar,
    Content,
    Meta,
    Actions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BubbleRenderNode {
    pub part: BubblePart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub side: BubbleSide,
    pub active: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BubbleLayoutMetrics {
    pub avatar_size: f32,
    pub avatar_font_size: f32,
    pub root_gap: f32,
    pub panel_padding: f32,
    pub panel_gap: f32,
    pub content_font_size: f32,
    pub meta_font_size: f32,
    pub line_height: f32,
    pub actions_gap: f32,
    pub actions_padding_top: f32,
    pub action_min_height: f32,
    pub action_padding_inline: f32,
    pub action_padding_block: f32,
}

impl BubbleAction {
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

impl BubbleModel {
    pub fn new(
        sender: impl Into<String>,
        avatar: impl Into<String>,
        content: impl Into<String>,
        meta: impl Into<String>,
    ) -> Self {
        Self {
            side: BubbleSide::Incoming,
            sender: sender.into(),
            avatar: avatar.into(),
            content: content.into(),
            meta: meta.into(),
            actions: vec![BubbleAction::new("Reply", "reply")],
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_side(mut self, side: BubbleSide) -> Self {
        self.side = side;
        self
    }

    pub fn with_actions(mut self, actions: Vec<BubbleAction>) -> Self {
        self.actions = actions;
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

    pub const fn state(&self) -> BubbleState {
        let _ = self;
        BubbleState::idle()
    }
}

impl BubbleState {
    pub const fn idle() -> Self {
        Self {
            active_action: None,
        }
    }

    pub fn active_action(&self) -> Option<&str> {
        self.active_action.as_deref()
    }

    pub fn is_active(&self, value: &str) -> bool {
        self.active_action.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: BubbleIntent) -> BubbleChange {
        match intent {
            BubbleIntent::Activate(value) => self.activate(value),
            BubbleIntent::Clear => self.clear(),
        }
    }

    fn activate(&mut self, value: String) -> BubbleChange {
        if value.is_empty() || self.active_action.as_ref() == Some(&value) {
            BubbleChange::Unchanged
        } else {
            self.active_action = Some(value.clone());
            BubbleChange::Activated(value)
        }
    }

    fn clear(&mut self) -> BubbleChange {
        if self.active_action.is_some() {
            self.active_action = None;
            BubbleChange::Cleared
        } else {
            BubbleChange::Unchanged
        }
    }
}

pub fn validate_bubble_model(model: &BubbleModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn bubble_layout_metrics(inline_size: f32) -> BubbleLayoutMetrics {
    BubbleLayoutMetrics {
        avatar_size: scale::space::l(inline_size),
        avatar_font_size: scale::font_size::f00(inline_size),
        root_gap: scale::space::xs(inline_size),
        panel_padding: scale::space::s(inline_size),
        panel_gap: scale::space::xs2(inline_size),
        content_font_size: scale::font_size::f0(inline_size),
        meta_font_size: scale::font_size::f00(inline_size),
        line_height: scale::line_height::LH0,
        actions_gap: scale::space::xs2(inline_size),
        actions_padding_top: scale::space::xs2(inline_size),
        action_min_height: scale::space::s(inline_size),
        action_padding_inline: scale::space::xs2(inline_size),
        action_padding_block: scale::space::xs3(inline_size),
    }
}

pub fn bubble_render_nodes(model: &BubbleModel, state: &BubbleState) -> Vec<BubbleRenderNode> {
    let mut nodes = Vec::with_capacity(4 + model.actions.len().max(1));
    let root_disabled = model.disabled || model.loading;
    nodes.push(BubbleRenderNode {
        part: BubblePart::Root,
        value: "bubble".to_owned(),
        label: model.sender.clone(),
        detail: "Message bubble".to_owned(),
        side: model.side,
        active: false,
        loading: model.loading,
        disabled: root_disabled,
    });
    nodes.push(BubbleRenderNode {
        part: BubblePart::Avatar,
        value: model.avatar.clone(),
        label: model.sender.clone(),
        detail: "Sender avatar".to_owned(),
        side: model.side,
        active: false,
        loading: model.loading,
        disabled: root_disabled,
    });
    nodes.push(BubbleRenderNode {
        part: BubblePart::Content,
        value: model.content.clone(),
        label: "Message content".to_owned(),
        detail: model.content.clone(),
        side: model.side,
        active: false,
        loading: model.loading,
        disabled: root_disabled,
    });
    nodes.push(BubbleRenderNode {
        part: BubblePart::Meta,
        value: model.meta.clone(),
        label: "Message metadata".to_owned(),
        detail: model.meta.clone(),
        side: model.side,
        active: false,
        loading: model.loading,
        disabled: root_disabled,
    });

    if model.actions.is_empty() {
        nodes.push(BubbleRenderNode {
            part: BubblePart::Actions,
            value: "no-action".to_owned(),
            label: "No action".to_owned(),
            detail: "Message action".to_owned(),
            side: model.side,
            active: false,
            loading: model.loading,
            disabled: true,
        });
    } else {
        for action in &model.actions {
            nodes.push(BubbleRenderNode {
                part: BubblePart::Actions,
                value: action.value.clone(),
                label: action.label.clone(),
                detail: "Message action".to_owned(),
                side: model.side,
                active: state.is_active(&action.value),
                loading: model.loading,
                disabled: root_disabled || action.disabled,
            });
        }
    }

    nodes
}

pub fn default_bubble_model() -> BubbleModel {
    BubbleModel::new(
        "Codex",
        "AI",
        "The sweep is ready for review.",
        "Delivered now",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_bubble_model(&default_bubble_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_sender() {
        let model = BubbleModel::new("", "AI", "Message", "Delivered");
        assert!(validate_bubble_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_content() {
        let model = BubbleModel::new("Codex", "AI", "", "Delivered");
        assert!(validate_bubble_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_action_contract() {
        let model = BubbleModel::new("Codex", "AI", "Message", "Delivered")
            .with_actions(vec![BubbleAction::new("", "")]);
        assert!(validate_bubble_model(&model).is_err());
    }

    #[test]
    fn local_state_tracks_action_activation() {
        let mut state = BubbleState::idle();
        assert_eq!(state.active_action(), None);
        assert_eq!(
            state.apply(BubbleIntent::Activate("reply".to_owned())),
            BubbleChange::Activated("reply".to_owned())
        );
        assert_eq!(state.active_action(), Some("reply"));
        assert_eq!(state.apply(BubbleIntent::Clear), BubbleChange::Cleared);
    }

    #[test]
    fn layout_metrics_resolve_fluid_tailwind_tokens() {
        let compact = bubble_layout_metrics(320.0);
        let wide = bubble_layout_metrics(1_000.0);

        assert!(compact.avatar_size < wide.avatar_size);
        assert!(compact.content_font_size < wide.content_font_size);
        assert!(compact.panel_padding < wide.panel_padding);
        assert_eq!(wide.line_height, scale::line_height::LH0);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_bubble_model();
        let state = model.state();
        let nodes = bubble_render_nodes(&model, &state);
        assert_eq!(nodes.first().map(|node| node.part), Some(BubblePart::Root));
        for part in [
            BubblePart::Avatar,
            BubblePart::Content,
            BubblePart::Meta,
            BubblePart::Actions,
        ] {
            assert!(
                nodes.iter().any(|node| node.part == part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn empty_actions_keep_disabled_action_node() {
        let model = default_bubble_model().with_actions(Vec::new());
        let state = model.state();
        let nodes = bubble_render_nodes(&model, &state);
        assert!(
            nodes
                .iter()
                .any(|node| node.part == BubblePart::Actions && node.disabled)
        );
    }
}
