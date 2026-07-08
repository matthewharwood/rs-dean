use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ItemDensity {
    Standard,
    Dense,
}

impl ItemDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ItemPart {
    Root,
    Media,
    Content,
    Title,
    Description,
    Actions,
}

impl ItemPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Media,
        Self::Content,
        Self::Title,
        Self::Description,
        Self::Actions,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Item",
            Self::Media => "ItemMedia",
            Self::Content => "ItemContent",
            Self::Title => "ItemTitle",
            Self::Description => "ItemDescription",
            Self::Actions => "ItemActions",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ItemAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ItemModel {
    #[garde(skip)]
    pub density: ItemDensity,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 320))]
    pub description: String,
    #[garde(custom(validate_optional_item_copy))]
    pub media: Option<String>,
    #[garde(custom(validate_item_actions))]
    pub actions: Vec<ItemAction>,
    #[garde(custom(validate_optional_item_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemState {
    active_part: Option<ItemPart>,
    active_action: Option<String>,
    activated_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemIntent {
    Focus(ItemPart),
    FocusAction(String),
    Blur,
    ActivateAction(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemChange {
    Focused(ItemPart),
    ActionFocused(String),
    Blurred,
    Activated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemRenderNode {
    pub part: ItemPart,
    pub index: Option<usize>,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: ItemDensity,
    pub active: bool,
    pub invalid: bool,
    pub visible: bool,
    pub actionable: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl ItemAction {
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

impl ItemModel {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            density: ItemDensity::Standard,
            title: title.into(),
            description: description.into(),
            media: Some("UI".to_owned()),
            actions: vec![ItemAction::new("Open", "open-item")],
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: ItemDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_media(mut self, media: impl Into<String>) -> Self {
        self.media = Some(media.into());
        self
    }

    pub fn without_media(mut self) -> Self {
        self.media = None;
        self
    }

    pub fn with_action(mut self, action: ItemAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn with_actions(mut self, actions: Vec<ItemAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn without_actions(mut self) -> Self {
        self.actions.clear();
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

    pub const fn state(&self) -> ItemState {
        let _ = self;
        ItemState::resting()
    }
}

impl ItemState {
    pub const fn resting() -> Self {
        Self {
            active_part: None,
            active_action: None,
            activated_value: None,
        }
    }

    pub const fn is_active(&self, part: ItemPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn is_action_active(&self, value: &str) -> bool {
        matches!(self.active_action.as_deref(), Some(active) if active == value)
    }

    pub fn activated_value(&self) -> Option<&str> {
        self.activated_value.as_deref()
    }

    pub fn apply(&mut self, intent: ItemIntent) -> ItemChange {
        match intent {
            ItemIntent::Focus(part) => self.focus(part),
            ItemIntent::FocusAction(value) => self.focus_action(value),
            ItemIntent::Blur => self.blur(),
            ItemIntent::ActivateAction(value) => self.activate_action(value),
            ItemIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, part: ItemPart) -> ItemChange {
        if self.is_active(part) && self.active_action.is_none() {
            ItemChange::Unchanged
        } else {
            self.active_part = Some(part);
            self.active_action = None;
            ItemChange::Focused(part)
        }
    }

    fn focus_action(&mut self, value: String) -> ItemChange {
        if value.is_empty() {
            return ItemChange::Unchanged;
        }
        if self.is_action_active(&value) {
            ItemChange::Unchanged
        } else {
            self.active_part = Some(ItemPart::Actions);
            self.active_action = Some(value.clone());
            ItemChange::ActionFocused(value)
        }
    }

    fn blur(&mut self) -> ItemChange {
        if self.active_part.is_none() && self.active_action.is_none() {
            ItemChange::Unchanged
        } else {
            self.active_part = None;
            self.active_action = None;
            ItemChange::Blurred
        }
    }

    fn activate_action(&mut self, value: String) -> ItemChange {
        if value.is_empty() {
            ItemChange::Unchanged
        } else {
            self.active_part = Some(ItemPart::Actions);
            self.active_action = Some(value.clone());
            self.activated_value = Some(value.clone());
            ItemChange::Activated(value)
        }
    }

    fn clear(&mut self) -> ItemChange {
        if self.active_part.is_none()
            && self.active_action.is_none()
            && self.activated_value.is_none()
        {
            ItemChange::Unchanged
        } else {
            self.active_part = None;
            self.active_action = None;
            self.activated_value = None;
            ItemChange::Cleared
        }
    }
}

pub fn validate_item_model(model: &ItemModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn item_render_nodes(model: &ItemModel, state: &ItemState) -> Vec<ItemRenderNode> {
    let invalid = model.error.is_some();
    let media_label = model.media.as_deref().unwrap_or("No media");
    let description_detail = model.error.as_deref().unwrap_or(&model.description);
    let mut nodes = Vec::with_capacity(5 + model.actions.len().max(1));

    nodes.push(item_node(
        model,
        state,
        ItemNodeDraft::new(ItemPart::Root, "item", "Item", &model.title)
            .disabled(model.disabled)
            .invalid(invalid),
    ));
    nodes.push(item_node(
        model,
        state,
        ItemNodeDraft::new(ItemPart::Media, "item-media", media_label, "Item media")
            .visible(model.media.is_some())
            .disabled(model.disabled || model.media.is_none()),
    ));
    nodes.push(item_node(
        model,
        state,
        ItemNodeDraft::new(
            ItemPart::Content,
            "item-content",
            "Item content",
            &model.description,
        )
        .disabled(model.disabled),
    ));
    nodes.push(item_node(
        model,
        state,
        ItemNodeDraft::new(ItemPart::Title, "item-title", &model.title, "Item title")
            .disabled(model.disabled),
    ));
    nodes.push(item_node(
        model,
        state,
        ItemNodeDraft::new(
            ItemPart::Description,
            "item-description",
            "Item description",
            description_detail,
        )
        .disabled(model.disabled)
        .invalid(invalid),
    ));

    if model.actions.is_empty() {
        nodes.push(item_node(
            model,
            state,
            ItemNodeDraft::new(
                ItemPart::Actions,
                "item-actions",
                "No actions",
                "No item action configured",
            )
            .visible(false)
            .disabled(true),
        ));
    } else {
        for (index, action) in model.actions.iter().enumerate() {
            nodes.push(item_node(
                model,
                state,
                ItemNodeDraft::new(
                    ItemPart::Actions,
                    &action.value,
                    &action.label,
                    "Item action",
                )
                .index(index)
                .actionable(true)
                .disabled(model.disabled || model.loading || action.disabled),
            ));
        }
    }

    nodes
}

pub fn default_item_model() -> ItemModel {
    ItemModel::new(
        "Build widgets",
        "Shared contracts are in place for Leptos and Bevy.",
    )
}

struct ItemNodeDraft<'a> {
    part: ItemPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    index: Option<usize>,
    visible: bool,
    actionable: bool,
    disabled: bool,
    invalid: bool,
}

impl<'a> ItemNodeDraft<'a> {
    const fn new(part: ItemPart, value: &'a str, label: &'a str, detail: &'a str) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            index: None,
            visible: true,
            actionable: false,
            disabled: false,
            invalid: false,
        }
    }

    const fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    const fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    const fn actionable(mut self, actionable: bool) -> Self {
        self.actionable = actionable;
        self
    }

    const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    const fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
}

fn item_node(model: &ItemModel, state: &ItemState, draft: ItemNodeDraft<'_>) -> ItemRenderNode {
    let active = if draft.part == ItemPart::Actions {
        state.is_action_active(draft.value)
    } else {
        state.is_active(draft.part)
    };
    ItemRenderNode {
        part: draft.part,
        index: draft.index,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        active,
        invalid: draft.invalid,
        visible: draft.visible,
        actionable: draft.actionable,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn validate_optional_item_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && (copy.is_empty() || copy.len() > 320)
    {
        return Err(garde::Error::new(
            "item optional copy must be 1..=320 characters",
        ));
    }
    Ok(())
}

fn validate_item_actions(actions: &Vec<ItemAction>, _context: &()) -> garde::Result {
    if actions.len() > 4 {
        return Err(garde::Error::new("item actions must contain 0..=4 actions"));
    }

    let mut values = HashSet::with_capacity(actions.len());
    for action in actions {
        action
            .validate()
            .map_err(|report| garde::Error::new(format!("invalid item action: {report}")))?;
        if !values.insert(action.value.as_str()) {
            return Err(garde::Error::new(
                "item action values must be unique within the item",
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_item_model(&default_item_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = ItemModel::new("", "Description");
        assert!(validate_item_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_description() {
        let model = ItemModel::new("Title", "");
        assert!(validate_item_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_media() {
        let model = default_item_model().with_media("");
        assert!(validate_item_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_action_contract() {
        let model = default_item_model().with_actions(vec![ItemAction::new("", "broken")]);
        assert!(validate_item_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_action_values() {
        let model = default_item_model().with_actions(vec![
            ItemAction::new("Open", "same"),
            ItemAction::new("Archive", "same"),
        ]);
        assert!(validate_item_model(&model).is_err());
    }

    #[test]
    fn state_tracks_action_focus_activation_blur_and_clear() {
        let mut state = ItemState::resting();
        assert!(!state.is_active(ItemPart::Actions));
        assert_eq!(
            state.apply(ItemIntent::FocusAction("open-item".to_owned())),
            ItemChange::ActionFocused("open-item".to_owned())
        );
        assert!(state.is_active(ItemPart::Actions));
        assert!(state.is_action_active("open-item"));
        assert_eq!(
            state.apply(ItemIntent::ActivateAction("open-item".to_owned())),
            ItemChange::Activated("open-item".to_owned())
        );
        assert_eq!(state.activated_value(), Some("open-item"));
        assert_eq!(state.apply(ItemIntent::Blur), ItemChange::Blurred);
        assert_eq!(state.activated_value(), Some("open-item"));
        assert_eq!(state.apply(ItemIntent::Clear), ItemChange::Cleared);
        assert_eq!(state.activated_value(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_item_model();
        let nodes = item_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), ItemPart::ALL.len());
        assert_eq!(nodes.first().map(|node| node.part), Some(ItemPart::Root));
        for part in ItemPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn repeatable_actions_keep_stable_indexes() {
        let model = default_item_model().with_actions(vec![
            ItemAction::new("Open", "open"),
            ItemAction::new("Archive", "archive"),
        ]);
        let nodes = item_render_nodes(&model, &model.state());
        let action_nodes = nodes
            .into_iter()
            .filter(|node| node.part == ItemPart::Actions)
            .collect::<Vec<_>>();
        assert_eq!(action_nodes.len(), 2);
        assert_eq!(action_nodes[0].index, Some(0));
        assert_eq!(action_nodes[1].index, Some(1));
    }

    #[test]
    fn missing_optional_parts_keep_hidden_disabled_nodes() {
        let model = default_item_model().without_media().without_actions();
        let nodes = item_render_nodes(&model, &model.state());
        let media = nodes
            .iter()
            .find(|node| node.part == ItemPart::Media)
            .expect("item render nodes include media");
        let actions = nodes
            .iter()
            .find(|node| node.part == ItemPart::Actions)
            .expect("item render nodes include actions");
        assert!(!media.visible);
        assert!(media.disabled);
        assert!(!actions.visible);
        assert!(actions.disabled);
        assert!(!actions.actionable);
    }

    #[test]
    fn error_marks_root_and_description_invalid() {
        let model = default_item_model().with_error("Description could not load.");
        let nodes = item_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == ItemPart::Root && node.invalid)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == ItemPart::Description && node.invalid)
        );
    }
}
