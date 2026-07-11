use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{dom::ui_dom_id, scale};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToastDensity {
    Standard,
    Dense,
}

impl ToastDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToastPosition {
    TopRight,
    BottomRight,
    BottomCenter,
}

impl ToastPosition {
    pub const fn label(self) -> &'static str {
        match self {
            Self::TopRight => "top-right",
            Self::BottomRight => "bottom-right",
            Self::BottomCenter => "bottom-center",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToastTone {
    Default,
    Info,
    Success,
    Warning,
    Destructive,
}

impl ToastTone {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Destructive => "destructive",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToastPart {
    Provider,
    Viewport,
    Toast,
    Title,
    Description,
    Action,
}

impl ToastPart {
    pub const ALL: &'static [Self] = &[
        Self::Provider,
        Self::Viewport,
        Self::Toast,
        Self::Title,
        Self::Description,
        Self::Action,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Provider => "ToastProvider",
            Self::Viewport => "ToastViewport",
            Self::Toast => "Toast",
            Self::Title => "ToastTitle",
            Self::Description => "ToastDescription",
            Self::Action => "ToastAction",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ToastAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ToastModel {
    #[garde(skip)]
    pub density: ToastDensity,
    #[garde(skip)]
    pub position: ToastPosition,
    #[garde(skip)]
    pub tone: ToastTone,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 480))]
    pub description: String,
    #[garde(custom(validate_optional_toast_action))]
    pub action: Option<ToastAction>,
    #[garde(range(min = 1_000, max = 60_000))]
    pub duration_ms: u32,
    #[garde(custom(validate_optional_toast_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToastState {
    open: bool,
    focused: bool,
    paused: bool,
    actioned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToastIntent {
    Open,
    Close,
    Toggle,
    Focus,
    Blur,
    Pause,
    Resume,
    Activate(String),
    ResetOpen(bool),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToastChange {
    Opened,
    Closed,
    Focused,
    Blurred,
    Paused,
    Resumed,
    Activated(String),
    Reset(bool),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToastRenderNode {
    pub part: ToastPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: ToastDensity,
    pub position: ToastPosition,
    pub tone: ToastTone,
    pub open: bool,
    pub focused: bool,
    pub paused: bool,
    pub actioned: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToastLayoutMetrics {
    pub max_width: f32,
    pub provider_gap: f32,
    pub provider_padding: f32,
    pub meta_font_size: f32,
    pub meta_line_height: f32,
    pub meta_letter_spacing: f32,
    pub viewport_gap: f32,
    pub card_width: f32,
    pub card_gap: f32,
    pub card_padding: f32,
    pub card_is_dense: bool,
    pub card_shadow_level: u8,
    pub header_gap: f32,
    pub copy_gap: f32,
    pub title_font_size: f32,
    pub title_line_height: f32,
    pub description_font_size: f32,
    pub description_line_height: f32,
    pub action_row_gap: f32,
    pub action_min_height: f32,
    pub action_padding_inline: f32,
    pub action_padding_block: f32,
    pub action_font_size: f32,
    pub action_line_height: f32,
    pub error_padding: f32,
    pub error_font_size: f32,
    pub error_line_height: f32,
}

impl ToastLayoutMetrics {
    pub fn action_content_min_height(self, border_width: f32) -> f32 {
        (self.action_min_height - border_width.max(0.0) * 2.0).max(0.0)
    }
}

impl ToastAction {
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

impl ToastModel {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            density: ToastDensity::Standard,
            position: ToastPosition::BottomRight,
            tone: ToastTone::Default,
            title: title.into(),
            description: description.into(),
            action: None,
            duration_ms: 4_000,
            error: None,
            default_open: true,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: ToastDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    pub const fn with_tone(mut self, tone: ToastTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_action(mut self, action: ToastAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn without_action(mut self) -> Self {
        self.action = None;
        self
    }

    pub const fn with_duration_ms(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms;
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

    pub const fn default_open(mut self, default_open: bool) -> Self {
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

    pub const fn state(&self) -> ToastState {
        ToastState::new(self.default_open)
    }
}

impl ToastState {
    pub const fn new(open: bool) -> Self {
        Self {
            open,
            focused: false,
            paused: false,
            actioned: false,
        }
    }

    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub const fn is_paused(&self) -> bool {
        self.paused
    }

    pub const fn is_actioned(&self) -> bool {
        self.actioned
    }

    pub fn apply(&mut self, intent: ToastIntent) -> ToastChange {
        match intent {
            ToastIntent::Open => self.open(),
            ToastIntent::Close => self.close(),
            ToastIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            ToastIntent::Focus => self.focus(),
            ToastIntent::Blur => self.blur(),
            ToastIntent::Pause => self.pause(),
            ToastIntent::Resume => self.resume(),
            ToastIntent::Activate(value) => self.activate(value),
            ToastIntent::ResetOpen(open) => self.reset_open(open),
            ToastIntent::Clear => self.clear(),
        }
    }

    fn open(&mut self) -> ToastChange {
        if self.open {
            ToastChange::Unchanged
        } else {
            self.open = true;
            ToastChange::Opened
        }
    }

    fn close(&mut self) -> ToastChange {
        if self.open {
            self.open = false;
            self.focused = false;
            self.paused = false;
            ToastChange::Closed
        } else {
            ToastChange::Unchanged
        }
    }

    fn focus(&mut self) -> ToastChange {
        if self.focused {
            ToastChange::Unchanged
        } else {
            self.focused = true;
            ToastChange::Focused
        }
    }

    fn blur(&mut self) -> ToastChange {
        if self.focused {
            self.focused = false;
            ToastChange::Blurred
        } else {
            ToastChange::Unchanged
        }
    }

    fn pause(&mut self) -> ToastChange {
        if self.paused {
            ToastChange::Unchanged
        } else {
            self.paused = true;
            ToastChange::Paused
        }
    }

    fn resume(&mut self) -> ToastChange {
        if self.paused {
            self.paused = false;
            ToastChange::Resumed
        } else {
            ToastChange::Unchanged
        }
    }

    fn activate(&mut self, value: String) -> ToastChange {
        if self.actioned {
            ToastChange::Unchanged
        } else {
            self.open = true;
            self.focused = true;
            self.actioned = true;
            ToastChange::Activated(value)
        }
    }

    fn reset_open(&mut self, open: bool) -> ToastChange {
        if self.open == open {
            ToastChange::Unchanged
        } else {
            self.open = open;
            if !open {
                self.focused = false;
                self.paused = false;
            }
            ToastChange::Reset(open)
        }
    }

    fn clear(&mut self) -> ToastChange {
        if !self.open && !self.focused && !self.paused && !self.actioned {
            ToastChange::Unchanged
        } else {
            self.open = false;
            self.focused = false;
            self.paused = false;
            self.actioned = false;
            ToastChange::Cleared
        }
    }
}

impl Default for ToastState {
    fn default() -> Self {
        Self::new(true)
    }
}

pub fn validate_toast_model(model: &ToastModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn toast_layout_metrics(
    model: &ToastModel,
    state: &ToastState,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> ToastLayoutMetrics {
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let dense = model.density == ToastDensity::Dense;
    let dense_provider = toast_uses_dense_provider_metrics(dense, invalid, model.disabled);
    let dense_card = toast_uses_dense_card_metrics(dense, state.is_focused(), invalid, blocked);
    let max_width = available_width.clamp(1.0, scale::container::CONTROL);
    let card_padding = if dense_card {
        scale::space::xs(inline_size)
    } else {
        scale::space::s(inline_size)
    };
    let card_gap = if dense_card {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let title_font_size = scale::font_size::f0(inline_size);
    let description_font_size = scale::font_size::f0(inline_size);
    let action_font_size = scale::font_size::f00(inline_size);
    let content_width = scale::estimate_inline_text_width(&model.title, title_font_size, 0.0)
        .max(scale::estimate_inline_text_width(
            &model.description,
            description_font_size,
            0.0,
        ))
        .max(model.action.as_ref().map_or(0.0, |action| {
            scale::estimate_inline_text_width(&action.label, action_font_size, 0.0)
                + scale::space::xs(inline_size) * 2.0
                + border_width.max(0.0) * 2.0
        }));
    let card_width =
        (content_width + card_padding * 2.0 + border_width.max(0.0) * 2.0).clamp(1.0, max_width);

    ToastLayoutMetrics {
        max_width,
        provider_gap: if dense_provider {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        provider_padding: if invalid {
            scale::space::s(inline_size)
        } else {
            0.0
        },
        meta_font_size: scale::font_size::f00(inline_size),
        meta_line_height: scale::line_height::LH0,
        meta_letter_spacing: scale::letter_spacing::LABEL,
        viewport_gap: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        card_width,
        card_gap,
        card_padding,
        card_is_dense: dense_card,
        card_shadow_level: if blocked {
            0
        } else if dense_card {
            1
        } else {
            2
        },
        header_gap: scale::space::xs(inline_size),
        copy_gap: scale::space::xs3(inline_size),
        title_font_size,
        title_line_height: scale::line_height::LH0,
        description_font_size,
        description_line_height: scale::line_height::LH0,
        action_row_gap: scale::space::xs2(inline_size),
        action_min_height: scale::space::s(inline_size),
        action_padding_inline: scale::space::xs(inline_size),
        action_padding_block: scale::space::xs3(inline_size),
        action_font_size,
        action_line_height: scale::line_height::LH0,
        error_padding: scale::space::s(inline_size),
        error_font_size: scale::font_size::f0(inline_size),
        error_line_height: scale::line_height::LH0,
    }
}

pub const fn toast_uses_dense_provider_metrics(dense: bool, invalid: bool, disabled: bool) -> bool {
    dense && !invalid && !disabled
}

pub const fn toast_uses_dense_card_metrics(
    dense: bool,
    focused: bool,
    invalid: bool,
    disabled: bool,
) -> bool {
    dense && !focused && !invalid && !disabled
}

pub fn toast_render_nodes(model: &ToastModel, state: &ToastState) -> Vec<ToastRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let action = model.action.as_ref();
    let action_visible = state.is_open() && action.is_some();
    let action_disabled = action.is_none_or(|action| action.disabled) || blocked;

    vec![
        toast_node(
            ToastNodeDraft {
                part: ToastPart::Provider,
                value: "toast-provider",
                label: "Toast provider",
                detail: model
                    .error
                    .as_deref()
                    .unwrap_or("Provider for one transient notification."),
                tone: ToastTone::Default,
                visible: true,
                actionable: false,
                disabled: model.disabled,
            },
            model,
            state,
            invalid,
        ),
        toast_node(
            ToastNodeDraft {
                part: ToastPart::Viewport,
                value: model.position.label(),
                label: "Toast viewport",
                detail: "Viewport for transient notification delivery.",
                tone: ToastTone::Default,
                visible: true,
                actionable: !blocked,
                disabled: blocked,
            },
            model,
            state,
            invalid,
        ),
        toast_node(
            ToastNodeDraft {
                part: ToastPart::Toast,
                value: "toast",
                label: &model.title,
                detail: &model.description,
                tone: model.tone,
                visible: state.is_open(),
                actionable: !blocked,
                disabled: blocked,
            },
            model,
            state,
            invalid,
        ),
        toast_node(
            ToastNodeDraft {
                part: ToastPart::Title,
                value: "toast-title",
                label: &model.title,
                detail: "Toast title",
                tone: model.tone,
                visible: state.is_open(),
                actionable: false,
                disabled: blocked,
            },
            model,
            state,
            invalid,
        ),
        toast_node(
            ToastNodeDraft {
                part: ToastPart::Description,
                value: "toast-description",
                label: "Toast description",
                detail: &model.description,
                tone: model.tone,
                visible: state.is_open(),
                actionable: false,
                disabled: blocked,
            },
            model,
            state,
            invalid,
        ),
        toast_node(
            ToastNodeDraft {
                part: ToastPart::Action,
                value: action.map_or("toast-action", |action| action.value.as_str()),
                label: action.map_or("Toast action", |action| action.label.as_str()),
                detail: action.map_or("No toast action configured.", |_| "Toast action."),
                tone: model.tone,
                visible: action_visible,
                actionable: action_visible && !action_disabled,
                disabled: action_disabled,
            },
            model,
            state,
            invalid,
        ),
    ]
}

pub fn default_toast_model() -> ToastModel {
    ToastModel::new(
        "Draft saved",
        "The component contract was saved locally and is ready for the next gate.",
    )
    .with_tone(ToastTone::Success)
    .with_action(ToastAction::new("Undo", "undo-save"))
}

pub fn toast_dom_id(prefix: &str, value: &str) -> String {
    ui_dom_id(prefix, value)
}

fn toast_node(
    draft: ToastNodeDraft<'_>,
    model: &ToastModel,
    state: &ToastState,
    invalid: bool,
) -> ToastRenderNode {
    ToastRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        position: model.position,
        tone: draft.tone,
        open: state.is_open(),
        focused: state.is_focused(),
        paused: state.is_paused(),
        actioned: state.is_actioned(),
        visible: draft.visible,
        actionable: draft.actionable,
        invalid,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

struct ToastNodeDraft<'a> {
    part: ToastPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    tone: ToastTone,
    visible: bool,
    actionable: bool,
    disabled: bool,
}

fn validate_optional_toast_action(action: &Option<ToastAction>, _: &()) -> garde::Result {
    if let Some(action) = action {
        action
            .validate()
            .map_err(|report| garde::Error::new(format!("invalid toast action: {report}")))?;
    }
    Ok(())
}

fn validate_optional_toast_error(error: &Option<String>, _: &()) -> garde::Result {
    if let Some(error) = error
        && (error.is_empty() || error.len() > 240)
    {
        return Err(garde::Error::new(
            "toast error must be between 1 and 240 characters when present",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_toast_model(&default_toast_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = default_toast_model().with_title("");
        assert!(validate_toast_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_description() {
        let model = default_toast_model().with_description("");
        assert!(validate_toast_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_action_contract() {
        let model = default_toast_model().with_action(ToastAction::new("", "retry"));
        assert!(validate_toast_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_toast_model().with_error("");
        assert!(validate_toast_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_duration() {
        let model = default_toast_model().with_duration_ms(999);
        assert!(validate_toast_model(&model).is_err());
    }

    #[test]
    fn state_tracks_open_focus_pause_action_and_clear() {
        let mut state = ToastState::new(false);
        assert!(!state.is_open());
        assert_eq!(state.apply(ToastIntent::Open), ToastChange::Opened);
        assert!(state.is_open());
        assert_eq!(state.apply(ToastIntent::Focus), ToastChange::Focused);
        assert!(state.is_focused());
        assert_eq!(state.apply(ToastIntent::Pause), ToastChange::Paused);
        assert!(state.is_paused());
        assert_eq!(
            state.apply(ToastIntent::Activate("undo".to_owned())),
            ToastChange::Activated("undo".to_owned())
        );
        assert!(state.is_actioned());
        assert_eq!(state.apply(ToastIntent::Close), ToastChange::Closed);
        assert!(!state.is_open());
        assert!(!state.is_focused());
        assert!(!state.is_paused());
        assert_eq!(state.apply(ToastIntent::Clear), ToastChange::Cleared);
        assert!(!state.is_actioned());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_toast_model();
        let state = model.state();
        let nodes = toast_render_nodes(&model, &state);
        assert_eq!(
            nodes.iter().map(|node| node.part).collect::<Vec<_>>(),
            ToastPart::ALL
        );
    }

    #[test]
    fn closed_state_hides_toast_content() {
        let model = default_toast_model().default_open(false);
        let state = model.state();
        let nodes = toast_render_nodes(&model, &state);
        assert!(
            nodes
                .iter()
                .filter(|node| {
                    matches!(
                        node.part,
                        ToastPart::Toast
                            | ToastPart::Title
                            | ToastPart::Description
                            | ToastPart::Action
                    )
                })
                .all(|node| !node.visible)
        );
    }

    #[test]
    fn missing_action_keeps_hidden_disabled_action_node() {
        let model = default_toast_model().without_action();
        let state = model.state();
        let nodes = toast_render_nodes(&model, &state);
        let action = nodes
            .iter()
            .find(|node| node.part == ToastPart::Action)
            .expect("toast action node");
        assert!(!action.visible);
        assert!(action.disabled);
        assert!(!action.actionable);
    }

    #[test]
    fn loading_disables_action_node() {
        let model = default_toast_model().loading();
        let state = model.state();
        let nodes = toast_render_nodes(&model, &state);
        let action = nodes
            .iter()
            .find(|node| node.part == ToastPart::Action)
            .expect("toast action node");
        assert!(action.visible);
        assert!(action.disabled);
        assert!(!action.actionable);
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_toast_model().with_error("Toast failed to render.");
        let state = model.state();
        assert!(
            toast_render_nodes(&model, &state)
                .iter()
                .all(|node| node.invalid)
        );
    }

    #[test]
    fn dense_metrics_follow_dom_state_precedence() {
        assert!(toast_uses_dense_provider_metrics(true, false, false));
        assert!(!toast_uses_dense_provider_metrics(true, true, false));
        assert!(!toast_uses_dense_provider_metrics(true, false, true));
        assert!(toast_uses_dense_card_metrics(true, false, false, false));
        assert!(!toast_uses_dense_card_metrics(true, true, false, false));
        assert!(!toast_uses_dense_card_metrics(true, false, true, false));
        assert!(!toast_uses_dense_card_metrics(true, false, false, true));
    }

    #[test]
    fn dense_intrinsic_card_width_tracks_fixture_copy() {
        let model = default_toast_model()
            .with_density(ToastDensity::Dense)
            .with_title("Gate running")
            .with_description("Dense toasts keep the same local open and action state.");
        let metrics = toast_layout_metrics(&model, &model.state(), 448.0, 952.0, 1.0);

        assert!(metrics.card_is_dense);
        assert!((420.0..=426.0).contains(&metrics.card_width));
        assert_eq!(metrics.card_shadow_level, 1);
    }

    #[test]
    fn focused_dense_card_restores_standard_metrics() {
        let model = default_toast_model().with_density(ToastDensity::Dense);
        let mut state = model.state();
        let resting = toast_layout_metrics(&model, &state, 448.0, 952.0, 1.0);
        state.apply(ToastIntent::Focus);
        let focused = toast_layout_metrics(&model, &state, 448.0, 952.0, 1.0);

        assert!(resting.card_is_dense);
        assert!(!focused.card_is_dense);
        assert!(focused.card_padding > resting.card_padding);
        assert_eq!(focused.card_shadow_level, 2);
    }

    #[test]
    fn blocked_card_uses_standard_metrics_without_shadow() {
        let model = default_toast_model()
            .with_density(ToastDensity::Dense)
            .loading();
        let metrics = toast_layout_metrics(&model, &model.state(), 448.0, 952.0, 1.0);

        assert!(!metrics.card_is_dense);
        assert_eq!(metrics.card_shadow_level, 0);
    }

    #[test]
    fn dom_ids_are_stable_and_ascii() {
        assert_eq!(
            toast_dom_id("toast-title", "Draft saved!"),
            "toast-title-draft-saved"
        );
    }
}
