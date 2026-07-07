use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::dom::ui_dom_id;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AlertDialogSize {
    Default,
    Small,
}

impl AlertDialogSize {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Small => "sm",
        }
    }
}

impl AlertDialogPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "AlertDialog",
            Self::Trigger => "AlertDialogTrigger",
            Self::Content => "AlertDialogContent",
            Self::Header => "AlertDialogHeader",
            Self::Footer => "AlertDialogFooter",
            Self::Action => "AlertDialogAction",
            Self::Cancel => "AlertDialogCancel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AlertDialogButton {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AlertDialogModel {
    #[garde(skip)]
    pub size: AlertDialogSize,
    #[garde(length(min = 1, max = 96))]
    pub trigger_label: String,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 2_000))]
    pub description: String,
    #[garde(dive, custom(confirm_value_differs_from_cancel(&self.cancel)))]
    pub action: AlertDialogButton,
    #[garde(dive)]
    pub cancel: AlertDialogButton,
    #[garde(skip)]
    pub destructive: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlertDialogState {
    open: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertDialogIntent {
    Open,
    Close,
    Cancel,
    Confirm(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertDialogChange {
    Opened,
    Closed,
    Cancelled,
    Confirmed(String),
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertDialogPart {
    Root,
    Trigger,
    Content,
    Header,
    Footer,
    Action,
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlertDialogRenderNode {
    pub part: AlertDialogPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub open: bool,
    pub size: AlertDialogSize,
    pub destructive: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl AlertDialogButton {
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

impl AlertDialogModel {
    pub fn new(
        trigger_label: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        action: AlertDialogButton,
        cancel: AlertDialogButton,
    ) -> Self {
        Self {
            size: AlertDialogSize::Default,
            trigger_label: trigger_label.into(),
            title: title.into(),
            description: description.into(),
            action,
            cancel,
            destructive: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_size(mut self, size: AlertDialogSize) -> Self {
        self.size = size;
        self
    }

    pub const fn destructive(mut self) -> Self {
        self.destructive = true;
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
}

impl AlertDialogState {
    pub const fn closed() -> Self {
        Self { open: false }
    }

    pub const fn open() -> Self {
        Self { open: true }
    }

    pub const fn is_open(self) -> bool {
        self.open
    }

    pub fn apply(&mut self, intent: AlertDialogIntent) -> AlertDialogChange {
        match intent {
            AlertDialogIntent::Open => self.open_dialog(),
            AlertDialogIntent::Close => self.close_dialog(),
            AlertDialogIntent::Cancel => self.cancel(),
            AlertDialogIntent::Confirm(value) => self.confirm(value),
        }
    }

    pub fn open_dialog(&mut self) -> AlertDialogChange {
        if self.open {
            AlertDialogChange::Unchanged
        } else {
            self.open = true;
            AlertDialogChange::Opened
        }
    }

    pub fn close_dialog(&mut self) -> AlertDialogChange {
        if self.open {
            self.open = false;
            AlertDialogChange::Closed
        } else {
            AlertDialogChange::Unchanged
        }
    }

    pub fn cancel(&mut self) -> AlertDialogChange {
        if self.open {
            self.open = false;
            AlertDialogChange::Cancelled
        } else {
            AlertDialogChange::Unchanged
        }
    }

    pub fn confirm(&mut self, value: String) -> AlertDialogChange {
        if value.is_empty() || !self.open {
            AlertDialogChange::Unchanged
        } else {
            self.open = false;
            AlertDialogChange::Confirmed(value)
        }
    }
}

pub fn validate_alert_dialog_model(model: &AlertDialogModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn alert_dialog_render_nodes(
    model: &AlertDialogModel,
    state: AlertDialogState,
) -> Vec<AlertDialogRenderNode> {
    let open = state.is_open();
    let base_disabled = model.disabled;
    let action_disabled = base_disabled || model.loading || model.action.disabled;
    let cancel_disabled = base_disabled || model.loading || model.cancel.disabled;

    vec![
        AlertDialogRenderNode {
            part: AlertDialogPart::Root,
            value: "alert-dialog".to_owned(),
            label: "Alert Dialog".to_owned(),
            detail: "Blocking confirmation dialog".to_owned(),
            open,
            size: model.size,
            destructive: model.destructive,
            loading: model.loading,
            disabled: base_disabled,
        },
        AlertDialogRenderNode {
            part: AlertDialogPart::Trigger,
            value: "alert-dialog-trigger".to_owned(),
            label: model.trigger_label.clone(),
            detail: "Open confirmation".to_owned(),
            open,
            size: model.size,
            destructive: model.destructive,
            loading: model.loading,
            disabled: base_disabled,
        },
        AlertDialogRenderNode {
            part: AlertDialogPart::Content,
            value: "alert-dialog-content".to_owned(),
            label: model.title.clone(),
            detail: model.description.clone(),
            open,
            size: model.size,
            destructive: model.destructive,
            loading: model.loading,
            disabled: base_disabled,
        },
        AlertDialogRenderNode {
            part: AlertDialogPart::Header,
            value: "alert-dialog-header".to_owned(),
            label: model.title.clone(),
            detail: model.description.clone(),
            open,
            size: model.size,
            destructive: model.destructive,
            loading: model.loading,
            disabled: base_disabled,
        },
        AlertDialogRenderNode {
            part: AlertDialogPart::Footer,
            value: "alert-dialog-footer".to_owned(),
            label: "Choose how to continue".to_owned(),
            detail: "Confirmation actions".to_owned(),
            open,
            size: model.size,
            destructive: model.destructive,
            loading: model.loading,
            disabled: base_disabled,
        },
        AlertDialogRenderNode {
            part: AlertDialogPart::Action,
            value: model.action.value.clone(),
            label: model.action.label.clone(),
            detail: "Confirm action".to_owned(),
            open,
            size: model.size,
            destructive: model.destructive,
            loading: model.loading,
            disabled: action_disabled,
        },
        AlertDialogRenderNode {
            part: AlertDialogPart::Cancel,
            value: model.cancel.value.clone(),
            label: model.cancel.label.clone(),
            detail: "Cancel action".to_owned(),
            open,
            size: model.size,
            destructive: false,
            loading: model.loading,
            disabled: cancel_disabled,
        },
    ]
}

pub fn alert_dialog_dom_id(prefix: &str, value: &str) -> String {
    ui_dom_id(prefix, value)
}

pub fn default_alert_dialog_model() -> AlertDialogModel {
    AlertDialogModel::new(
        "Delete draft",
        "Delete this draft?",
        "This cannot be undone. The draft and its local review state will be removed.",
        AlertDialogButton::new("Delete", "delete-draft"),
        AlertDialogButton::new("Cancel", "cancel"),
    )
    .destructive()
}

fn confirm_value_differs_from_cancel<'a>(
    cancel: &'a AlertDialogButton,
) -> impl FnOnce(&AlertDialogButton, &()) -> garde::Result + 'a {
    move |action, _context| {
        if action.value == cancel.value {
            Err(garde::Error::new(
                "confirmation and cancel actions must use distinct values",
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_alert_dialog_model(&default_alert_dialog_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = AlertDialogModel::new(
            "Open",
            "",
            "Description",
            AlertDialogButton::new("Confirm", "confirm"),
            AlertDialogButton::new("Cancel", "cancel"),
        );
        assert!(validate_alert_dialog_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_action_values() {
        let model = AlertDialogModel::new(
            "Open",
            "Title",
            "Description",
            AlertDialogButton::new("Confirm", "same"),
            AlertDialogButton::new("Cancel", "same"),
        );
        assert!(validate_alert_dialog_model(&model).is_err());
    }

    #[test]
    fn state_transitions_open_cancel_and_confirm() {
        let mut state = AlertDialogState::closed();
        assert_eq!(
            state.apply(AlertDialogIntent::Open),
            AlertDialogChange::Opened
        );
        assert!(state.is_open());
        assert_eq!(
            state.apply(AlertDialogIntent::Cancel),
            AlertDialogChange::Cancelled
        );
        assert!(!state.is_open());
        assert_eq!(
            state.apply(AlertDialogIntent::Open),
            AlertDialogChange::Opened
        );
        assert_eq!(
            state.apply(AlertDialogIntent::Confirm("delete-draft".to_owned())),
            AlertDialogChange::Confirmed("delete-draft".to_owned())
        );
        assert!(!state.is_open());
    }

    #[test]
    fn render_nodes_cover_local_alert_dialog_anatomy() {
        let model = default_alert_dialog_model();
        let nodes = alert_dialog_render_nodes(&model, AlertDialogState::open());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(AlertDialogPart::Root)
        );
        assert_eq!(
            nodes.first().map(|node| node.label.as_str()),
            Some("Alert Dialog")
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AlertDialogPart::Trigger)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AlertDialogPart::Content)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AlertDialogPart::Header)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AlertDialogPart::Footer)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AlertDialogPart::Action)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AlertDialogPart::Cancel)
        );
    }

    #[test]
    fn loading_model_disables_action_nodes() {
        let model = default_alert_dialog_model().loading();
        let nodes = alert_dialog_render_nodes(&model, AlertDialogState::open());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AlertDialogPart::Action && node.disabled)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AlertDialogPart::Cancel && node.disabled)
        );
    }
}
