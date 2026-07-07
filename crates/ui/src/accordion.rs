use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::dom::ui_dom_id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccordionMode {
    Single,
    Multiple,
}

impl AccordionMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

impl AccordionPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Accordion",
            Self::Item => "AccordionItem",
            Self::Trigger => "AccordionTrigger",
            Self::Content => "AccordionContent",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AccordionItem {
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 2_000))]
    pub content: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AccordionModel {
    #[garde(skip)]
    pub mode: AccordionMode,
    #[garde(length(min = 1), dive, custom(unique_accordion_values))]
    pub items: Vec<AccordionItem>,
    #[garde(custom(default_open_references_items(&self.items)))]
    pub default_open: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccordionState {
    mode: AccordionMode,
    open: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccordionIntent {
    Toggle(String),
    Open(String),
    Close(String),
    CloseAll,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccordionChange {
    Opened(String),
    Closed(String),
    ClosedAll,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccordionPart {
    Root,
    Item,
    Trigger,
    Content,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccordionRenderNode {
    pub part: AccordionPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub open: bool,
    pub disabled: bool,
}

impl AccordionItem {
    pub fn new(
        value: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            content: content.into(),
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl AccordionModel {
    pub fn new(mode: AccordionMode, items: Vec<AccordionItem>) -> Self {
        Self {
            mode,
            items,
            default_open: Vec::new(),
        }
    }

    pub fn single(items: Vec<AccordionItem>) -> Self {
        Self::new(AccordionMode::Single, items)
    }

    pub fn multiple(items: Vec<AccordionItem>) -> Self {
        Self::new(AccordionMode::Multiple, items)
    }

    pub fn with_default_open(mut self, default_open: Vec<String>) -> Self {
        self.default_open = normalize_open(self.mode, default_open);
        self
    }

    pub fn state(&self) -> AccordionState {
        AccordionState::new(self.mode, self.default_open.clone())
    }
}

pub fn validate_accordion_model(model: &AccordionModel) -> Result<(), garde::Report> {
    model.validate()
}

impl AccordionState {
    pub fn new(mode: AccordionMode, open: Vec<String>) -> Self {
        Self {
            mode,
            open: normalize_open(mode, open),
        }
    }

    pub const fn mode(&self) -> AccordionMode {
        self.mode
    }

    pub fn open_values(&self) -> &[String] {
        &self.open
    }

    pub fn is_open(&self, value: &str) -> bool {
        self.open.iter().any(|open| open == value)
    }

    pub fn apply(&mut self, intent: AccordionIntent) -> AccordionChange {
        match intent {
            AccordionIntent::Toggle(value) => self.toggle(&value),
            AccordionIntent::Open(value) => self.open(value),
            AccordionIntent::Close(value) => self.close(&value),
            AccordionIntent::CloseAll => self.close_all(),
        }
    }

    pub fn toggle(&mut self, value: &str) -> AccordionChange {
        if self.is_open(value) {
            self.close(value)
        } else {
            self.open(value.to_owned())
        }
    }

    pub fn open(&mut self, value: String) -> AccordionChange {
        if value.is_empty() || self.is_open(&value) {
            return AccordionChange::Unchanged;
        }

        if self.mode == AccordionMode::Single {
            self.open.clear();
        }

        self.open.push(value.clone());
        AccordionChange::Opened(value)
    }

    pub fn close(&mut self, value: &str) -> AccordionChange {
        let Some(index) = self.open.iter().position(|open| open == value) else {
            return AccordionChange::Unchanged;
        };
        let closed = self.open.remove(index);
        AccordionChange::Closed(closed)
    }

    pub fn close_all(&mut self) -> AccordionChange {
        if self.open.is_empty() {
            AccordionChange::Unchanged
        } else {
            self.open.clear();
            AccordionChange::ClosedAll
        }
    }
}

pub fn accordion_render_nodes(
    model: &AccordionModel,
    state: &AccordionState,
) -> Vec<AccordionRenderNode> {
    let mut nodes = Vec::with_capacity(model.items.len().saturating_mul(3).saturating_add(1));
    nodes.push(AccordionRenderNode {
        part: AccordionPart::Root,
        value: "accordion".to_owned(),
        label: "Accordion".to_owned(),
        detail: match model.mode {
            AccordionMode::Single => "Single section disclosure".to_owned(),
            AccordionMode::Multiple => "Multiple section disclosure".to_owned(),
        },
        open: !state.open_values().is_empty(),
        disabled: false,
    });

    for item in &model.items {
        let open = state.is_open(&item.value);
        nodes.push(AccordionRenderNode {
            part: AccordionPart::Item,
            value: item.value.clone(),
            label: item.title.clone(),
            detail: "Accordion item".to_owned(),
            open,
            disabled: item.disabled,
        });
        nodes.push(AccordionRenderNode {
            part: AccordionPart::Trigger,
            value: item.value.clone(),
            label: item.title.clone(),
            detail: "Accordion trigger".to_owned(),
            open,
            disabled: item.disabled,
        });
        nodes.push(AccordionRenderNode {
            part: AccordionPart::Content,
            value: item.value.clone(),
            label: item.title.clone(),
            detail: item.content.clone(),
            open,
            disabled: item.disabled,
        });
    }

    nodes
}

pub fn accordion_dom_id(prefix: &str, value: &str) -> String {
    ui_dom_id(prefix, value)
}

pub fn default_accordion_items() -> Vec<AccordionItem> {
    vec![
        AccordionItem::new(
            "tokens",
            "Design tokens",
            "Colors, type, spacing, radius, shadow, and motion resolve through rs-dean-ui tokens.",
        ),
        AccordionItem::new(
            "state",
            "Renderer-local state",
            "Open and closed sections stay local to the renderer unless a consumer chooses to persist workflow progress.",
        ),
        AccordionItem::new(
            "bevy",
            "Shared scene contract",
            "The same Rust model can produce render nodes for non-DOM surfaces without depending on Leptos.",
        ),
    ]
}

fn unique_accordion_values(items: &[AccordionItem], _context: &()) -> garde::Result {
    for (index, item) in items.iter().enumerate() {
        if items
            .iter()
            .skip(index.saturating_add(1))
            .any(|candidate| candidate.value == item.value)
        {
            return Err(garde::Error::new(format!(
                "duplicate accordion item value `{}`",
                item.value
            )));
        }
    }
    Ok(())
}

fn default_open_references_items<'a>(
    items: &'a [AccordionItem],
) -> impl FnOnce(&[String], &()) -> garde::Result + 'a {
    move |default_open, _context| {
        for (index, value) in default_open.iter().enumerate() {
            if value.is_empty() {
                return Err(garde::Error::new("default open value cannot be empty"));
            }
            if default_open
                .iter()
                .skip(index.saturating_add(1))
                .any(|candidate| candidate == value)
            {
                return Err(garde::Error::new(format!(
                    "duplicate default open value `{value}`"
                )));
            }
            let Some(item) = items.iter().find(|item| item.value == *value) else {
                return Err(garde::Error::new(format!(
                    "default open value `{value}` does not match an accordion item"
                )));
            };
            if item.disabled {
                return Err(garde::Error::new(format!(
                    "default open value `{value}` references a disabled item"
                )));
            }
        }
        Ok(())
    }
}

fn normalize_open(mode: AccordionMode, open: Vec<String>) -> Vec<String> {
    let mut normalized = Vec::new();
    for value in open {
        if value.is_empty() || normalized.contains(&value) {
            continue;
        }
        normalized.push(value);
        if mode == AccordionMode::Single {
            break;
        }
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_mode_keeps_one_open_item() {
        let mut state = AccordionState::new(
            AccordionMode::Single,
            vec!["first".to_owned(), "second".to_owned()],
        );
        assert_eq!(state.open_values(), &["first".to_owned()]);

        assert_eq!(
            state.apply(AccordionIntent::Open("second".to_owned())),
            AccordionChange::Opened("second".to_owned())
        );
        assert_eq!(state.open_values(), &["second".to_owned()]);
    }

    #[test]
    fn multiple_mode_toggles_independent_items() {
        let mut state = AccordionState::new(AccordionMode::Multiple, vec!["first".to_owned()]);

        assert_eq!(
            state.apply(AccordionIntent::Toggle("second".to_owned())),
            AccordionChange::Opened("second".to_owned())
        );
        assert!(state.is_open("first"));
        assert!(state.is_open("second"));

        assert_eq!(
            state.apply(AccordionIntent::Toggle("first".to_owned())),
            AccordionChange::Closed("first".to_owned())
        );
        assert!(!state.is_open("first"));
        assert!(state.is_open("second"));
    }

    #[test]
    fn model_defaults_are_normalized() {
        let model = AccordionModel::multiple(default_accordion_items()).with_default_open(vec![
            "tokens".to_owned(),
            "tokens".to_owned(),
            String::new(),
            "bevy".to_owned(),
        ]);
        assert_eq!(
            model.state().open_values(),
            &["tokens".to_owned(), "bevy".to_owned()]
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = AccordionModel::single(default_accordion_items())
            .with_default_open(vec!["tokens".to_owned()]);
        let state = model.state();
        let nodes = accordion_render_nodes(&model, &state);

        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(AccordionPart::Root)
        );
        assert_eq!(nodes.len(), 1 + model.items.len() * 3);
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AccordionPart::Trigger && node.open)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AccordionPart::Content && node.open)
        );
    }

    #[test]
    fn dom_ids_are_stable_and_ascii() {
        assert_eq!(
            accordion_dom_id("accordion-panel", "Billing & Plans"),
            "accordion-panel-billing-plans"
        );
    }

    #[test]
    fn default_model_validates_with_garde() {
        let model = AccordionModel::single(default_accordion_items())
            .with_default_open(vec!["tokens".to_owned()]);
        assert!(validate_accordion_model(&model).is_ok());
    }

    #[test]
    fn garde_rejects_empty_item_fields() {
        let model = AccordionModel::single(vec![AccordionItem::new("", "Title", "Content")]);
        assert!(validate_accordion_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_item_values() {
        let model = AccordionModel::multiple(vec![
            AccordionItem::new("same", "First", "First content"),
            AccordionItem::new("same", "Second", "Second content"),
        ]);
        assert!(validate_accordion_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_default_open_values() {
        let model = AccordionModel::single(default_accordion_items())
            .with_default_open(vec!["missing".to_owned()]);
        assert!(validate_accordion_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_default_open_values() {
        let model = AccordionModel::single(vec![
            AccordionItem::new("active", "Active", "Active content"),
            AccordionItem::new("disabled", "Disabled", "Disabled content").disabled(),
        ])
        .with_default_open(vec!["disabled".to_owned()]);
        assert!(validate_accordion_model(&model).is_err());
    }
}
