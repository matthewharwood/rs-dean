use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{MessageAction, MessageModel, MessageSide};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MessageScrollerDensity {
    Standard,
    Dense,
}

impl MessageScrollerDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MessageScrollerPart {
    Root,
    Viewport,
    List,
    Anchor,
    JumpButton,
}

impl MessageScrollerPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Viewport,
        Self::List,
        Self::Anchor,
        Self::JumpButton,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "MessageScroller",
            Self::Viewport => "MessageViewport",
            Self::List => "MessageList",
            Self::Anchor => "MessageAnchor",
            Self::JumpButton => "MessageJumpButton",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct MessageScrollerEntry {
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(dive)]
    pub message: MessageModel,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct MessageScrollerModel {
    #[garde(skip)]
    pub density: MessageScrollerDensity,
    #[garde(length(max = 12), dive, custom(message_scroller_entries_are_unique))]
    pub messages: Vec<MessageScrollerEntry>,
    #[garde(length(min = 1, max = 160))]
    pub anchor_label: String,
    #[garde(length(min = 1, max = 96))]
    pub jump_label: String,
    #[garde(skip)]
    pub at_latest: bool,
    #[garde(skip)]
    pub auto_scroll: bool,
    #[garde(custom(validate_optional_message_scroller_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageScrollerState {
    active_part: Option<MessageScrollerPart>,
    at_latest: bool,
    jump_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageScrollerIntent {
    Focus(MessageScrollerPart),
    Hover(MessageScrollerPart),
    SetAtLatest(bool),
    JumpToLatest,
    Blur,
    Leave,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageScrollerChange {
    Focused(MessageScrollerPart),
    Hovered(MessageScrollerPart),
    PositionChanged(bool),
    JumpedToLatest,
    Blurred,
    Left,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageScrollerRenderNode {
    pub part: MessageScrollerPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: MessageScrollerDensity,
    pub message_count: usize,
    pub at_latest: bool,
    pub active: bool,
    pub invalid: bool,
    pub visible: bool,
    pub actionable: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl MessageScrollerEntry {
    pub fn new(value: impl Into<String>, message: MessageModel) -> Self {
        Self {
            value: value.into(),
            message,
        }
    }
}

impl MessageScrollerModel {
    pub fn new(messages: Vec<MessageScrollerEntry>) -> Self {
        Self {
            density: MessageScrollerDensity::Standard,
            messages,
            anchor_label: "Latest message".to_owned(),
            jump_label: "Jump to latest".to_owned(),
            at_latest: false,
            auto_scroll: true,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: MessageScrollerDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_messages(mut self, messages: Vec<MessageScrollerEntry>) -> Self {
        self.messages = messages;
        self
    }

    pub fn without_messages(mut self) -> Self {
        self.messages = Vec::new();
        self
    }

    pub fn with_anchor_label(mut self, anchor_label: impl Into<String>) -> Self {
        self.anchor_label = anchor_label.into();
        self
    }

    pub fn with_jump_label(mut self, jump_label: impl Into<String>) -> Self {
        self.jump_label = jump_label.into();
        self
    }

    pub const fn with_at_latest(mut self, at_latest: bool) -> Self {
        self.at_latest = at_latest;
        self
    }

    pub const fn manual_scroll(mut self) -> Self {
        self.auto_scroll = false;
        self
    }

    pub const fn auto_scroll(mut self) -> Self {
        self.auto_scroll = true;
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

    pub const fn state(&self) -> MessageScrollerState {
        MessageScrollerState::new(self.at_latest)
    }
}

impl MessageScrollerState {
    pub const fn new(at_latest: bool) -> Self {
        Self {
            active_part: None,
            at_latest,
            jump_active: false,
        }
    }

    pub const fn is_active_part(&self, part: MessageScrollerPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub const fn at_latest(&self) -> bool {
        self.at_latest
    }

    pub const fn jump_active(&self) -> bool {
        self.jump_active
    }

    pub fn apply(&mut self, intent: MessageScrollerIntent) -> MessageScrollerChange {
        match intent {
            MessageScrollerIntent::Focus(part) => self.focus(part),
            MessageScrollerIntent::Hover(part) => self.hover(part),
            MessageScrollerIntent::SetAtLatest(value) => self.set_at_latest(value),
            MessageScrollerIntent::JumpToLatest => self.jump_to_latest(),
            MessageScrollerIntent::Blur => self.blur(),
            MessageScrollerIntent::Leave => self.leave(),
            MessageScrollerIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, part: MessageScrollerPart) -> MessageScrollerChange {
        if self.is_active_part(part) {
            MessageScrollerChange::Unchanged
        } else {
            self.active_part = Some(part);
            MessageScrollerChange::Focused(part)
        }
    }

    fn hover(&mut self, part: MessageScrollerPart) -> MessageScrollerChange {
        if self.is_active_part(part) {
            MessageScrollerChange::Unchanged
        } else {
            self.active_part = Some(part);
            MessageScrollerChange::Hovered(part)
        }
    }

    fn set_at_latest(&mut self, value: bool) -> MessageScrollerChange {
        if self.at_latest == value {
            MessageScrollerChange::Unchanged
        } else {
            self.at_latest = value;
            if !value {
                self.jump_active = false;
            }
            MessageScrollerChange::PositionChanged(value)
        }
    }

    fn jump_to_latest(&mut self) -> MessageScrollerChange {
        if self.at_latest {
            MessageScrollerChange::Unchanged
        } else {
            self.at_latest = true;
            self.jump_active = true;
            self.active_part = Some(MessageScrollerPart::JumpButton);
            MessageScrollerChange::JumpedToLatest
        }
    }

    fn blur(&mut self) -> MessageScrollerChange {
        if self.active_part.is_some() {
            self.active_part = None;
            MessageScrollerChange::Blurred
        } else {
            MessageScrollerChange::Unchanged
        }
    }

    fn leave(&mut self) -> MessageScrollerChange {
        if self.active_part.is_some() {
            self.active_part = None;
            MessageScrollerChange::Left
        } else {
            MessageScrollerChange::Unchanged
        }
    }

    fn clear(&mut self) -> MessageScrollerChange {
        if self.active_part.is_none() && !self.jump_active {
            MessageScrollerChange::Unchanged
        } else {
            self.active_part = None;
            self.jump_active = false;
            MessageScrollerChange::Cleared
        }
    }
}

pub fn validate_message_scroller_model(model: &MessageScrollerModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn message_scroller_render_nodes(
    model: &MessageScrollerModel,
    state: &MessageScrollerState,
) -> Vec<MessageScrollerRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.disabled || model.loading;
    let message_count = model.messages.len();
    let position = if state.at_latest() {
        "at latest"
    } else {
        "history available"
    };
    let jump_visible = !state.at_latest();
    let mut nodes = Vec::with_capacity(MessageScrollerPart::ALL.len());

    nodes.push(MessageScrollerRenderNode {
        part: MessageScrollerPart::Root,
        index: 0,
        value: message_count.to_string(),
        label: "Message scroller".to_owned(),
        detail: model
            .error
            .clone()
            .unwrap_or_else(|| format!("{message_count} messages, {position}")),
        density: model.density,
        message_count,
        at_latest: state.at_latest(),
        active: state.is_active_part(MessageScrollerPart::Root),
        invalid,
        visible: true,
        actionable: false,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(MessageScrollerRenderNode {
        part: MessageScrollerPart::Viewport,
        index: 0,
        value: "viewport".to_owned(),
        label: "Message viewport".to_owned(),
        detail: if model.auto_scroll {
            "Auto-scroll enabled".to_owned()
        } else {
            "Manual scroll".to_owned()
        },
        density: model.density,
        message_count,
        at_latest: state.at_latest(),
        active: state.is_active_part(MessageScrollerPart::Viewport),
        invalid,
        visible: true,
        actionable: false,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(MessageScrollerRenderNode {
        part: MessageScrollerPart::List,
        index: 0,
        value: message_count.to_string(),
        label: if message_count == 0 {
            "No messages".to_owned()
        } else {
            "Message list".to_owned()
        },
        detail: format!("{message_count} messages"),
        density: model.density,
        message_count,
        at_latest: state.at_latest(),
        active: state.is_active_part(MessageScrollerPart::List),
        invalid,
        visible: true,
        actionable: false,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(MessageScrollerRenderNode {
        part: MessageScrollerPart::Anchor,
        index: 0,
        value: "latest".to_owned(),
        label: model.anchor_label.clone(),
        detail: "Latest message anchor".to_owned(),
        density: model.density,
        message_count,
        at_latest: state.at_latest(),
        active: state.is_active_part(MessageScrollerPart::Anchor),
        invalid,
        visible: true,
        actionable: false,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(MessageScrollerRenderNode {
        part: MessageScrollerPart::JumpButton,
        index: 0,
        value: "jump-to-latest".to_owned(),
        label: model.jump_label.clone(),
        detail: position.to_owned(),
        density: model.density,
        message_count,
        at_latest: state.at_latest(),
        active: state.jump_active() || state.is_active_part(MessageScrollerPart::JumpButton),
        invalid,
        visible: jump_visible,
        actionable: true,
        loading: model.loading,
        disabled: blocked || !jump_visible,
    });

    nodes
}

pub fn default_message_scroller_model() -> MessageScrollerModel {
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
                "Keep the transcript durable and the scroll state local.",
                "Read",
            )
            .with_side(MessageSide::Outgoing)
            .with_actions(vec![MessageAction::new("Edit", "edit")]),
        ),
    ])
}

fn message_scroller_entries_are_unique(
    entries: &Vec<MessageScrollerEntry>,
    _context: &(),
) -> garde::Result {
    let mut values = HashSet::with_capacity(entries.len());
    for entry in entries {
        if !values.insert(entry.value.as_str()) {
            return Err(garde::Error::new(
                "message scroller entry values must be unique",
            ));
        }
    }
    Ok(())
}

fn validate_optional_message_scroller_error(
    error: &Option<String>,
    _context: &(),
) -> garde::Result {
    if let Some(error) = error
        && (error.is_empty() || error.len() > 240)
    {
        return Err(garde::Error::new(
            "message scroller optional error must be 1..=240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_message_scroller_model(&default_message_scroller_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_anchor_label() {
        let model = default_message_scroller_model().with_anchor_label("");
        assert!(validate_message_scroller_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_jump_label() {
        let model = default_message_scroller_model().with_jump_label("");
        assert!(validate_message_scroller_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_entry_value() {
        let model = MessageScrollerModel::new(vec![MessageScrollerEntry::new(
            "",
            MessageModel::new("Codex", "Now", "Message", "Delivered"),
        )]);
        assert!(validate_message_scroller_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_entry_values() {
        let message = MessageModel::new("Codex", "Now", "Message", "Delivered");
        let model = MessageScrollerModel::new(vec![
            MessageScrollerEntry::new("same", message.clone()),
            MessageScrollerEntry::new("same", message),
        ]);
        assert!(validate_message_scroller_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_nested_message() {
        let model = MessageScrollerModel::new(vec![MessageScrollerEntry::new(
            "invalid",
            MessageModel::new("", "Now", "Message", "Delivered"),
        )]);
        assert!(validate_message_scroller_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_message_scroller_model().with_error("");
        assert!(validate_message_scroller_model(&model).is_err());
    }

    #[test]
    fn state_tracks_position_jump_focus_and_clear() {
        let mut state = MessageScrollerState::new(false);
        assert_eq!(
            state.apply(MessageScrollerIntent::Focus(MessageScrollerPart::Viewport)),
            MessageScrollerChange::Focused(MessageScrollerPart::Viewport)
        );
        assert!(state.is_active_part(MessageScrollerPart::Viewport));
        assert_eq!(
            state.apply(MessageScrollerIntent::SetAtLatest(true)),
            MessageScrollerChange::PositionChanged(true)
        );
        assert!(state.at_latest());
        assert_eq!(
            state.apply(MessageScrollerIntent::SetAtLatest(false)),
            MessageScrollerChange::PositionChanged(false)
        );
        assert_eq!(
            state.apply(MessageScrollerIntent::JumpToLatest),
            MessageScrollerChange::JumpedToLatest
        );
        assert!(state.at_latest());
        assert!(state.jump_active());
        assert_eq!(
            state.apply(MessageScrollerIntent::Clear),
            MessageScrollerChange::Cleared
        );
        assert!(!state.jump_active());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_message_scroller_model();
        let nodes = message_scroller_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(MessageScrollerPart::Root)
        );
        for part in MessageScrollerPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn at_latest_hides_jump_button() {
        let model = default_message_scroller_model().with_at_latest(true);
        let nodes = message_scroller_render_nodes(&model, &model.state());
        let jump = nodes
            .iter()
            .find(|node| node.part == MessageScrollerPart::JumpButton)
            .expect("message scroller includes jump button node");
        assert!(!jump.visible);
        assert!(jump.disabled);
    }

    #[test]
    fn loading_disables_jump_button() {
        let model = default_message_scroller_model().loading();
        let nodes = message_scroller_render_nodes(&model, &model.state());
        let jump = nodes
            .iter()
            .find(|node| node.part == MessageScrollerPart::JumpButton)
            .expect("message scroller includes jump button node");
        assert!(jump.disabled);
    }

    #[test]
    fn empty_messages_keep_list_node_visible() {
        let model = default_message_scroller_model().without_messages();
        let nodes = message_scroller_render_nodes(&model, &model.state());
        let list = nodes
            .iter()
            .find(|node| node.part == MessageScrollerPart::List)
            .expect("message scroller includes list node");
        assert!(list.visible);
        assert_eq!(list.message_count, 0);
    }
}
