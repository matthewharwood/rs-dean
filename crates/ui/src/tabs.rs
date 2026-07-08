use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::dom::ui_dom_id;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TabsDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

impl TabsDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl TabsOrientation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TabsPart {
    Root,
    List,
    Trigger,
    Content,
}

impl TabsPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::List, Self::Trigger, Self::Content];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Tabs",
            Self::List => "TabsList",
            Self::Trigger => "TabsTrigger",
            Self::Content => "TabsContent",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct TabsItem {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 1_000))]
    pub content: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct TabsModel {
    #[garde(skip)]
    pub density: TabsDensity,
    #[garde(skip)]
    pub orientation: TabsOrientation,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 12), dive, custom(tabs_items_are_valid))]
    pub items: Vec<TabsItem>,
    #[garde(custom(tabs_selected_value_references_enabled_item(&self.items)))]
    pub selected_value: Option<String>,
    #[garde(custom(validate_optional_tabs_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabsState {
    selected_value: String,
    focused_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabsIntent {
    Select(String),
    Focus(String),
    Blur,
    Reset(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabsChange {
    Selected(String),
    Focused(String),
    Blurred,
    Reset(String),
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabsRenderNode {
    pub part: TabsPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: TabsDensity,
    pub orientation: TabsOrientation,
    pub selected: bool,
    pub focused: bool,
    pub visible: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
    pub actionable: bool,
}

impl TabsItem {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            content: content.into(),
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl TabsModel {
    pub fn new(items: Vec<TabsItem>) -> Self {
        Self {
            density: TabsDensity::Standard,
            orientation: TabsOrientation::Horizontal,
            label: "Component tabs".to_owned(),
            items,
            selected_value: None,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: TabsDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_orientation(mut self, orientation: TabsOrientation) -> Self {
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

    pub fn state(&self) -> TabsState {
        TabsState::new(self.initial_selected_value())
    }

    fn initial_selected_value(&self) -> String {
        self.selected_value.clone().unwrap_or_else(|| {
            self.items
                .iter()
                .find(|item| !item.disabled)
                .or_else(|| self.items.first())
                .map(|item| item.value.clone())
                .unwrap_or_default()
        })
    }
}

impl TabsState {
    pub fn new(selected_value: String) -> Self {
        Self {
            selected_value,
            focused_value: None,
        }
    }

    pub fn selected_value(&self) -> &str {
        &self.selected_value
    }

    pub fn focused_value(&self) -> Option<&str> {
        self.focused_value.as_deref()
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value == value
    }

    pub fn is_focused(&self, value: &str) -> bool {
        self.focused_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: TabsIntent) -> TabsChange {
        match intent {
            TabsIntent::Select(value) => {
                if value.is_empty() || self.selected_value == value {
                    TabsChange::Unchanged
                } else {
                    self.selected_value = value.clone();
                    TabsChange::Selected(value)
                }
            }
            TabsIntent::Focus(value) => {
                if value.is_empty() || self.focused_value.as_ref() == Some(&value) {
                    TabsChange::Unchanged
                } else {
                    self.focused_value = Some(value.clone());
                    TabsChange::Focused(value)
                }
            }
            TabsIntent::Blur => {
                if self.focused_value.is_none() {
                    TabsChange::Unchanged
                } else {
                    self.focused_value = None;
                    TabsChange::Blurred
                }
            }
            TabsIntent::Reset(value) => {
                if self.selected_value == value && self.focused_value.is_none() {
                    TabsChange::Unchanged
                } else {
                    self.selected_value = value.clone();
                    self.focused_value = None;
                    TabsChange::Reset(value)
                }
            }
        }
    }
}

pub fn validate_tabs_model(model: &TabsModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn tabs_render_nodes(model: &TabsModel, state: &TabsState) -> Vec<TabsRenderNode> {
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let selected_label = model
        .items
        .iter()
        .find(|item| state.is_selected(&item.value))
        .map(|item| item.label.clone())
        .unwrap_or_else(|| "No selected tab".to_owned());
    let mut nodes = Vec::with_capacity(model.items.len().saturating_mul(2).saturating_add(2));
    nodes.push(TabsRenderNode {
        part: TabsPart::Root,
        index: 0,
        value: state.selected_value().to_owned(),
        label: model.label.clone(),
        detail: format!("Selected tab: {selected_label}"),
        density: model.density,
        orientation: model.orientation,
        selected: true,
        focused: false,
        visible: true,
        invalid,
        loading: model.loading,
        disabled: blocked,
        actionable: false,
    });
    nodes.push(TabsRenderNode {
        part: TabsPart::List,
        index: 0,
        value: "list".to_owned(),
        label: "Tabs list".to_owned(),
        detail: format!("{} tab triggers", model.items.len()),
        density: model.density,
        orientation: model.orientation,
        selected: false,
        focused: false,
        visible: true,
        invalid,
        loading: model.loading,
        disabled: blocked,
        actionable: false,
    });
    for (index, item) in model.items.iter().enumerate() {
        let item_disabled = blocked || item.disabled;
        let selected = state.is_selected(&item.value);
        let focused = state.is_focused(&item.value);
        nodes.push(TabsRenderNode {
            part: TabsPart::Trigger,
            index,
            value: item.value.clone(),
            label: item.label.clone(),
            detail: if selected {
                "Selected tab trigger".to_owned()
            } else {
                "Tab trigger".to_owned()
            },
            density: model.density,
            orientation: model.orientation,
            selected,
            focused,
            visible: true,
            invalid,
            loading: model.loading,
            disabled: item_disabled,
            actionable: !item_disabled,
        });
        nodes.push(TabsRenderNode {
            part: TabsPart::Content,
            index,
            value: item.value.clone(),
            label: item.label.clone(),
            detail: model.error.clone().unwrap_or_else(|| item.content.clone()),
            density: model.density,
            orientation: model.orientation,
            selected,
            focused,
            visible: selected,
            invalid,
            loading: model.loading,
            disabled: item_disabled || !selected,
            actionable: false,
        });
    }
    nodes
}

pub fn default_tabs_items() -> Vec<TabsItem> {
    vec![
        TabsItem::new(
            "Leptos",
            "leptos",
            "Token-only DOM components render from shared Rust models.",
        ),
        TabsItem::new(
            "Bevy",
            "bevy",
            "Bevy primitives derive from the same typed render nodes without Leptos.",
        ),
        TabsItem::new(
            "State",
            "state",
            "Durable tab preferences stay app-owned when consumers choose to persist them.",
        ),
    ]
}

pub fn default_tabs_model() -> TabsModel {
    TabsModel::new(default_tabs_items())
        .with_label("Shared framework tabs")
        .with_selected_value("leptos")
}

pub fn tabs_dom_id(prefix: &str, value: &str) -> String {
    ui_dom_id(prefix, value)
}

fn tabs_items_are_valid(items: &Vec<TabsItem>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(items.len());
    for item in items {
        if !values.insert(item.value.as_str()) {
            return Err(garde::Error::new("tabs item values must be unique"));
        }
    }
    Ok(())
}

fn tabs_selected_value_references_enabled_item<'a>(
    items: &'a [TabsItem],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |selected_value, _context| {
        if let Some(selected_value) = selected_value
            && !items
                .iter()
                .any(|item| &item.value == selected_value && !item.disabled)
        {
            return Err(garde::Error::new(
                "selected tabs value must reference an enabled item",
            ));
        }
        Ok(())
    }
}

fn validate_optional_tabs_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new("tabs copy must be 1 to 240 characters"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_tabs_model(&default_tabs_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_items() {
        let model = TabsModel::new(Vec::new());
        assert!(validate_tabs_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_values() {
        let model = TabsModel::new(vec![
            TabsItem::new("One", "same", "First panel"),
            TabsItem::new("Two", "same", "Second panel"),
        ]);
        assert!(validate_tabs_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_selected_value() {
        let model = TabsModel::new(vec![
            TabsItem::new("One", "one", "First panel"),
            TabsItem::new("Two", "two", "Second panel").disabled(),
        ])
        .with_selected_value("two");
        assert!(validate_tabs_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_tabs_model().with_error("");
        assert!(validate_tabs_model(&model).is_err());
    }

    #[test]
    fn state_selects_focuses_blurs_and_resets() {
        let mut state = TabsState::new("leptos".to_owned());
        assert!(state.is_selected("leptos"));
        assert_eq!(
            state.apply(TabsIntent::Select("bevy".to_owned())),
            TabsChange::Selected("bevy".to_owned())
        );
        assert!(state.is_selected("bevy"));
        assert_eq!(
            state.apply(TabsIntent::Focus("state".to_owned())),
            TabsChange::Focused("state".to_owned())
        );
        assert!(state.is_focused("state"));
        assert_eq!(state.apply(TabsIntent::Blur), TabsChange::Blurred);
        assert_eq!(
            state.apply(TabsIntent::Reset("leptos".to_owned())),
            TabsChange::Reset("leptos".to_owned())
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_tabs_model();
        let nodes = tabs_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(TabsPart::Root));
        for part in TabsPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn only_selected_content_is_visible() {
        let model = default_tabs_model();
        let nodes = tabs_render_nodes(&model, &TabsState::new("bevy".to_owned()));
        assert!(
            nodes
                .iter()
                .any(|node| node.part == TabsPart::Content && node.value == "bevy" && node.visible)
        );
        assert!(
            nodes.iter().any(|node| node.part == TabsPart::Content
                && node.value == "leptos"
                && !node.visible)
        );
    }

    #[test]
    fn dom_ids_are_stable_and_ascii() {
        assert_eq!(
            tabs_dom_id("tabs-panel", "Billing & Plans"),
            "tabs-panel-billing-plans"
        );
    }
}
