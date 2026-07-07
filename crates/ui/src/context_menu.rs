use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextMenuDensity {
    Standard,
    Dense,
}

impl ContextMenuDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextMenuPart {
    Root,
    Trigger,
    Content,
    Item,
    Separator,
    Submenu,
}

impl ContextMenuPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Trigger,
        Self::Content,
        Self::Item,
        Self::Separator,
        Self::Submenu,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "ContextMenu",
            Self::Trigger => "ContextMenuTrigger",
            Self::Content => "ContextMenuContent",
            Self::Item => "ContextMenuItem",
            Self::Separator => "ContextMenuSeparator",
            Self::Submenu => "ContextMenuSubmenu",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ContextMenuAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(max = 160))]
    pub detail: String,
    #[garde(custom(validate_optional_context_menu_shortcut))]
    pub shortcut: Option<String>,
    #[garde(skip)]
    pub disabled: bool,
    #[garde(skip)]
    pub destructive: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ContextMenuSubmenu {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(
        length(min = 1, max = 12),
        dive,
        custom(context_menu_submenu_items_are_valid)
    )]
    pub items: Vec<ContextMenuAction>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum ContextMenuEntry {
    Item(ContextMenuAction),
    Separator { value: String },
    Submenu(ContextMenuSubmenu),
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ContextMenuModel {
    #[garde(skip)]
    pub density: ContextMenuDensity,
    #[garde(length(min = 1, max = 96))]
    pub trigger_label: String,
    #[garde(length(min = 1, max = 128))]
    pub content_label: String,
    #[garde(length(min = 1, max = 24), custom(context_menu_entries_are_valid))]
    pub entries: Vec<ContextMenuEntry>,
    #[garde(custom(context_menu_value_references_enabled_action(&self.entries, "selected context menu value must reference an enabled item")))]
    pub selected_value: Option<String>,
    #[garde(custom(context_menu_value_references_enabled_entry(&self.entries, "active context menu value must reference an enabled entry")))]
    pub active_value: Option<String>,
    #[garde(custom(context_menu_value_references_enabled_submenu(&self.entries)))]
    pub open_submenu: Option<String>,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextMenuState {
    open: bool,
    active_value: Option<String>,
    selected_value: Option<String>,
    open_submenu: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextMenuIntent {
    Open,
    Close,
    Toggle,
    Focus(String),
    Select(String),
    OpenSubmenu(String),
    CloseSubmenu,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextMenuChange {
    Opened,
    Closed,
    Focused(String),
    Selected(String),
    SubmenuOpened(String),
    SubmenuClosed,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextMenuRenderNode {
    pub part: ContextMenuPart,
    pub index: usize,
    pub parent_value: String,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub shortcut: String,
    pub density: ContextMenuDensity,
    pub active: bool,
    pub selected: bool,
    pub visible: bool,
    pub open: bool,
    pub submenu_open: bool,
    pub loading: bool,
    pub disabled: bool,
    pub destructive: bool,
}

impl ContextMenuAction {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            detail: String::new(),
            shortcut: None,
            disabled: false,
            destructive: false,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

impl ContextMenuSubmenu {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        items: Vec<ContextMenuAction>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            items,
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl ContextMenuEntry {
    pub fn item(action: ContextMenuAction) -> Self {
        Self::Item(action)
    }

    pub fn separator(value: impl Into<String>) -> Self {
        Self::Separator {
            value: value.into(),
        }
    }

    pub fn submenu(submenu: ContextMenuSubmenu) -> Self {
        Self::Submenu(submenu)
    }

    fn value(&self) -> &str {
        match self {
            Self::Item(action) => &action.value,
            Self::Separator { value } => value,
            Self::Submenu(submenu) => &submenu.value,
        }
    }

    fn disabled(&self) -> bool {
        match self {
            Self::Item(action) => action.disabled,
            Self::Separator { .. } => true,
            Self::Submenu(submenu) => submenu.disabled,
        }
    }
}

impl ContextMenuModel {
    pub fn new(entries: Vec<ContextMenuEntry>) -> Self {
        Self {
            density: ContextMenuDensity::Standard,
            trigger_label: "Open context menu".to_owned(),
            content_label: "Context menu actions".to_owned(),
            entries,
            selected_value: None,
            active_value: None,
            open_submenu: None,
            default_open: true,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: ContextMenuDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_trigger_label(mut self, label: impl Into<String>) -> Self {
        self.trigger_label = label.into();
        self
    }

    pub fn with_content_label(mut self, label: impl Into<String>) -> Self {
        self.content_label = label.into();
        self
    }

    pub fn with_selected_value(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    pub fn with_active_value(mut self, value: impl Into<String>) -> Self {
        self.active_value = Some(value.into());
        self
    }

    pub fn with_open_submenu(mut self, value: impl Into<String>) -> Self {
        self.open_submenu = Some(value.into());
        self
    }

    pub const fn closed(mut self) -> Self {
        self.default_open = false;
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

    pub fn state(&self) -> ContextMenuState {
        ContextMenuState::new(
            self.default_open,
            self.active_value.clone(),
            self.selected_value.clone(),
            self.open_submenu.clone(),
        )
    }
}

impl ContextMenuState {
    pub fn new(
        open: bool,
        active_value: Option<String>,
        selected_value: Option<String>,
        open_submenu: Option<String>,
    ) -> Self {
        Self {
            open,
            active_value,
            selected_value,
            open_submenu,
        }
    }

    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub fn active_value(&self) -> Option<&str> {
        self.active_value.as_deref()
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    pub fn open_submenu(&self) -> Option<&str> {
        self.open_submenu.as_deref()
    }

    pub fn is_active(&self, value: &str) -> bool {
        self.active_value.as_deref() == Some(value)
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn is_submenu_open(&self, value: &str) -> bool {
        self.open_submenu.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: ContextMenuIntent) -> ContextMenuChange {
        match intent {
            ContextMenuIntent::Open => self.open(),
            ContextMenuIntent::Close => self.close(),
            ContextMenuIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            ContextMenuIntent::Focus(value) => self.focus(value),
            ContextMenuIntent::Select(value) => self.select(value),
            ContextMenuIntent::OpenSubmenu(value) => self.open_submenu_value(value),
            ContextMenuIntent::CloseSubmenu => self.close_submenu(),
            ContextMenuIntent::Clear => self.clear(),
        }
    }

    fn open(&mut self) -> ContextMenuChange {
        if self.open {
            ContextMenuChange::Unchanged
        } else {
            self.open = true;
            ContextMenuChange::Opened
        }
    }

    fn close(&mut self) -> ContextMenuChange {
        if self.open {
            self.open = false;
            self.open_submenu = None;
            ContextMenuChange::Closed
        } else {
            ContextMenuChange::Unchanged
        }
    }

    fn focus(&mut self, value: String) -> ContextMenuChange {
        if value.is_empty() || self.active_value.as_ref() == Some(&value) {
            return ContextMenuChange::Unchanged;
        }
        self.active_value = Some(value.clone());
        ContextMenuChange::Focused(value)
    }

    fn select(&mut self, value: String) -> ContextMenuChange {
        if value.is_empty() || self.selected_value.as_ref() == Some(&value) {
            return ContextMenuChange::Unchanged;
        }
        self.active_value = Some(value.clone());
        self.selected_value = Some(value.clone());
        self.open = false;
        self.open_submenu = None;
        ContextMenuChange::Selected(value)
    }

    fn open_submenu_value(&mut self, value: String) -> ContextMenuChange {
        if value.is_empty() || self.open_submenu.as_ref() == Some(&value) {
            return ContextMenuChange::Unchanged;
        }
        self.open = true;
        self.active_value = Some(value.clone());
        self.open_submenu = Some(value.clone());
        ContextMenuChange::SubmenuOpened(value)
    }

    fn close_submenu(&mut self) -> ContextMenuChange {
        if self.open_submenu.take().is_some() {
            ContextMenuChange::SubmenuClosed
        } else {
            ContextMenuChange::Unchanged
        }
    }

    fn clear(&mut self) -> ContextMenuChange {
        if self.active_value.is_none()
            && self.selected_value.is_none()
            && self.open_submenu.is_none()
            && !self.open
        {
            return ContextMenuChange::Unchanged;
        }
        self.open = false;
        self.active_value = None;
        self.selected_value = None;
        self.open_submenu = None;
        ContextMenuChange::Cleared
    }
}

pub fn validate_context_menu_model(model: &ContextMenuModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn context_menu_render_nodes(
    model: &ContextMenuModel,
    state: &ContextMenuState,
) -> Vec<ContextMenuRenderNode> {
    let blocked = model.disabled || model.loading;
    let mut nodes = Vec::with_capacity(model.entries.len().saturating_mul(3).saturating_add(3));
    nodes.push(ContextMenuRenderNode {
        part: ContextMenuPart::Root,
        index: 0,
        parent_value: String::new(),
        value: state.selected_value().unwrap_or("none").to_owned(),
        label: "Context menu".to_owned(),
        detail: if state.is_open() {
            "Context menu open".to_owned()
        } else {
            "Context menu closed".to_owned()
        },
        shortcut: String::new(),
        density: model.density,
        active: state.active_value().is_some(),
        selected: state.selected_value().is_some(),
        visible: true,
        open: state.is_open(),
        submenu_open: state.open_submenu().is_some(),
        loading: model.loading,
        disabled: blocked,
        destructive: false,
    });
    nodes.push(ContextMenuRenderNode {
        part: ContextMenuPart::Trigger,
        index: 0,
        parent_value: String::new(),
        value: "trigger".to_owned(),
        label: model.trigger_label.clone(),
        detail: "Pointer or keyboard trigger".to_owned(),
        shortcut: String::new(),
        density: model.density,
        active: state.is_open(),
        selected: false,
        visible: true,
        open: state.is_open(),
        submenu_open: state.open_submenu().is_some(),
        loading: model.loading,
        disabled: blocked,
        destructive: false,
    });
    nodes.push(ContextMenuRenderNode {
        part: ContextMenuPart::Content,
        index: 0,
        parent_value: String::new(),
        value: "content".to_owned(),
        label: model.content_label.clone(),
        detail: format!("{} top-level entries", model.entries.len()),
        shortcut: String::new(),
        density: model.density,
        active: false,
        selected: false,
        visible: state.is_open(),
        open: state.is_open(),
        submenu_open: state.open_submenu().is_some(),
        loading: model.loading,
        disabled: blocked,
        destructive: false,
    });

    for (index, entry) in model.entries.iter().enumerate() {
        match entry {
            ContextMenuEntry::Item(action) => nodes.push(action_node(
                model,
                state,
                action,
                index,
                String::new(),
                blocked,
            )),
            ContextMenuEntry::Separator { value } => nodes.push(ContextMenuRenderNode {
                part: ContextMenuPart::Separator,
                index,
                parent_value: String::new(),
                value: value.clone(),
                label: "Separator".to_owned(),
                detail: "Menu item separator".to_owned(),
                shortcut: String::new(),
                density: model.density,
                active: false,
                selected: false,
                visible: state.is_open(),
                open: state.is_open(),
                submenu_open: state.open_submenu().is_some(),
                loading: model.loading,
                disabled: true,
                destructive: false,
            }),
            ContextMenuEntry::Submenu(submenu) => {
                let submenu_open = state.is_submenu_open(&submenu.value);
                let active = state.is_active(&submenu.value);
                nodes.push(ContextMenuRenderNode {
                    part: ContextMenuPart::Submenu,
                    index,
                    parent_value: String::new(),
                    value: submenu.value.clone(),
                    label: submenu.label.clone(),
                    detail: format!("{} nested actions", submenu.items.len()),
                    shortcut: String::new(),
                    density: model.density,
                    active,
                    selected: false,
                    visible: state.is_open(),
                    open: state.is_open(),
                    submenu_open,
                    loading: model.loading,
                    disabled: blocked || submenu.disabled,
                    destructive: false,
                });
                if submenu_open {
                    for (item_index, action) in submenu.items.iter().enumerate() {
                        nodes.push(action_node(
                            model,
                            state,
                            action,
                            item_index,
                            submenu.value.clone(),
                            blocked || submenu.disabled,
                        ));
                    }
                }
            }
        }
    }
    nodes
}

pub fn default_context_menu_model() -> ContextMenuModel {
    ContextMenuModel::new(vec![
        ContextMenuEntry::item(
            ContextMenuAction::new("Back", "back")
                .with_detail("Go to the previous route.")
                .with_shortcut("Cmd+["),
        ),
        ContextMenuEntry::item(
            ContextMenuAction::new("Reload", "reload")
                .with_detail("Refresh the current view.")
                .with_shortcut("Cmd+R"),
        ),
        ContextMenuEntry::separator("primary-separator"),
        ContextMenuEntry::submenu(ContextMenuSubmenu::new(
            "Insert",
            "insert",
            vec![
                ContextMenuAction::new("Card", "insert-card").with_shortcut("C"),
                ContextMenuAction::new("Chart", "insert-chart").with_shortcut("H"),
            ],
        )),
        ContextMenuEntry::separator("danger-separator"),
        ContextMenuEntry::item(
            ContextMenuAction::new("Delete", "delete")
                .with_detail("Remove the selected object.")
                .destructive(),
        ),
    ])
    .with_selected_value("reload")
    .with_active_value("insert")
    .with_open_submenu("insert")
}

fn action_node(
    model: &ContextMenuModel,
    state: &ContextMenuState,
    action: &ContextMenuAction,
    index: usize,
    parent_value: String,
    blocked: bool,
) -> ContextMenuRenderNode {
    ContextMenuRenderNode {
        part: ContextMenuPart::Item,
        index,
        parent_value,
        value: action.value.clone(),
        label: action.label.clone(),
        detail: action.detail.clone(),
        shortcut: action.shortcut.clone().unwrap_or_default(),
        density: model.density,
        active: state.is_active(&action.value),
        selected: state.is_selected(&action.value),
        visible: state.is_open(),
        open: state.is_open(),
        submenu_open: state.open_submenu().is_some(),
        loading: model.loading,
        disabled: blocked || action.disabled,
        destructive: action.destructive,
    }
}

fn validate_optional_context_menu_shortcut(
    shortcut: &Option<String>,
    _context: &(),
) -> garde::Result {
    if let Some(shortcut) = shortcut
        && !(1..=32).contains(&shortcut.chars().count())
    {
        return Err(garde::Error::new(
            "context menu shortcut must be 1 to 32 characters",
        ));
    }
    Ok(())
}

fn context_menu_submenu_items_are_valid(
    items: &Vec<ContextMenuAction>,
    _context: &(),
) -> garde::Result {
    let mut values = HashSet::with_capacity(items.len());
    for item in items {
        validate_context_menu_action(item)?;
        if !values.insert(item.value.as_str()) {
            return Err(garde::Error::new(
                "context menu submenu item values must be unique",
            ));
        }
    }
    Ok(())
}

fn context_menu_entries_are_valid(entries: &Vec<ContextMenuEntry>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(entries.len());
    let mut action_values = HashSet::new();
    for entry in entries {
        if !values.insert(entry.value()) {
            return Err(garde::Error::new(
                "context menu entry values must be unique",
            ));
        }
        match entry {
            ContextMenuEntry::Item(action) => {
                validate_context_menu_action(action)?;
                if !action_values.insert(action.value.as_str()) {
                    return Err(garde::Error::new(
                        "context menu action values must be unique",
                    ));
                }
            }
            ContextMenuEntry::Separator { value } => {
                if !(1..=128).contains(&value.chars().count()) {
                    return Err(garde::Error::new(
                        "context menu separator value must be 1 to 128 characters",
                    ));
                }
            }
            ContextMenuEntry::Submenu(submenu) => {
                validate_context_menu_submenu(submenu)?;
                for action in &submenu.items {
                    if !values.insert(action.value.as_str()) {
                        return Err(garde::Error::new(
                            "context menu entry and action values must be unique",
                        ));
                    }
                    if !action_values.insert(action.value.as_str()) {
                        return Err(garde::Error::new(
                            "context menu action values must be unique",
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate_context_menu_action(action: &ContextMenuAction) -> garde::Result {
    if !(1..=96).contains(&action.label.chars().count()) {
        return Err(garde::Error::new(
            "context menu action label must be 1 to 96 characters",
        ));
    }
    if !(1..=128).contains(&action.value.chars().count()) {
        return Err(garde::Error::new(
            "context menu action value must be 1 to 128 characters",
        ));
    }
    if action.detail.chars().count() > 160 {
        return Err(garde::Error::new(
            "context menu action detail must be at most 160 characters",
        ));
    }
    validate_optional_context_menu_shortcut(&action.shortcut, &())
}

fn validate_context_menu_submenu(submenu: &ContextMenuSubmenu) -> garde::Result {
    if !(1..=96).contains(&submenu.label.chars().count()) {
        return Err(garde::Error::new(
            "context menu submenu label must be 1 to 96 characters",
        ));
    }
    if !(1..=128).contains(&submenu.value.chars().count()) {
        return Err(garde::Error::new(
            "context menu submenu value must be 1 to 128 characters",
        ));
    }
    if !(1..=12).contains(&submenu.items.len()) {
        return Err(garde::Error::new(
            "context menu submenu must have 1 to 12 items",
        ));
    }
    context_menu_submenu_items_are_valid(&submenu.items, &())
}

fn context_menu_value_references_enabled_action<'a>(
    entries: &'a [ContextMenuEntry],
    message: &'static str,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !context_menu_has_enabled_action(entries, value)
        {
            return Err(garde::Error::new(message));
        }
        Ok(())
    }
}

fn context_menu_value_references_enabled_entry<'a>(
    entries: &'a [ContextMenuEntry],
    message: &'static str,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !entries
                .iter()
                .any(|entry| entry.value() == value && !entry.disabled())
            && !context_menu_has_enabled_action(entries, value)
        {
            return Err(garde::Error::new(message));
        }
        Ok(())
    }
}

fn context_menu_value_references_enabled_submenu<'a>(
    entries: &'a [ContextMenuEntry],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !entries.iter().any(|entry| {
                matches!(entry, ContextMenuEntry::Submenu(submenu) if submenu.value == *value && !submenu.disabled)
            })
        {
            return Err(garde::Error::new(
                "open context menu submenu must reference an enabled submenu",
            ));
        }
        Ok(())
    }
}

fn context_menu_has_enabled_action(entries: &[ContextMenuEntry], value: &str) -> bool {
    entries.iter().any(|entry| match entry {
        ContextMenuEntry::Item(action) => action.value == value && !action.disabled,
        ContextMenuEntry::Separator { .. } => false,
        ContextMenuEntry::Submenu(submenu) => submenu
            .items
            .iter()
            .any(|action| action.value == value && !action.disabled),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_context_menu_model(&default_context_menu_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_entries() {
        let model = ContextMenuModel::new(Vec::new());
        assert!(validate_context_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_entry_values() {
        let model = ContextMenuModel::new(vec![
            ContextMenuEntry::separator("same"),
            ContextMenuEntry::separator("same"),
        ]);
        assert!(validate_context_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_action_values_across_submenus() {
        let model = ContextMenuModel::new(vec![
            ContextMenuEntry::item(ContextMenuAction::new("Back", "same")),
            ContextMenuEntry::submenu(ContextMenuSubmenu::new(
                "Insert",
                "insert",
                vec![ContextMenuAction::new("Card", "same")],
            )),
        ]);
        assert!(validate_context_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_action_label() {
        let model = ContextMenuModel::new(vec![ContextMenuEntry::item(ContextMenuAction::new(
            "", "back",
        ))]);
        assert!(validate_context_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_shortcut() {
        let model = ContextMenuModel::new(vec![ContextMenuEntry::item(
            ContextMenuAction::new("Back", "back").with_shortcut(""),
        )]);
        assert!(validate_context_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_value() {
        let model = default_context_menu_model().with_selected_value("unknown");
        assert!(validate_context_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_open_submenu() {
        let model = ContextMenuModel::new(vec![ContextMenuEntry::submenu(
            ContextMenuSubmenu::new(
                "Insert",
                "insert",
                vec![ContextMenuAction::new("Card", "card")],
            )
            .disabled(),
        )])
        .with_open_submenu("insert");
        assert!(validate_context_menu_model(&model).is_err());
    }

    #[test]
    fn state_opens_submenu_selects_and_clears() {
        let mut state = ContextMenuState::new(false, None, None, None);
        assert_eq!(
            state.apply(ContextMenuIntent::Open),
            ContextMenuChange::Opened
        );
        assert!(state.is_open());
        assert_eq!(
            state.apply(ContextMenuIntent::OpenSubmenu("insert".to_owned())),
            ContextMenuChange::SubmenuOpened("insert".to_owned())
        );
        assert_eq!(state.open_submenu(), Some("insert"));
        assert_eq!(
            state.apply(ContextMenuIntent::Select("insert-card".to_owned())),
            ContextMenuChange::Selected("insert-card".to_owned())
        );
        assert_eq!(state.selected_value(), Some("insert-card"));
        assert!(!state.is_open());
        assert_eq!(
            state.apply(ContextMenuIntent::Clear),
            ContextMenuChange::Cleared
        );
        assert_eq!(state.selected_value(), None);
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_context_menu_model();
        let nodes = context_menu_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(ContextMenuPart::Root)
        );
        for part in ContextMenuPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == ContextMenuPart::Item)
                .count()
                >= 3
        );
    }

    #[test]
    fn closed_state_hides_content_nodes() {
        let model = default_context_menu_model().closed();
        let nodes = context_menu_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == ContextMenuPart::Content && !node.visible)
        );
    }

    #[test]
    fn loading_disables_item_nodes() {
        let model = default_context_menu_model().loading();
        let nodes = context_menu_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == ContextMenuPart::Item && node.disabled)
        );
    }
}
