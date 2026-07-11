use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MenubarDensity {
    Standard,
    Dense,
}

impl MenubarDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MenubarPart {
    Root,
    Menu,
    Trigger,
    Content,
    Item,
}

impl MenubarPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Menu,
        Self::Trigger,
        Self::Content,
        Self::Item,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Menubar",
            Self::Menu => "MenubarMenu",
            Self::Trigger => "MenubarTrigger",
            Self::Content => "MenubarContent",
            Self::Item => "MenubarItem",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct MenubarItem {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(custom(validate_optional_menubar_shortcut))]
    pub shortcut: Option<String>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct MenubarMenu {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 12), dive)]
    pub items: Vec<MenubarItem>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct MenubarModel {
    #[garde(skip)]
    pub density: MenubarDensity,
    #[garde(length(min = 1, max = 8), dive, custom(menubar_menus_are_valid))]
    pub menus: Vec<MenubarMenu>,
    #[garde(custom(menubar_menu_value_references_enabled_menu(
        &self.menus,
        "default open menubar menu must reference an enabled menu",
    )))]
    pub default_open: Option<String>,
    #[garde(custom(menubar_item_value_references_enabled_item(
        &self.menus,
        "selected menubar item must reference an enabled item",
    )))]
    pub selected_value: Option<String>,
    #[garde(custom(validate_optional_menubar_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenubarState {
    open_menu: Option<String>,
    focused_value: Option<String>,
    selected_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenubarIntent {
    Open(String),
    Close,
    Toggle(String),
    FocusItem(String),
    Activate(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenubarChange {
    Opened(String),
    Closed,
    Focused(String),
    Activated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenubarRenderNode {
    pub part: MenubarPart,
    pub menu_index: usize,
    pub item_index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub shortcut: String,
    pub menu_value: String,
    pub menu_label: String,
    pub density: MenubarDensity,
    pub open: bool,
    pub focused: bool,
    pub selected: bool,
    pub invalid: bool,
    pub visible: bool,
    pub actionable: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenubarLayoutMetrics {
    pub max_width: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub row_gap: f32,
    pub menu_gap: f32,
    pub trigger_min_height: f32,
    pub trigger_padding_inline: f32,
    pub trigger_padding_block: f32,
    pub trigger_font_size: f32,
    pub emphasized_trigger_min_height: f32,
    pub emphasized_trigger_padding_inline: f32,
    pub emphasized_trigger_padding_block: f32,
    pub emphasized_trigger_font_size: f32,
    pub content_min_width: f32,
    pub content_padding: f32,
    pub content_gap: f32,
    pub item_min_height: f32,
    pub item_padding_inline: f32,
    pub item_padding_block: f32,
    pub item_gap: f32,
    pub item_font_size: f32,
    pub emphasized_item_min_height: f32,
    pub emphasized_item_padding_inline: f32,
    pub emphasized_item_padding_block: f32,
    pub emphasized_item_gap: f32,
    pub emphasized_item_font_size: f32,
    pub shortcut_margin_inline: f32,
    pub shortcut_font_size: f32,
    pub line_height: f32,
}

impl MenubarItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            shortcut: None,
            disabled: false,
        }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl MenubarMenu {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        items: Vec<MenubarItem>,
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

impl MenubarModel {
    pub fn new(menus: Vec<MenubarMenu>) -> Self {
        Self {
            density: MenubarDensity::Standard,
            default_open: menus.first().map(|menu| menu.value.clone()),
            menus,
            selected_value: None,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: MenubarDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_default_open(mut self, value: impl Into<String>) -> Self {
        self.default_open = Some(value.into());
        self
    }

    pub fn without_default_open(mut self) -> Self {
        self.default_open = None;
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

    pub const fn loading(mut self) -> Self {
        self.loading = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn state(&self) -> MenubarState {
        MenubarState::new(self.default_open.clone(), None, self.selected_value.clone())
    }
}

impl MenubarState {
    pub const fn new(
        open_menu: Option<String>,
        focused_value: Option<String>,
        selected_value: Option<String>,
    ) -> Self {
        Self {
            open_menu,
            focused_value,
            selected_value,
        }
    }

    pub const fn resting() -> Self {
        Self::new(None, None, None)
    }

    pub fn open_menu(&self) -> Option<&str> {
        self.open_menu.as_deref()
    }

    pub fn focused_value(&self) -> Option<&str> {
        self.focused_value.as_deref()
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    pub fn is_open(&self, value: &str) -> bool {
        self.open_menu.as_deref() == Some(value)
    }

    pub fn is_focused(&self, value: &str) -> bool {
        self.focused_value.as_deref() == Some(value)
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: MenubarIntent) -> MenubarChange {
        match intent {
            MenubarIntent::Open(value) => self.open(value),
            MenubarIntent::Close => self.close(),
            MenubarIntent::Toggle(value) => {
                if self.is_open(&value) {
                    self.close()
                } else {
                    self.open(value)
                }
            }
            MenubarIntent::FocusItem(value) => self.focus(value),
            MenubarIntent::Activate(value) => self.activate(value),
            MenubarIntent::Clear => self.clear(),
        }
    }

    fn open(&mut self, value: String) -> MenubarChange {
        if value.is_empty() || self.open_menu.as_ref() == Some(&value) {
            MenubarChange::Unchanged
        } else {
            self.open_menu = Some(value.clone());
            self.focused_value = None;
            MenubarChange::Opened(value)
        }
    }

    fn close(&mut self) -> MenubarChange {
        if self.open_menu.is_some() || self.focused_value.is_some() {
            self.open_menu = None;
            self.focused_value = None;
            MenubarChange::Closed
        } else {
            MenubarChange::Unchanged
        }
    }

    fn focus(&mut self, value: String) -> MenubarChange {
        if value.is_empty() || self.focused_value.as_ref() == Some(&value) {
            MenubarChange::Unchanged
        } else {
            self.focused_value = Some(value.clone());
            MenubarChange::Focused(value)
        }
    }

    fn activate(&mut self, value: String) -> MenubarChange {
        if value.is_empty() || self.selected_value.as_ref() == Some(&value) {
            MenubarChange::Unchanged
        } else {
            self.focused_value = Some(value.clone());
            self.selected_value = Some(value.clone());
            self.open_menu = None;
            MenubarChange::Activated(value)
        }
    }

    fn clear(&mut self) -> MenubarChange {
        if self.open_menu.is_none() && self.focused_value.is_none() && self.selected_value.is_none()
        {
            MenubarChange::Unchanged
        } else {
            self.open_menu = None;
            self.focused_value = None;
            self.selected_value = None;
            MenubarChange::Cleared
        }
    }
}

pub fn validate_menubar_model(model: &MenubarModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn menubar_layout_metrics(model: &MenubarModel, inline_size: f32) -> MenubarLayoutMetrics {
    let dense = model.density == MenubarDensity::Dense;
    let dense_root = dense && model.error.is_none() && !model.loading && !model.disabled;
    MenubarLayoutMetrics {
        max_width: scale::container::NARROW,
        root_padding: if dense_root {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        root_gap: if dense_root {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        row_gap: scale::space::xs2(inline_size),
        menu_gap: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        trigger_min_height: if dense {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        trigger_padding_inline: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        trigger_padding_block: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        trigger_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        emphasized_trigger_min_height: scale::space::FIELD,
        emphasized_trigger_padding_inline: scale::space::xs(inline_size),
        emphasized_trigger_padding_block: scale::space::xs2(inline_size),
        emphasized_trigger_font_size: scale::font_size::f0(inline_size),
        content_min_width: if dense {
            scale::space::xl3(inline_size)
        } else {
            scale::space::xl4(inline_size)
        },
        content_padding: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        content_gap: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        item_min_height: if dense {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        item_padding_inline: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        item_padding_block: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        item_gap: if dense {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        item_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        emphasized_item_min_height: scale::space::FIELD,
        emphasized_item_padding_inline: scale::space::xs(inline_size),
        emphasized_item_padding_block: scale::space::xs2(inline_size),
        emphasized_item_gap: scale::space::s(inline_size),
        emphasized_item_font_size: scale::font_size::f0(inline_size),
        shortcut_margin_inline: scale::space::s(inline_size),
        shortcut_font_size: scale::font_size::f00(inline_size),
        line_height: scale::line_height::LH0,
    }
}

pub fn menubar_render_nodes(model: &MenubarModel, state: &MenubarState) -> Vec<MenubarRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.disabled || model.loading;
    let selected = state.selected_value().unwrap_or("none").to_owned();
    let open_menu = state.open_menu().unwrap_or("none").to_owned();
    let item_count = model
        .menus
        .iter()
        .map(|menu| menu.items.len())
        .sum::<usize>();
    let mut nodes = Vec::with_capacity(model.menus.len().saturating_mul(3) + item_count + 1);

    nodes.push(MenubarRenderNode {
        part: MenubarPart::Root,
        menu_index: 0,
        item_index: 0,
        value: selected.clone(),
        label: "Menubar".to_owned(),
        detail: model
            .error
            .clone()
            .unwrap_or_else(|| format!("Open menu: {open_menu}")),
        shortcut: String::new(),
        menu_value: String::new(),
        menu_label: String::new(),
        density: model.density,
        open: open_menu != "none",
        focused: state.focused_value().is_some(),
        selected: selected != "none",
        invalid,
        visible: true,
        actionable: false,
        loading: model.loading,
        disabled: blocked,
    });

    for (menu_index, menu) in model.menus.iter().enumerate() {
        let open = state.is_open(&menu.value);
        let menu_blocked = blocked || menu.disabled;
        nodes.push(MenubarRenderNode {
            part: MenubarPart::Menu,
            menu_index,
            item_index: 0,
            value: menu.value.clone(),
            label: menu.label.clone(),
            detail: format!("{} menu items", menu.items.len()),
            shortcut: String::new(),
            menu_value: menu.value.clone(),
            menu_label: menu.label.clone(),
            density: model.density,
            open,
            focused: false,
            selected: false,
            invalid,
            visible: true,
            actionable: false,
            loading: model.loading,
            disabled: menu_blocked,
        });
        nodes.push(MenubarRenderNode {
            part: MenubarPart::Trigger,
            menu_index,
            item_index: 0,
            value: menu.value.clone(),
            label: menu.label.clone(),
            detail: if open {
                "Menu trigger open".to_owned()
            } else {
                "Menu trigger closed".to_owned()
            },
            shortcut: String::new(),
            menu_value: menu.value.clone(),
            menu_label: menu.label.clone(),
            density: model.density,
            open,
            focused: open,
            selected: open,
            invalid,
            visible: true,
            actionable: true,
            loading: model.loading,
            disabled: menu_blocked,
        });
        nodes.push(MenubarRenderNode {
            part: MenubarPart::Content,
            menu_index,
            item_index: 0,
            value: menu.value.clone(),
            label: format!("{} commands", menu.label),
            detail: if open {
                "Menu content visible".to_owned()
            } else {
                "Menu content hidden".to_owned()
            },
            shortcut: String::new(),
            menu_value: menu.value.clone(),
            menu_label: menu.label.clone(),
            density: model.density,
            open,
            focused: false,
            selected: false,
            invalid,
            visible: open,
            actionable: false,
            loading: model.loading,
            disabled: menu_blocked,
        });

        for (item_index, item) in menu.items.iter().enumerate() {
            let focused = state.is_focused(&item.value);
            let selected = state.is_selected(&item.value);
            nodes.push(MenubarRenderNode {
                part: MenubarPart::Item,
                menu_index,
                item_index,
                value: item.value.clone(),
                label: item.label.clone(),
                detail: item
                    .shortcut
                    .clone()
                    .unwrap_or_else(|| "Menu item".to_owned()),
                shortcut: item.shortcut.clone().unwrap_or_default(),
                menu_value: menu.value.clone(),
                menu_label: menu.label.clone(),
                density: model.density,
                open,
                focused,
                selected,
                invalid,
                visible: open,
                actionable: true,
                loading: model.loading,
                disabled: menu_blocked || item.disabled,
            });
        }
    }

    nodes
}

pub fn default_menubar_model() -> MenubarModel {
    MenubarModel::new(vec![
        MenubarMenu::new(
            "File",
            "file",
            vec![
                MenubarItem::new("New project", "new-project").with_shortcut("N"),
                MenubarItem::new("Import deck", "import-deck").with_shortcut("I"),
            ],
        ),
        MenubarMenu::new(
            "Edit",
            "edit",
            vec![
                MenubarItem::new("Undo", "undo").with_shortcut("Z"),
                MenubarItem::new("Redo", "redo").with_shortcut("R"),
            ],
        ),
    ])
}

fn menubar_menus_are_valid(menus: &Vec<MenubarMenu>, _context: &()) -> garde::Result {
    let mut menu_values = HashSet::with_capacity(menus.len());
    let mut item_values = HashSet::new();
    for menu in menus {
        if !menu_values.insert(menu.value.as_str()) {
            return Err(garde::Error::new("menubar menu values must be unique"));
        }
        for item in &menu.items {
            if !item_values.insert(item.value.as_str()) {
                return Err(garde::Error::new(
                    "menubar item values must be unique across menus",
                ));
            }
        }
    }
    Ok(())
}

fn menubar_menu_value_references_enabled_menu<'a>(
    menus: &'a [MenubarMenu],
    message: &'static str,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !menus
                .iter()
                .any(|menu| menu.value == *value && !menu.disabled)
        {
            return Err(garde::Error::new(message));
        }
        Ok(())
    }
}

fn menubar_item_value_references_enabled_item<'a>(
    menus: &'a [MenubarMenu],
    message: &'static str,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !menus
                .iter()
                .flat_map(|menu| menu.items.iter())
                .any(|item| item.value == *value && !item.disabled)
        {
            return Err(garde::Error::new(message));
        }
        Ok(())
    }
}

fn validate_optional_menubar_shortcut(shortcut: &Option<String>, _context: &()) -> garde::Result {
    if let Some(shortcut) = shortcut
        && (shortcut.is_empty() || shortcut.len() > 24)
    {
        return Err(garde::Error::new(
            "menubar shortcut must be 1..=24 characters",
        ));
    }
    Ok(())
}

fn validate_optional_menubar_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && (error.is_empty() || error.len() > 240)
    {
        return Err(garde::Error::new(
            "menubar optional error must be 1..=240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_menubar_model(&default_menubar_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_menus() {
        let model = MenubarModel::new(Vec::new());
        assert!(validate_menubar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_menu_items() {
        let model = MenubarModel::new(vec![MenubarMenu::new("File", "file", Vec::new())]);
        assert!(validate_menubar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_menu_values() {
        let model = MenubarModel::new(vec![
            MenubarMenu::new("File", "file", vec![MenubarItem::new("New", "new")]),
            MenubarMenu::new("Project", "file", vec![MenubarItem::new("Open", "open")]),
        ]);
        assert!(validate_menubar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_item_values() {
        let model = MenubarModel::new(vec![
            MenubarMenu::new("File", "file", vec![MenubarItem::new("New", "new")]),
            MenubarMenu::new("Project", "project", vec![MenubarItem::new("New", "new")]),
        ]);
        assert!(validate_menubar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_default_open_menu() {
        let model = MenubarModel::new(vec![
            MenubarMenu::new("File", "file", vec![MenubarItem::new("New", "new")]).disabled(),
        ]);
        assert!(validate_menubar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_item() {
        let model = default_menubar_model().with_selected_value("missing");
        assert!(validate_menubar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_shortcut() {
        let model = MenubarModel::new(vec![MenubarMenu::new(
            "File",
            "file",
            vec![MenubarItem::new("New", "new").with_shortcut("")],
        )]);
        assert!(validate_menubar_model(&model).is_err());
    }

    #[test]
    fn state_opens_focuses_activates_closes_and_clears() {
        let mut state = MenubarState::resting();
        assert_eq!(
            state.apply(MenubarIntent::Open("file".to_owned())),
            MenubarChange::Opened("file".to_owned())
        );
        assert!(state.is_open("file"));
        assert_eq!(
            state.apply(MenubarIntent::FocusItem("new-project".to_owned())),
            MenubarChange::Focused("new-project".to_owned())
        );
        assert!(state.is_focused("new-project"));
        assert_eq!(
            state.apply(MenubarIntent::Activate("new-project".to_owned())),
            MenubarChange::Activated("new-project".to_owned())
        );
        assert!(!state.is_open("file"));
        assert!(state.is_selected("new-project"));
        assert_eq!(
            state.apply(MenubarIntent::Toggle("edit".to_owned())),
            MenubarChange::Opened("edit".to_owned())
        );
        assert_eq!(state.apply(MenubarIntent::Close), MenubarChange::Closed);
        assert_eq!(state.apply(MenubarIntent::Clear), MenubarChange::Cleared);
        assert_eq!(state.selected_value(), None);
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_menubar_model();
        let nodes = menubar_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(MenubarPart::Root));
        for part in MenubarPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn closed_menu_keeps_hidden_content_and_items() {
        let model = default_menubar_model().without_default_open();
        let nodes = menubar_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| matches!(node.part, MenubarPart::Content | MenubarPart::Item))
                .all(|node| !node.visible)
        );
    }

    #[test]
    fn loading_disables_menu_items() {
        let model = default_menubar_model().loading();
        let nodes = menubar_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == MenubarPart::Item)
                .all(|node| node.disabled)
        );
    }

    #[test]
    fn item_nodes_keep_stable_indexes() {
        let model = default_menubar_model();
        let nodes = menubar_render_nodes(&model, &model.state());
        let new_project = nodes
            .iter()
            .find(|node| node.value == "new-project")
            .expect("default menubar includes new project");
        assert_eq!(new_project.menu_index, 0);
        assert_eq!(new_project.item_index, 0);
    }

    #[test]
    fn layout_metrics_preserve_dense_and_emphasis_precedence() {
        let standard = menubar_layout_metrics(&default_menubar_model(), 1_280.0);
        let dense = menubar_layout_metrics(
            &default_menubar_model().with_density(MenubarDensity::Dense),
            1_280.0,
        );
        let dense_loading = menubar_layout_metrics(
            &default_menubar_model()
                .with_density(MenubarDensity::Dense)
                .loading(),
            1_280.0,
        );

        assert!(dense.root_padding < standard.root_padding);
        assert!(dense.trigger_min_height < standard.trigger_min_height);
        assert!(dense.content_min_width < standard.content_min_width);
        assert!(dense.item_font_size < standard.item_font_size);
        assert_eq!(dense_loading.root_padding, standard.root_padding);
        assert_eq!(
            dense.emphasized_trigger_min_height,
            standard.trigger_min_height
        );
        assert_eq!(dense.emphasized_item_font_size, standard.item_font_size);
    }
}
