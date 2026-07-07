use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DialogSize {
    Default,
    Small,
}

impl DialogSize {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Small => "small",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DialogMode {
    Modal,
    NonModal,
}

impl DialogMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Modal => "modal",
            Self::NonModal => "non-modal",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DialogPart {
    Root,
    Trigger,
    Content,
    Header,
    Title,
    Description,
    Footer,
}

impl DialogPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Trigger,
        Self::Content,
        Self::Header,
        Self::Title,
        Self::Description,
        Self::Footer,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Dialog",
            Self::Trigger => "DialogTrigger",
            Self::Content => "DialogContent",
            Self::Header => "DialogHeader",
            Self::Title => "DialogTitle",
            Self::Description => "DialogDescription",
            Self::Footer => "DialogFooter",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DialogAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub close_dialog: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DialogModel {
    #[garde(skip)]
    pub size: DialogSize,
    #[garde(skip)]
    pub mode: DialogMode,
    #[garde(length(min = 1, max = 96))]
    pub trigger_label: String,
    #[garde(length(min = 1, max = 128))]
    pub title: String,
    #[garde(length(min = 1, max = 240))]
    pub description: String,
    #[garde(length(min = 1, max = 600))]
    pub body: String,
    #[garde(length(max = 3), dive, custom(dialog_actions_are_valid))]
    pub actions: Vec<DialogAction>,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogState {
    open: bool,
    focused: bool,
    active_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogIntent {
    Open,
    Close,
    Toggle,
    Focus,
    Blur,
    ActivateFooter(String),
    ClearAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogChange {
    Opened,
    Closed,
    Focused,
    Blurred,
    FooterActivated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogRenderNode {
    pub part: DialogPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub size: DialogSize,
    pub mode: DialogMode,
    pub open: bool,
    pub focused: bool,
    pub actionable: bool,
    pub close_dialog: bool,
    pub selected: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl DialogAction {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            close_dialog: true,
        }
    }

    pub const fn keep_open(mut self) -> Self {
        self.close_dialog = false;
        self
    }
}

impl DialogModel {
    pub fn new(
        trigger_label: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            size: DialogSize::Default,
            mode: DialogMode::Modal,
            trigger_label: trigger_label.into(),
            title: title.into(),
            description: description.into(),
            body: body.into(),
            actions: vec![
                DialogAction::new("Save changes", "save"),
                DialogAction::new("Close", "close"),
            ],
            default_open: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    pub const fn with_mode(mut self, mode: DialogMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_actions(mut self, actions: Vec<DialogAction>) -> Self {
        self.actions = actions;
        self
    }

    pub const fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
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

    pub const fn state(&self) -> DialogState {
        DialogState {
            open: self.default_open,
            focused: false,
            active_action: None,
        }
    }
}

impl DialogState {
    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn active_action(&self) -> Option<&str> {
        self.active_action.as_deref()
    }

    pub fn apply(&mut self, intent: DialogIntent) -> DialogChange {
        match intent {
            DialogIntent::Open => self.open(),
            DialogIntent::Close => self.close(),
            DialogIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            DialogIntent::Focus => self.focus(),
            DialogIntent::Blur => self.blur(),
            DialogIntent::ActivateFooter(value) => self.activate_footer(value),
            DialogIntent::ClearAction => self.clear_action(),
        }
    }

    fn open(&mut self) -> DialogChange {
        if self.open {
            DialogChange::Unchanged
        } else {
            self.open = true;
            DialogChange::Opened
        }
    }

    fn close(&mut self) -> DialogChange {
        if self.open {
            self.open = false;
            DialogChange::Closed
        } else {
            DialogChange::Unchanged
        }
    }

    fn focus(&mut self) -> DialogChange {
        if self.focused {
            DialogChange::Unchanged
        } else {
            self.focused = true;
            DialogChange::Focused
        }
    }

    fn blur(&mut self) -> DialogChange {
        if self.focused {
            self.focused = false;
            DialogChange::Blurred
        } else {
            DialogChange::Unchanged
        }
    }

    fn activate_footer(&mut self, value: String) -> DialogChange {
        if value.is_empty() || self.active_action.as_ref() == Some(&value) {
            return DialogChange::Unchanged;
        }
        self.active_action = Some(value.clone());
        DialogChange::FooterActivated(value)
    }

    fn clear_action(&mut self) -> DialogChange {
        if self.active_action.is_none() {
            DialogChange::Unchanged
        } else {
            self.active_action = None;
            DialogChange::Cleared
        }
    }
}

pub fn validate_dialog_model(model: &DialogModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn dialog_render_nodes(model: &DialogModel, state: &DialogState) -> Vec<DialogRenderNode> {
    let blocked = model.disabled || model.loading;
    let visible = state.is_open();
    let mut nodes = Vec::with_capacity(model.actions.len().saturating_add(6));
    nodes.push(DialogRenderNode {
        part: DialogPart::Root,
        index: 0,
        value: model.title.clone(),
        label: model.title.clone(),
        detail: model.description.clone(),
        size: model.size,
        mode: model.mode,
        open: state.is_open(),
        focused: state.is_focused(),
        actionable: false,
        close_dialog: false,
        selected: state.is_open(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DialogRenderNode {
        part: DialogPart::Trigger,
        index: 0,
        value: model.trigger_label.clone(),
        label: model.trigger_label.clone(),
        detail: "Open dialog".to_owned(),
        size: model.size,
        mode: model.mode,
        open: state.is_open(),
        focused: state.is_focused(),
        actionable: true,
        close_dialog: false,
        selected: state.is_open(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DialogRenderNode {
        part: DialogPart::Content,
        index: 0,
        value: model.title.clone(),
        label: model.title.clone(),
        detail: model.body.clone(),
        size: model.size,
        mode: model.mode,
        open: state.is_open(),
        focused: false,
        actionable: false,
        close_dialog: false,
        selected: false,
        visible,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DialogRenderNode {
        part: DialogPart::Header,
        index: 0,
        value: "header".to_owned(),
        label: model.title.clone(),
        detail: model.description.clone(),
        size: model.size,
        mode: model.mode,
        open: state.is_open(),
        focused: false,
        actionable: false,
        close_dialog: false,
        selected: false,
        visible,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DialogRenderNode {
        part: DialogPart::Title,
        index: 0,
        value: model.title.clone(),
        label: model.title.clone(),
        detail: model.title.clone(),
        size: model.size,
        mode: model.mode,
        open: state.is_open(),
        focused: false,
        actionable: false,
        close_dialog: false,
        selected: false,
        visible,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DialogRenderNode {
        part: DialogPart::Description,
        index: 0,
        value: model.description.clone(),
        label: model.description.clone(),
        detail: model.description.clone(),
        size: model.size,
        mode: model.mode,
        open: state.is_open(),
        focused: false,
        actionable: false,
        close_dialog: false,
        selected: false,
        visible,
        loading: model.loading,
        disabled: blocked,
    });
    for (index, action) in model.actions.iter().enumerate() {
        nodes.push(DialogRenderNode {
            part: DialogPart::Footer,
            index,
            value: action.value.clone(),
            label: action.label.clone(),
            detail: if action.close_dialog {
                "Close dialog action".to_owned()
            } else {
                "Keep dialog open action".to_owned()
            },
            size: model.size,
            mode: model.mode,
            open: state.is_open(),
            focused: false,
            actionable: true,
            close_dialog: action.close_dialog,
            selected: state.active_action() == Some(action.value.as_str()),
            visible,
            loading: model.loading,
            disabled: blocked,
        });
    }
    if model.actions.is_empty() {
        nodes.push(DialogRenderNode {
            part: DialogPart::Footer,
            index: 0,
            value: "empty".to_owned(),
            label: "No actions".to_owned(),
            detail: "Footer has no actions".to_owned(),
            size: model.size,
            mode: model.mode,
            open: state.is_open(),
            focused: false,
            actionable: false,
            close_dialog: false,
            selected: false,
            visible,
            loading: model.loading,
            disabled: true,
        });
    }
    nodes
}

pub fn default_dialog_model() -> DialogModel {
    DialogModel::new(
        "Edit profile",
        "Edit profile",
        "Make changes to your profile here.",
        "Update the fields, then save the changes when you are done.",
    )
}

fn dialog_actions_are_valid(actions: &Vec<DialogAction>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(actions.len());
    for action in actions {
        if !values.insert(action.value.as_str()) {
            return Err(garde::Error::new("dialog action values must be unique"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_dialog_model(&default_dialog_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = DialogModel::new("Open", "", "Description", "Body");
        assert!(validate_dialog_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_action_values() {
        let model = DialogModel::new("Open", "Title", "Description", "Body").with_actions(vec![
            DialogAction::new("Save", "same"),
            DialogAction::new("Close", "same"),
        ]);
        assert!(validate_dialog_model(&model).is_err());
    }

    #[test]
    fn state_opens_focuses_activates_and_closes() {
        let mut state = default_dialog_model().state();
        assert_eq!(state.apply(DialogIntent::Open), DialogChange::Opened);
        assert_eq!(state.apply(DialogIntent::Focus), DialogChange::Focused);
        assert_eq!(
            state.apply(DialogIntent::ActivateFooter("save".to_owned())),
            DialogChange::FooterActivated("save".to_owned())
        );
        assert_eq!(state.active_action(), Some("save"));
        assert_eq!(state.apply(DialogIntent::Close), DialogChange::Closed);
    }

    #[test]
    fn render_nodes_cover_repeatable_dialog_anatomy() {
        let model = default_dialog_model().with_default_open(true);
        let nodes = dialog_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(DialogPart::Root));
        for part in DialogPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == DialogPart::Footer)
                .count(),
            2
        );
    }

    #[test]
    fn closed_state_hides_content_nodes() {
        let model = default_dialog_model();
        let nodes = dialog_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| {
                    matches!(
                        node.part,
                        DialogPart::Content
                            | DialogPart::Header
                            | DialogPart::Title
                            | DialogPart::Description
                            | DialogPart::Footer
                    )
                })
                .all(|node| !node.visible)
        );
    }

    #[test]
    fn loading_disables_action_nodes() {
        let model = default_dialog_model().with_default_open(true).loading();
        let nodes = dialog_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == DialogPart::Footer && node.disabled)
        );
    }
}
