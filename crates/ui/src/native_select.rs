use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NativeSelectDensity {
    Standard,
    Dense,
}

impl NativeSelectDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NativeSelectPart {
    Root,
    Trigger,
    Option,
    Value,
}

impl NativeSelectPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Trigger, Self::Option, Self::Value];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "NativeSelect",
            Self::Trigger => "NativeSelectTrigger",
            Self::Option => "NativeSelectOption",
            Self::Value => "NativeSelectValue",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct NativeSelectOption {
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
pub struct NativeSelectModel {
    #[garde(skip)]
    pub density: NativeSelectDensity,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 160))]
    pub placeholder: String,
    #[garde(
        length(min = 1, max = 24),
        dive,
        custom(native_select_options_are_unique)
    )]
    pub options: Vec<NativeSelectOption>,
    #[garde(custom(selected_native_select_value_references_option(&self.options)))]
    pub selected_value: Option<String>,
    #[garde(custom(validate_optional_native_select_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeSelectState {
    focused: bool,
    selected_value: Option<String>,
    active_part: Option<NativeSelectPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeSelectIntent {
    Focus,
    Blur,
    Select(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeSelectChange {
    Focused,
    Blurred,
    Selected(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeSelectRenderNode {
    pub part: NativeSelectPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: NativeSelectDensity,
    pub focused: bool,
    pub active: bool,
    pub selected: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub required: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NativeSelectLayoutMetrics {
    pub max_width: f32,
    pub root_gap: f32,
    pub label_font_size: f32,
    pub label_line_height: f32,
    pub compact_trigger: bool,
    pub trigger_min_height: f32,
    pub trigger_padding_inline: f32,
    pub trigger_padding_block: f32,
    pub trigger_gap: f32,
    pub trigger_font_size: f32,
    pub trigger_line_height: f32,
    pub value_font_size: f32,
    pub value_line_height: f32,
    pub option_panel_padding: f32,
    pub option_panel_gap: f32,
    pub option_padding_inline: f32,
    pub option_padding_block: f32,
}

impl NativeSelectOption {
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

impl NativeSelectModel {
    pub fn new(options: Vec<NativeSelectOption>) -> Self {
        Self {
            density: NativeSelectDensity::Standard,
            label: "Framework".to_owned(),
            placeholder: "Choose framework".to_owned(),
            options,
            selected_value: None,
            error: None,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: NativeSelectDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_options(mut self, options: Vec<NativeSelectOption>) -> Self {
        self.options = options;
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

    pub fn state(&self) -> NativeSelectState {
        NativeSelectState::new(self.selected_value.clone())
    }
}

impl NativeSelectState {
    pub fn new(selected_value: Option<String>) -> Self {
        Self {
            focused: false,
            selected_value,
            active_part: None,
        }
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub const fn is_active(&self, part: NativeSelectPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: NativeSelectIntent) -> NativeSelectChange {
        match intent {
            NativeSelectIntent::Focus => self.focus(),
            NativeSelectIntent::Blur => self.blur(),
            NativeSelectIntent::Select(value) => self.select(value),
            NativeSelectIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self) -> NativeSelectChange {
        if self.focused {
            NativeSelectChange::Unchanged
        } else {
            self.focused = true;
            self.active_part = Some(NativeSelectPart::Trigger);
            NativeSelectChange::Focused
        }
    }

    fn blur(&mut self) -> NativeSelectChange {
        if self.focused || self.active_part.is_some() {
            self.focused = false;
            self.active_part = None;
            NativeSelectChange::Blurred
        } else {
            NativeSelectChange::Unchanged
        }
    }

    fn select(&mut self, value: String) -> NativeSelectChange {
        if value.is_empty() {
            return self.clear();
        }
        if self.selected_value.as_deref() == Some(value.as_str()) {
            NativeSelectChange::Unchanged
        } else {
            self.focused = true;
            self.active_part = Some(NativeSelectPart::Trigger);
            self.selected_value = Some(value.clone());
            NativeSelectChange::Selected(value)
        }
    }

    fn clear(&mut self) -> NativeSelectChange {
        if self.selected_value.is_none() {
            NativeSelectChange::Unchanged
        } else {
            self.selected_value = None;
            self.active_part = Some(NativeSelectPart::Value);
            NativeSelectChange::Cleared
        }
    }
}

pub fn validate_native_select_model(model: &NativeSelectModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn native_select_layout_metrics(
    model: &NativeSelectModel,
    inline_size: f32,
) -> NativeSelectLayoutMetrics {
    let compact_trigger =
        model.density == NativeSelectDensity::Dense && !model.loading && !model.disabled;
    NativeSelectLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_gap: scale::space::xs2(inline_size),
        label_font_size: scale::font_size::f00(inline_size),
        label_line_height: scale::line_height::LH0,
        compact_trigger,
        trigger_min_height: if compact_trigger {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        trigger_padding_inline: if compact_trigger {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        trigger_padding_block: if compact_trigger {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        trigger_gap: if compact_trigger {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        trigger_font_size: if compact_trigger {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        trigger_line_height: scale::line_height::LH0,
        value_font_size: scale::font_size::f00(inline_size),
        value_line_height: scale::line_height::LH0,
        option_panel_padding: scale::space::xs2(inline_size),
        option_panel_gap: scale::space::xs3(inline_size),
        option_padding_inline: if compact_trigger {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        option_padding_block: if compact_trigger {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
    }
}

pub fn native_select_render_nodes(
    model: &NativeSelectModel,
    state: &NativeSelectState,
) -> Vec<NativeSelectRenderNode> {
    let blocked = model.loading || model.disabled;
    let selected_label =
        selected_native_select_label(model, state).unwrap_or_else(|| model.placeholder.clone());
    let selected_value = state.selected_value().unwrap_or("").to_owned();
    let selected_detail = model
        .error
        .clone()
        .unwrap_or_else(|| "Selected native option".to_owned());
    let mut nodes = Vec::with_capacity(model.options.len().saturating_add(3));
    nodes.push(native_select_node(
        model,
        state,
        NativeSelectNodeDraft {
            part: NativeSelectPart::Root,
            index: 0,
            value: &selected_value,
            label: &model.label,
            detail: &selected_detail,
            visible: true,
            actionable: false,
            selected: selected_value.is_empty(),
            disabled: model.disabled,
        },
    ));
    nodes.push(native_select_node(
        model,
        state,
        NativeSelectNodeDraft {
            part: NativeSelectPart::Trigger,
            index: 0,
            value: &selected_value,
            label: &model.label,
            detail: &model.placeholder,
            visible: true,
            actionable: !blocked,
            selected: selected_value.is_empty(),
            disabled: blocked,
        },
    ));
    for (index, option) in model.options.iter().enumerate() {
        let selected = state.is_selected(&option.value);
        nodes.push(native_select_node(
            model,
            state,
            NativeSelectNodeDraft {
                part: NativeSelectPart::Option,
                index,
                value: &option.value,
                label: &option.label,
                detail: &option.detail,
                visible: true,
                actionable: !blocked && !option.disabled,
                selected,
                disabled: blocked || option.disabled,
            },
        ));
    }
    nodes.push(native_select_node(
        model,
        state,
        NativeSelectNodeDraft {
            part: NativeSelectPart::Value,
            index: 0,
            value: &selected_value,
            label: &selected_label,
            detail: &selected_detail,
            visible: true,
            actionable: false,
            selected: !selected_value.is_empty(),
            disabled: blocked,
        },
    ));
    nodes
}

pub fn selected_native_select_label(
    model: &NativeSelectModel,
    state: &NativeSelectState,
) -> Option<String> {
    state.selected_value().and_then(|selected| {
        model
            .options
            .iter()
            .find(|option| option.value == selected)
            .map(|option| option.label.clone())
    })
}

pub fn default_native_select_model() -> NativeSelectModel {
    NativeSelectModel::new(vec![
        NativeSelectOption::new("Leptos", "leptos")
            .with_detail("DOM renderer backed by the shared token contract"),
        NativeSelectOption::new("Bevy", "bevy")
            .with_detail("WebGPU renderer consuming the same Rust render nodes"),
        NativeSelectOption::new("Shared state", "state")
            .with_detail("Durable selections belong above the component boundary"),
    ])
    .with_label("Renderer")
    .with_placeholder("Choose renderer")
    .with_selected_value("leptos")
}

struct NativeSelectNodeDraft<'a> {
    part: NativeSelectPart,
    index: usize,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    actionable: bool,
    selected: bool,
    disabled: bool,
}

fn native_select_node(
    model: &NativeSelectModel,
    state: &NativeSelectState,
    draft: NativeSelectNodeDraft<'_>,
) -> NativeSelectRenderNode {
    NativeSelectRenderNode {
        part: draft.part,
        index: draft.index,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        focused: state.is_focused(),
        active: state.is_active(draft.part),
        selected: draft.selected,
        visible: draft.visible,
        actionable: draft.actionable,
        invalid: model.error.is_some(),
        required: model.required,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn native_select_options_are_unique(
    options: &Vec<NativeSelectOption>,
    _context: &(),
) -> garde::Result {
    let mut values = HashSet::with_capacity(options.len());
    for option in options {
        if !values.insert(option.value.as_str()) {
            return Err(garde::Error::new(
                "native select option values must be unique",
            ));
        }
    }
    Ok(())
}

fn selected_native_select_value_references_option(
    options: &[NativeSelectOption],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + '_ {
    move |selected, _context| {
        if let Some(selected) = selected
            && !options
                .iter()
                .any(|option| option.value == *selected && !option.disabled)
        {
            return Err(garde::Error::new(
                "selected native select value must reference an enabled option",
            ));
        }
        Ok(())
    }
}

fn validate_optional_native_select_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "native select error must be 1..=240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_native_select_model(&default_native_select_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_options() {
        let model = NativeSelectModel::new(Vec::new());
        assert!(validate_native_select_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_option_values() {
        let model = NativeSelectModel::new(vec![
            NativeSelectOption::new("Leptos", "stack"),
            NativeSelectOption::new("Bevy", "stack"),
        ]);
        assert!(validate_native_select_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_selected_value() {
        let model = NativeSelectModel::new(vec![
            NativeSelectOption::new("Leptos", "leptos").disabled(),
            NativeSelectOption::new("Bevy", "bevy"),
        ])
        .with_selected_value("leptos");
        assert!(validate_native_select_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_native_select_model().with_error("");
        assert!(validate_native_select_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_selection_clear_and_blur() {
        let mut state = NativeSelectState::new(None);
        assert_eq!(
            state.apply(NativeSelectIntent::Focus),
            NativeSelectChange::Focused
        );
        assert!(state.is_focused());
        assert_eq!(
            state.apply(NativeSelectIntent::Select("bevy".to_owned())),
            NativeSelectChange::Selected("bevy".to_owned())
        );
        assert_eq!(state.selected_value(), Some("bevy"));
        assert!(state.is_active(NativeSelectPart::Trigger));
        assert_eq!(
            state.apply(NativeSelectIntent::Clear),
            NativeSelectChange::Cleared
        );
        assert_eq!(state.selected_value(), None);
        assert_eq!(
            state.apply(NativeSelectIntent::Blur),
            NativeSelectChange::Blurred
        );
        assert!(!state.is_focused());
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_native_select_model();
        let nodes = native_select_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(NativeSelectPart::Root)
        );
        for part in NativeSelectPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == NativeSelectPart::Option)
                .count(),
            3
        );
    }

    #[test]
    fn layout_metrics_preserve_dense_and_blocked_precedence() {
        let standard = native_select_layout_metrics(&default_native_select_model(), 1_280.0);
        let dense = native_select_layout_metrics(
            &default_native_select_model().with_density(NativeSelectDensity::Dense),
            1_280.0,
        );
        let dense_loading = native_select_layout_metrics(
            &default_native_select_model()
                .with_density(NativeSelectDensity::Dense)
                .loading(),
            1_280.0,
        );

        assert!(dense.compact_trigger);
        assert!(dense.trigger_min_height < standard.trigger_min_height);
        assert!(dense.trigger_padding_inline < standard.trigger_padding_inline);
        assert!(dense.trigger_font_size < standard.trigger_font_size);
        assert!(!dense_loading.compact_trigger);
        assert_eq!(
            dense_loading.trigger_min_height,
            standard.trigger_min_height
        );
    }

    #[test]
    fn selected_value_label_uses_option_label() {
        let model = default_native_select_model().with_selected_value("bevy");
        let state = model.state();
        assert_eq!(
            selected_native_select_label(&model, &state).as_deref(),
            Some("Bevy")
        );
        let nodes = native_select_render_nodes(&model, &state);
        assert!(nodes.iter().any(|node| node.part == NativeSelectPart::Value
            && node.label == "Bevy"
            && node.selected));
    }

    #[test]
    fn loading_disables_trigger_and_options() {
        let model = default_native_select_model().loading();
        let nodes = native_select_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == NativeSelectPart::Trigger && node.disabled)
        );
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == NativeSelectPart::Option)
                .all(|node| node.disabled)
        );
    }
}
