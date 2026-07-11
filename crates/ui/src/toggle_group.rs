use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{
    ToggleControlLayoutMetrics, ToggleDensity, TogglePressed, ToggleVariant, scale,
    toggle_control_layout_metrics,
};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToggleGroupSelectionMode {
    Single,
    Multiple,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToggleGroupOrientation {
    Horizontal,
    Vertical,
}

impl ToggleGroupSelectionMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

impl ToggleGroupOrientation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

impl ToggleGroupPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Item, Self::Indicator];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "ToggleGroup",
            Self::Item => "ToggleGroupItem",
            Self::Indicator => "ToggleGroupIndicator",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ToggleGroupItem {
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
pub struct ToggleGroupModel {
    #[garde(skip)]
    pub density: ToggleDensity,
    #[garde(skip)]
    pub variant: ToggleVariant,
    #[garde(skip)]
    pub selection_mode: ToggleGroupSelectionMode,
    #[garde(skip)]
    pub orientation: ToggleGroupOrientation,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 12), dive, custom(toggle_group_items_are_unique))]
    pub items: Vec<ToggleGroupItem>,
    #[garde(custom(toggle_group_selected_values_reference_enabled_items(&self.items, &self.selection_mode)))]
    pub selected_values: Vec<String>,
    #[garde(custom(validate_optional_toggle_group_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToggleGroupState {
    focused_value: Option<String>,
    selected_values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleGroupIntent {
    Toggle(String),
    Select(String),
    Deselect(String),
    Focus(String),
    Blur,
    Clear,
    Reset(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleGroupChange {
    Selected(String),
    Deselected(String),
    Focused(String),
    Blurred,
    Cleared,
    Reset(Vec<String>),
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToggleGroupPart {
    Root,
    Item,
    Indicator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToggleGroupRenderNode {
    pub part: ToggleGroupPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: ToggleDensity,
    pub variant: ToggleVariant,
    pub selection_mode: ToggleGroupSelectionMode,
    pub orientation: ToggleGroupOrientation,
    pub pressed: TogglePressed,
    pub focused: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleGroupLayoutMetrics {
    pub width: f32,
    pub root_gap: f32,
    pub root_padding: f32,
    pub root_is_dense: bool,
    pub header_gap: f32,
    pub title_font_size: f32,
    pub title_line_height: f32,
    pub status_width: f32,
    pub status_height: f32,
    pub status_padding_inline: f32,
    pub status_padding_block: f32,
    pub status_font_size: f32,
    pub status_line_height: f32,
    pub status_letter_spacing: f32,
    pub list_gap: f32,
    pub horizontal: bool,
    pub items: Vec<ToggleControlLayoutMetrics>,
    pub detail_font_size: f32,
    pub detail_line_height: f32,
    pub error_padding: f32,
}

impl ToggleGroupItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            detail: format!("{label} toggle"),
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

impl ToggleGroupModel {
    pub fn new(items: Vec<ToggleGroupItem>) -> Self {
        Self {
            density: ToggleDensity::Standard,
            variant: ToggleVariant::Default,
            selection_mode: ToggleGroupSelectionMode::Single,
            orientation: ToggleGroupOrientation::Horizontal,
            label: "Text alignment".to_owned(),
            items,
            selected_values: Vec::new(),
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

    pub const fn with_selection_mode(mut self, selection_mode: ToggleGroupSelectionMode) -> Self {
        self.selection_mode = selection_mode;
        self
    }

    pub const fn with_orientation(mut self, orientation: ToggleGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_selected_value(mut self, value: impl Into<String>) -> Self {
        self.selected_values = vec![value.into()];
        self
    }

    pub fn with_selected_values(mut self, values: Vec<String>) -> Self {
        self.selected_values = values;
        self
    }

    pub fn without_selected_values(mut self) -> Self {
        self.selected_values.clear();
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

    pub fn state(&self) -> ToggleGroupState {
        ToggleGroupState::new(self.selected_values.clone())
    }
}

impl ToggleGroupState {
    pub fn new(selected_values: Vec<String>) -> Self {
        Self {
            focused_value: None,
            selected_values,
        }
    }

    pub fn focused_value(&self) -> Option<&str> {
        self.focused_value.as_deref()
    }

    pub fn selected_values(&self) -> &[String] {
        &self.selected_values
    }

    pub fn is_focused(&self, value: &str) -> bool {
        self.focused_value.as_deref() == Some(value)
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_values
            .iter()
            .any(|selected| selected == value)
    }

    pub fn apply(
        &mut self,
        selection_mode: ToggleGroupSelectionMode,
        intent: ToggleGroupIntent,
    ) -> ToggleGroupChange {
        match intent {
            ToggleGroupIntent::Toggle(value) => self.toggle(selection_mode, value),
            ToggleGroupIntent::Select(value) => self.select(selection_mode, value),
            ToggleGroupIntent::Deselect(value) => self.deselect(value),
            ToggleGroupIntent::Focus(value) => self.focus(value),
            ToggleGroupIntent::Blur => self.blur(),
            ToggleGroupIntent::Clear => self.clear(),
            ToggleGroupIntent::Reset(values) => self.reset(values),
        }
    }

    fn toggle(
        &mut self,
        selection_mode: ToggleGroupSelectionMode,
        value: String,
    ) -> ToggleGroupChange {
        if value.is_empty() {
            return ToggleGroupChange::Unchanged;
        }
        if self.is_selected(&value) {
            return self.deselect(value);
        }
        self.select(selection_mode, value)
    }

    fn select(
        &mut self,
        selection_mode: ToggleGroupSelectionMode,
        value: String,
    ) -> ToggleGroupChange {
        if value.is_empty() || self.is_selected(&value) {
            return ToggleGroupChange::Unchanged;
        }
        self.focused_value = Some(value.clone());
        match selection_mode {
            ToggleGroupSelectionMode::Single => {
                self.selected_values.clear();
                self.selected_values.push(value.clone());
            }
            ToggleGroupSelectionMode::Multiple => self.selected_values.push(value.clone()),
        }
        ToggleGroupChange::Selected(value)
    }

    fn deselect(&mut self, value: String) -> ToggleGroupChange {
        if let Some(index) = self
            .selected_values
            .iter()
            .position(|selected| selected == &value)
        {
            self.selected_values.remove(index);
            ToggleGroupChange::Deselected(value)
        } else {
            ToggleGroupChange::Unchanged
        }
    }

    fn focus(&mut self, value: String) -> ToggleGroupChange {
        if value.is_empty() || self.focused_value.as_ref() == Some(&value) {
            return ToggleGroupChange::Unchanged;
        }
        self.focused_value = Some(value.clone());
        ToggleGroupChange::Focused(value)
    }

    fn blur(&mut self) -> ToggleGroupChange {
        if self.focused_value.take().is_some() {
            ToggleGroupChange::Blurred
        } else {
            ToggleGroupChange::Unchanged
        }
    }

    fn clear(&mut self) -> ToggleGroupChange {
        if self.focused_value.is_none() && self.selected_values.is_empty() {
            return ToggleGroupChange::Unchanged;
        }
        self.focused_value = None;
        self.selected_values.clear();
        ToggleGroupChange::Cleared
    }

    fn reset(&mut self, values: Vec<String>) -> ToggleGroupChange {
        if self.focused_value.is_none() && self.selected_values == values {
            return ToggleGroupChange::Unchanged;
        }
        self.focused_value = None;
        self.selected_values = values.clone();
        ToggleGroupChange::Reset(values)
    }
}

pub fn validate_toggle_group_model(model: &ToggleGroupModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn toggle_group_render_nodes(
    model: &ToggleGroupModel,
    state: &ToggleGroupState,
) -> Vec<ToggleGroupRenderNode> {
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let selected_value = toggle_group_selected_values_label(state.selected_values());
    let mut nodes = Vec::with_capacity(model.items.len().saturating_mul(2).saturating_add(1));
    nodes.push(ToggleGroupRenderNode {
        part: ToggleGroupPart::Root,
        index: 0,
        value: selected_value.clone(),
        label: model.label.clone(),
        detail: model.error.clone().unwrap_or_else(|| {
            format!(
                "{} {} toggle group",
                model.selection_mode.label(),
                model.orientation.label()
            )
        }),
        density: model.density,
        variant: model.variant,
        selection_mode: model.selection_mode,
        orientation: model.orientation,
        pressed: if state.selected_values().is_empty() {
            TogglePressed::Unpressed
        } else {
            TogglePressed::Pressed
        },
        focused: state.focused_value().is_some(),
        actionable: false,
        invalid,
        loading: model.loading,
        disabled: model.disabled,
    });

    for (index, item) in model.items.iter().enumerate() {
        let selected = state.is_selected(&item.value);
        let focused = state.is_focused(&item.value);
        let disabled = blocked || item.disabled;
        let pressed = if selected {
            TogglePressed::Pressed
        } else {
            TogglePressed::Unpressed
        };
        nodes.push(ToggleGroupRenderNode {
            part: ToggleGroupPart::Item,
            index,
            value: item.value.clone(),
            label: item.label.clone(),
            detail: item.detail.clone(),
            density: model.density,
            variant: model.variant,
            selection_mode: model.selection_mode,
            orientation: model.orientation,
            pressed,
            focused,
            actionable: !disabled,
            invalid,
            loading: model.loading,
            disabled,
        });
        nodes.push(ToggleGroupRenderNode {
            part: ToggleGroupPart::Indicator,
            index,
            value: item.value.clone(),
            label: pressed.label().to_owned(),
            detail: if selected {
                "Selected toggle indicator".to_owned()
            } else {
                "Unselected toggle indicator".to_owned()
            },
            density: model.density,
            variant: model.variant,
            selection_mode: model.selection_mode,
            orientation: model.orientation,
            pressed,
            focused,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled,
        });
    }

    nodes
}

pub fn toggle_group_layout_metrics(
    model: &ToggleGroupModel,
    state: &ToggleGroupState,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> ToggleGroupLayoutMetrics {
    let border_width = border_width.max(0.0);
    let invalid = model.error.is_some();
    let dense = model.density == ToggleDensity::Dense;
    let root_is_dense = dense && !invalid && !model.disabled;
    let title_is_dense = dense && !model.disabled;
    let status_font_size = scale::font_size::f00(inline_size);
    let status_padding_inline = scale::space::xs2(inline_size);
    let status_padding_block = scale::space::xs3(inline_size);
    let status_copy = toggle_group_selected_status_label(state.selected_values()).to_uppercase();
    let status_width = border_width * 2.0
        + status_padding_inline * 2.0
        + scale::estimate_inline_text_width(
            &status_copy,
            status_font_size,
            scale::letter_spacing::LABEL,
        );
    let status_height = border_width * 2.0
        + status_padding_block * 2.0
        + status_font_size * scale::line_height::LH00;
    let items = model
        .items
        .iter()
        .map(|item| {
            toggle_control_layout_metrics(
                model.density,
                &item.label,
                if state.is_selected(&item.value) {
                    "ON"
                } else {
                    "OFF"
                },
                inline_size,
                border_width,
            )
        })
        .collect();

    ToggleGroupLayoutMetrics {
        width: available_width.clamp(1.0, scale::container::CONTROL),
        root_gap: scale::space::xs2(inline_size),
        root_padding: if root_is_dense {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        root_is_dense,
        header_gap: scale::space::xs2(inline_size),
        title_font_size: if title_is_dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        title_line_height: scale::line_height::LH0,
        status_width,
        status_height,
        status_padding_inline,
        status_padding_block,
        status_font_size,
        status_line_height: scale::line_height::LH00,
        status_letter_spacing: scale::letter_spacing::LABEL,
        list_gap: scale::space::xs2(inline_size),
        horizontal: model.orientation == ToggleGroupOrientation::Horizontal,
        items,
        detail_font_size: scale::font_size::f0(inline_size),
        detail_line_height: scale::line_height::LH0,
        error_padding: scale::space::xs(inline_size),
    }
}

pub fn default_toggle_group_items() -> Vec<ToggleGroupItem> {
    vec![
        ToggleGroupItem::new("Left", "left").with_detail("Align content to the left edge."),
        ToggleGroupItem::new("Center", "center").with_detail("Center content in the container."),
        ToggleGroupItem::new("Right", "right").with_detail("Align content to the right edge."),
    ]
}

pub fn default_toggle_group_model() -> ToggleGroupModel {
    ToggleGroupModel::new(default_toggle_group_items()).with_selected_value("left")
}

pub fn toggle_group_selected_values_label(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_owned()
    } else {
        values.join(",")
    }
}

pub fn toggle_group_selected_status_label(values: &[String]) -> String {
    match values.len() {
        0 => "none".to_owned(),
        1 => "1 selected".to_owned(),
        count => format!("{count} selected"),
    }
}

fn toggle_group_items_are_unique(items: &Vec<ToggleGroupItem>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(items.len());
    for item in items {
        if !values.insert(item.value.as_str()) {
            return Err(garde::Error::new("toggle group item values must be unique"));
        }
    }
    Ok(())
}

fn toggle_group_selected_values_reference_enabled_items<'a>(
    items: &'a [ToggleGroupItem],
    selection_mode: &'a ToggleGroupSelectionMode,
) -> impl FnOnce(&Vec<String>, &()) -> garde::Result + 'a {
    move |selected_values, _context| {
        if *selection_mode == ToggleGroupSelectionMode::Single && selected_values.len() > 1 {
            return Err(garde::Error::new(
                "single toggle group can select at most one value",
            ));
        }
        let mut unique = HashSet::with_capacity(selected_values.len());
        for selected in selected_values {
            if selected.is_empty() {
                return Err(garde::Error::new(
                    "selected toggle group values cannot be empty",
                ));
            }
            if !unique.insert(selected.as_str()) {
                return Err(garde::Error::new(
                    "selected toggle group values must be unique",
                ));
            }
            if !items
                .iter()
                .any(|item| item.value == *selected && !item.disabled)
            {
                return Err(garde::Error::new(
                    "selected toggle group value must reference an enabled item",
                ));
            }
        }
        Ok(())
    }
}

fn validate_optional_toggle_group_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "toggle group error must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_toggle_group_model(&default_toggle_group_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_items() {
        let model = ToggleGroupModel::new(Vec::new());
        assert!(validate_toggle_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_item_values() {
        let model = ToggleGroupModel::new(vec![
            ToggleGroupItem::new("Left", "align"),
            ToggleGroupItem::new("Right", "align"),
        ]);
        assert!(validate_toggle_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_or_disabled_selected_values() {
        let model = default_toggle_group_model().with_selected_value("missing");
        assert!(validate_toggle_group_model(&model).is_err());

        let model = ToggleGroupModel::new(vec![
            ToggleGroupItem::new("Left", "left").disabled(),
            ToggleGroupItem::new("Right", "right"),
        ])
        .with_selected_value("left");
        assert!(validate_toggle_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_multiple_values_for_single_mode() {
        let model = default_toggle_group_model()
            .with_selected_values(vec!["left".to_owned(), "right".to_owned()]);
        assert!(validate_toggle_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_item_fields_and_error() {
        let model = ToggleGroupModel::new(vec![ToggleGroupItem::new("", "left")]);
        assert!(validate_toggle_group_model(&model).is_err());
        let model = ToggleGroupModel::new(vec![ToggleGroupItem::new("Left", "")]);
        assert!(validate_toggle_group_model(&model).is_err());
        let model = default_toggle_group_model().with_error("");
        assert!(validate_toggle_group_model(&model).is_err());
    }

    #[test]
    fn single_state_selects_one_and_allows_deselect() {
        let mut state = ToggleGroupState::new(vec!["left".to_owned()]);
        assert!(state.is_selected("left"));
        assert_eq!(
            state.apply(
                ToggleGroupSelectionMode::Single,
                ToggleGroupIntent::Toggle("right".to_owned())
            ),
            ToggleGroupChange::Selected("right".to_owned())
        );
        assert!(!state.is_selected("left"));
        assert!(state.is_selected("right"));
        assert_eq!(
            state.apply(
                ToggleGroupSelectionMode::Single,
                ToggleGroupIntent::Toggle("right".to_owned())
            ),
            ToggleGroupChange::Deselected("right".to_owned())
        );
        assert!(state.selected_values().is_empty());
    }

    #[test]
    fn multiple_state_toggles_independent_values() {
        let mut state = ToggleGroupState::new(vec!["bold".to_owned()]);
        assert_eq!(
            state.apply(
                ToggleGroupSelectionMode::Multiple,
                ToggleGroupIntent::Toggle("italic".to_owned())
            ),
            ToggleGroupChange::Selected("italic".to_owned())
        );
        assert!(state.is_selected("bold"));
        assert!(state.is_selected("italic"));
        assert_eq!(
            state.apply(
                ToggleGroupSelectionMode::Multiple,
                ToggleGroupIntent::Toggle("bold".to_owned())
            ),
            ToggleGroupChange::Deselected("bold".to_owned())
        );
        assert!(!state.is_selected("bold"));
        assert!(state.is_selected("italic"));
    }

    #[test]
    fn state_focuses_blurs_clears_and_resets() {
        let mut state = ToggleGroupState::new(vec!["left".to_owned()]);
        assert_eq!(
            state.apply(
                ToggleGroupSelectionMode::Single,
                ToggleGroupIntent::Focus("left".to_owned())
            ),
            ToggleGroupChange::Focused("left".to_owned())
        );
        assert_eq!(
            state.apply(ToggleGroupSelectionMode::Single, ToggleGroupIntent::Blur),
            ToggleGroupChange::Blurred
        );
        assert_eq!(
            state.apply(ToggleGroupSelectionMode::Single, ToggleGroupIntent::Clear),
            ToggleGroupChange::Cleared
        );
        assert_eq!(
            state.apply(
                ToggleGroupSelectionMode::Single,
                ToggleGroupIntent::Reset(vec!["right".to_owned()])
            ),
            ToggleGroupChange::Reset(vec!["right".to_owned()])
        );
        assert!(state.is_selected("right"));
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_toggle_group_model();
        let nodes = toggle_group_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(ToggleGroupPart::Root)
        );
        for part in ToggleGroupPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == ToggleGroupPart::Item)
                .count(),
            model.items.len()
        );
    }

    #[test]
    fn loading_disables_item_nodes() {
        let model = default_toggle_group_model().loading();
        let nodes = toggle_group_render_nodes(&model, &model.state());
        assert!(
            nodes.iter().any(|node| node.part == ToggleGroupPart::Item
                && node.disabled
                && !node.actionable)
        );
    }

    #[test]
    fn layout_metrics_share_toggle_control_geometry() {
        let model = default_toggle_group_model();
        let state = model.state();
        let metrics = toggle_group_layout_metrics(&model, &state, 640.0, 480.0, 1.0);
        let expected =
            toggle_control_layout_metrics(model.density, &model.items[0].label, "ON", 480.0, 1.0);

        assert_eq!(metrics.width, scale::container::CONTROL);
        assert_eq!(metrics.items[0], expected);
        assert!(metrics.horizontal);
        assert!(!metrics.root_is_dense);
    }

    #[test]
    fn dense_and_vertical_metrics_follow_dom_contract() {
        let model = default_toggle_group_model()
            .with_density(ToggleDensity::Dense)
            .with_orientation(ToggleGroupOrientation::Vertical);
        let metrics = toggle_group_layout_metrics(&model, &model.state(), 320.0, 320.0, 1.0);

        assert!(metrics.root_is_dense);
        assert!(!metrics.horizontal);
        assert_eq!(metrics.items.len(), model.items.len());
        assert_eq!(metrics.status_letter_spacing, scale::letter_spacing::LABEL);
    }

    #[test]
    fn selected_status_label_tracks_selection_count() {
        assert_eq!(toggle_group_selected_status_label(&[]), "none");
        assert_eq!(
            toggle_group_selected_status_label(&["left".to_owned()]),
            "1 selected"
        );
        assert_eq!(
            toggle_group_selected_status_label(&["bold".to_owned(), "italic".to_owned()]),
            "2 selected"
        );
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_toggle_group_model().with_error("Choose a supported alignment.");
        let nodes = toggle_group_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(nodes.iter().any(|node| node.part == ToggleGroupPart::Root
            && node.detail == "Choose a supported alignment."));
    }
}
