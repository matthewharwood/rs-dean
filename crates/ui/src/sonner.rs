use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SonnerDensity {
    Standard,
    Dense,
}

impl SonnerDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SonnerPosition {
    TopRight,
    BottomRight,
    BottomCenter,
}

impl SonnerPosition {
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
pub enum SonnerTone {
    Default,
    Info,
    Success,
    Warning,
    Destructive,
}

impl SonnerTone {
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
pub enum SonnerPart {
    Provider,
    Viewport,
    Toast,
    Action,
    Dismiss,
}

impl SonnerPart {
    pub const ALL: &'static [Self] = &[
        Self::Provider,
        Self::Viewport,
        Self::Toast,
        Self::Action,
        Self::Dismiss,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Provider => "SonnerProvider",
            Self::Viewport => "SonnerViewport",
            Self::Toast => "SonnerToast",
            Self::Action => "SonnerAction",
            Self::Dismiss => "SonnerDismiss",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SonnerAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SonnerToast {
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 480))]
    pub description: String,
    #[garde(skip)]
    pub tone: SonnerTone,
    #[garde(range(min = 1_000, max = 60_000))]
    pub duration_ms: u32,
    #[garde(custom(validate_optional_sonner_action))]
    pub action: Option<SonnerAction>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SonnerModel {
    #[garde(skip)]
    pub density: SonnerDensity,
    #[garde(skip)]
    pub position: SonnerPosition,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 6), dive, custom(sonner_toast_values_are_unique))]
    pub toasts: Vec<SonnerToast>,
    #[garde(custom(validate_optional_sonner_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SonnerState {
    active_value: Option<String>,
    dismissed_values: Vec<String>,
    actioned_values: Vec<String>,
    paused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SonnerIntent {
    Focus(String),
    Blur,
    Activate(String),
    Dismiss(String),
    ClearDismissed,
    Pause,
    Resume,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SonnerChange {
    Focused(String),
    Blurred,
    Activated(String),
    Dismissed(String),
    DismissedCleared,
    Paused,
    Resumed,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SonnerRenderNode {
    pub part: SonnerPart,
    pub index: usize,
    pub toast_value: String,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: SonnerDensity,
    pub position: SonnerPosition,
    pub tone: SonnerTone,
    pub active: bool,
    pub actioned: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SonnerLayoutMetrics {
    pub max_width: f32,
    pub provider_gap: f32,
    pub provider_padding: f32,
    pub meta_font_size: f32,
    pub meta_line_height: f32,
    pub meta_letter_spacing: f32,
    pub viewport_gap: f32,
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
    pub dismiss_size: f32,
    pub error_padding: f32,
    pub error_font_size: f32,
    pub error_line_height: f32,
    standard_toast_gap: f32,
    standard_toast_padding: f32,
    dense_toast_gap: f32,
    dense_toast_padding: f32,
    dense: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SonnerToastLayoutMetrics {
    pub gap: f32,
    pub padding: f32,
    pub dense: bool,
    pub shadow_level: u8,
}

impl SonnerLayoutMetrics {
    pub const fn toast(
        self,
        active: bool,
        invalid: bool,
        disabled: bool,
    ) -> SonnerToastLayoutMetrics {
        let dense = sonner_uses_dense_toast_metrics(self.dense, active, invalid, disabled);
        SonnerToastLayoutMetrics {
            gap: if dense {
                self.dense_toast_gap
            } else {
                self.standard_toast_gap
            },
            padding: if dense {
                self.dense_toast_padding
            } else {
                self.standard_toast_padding
            },
            dense,
            shadow_level: if disabled {
                0
            } else if invalid || dense {
                1
            } else {
                2
            },
        }
    }

    pub fn action_content_min_height(self, border_width: f32) -> f32 {
        (self.action_min_height - border_width.max(0.0) * 2.0).max(0.0)
    }

    pub fn dismiss_content_size(self, border_width: f32) -> f32 {
        (self.dismiss_size - border_width.max(0.0) * 2.0).max(0.0)
    }
}

impl SonnerAction {
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

impl SonnerToast {
    pub fn new(
        value: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            description: description.into(),
            tone: SonnerTone::Default,
            duration_ms: 4_000,
            action: None,
            disabled: false,
        }
    }

    pub const fn with_tone(mut self, tone: SonnerTone) -> Self {
        self.tone = tone;
        self
    }

    pub const fn with_duration_ms(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_action(mut self, action: SonnerAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn without_action(mut self) -> Self {
        self.action = None;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl SonnerModel {
    pub fn new(toasts: Vec<SonnerToast>) -> Self {
        Self {
            density: SonnerDensity::Standard,
            position: SonnerPosition::BottomRight,
            label: "Notifications".to_owned(),
            toasts,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SonnerDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_position(mut self, position: SonnerPosition) -> Self {
        self.position = position;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_toasts(mut self, toasts: Vec<SonnerToast>) -> Self {
        self.toasts = toasts;
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

    pub fn state(&self) -> SonnerState {
        SonnerState::new()
    }
}

impl SonnerState {
    pub fn new() -> Self {
        Self {
            active_value: None,
            dismissed_values: Vec::new(),
            actioned_values: Vec::new(),
            paused: false,
        }
    }

    pub fn active_value(&self) -> Option<&str> {
        self.active_value.as_deref()
    }

    pub fn dismissed_values(&self) -> &[String] {
        self.dismissed_values.as_slice()
    }

    pub fn actioned_values(&self) -> &[String] {
        self.actioned_values.as_slice()
    }

    pub const fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_active(&self, value: &str) -> bool {
        self.active_value.as_deref() == Some(value)
    }

    pub fn is_dismissed(&self, value: &str) -> bool {
        self.dismissed_values
            .iter()
            .any(|dismissed| dismissed == value)
    }

    pub fn is_actioned(&self, value: &str) -> bool {
        self.actioned_values
            .iter()
            .any(|actioned| actioned == value)
    }

    pub fn apply(&mut self, intent: SonnerIntent) -> SonnerChange {
        match intent {
            SonnerIntent::Focus(value) => self.focus(value),
            SonnerIntent::Blur => self.blur(),
            SonnerIntent::Activate(value) => self.activate(value),
            SonnerIntent::Dismiss(value) => self.dismiss(value),
            SonnerIntent::ClearDismissed => self.clear_dismissed(),
            SonnerIntent::Pause => self.pause(),
            SonnerIntent::Resume => self.resume(),
            SonnerIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, value: String) -> SonnerChange {
        if self.active_value.as_ref() == Some(&value) {
            SonnerChange::Unchanged
        } else {
            self.active_value = Some(value.clone());
            SonnerChange::Focused(value)
        }
    }

    fn blur(&mut self) -> SonnerChange {
        if self.active_value.is_some() {
            self.active_value = None;
            SonnerChange::Blurred
        } else {
            SonnerChange::Unchanged
        }
    }

    fn activate(&mut self, value: String) -> SonnerChange {
        if self.actioned_values.contains(&value) {
            SonnerChange::Unchanged
        } else {
            self.actioned_values.push(value.clone());
            self.active_value = Some(value.clone());
            SonnerChange::Activated(value)
        }
    }

    fn dismiss(&mut self, value: String) -> SonnerChange {
        if self.dismissed_values.contains(&value) {
            SonnerChange::Unchanged
        } else {
            self.dismissed_values.push(value.clone());
            if self.active_value.as_ref() == Some(&value) {
                self.active_value = None;
            }
            SonnerChange::Dismissed(value)
        }
    }

    fn clear_dismissed(&mut self) -> SonnerChange {
        if self.dismissed_values.is_empty() {
            SonnerChange::Unchanged
        } else {
            self.dismissed_values.clear();
            SonnerChange::DismissedCleared
        }
    }

    fn pause(&mut self) -> SonnerChange {
        if self.paused {
            SonnerChange::Unchanged
        } else {
            self.paused = true;
            SonnerChange::Paused
        }
    }

    fn resume(&mut self) -> SonnerChange {
        if self.paused {
            self.paused = false;
            SonnerChange::Resumed
        } else {
            SonnerChange::Unchanged
        }
    }

    fn clear(&mut self) -> SonnerChange {
        if self.active_value.is_none()
            && self.dismissed_values.is_empty()
            && self.actioned_values.is_empty()
            && !self.paused
        {
            SonnerChange::Unchanged
        } else {
            self.active_value = None;
            self.dismissed_values.clear();
            self.actioned_values.clear();
            self.paused = false;
            SonnerChange::Cleared
        }
    }
}

impl Default for SonnerState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn validate_sonner_model(model: &SonnerModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn sonner_layout_metrics(
    model: &SonnerModel,
    available_width: f32,
    inline_size: f32,
) -> SonnerLayoutMetrics {
    let dense = model.density == SonnerDensity::Dense;
    let invalid = model.error.is_some();
    let dense_provider = sonner_uses_dense_provider_metrics(dense, invalid, model.disabled);

    SonnerLayoutMetrics {
        max_width: available_width.clamp(1.0, scale::container::CONTROL),
        provider_gap: if dense_provider {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        provider_padding: if invalid {
            scale::space::s(inline_size)
        } else {
            0.0
        },
        meta_font_size: scale::font_size::f00(inline_size),
        meta_line_height: scale::line_height::LH0,
        meta_letter_spacing: scale::letter_spacing::LABEL,
        viewport_gap: if dense_provider {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        header_gap: scale::space::xs(inline_size),
        copy_gap: scale::space::xs3(inline_size),
        title_font_size: scale::font_size::f0(inline_size),
        title_line_height: scale::line_height::LH0,
        description_font_size: scale::font_size::f0(inline_size),
        description_line_height: scale::line_height::LH0,
        action_row_gap: scale::space::xs2(inline_size),
        action_min_height: scale::space::s(inline_size),
        action_padding_inline: scale::space::xs(inline_size),
        action_padding_block: scale::space::xs3(inline_size),
        action_font_size: scale::font_size::f00(inline_size),
        dismiss_size: scale::space::m(inline_size),
        error_padding: scale::space::xs(inline_size),
        error_font_size: scale::font_size::f0(inline_size),
        error_line_height: scale::line_height::LH0,
        standard_toast_gap: scale::space::xs(inline_size),
        standard_toast_padding: scale::space::s(inline_size),
        dense_toast_gap: scale::space::xs2(inline_size),
        dense_toast_padding: scale::space::xs(inline_size),
        dense,
    }
}

pub const fn sonner_uses_dense_provider_metrics(
    dense: bool,
    invalid: bool,
    disabled: bool,
) -> bool {
    dense && !invalid && !disabled
}

pub const fn sonner_uses_dense_toast_metrics(
    dense: bool,
    active: bool,
    invalid: bool,
    disabled: bool,
) -> bool {
    dense && !active && !invalid && !disabled
}

pub fn sonner_render_nodes(model: &SonnerModel, state: &SonnerState) -> Vec<SonnerRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let toast_count = model.toasts.len();
    let mut nodes = Vec::with_capacity(toast_count.saturating_mul(3).saturating_add(2));

    nodes.push(SonnerRenderNode {
        part: SonnerPart::Provider,
        index: 0,
        toast_value: String::new(),
        value: model.label.clone(),
        label: model.label.clone(),
        detail: model
            .error
            .clone()
            .unwrap_or_else(|| format!("{toast_count} toast notifications")),
        density: model.density,
        position: model.position,
        tone: SonnerTone::Default,
        active: state.active_value().is_some(),
        actioned: false,
        visible: true,
        actionable: false,
        invalid,
        loading: model.loading,
        disabled: model.disabled,
    });
    nodes.push(SonnerRenderNode {
        part: SonnerPart::Viewport,
        index: 0,
        toast_value: String::new(),
        value: model.position.label().to_owned(),
        label: format!("{} viewport", model.label),
        detail: "Renderer-local toast viewport over shared render nodes.".to_owned(),
        density: model.density,
        position: model.position,
        tone: SonnerTone::Default,
        active: state.is_paused(),
        actioned: false,
        visible: true,
        actionable: !blocked,
        invalid,
        loading: model.loading,
        disabled: blocked,
    });

    for (index, toast) in model.toasts.iter().enumerate() {
        let visible = !state.is_dismissed(&toast.value);
        let toast_disabled = blocked || toast.disabled;
        let active = state.is_active(&toast.value);
        let actioned = state.is_actioned(&toast.value);
        nodes.push(SonnerRenderNode {
            part: SonnerPart::Toast,
            index,
            toast_value: toast.value.clone(),
            value: toast.title.clone(),
            label: toast.title.clone(),
            detail: toast.description.clone(),
            density: model.density,
            position: model.position,
            tone: toast.tone,
            active,
            actioned,
            visible,
            actionable: !toast_disabled,
            invalid,
            loading: model.loading,
            disabled: toast_disabled,
        });
        if let Some(action) = &toast.action {
            nodes.push(SonnerRenderNode {
                part: SonnerPart::Action,
                index,
                toast_value: toast.value.clone(),
                value: action.value.clone(),
                label: action.label.clone(),
                detail: format!("Action for {}", toast.title),
                density: model.density,
                position: model.position,
                tone: toast.tone,
                active,
                actioned,
                visible,
                actionable: !toast_disabled && !action.disabled,
                invalid,
                loading: model.loading,
                disabled: toast_disabled || action.disabled,
            });
        }
        nodes.push(SonnerRenderNode {
            part: SonnerPart::Dismiss,
            index,
            toast_value: toast.value.clone(),
            value: format!("dismiss-{}", toast.value),
            label: "Dismiss".to_owned(),
            detail: format!("Dismiss {}", toast.title),
            density: model.density,
            position: model.position,
            tone: toast.tone,
            active,
            actioned: false,
            visible,
            actionable: !toast_disabled,
            invalid,
            loading: model.loading,
            disabled: toast_disabled,
        });
    }

    nodes
}

pub fn default_sonner_model() -> SonnerModel {
    SonnerModel::new(vec![
        SonnerToast::new(
            "saved",
            "Project saved",
            "Your changes are staged locally and ready for the next gate run.",
        )
        .with_tone(SonnerTone::Success)
        .with_action(SonnerAction::new("Undo", "undo-save")),
        SonnerToast::new(
            "queue",
            "Gate queued",
            "The full sweep pass is running before the branch moves forward.",
        )
        .with_tone(SonnerTone::Info),
    ])
}

fn sonner_toast_values_are_unique(toasts: &[SonnerToast], _: &()) -> garde::Result {
    let mut values: Vec<&str> = Vec::with_capacity(toasts.len());
    for toast in toasts {
        if values.iter().any(|value| *value == toast.value) {
            return Err(garde::Error::new("sonner toast values must be unique"));
        }
        values.push(&toast.value);
    }
    Ok(())
}

fn validate_optional_sonner_action(action: &Option<SonnerAction>, _context: &()) -> garde::Result {
    if let Some(action) = action {
        action.validate().map_err(|report| {
            garde::Error::new(format!("invalid sonner action contract: {report}"))
        })?;
    }
    Ok(())
}

fn validate_optional_sonner_error(error: &Option<String>, _: &()) -> garde::Result {
    if let Some(error) = error
        && (error.is_empty() || error.len() > 240)
    {
        return Err(garde::Error::new(
            "sonner error must be between 1 and 240 characters when present",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_sonner_model(&default_sonner_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_toasts() {
        let model = SonnerModel::new(Vec::new());
        assert!(validate_sonner_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_toast_values() {
        let model = SonnerModel::new(vec![
            SonnerToast::new("saved", "Saved", "Saved once."),
            SonnerToast::new("saved", "Saved again", "Duplicate value."),
        ]);
        assert!(validate_sonner_model(&model).is_err());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_sonner_model();
        let state = model.state();
        let nodes = sonner_render_nodes(&model, &state);

        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(SonnerPart::Provider)
        );
        for part in SonnerPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn state_tracks_focus_action_pause_and_dismissal() {
        let mut state = SonnerState::new();

        assert_eq!(
            state.apply(SonnerIntent::Focus("saved".to_owned())),
            SonnerChange::Focused("saved".to_owned())
        );
        assert!(state.is_active("saved"));
        assert_eq!(
            state.apply(SonnerIntent::Activate("saved".to_owned())),
            SonnerChange::Activated("saved".to_owned())
        );
        assert!(state.is_actioned("saved"));
        assert_eq!(state.apply(SonnerIntent::Pause), SonnerChange::Paused);
        assert!(state.is_paused());
        assert_eq!(
            state.apply(SonnerIntent::Dismiss("saved".to_owned())),
            SonnerChange::Dismissed("saved".to_owned())
        );
        assert!(!state.is_active("saved"));
        assert!(state.is_dismissed("saved"));
        assert_eq!(state.apply(SonnerIntent::Clear), SonnerChange::Cleared);
        assert!(!state.is_dismissed("saved"));
        assert!(!state.is_actioned("saved"));
        assert!(!state.is_paused());
    }

    #[test]
    fn layout_metrics_follow_density_and_state_precedence() {
        let standard = default_sonner_model();
        let dense = standard.clone().with_density(SonnerDensity::Dense);
        let disabled = dense.clone().disabled();
        let standard_metrics = sonner_layout_metrics(&standard, 420.0, 1_000.0);
        let dense_metrics = sonner_layout_metrics(&dense, 420.0, 1_000.0);
        let disabled_metrics = sonner_layout_metrics(&disabled, 420.0, 1_000.0);

        assert!(dense_metrics.provider_gap < standard_metrics.provider_gap);
        assert_eq!(disabled_metrics.provider_gap, standard_metrics.provider_gap);
        assert!(dense_metrics.toast(false, false, false).dense);
        assert!(!dense_metrics.toast(true, false, false).dense);
        assert!(!dense_metrics.toast(false, true, false).dense);
        assert!(!dense_metrics.toast(false, false, true).dense);
        assert_eq!(dense_metrics.toast(true, false, false).shadow_level, 2);
        assert_eq!(dense_metrics.toast(false, true, false).shadow_level, 1);
        assert_eq!(dense_metrics.toast(false, false, true).shadow_level, 0);
    }

    #[test]
    fn border_box_helpers_preserve_control_outer_sizes() {
        let metrics = sonner_layout_metrics(&default_sonner_model(), 420.0, 1_000.0);

        assert_eq!(
            metrics.action_content_min_height(2.0) + 4.0,
            metrics.action_min_height
        );
        assert_eq!(
            metrics.dismiss_content_size(2.0) + 4.0,
            metrics.dismiss_size
        );
        assert_eq!(
            metrics.action_content_min_height(-2.0),
            metrics.action_min_height
        );
    }
}
