use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComboboxDensity {
    Standard,
    Dense,
}

impl ComboboxDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl ComboboxPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Input,
        Self::List,
        Self::Option,
        Self::Empty,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Combobox",
            Self::Input => "ComboboxInput",
            Self::List => "ComboboxList",
            Self::Option => "ComboboxOption",
            Self::Empty => "ComboboxEmpty",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ComboboxOption {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub keywords: Vec<String>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ComboboxModel {
    #[garde(skip)]
    pub density: ComboboxDensity,
    #[garde(length(min = 1, max = 128))]
    pub placeholder: String,
    #[garde(length(min = 1, max = 160))]
    pub empty_label: String,
    #[garde(length(min = 1, max = 24), dive, custom(combobox_options_are_valid))]
    pub options: Vec<ComboboxOption>,
    #[garde(custom(selected_combobox_value_references_option(&self.options)))]
    pub selected_value: Option<String>,
    #[garde(length(max = 128))]
    pub default_query: String,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboboxState {
    open: bool,
    query: String,
    selected_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComboboxIntent {
    Open,
    Close,
    Toggle,
    Input(String),
    Select(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComboboxChange {
    Opened,
    Closed,
    Input(String),
    Selected(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComboboxPart {
    Root,
    Input,
    List,
    Option,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboboxRenderNode {
    pub part: ComboboxPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: ComboboxDensity,
    pub selected: bool,
    pub visible: bool,
    pub open: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComboboxLayoutMetrics {
    pub max_width: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub input_min_height: f32,
    pub input_padding_inline: f32,
    pub input_padding_block: f32,
    pub input_font_size: f32,
    pub input_line_height: f32,
    pub list_max_height: f32,
    pub list_padding: f32,
    pub list_gap: f32,
    pub option_min_height: f32,
    pub option_padding_inline: f32,
    pub option_padding_block: f32,
    pub option_gap: f32,
    pub option_font_size: f32,
    pub option_line_height: f32,
    pub meta_font_size: f32,
    pub meta_line_height: f32,
    pub empty_padding: f32,
    pub empty_font_size: f32,
    pub empty_line_height: f32,
}

impl ComboboxOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            keywords: Vec::new(),
            disabled: false,
        }
    }

    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    fn matches_query(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let query = query.to_ascii_lowercase();
        self.label.to_ascii_lowercase().contains(&query)
            || self.value.to_ascii_lowercase().contains(&query)
            || self
                .keywords
                .iter()
                .any(|keyword| keyword.to_ascii_lowercase().contains(&query))
    }
}

impl ComboboxModel {
    pub fn new(options: Vec<ComboboxOption>) -> Self {
        Self {
            density: ComboboxDensity::Standard,
            placeholder: "Search option".to_owned(),
            empty_label: "No option found.".to_owned(),
            options,
            selected_value: None,
            default_query: String::new(),
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: ComboboxDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_empty_label(mut self, empty_label: impl Into<String>) -> Self {
        self.empty_label = empty_label.into();
        self
    }

    pub fn with_selected_value(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    pub fn with_default_query(mut self, query: impl Into<String>) -> Self {
        self.default_query = query.into();
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

    pub fn state(&self) -> ComboboxState {
        ComboboxState::new(
            false,
            self.default_query.clone(),
            self.selected_value.clone(),
        )
    }
}

impl ComboboxState {
    pub fn new(open: bool, query: String, selected_value: Option<String>) -> Self {
        Self {
            open,
            query,
            selected_value,
        }
    }

    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: ComboboxIntent) -> ComboboxChange {
        match intent {
            ComboboxIntent::Open => self.open(),
            ComboboxIntent::Close => self.close(),
            ComboboxIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            ComboboxIntent::Input(query) => self.input(query),
            ComboboxIntent::Select(value) => self.select(value),
            ComboboxIntent::Clear => self.clear(),
        }
    }

    fn open(&mut self) -> ComboboxChange {
        if self.open {
            ComboboxChange::Unchanged
        } else {
            self.open = true;
            ComboboxChange::Opened
        }
    }

    fn close(&mut self) -> ComboboxChange {
        if self.open {
            self.open = false;
            ComboboxChange::Closed
        } else {
            ComboboxChange::Unchanged
        }
    }

    fn input(&mut self, query: String) -> ComboboxChange {
        if self.query == query {
            ComboboxChange::Unchanged
        } else {
            self.query = query.clone();
            self.open = true;
            ComboboxChange::Input(query)
        }
    }

    fn select(&mut self, value: String) -> ComboboxChange {
        if value.is_empty() || self.selected_value.as_ref() == Some(&value) {
            return ComboboxChange::Unchanged;
        }
        self.selected_value = Some(value.clone());
        self.open = false;
        ComboboxChange::Selected(value)
    }

    fn clear(&mut self) -> ComboboxChange {
        if self.selected_value.take().is_some() {
            self.query.clear();
            self.open = false;
            ComboboxChange::Cleared
        } else {
            ComboboxChange::Unchanged
        }
    }
}

pub fn validate_combobox_model(model: &ComboboxModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn combobox_layout_metrics(
    density: ComboboxDensity,
    disabled: bool,
    inline_size: f32,
) -> ComboboxLayoutMetrics {
    let dense_root = density == ComboboxDensity::Dense && !disabled;
    let dense_input = density == ComboboxDensity::Dense;
    ComboboxLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_padding: if dense_root {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        root_gap: scale::space::xs2(inline_size),
        input_min_height: if dense_input {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        input_padding_inline: if dense_input {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        input_padding_block: if dense_input {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        input_font_size: if dense_input {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        input_line_height: scale::line_height::LH0,
        list_max_height: scale::space::xl4(inline_size),
        list_padding: scale::space::xs2(inline_size),
        list_gap: scale::space::xs2(inline_size),
        option_min_height: scale::space::FIELD,
        option_padding_inline: scale::space::xs(inline_size),
        option_padding_block: scale::space::xs2(inline_size),
        option_gap: scale::space::xs(inline_size),
        option_font_size: scale::font_size::f0(inline_size),
        option_line_height: scale::line_height::LH0,
        meta_font_size: scale::font_size::f00(inline_size),
        meta_line_height: scale::line_height::LH00,
        empty_padding: scale::space::xs(inline_size),
        empty_font_size: scale::font_size::f0(inline_size),
        empty_line_height: scale::line_height::LH0,
    }
}

pub fn combobox_render_nodes(
    model: &ComboboxModel,
    state: &ComboboxState,
) -> Vec<ComboboxRenderNode> {
    let blocked = model.disabled || model.loading;
    let visible_options = filtered_combobox_options(model, state.query());
    let selected = state.selected_value().unwrap_or("none").to_owned();
    let mut nodes = Vec::with_capacity(visible_options.len().saturating_add(4));
    nodes.push(ComboboxRenderNode {
        part: ComboboxPart::Root,
        index: 0,
        value: selected.clone(),
        label: "Combobox".to_owned(),
        detail: if state.is_open() {
            "Combobox open".to_owned()
        } else {
            "Combobox closed".to_owned()
        },
        density: model.density,
        selected: selected != "none",
        visible: true,
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(ComboboxRenderNode {
        part: ComboboxPart::Input,
        index: 0,
        value: state.query().to_owned(),
        label: model.placeholder.clone(),
        detail: selected_combobox_label(model, state)
            .unwrap_or_else(|| "No option selected".to_owned()),
        density: model.density,
        selected: selected != "none",
        visible: true,
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(ComboboxRenderNode {
        part: ComboboxPart::List,
        index: 0,
        value: "options".to_owned(),
        label: "Combobox options".to_owned(),
        detail: format!("{} visible options", visible_options.len()),
        density: model.density,
        selected: false,
        visible: state.is_open(),
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked,
    });
    for (index, option) in visible_options.iter().enumerate() {
        let selected = state.is_selected(&option.value);
        nodes.push(ComboboxRenderNode {
            part: ComboboxPart::Option,
            index,
            value: option.value.clone(),
            label: option.label.clone(),
            detail: if selected {
                "Selected option".to_owned()
            } else {
                "Combobox option".to_owned()
            },
            density: model.density,
            selected,
            visible: state.is_open(),
            open: state.is_open(),
            loading: model.loading,
            disabled: blocked || option.disabled,
        });
    }
    nodes.push(ComboboxRenderNode {
        part: ComboboxPart::Empty,
        index: 0,
        value: "empty".to_owned(),
        label: model.empty_label.clone(),
        detail: state.query().to_owned(),
        density: model.density,
        selected: false,
        visible: state.is_open() && visible_options.is_empty(),
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked || !visible_options.is_empty(),
    });
    nodes
}

pub fn filtered_combobox_options<'a>(
    model: &'a ComboboxModel,
    query: &str,
) -> Vec<&'a ComboboxOption> {
    model
        .options
        .iter()
        .filter(|option| option.matches_query(query))
        .collect()
}

pub fn selected_combobox_label(model: &ComboboxModel, state: &ComboboxState) -> Option<String> {
    state.selected_value().and_then(|selected| {
        model
            .options
            .iter()
            .find(|option| option.value == selected)
            .map(|option| option.label.clone())
    })
}

pub fn default_combobox_model() -> ComboboxModel {
    ComboboxModel::new(vec![
        ComboboxOption::new("Leptos", "leptos")
            .with_keywords(vec!["dom".to_owned(), "wasm".to_owned()]),
        ComboboxOption::new("Bevy", "bevy")
            .with_keywords(vec!["scene".to_owned(), "webgpu".to_owned()]),
        ComboboxOption::new("Shared state", "state")
            .with_keywords(vec!["idb".to_owned(), "durable".to_owned()]),
    ])
    .with_placeholder("Search stack")
    .with_selected_value("leptos")
}

fn combobox_options_are_valid(options: &Vec<ComboboxOption>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(options.len());
    for option in options {
        if !values.insert(option.value.as_str()) {
            return Err(garde::Error::new("combobox option values must be unique"));
        }
        if option.keywords.len() > 8 {
            return Err(garde::Error::new(
                "combobox options may include at most 8 keywords",
            ));
        }
        if option
            .keywords
            .iter()
            .any(|keyword| !(1..=64).contains(&keyword.chars().count()))
        {
            return Err(garde::Error::new(
                "combobox option keywords must be 1 to 64 characters",
            ));
        }
    }
    Ok(())
}

fn selected_combobox_value_references_option(
    options: &[ComboboxOption],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + '_ {
    move |selected, _context| {
        if let Some(selected) = selected
            && !options
                .iter()
                .any(|option| option.value == *selected && !option.disabled)
        {
            return Err(garde::Error::new(
                "selected combobox value must reference an enabled option",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_combobox_model(&default_combobox_model()).is_ok());
    }

    #[test]
    fn layout_metrics_share_fluid_tailwind_tokens() {
        let compact = combobox_layout_metrics(ComboboxDensity::Standard, false, 320.0);
        let wide = combobox_layout_metrics(ComboboxDensity::Standard, false, 1_000.0);
        let dense = combobox_layout_metrics(ComboboxDensity::Dense, false, 1_000.0);
        let disabled_dense = combobox_layout_metrics(ComboboxDensity::Dense, true, 1_000.0);

        assert!(compact.root_padding < wide.root_padding);
        assert!(dense.root_padding < wide.root_padding);
        assert_eq!(disabled_dense.root_padding, wide.root_padding);
        assert!(dense.input_font_size < wide.input_font_size);
        assert_eq!(wide.max_width, scale::container::CONTROL);
    }

    #[test]
    fn garde_rejects_empty_options() {
        let model = ComboboxModel::new(Vec::new());
        assert!(validate_combobox_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_option_values() {
        let model = ComboboxModel::new(vec![
            ComboboxOption::new("Leptos", "stack"),
            ComboboxOption::new("Bevy", "stack"),
        ]);
        assert!(validate_combobox_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_option_label() {
        let model = ComboboxModel::new(vec![ComboboxOption::new("", "leptos")]);
        assert!(validate_combobox_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_value() {
        let model = default_combobox_model().with_selected_value("unknown");
        assert!(validate_combobox_model(&model).is_err());
    }

    #[test]
    fn state_filters_selects_and_clears() {
        let mut state = ComboboxState::new(false, String::new(), None);
        assert_eq!(
            state.apply(ComboboxIntent::Input("bev".to_owned())),
            ComboboxChange::Input("bev".to_owned())
        );
        assert!(state.is_open());
        assert_eq!(state.query(), "bev");
        assert_eq!(
            state.apply(ComboboxIntent::Select("bevy".to_owned())),
            ComboboxChange::Selected("bevy".to_owned())
        );
        assert_eq!(state.selected_value(), Some("bevy"));
        assert!(!state.is_open());
        assert_eq!(state.apply(ComboboxIntent::Clear), ComboboxChange::Cleared);
        assert_eq!(state.selected_value(), None);
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_combobox_model();
        let nodes = combobox_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(ComboboxPart::Root)
        );
        for part in ComboboxPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == ComboboxPart::Option)
                .count(),
            3
        );
    }

    #[test]
    fn empty_node_visible_when_filter_has_no_matches() {
        let model = default_combobox_model().with_default_query("zzzz");
        let mut state = model.state();
        let _ = state.apply(ComboboxIntent::Open);
        let nodes = combobox_render_nodes(&model, &state);
        assert!(
            nodes
                .iter()
                .any(|node| node.part == ComboboxPart::Empty && node.visible)
        );
        assert!(
            !nodes
                .iter()
                .any(|node| node.part == ComboboxPart::Option && node.visible)
        );
    }

    #[test]
    fn loading_disables_option_nodes() {
        let model = default_combobox_model().loading();
        let mut state = model.state();
        let _ = state.apply(ComboboxIntent::Open);
        let nodes = combobox_render_nodes(&model, &state);
        assert!(
            nodes
                .iter()
                .any(|node| node.part == ComboboxPart::Option && node.disabled)
        );
    }
}
