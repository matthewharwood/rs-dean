use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToggleDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToggleVariant {
    Default,
    Outline,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TogglePressed {
    Unpressed,
    Pressed,
}

impl ToggleDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl ToggleVariant {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
        }
    }
}

impl TogglePressed {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unpressed => "unpressed",
            Self::Pressed => "pressed",
        }
    }

    pub const fn aria_pressed(self) -> &'static str {
        match self {
            Self::Unpressed => "false",
            Self::Pressed => "true",
        }
    }

    pub const fn is_pressed(self) -> bool {
        matches!(self, Self::Pressed)
    }

    pub const fn toggle_target(self) -> Self {
        match self {
            Self::Unpressed => Self::Pressed,
            Self::Pressed => Self::Unpressed,
        }
    }
}

impl TogglePart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Indicator, Self::Label];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Toggle",
            Self::Indicator => "ToggleIndicator",
            Self::Label => "ToggleLabel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ToggleModel {
    #[garde(skip)]
    pub density: ToggleDensity,
    #[garde(skip)]
    pub variant: ToggleVariant,
    #[garde(skip)]
    pub pressed: TogglePressed,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(length(min = 1, max = 48))]
    pub pressed_label: String,
    #[garde(length(min = 1, max = 48))]
    pub unpressed_label: String,
    #[garde(length(min = 1, max = 24))]
    pub pressed_indicator: String,
    #[garde(length(min = 1, max = 24))]
    pub unpressed_indicator: String,
    #[garde(custom(validate_optional_toggle_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToggleState {
    pressed: TogglePressed,
    active_part: Option<TogglePart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleIntent {
    Toggle,
    Set(TogglePressed),
    Focus(TogglePart),
    Blur,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleChange {
    Changed(TogglePressed),
    Focused(TogglePart),
    Blurred,
    Reset,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TogglePart {
    Root,
    Indicator,
    Label,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToggleRenderNode {
    pub part: TogglePart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: ToggleDensity,
    pub variant: ToggleVariant,
    pub pressed: TogglePressed,
    pub active: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
    pub actionable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToggleLayoutMetrics {
    pub width: f32,
    pub frame_gap: f32,
    pub button_width: f32,
    pub button_height: f32,
    pub button_padding_inline: f32,
    pub button_padding_block: f32,
    pub button_gap: f32,
    pub button_font_size: f32,
    pub button_line_height: f32,
    pub indicator_width: f32,
    pub indicator_height: f32,
    pub indicator_padding_inline: f32,
    pub indicator_font_size: f32,
    pub indicator_line_height: f32,
    pub indicator_letter_spacing: f32,
    pub status_width: f32,
    pub status_height: f32,
    pub status_padding_inline: f32,
    pub status_padding_block: f32,
    pub status_font_size: f32,
    pub status_line_height: f32,
    pub status_letter_spacing: f32,
    pub detail_font_size: f32,
    pub detail_line_height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToggleControlLayoutMetrics {
    pub width: f32,
    pub height: f32,
    pub padding_inline: f32,
    pub padding_block: f32,
    pub gap: f32,
    pub font_size: f32,
    pub line_height: f32,
    pub indicator_width: f32,
    pub indicator_height: f32,
    pub indicator_padding_inline: f32,
    pub indicator_font_size: f32,
    pub indicator_line_height: f32,
    pub indicator_letter_spacing: f32,
}

impl ToggleModel {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            density: ToggleDensity::Standard,
            variant: ToggleVariant::Default,
            pressed: TogglePressed::Unpressed,
            label: label.into(),
            value: value.into(),
            detail: "Renderer-local pressed state until the consumer persists the tool choice."
                .to_owned(),
            pressed_label: "Pressed".to_owned(),
            unpressed_label: "Unpressed".to_owned(),
            pressed_indicator: "on".to_owned(),
            unpressed_indicator: "off".to_owned(),
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: ToggleDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_pressed(mut self, pressed: TogglePressed) -> Self {
        self.pressed = pressed;
        self
    }

    pub const fn pressed(mut self) -> Self {
        self.pressed = TogglePressed::Pressed;
        self
    }

    pub const fn unpressed(mut self) -> Self {
        self.pressed = TogglePressed::Unpressed;
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub fn with_pressed_label(mut self, label: impl Into<String>) -> Self {
        self.pressed_label = label.into();
        self
    }

    pub fn with_unpressed_label(mut self, label: impl Into<String>) -> Self {
        self.unpressed_label = label.into();
        self
    }

    pub fn with_pressed_indicator(mut self, indicator: impl Into<String>) -> Self {
        self.pressed_indicator = indicator.into();
        self
    }

    pub fn with_unpressed_indicator(mut self, indicator: impl Into<String>) -> Self {
        self.unpressed_indicator = indicator.into();
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
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

    pub const fn state(&self) -> ToggleState {
        ToggleState::new(self.pressed)
    }
}

impl ToggleState {
    pub const fn new(pressed: TogglePressed) -> Self {
        Self {
            pressed,
            active_part: None,
        }
    }

    pub const fn pressed(self) -> TogglePressed {
        self.pressed
    }

    pub const fn is_pressed(self) -> bool {
        self.pressed.is_pressed()
    }

    pub const fn active_part(self) -> Option<TogglePart> {
        self.active_part
    }

    pub const fn is_active(self, part: TogglePart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn apply(&mut self, intent: ToggleIntent) -> ToggleChange {
        match intent {
            ToggleIntent::Toggle => self.set(self.pressed.toggle_target()),
            ToggleIntent::Set(pressed) => self.set(pressed),
            ToggleIntent::Focus(part) => {
                if self.active_part == Some(part) {
                    ToggleChange::Unchanged
                } else {
                    self.active_part = Some(part);
                    ToggleChange::Focused(part)
                }
            }
            ToggleIntent::Blur => {
                if self.active_part.is_none() {
                    ToggleChange::Unchanged
                } else {
                    self.active_part = None;
                    ToggleChange::Blurred
                }
            }
            ToggleIntent::Reset => {
                if self.pressed == TogglePressed::Unpressed && self.active_part.is_none() {
                    ToggleChange::Unchanged
                } else {
                    self.pressed = TogglePressed::Unpressed;
                    self.active_part = None;
                    ToggleChange::Reset
                }
            }
        }
    }

    fn set(&mut self, pressed: TogglePressed) -> ToggleChange {
        if self.pressed == pressed {
            ToggleChange::Unchanged
        } else {
            self.pressed = pressed;
            ToggleChange::Changed(pressed)
        }
    }
}

pub fn validate_toggle_model(model: &ToggleModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn toggle_render_nodes(model: &ToggleModel, state: &ToggleState) -> Vec<ToggleRenderNode> {
    let pressed = state.pressed();
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let status = toggle_status_label(model, pressed);
    let indicator = toggle_indicator_label(model, pressed);
    let detail = model.error.clone().unwrap_or_else(|| model.detail.clone());
    vec![
        ToggleRenderNode {
            part: TogglePart::Root,
            value: model.value.clone(),
            label: model.label.clone(),
            detail: status.clone(),
            density: model.density,
            variant: model.variant,
            pressed,
            active: state.is_active(TogglePart::Root),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: !blocked,
        },
        ToggleRenderNode {
            part: TogglePart::Indicator,
            value: model.value.clone(),
            label: indicator,
            detail: status,
            density: model.density,
            variant: model.variant,
            pressed,
            active: state.is_active(TogglePart::Indicator),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: false,
        },
        ToggleRenderNode {
            part: TogglePart::Label,
            value: model.value.clone(),
            label: model.label.clone(),
            detail,
            density: model.density,
            variant: model.variant,
            pressed,
            active: state.is_active(TogglePart::Label),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: false,
        },
    ]
}

pub fn toggle_layout_metrics(
    model: &ToggleModel,
    state: &ToggleState,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> ToggleLayoutMetrics {
    let border_width = border_width.max(0.0);
    let frame_gap = scale::space::xs2(inline_size);
    let indicator_copy = toggle_indicator_label(model, state.pressed()).to_uppercase();
    let control = toggle_control_layout_metrics(
        model.density,
        &model.label,
        &indicator_copy,
        inline_size,
        border_width,
    );
    let status_font_size = scale::font_size::f00(inline_size);
    let status_padding_inline = scale::space::xs2(inline_size);
    let status_padding_block = scale::space::xs3(inline_size);
    let status_copy = toggle_status_label(model, state.pressed()).to_uppercase();
    let status_text_width = scale::estimate_inline_text_width(
        &status_copy,
        status_font_size,
        scale::letter_spacing::LABEL,
    );
    let status_width = border_width * 2.0 + status_padding_inline * 2.0 + status_text_width;
    let status_height = border_width * 2.0
        + status_padding_block * 2.0
        + status_font_size * scale::line_height::LH0;

    ToggleLayoutMetrics {
        width: available_width.clamp(1.0, scale::container::CONTROL),
        frame_gap,
        button_width: control.width,
        button_height: control.height,
        button_padding_inline: control.padding_inline,
        button_padding_block: control.padding_block,
        button_gap: control.gap,
        button_font_size: control.font_size,
        button_line_height: control.line_height,
        indicator_width: control.indicator_width,
        indicator_height: control.indicator_height,
        indicator_padding_inline: control.indicator_padding_inline,
        indicator_font_size: control.indicator_font_size,
        indicator_line_height: control.indicator_line_height,
        indicator_letter_spacing: control.indicator_letter_spacing,
        status_width,
        status_height,
        status_padding_inline,
        status_padding_block,
        status_font_size,
        status_line_height: scale::line_height::LH00,
        status_letter_spacing: scale::letter_spacing::LABEL,
        detail_font_size: scale::font_size::f0(inline_size),
        detail_line_height: scale::line_height::LH0,
    }
}

pub fn toggle_control_layout_metrics(
    density: ToggleDensity,
    label: &str,
    indicator_copy: &str,
    inline_size: f32,
    border_width: f32,
) -> ToggleControlLayoutMetrics {
    let border_width = border_width.max(0.0);
    let dense = density == ToggleDensity::Dense;
    let padding_inline = if dense {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let padding_block = if dense {
        scale::space::xs3(inline_size)
    } else {
        scale::space::xs2(inline_size)
    };
    let gap = scale::space::xs2(inline_size);
    let font_size = if dense {
        scale::font_size::f00(inline_size)
    } else {
        scale::font_size::f0(inline_size)
    };
    let line_height = scale::line_height::LH0;
    let indicator_font_size = scale::font_size::f00(inline_size);
    let indicator_padding_inline = scale::space::xs2(inline_size);
    let indicator_text_width = scale::estimate_inline_text_width(
        indicator_copy,
        indicator_font_size,
        scale::letter_spacing::LABEL,
    );
    let indicator_height = scale::space::s(inline_size);
    let indicator_width =
        (indicator_text_width + indicator_padding_inline * 2.0).max(indicator_height);
    let label_width = scale::estimate_inline_text_width(label, font_size, 0.0);
    let width = border_width * 2.0 + padding_inline * 2.0 + indicator_width + gap + label_width;
    let content_height = indicator_height.max(font_size * line_height);
    let minimum_height = if dense {
        scale::space::s(inline_size)
    } else {
        scale::space::L
    };
    let height = minimum_height.max(border_width * 2.0 + padding_block * 2.0 + content_height);

    ToggleControlLayoutMetrics {
        width,
        height,
        padding_inline,
        padding_block,
        gap,
        font_size,
        line_height,
        indicator_width,
        indicator_height,
        indicator_padding_inline,
        indicator_font_size,
        indicator_line_height: scale::line_height::LH00,
        indicator_letter_spacing: scale::letter_spacing::LABEL,
    }
}

pub fn default_toggle_model() -> ToggleModel {
    ToggleModel::new("Bold", "bold")
        .with_detail("Pressing the toggle updates local tool state; the editor persists accepted formatting.")
        .with_pressed_label("Active")
        .with_unpressed_label("Inactive")
        .with_pressed_indicator("on")
        .with_unpressed_indicator("off")
        .pressed()
}

pub fn toggle_status_label(model: &ToggleModel, pressed: TogglePressed) -> String {
    match pressed {
        TogglePressed::Unpressed => model.unpressed_label.clone(),
        TogglePressed::Pressed => model.pressed_label.clone(),
    }
}

pub fn toggle_indicator_label(model: &ToggleModel, pressed: TogglePressed) -> String {
    match pressed {
        TogglePressed::Unpressed => model.unpressed_indicator.clone(),
        TogglePressed::Pressed => model.pressed_indicator.clone(),
    }
}

fn validate_optional_toggle_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new("toggle copy must be 1 to 240 characters"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_toggle_model(&default_toggle_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = ToggleModel::new("", "bold");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = ToggleModel::new("Bold", "");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_status_labels() {
        let model = default_toggle_model().with_pressed_label("");
        assert!(validate_toggle_model(&model).is_err());
        let model = default_toggle_model().with_unpressed_label("");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_indicator_labels() {
        let model = default_toggle_model().with_pressed_indicator("");
        assert!(validate_toggle_model(&model).is_err());
        let model = default_toggle_model().with_unpressed_indicator("");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_toggle_model().with_error("");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn state_toggles_sets_focuses_and_resets() {
        let mut state = ToggleState::new(TogglePressed::Unpressed);
        assert!(!state.is_pressed());
        assert_eq!(
            state.apply(ToggleIntent::Toggle),
            ToggleChange::Changed(TogglePressed::Pressed)
        );
        assert!(state.is_pressed());
        assert_eq!(
            state.apply(ToggleIntent::Set(TogglePressed::Unpressed)),
            ToggleChange::Changed(TogglePressed::Unpressed)
        );
        assert_eq!(
            state.apply(ToggleIntent::Focus(TogglePart::Root)),
            ToggleChange::Focused(TogglePart::Root)
        );
        assert!(state.is_active(TogglePart::Root));
        assert_eq!(state.apply(ToggleIntent::Reset), ToggleChange::Reset);
        assert_eq!(state.pressed(), TogglePressed::Unpressed);
        assert_eq!(state.active_part(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_toggle_model();
        let nodes = toggle_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(TogglePart::Root));
        for part in TogglePart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_toggle_model().with_error("Toolbar state failed validation.");
        let nodes = toggle_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(nodes.iter().any(|node| node.part == TogglePart::Label
            && node.detail == "Toolbar state failed validation."));
    }

    #[test]
    fn loading_disables_root_action() {
        let model = default_toggle_model().loading();
        let nodes = toggle_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == TogglePart::Root && node.disabled && !node.actionable)
        );
    }

    #[test]
    fn layout_metrics_follow_shared_density_and_container_tokens() {
        let standard = default_toggle_model();
        let dense = default_toggle_model().with_density(ToggleDensity::Dense);
        let standard_metrics =
            toggle_layout_metrics(&standard, &standard.state(), 640.0, 480.0, 1.0);
        let dense_metrics = toggle_layout_metrics(&dense, &dense.state(), 640.0, 480.0, 1.0);

        assert_eq!(standard_metrics.width, scale::container::CONTROL);
        assert!(dense_metrics.button_height < standard_metrics.button_height);
        assert!(dense_metrics.button_width < standard_metrics.button_width);
        assert!(standard_metrics.indicator_width >= standard_metrics.indicator_height);
        assert_eq!(
            standard_metrics.indicator_letter_spacing,
            scale::letter_spacing::LABEL
        );
        assert_eq!(
            standard_metrics.status_letter_spacing,
            scale::letter_spacing::LABEL
        );
        assert_eq!(standard_metrics.button_line_height, scale::line_height::LH0);
        assert_eq!(standard_metrics.detail_line_height, scale::line_height::LH0);
    }

    #[test]
    fn layout_metrics_use_current_shared_status_copy() {
        let model = ToggleModel::new("Bold", "bold")
            .with_pressed_label("Enabled for selection")
            .with_unpressed_label("Off");
        let unpressed = toggle_layout_metrics(&model, &model.state(), 320.0, 320.0, 1.0);
        let mut state = model.state();
        state.apply(ToggleIntent::Toggle);
        let pressed = toggle_layout_metrics(&model, &state, 320.0, 320.0, 1.0);

        assert!(pressed.status_width > unpressed.status_width);
    }

    #[test]
    fn standalone_toggle_uses_shared_control_geometry() {
        let model = default_toggle_model();
        let metrics = toggle_layout_metrics(&model, &model.state(), 320.0, 320.0, 1.0);
        let control = toggle_control_layout_metrics(model.density, &model.label, "ON", 320.0, 1.0);

        assert_eq!(metrics.button_width, control.width);
        assert_eq!(metrics.button_height, control.height);
        assert_eq!(metrics.indicator_width, control.indicator_width);
        assert_eq!(metrics.button_font_size, control.font_size);
    }
}
