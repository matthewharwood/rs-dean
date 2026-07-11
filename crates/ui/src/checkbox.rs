use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckboxDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckboxChecked {
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckboxDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl CheckboxChecked {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unchecked => "unchecked",
            Self::Checked => "checked",
            Self::Indeterminate => "indeterminate",
        }
    }

    pub const fn aria_checked(self) -> &'static str {
        match self {
            Self::Unchecked => "false",
            Self::Checked => "true",
            Self::Indeterminate => "mixed",
        }
    }

    pub const fn is_active(self) -> bool {
        !matches!(self, Self::Unchecked)
    }

    pub const fn toggle_target(self) -> Self {
        match self {
            Self::Unchecked | Self::Indeterminate => Self::Checked,
            Self::Checked => Self::Unchecked,
        }
    }
}

impl CheckboxPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Indicator, Self::Label, Self::Description];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Checkbox",
            Self::Indicator => "CheckboxIndicator",
            Self::Label => "CheckboxLabel",
            Self::Description => "CheckboxDescription",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CheckboxModel {
    #[garde(skip)]
    pub density: CheckboxDensity,
    #[garde(skip)]
    pub checked: CheckboxChecked,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(custom(validate_optional_checkbox_copy))]
    pub description: Option<String>,
    #[garde(custom(validate_optional_checkbox_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckboxState {
    checked: CheckboxChecked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckboxIntent {
    Toggle,
    Set(CheckboxChecked),
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckboxChange {
    Changed(CheckboxChecked),
    Reset,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckboxPart {
    Root,
    Indicator,
    Label,
    Description,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckboxRenderNode {
    pub part: CheckboxPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: CheckboxDensity,
    pub checked: CheckboxChecked,
    pub required: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckboxLayoutMetrics {
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub gap: f32,
    pub text_gap: f32,
    pub indicator_size: f32,
    pub label_font_size: f32,
    pub helper_font_size: f32,
    pub label_height: f32,
    pub description_height: f32,
}

impl CheckboxModel {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            density: CheckboxDensity::Standard,
            checked: CheckboxChecked::Unchecked,
            label: label.into(),
            value: value.into(),
            description: Some("Renderer-local state until a consumer persists it.".to_owned()),
            error: None,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: CheckboxDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_checked(mut self, checked: CheckboxChecked) -> Self {
        self.checked = checked;
        self
    }

    pub const fn checked(mut self) -> Self {
        self.checked = CheckboxChecked::Checked;
        self
    }

    pub const fn indeterminate(mut self) -> Self {
        self.checked = CheckboxChecked::Indeterminate;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn without_description(mut self) -> Self {
        self.description = None;
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

    pub const fn state(&self) -> CheckboxState {
        CheckboxState::new(self.checked)
    }
}

impl CheckboxState {
    pub const fn new(checked: CheckboxChecked) -> Self {
        Self { checked }
    }

    pub const fn checked(self) -> CheckboxChecked {
        self.checked
    }

    pub const fn is_checked(self) -> bool {
        matches!(self.checked, CheckboxChecked::Checked)
    }

    pub const fn is_active(self) -> bool {
        self.checked.is_active()
    }

    pub fn apply(&mut self, intent: CheckboxIntent) -> CheckboxChange {
        match intent {
            CheckboxIntent::Toggle => self.set(self.checked.toggle_target()),
            CheckboxIntent::Set(checked) => self.set(checked),
            CheckboxIntent::Reset => {
                if self.checked == CheckboxChecked::Unchecked {
                    CheckboxChange::Unchanged
                } else {
                    self.checked = CheckboxChecked::Unchecked;
                    CheckboxChange::Reset
                }
            }
        }
    }

    fn set(&mut self, checked: CheckboxChecked) -> CheckboxChange {
        if self.checked == checked {
            CheckboxChange::Unchanged
        } else {
            self.checked = checked;
            CheckboxChange::Changed(checked)
        }
    }
}

pub fn validate_checkbox_model(model: &CheckboxModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn checkbox_render_nodes(
    model: &CheckboxModel,
    state: &CheckboxState,
) -> Vec<CheckboxRenderNode> {
    let checked = state.checked();
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let description = model
        .error
        .clone()
        .or_else(|| model.description.clone())
        .unwrap_or_else(|| "No checkbox description".to_owned());
    vec![
        CheckboxRenderNode {
            part: CheckboxPart::Root,
            value: model.value.clone(),
            label: model.label.clone(),
            detail: format!("{} {} checkbox", model.density.label(), checked.label()),
            density: model.density,
            checked,
            required: model.required,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        CheckboxRenderNode {
            part: CheckboxPart::Indicator,
            value: model.value.clone(),
            label: "Checkbox indicator".to_owned(),
            detail: checked.label().to_owned(),
            density: model.density,
            checked,
            required: model.required,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        CheckboxRenderNode {
            part: CheckboxPart::Label,
            value: model.value.clone(),
            label: model.label.clone(),
            detail: if model.required {
                "Required checkbox label".to_owned()
            } else {
                "Checkbox label".to_owned()
            },
            density: model.density,
            checked,
            required: model.required,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        CheckboxRenderNode {
            part: CheckboxPart::Description,
            value: model.value.clone(),
            label: if invalid {
                "Checkbox error".to_owned()
            } else {
                "Checkbox description".to_owned()
            },
            detail: description,
            density: model.density,
            checked,
            required: model.required,
            invalid,
            loading: model.loading,
            disabled: blocked || (model.description.is_none() && model.error.is_none()),
        },
    ]
}

pub fn checkbox_layout_metrics(
    model: &CheckboxModel,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> CheckboxLayoutMetrics {
    let dense = model.density == CheckboxDensity::Dense;
    let width = available_width.clamp(1.0, scale::container::CONTROL);
    let border_width = border_width.max(0.0);
    let padding = if dense {
        scale::space::xs(inline_size)
    } else {
        scale::space::s(inline_size)
    };
    let gap = if dense {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let text_gap = scale::space::xs3(inline_size);
    let indicator_size = scale::space::s(inline_size);
    let label_font_size = if dense {
        scale::font_size::f00(inline_size)
    } else {
        scale::font_size::f0(inline_size)
    };
    let helper_font_size = scale::font_size::f00(inline_size);
    let text_width = (width - padding * 2.0 - border_width * 2.0 - indicator_size - gap).max(1.0);
    let label_copy = if model.required {
        format!("{} *", model.label)
    } else {
        model.label.clone()
    };
    let description = model
        .error
        .as_deref()
        .or(model.description.as_deref())
        .unwrap_or("No checkbox description");
    let label_height = scale::estimate_text_block_height(
        &label_copy,
        text_width,
        label_font_size,
        scale::line_height::LH0,
        0.48,
    );
    let description_height = scale::estimate_text_block_height(
        description,
        text_width,
        helper_font_size,
        scale::line_height::LH0,
        0.44,
    );
    let content_height = label_height + text_gap + description_height;
    let height = border_width * 2.0 + padding * 2.0 + indicator_size.max(content_height);

    CheckboxLayoutMetrics {
        width,
        height,
        padding,
        gap,
        text_gap,
        indicator_size,
        label_font_size,
        helper_font_size,
        label_height,
        description_height,
    }
}

pub fn default_checkbox_model() -> CheckboxModel {
    CheckboxModel::new("Enable shared theme", "shared-theme")
        .with_description("Use the shared Rust token palette across Leptos and Bevy.")
        .checked()
}

fn validate_optional_checkbox_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new(
            "checkbox copy must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_checkbox_model(&default_checkbox_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = CheckboxModel::new("", "shared-theme");
        assert!(validate_checkbox_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = CheckboxModel::new("Enable shared theme", "");
        assert!(validate_checkbox_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_description() {
        let model = default_checkbox_model().with_description("");
        assert!(validate_checkbox_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_checkbox_model().with_error("");
        assert!(validate_checkbox_model(&model).is_err());
    }

    #[test]
    fn state_toggles_and_resets() {
        let mut state = CheckboxState::new(CheckboxChecked::Unchecked);
        assert!(!state.is_active());
        assert_eq!(
            state.apply(CheckboxIntent::Toggle),
            CheckboxChange::Changed(CheckboxChecked::Checked)
        );
        assert!(state.is_checked());
        assert_eq!(
            state.apply(CheckboxIntent::Set(CheckboxChecked::Indeterminate)),
            CheckboxChange::Changed(CheckboxChecked::Indeterminate)
        );
        assert!(state.is_active());
        assert_eq!(state.apply(CheckboxIntent::Reset), CheckboxChange::Reset);
        assert_eq!(state.checked(), CheckboxChecked::Unchecked);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_checkbox_model();
        let nodes = checkbox_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(CheckboxPart::Root)
        );
        for part in CheckboxPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn error_marks_description_invalid() {
        let model = default_checkbox_model().with_error("Select this option before continuing.");
        let nodes = checkbox_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(
            nodes.iter().any(
                |node| node.part == CheckboxPart::Description && node.label == "Checkbox error"
            )
        );
    }

    #[test]
    fn loading_disables_indicator_node() {
        let model = default_checkbox_model().loading();
        let nodes = checkbox_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == CheckboxPart::Indicator && node.disabled)
        );
    }

    #[test]
    fn layout_metrics_match_the_shared_checkbox_box_model() {
        let model = CheckboxModel::new("Use shared theme tokens", "shared-theme")
            .with_description("Leptos and Bevy read the same semantic checkbox state.")
            .checked();
        let metrics = checkbox_layout_metrics(&model, 860.0, 1_000.0, 1.0);
        let content_height = metrics.label_height + metrics.text_gap + metrics.description_height;
        let expected = 2.0 + metrics.padding * 2.0 + metrics.indicator_size.max(content_height);

        assert_eq!(metrics.width, scale::container::CONTROL);
        assert!((metrics.height - expected).abs() < 0.01);
    }

    #[test]
    fn dense_checkbox_metrics_remain_smaller_than_standard_metrics() {
        let standard = default_checkbox_model();
        let dense = standard.clone().with_density(CheckboxDensity::Dense);
        let standard = checkbox_layout_metrics(&standard, 448.0, 1_000.0, 1.0);
        let dense = checkbox_layout_metrics(&dense, 448.0, 1_000.0, 1.0);

        assert!(dense.height < standard.height);
        assert!(dense.padding < standard.padding);
    }
}
