use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpinnerDensity {
    Standard,
    Dense,
}

impl SpinnerDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpinnerSize {
    Small,
    Medium,
    Large,
}

impl SpinnerSize {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpinnerTone {
    Default,
    Brand,
    Info,
    Success,
    Warning,
    Destructive,
}

impl SpinnerTone {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Brand => "brand",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Destructive => "destructive",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpinnerPart {
    Root,
    Track,
    Indicator,
    Label,
}

impl SpinnerPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Track, Self::Indicator, Self::Label];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Spinner",
            Self::Track => "SpinnerTrack",
            Self::Indicator => "SpinnerIndicator",
            Self::Label => "SpinnerLabel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SpinnerModel {
    #[garde(skip)]
    pub density: SpinnerDensity,
    #[garde(skip)]
    pub size: SpinnerSize,
    #[garde(skip)]
    pub tone: SpinnerTone,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(range(min = 400, max = 4_000))]
    pub speed_ms: u16,
    #[garde(custom(validate_optional_spinner_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub show_label: bool,
    #[garde(skip)]
    pub animated: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpinnerState {
    active_part: Option<SpinnerPart>,
    paused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpinnerIntent {
    Focus(SpinnerPart),
    Blur,
    Pause,
    Resume,
    ToggleMotion,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpinnerChange {
    Focused(SpinnerPart),
    Blurred,
    Paused,
    Resumed,
    MotionToggled(bool),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpinnerRenderNode {
    pub part: SpinnerPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: SpinnerDensity,
    pub size: SpinnerSize,
    pub tone: SpinnerTone,
    pub speed_ms: u16,
    pub active: bool,
    pub paused: bool,
    pub animated: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpinnerLayoutMetrics {
    pub width: f32,
    pub height: f32,
    pub padding_inline: f32,
    pub padding_block: f32,
    pub gap: f32,
    pub edge: f32,
    pub border_width: f32,
    pub label_font_size: f32,
    pub label_line_height: f32,
    pub error_padding: f32,
    pub error_font_size: f32,
    pub error_line_height: f32,
    pub shadow_level: u8,
}

impl SpinnerModel {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            density: SpinnerDensity::Standard,
            size: SpinnerSize::Medium,
            tone: SpinnerTone::Brand,
            label: label.into(),
            detail: "Loading shared UI state.".to_owned(),
            speed_ms: 900,
            error: None,
            show_label: true,
            animated: true,
            loading: true,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SpinnerDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub const fn with_tone(mut self, tone: SpinnerTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub const fn with_speed_ms(mut self, speed_ms: u16) -> Self {
        self.speed_ms = speed_ms;
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

    pub const fn hide_label(mut self) -> Self {
        self.show_label = false;
        self
    }

    pub const fn show_label(mut self) -> Self {
        self.show_label = true;
        self
    }

    pub const fn static_indicator(mut self) -> Self {
        self.animated = false;
        self
    }

    pub const fn animated(mut self) -> Self {
        self.animated = true;
        self
    }

    pub const fn ready(mut self) -> Self {
        self.loading = false;
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

    pub const fn state(&self) -> SpinnerState {
        SpinnerState::new()
    }
}

impl SpinnerState {
    pub const fn new() -> Self {
        Self {
            active_part: None,
            paused: false,
        }
    }

    pub const fn is_active(&self, part: SpinnerPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub const fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn apply(&mut self, intent: SpinnerIntent) -> SpinnerChange {
        match intent {
            SpinnerIntent::Focus(part) => self.focus(part),
            SpinnerIntent::Blur => self.blur(),
            SpinnerIntent::Pause => self.pause(),
            SpinnerIntent::Resume => self.resume(),
            SpinnerIntent::ToggleMotion => self.toggle_motion(),
            SpinnerIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, part: SpinnerPart) -> SpinnerChange {
        if self.is_active(part) {
            SpinnerChange::Unchanged
        } else {
            self.active_part = Some(part);
            SpinnerChange::Focused(part)
        }
    }

    fn blur(&mut self) -> SpinnerChange {
        if self.active_part.is_some() {
            self.active_part = None;
            SpinnerChange::Blurred
        } else {
            SpinnerChange::Unchanged
        }
    }

    fn pause(&mut self) -> SpinnerChange {
        if self.paused {
            SpinnerChange::Unchanged
        } else {
            self.paused = true;
            SpinnerChange::Paused
        }
    }

    fn resume(&mut self) -> SpinnerChange {
        if self.paused {
            self.paused = false;
            SpinnerChange::Resumed
        } else {
            SpinnerChange::Unchanged
        }
    }

    fn toggle_motion(&mut self) -> SpinnerChange {
        self.paused = !self.paused;
        SpinnerChange::MotionToggled(self.paused)
    }

    fn clear(&mut self) -> SpinnerChange {
        if self.active_part.is_none() && !self.paused {
            SpinnerChange::Unchanged
        } else {
            self.active_part = None;
            self.paused = false;
            SpinnerChange::Cleared
        }
    }
}

impl Default for SpinnerState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn validate_spinner_model(model: &SpinnerModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn spinner_render_nodes(model: &SpinnerModel, state: &SpinnerState) -> Vec<SpinnerRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.disabled;
    let paused = state.is_paused() || !model.animated || !model.loading;
    let active_detail = model.error.clone().unwrap_or_else(|| model.detail.clone());

    vec![
        SpinnerRenderNode {
            part: SpinnerPart::Root,
            value: model.label.clone(),
            label: model.label.clone(),
            detail: active_detail.clone(),
            density: model.density,
            size: model.size,
            tone: model.tone,
            speed_ms: model.speed_ms,
            active: state.is_active(SpinnerPart::Root),
            paused,
            animated: model.animated,
            visible: true,
            actionable: model.animated && model.loading && !blocked,
            invalid,
            loading: model.loading,
            disabled: model.disabled,
        },
        SpinnerRenderNode {
            part: SpinnerPart::Track,
            value: "track".to_owned(),
            label: format!("{} track", model.label),
            detail: "Stable circular track preserves layout while loading.".to_owned(),
            density: model.density,
            size: model.size,
            tone: model.tone,
            speed_ms: model.speed_ms,
            active: state.is_active(SpinnerPart::Track),
            paused,
            animated: model.animated,
            visible: model.loading,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        SpinnerRenderNode {
            part: SpinnerPart::Indicator,
            value: if paused { "paused" } else { "spinning" }.to_owned(),
            label: format!("{} indicator", model.label),
            detail: format!("Indicator speed is {}ms per rotation.", model.speed_ms),
            density: model.density,
            size: model.size,
            tone: model.tone,
            speed_ms: model.speed_ms,
            active: state.is_active(SpinnerPart::Indicator),
            paused,
            animated: model.animated,
            visible: model.loading,
            actionable: model.animated && model.loading && !blocked,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        SpinnerRenderNode {
            part: SpinnerPart::Label,
            value: model.label.clone(),
            label: model.label.clone(),
            detail: active_detail,
            density: model.density,
            size: model.size,
            tone: model.tone,
            speed_ms: model.speed_ms,
            active: state.is_active(SpinnerPart::Label),
            paused,
            animated: model.animated,
            visible: model.show_label,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
    ]
}

pub fn spinner_layout_metrics(
    model: &SpinnerModel,
    inline_size: f32,
    border_width: f32,
) -> SpinnerLayoutMetrics {
    let border_width = border_width.max(0.0);
    let invalid = model.error.is_some();
    let dense = model.density == SpinnerDensity::Dense;
    let dense_root = spinner_uses_dense_root_metrics(dense, invalid, model.disabled);
    let padding_inline = if dense_root {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let padding_block = if dense_root {
        scale::space::xs3(inline_size)
    } else {
        scale::space::xs2(inline_size)
    };
    let gap = scale::space::xs2(inline_size);
    let edge = match if spinner_uses_standard_track_metrics(invalid, model.disabled) {
        SpinnerSize::Medium
    } else {
        model.size
    } {
        SpinnerSize::Small => scale::space::s(inline_size),
        SpinnerSize::Medium => scale::space::m(inline_size),
        SpinnerSize::Large => scale::space::l(inline_size),
    };
    let label_font_size = if dense {
        scale::font_size::f00(inline_size)
    } else {
        scale::font_size::f0(inline_size)
    };
    let label_width = if model.show_label {
        scale::estimate_inline_text_width(&model.label, label_font_size, 0.0)
    } else {
        0.0
    };
    let track_width = if model.loading { edge } else { 0.0 };
    let visible_children = usize::from(model.loading)
        + usize::from(model.show_label)
        + usize::from(model.error.is_some());
    let content_gap = gap * visible_children.saturating_sub(1) as f32;
    let error_padding = scale::space::xs(inline_size);
    let error_font_size = scale::font_size::f0(inline_size);
    let error_width = model.error.as_ref().map_or(0.0, |error| {
        border_width * 2.0
            + error_padding * 2.0
            + scale::estimate_inline_text_width(error, error_font_size, 0.0)
    });
    let width = border_width * 2.0
        + padding_inline * 2.0
        + track_width
        + content_gap
        + label_width
        + error_width;
    let error_height = if model.error.is_some() {
        border_width * 2.0 + error_padding * 2.0 + error_font_size * scale::line_height::LH0
    } else {
        0.0
    };
    let content_height = edge
        .max(label_font_size * scale::line_height::LH0)
        .max(error_height);
    let height = border_width * 2.0 + padding_block * 2.0 + content_height;

    SpinnerLayoutMetrics {
        width: width.max(1.0),
        height,
        padding_inline,
        padding_block,
        gap,
        edge,
        border_width,
        label_font_size,
        label_line_height: scale::line_height::LH0,
        error_padding,
        error_font_size,
        error_line_height: scale::line_height::LH0,
        shadow_level: u8::from(!model.disabled),
    }
}

pub const fn spinner_uses_dense_root_metrics(dense: bool, invalid: bool, disabled: bool) -> bool {
    dense && !invalid && !disabled
}

pub const fn spinner_uses_standard_track_metrics(invalid: bool, disabled: bool) -> bool {
    invalid || disabled
}

pub fn default_spinner_model() -> SpinnerModel {
    SpinnerModel::new("Loading components")
        .with_detail("Compact activity state shared by Leptos and Bevy renderers.")
}

fn validate_optional_spinner_error(error: &Option<String>, _: &()) -> garde::Result {
    if let Some(error) = error
        && (error.is_empty() || error.len() > 240)
    {
        return Err(garde::Error::new(
            "spinner error must be between 1 and 240 characters when present",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_spinner_model(&default_spinner_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = SpinnerModel::new("");
        assert!(validate_spinner_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_speed() {
        let model = default_spinner_model().with_speed_ms(100);
        assert!(validate_spinner_model(&model).is_err());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_spinner_model();
        let state = model.state();
        let nodes = spinner_render_nodes(&model, &state);

        assert_eq!(nodes.first().map(|node| node.part), Some(SpinnerPart::Root));
        for part in SpinnerPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn state_tracks_focus_pause_resume_and_clear() {
        let mut state = SpinnerState::new();

        assert_eq!(
            state.apply(SpinnerIntent::Focus(SpinnerPart::Indicator)),
            SpinnerChange::Focused(SpinnerPart::Indicator)
        );
        assert!(state.is_active(SpinnerPart::Indicator));
        assert_eq!(state.apply(SpinnerIntent::Pause), SpinnerChange::Paused);
        assert!(state.is_paused());
        assert_eq!(state.apply(SpinnerIntent::Resume), SpinnerChange::Resumed);
        assert!(!state.is_paused());
        assert_eq!(
            state.apply(SpinnerIntent::ToggleMotion),
            SpinnerChange::MotionToggled(true)
        );
        assert_eq!(state.apply(SpinnerIntent::Clear), SpinnerChange::Cleared);
        assert!(!state.is_active(SpinnerPart::Indicator));
        assert!(!state.is_paused());
    }

    #[test]
    fn layout_metrics_follow_size_density_and_label_visibility() {
        let standard = default_spinner_model();
        let dense = default_spinner_model()
            .with_density(SpinnerDensity::Dense)
            .with_size(SpinnerSize::Small);
        let ready = default_spinner_model().ready();
        let standard_metrics = spinner_layout_metrics(&standard, 1_000.0, 1.0);
        let dense_metrics = spinner_layout_metrics(&dense, 1_000.0, 1.0);
        let ready_metrics = spinner_layout_metrics(&ready, 1_000.0, 1.0);

        assert!(standard_metrics.edge > dense_metrics.edge);
        assert!(standard_metrics.height > dense_metrics.height);
        assert!(ready_metrics.width < standard_metrics.width);
    }

    #[test]
    fn layout_metrics_follow_invalid_and_disabled_class_precedence() {
        let dense = default_spinner_model()
            .with_density(SpinnerDensity::Dense)
            .with_size(SpinnerSize::Small);
        let invalid = dense.clone().with_error("Loading failed");
        let disabled = dense.clone().disabled();
        let dense_metrics = spinner_layout_metrics(&dense, 1_000.0, 1.0);
        let invalid_metrics = spinner_layout_metrics(&invalid, 1_000.0, 1.0);
        let disabled_metrics = spinner_layout_metrics(&disabled, 1_000.0, 1.0);

        assert!(dense_metrics.padding_inline < invalid_metrics.padding_inline);
        assert_eq!(
            invalid_metrics.padding_inline,
            disabled_metrics.padding_inline
        );
        assert_eq!(invalid_metrics.edge, disabled_metrics.edge);
        assert_eq!(
            dense_metrics.label_font_size,
            invalid_metrics.label_font_size
        );
        assert_eq!(invalid_metrics.shadow_level, 1);
        assert_eq!(disabled_metrics.shadow_level, 0);
        assert!(invalid_metrics.error_padding > 0.0);
    }
}
