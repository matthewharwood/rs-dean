use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SliderDensity {
    Standard,
    Dense,
}

impl SliderDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

impl SliderOrientation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SliderPart {
    Root,
    Track,
    Range,
    Thumb,
    Value,
}

impl SliderPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Track,
        Self::Range,
        Self::Thumb,
        Self::Value,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Slider",
            Self::Track => "SliderTrack",
            Self::Range => "SliderRange",
            Self::Thumb => "SliderThumb",
            Self::Value => "SliderValue",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SliderModel {
    #[garde(skip)]
    pub density: SliderDensity,
    #[garde(skip)]
    pub orientation: SliderOrientation,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(skip)]
    pub min: i32,
    #[garde(custom(slider_max_greater_than_min(&self.min)))]
    pub max: i32,
    #[garde(range(min = 1, max = 1_000), custom(slider_step_fits_range(&self.min, &self.max)))]
    pub step: u16,
    #[garde(custom(slider_value_matches_range(&self.min, &self.max, &self.step)))]
    pub value: i32,
    #[garde(length(min = 1, max = 24))]
    pub unit: String,
    #[garde(custom(validate_optional_slider_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliderState {
    min: i32,
    max: i32,
    step: u16,
    value: i32,
    focused: bool,
    dragging: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SliderIntent {
    Focus,
    Blur,
    StartDrag,
    StopDrag,
    SetValue(i32),
    Increment,
    Decrement,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SliderChange {
    Focused,
    Blurred,
    DragStarted,
    DragStopped,
    ValueChanged(i32),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliderRenderNode {
    pub part: SliderPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: SliderDensity,
    pub orientation: SliderOrientation,
    pub min: i32,
    pub max: i32,
    pub step: u16,
    pub current_value: i32,
    pub percent: u8,
    pub focused: bool,
    pub dragging: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderLayoutMetrics {
    pub width: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub header_gap: f32,
    pub label_font_size: f32,
    pub label_line_height: f32,
    pub value_padding_inline: f32,
    pub value_padding_block: f32,
    pub value_font_size: f32,
    pub value_line_height: f32,
    pub track_wrap_min_height: f32,
    pub track_wrap_width: f32,
    pub track_height: f32,
    pub track_width: f32,
    pub detail_font_size: f32,
    pub detail_line_height: f32,
    pub error_padding: f32,
    pub error_font_size: f32,
    pub error_line_height: f32,
    pub shadow_level: u8,
    standard_thumb_size: f32,
    dense_thumb_size: f32,
    dense: bool,
}

impl SliderLayoutMetrics {
    pub const fn thumb_size(self, focused: bool, disabled: bool) -> f32 {
        if slider_uses_dense_thumb_metrics(self.dense, focused, disabled) {
            self.dense_thumb_size
        } else {
            self.standard_thumb_size
        }
    }

    pub fn track_content_height(self, border_width: f32, bordered: bool) -> f32 {
        (self.track_height
            - if bordered {
                border_width.max(0.0) * 2.0
            } else {
                0.0
            })
        .max(0.0)
    }

    pub fn thumb_content_size(self, focused: bool, disabled: bool, border_width: f32) -> f32 {
        (self.thumb_size(focused, disabled) - border_width.max(0.0) * 2.0).max(0.0)
    }
}

impl SliderModel {
    pub fn new(min: i32, max: i32, value: i32) -> Self {
        Self {
            density: SliderDensity::Standard,
            orientation: SliderOrientation::Horizontal,
            label: "Slider".to_owned(),
            detail: "Numeric input state remains local unless the consuming app persists it."
                .to_owned(),
            min,
            max,
            step: 1,
            value,
            unit: "%".to_owned(),
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SliderDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub const fn with_step(mut self, step: u16) -> Self {
        self.step = step;
        self
    }

    pub const fn with_value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = unit.into();
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

    pub fn state(&self) -> SliderState {
        SliderState::new(self.min, self.max, self.step, self.value)
    }
}

impl SliderState {
    pub fn new(min: i32, max: i32, step: u16, value: i32) -> Self {
        Self {
            min,
            max,
            step,
            value: normalize_slider_value(min, max, step, value),
            focused: false,
            dragging: false,
        }
    }

    pub const fn value(&self) -> i32 {
        self.value
    }

    pub const fn min(&self) -> i32 {
        self.min
    }

    pub const fn max(&self) -> i32 {
        self.max
    }

    pub const fn step(&self) -> u16 {
        self.step
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub const fn is_dragging(&self) -> bool {
        self.dragging
    }

    pub fn percent(&self) -> u8 {
        slider_percent(self.min, self.max, self.value)
    }

    pub fn apply(&mut self, intent: SliderIntent) -> SliderChange {
        match intent {
            SliderIntent::Focus => self.focus(),
            SliderIntent::Blur => self.blur(),
            SliderIntent::StartDrag => self.start_drag(),
            SliderIntent::StopDrag => self.stop_drag(),
            SliderIntent::SetValue(value) => self.set_value(value),
            SliderIntent::Increment => {
                self.set_value(self.value.saturating_add(i32::from(self.step)))
            }
            SliderIntent::Decrement => {
                self.set_value(self.value.saturating_sub(i32::from(self.step)))
            }
            SliderIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self) -> SliderChange {
        if self.focused {
            SliderChange::Unchanged
        } else {
            self.focused = true;
            SliderChange::Focused
        }
    }

    fn blur(&mut self) -> SliderChange {
        if self.focused {
            self.focused = false;
            SliderChange::Blurred
        } else {
            SliderChange::Unchanged
        }
    }

    fn start_drag(&mut self) -> SliderChange {
        if self.dragging {
            SliderChange::Unchanged
        } else {
            self.dragging = true;
            SliderChange::DragStarted
        }
    }

    fn stop_drag(&mut self) -> SliderChange {
        if self.dragging {
            self.dragging = false;
            SliderChange::DragStopped
        } else {
            SliderChange::Unchanged
        }
    }

    fn set_value(&mut self, value: i32) -> SliderChange {
        let next = normalize_slider_value(self.min, self.max, self.step, value);
        if self.value == next {
            SliderChange::Unchanged
        } else {
            self.value = next;
            SliderChange::ValueChanged(next)
        }
    }

    fn clear(&mut self) -> SliderChange {
        if self.focused || self.dragging {
            self.focused = false;
            self.dragging = false;
            SliderChange::Cleared
        } else {
            SliderChange::Unchanged
        }
    }
}

pub fn validate_slider_model(model: &SliderModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn slider_layout_metrics(
    model: &SliderModel,
    available_width: f32,
    inline_size: f32,
) -> SliderLayoutMetrics {
    let dense = model.density == SliderDensity::Dense;
    let vertical = model.orientation == SliderOrientation::Vertical;
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let dense_root = slider_uses_dense_root_metrics(dense, vertical, invalid, model.disabled);
    let vertical_track = slider_uses_vertical_track_metrics(vertical, invalid, blocked);
    let dense_track = slider_uses_dense_track_metrics(dense, vertical, invalid, blocked);

    SliderLayoutMetrics {
        width: available_width.clamp(1.0, scale::container::CONTROL),
        root_padding: if dense_root {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        root_gap: if dense_root {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        header_gap: scale::space::xs(inline_size),
        label_font_size: scale::font_size::f0(inline_size),
        label_line_height: scale::line_height::LH0,
        value_padding_inline: scale::space::xs(inline_size),
        value_padding_block: scale::space::xs3(inline_size),
        value_font_size: scale::font_size::f00(inline_size),
        value_line_height: scale::line_height::LH0,
        track_wrap_min_height: if vertical_track {
            scale::space::xl(inline_size)
        } else {
            scale::space::FIELD
        },
        track_wrap_width: if vertical_track {
            scale::space::l(inline_size)
        } else {
            f32::INFINITY
        },
        track_height: if vertical_track {
            scale::space::xl(inline_size)
        } else if dense_track {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        track_width: if vertical_track {
            scale::space::xs(inline_size)
        } else {
            f32::INFINITY
        },
        detail_font_size: scale::font_size::f0(inline_size),
        detail_line_height: scale::line_height::LH0,
        error_padding: scale::space::xs(inline_size),
        error_font_size: scale::font_size::f0(inline_size),
        error_line_height: scale::line_height::LH0,
        shadow_level: u8::from(!model.disabled),
        standard_thumb_size: scale::space::s(inline_size),
        dense_thumb_size: scale::space::xs(inline_size),
        dense,
    }
}

pub const fn slider_uses_dense_root_metrics(
    dense: bool,
    vertical: bool,
    invalid: bool,
    disabled: bool,
) -> bool {
    dense && !vertical && !invalid && !disabled
}

pub const fn slider_uses_vertical_track_metrics(
    vertical: bool,
    invalid: bool,
    blocked: bool,
) -> bool {
    vertical && !invalid && !blocked
}

pub const fn slider_uses_dense_track_metrics(
    dense: bool,
    vertical: bool,
    invalid: bool,
    blocked: bool,
) -> bool {
    dense && !vertical && !invalid && !blocked
}

pub const fn slider_uses_dense_thumb_metrics(dense: bool, focused: bool, disabled: bool) -> bool {
    dense && !focused && !disabled
}

pub fn slider_render_nodes(model: &SliderModel, state: &SliderState) -> Vec<SliderRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.disabled || model.loading;
    let value_label = slider_value_label(state.value(), &model.unit);
    let percent = state.percent();

    vec![
        SliderRenderNode {
            part: SliderPart::Root,
            value: model.label.clone(),
            label: model.label.clone(),
            detail: model.error.clone().unwrap_or_else(|| model.detail.clone()),
            density: model.density,
            orientation: model.orientation,
            min: state.min(),
            max: state.max(),
            step: state.step(),
            current_value: state.value(),
            percent,
            focused: state.is_focused(),
            dragging: state.is_dragging(),
            visible: true,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: model.disabled,
        },
        SliderRenderNode {
            part: SliderPart::Track,
            value: "track".to_owned(),
            label: format!("{} track", model.label),
            detail: "The track exposes the full numeric range.".to_owned(),
            density: model.density,
            orientation: model.orientation,
            min: state.min(),
            max: state.max(),
            step: state.step(),
            current_value: state.value(),
            percent,
            focused: state.is_focused(),
            dragging: state.is_dragging(),
            visible: true,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        SliderRenderNode {
            part: SliderPart::Range,
            value: format!("{percent}"),
            label: format!("{} selected range", model.label),
            detail: format!("Selected range covers {percent}% of the track."),
            density: model.density,
            orientation: model.orientation,
            min: state.min(),
            max: state.max(),
            step: state.step(),
            current_value: state.value(),
            percent,
            focused: state.is_focused(),
            dragging: state.is_dragging(),
            visible: true,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        SliderRenderNode {
            part: SliderPart::Thumb,
            value: state.value().to_string(),
            label: format!("{} thumb", model.label),
            detail: "The thumb is renderer-local unless the consuming app persists changes."
                .to_owned(),
            density: model.density,
            orientation: model.orientation,
            min: state.min(),
            max: state.max(),
            step: state.step(),
            current_value: state.value(),
            percent,
            focused: state.is_focused(),
            dragging: state.is_dragging(),
            visible: true,
            actionable: !blocked,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        SliderRenderNode {
            part: SliderPart::Value,
            value: value_label.clone(),
            label: value_label,
            detail: "Formatted slider value.".to_owned(),
            density: model.density,
            orientation: model.orientation,
            min: state.min(),
            max: state.max(),
            step: state.step(),
            current_value: state.value(),
            percent,
            focused: state.is_focused(),
            dragging: state.is_dragging(),
            visible: true,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
    ]
}

pub fn default_slider_model() -> SliderModel {
    SliderModel::new(0, 100, 64)
        .with_label("Completion")
        .with_step(4)
        .with_detail("Token-backed slider value shared by Leptos DOM and Bevy primitives.")
}

pub fn slider_percent(min: i32, max: i32, value: i32) -> u8 {
    if max <= min {
        return 0;
    }
    let span = i64::from(max) - i64::from(min);
    let offset = i64::from(value.clamp(min, max)) - i64::from(min);
    ((offset * 100) / span).clamp(0, 100) as u8
}

pub fn slider_value_label(value: i32, unit: &str) -> String {
    format!("{value}{unit}")
}

fn normalize_slider_value(min: i32, max: i32, step: u16, value: i32) -> i32 {
    if max <= min {
        return min;
    }
    let clamped = value.clamp(min, max);
    let step = i32::from(step.max(1));
    let offset = clamped.saturating_sub(min);
    let steps = offset.saturating_add(step / 2) / step;
    min.saturating_add(steps.saturating_mul(step))
        .clamp(min, max)
}

fn slider_max_greater_than_min(min: &i32) -> impl FnOnce(&i32, &()) -> garde::Result + '_ {
    move |max, _| {
        if *max <= *min {
            return Err(garde::Error::new("slider max must be greater than min"));
        }
        Ok(())
    }
}

fn slider_step_fits_range<'a>(
    min: &'a i32,
    max: &'a i32,
) -> impl FnOnce(&u16, &()) -> garde::Result + 'a {
    move |step, _| {
        let span = max.saturating_sub(*min);
        if span <= 0 || i32::from(*step) > span {
            return Err(garde::Error::new(
                "slider step must fit inside the numeric range",
            ));
        }
        Ok(())
    }
}

fn slider_value_matches_range<'a>(
    min: &'a i32,
    max: &'a i32,
    step: &'a u16,
) -> impl FnOnce(&i32, &()) -> garde::Result + 'a {
    move |value, _| {
        if *value < *min || *value > *max {
            return Err(garde::Error::new(
                "slider value must be inside the numeric range",
            ));
        }
        let step = i32::from((*step).max(1));
        if (*value).saturating_sub(*min) % step != 0 {
            return Err(garde::Error::new(
                "slider value must align with min plus step increments",
            ));
        }
        Ok(())
    }
}

fn validate_optional_slider_error(value: &Option<String>, _: &()) -> garde::Result {
    if let Some(value) = value
        && (value.is_empty() || value.len() > 240)
    {
        return Err(garde::Error::new(
            "slider error must be between 1 and 240 characters when present",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_slider_model(&default_slider_model()).is_ok());
    }

    #[test]
    fn garde_rejects_invalid_range() {
        let model = SliderModel::new(10, 10, 10);
        assert!(validate_slider_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_step_outside_range() {
        let model = SliderModel::new(0, 10, 0).with_step(20);
        assert!(validate_slider_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_value_outside_range() {
        let model = SliderModel::new(0, 100, 128);
        assert!(validate_slider_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unaligned_value() {
        let model = SliderModel::new(0, 100, 63).with_step(4);
        assert!(validate_slider_model(&model).is_err());
    }

    #[test]
    fn layout_metrics_follow_orientation_density_and_state_precedence() {
        let standard = slider_layout_metrics(&default_slider_model(), 448.0, 1_000.0);
        let dense_model = default_slider_model().with_density(SliderDensity::Dense);
        let dense = slider_layout_metrics(&dense_model, 448.0, 1_000.0);
        let vertical_model = default_slider_model().with_orientation(SliderOrientation::Vertical);
        let vertical = slider_layout_metrics(&vertical_model, 448.0, 1_000.0);
        let dense_disabled = slider_layout_metrics(&dense_model.disabled(), 448.0, 1_000.0);

        assert_eq!(standard.width, scale::container::CONTROL);
        assert!(dense.root_padding < standard.root_padding);
        assert!(dense.track_height < standard.track_height);
        assert!(vertical.track_height > standard.track_height);
        assert!(vertical.track_width.is_finite());
        assert_eq!(dense_disabled.root_padding, standard.root_padding);
        assert_eq!(
            dense.thumb_size(true, false),
            standard.thumb_size(false, false)
        );
        assert_eq!(
            dense.thumb_size(false, true),
            standard.thumb_size(false, false)
        );
    }

    #[test]
    fn border_box_helpers_preserve_track_and_thumb_outer_sizes() {
        let metrics = slider_layout_metrics(&default_slider_model(), 448.0, 1_000.0);

        assert_eq!(
            metrics.track_content_height(2.0, true) + 4.0,
            metrics.track_height
        );
        assert_eq!(
            metrics.thumb_content_size(false, false, 2.0) + 4.0,
            metrics.thumb_size(false, false)
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_slider_model();
        let state = model.state();
        let nodes = slider_render_nodes(&model, &state);

        assert_eq!(nodes.first().map(|node| node.part), Some(SliderPart::Root));
        for part in SliderPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn state_clamps_steps_and_tracks_focus_drag() {
        let model = default_slider_model();
        let mut state = model.state();

        assert_eq!(state.value(), 64);
        assert_eq!(state.apply(SliderIntent::Focus), SliderChange::Focused);
        assert!(state.is_focused());
        assert_eq!(
            state.apply(SliderIntent::StartDrag),
            SliderChange::DragStarted
        );
        assert!(state.is_dragging());
        assert_eq!(
            state.apply(SliderIntent::SetValue(73)),
            SliderChange::ValueChanged(72)
        );
        assert_eq!(state.value(), 72);
        assert_eq!(
            state.apply(SliderIntent::Increment),
            SliderChange::ValueChanged(76)
        );
        assert_eq!(state.apply(SliderIntent::Clear), SliderChange::Cleared);
        assert!(!state.is_focused());
        assert!(!state.is_dragging());
    }
}
