use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{ButtonSize, ButtonVariant};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ButtonGroupOrientation {
    Horizontal,
    Vertical,
}

impl ButtonGroupOrientation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

impl ButtonGroupPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "ButtonGroup",
            Self::Item => "ButtonGroupItem",
            Self::Separator => "ButtonGroupSeparator",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ButtonGroupItem {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(custom(validate_optional_button_group_icon))]
    pub icon: Option<String>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ButtonGroupModel {
    #[garde(skip)]
    pub variant: ButtonVariant,
    #[garde(skip)]
    pub size: ButtonSize,
    #[garde(skip)]
    pub orientation: ButtonGroupOrientation,
    #[garde(length(min = 1, max = 8), dive, custom(button_group_items_match_size(&self.size)))]
    pub items: Vec<ButtonGroupItem>,
    #[garde(custom(selected_button_group_value_references_item(&self.items)))]
    pub selected: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonGroupState {
    selected: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonGroupIntent {
    Select(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonGroupChange {
    Selected(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonGroupPart {
    Root,
    Item,
    Separator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonGroupRenderNode {
    pub part: ButtonGroupPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub icon: Option<String>,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub orientation: ButtonGroupOrientation,
    pub selected: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl ButtonGroupItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            icon: None,
            disabled: false,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl ButtonGroupModel {
    pub fn new(items: Vec<ButtonGroupItem>) -> Self {
        Self {
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Medium,
            orientation: ButtonGroupOrientation::Horizontal,
            items,
            selected: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub const fn with_orientation(mut self, orientation: ButtonGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_selected(mut self, selected: impl Into<String>) -> Self {
        self.selected = Some(selected.into());
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

    pub fn state(&self) -> ButtonGroupState {
        let selected = self.selected.clone().or_else(|| {
            self.items
                .iter()
                .find(|item| !item.disabled)
                .map(|item| item.value.clone())
        });
        ButtonGroupState { selected }
    }
}

impl ButtonGroupState {
    pub const fn empty() -> Self {
        Self { selected: None }
    }

    pub fn selected(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: ButtonGroupIntent) -> ButtonGroupChange {
        match intent {
            ButtonGroupIntent::Select(value) => self.select(value),
            ButtonGroupIntent::Clear => self.clear(),
        }
    }

    fn select(&mut self, value: String) -> ButtonGroupChange {
        if value.is_empty() || self.selected.as_ref() == Some(&value) {
            ButtonGroupChange::Unchanged
        } else {
            self.selected = Some(value.clone());
            ButtonGroupChange::Selected(value)
        }
    }

    fn clear(&mut self) -> ButtonGroupChange {
        if self.selected.take().is_some() {
            ButtonGroupChange::Cleared
        } else {
            ButtonGroupChange::Unchanged
        }
    }
}

pub fn validate_button_group_model(model: &ButtonGroupModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn button_group_render_nodes(
    model: &ButtonGroupModel,
    state: &ButtonGroupState,
) -> Vec<ButtonGroupRenderNode> {
    let mut nodes = Vec::with_capacity(model.items.len().saturating_mul(2));
    let selected = state.selected().unwrap_or("none").to_owned();
    nodes.push(ButtonGroupRenderNode {
        part: ButtonGroupPart::Root,
        index: 0,
        value: selected.clone(),
        label: "Button group".to_owned(),
        detail: format!(
            "{} {} group",
            model.orientation.label(),
            model.variant.label()
        ),
        icon: None,
        variant: model.variant,
        size: model.size,
        orientation: model.orientation,
        selected: selected != "none",
        loading: model.loading,
        disabled: model.disabled || model.loading,
    });

    for (index, item) in model.items.iter().enumerate() {
        let selected = state.is_selected(&item.value);
        nodes.push(ButtonGroupRenderNode {
            part: ButtonGroupPart::Item,
            index,
            value: item.value.clone(),
            label: item.label.clone(),
            detail: if selected {
                "Selected grouped action".to_owned()
            } else {
                "Grouped action".to_owned()
            },
            icon: item.icon.clone(),
            variant: model.variant,
            size: model.size,
            orientation: model.orientation,
            selected,
            loading: model.loading,
            disabled: model.disabled || model.loading || item.disabled,
        });
        if index + 1 < model.items.len() {
            nodes.push(ButtonGroupRenderNode {
                part: ButtonGroupPart::Separator,
                index,
                value: format!("separator-{index}"),
                label: "Separator".to_owned(),
                detail: "Button group separator".to_owned(),
                icon: None,
                variant: model.variant,
                size: model.size,
                orientation: model.orientation,
                selected: false,
                loading: model.loading,
                disabled: model.disabled || model.loading,
            });
        }
    }

    nodes
}

pub fn default_button_group_model() -> ButtonGroupModel {
    ButtonGroupModel::new(vec![
        ButtonGroupItem::new("Day", "day").with_icon("D"),
        ButtonGroupItem::new("Week", "week").with_icon("W"),
        ButtonGroupItem::new("Month", "month").with_icon("M"),
    ])
    .with_selected("day")
}

fn validate_optional_button_group_icon(icon: &Option<String>, _context: &()) -> garde::Result {
    if let Some(icon) = icon
        && !(1..=16).contains(&icon.chars().count())
    {
        return Err(garde::Error::new(
            "button group icon must be 1 to 16 characters",
        ));
    }
    Ok(())
}

fn button_group_items_match_size(
    size: &ButtonSize,
) -> impl FnOnce(&Vec<ButtonGroupItem>, &()) -> garde::Result + '_ {
    move |items, _context| {
        let mut values = HashSet::with_capacity(items.len());
        for item in items {
            if !values.insert(item.value.as_str()) {
                return Err(garde::Error::new("button group item values must be unique"));
            }
            if *size == ButtonSize::Icon && item.icon.is_none() {
                return Err(garde::Error::new(
                    "icon-sized button group items must include icons",
                ));
            }
        }
        Ok(())
    }
}

fn selected_button_group_value_references_item(
    items: &[ButtonGroupItem],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + '_ {
    move |selected, _context| {
        if let Some(selected) = selected
            && !items
                .iter()
                .any(|item| item.value == *selected && !item.disabled)
        {
            return Err(garde::Error::new(
                "selected button group value must reference an enabled item",
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
        assert!(validate_button_group_model(&default_button_group_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_items() {
        let model = ButtonGroupModel::new(Vec::new());
        assert!(validate_button_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_item_values() {
        let model = ButtonGroupModel::new(vec![
            ButtonGroupItem::new("Day", "same"),
            ButtonGroupItem::new("Week", "same"),
        ]);
        assert!(validate_button_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_value() {
        let model = default_button_group_model().with_selected("year");
        assert!(validate_button_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_icon_size_without_icons() {
        let model = ButtonGroupModel::new(vec![ButtonGroupItem::new("Day", "day")])
            .with_size(ButtonSize::Icon);
        assert!(validate_button_group_model(&model).is_err());
    }

    #[test]
    fn local_state_selects_and_clears_items() {
        let mut state = ButtonGroupState::empty();
        assert_eq!(
            state.apply(ButtonGroupIntent::Select("day".to_owned())),
            ButtonGroupChange::Selected("day".to_owned())
        );
        assert!(state.is_selected("day"));
        assert_eq!(
            state.apply(ButtonGroupIntent::Clear),
            ButtonGroupChange::Cleared
        );
        assert!(state.selected().is_none());
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_button_group_model();
        let nodes = button_group_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(ButtonGroupPart::Root)
        );
        assert!(nodes.iter().any(|node| node.part == ButtonGroupPart::Item));
        assert!(
            nodes
                .iter()
                .any(|node| node.part == ButtonGroupPart::Separator)
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == ButtonGroupPart::Item)
                .count(),
            3
        );
    }
}
