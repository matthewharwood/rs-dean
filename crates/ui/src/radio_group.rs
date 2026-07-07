use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RadioGroupDensity {
    Standard,
    Dense,
}

impl RadioGroupDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RadioGroupOrientation {
    Vertical,
    Horizontal,
}

impl RadioGroupOrientation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RadioGroupPart {
    Root,
    Item,
    Indicator,
    Label,
}

impl RadioGroupPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Item, Self::Indicator, Self::Label];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "RadioGroup",
            Self::Item => "RadioGroupItem",
            Self::Indicator => "RadioGroupIndicator",
            Self::Label => "RadioGroupLabel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct RadioGroupOption {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct RadioGroupModel {
    #[garde(skip)]
    pub density: RadioGroupDensity,
    #[garde(skip)]
    pub orientation: RadioGroupOrientation,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(
        length(min = 1, max = 24),
        dive,
        custom(radio_group_options_are_unique)
    )]
    pub options: Vec<RadioGroupOption>,
    #[garde(custom(radio_group_selected_value_references_enabled_option(&self.options)))]
    pub selected_value: Option<String>,
    #[garde(custom(validate_optional_radio_group_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupState {
    focused_value: Option<String>,
    selected_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RadioGroupIntent {
    Focus(String),
    Select(String),
    Blur,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RadioGroupChange {
    Focused(String),
    Selected(String),
    Blurred,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupRenderNode {
    pub part: RadioGroupPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: RadioGroupDensity,
    pub orientation: RadioGroupOrientation,
    pub selected: bool,
    pub focused: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub required: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl RadioGroupOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            detail: format!("{label} option"),
            label,
            value: value.into(),
            disabled: false,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl RadioGroupModel {
    pub fn new(options: Vec<RadioGroupOption>) -> Self {
        Self {
            density: RadioGroupDensity::Standard,
            orientation: RadioGroupOrientation::Vertical,
            label: "Theme".to_owned(),
            options,
            selected_value: None,
            error: None,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: RadioGroupDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_orientation(mut self, orientation: RadioGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_selected_value(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    pub fn without_selected_value(mut self) -> Self {
        self.selected_value = None;
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

    pub fn state(&self) -> RadioGroupState {
        RadioGroupState::new(self.selected_value.clone())
    }
}

impl RadioGroupState {
    pub fn new(selected_value: Option<String>) -> Self {
        Self {
            focused_value: None,
            selected_value,
        }
    }

    pub fn focused_value(&self) -> Option<&str> {
        self.focused_value.as_deref()
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    pub fn is_focused(&self, value: &str) -> bool {
        self.focused_value.as_deref() == Some(value)
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: RadioGroupIntent) -> RadioGroupChange {
        match intent {
            RadioGroupIntent::Focus(value) => self.focus(value),
            RadioGroupIntent::Select(value) => self.select(value),
            RadioGroupIntent::Blur => self.blur(),
            RadioGroupIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, value: String) -> RadioGroupChange {
        if value.is_empty() || self.focused_value.as_ref() == Some(&value) {
            return RadioGroupChange::Unchanged;
        }
        self.focused_value = Some(value.clone());
        RadioGroupChange::Focused(value)
    }

    fn select(&mut self, value: String) -> RadioGroupChange {
        if value.is_empty() || self.selected_value.as_ref() == Some(&value) {
            return RadioGroupChange::Unchanged;
        }
        self.focused_value = Some(value.clone());
        self.selected_value = Some(value.clone());
        RadioGroupChange::Selected(value)
    }

    fn blur(&mut self) -> RadioGroupChange {
        if self.focused_value.take().is_some() {
            RadioGroupChange::Blurred
        } else {
            RadioGroupChange::Unchanged
        }
    }

    fn clear(&mut self) -> RadioGroupChange {
        if self.focused_value.is_none() && self.selected_value.is_none() {
            return RadioGroupChange::Unchanged;
        }
        self.focused_value = None;
        self.selected_value = None;
        RadioGroupChange::Cleared
    }
}

pub fn validate_radio_group_model(model: &RadioGroupModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn radio_group_render_nodes(
    model: &RadioGroupModel,
    state: &RadioGroupState,
) -> Vec<RadioGroupRenderNode> {
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let mut nodes = Vec::with_capacity(model.options.len().saturating_mul(3).saturating_add(1));
    nodes.push(RadioGroupRenderNode {
        part: RadioGroupPart::Root,
        index: 0,
        value: state.selected_value().unwrap_or("none").to_owned(),
        label: model.label.clone(),
        detail: model
            .error
            .clone()
            .unwrap_or_else(|| format!("{} radio options", model.options.len())),
        density: model.density,
        orientation: model.orientation,
        selected: state.selected_value().is_some(),
        focused: state.focused_value().is_some(),
        visible: true,
        actionable: false,
        invalid,
        required: model.required,
        loading: model.loading,
        disabled: model.disabled,
    });

    for (index, option) in model.options.iter().enumerate() {
        let selected = state.is_selected(&option.value);
        let focused = state.is_focused(&option.value);
        let disabled = blocked || option.disabled;
        nodes.push(RadioGroupRenderNode {
            part: RadioGroupPart::Item,
            index,
            value: option.value.clone(),
            label: option.label.clone(),
            detail: option.detail.clone(),
            density: model.density,
            orientation: model.orientation,
            selected,
            focused,
            visible: true,
            actionable: !disabled,
            invalid,
            required: model.required,
            loading: model.loading,
            disabled,
        });
        nodes.push(RadioGroupRenderNode {
            part: RadioGroupPart::Indicator,
            index,
            value: option.value.clone(),
            label: if selected { "selected" } else { "not selected" }.to_owned(),
            detail: "Selection indicator".to_owned(),
            density: model.density,
            orientation: model.orientation,
            selected,
            focused,
            visible: true,
            actionable: !disabled,
            invalid,
            required: model.required,
            loading: model.loading,
            disabled,
        });
        nodes.push(RadioGroupRenderNode {
            part: RadioGroupPart::Label,
            index,
            value: option.value.clone(),
            label: option.label.clone(),
            detail: option.detail.clone(),
            density: model.density,
            orientation: model.orientation,
            selected,
            focused,
            visible: true,
            actionable: !disabled,
            invalid,
            required: model.required,
            loading: model.loading,
            disabled,
        });
    }
    nodes
}

pub fn default_radio_group_model() -> RadioGroupModel {
    RadioGroupModel::new(default_radio_group_options())
        .with_label("Theme")
        .with_selected_value("light")
}

pub fn default_radio_group_options() -> Vec<RadioGroupOption> {
    vec![
        RadioGroupOption::new("Light", "light").with_detail("Use the light app theme."),
        RadioGroupOption::new("Dark", "dark").with_detail("Use the dark app theme."),
        RadioGroupOption::new("System", "system").with_detail("Follow the device setting."),
    ]
}

pub fn selected_radio_group_label(model: &RadioGroupModel, state: &RadioGroupState) -> String {
    state
        .selected_value()
        .and_then(|value| {
            model
                .options
                .iter()
                .find(|option| option.value == value)
                .map(|option| option.label.clone())
        })
        .unwrap_or_else(|| "No selection".to_owned())
}

fn radio_group_options_are_unique(options: &Vec<RadioGroupOption>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(options.len());
    for option in options {
        if !values.insert(option.value.as_str()) {
            return Err(garde::Error::new(
                "radio group option values must be unique",
            ));
        }
    }
    Ok(())
}

fn radio_group_selected_value_references_enabled_option<'a>(
    options: &'a [RadioGroupOption],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !options
                .iter()
                .any(|option| option.value == *value && !option.disabled)
        {
            return Err(garde::Error::new(
                "selected radio group value must reference an enabled option",
            ));
        }
        Ok(())
    }
}

fn validate_optional_radio_group_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "radio group error must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_radio_group_model(&default_radio_group_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_options() {
        let model = RadioGroupModel::new(Vec::new());
        assert!(validate_radio_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_option_values() {
        let model = RadioGroupModel::new(vec![
            RadioGroupOption::new("Light", "theme"),
            RadioGroupOption::new("Dark", "theme"),
        ]);
        assert!(validate_radio_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_selected_value() {
        let model = RadioGroupModel::new(vec![
            RadioGroupOption::new("Light", "light").disabled(),
            RadioGroupOption::new("Dark", "dark"),
        ])
        .with_selected_value("light");
        assert!(validate_radio_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_radio_group_model().with_error("");
        assert!(validate_radio_group_model(&model).is_err());
    }

    #[test]
    fn state_focuses_selects_blurs_and_clears() {
        let mut state = RadioGroupState::new(None);
        assert_eq!(
            state.apply(RadioGroupIntent::Focus("dark".to_owned())),
            RadioGroupChange::Focused("dark".to_owned())
        );
        assert!(state.is_focused("dark"));
        assert_eq!(
            state.apply(RadioGroupIntent::Select("dark".to_owned())),
            RadioGroupChange::Selected("dark".to_owned())
        );
        assert!(state.is_selected("dark"));
        assert_eq!(
            state.apply(RadioGroupIntent::Blur),
            RadioGroupChange::Blurred
        );
        assert!(!state.is_focused("dark"));
        assert_eq!(
            state.apply(RadioGroupIntent::Clear),
            RadioGroupChange::Cleared
        );
        assert!(state.selected_value().is_none());
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_radio_group_model();
        let nodes = radio_group_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(RadioGroupPart::Root)
        );
        for part in RadioGroupPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn loading_disables_options() {
        let model = default_radio_group_model().loading();
        let nodes = radio_group_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == RadioGroupPart::Item)
                .all(|node| node.disabled && !node.actionable)
        );
    }

    #[test]
    fn selected_label_uses_selected_option() {
        let model = default_radio_group_model().with_selected_value("system");
        assert_eq!(selected_radio_group_label(&model, &model.state()), "System");
    }
}
