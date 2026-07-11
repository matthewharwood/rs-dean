use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{dom::ui_dom_id, scale};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlertDialogLayoutMetrics {
    pub width: f32,
    pub root_gap: f32,
    pub trigger_height: f32,
    pub trigger_padding_inline: f32,
    pub trigger_padding_block: f32,
    pub overlay_padding: f32,
    pub overlay_height: f32,
    pub content_width: f32,
    pub content_height: f32,
    pub content_padding: f32,
    pub content_gap: f32,
    pub header_gap: f32,
    pub header_height: f32,
    pub title_height: f32,
    pub description_height: f32,
    pub title_font_size: f32,
    pub description_font_size: f32,
    pub footer_gap: f32,
    pub footer_height: f32,
    pub action_width: f32,
    pub cancel_width: f32,
    pub button_height: f32,
    pub height: f32,
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

pub fn alert_dialog_layout_metrics(
    model: &AlertDialogModel,
    open: bool,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> AlertDialogLayoutMetrics {
    let border_width = border_width.max(0.0);
    let width = available_width.max(1.0);
    let small = model.size == AlertDialogSize::Small;
    let root_gap = scale::space::xs(inline_size);
    let trigger_padding_inline = scale::space::xs(inline_size);
    let trigger_padding_block = scale::space::xs2(inline_size);
    let trigger_font_size = scale::font_size::f0(inline_size);
    let trigger_height = (trigger_font_size * scale::line_height::LH0
        + trigger_padding_block * 2.0
        + border_width * 2.0)
        .max(40.0);
    let overlay_padding = scale::space::s(inline_size);
    let content_padding = if small {
        scale::space::xs(inline_size)
    } else {
        scale::space::s(inline_size)
    };
    let content_gap = content_padding;
    let header_gap = scale::space::xs2(inline_size);
    let footer_gap = scale::space::xs2(inline_size);
    let title_font_size = if small {
        scale::font_size::f0(inline_size)
    } else {
        scale::font_size::f1(inline_size)
    };
    let title_line_height = if small {
        scale::line_height::LH0
    } else {
        scale::line_height::LH2
    };
    let description_font_size = if small {
        scale::font_size::f00(inline_size)
    } else {
        scale::font_size::f0(inline_size)
    };
    let content_width = (width - overlay_padding * 2.0).max(1.0);
    let text_width = (content_width - content_padding * 2.0 - border_width * 2.0).max(1.0);
    let title_height = scale::estimate_text_block_height(
        &model.title,
        text_width,
        title_font_size,
        title_line_height,
        0.52,
    );
    let description_height = scale::estimate_text_block_height(
        &model.description,
        text_width,
        description_font_size,
        scale::line_height::LH0,
        0.52,
    );
    let header_height = title_height + header_gap + description_height;
    let button_font_size = scale::font_size::f0(inline_size);
    let button_height = (button_font_size * scale::line_height::LH0
        + trigger_padding_block * 2.0
        + border_width * 2.0)
        .max(40.0);
    let button_width = |label: &str| {
        scale::estimate_inline_text_width(label, button_font_size, 0.0)
            + trigger_padding_inline * 2.0
            + border_width * 2.0
    };
    let action_label = if model.loading {
        "Working"
    } else {
        &model.action.label
    };
    let action_width = button_width(action_label);
    let cancel_width = button_width(&model.cancel.label);
    let footer_height = button_height;
    let content_height =
        content_padding * 2.0 + border_width * 2.0 + header_height + content_gap + footer_height;
    let overlay_height = content_height + overlay_padding * 2.0;
    let height = if open {
        trigger_height + root_gap + overlay_height
    } else {
        trigger_height
    };

    AlertDialogLayoutMetrics {
        width,
        root_gap,
        trigger_height,
        trigger_padding_inline,
        trigger_padding_block,
        overlay_padding,
        overlay_height,
        content_width,
        content_height,
        content_padding,
        content_gap,
        header_gap,
        header_height,
        title_height,
        description_height,
        title_font_size,
        description_font_size,
        footer_gap,
        footer_height,
        action_width,
        cancel_width,
        button_height,
        height,
    }
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

    #[test]
    fn layout_metrics_follow_open_state_and_size_tokens() {
        let model = default_alert_dialog_model();
        let closed = alert_dialog_layout_metrics(&model, false, 448.0, 1_000.0, 1.0);
        let open = alert_dialog_layout_metrics(&model, true, 448.0, 1_000.0, 1.0);
        let small = alert_dialog_layout_metrics(
            &model.with_size(AlertDialogSize::Small),
            true,
            448.0,
            1_000.0,
            1.0,
        );

        assert_eq!(closed.height, closed.trigger_height);
        assert!(open.height > closed.height);
        assert!(small.content_padding < open.content_padding);
        assert!(open.action_width > 0.0);
    }
}
