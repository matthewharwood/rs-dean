use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MessageDensity {
    Standard,
    Dense,
}

impl MessageDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MessageSide {
    Incoming,
    Outgoing,
    System,
}

impl MessageSide {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Incoming => "incoming",
            Self::Outgoing => "outgoing",
            Self::System => "system",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MessagePart {
    Root,
    Header,
    Content,
    Footer,
    Actions,
}

impl MessagePart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Header,
        Self::Content,
        Self::Footer,
        Self::Actions,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Message",
            Self::Header => "MessageHeader",
            Self::Content => "MessageContent",
            Self::Footer => "MessageFooter",
            Self::Actions => "MessageActions",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct MessageAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct MessageModel {
    #[garde(skip)]
    pub density: MessageDensity,
    #[garde(skip)]
    pub side: MessageSide,
    #[garde(length(min = 1, max = 160))]
    pub sender: String,
    #[garde(length(min = 1, max = 160))]
    pub meta: String,
    #[garde(length(min = 1, max = 2_000))]
    pub content: String,
    #[garde(length(min = 1, max = 160))]
    pub footer: String,
    #[garde(length(max = 4), dive, custom(message_actions_are_unique))]
    pub actions: Vec<MessageAction>,
    #[garde(custom(validate_optional_message_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageState {
    active_part: Option<MessagePart>,
    active_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageIntent {
    Focus(MessagePart),
    Hover(MessagePart),
    Activate(String),
    Blur,
    Leave,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageChange {
    Focused(MessagePart),
    Hovered(MessagePart),
    Activated(String),
    Blurred,
    Left,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageRenderNode {
    pub part: MessagePart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: MessageDensity,
    pub side: MessageSide,
    pub active: bool,
    pub invalid: bool,
    pub visible: bool,
    pub actionable: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl MessageAction {
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

impl MessageModel {
    pub fn new(
        sender: impl Into<String>,
        meta: impl Into<String>,
        content: impl Into<String>,
        footer: impl Into<String>,
    ) -> Self {
        Self {
            density: MessageDensity::Standard,
            side: MessageSide::Incoming,
            sender: sender.into(),
            meta: meta.into(),
            content: content.into(),
            footer: footer.into(),
            actions: vec![MessageAction::new("Reply", "reply")],
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: MessageDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_side(mut self, side: MessageSide) -> Self {
        self.side = side;
        self
    }

    pub fn with_actions(mut self, actions: Vec<MessageAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn without_actions(mut self) -> Self {
        self.actions = Vec::new();
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn without_error(mut self) -> Self {
        self.error = None;
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

    pub const fn state(&self) -> MessageState {
        let _ = self;
        MessageState::resting()
    }
}

impl MessageState {
    pub const fn resting() -> Self {
        Self {
            active_part: None,
            active_action: None,
        }
    }

    pub const fn is_active_part(&self, part: MessagePart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn active_action(&self) -> Option<&str> {
        self.active_action.as_deref()
    }

    pub fn is_active_action(&self, value: &str) -> bool {
        self.active_action.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: MessageIntent) -> MessageChange {
        match intent {
            MessageIntent::Focus(part) => self.focus(part),
            MessageIntent::Hover(part) => self.hover(part),
            MessageIntent::Activate(value) => self.activate(value),
            MessageIntent::Blur => self.blur(),
            MessageIntent::Leave => self.leave(),
            MessageIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, part: MessagePart) -> MessageChange {
        if self.is_active_part(part) {
            MessageChange::Unchanged
        } else {
            self.active_part = Some(part);
            MessageChange::Focused(part)
        }
    }

    fn hover(&mut self, part: MessagePart) -> MessageChange {
        if self.is_active_part(part) {
            MessageChange::Unchanged
        } else {
            self.active_part = Some(part);
            MessageChange::Hovered(part)
        }
    }

    fn activate(&mut self, value: String) -> MessageChange {
        if value.is_empty() || self.active_action.as_ref() == Some(&value) {
            MessageChange::Unchanged
        } else {
            self.active_part = Some(MessagePart::Actions);
            self.active_action = Some(value.clone());
            MessageChange::Activated(value)
        }
    }

    fn blur(&mut self) -> MessageChange {
        if self.active_part.is_some() {
            self.active_part = None;
            MessageChange::Blurred
        } else {
            MessageChange::Unchanged
        }
    }

    fn leave(&mut self) -> MessageChange {
        if self.active_part.is_some() {
            self.active_part = None;
            MessageChange::Left
        } else {
            MessageChange::Unchanged
        }
    }

    fn clear(&mut self) -> MessageChange {
        if self.active_part.is_none() && self.active_action.is_none() {
            MessageChange::Unchanged
        } else {
            self.active_part = None;
            self.active_action = None;
            MessageChange::Cleared
        }
    }
}

pub fn validate_message_model(model: &MessageModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn message_render_nodes(model: &MessageModel, state: &MessageState) -> Vec<MessageRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.disabled || model.loading;
    let mut nodes = Vec::with_capacity(4 + model.actions.len().max(1));

    nodes.push(MessageRenderNode {
        part: MessagePart::Root,
        index: 0,
        value: model.sender.clone(),
        label: model.sender.clone(),
        detail: model
            .error
            .clone()
            .unwrap_or_else(|| format!("Message from {}", model.sender)),
        density: model.density,
        side: model.side,
        active: state.is_active_part(MessagePart::Root),
        invalid,
        visible: true,
        actionable: false,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(MessageRenderNode {
        part: MessagePart::Header,
        index: 0,
        value: model.meta.clone(),
        label: model.sender.clone(),
        detail: model.meta.clone(),
        density: model.density,
        side: model.side,
        active: state.is_active_part(MessagePart::Header),
        invalid,
        visible: true,
        actionable: false,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(MessageRenderNode {
        part: MessagePart::Content,
        index: 0,
        value: model.content.clone(),
        label: "Message content".to_owned(),
        detail: model.content.clone(),
        density: model.density,
        side: model.side,
        active: state.is_active_part(MessagePart::Content),
        invalid,
        visible: true,
        actionable: false,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(MessageRenderNode {
        part: MessagePart::Footer,
        index: 0,
        value: model.footer.clone(),
        label: model.footer.clone(),
        detail: model.error.clone().unwrap_or_else(|| model.footer.clone()),
        density: model.density,
        side: model.side,
        active: state.is_active_part(MessagePart::Footer),
        invalid,
        visible: true,
        actionable: false,
        loading: model.loading,
        disabled: blocked,
    });

    if model.actions.is_empty() {
        nodes.push(MessageRenderNode {
            part: MessagePart::Actions,
            index: 0,
            value: "no-action".to_owned(),
            label: "No action".to_owned(),
            detail: "Message action unavailable".to_owned(),
            density: model.density,
            side: model.side,
            active: false,
            invalid,
            visible: false,
            actionable: false,
            loading: model.loading,
            disabled: true,
        });
    } else {
        for (index, action) in model.actions.iter().enumerate() {
            nodes.push(MessageRenderNode {
                part: MessagePart::Actions,
                index,
                value: action.value.clone(),
                label: action.label.clone(),
                detail: "Message action".to_owned(),
                density: model.density,
                side: model.side,
                active: state.is_active_action(&action.value),
                invalid,
                visible: true,
                actionable: true,
                loading: model.loading,
                disabled: blocked || action.disabled,
            });
        }
    }

    nodes
}

pub fn default_message_model() -> MessageModel {
    MessageModel::new(
        "Codex",
        "Today at 9:41",
        "The UI sweep is ready for the next component.",
        "Delivered",
    )
}

fn message_actions_are_unique(actions: &Vec<MessageAction>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(actions.len());
    for action in actions {
        if !values.insert(action.value.as_str()) {
            return Err(garde::Error::new("message action values must be unique"));
        }
    }
    Ok(())
}

fn validate_optional_message_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && (error.is_empty() || error.len() > 240)
    {
        return Err(garde::Error::new(
            "message optional error must be 1..=240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_message_model(&default_message_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_sender() {
        let model = MessageModel::new("", "Now", "Content", "Delivered");
        assert!(validate_message_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_meta() {
        let model = MessageModel::new("Codex", "", "Content", "Delivered");
        assert!(validate_message_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_content() {
        let model = MessageModel::new("Codex", "Now", "", "Delivered");
        assert!(validate_message_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_footer() {
        let model = MessageModel::new("Codex", "Now", "Content", "");
        assert!(validate_message_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_action_contract() {
        let model = default_message_model().with_actions(vec![MessageAction::new("", "")]);
        assert!(validate_message_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_action_values() {
        let model = default_message_model().with_actions(vec![
            MessageAction::new("Reply", "reply"),
            MessageAction::new("Retry", "reply"),
        ]);
        assert!(validate_message_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_message_model().with_error("");
        assert!(validate_message_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_hover_activation_blur_leave_and_clear() {
        let mut state = MessageState::resting();
        assert_eq!(
            state.apply(MessageIntent::Focus(MessagePart::Header)),
            MessageChange::Focused(MessagePart::Header)
        );
        assert!(state.is_active_part(MessagePart::Header));
        assert_eq!(
            state.apply(MessageIntent::Hover(MessagePart::Content)),
            MessageChange::Hovered(MessagePart::Content)
        );
        assert!(state.is_active_part(MessagePart::Content));
        assert_eq!(
            state.apply(MessageIntent::Activate("reply".to_owned())),
            MessageChange::Activated("reply".to_owned())
        );
        assert_eq!(state.active_action(), Some("reply"));
        assert_eq!(state.apply(MessageIntent::Blur), MessageChange::Blurred);
        assert_eq!(
            state.apply(MessageIntent::Hover(MessagePart::Footer)),
            MessageChange::Hovered(MessagePart::Footer)
        );
        assert_eq!(state.apply(MessageIntent::Leave), MessageChange::Left);
        assert_eq!(state.apply(MessageIntent::Clear), MessageChange::Cleared);
        assert_eq!(state.active_action(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_message_model();
        let nodes = message_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(MessagePart::Root));
        for part in MessagePart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn empty_actions_keep_hidden_disabled_action_node() {
        let model = default_message_model().without_actions();
        let nodes = message_render_nodes(&model, &model.state());
        assert!(nodes.iter().any(|node| {
            node.part == MessagePart::Actions && !node.visible && node.disabled && !node.actionable
        }));
    }

    #[test]
    fn loading_disables_action_nodes() {
        let model = default_message_model().loading();
        let nodes = message_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == MessagePart::Actions)
                .all(|node| node.disabled)
        );
    }

    #[test]
    fn action_nodes_keep_stable_indexes() {
        let model = default_message_model().with_actions(vec![
            MessageAction::new("Reply", "reply"),
            MessageAction::new("Resolve", "resolve"),
        ]);
        let nodes = message_render_nodes(&model, &model.state());
        let resolve = nodes
            .iter()
            .find(|node| node.value == "resolve")
            .expect("message includes resolve action");
        assert_eq!(resolve.index, 1);
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_message_model().with_error("Message failed to send.");
        let nodes = message_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
    }
}
