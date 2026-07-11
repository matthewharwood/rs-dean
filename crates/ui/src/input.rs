use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputDensity {
    Standard,
    Dense,
}

impl InputDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputKind {
    Text,
    Email,
    Password,
    Search,
    Url,
}

impl InputKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Search => "search",
            Self::Url => "url",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputPart {
    Root,
    Prefix,
    Control,
    Suffix,
}

impl InputPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Prefix, Self::Control, Self::Suffix];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Input",
            Self::Prefix => "InputPrefix",
            Self::Control => "InputControl",
            Self::Suffix => "InputSuffix",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct InputAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct InputModel {
    #[garde(skip)]
    pub density: InputDensity,
    #[garde(skip)]
    pub input_kind: InputKind,
    #[garde(length(max = 240))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub placeholder: String,
    #[garde(custom(validate_optional_input_copy))]
    pub prefix: Option<String>,
    #[garde(custom(validate_optional_input_action))]
    pub suffix: Option<InputAction>,
    #[garde(custom(validate_optional_input_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputState {
    focused: bool,
    value: String,
    active_part: Option<InputPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputIntent {
    Focus,
    Blur,
    Input(String),
    ActivateSuffix(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputChange {
    Focused,
    Blurred,
    Input(String),
    SuffixActivated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputRenderNode {
    pub part: InputPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: InputDensity,
    pub input_kind: InputKind,
    pub focused: bool,
    pub active: bool,
    pub invalid: bool,
    pub visible: bool,
    pub actionable: bool,
    pub required: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InputLayoutMetrics {
    pub max_width: f32,
    pub root_gap: f32,
    pub row_min_height: f32,
    pub prefix_padding_inline: f32,
    pub prefix_font_size: f32,
    pub prefix_line_height: f32,
    pub control_padding_inline: f32,
    pub control_padding_block: f32,
    pub control_font_size: f32,
    pub control_line_height: f32,
    pub suffix_padding_inline: f32,
    pub suffix_font_size: f32,
    pub suffix_line_height: f32,
    pub error_font_size: f32,
    pub error_line_height: f32,
}

impl InputAction {
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

impl InputModel {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            density: InputDensity::Standard,
            input_kind: InputKind::Text,
            value: String::new(),
            placeholder: placeholder.into(),
            prefix: None,
            suffix: None,
            error: None,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: InputDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_input_kind(mut self, input_kind: InputKind) -> Self {
        self.input_kind = input_kind;
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn without_prefix(mut self) -> Self {
        self.prefix = None;
        self
    }

    pub fn with_suffix(mut self, suffix: InputAction) -> Self {
        self.suffix = Some(suffix);
        self
    }

    pub fn without_suffix(mut self) -> Self {
        self.suffix = None;
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

    pub fn state(&self) -> InputState {
        InputState::new(self.value.clone())
    }
}

impl InputState {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            focused: false,
            value: value.into(),
            active_part: None,
        }
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub const fn is_active(&self, part: InputPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn apply(&mut self, intent: InputIntent) -> InputChange {
        match intent {
            InputIntent::Focus => self.focus(),
            InputIntent::Blur => self.blur(),
            InputIntent::Input(value) => self.input(value),
            InputIntent::ActivateSuffix(value) => self.activate_suffix(value),
            InputIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self) -> InputChange {
        if self.focused {
            InputChange::Unchanged
        } else {
            self.focused = true;
            self.active_part = Some(InputPart::Control);
            InputChange::Focused
        }
    }

    fn blur(&mut self) -> InputChange {
        if self.focused || self.active_part.is_some() {
            self.focused = false;
            self.active_part = None;
            InputChange::Blurred
        } else {
            InputChange::Unchanged
        }
    }

    fn input(&mut self, value: String) -> InputChange {
        if self.value == value {
            InputChange::Unchanged
        } else {
            self.focused = true;
            self.active_part = Some(InputPart::Control);
            self.value = value.clone();
            InputChange::Input(value)
        }
    }

    fn activate_suffix(&mut self, value: String) -> InputChange {
        if value.is_empty() {
            InputChange::Unchanged
        } else {
            self.active_part = Some(InputPart::Suffix);
            InputChange::SuffixActivated(value)
        }
    }

    fn clear(&mut self) -> InputChange {
        if self.value.is_empty() {
            InputChange::Unchanged
        } else {
            self.value.clear();
            self.active_part = None;
            InputChange::Cleared
        }
    }
}

pub fn validate_input_model(model: &InputModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn input_layout_metrics(
    model: &InputModel,
    state: &InputState,
    inline_size: f32,
) -> InputLayoutMetrics {
    let blocked = model.loading || model.disabled;
    let suffix_disabled = model
        .suffix
        .as_ref()
        .is_none_or(|suffix| suffix.disabled || blocked);
    input_shell_layout_metrics(
        model.density,
        model.loading,
        model.disabled,
        suffix_disabled,
        state.is_active(InputPart::Suffix),
        inline_size,
    )
}

pub(crate) fn input_shell_layout_metrics(
    density: InputDensity,
    loading: bool,
    disabled: bool,
    action_disabled: bool,
    action_active: bool,
    inline_size: f32,
) -> InputLayoutMetrics {
    let dense = density == InputDensity::Dense;
    let blocked = loading || disabled;
    let dense_row = dense && !blocked;
    let dense_control = dense && !blocked;
    let dense_suffix = dense && !action_disabled && !action_active;

    InputLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_gap: scale::space::xs2(inline_size),
        row_min_height: if dense_row {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        prefix_padding_inline: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        prefix_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        prefix_line_height: scale::line_height::LH0,
        control_padding_inline: if dense_control {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        control_padding_block: if dense_control {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        control_font_size: if dense_control {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        control_line_height: scale::line_height::LH0,
        suffix_padding_inline: if dense_suffix {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        suffix_font_size: if dense_suffix {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        suffix_line_height: scale::line_height::LH0,
        error_font_size: scale::font_size::f00(inline_size),
        error_line_height: scale::line_height::LH0,
    }
}

pub fn input_render_nodes(model: &InputModel, state: &InputState) -> Vec<InputRenderNode> {
    let blocked = model.loading || model.disabled;
    let prefix = model.prefix.as_deref().unwrap_or("No prefix");
    let suffix = model.suffix.as_ref();
    let suffix_label = suffix
        .map(|suffix| suffix.label.as_str())
        .unwrap_or("No suffix");
    let suffix_value = suffix
        .map(|suffix| suffix.value.as_str())
        .unwrap_or("input-suffix");
    let suffix_disabled = suffix.map(|suffix| suffix.disabled).unwrap_or(true);
    let control_detail = model
        .error
        .as_deref()
        .unwrap_or("Single-line input control");

    vec![
        input_node(
            model,
            state,
            InputNodeDraft::new(
                InputPart::Root,
                "input",
                "Input",
                control_detail,
                true,
                false,
                model.disabled,
            ),
        ),
        input_node(
            model,
            state,
            InputNodeDraft::new(
                InputPart::Prefix,
                "input-prefix",
                prefix,
                "Input prefix",
                model.prefix.is_some(),
                false,
                model.prefix.is_none() || model.disabled,
            ),
        ),
        input_node(
            model,
            state,
            InputNodeDraft::new(
                InputPart::Control,
                state.value(),
                &model.placeholder,
                control_detail,
                true,
                false,
                blocked,
            ),
        ),
        input_node(
            model,
            state,
            InputNodeDraft::new(
                InputPart::Suffix,
                suffix_value,
                suffix_label,
                "Input suffix action",
                suffix.is_some(),
                suffix.is_some() && !suffix_disabled,
                suffix.is_none() || suffix_disabled || blocked,
            ),
        ),
    ]
}

pub fn default_input_model() -> InputModel {
    InputModel::new("engmanager.xyz")
        .with_input_kind(InputKind::Url)
        .with_value("engmanager.xyz")
        .with_prefix("https://")
        .with_suffix(InputAction::new("Copy", "copy-url"))
}

struct InputNodeDraft<'a> {
    part: InputPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    actionable: bool,
    disabled: bool,
}

impl<'a> InputNodeDraft<'a> {
    const fn new(
        part: InputPart,
        value: &'a str,
        label: &'a str,
        detail: &'a str,
        visible: bool,
        actionable: bool,
        disabled: bool,
    ) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            visible,
            actionable,
            disabled,
        }
    }
}

fn input_node(
    model: &InputModel,
    state: &InputState,
    draft: InputNodeDraft<'_>,
) -> InputRenderNode {
    InputRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        input_kind: model.input_kind,
        focused: state.is_focused(),
        active: state.is_active(draft.part),
        invalid: model.error.is_some(),
        visible: draft.visible,
        actionable: draft.actionable,
        required: model.required,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn validate_optional_input_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && (copy.is_empty() || copy.len() > 320)
    {
        return Err(garde::Error::new(
            "input optional copy must be 1..=320 characters",
        ));
    }
    Ok(())
}

fn validate_optional_input_action(action: &Option<InputAction>, _context: &()) -> garde::Result {
    if let Some(action) = action
        && action.validate().is_err()
    {
        return Err(garde::Error::new(
            "input suffix action must include non-empty label and value",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_input_model(&default_input_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_placeholder() {
        let model = InputModel::new("");
        assert!(validate_input_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_prefix() {
        let model = default_input_model().with_prefix("");
        assert!(validate_input_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_suffix_contract() {
        let model = default_input_model().with_suffix(InputAction::new("", "copy"));
        assert!(validate_input_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_input_model().with_error("");
        assert!(validate_input_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_input_suffix_activation_and_clear() {
        let mut state = InputState::new("engmanager.xyz");
        assert_eq!(state.apply(InputIntent::Focus), InputChange::Focused);
        assert!(state.is_focused());
        assert_eq!(
            state.apply(InputIntent::Input("rs-dean.dev".to_owned())),
            InputChange::Input("rs-dean.dev".to_owned())
        );
        assert_eq!(state.value(), "rs-dean.dev");
        assert_eq!(
            state.apply(InputIntent::ActivateSuffix("copy-url".to_owned())),
            InputChange::SuffixActivated("copy-url".to_owned())
        );
        assert!(state.is_active(InputPart::Suffix));
        assert_eq!(state.apply(InputIntent::Clear), InputChange::Cleared);
        assert_eq!(state.value(), "");
        assert_eq!(state.apply(InputIntent::Blur), InputChange::Blurred);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_input_model();
        let nodes = input_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), InputPart::ALL.len());
        assert_eq!(nodes.first().map(|node| node.part), Some(InputPart::Root));
        for part in InputPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn missing_prefix_and_suffix_keep_disabled_nodes() {
        let model = default_input_model().without_prefix().without_suffix();
        let nodes = input_render_nodes(&model, &model.state());
        for part in [InputPart::Prefix, InputPart::Suffix] {
            let node = nodes
                .iter()
                .find(|node| node.part == part)
                .expect("input render nodes include optional anatomy");
            assert!(!node.visible);
            assert!(node.disabled);
        }
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_input_model().with_error("URL is required.");
        let nodes = input_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(
            nodes
                .iter()
                .any(|node| node.part == InputPart::Control && node.detail == "URL is required.")
        );
    }

    #[test]
    fn layout_metrics_preserve_dense_blocked_and_active_suffix_precedence() {
        let dense_model = default_input_model().with_density(InputDensity::Dense);
        let resting = dense_model.state();
        let mut active = dense_model.state();
        active.apply(InputIntent::ActivateSuffix("copy-url".to_owned()));
        let dense = input_layout_metrics(&dense_model, &resting, 1_024.0);
        let active = input_layout_metrics(&dense_model, &active, 1_024.0);
        let loading = input_layout_metrics(&dense_model.loading(), &resting, 1_024.0);

        assert_eq!(dense.max_width, scale::container::CONTROL);
        assert!(dense.control_font_size < loading.control_font_size);
        assert!(dense.suffix_font_size < active.suffix_font_size);
        assert_eq!(loading.row_min_height, scale::space::FIELD);
        assert_eq!(loading.prefix_font_size, dense.prefix_font_size);
    }
}
