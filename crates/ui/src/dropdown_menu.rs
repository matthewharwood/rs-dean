use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DropdownMenuDensity {
    Standard,
    Dense,
}

impl DropdownMenuDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DropdownMenuPart {
    Root,
    Trigger,
    Content,
    Item,
    Label,
    Separator,
}

impl DropdownMenuPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Trigger,
        Self::Content,
        Self::Item,
        Self::Label,
        Self::Separator,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "DropdownMenu",
            Self::Trigger => "DropdownMenuTrigger",
            Self::Content => "DropdownMenuContent",
            Self::Item => "DropdownMenuItem",
            Self::Label => "DropdownMenuLabel",
            Self::Separator => "DropdownMenuSeparator",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DropdownMenuItem {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(max = 160))]
    pub detail: String,
    #[garde(custom(validate_optional_dropdown_menu_shortcut))]
    pub shortcut: Option<String>,
    #[garde(skip)]
    pub disabled: bool,
    #[garde(skip)]
    pub destructive: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum DropdownMenuEntry {
    Item(DropdownMenuItem),
    Label { label: String, value: String },
    Separator { value: String },
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DropdownMenuModel {
    #[garde(skip)]
    pub density: DropdownMenuDensity,
    #[garde(length(min = 1, max = 96))]
    pub trigger_label: String,
    #[garde(length(min = 1, max = 128))]
    pub content_label: String,
    #[garde(length(min = 1, max = 24), custom(dropdown_menu_entries_are_valid))]
    pub entries: Vec<DropdownMenuEntry>,
    #[garde(custom(dropdown_menu_value_references_enabled_item(&self.entries, "selected dropdown menu value must reference an enabled item")))]
    pub selected_value: Option<String>,
    #[garde(custom(dropdown_menu_value_references_enabled_item(&self.entries, "active dropdown menu value must reference an enabled item")))]
    pub active_value: Option<String>,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropdownMenuState {
    open: bool,
    active_value: Option<String>,
    selected_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropdownMenuIntent {
    Open,
    Close,
    Toggle,
    Focus(String),
    Select(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropdownMenuChange {
    Opened,
    Closed,
    Focused(String),
    Selected(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropdownMenuRenderNode {
    pub part: DropdownMenuPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub shortcut: String,
    pub density: DropdownMenuDensity,
    pub active: bool,
    pub selected: bool,
    pub visible: bool,
    pub open: bool,
    pub loading: bool,
    pub disabled: bool,
    pub destructive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropdownMenuItemLayoutMetrics {
    pub min_height: f32,
    pub padding_inline: f32,
    pub padding_block: f32,
    pub gap: f32,
    pub font_size: f32,
    pub line_height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropdownMenuLayoutMetrics {
    pub max_width: f32,
    pub root_gap: f32,
    pub trigger_min_height: f32,
    pub trigger_padding_inline: f32,
    pub trigger_padding_block: f32,
    pub trigger_font_size: f32,
    pub trigger_line_height: f32,
    pub content_margin_top: f32,
    pub content_padding: f32,
    pub content_gap: f32,
    pub standard_item: DropdownMenuItemLayoutMetrics,
    pub dense_item: DropdownMenuItemLayoutMetrics,
    pub body_gap: f32,
    pub detail_font_size: f32,
    pub detail_line_height: f32,
    pub shortcut_padding_inline: f32,
    pub shortcut_padding_block: f32,
    pub shortcut_font_size: f32,
    pub shortcut_line_height: f32,
    pub label_padding_inline: f32,
    pub label_padding_block: f32,
    pub label_font_size: f32,
    pub label_line_height: f32,
    pub separator_margin_block: f32,
    pub separator_height: f32,
}

impl DropdownMenuItem {
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

impl DropdownMenuEntry {
    pub fn item(item: DropdownMenuItem) -> Self {
        Self::Item(item)
    }

    pub fn label(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Label {
            label: label.into(),
            value: value.into(),
        }
    }

    pub fn separator(value: impl Into<String>) -> Self {
        Self::Separator {
            value: value.into(),
        }
    }

    fn value(&self) -> &str {
        match self {
            Self::Item(item) => &item.value,
            Self::Label { value, .. } | Self::Separator { value } => value,
        }
    }
}

impl DropdownMenuModel {
    pub fn new(entries: Vec<DropdownMenuEntry>) -> Self {
        Self {
            density: DropdownMenuDensity::Standard,
            trigger_label: "Actions".to_owned(),
            content_label: "Dropdown actions".to_owned(),
            entries,
            selected_value: None,
            active_value: None,
            default_open: true,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: DropdownMenuDensity) -> Self {
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

    pub fn state(&self) -> DropdownMenuState {
        DropdownMenuState::new(
            self.default_open,
            self.active_value.clone(),
            self.selected_value.clone(),
        )
    }
}

impl DropdownMenuState {
    pub fn new(open: bool, active_value: Option<String>, selected_value: Option<String>) -> Self {
        Self {
            open,
            active_value,
            selected_value,
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

    pub fn is_active(&self, value: &str) -> bool {
        self.active_value.as_deref() == Some(value)
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: DropdownMenuIntent) -> DropdownMenuChange {
        match intent {
            DropdownMenuIntent::Open => self.open(),
            DropdownMenuIntent::Close => self.close(),
            DropdownMenuIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            DropdownMenuIntent::Focus(value) => self.focus(value),
            DropdownMenuIntent::Select(value) => self.select(value),
            DropdownMenuIntent::Clear => self.clear(),
        }
    }

    fn open(&mut self) -> DropdownMenuChange {
        if self.open {
            DropdownMenuChange::Unchanged
        } else {
            self.open = true;
            DropdownMenuChange::Opened
        }
    }

    fn close(&mut self) -> DropdownMenuChange {
        if self.open {
            self.open = false;
            DropdownMenuChange::Closed
        } else {
            DropdownMenuChange::Unchanged
        }
    }

    fn focus(&mut self, value: String) -> DropdownMenuChange {
        if value.is_empty() || self.active_value.as_ref() == Some(&value) {
            return DropdownMenuChange::Unchanged;
        }
        self.active_value = Some(value.clone());
        DropdownMenuChange::Focused(value)
    }

    fn select(&mut self, value: String) -> DropdownMenuChange {
        if value.is_empty() || self.selected_value.as_ref() == Some(&value) {
            return DropdownMenuChange::Unchanged;
        }
        self.active_value = Some(value.clone());
        self.selected_value = Some(value.clone());
        self.open = false;
        DropdownMenuChange::Selected(value)
    }

    fn clear(&mut self) -> DropdownMenuChange {
        if self.active_value.is_none() && self.selected_value.is_none() && !self.open {
            return DropdownMenuChange::Unchanged;
        }
        self.open = false;
        self.active_value = None;
        self.selected_value = None;
        DropdownMenuChange::Cleared
    }
}

pub fn validate_dropdown_menu_model(model: &DropdownMenuModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn dropdown_menu_layout_metrics(
    density: DropdownMenuDensity,
    open: bool,
    disabled: bool,
    inline_size: f32,
) -> DropdownMenuLayoutMetrics {
    let dense_root = density == DropdownMenuDensity::Dense && !disabled;
    let dense_trigger = density == DropdownMenuDensity::Dense && !open;
    let standard_item = DropdownMenuItemLayoutMetrics {
        min_height: scale::space::FIELD,
        padding_inline: scale::space::xs(inline_size),
        padding_block: scale::space::xs2(inline_size),
        gap: scale::space::xs(inline_size),
        font_size: scale::font_size::f0(inline_size),
        line_height: scale::line_height::LH0,
    };
    let dense_item = DropdownMenuItemLayoutMetrics {
        min_height: scale::space::s(inline_size),
        padding_inline: scale::space::xs2(inline_size),
        padding_block: scale::space::xs3(inline_size),
        gap: scale::space::xs2(inline_size),
        font_size: scale::font_size::f00(inline_size),
        line_height: scale::line_height::LH0,
    };
    DropdownMenuLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_gap: if dense_root {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        trigger_min_height: if dense_trigger {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        trigger_padding_inline: if dense_trigger {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        trigger_padding_block: if dense_trigger {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        trigger_font_size: if dense_trigger {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        trigger_line_height: scale::line_height::LH0,
        content_margin_top: scale::space::xs2(inline_size),
        content_padding: if density == DropdownMenuDensity::Dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        content_gap: scale::space::xs3(inline_size),
        standard_item,
        dense_item,
        body_gap: scale::space::xs3(inline_size),
        detail_font_size: scale::font_size::f00(inline_size),
        detail_line_height: scale::line_height::LH0,
        shortcut_padding_inline: scale::space::xs2(inline_size),
        shortcut_padding_block: scale::space::xs3(inline_size),
        shortcut_font_size: scale::font_size::f00(inline_size),
        shortcut_line_height: scale::line_height::LH0,
        label_padding_inline: scale::space::xs(inline_size),
        label_padding_block: scale::space::xs3(inline_size),
        label_font_size: scale::font_size::f00(inline_size),
        label_line_height: scale::line_height::LH0,
        separator_margin_block: scale::space::xs3(inline_size),
        separator_height: scale::space::xs3(inline_size),
    }
}

pub fn dropdown_menu_render_nodes(
    model: &DropdownMenuModel,
    state: &DropdownMenuState,
) -> Vec<DropdownMenuRenderNode> {
    let blocked = model.disabled || model.loading;
    let mut nodes = Vec::with_capacity(model.entries.len().saturating_add(3));
    nodes.push(DropdownMenuRenderNode {
        part: DropdownMenuPart::Root,
        index: 0,
        value: state.selected_value().unwrap_or("none").to_owned(),
        label: "Dropdown menu".to_owned(),
        detail: if state.is_open() {
            "Dropdown menu open".to_owned()
        } else {
            "Dropdown menu closed".to_owned()
        },
        shortcut: String::new(),
        density: model.density,
        active: state.active_value().is_some(),
        selected: state.selected_value().is_some(),
        visible: true,
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked,
        destructive: false,
    });
    nodes.push(DropdownMenuRenderNode {
        part: DropdownMenuPart::Trigger,
        index: 0,
        value: "trigger".to_owned(),
        label: model.trigger_label.clone(),
        detail: "Open dropdown menu".to_owned(),
        shortcut: String::new(),
        density: model.density,
        active: state.is_open(),
        selected: false,
        visible: true,
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked,
        destructive: false,
    });
    nodes.push(DropdownMenuRenderNode {
        part: DropdownMenuPart::Content,
        index: 0,
        value: "content".to_owned(),
        label: model.content_label.clone(),
        detail: format!("{} menu entries", model.entries.len()),
        shortcut: String::new(),
        density: model.density,
        active: false,
        selected: false,
        visible: state.is_open(),
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked,
        destructive: false,
    });

    for (index, entry) in model.entries.iter().enumerate() {
        match entry {
            DropdownMenuEntry::Item(item) => {
                nodes.push(item_node(model, state, item, index, blocked))
            }
            DropdownMenuEntry::Label { label, value } => nodes.push(DropdownMenuRenderNode {
                part: DropdownMenuPart::Label,
                index,
                value: value.clone(),
                label: label.clone(),
                detail: "Dropdown menu group label".to_owned(),
                shortcut: String::new(),
                density: model.density,
                active: false,
                selected: false,
                visible: state.is_open(),
                open: state.is_open(),
                loading: model.loading,
                disabled: true,
                destructive: false,
            }),
            DropdownMenuEntry::Separator { value } => nodes.push(DropdownMenuRenderNode {
                part: DropdownMenuPart::Separator,
                index,
                value: value.clone(),
                label: "Separator".to_owned(),
                detail: "Dropdown menu separator".to_owned(),
                shortcut: String::new(),
                density: model.density,
                active: false,
                selected: false,
                visible: state.is_open(),
                open: state.is_open(),
                loading: model.loading,
                disabled: true,
                destructive: false,
            }),
        }
    }
    nodes
}

pub fn default_dropdown_menu_model() -> DropdownMenuModel {
    DropdownMenuModel::new(vec![
        DropdownMenuEntry::label("File", "file-group"),
        DropdownMenuEntry::item(
            DropdownMenuItem::new("Rename", "rename")
                .with_detail("Rename the current component.")
                .with_shortcut("R"),
        ),
        DropdownMenuEntry::item(
            DropdownMenuItem::new("Duplicate", "duplicate")
                .with_detail("Create a copy.")
                .with_shortcut("D"),
        ),
        DropdownMenuEntry::separator("file-separator"),
        DropdownMenuEntry::item(
            DropdownMenuItem::new("Delete", "delete")
                .with_detail("Remove the selected item.")
                .destructive(),
        ),
    ])
    .with_selected_value("rename")
    .with_active_value("duplicate")
}

fn item_node(
    model: &DropdownMenuModel,
    state: &DropdownMenuState,
    item: &DropdownMenuItem,
    index: usize,
    blocked: bool,
) -> DropdownMenuRenderNode {
    DropdownMenuRenderNode {
        part: DropdownMenuPart::Item,
        index,
        value: item.value.clone(),
        label: item.label.clone(),
        detail: item.detail.clone(),
        shortcut: item.shortcut.clone().unwrap_or_default(),
        density: model.density,
        active: state.is_active(&item.value),
        selected: state.is_selected(&item.value),
        visible: state.is_open(),
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked || item.disabled,
        destructive: item.destructive,
    }
}

fn validate_optional_dropdown_menu_shortcut(
    shortcut: &Option<String>,
    _context: &(),
) -> garde::Result {
    if let Some(shortcut) = shortcut
        && !(1..=32).contains(&shortcut.chars().count())
    {
        return Err(garde::Error::new(
            "dropdown menu shortcut must be 1 to 32 characters",
        ));
    }
    Ok(())
}

fn dropdown_menu_entries_are_valid(
    entries: &Vec<DropdownMenuEntry>,
    _context: &(),
) -> garde::Result {
    let mut values = HashSet::with_capacity(entries.len());
    let mut item_count = 0usize;
    for entry in entries {
        if !values.insert(entry.value()) {
            return Err(garde::Error::new(
                "dropdown menu entry values must be unique",
            ));
        }
        match entry {
            DropdownMenuEntry::Item(item) => {
                item_count = item_count.saturating_add(1);
                validate_dropdown_menu_item(item)?;
            }
            DropdownMenuEntry::Label { label, value } => {
                validate_menu_copy("dropdown menu label", label, 1, 96)?;
                validate_menu_copy("dropdown menu label value", value, 1, 128)?;
            }
            DropdownMenuEntry::Separator { value } => {
                validate_menu_copy("dropdown menu separator value", value, 1, 128)?;
            }
        }
    }
    if item_count == 0 {
        return Err(garde::Error::new(
            "dropdown menu must include at least one item",
        ));
    }
    Ok(())
}

fn validate_dropdown_menu_item(item: &DropdownMenuItem) -> garde::Result {
    validate_menu_copy("dropdown menu item label", &item.label, 1, 96)?;
    validate_menu_copy("dropdown menu item value", &item.value, 1, 128)?;
    validate_menu_copy("dropdown menu item detail", &item.detail, 0, 160)?;
    validate_optional_dropdown_menu_shortcut(&item.shortcut, &())
}

fn validate_menu_copy(label: &'static str, value: &str, min: usize, max: usize) -> garde::Result {
    let count = value.chars().count();
    if (min..=max).contains(&count) {
        Ok(())
    } else {
        Err(garde::Error::new(format!(
            "{label} must be {min} to {max} characters"
        )))
    }
}

fn dropdown_menu_value_references_enabled_item<'a>(
    entries: &'a [DropdownMenuEntry],
    message: &'static str,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !dropdown_menu_has_enabled_item(entries, value)
        {
            return Err(garde::Error::new(message));
        }
        Ok(())
    }
}

fn dropdown_menu_has_enabled_item(entries: &[DropdownMenuEntry], value: &str) -> bool {
    entries.iter().any(|entry| match entry {
        DropdownMenuEntry::Item(item) => item.value == value && !item.disabled,
        DropdownMenuEntry::Label { .. } | DropdownMenuEntry::Separator { .. } => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_dropdown_menu_model(&default_dropdown_menu_model()).is_ok());
    }

    #[test]
    fn layout_metrics_preserve_dense_and_emphasized_token_scales() {
        let standard =
            dropdown_menu_layout_metrics(DropdownMenuDensity::Standard, true, false, 1_000.0);
        let dense_closed =
            dropdown_menu_layout_metrics(DropdownMenuDensity::Dense, false, false, 1_000.0);
        let dense_open =
            dropdown_menu_layout_metrics(DropdownMenuDensity::Dense, true, false, 1_000.0);

        assert!(dense_closed.trigger_min_height < standard.trigger_min_height);
        assert_eq!(dense_open.trigger_min_height, standard.trigger_min_height);
        assert!(dense_open.dense_item.min_height < dense_open.standard_item.min_height);
        assert_eq!(standard.max_width, scale::container::CONTROL);
    }

    #[test]
    fn garde_rejects_empty_entries() {
        let model = DropdownMenuModel::new(Vec::new());
        assert!(validate_dropdown_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_entry_values() {
        let model = DropdownMenuModel::new(vec![
            DropdownMenuEntry::separator("same"),
            DropdownMenuEntry::separator("same"),
        ]);
        assert!(validate_dropdown_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_item_label() {
        let model = DropdownMenuModel::new(vec![DropdownMenuEntry::item(DropdownMenuItem::new(
            "", "rename",
        ))]);
        assert!(validate_dropdown_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_value() {
        let model = default_dropdown_menu_model().with_selected_value("unknown");
        assert!(validate_dropdown_menu_model(&model).is_err());
    }

    #[test]
    fn state_opens_selects_and_clears() {
        let mut state = DropdownMenuState::new(false, None, None);
        assert_eq!(
            state.apply(DropdownMenuIntent::Open),
            DropdownMenuChange::Opened
        );
        assert!(state.is_open());
        assert_eq!(
            state.apply(DropdownMenuIntent::Focus("rename".to_owned())),
            DropdownMenuChange::Focused("rename".to_owned())
        );
        assert_eq!(state.active_value(), Some("rename"));
        assert_eq!(
            state.apply(DropdownMenuIntent::Select("duplicate".to_owned())),
            DropdownMenuChange::Selected("duplicate".to_owned())
        );
        assert_eq!(state.selected_value(), Some("duplicate"));
        assert!(!state.is_open());
        assert_eq!(
            state.apply(DropdownMenuIntent::Clear),
            DropdownMenuChange::Cleared
        );
    }

    #[test]
    fn render_nodes_cover_repeatable_dropdown_menu_anatomy() {
        let model = default_dropdown_menu_model();
        let nodes = dropdown_menu_render_nodes(&model, &model.state());
        for part in DropdownMenuPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == DropdownMenuPart::Item)
                .count(),
            3
        );
    }

    #[test]
    fn closed_state_hides_menu_entries() {
        let model = default_dropdown_menu_model().closed();
        let nodes = dropdown_menu_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| matches!(
                    node.part,
                    DropdownMenuPart::Content
                        | DropdownMenuPart::Item
                        | DropdownMenuPart::Label
                        | DropdownMenuPart::Separator
                ))
                .all(|node| !node.visible)
        );
    }

    #[test]
    fn loading_disables_item_nodes() {
        let model = default_dropdown_menu_model().loading();
        let nodes = dropdown_menu_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == DropdownMenuPart::Item)
                .all(|node| node.disabled)
        );
    }
}
