use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwitchDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwitchChecked {
    Off,
    On,
}

impl SwitchDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl SwitchChecked {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::On => "on",
        }
    }

    pub const fn aria_checked(self) -> &'static str {
        match self {
            Self::Off => "false",
            Self::On => "true",
        }
    }

    pub const fn is_on(self) -> bool {
        matches!(self, Self::On)
    }

    pub const fn toggle_target(self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

impl SwitchPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Track, Self::Thumb, Self::Label];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Switch",
            Self::Track => "SwitchTrack",
            Self::Thumb => "SwitchThumb",
            Self::Label => "SwitchLabel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SwitchModel {
    #[garde(skip)]
    pub density: SwitchDensity,
    #[garde(skip)]
    pub checked: SwitchChecked,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(length(min = 1, max = 48))]
    pub on_label: String,
    #[garde(length(min = 1, max = 48))]
    pub off_label: String,
    #[garde(custom(validate_optional_switch_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchState {
    checked: SwitchChecked,
    active_part: Option<SwitchPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchIntent {
    Toggle,
    Set(SwitchChecked),
    Focus(SwitchPart),
    Blur,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchChange {
    Changed(SwitchChecked),
    Focused(SwitchPart),
    Blurred,
    Reset,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwitchPart {
    Root,
    Track,
    Thumb,
    Label,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchRenderNode {
    pub part: SwitchPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: SwitchDensity,
    pub checked: SwitchChecked,
    pub required: bool,
    pub active: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
    pub actionable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwitchLayoutMetrics {
    pub width: f32,
    pub height: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub copy_gap: f32,
    pub label_row_gap: f32,
    pub label_font_size: f32,
    pub detail_font_size: f32,
    pub status_font_size: f32,
    pub status_padding_inline: f32,
    pub status_padding_block: f32,
    pub status_letter_spacing: f32,
    pub status_width: f32,
    pub track_width: f32,
    pub track_height: f32,
    pub track_padding: f32,
    pub thumb_size: f32,
    pub label_height: f32,
    pub detail_height: f32,
    pub status_height: f32,
    pub error_height: f32,
}

impl SwitchModel {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            density: SwitchDensity::Standard,
            checked: SwitchChecked::Off,
            label: label.into(),
            value: value.into(),
            detail: "Renderer-local draft state until the consumer persists the choice.".to_owned(),
            on_label: "On".to_owned(),
            off_label: "Off".to_owned(),
            error: None,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SwitchDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_checked(mut self, checked: SwitchChecked) -> Self {
        self.checked = checked;
        self
    }

    pub const fn checked(mut self) -> Self {
        self.checked = SwitchChecked::On;
        self
    }

    pub const fn unchecked(mut self) -> Self {
        self.checked = SwitchChecked::Off;
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub fn with_on_label(mut self, label: impl Into<String>) -> Self {
        self.on_label = label.into();
        self
    }

    pub fn with_off_label(mut self, label: impl Into<String>) -> Self {
        self.off_label = label.into();
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub const fn required(mut self) -> Self {
        self.required = true;
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

    pub const fn state(&self) -> SwitchState {
        SwitchState::new(self.checked)
    }
}

impl SwitchState {
    pub const fn new(checked: SwitchChecked) -> Self {
        Self {
            checked,
            active_part: None,
        }
    }

    pub const fn checked(self) -> SwitchChecked {
        self.checked
    }

    pub const fn is_on(self) -> bool {
        self.checked.is_on()
    }

    pub const fn active_part(self) -> Option<SwitchPart> {
        self.active_part
    }

    pub const fn is_active(self, part: SwitchPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn apply(&mut self, intent: SwitchIntent) -> SwitchChange {
        match intent {
            SwitchIntent::Toggle => self.set(self.checked.toggle_target()),
            SwitchIntent::Set(checked) => self.set(checked),
            SwitchIntent::Focus(part) => {
                if self.active_part == Some(part) {
                    SwitchChange::Unchanged
                } else {
                    self.active_part = Some(part);
                    SwitchChange::Focused(part)
                }
            }
            SwitchIntent::Blur => {
                if self.active_part.is_none() {
                    SwitchChange::Unchanged
                } else {
                    self.active_part = None;
                    SwitchChange::Blurred
                }
            }
            SwitchIntent::Reset => {
                if self.checked == SwitchChecked::Off && self.active_part.is_none() {
                    SwitchChange::Unchanged
                } else {
                    self.checked = SwitchChecked::Off;
                    self.active_part = None;
                    SwitchChange::Reset
                }
            }
        }
    }

    fn set(&mut self, checked: SwitchChecked) -> SwitchChange {
        if self.checked == checked {
            SwitchChange::Unchanged
        } else {
            self.checked = checked;
            SwitchChange::Changed(checked)
        }
    }
}

pub fn validate_switch_model(model: &SwitchModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn switch_render_nodes(model: &SwitchModel, state: &SwitchState) -> Vec<SwitchRenderNode> {
    let checked = state.checked();
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let status = switch_status_label(model, checked).to_owned();
    let detail = model.error.clone().unwrap_or_else(|| model.detail.clone());
    vec![
        SwitchRenderNode {
            part: SwitchPart::Root,
            value: model.value.clone(),
            label: model.label.clone(),
            detail: format!("{} {} switch", model.density.label(), checked.label()),
            density: model.density,
            checked,
            required: model.required,
            active: state.is_active(SwitchPart::Root),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: !blocked,
        },
        SwitchRenderNode {
            part: SwitchPart::Track,
            value: model.value.clone(),
            label: "Switch track".to_owned(),
            detail: status.clone(),
            density: model.density,
            checked,
            required: model.required,
            active: state.is_active(SwitchPart::Track),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: !blocked,
        },
        SwitchRenderNode {
            part: SwitchPart::Thumb,
            value: model.value.clone(),
            label: "Switch thumb".to_owned(),
            detail: status,
            density: model.density,
            checked,
            required: model.required,
            active: state.is_active(SwitchPart::Thumb),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: false,
        },
        SwitchRenderNode {
            part: SwitchPart::Label,
            value: model.value.clone(),
            label: model.label.clone(),
            detail,
            density: model.density,
            checked,
            required: model.required,
            active: state.is_active(SwitchPart::Label),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: false,
        },
    ]
}

pub const fn switch_uses_dense_root_metrics(
    density: SwitchDensity,
    disabled: bool,
    invalid: bool,
) -> bool {
    matches!(density, SwitchDensity::Dense) && !disabled && !invalid
}

pub const fn switch_uses_dense_track_metrics(
    density: SwitchDensity,
    blocked: bool,
    invalid: bool,
) -> bool {
    matches!(density, SwitchDensity::Dense) && !blocked && !invalid
}

pub fn switch_layout_metrics(
    model: &SwitchModel,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> SwitchLayoutMetrics {
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let dense_root = switch_uses_dense_root_metrics(model.density, model.disabled, invalid);
    let dense_track = switch_uses_dense_track_metrics(model.density, blocked, invalid);
    let width = available_width.clamp(1.0, scale::container::CONTROL);
    let border_width = border_width.max(0.0);
    let root_padding = if dense_root {
        scale::space::xs(inline_size)
    } else {
        scale::space::s(inline_size)
    };
    let root_gap = if dense_root {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let copy_gap = scale::space::xs3(inline_size);
    let label_row_gap = scale::space::xs2(inline_size);
    let label_font_size = scale::font_size::f0(inline_size);
    let detail_font_size = label_font_size;
    let status_font_size = scale::font_size::f00(inline_size);
    let status_padding_inline = scale::space::xs2(inline_size);
    let status_padding_block = scale::space::xs3(inline_size);
    let status_letter_spacing = scale::letter_spacing::LABEL;
    let track_width = if dense_track {
        scale::space::l(inline_size)
    } else {
        scale::space::xl(inline_size)
    };
    let track_height = if dense_track {
        scale::space::s(inline_size)
    } else {
        scale::space::m(inline_size)
    };
    let track_padding = scale::space::xs3(inline_size);
    let thumb_size = if dense_track {
        scale::space::xs(inline_size)
    } else {
        scale::space::s(inline_size)
    };
    let copy_width =
        (width - border_width * 2.0 - root_padding * 2.0 - root_gap - track_width).max(1.0);
    let status_width = switch_status_width(
        switch_status_label(model, model.checked),
        inline_size,
        border_width,
    );
    let label_width = (copy_width - label_row_gap - status_width).max(1.0);
    let label_copy = if model.required {
        format!("{} *", model.label)
    } else {
        model.label.clone()
    };
    let label_height = scale::estimate_text_block_height(
        &label_copy,
        label_width,
        label_font_size,
        scale::line_height::LH0,
        0.56,
    );
    let status_height = status_font_size * scale::line_height::LH0
        + status_padding_block * 2.0
        + border_width * 2.0;
    let detail_copy = model.error.as_deref().unwrap_or(model.detail.as_str());
    let detail_height = scale::estimate_text_block_height(
        detail_copy,
        copy_width,
        detail_font_size,
        scale::line_height::LH0,
        0.5,
    );
    let error_height = model.error.as_ref().map_or(0.0, |error| {
        let error_padding = scale::space::xs(inline_size);
        scale::estimate_text_block_height(
            error,
            (copy_width - error_padding * 2.0 - border_width * 2.0).max(1.0),
            detail_font_size,
            scale::line_height::LH0,
            0.5,
        ) + error_padding * 2.0
            + border_width * 2.0
    });
    let copy_height = label_height.max(status_height)
        + copy_gap
        + detail_height
        + if error_height > 0.0 {
            copy_gap + error_height
        } else {
            0.0
        };
    let height = border_width * 2.0 + root_padding * 2.0 + copy_height.max(track_height);

    SwitchLayoutMetrics {
        width,
        height,
        root_padding,
        root_gap,
        copy_gap,
        label_row_gap,
        label_font_size,
        detail_font_size,
        status_font_size,
        status_padding_inline,
        status_padding_block,
        status_letter_spacing,
        status_width,
        track_width,
        track_height,
        track_padding,
        thumb_size,
        label_height,
        detail_height,
        status_height,
        error_height,
    }
}

pub fn default_switch_model() -> SwitchModel {
    SwitchModel::new("Enable shared theme", "shared-theme")
        .with_detail("Persist this choice through app state while the renderer owns focus only.")
        .with_on_label("Enabled")
        .with_off_label("Disabled")
        .checked()
}

pub fn switch_status_label(model: &SwitchModel, checked: SwitchChecked) -> &str {
    match checked {
        SwitchChecked::Off => &model.off_label,
        SwitchChecked::On => &model.on_label,
    }
}

pub fn switch_status_width(status: &str, inline_size: f32, border_width: f32) -> f32 {
    scale::estimate_inline_text_width(
        &status.to_uppercase(),
        scale::font_size::f00(inline_size),
        scale::letter_spacing::LABEL,
    ) + scale::space::xs2(inline_size) * 2.0
        + border_width.max(0.0) * 2.0
}

fn validate_optional_switch_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new("switch copy must be 1 to 240 characters"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_switch_model(&default_switch_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = SwitchModel::new("", "shared-theme");
        assert!(validate_switch_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = SwitchModel::new("Enable shared theme", "");
        assert!(validate_switch_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_status_labels() {
        let model = default_switch_model().with_on_label("");
        assert!(validate_switch_model(&model).is_err());
        let model = default_switch_model().with_off_label("");
        assert!(validate_switch_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_switch_model().with_error("");
        assert!(validate_switch_model(&model).is_err());
    }

    #[test]
    fn state_toggles_sets_focuses_and_resets() {
        let mut state = SwitchState::new(SwitchChecked::Off);
        assert!(!state.is_on());
        assert_eq!(
            state.apply(SwitchIntent::Toggle),
            SwitchChange::Changed(SwitchChecked::On)
        );
        assert!(state.is_on());
        assert_eq!(
            state.apply(SwitchIntent::Set(SwitchChecked::Off)),
            SwitchChange::Changed(SwitchChecked::Off)
        );
        assert_eq!(
            state.apply(SwitchIntent::Focus(SwitchPart::Track)),
            SwitchChange::Focused(SwitchPart::Track)
        );
        assert!(state.is_active(SwitchPart::Track));
        assert_eq!(state.apply(SwitchIntent::Reset), SwitchChange::Reset);
        assert_eq!(state.checked(), SwitchChecked::Off);
        assert_eq!(state.active_part(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_switch_model();
        let nodes = switch_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(SwitchPart::Root));
        for part in SwitchPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn error_marks_label_invalid() {
        let model = default_switch_model().with_error("Choose a valid setting before continuing.");
        let nodes = switch_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(nodes.iter().any(|node| node.part == SwitchPart::Label
            && node.detail == "Choose a valid setting before continuing."));
    }

    #[test]
    fn loading_disables_actionable_nodes() {
        let model = default_switch_model().loading();
        let nodes = switch_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == SwitchPart::Track && node.disabled && !node.actionable)
        );
    }

    #[test]
    fn layout_metrics_follow_token_scale_and_control_width() {
        let model = default_switch_model();
        let metrics = switch_layout_metrics(&model, 860.0, 1_000.0, 1.0);
        assert_eq!(metrics.width, scale::container::CONTROL);
        assert_eq!(metrics.root_padding, scale::space::s(1_000.0));
        assert_eq!(metrics.track_width, scale::space::xl(1_000.0));
        assert_eq!(metrics.thumb_size, scale::space::s(1_000.0));
        assert!(metrics.height > metrics.track_height);
    }

    #[test]
    fn dense_metrics_preserve_css_state_precedence() {
        let dense = default_switch_model().with_density(SwitchDensity::Dense);
        let dense_metrics = switch_layout_metrics(&dense, 448.0, 1_000.0, 1.0);
        assert_eq!(dense_metrics.root_padding, scale::space::xs(1_000.0));
        assert_eq!(dense_metrics.track_width, scale::space::l(1_000.0));
        assert_eq!(dense_metrics.thumb_size, scale::space::xs(1_000.0));

        let loading = dense.clone().loading();
        let loading_metrics = switch_layout_metrics(&loading, 448.0, 1_000.0, 1.0);
        assert_eq!(loading_metrics.root_padding, scale::space::xs(1_000.0));
        assert_eq!(loading_metrics.track_width, scale::space::xl(1_000.0));
        assert_eq!(loading_metrics.thumb_size, scale::space::s(1_000.0));

        let disabled = dense.clone().disabled();
        let disabled_metrics = switch_layout_metrics(&disabled, 448.0, 1_000.0, 1.0);
        assert_eq!(disabled_metrics.root_padding, scale::space::s(1_000.0));
        assert_eq!(disabled_metrics.track_width, scale::space::xl(1_000.0));

        let invalid = dense.with_error("Choose a valid setting.");
        let invalid_metrics = switch_layout_metrics(&invalid, 448.0, 1_000.0, 1.0);
        assert_eq!(invalid_metrics.root_padding, scale::space::s(1_000.0));
        assert_eq!(invalid_metrics.track_width, scale::space::xl(1_000.0));
        assert!(invalid_metrics.error_height > 0.0);
    }

    #[test]
    fn status_width_tracks_visible_copy_without_custom_dimensions() {
        let model = default_switch_model()
            .with_on_label("Synced")
            .with_off_label("Local");
        let on = switch_status_width(switch_status_label(&model, SwitchChecked::On), 1_000.0, 1.0);
        let off = switch_status_width(
            switch_status_label(&model, SwitchChecked::Off),
            1_000.0,
            1.0,
        );
        assert!(on > off);
    }
}
