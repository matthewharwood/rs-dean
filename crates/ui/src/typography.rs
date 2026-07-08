use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::dom::ui_dom_id;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TypographyDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TypographyPart {
    Root,
    H1,
    H2,
    P,
    List,
    Blockquote,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct TypographyListItem {
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 96))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct TypographyModel {
    #[garde(skip)]
    pub density: TypographyDensity,
    #[garde(length(min = 1, max = 160))]
    pub heading: String,
    #[garde(length(min = 1, max = 160))]
    pub subheading: String,
    #[garde(length(min = 1, max = 640))]
    pub paragraph: String,
    #[garde(length(min = 1, max = 128))]
    pub list_label: String,
    #[garde(custom(validate_typography_items))]
    pub list_items: Vec<TypographyListItem>,
    #[garde(length(min = 1, max = 480))]
    pub blockquote: String,
    #[garde(custom(validate_optional_typography_copy))]
    pub cite: Option<String>,
    #[garde(custom(validate_optional_typography_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypographyState {
    list_count: usize,
    active_part: Option<TypographyPart>,
    active_list_item: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypographyIntent {
    Focus(TypographyPart),
    FocusListItem(usize),
    Hover(TypographyPart),
    HoverListItem(usize),
    Blur,
    Leave,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypographyChange {
    Focused(TypographyPart),
    ListItemFocused(usize),
    Hovered(TypographyPart),
    ListItemHovered(usize),
    Blurred,
    Left,
    Reset,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypographyRenderNode {
    pub part: TypographyPart,
    pub index: Option<usize>,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: TypographyDensity,
    pub active: bool,
    pub visible: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl TypographyDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl TypographyPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::H1,
        Self::H2,
        Self::P,
        Self::List,
        Self::Blockquote,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Typography",
            Self::H1 => "TypographyH1",
            Self::H2 => "TypographyH2",
            Self::P => "TypographyP",
            Self::List => "TypographyList",
            Self::Blockquote => "TypographyBlockquote",
        }
    }
}

impl TypographyListItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            detail: "Typography list item".to_owned(),
            disabled: false,
        }
    }

    pub fn from_label(label: impl Into<String>) -> Self {
        let label = label.into();
        let value = typography_value(&label);
        Self::new(label, value)
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

impl TypographyModel {
    pub fn new(
        heading: impl Into<String>,
        subheading: impl Into<String>,
        paragraph: impl Into<String>,
        list_items: Vec<TypographyListItem>,
        blockquote: impl Into<String>,
    ) -> Self {
        Self {
            density: TypographyDensity::Standard,
            heading: heading.into(),
            subheading: subheading.into(),
            paragraph: paragraph.into(),
            list_label: "Typography checklist".to_owned(),
            list_items,
            blockquote: blockquote.into(),
            cite: None,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: TypographyDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_heading(mut self, heading: impl Into<String>) -> Self {
        self.heading = heading.into();
        self
    }

    pub fn with_subheading(mut self, subheading: impl Into<String>) -> Self {
        self.subheading = subheading.into();
        self
    }

    pub fn with_paragraph(mut self, paragraph: impl Into<String>) -> Self {
        self.paragraph = paragraph.into();
        self
    }

    pub fn with_list_label(mut self, list_label: impl Into<String>) -> Self {
        self.list_label = list_label.into();
        self
    }

    pub fn with_list_items(mut self, list_items: Vec<TypographyListItem>) -> Self {
        self.list_items = list_items;
        self
    }

    pub fn with_blockquote(mut self, blockquote: impl Into<String>) -> Self {
        self.blockquote = blockquote.into();
        self
    }

    pub fn with_cite(mut self, cite: impl Into<String>) -> Self {
        self.cite = Some(cite.into());
        self
    }

    pub fn without_cite(mut self) -> Self {
        self.cite = None;
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

    pub const fn loading(mut self) -> Self {
        self.loading = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn state(&self) -> TypographyState {
        TypographyState::new(self.list_items.len())
    }
}

impl TypographyState {
    pub const fn new(list_count: usize) -> Self {
        Self {
            list_count,
            active_part: None,
            active_list_item: None,
        }
    }

    pub const fn active_part(&self) -> Option<TypographyPart> {
        self.active_part
    }

    pub const fn active_list_item(&self) -> Option<usize> {
        self.active_list_item
    }

    pub const fn is_active(&self, part: TypographyPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub const fn is_list_item_active(&self, index: usize) -> bool {
        matches!(self.active_list_item, Some(active) if active == index)
    }

    pub fn apply(&mut self, intent: TypographyIntent) -> TypographyChange {
        match intent {
            TypographyIntent::Focus(part) => self.focus(part),
            TypographyIntent::FocusListItem(index) => self.focus_list_item(index),
            TypographyIntent::Hover(part) => self.hover(part),
            TypographyIntent::HoverListItem(index) => self.hover_list_item(index),
            TypographyIntent::Blur => self.blur(),
            TypographyIntent::Leave => self.leave(),
            TypographyIntent::Reset => self.reset(),
        }
    }

    fn focus(&mut self, part: TypographyPart) -> TypographyChange {
        if self.active_part == Some(part) && self.active_list_item.is_none() {
            TypographyChange::Unchanged
        } else {
            self.active_part = Some(part);
            self.active_list_item = None;
            TypographyChange::Focused(part)
        }
    }

    fn focus_list_item(&mut self, index: usize) -> TypographyChange {
        if index >= self.list_count {
            return TypographyChange::Unchanged;
        }
        if self.active_part == Some(TypographyPart::List) && self.active_list_item == Some(index) {
            TypographyChange::Unchanged
        } else {
            self.active_part = Some(TypographyPart::List);
            self.active_list_item = Some(index);
            TypographyChange::ListItemFocused(index)
        }
    }

    fn hover(&mut self, part: TypographyPart) -> TypographyChange {
        if self.active_part == Some(part) && self.active_list_item.is_none() {
            TypographyChange::Unchanged
        } else {
            self.active_part = Some(part);
            self.active_list_item = None;
            TypographyChange::Hovered(part)
        }
    }

    fn hover_list_item(&mut self, index: usize) -> TypographyChange {
        if index >= self.list_count {
            return TypographyChange::Unchanged;
        }
        if self.active_part == Some(TypographyPart::List) && self.active_list_item == Some(index) {
            TypographyChange::Unchanged
        } else {
            self.active_part = Some(TypographyPart::List);
            self.active_list_item = Some(index);
            TypographyChange::ListItemHovered(index)
        }
    }

    fn blur(&mut self) -> TypographyChange {
        if self.active_part.is_none() && self.active_list_item.is_none() {
            TypographyChange::Unchanged
        } else {
            self.active_part = None;
            self.active_list_item = None;
            TypographyChange::Blurred
        }
    }

    fn leave(&mut self) -> TypographyChange {
        if self.active_part.is_none() && self.active_list_item.is_none() {
            TypographyChange::Unchanged
        } else {
            self.active_part = None;
            self.active_list_item = None;
            TypographyChange::Left
        }
    }

    fn reset(&mut self) -> TypographyChange {
        if self.active_part.is_none() && self.active_list_item.is_none() {
            TypographyChange::Unchanged
        } else {
            self.active_part = None;
            self.active_list_item = None;
            TypographyChange::Reset
        }
    }
}

pub fn validate_typography_model(model: &TypographyModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn typography_render_nodes(
    model: &TypographyModel,
    state: &TypographyState,
) -> Vec<TypographyRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let mut nodes = Vec::with_capacity(model.list_items.len() + 6);
    nodes.push(typography_node(
        model,
        state,
        TypographyNodeDraft::new(
            TypographyPart::Root,
            "typography",
            "Typography",
            "Typographic content flow",
        )
        .invalid(invalid)
        .disabled(model.disabled),
    ));
    nodes.push(typography_node(
        model,
        state,
        TypographyNodeDraft::new(
            TypographyPart::H1,
            &typography_value(&model.heading),
            &model.heading,
            "Heading level 1",
        )
        .invalid(invalid)
        .disabled(blocked),
    ));
    nodes.push(typography_node(
        model,
        state,
        TypographyNodeDraft::new(
            TypographyPart::H2,
            &typography_value(&model.subheading),
            &model.subheading,
            "Heading level 2",
        )
        .invalid(invalid)
        .disabled(blocked),
    ));
    nodes.push(typography_node(
        model,
        state,
        TypographyNodeDraft::new(
            TypographyPart::P,
            "paragraph",
            "Paragraph",
            model.error.as_deref().unwrap_or(&model.paragraph),
        )
        .invalid(invalid)
        .disabled(blocked),
    ));
    nodes.push(typography_node(
        model,
        state,
        TypographyNodeDraft::new(
            TypographyPart::List,
            "list",
            &model.list_label,
            "Repeatable typography list",
        )
        .invalid(invalid)
        .disabled(blocked),
    ));
    for (index, item) in model.list_items.iter().enumerate() {
        nodes.push(typography_node(
            model,
            state,
            TypographyNodeDraft::new(TypographyPart::List, &item.value, &item.label, &item.detail)
                .index(index)
                .invalid(invalid)
                .disabled(blocked || item.disabled),
        ));
    }
    let blockquote_detail = match (model.error.as_deref(), model.cite.as_deref()) {
        (Some(error), _) => error.to_owned(),
        (None, Some(cite)) => format!("{} - {}", model.blockquote, cite),
        (None, None) => model.blockquote.clone(),
    };
    nodes.push(typography_node(
        model,
        state,
        TypographyNodeDraft::new(
            TypographyPart::Blockquote,
            "blockquote",
            "Blockquote",
            &blockquote_detail,
        )
        .invalid(invalid)
        .disabled(blocked),
    ));
    nodes
}

pub fn default_typography_items() -> Vec<TypographyListItem> {
    vec![
        TypographyListItem::new("Shared tokens", "shared-tokens")
            .with_detail("Type, spacing, and color resolve through rs-dean-ui tokens."),
        TypographyListItem::new("Leptos DOM", "leptos-dom")
            .with_detail("The DOM renderer only consumes validated render nodes."),
        TypographyListItem::new("Bevy scene", "bevy-scene")
            .with_detail("Scene primitives derive from the same Rust typography contract."),
    ]
}

pub fn default_typography_model() -> TypographyModel {
    TypographyModel::new(
        "Readable systems",
        "Token scales",
        "Text sizes, leading, spacing, and color come from the shared scale.",
        default_typography_items(),
        "Theme once, render everywhere.",
    )
    .with_list_label("Typography surfaces")
    .with_cite("rs-dean-ui")
}

pub fn typography_dom_id(prefix: &str, value: &str) -> String {
    ui_dom_id(prefix, value)
}

struct TypographyNodeDraft<'a> {
    part: TypographyPart,
    index: Option<usize>,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    invalid: bool,
    disabled: bool,
}

impl<'a> TypographyNodeDraft<'a> {
    const fn new(part: TypographyPart, value: &'a str, label: &'a str, detail: &'a str) -> Self {
        Self {
            part,
            index: None,
            value,
            label,
            detail,
            visible: true,
            invalid: false,
            disabled: false,
        }
    }

    const fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    const fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

fn typography_node(
    model: &TypographyModel,
    state: &TypographyState,
    draft: TypographyNodeDraft<'_>,
) -> TypographyRenderNode {
    let active = draft.index.map_or_else(
        || state.is_active(draft.part),
        |index| state.is_list_item_active(index),
    );
    TypographyRenderNode {
        part: draft.part,
        index: draft.index,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        active,
        visible: draft.visible,
        invalid: draft.invalid,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn typography_value(label: &str) -> String {
    let value = label
        .trim()
        .to_ascii_lowercase()
        .replace(|character: char| !character.is_ascii_alphanumeric(), "-");
    value.trim_matches('-').to_owned()
}

fn validate_typography_items(items: &Vec<TypographyListItem>, _context: &()) -> garde::Result {
    if items.is_empty() || items.len() > 8 {
        return Err(garde::Error::new(
            "typography list items must contain 1..=8 items",
        ));
    }

    let mut values = HashSet::with_capacity(items.len());
    for item in items {
        item.validate()
            .map_err(|report| garde::Error::new(format!("invalid typography item: {report}")))?;
        if !values.insert(item.value.as_str()) {
            return Err(garde::Error::new(
                "typography list item values must be unique",
            ));
        }
    }

    Ok(())
}

fn validate_optional_typography_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new(
            "typography optional copy must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_typography_model(&default_typography_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_heading() {
        let model = default_typography_model().with_heading("");
        assert!(validate_typography_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_items() {
        let model = default_typography_model().with_list_items(Vec::new());
        assert!(validate_typography_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_item_values() {
        let model = default_typography_model().with_list_items(vec![
            TypographyListItem::new("First", "same"),
            TypographyListItem::new("Second", "same"),
        ]);
        assert!(validate_typography_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_cite_and_error() {
        assert!(validate_typography_model(&default_typography_model().with_cite("")).is_err());
        assert!(validate_typography_model(&default_typography_model().with_error("")).is_err());
    }

    #[test]
    fn state_tracks_focus_hover_and_reset() {
        let mut state = TypographyState::new(2);
        assert_eq!(
            state.apply(TypographyIntent::Focus(TypographyPart::H1)),
            TypographyChange::Focused(TypographyPart::H1)
        );
        assert!(state.is_active(TypographyPart::H1));
        assert_eq!(
            state.apply(TypographyIntent::HoverListItem(1)),
            TypographyChange::ListItemHovered(1)
        );
        assert!(state.is_active(TypographyPart::List));
        assert!(state.is_list_item_active(1));
        assert_eq!(
            state.apply(TypographyIntent::FocusListItem(4)),
            TypographyChange::Unchanged
        );
        assert_eq!(
            state.apply(TypographyIntent::Reset),
            TypographyChange::Reset
        );
        assert_eq!(state.active_part(), None);
        assert_eq!(state.active_list_item(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy_and_list_items() {
        let model = default_typography_model();
        let nodes = typography_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(TypographyPart::Root)
        );
        for part in TypographyPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        let item_nodes = nodes
            .iter()
            .filter(|node| node.part == TypographyPart::List && node.index.is_some())
            .collect::<Vec<_>>();
        assert_eq!(item_nodes.len(), model.list_items.len());
        assert_eq!(item_nodes[0].index, Some(0));
    }

    #[test]
    fn loading_disables_readable_parts() {
        let model = default_typography_model().loading();
        let nodes = typography_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .all(|node| node.disabled || matches!(node.part, TypographyPart::Root))
        );
    }

    #[test]
    fn disabled_model_marks_all_nodes_disabled() {
        let model = default_typography_model().disabled();
        let nodes = typography_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.disabled));
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_typography_model().with_error("Typography content is invalid.");
        let nodes = typography_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(nodes.iter().any(|node| node.part == TypographyPart::P
            && node.detail == "Typography content is invalid."));
    }

    #[test]
    fn dom_ids_are_stable_and_ascii() {
        assert_eq!(
            typography_dom_id("typography-heading", "Readable Systems"),
            "typography-heading-readable-systems"
        );
    }
}
