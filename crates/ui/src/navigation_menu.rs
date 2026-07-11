use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NavigationMenuDensity {
    Standard,
    Dense,
}

impl NavigationMenuDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NavigationMenuPart {
    Root,
    List,
    Item,
    Trigger,
    Content,
    Link,
}

impl NavigationMenuPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::List,
        Self::Item,
        Self::Trigger,
        Self::Content,
        Self::Link,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "NavigationMenu",
            Self::List => "NavigationMenuList",
            Self::Item => "NavigationMenuItem",
            Self::Trigger => "NavigationMenuTrigger",
            Self::Content => "NavigationMenuContent",
            Self::Link => "NavigationMenuLink",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct NavigationMenuLink {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(custom(validate_navigation_menu_href))]
    pub href: String,
    #[garde(length(min = 1, max = 240))]
    pub description: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct NavigationMenuItem {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub description: String,
    #[garde(custom(validate_optional_navigation_menu_href))]
    pub href: Option<String>,
    #[garde(length(max = 12), dive)]
    pub links: Vec<NavigationMenuLink>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct NavigationMenuModel {
    #[garde(skip)]
    pub density: NavigationMenuDensity,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(
        length(min = 1, max = 8),
        dive,
        custom(navigation_menu_items_are_valid)
    )]
    pub items: Vec<NavigationMenuItem>,
    #[garde(custom(navigation_menu_panel_value_references_enabled_item(
        &self.items,
        "default open navigation item must reference an enabled panel item",
    )))]
    pub default_open: Option<String>,
    #[garde(custom(navigation_menu_value_references_enabled_target(
        &self.items,
        "selected navigation value must reference an enabled item or link",
    )))]
    pub selected_value: Option<String>,
    #[garde(custom(validate_optional_navigation_menu_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationMenuState {
    open_item: Option<String>,
    focused_value: Option<String>,
    selected_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationMenuIntent {
    Open(String),
    Close,
    Toggle(String),
    Focus(String),
    Navigate(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationMenuChange {
    Opened(String),
    Closed,
    Focused(String),
    Navigated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationMenuRenderNode {
    pub part: NavigationMenuPart,
    pub item_index: usize,
    pub link_index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub href: String,
    pub item_value: String,
    pub item_label: String,
    pub density: NavigationMenuDensity,
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
pub struct NavigationMenuLayoutMetrics {
    pub max_width: f32,
    pub list_padding: f32,
    pub list_gap: f32,
    pub item_gap: f32,
    pub panel_width: f32,
    pub control_min_height: f32,
    pub control_padding_inline: f32,
    pub control_padding_block: f32,
    pub control_font_size: f32,
    pub emphasized_control_min_height: f32,
    pub emphasized_control_padding_inline: f32,
    pub emphasized_control_padding_block: f32,
    pub emphasized_control_font_size: f32,
    pub content_min_width: f32,
    pub content_padding: f32,
    pub content_gap: f32,
    pub panel_link_padding: f32,
    pub emphasized_panel_link_padding: f32,
    pub panel_link_gap: f32,
    pub title_font_size: f32,
    pub title_line_height: f32,
    pub detail_font_size: f32,
    pub detail_line_height: f32,
}

impl NavigationMenuLayoutMetrics {
    pub fn panel_outer_width(self, border_width: f32) -> f32 {
        self.panel_width + border_width.max(0.0) * 4.0
    }
}

impl NavigationMenuLink {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        href: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            href: href.into(),
            description: description.into(),
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl NavigationMenuItem {
    pub fn link(
        label: impl Into<String>,
        value: impl Into<String>,
        href: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            href: Some(href.into()),
            description: description.into(),
            links: Vec::new(),
            disabled: false,
        }
    }

    pub fn panel(
        label: impl Into<String>,
        value: impl Into<String>,
        description: impl Into<String>,
        links: Vec<NavigationMenuLink>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: description.into(),
            href: None,
            links,
            disabled: false,
        }
    }

    pub fn with_links(mut self, links: Vec<NavigationMenuLink>) -> Self {
        self.links = links;
        self.href = None;
        self
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self.links = Vec::new();
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn is_panel(&self) -> bool {
        !self.links.is_empty()
    }
}

impl NavigationMenuModel {
    pub fn new(items: Vec<NavigationMenuItem>) -> Self {
        Self {
            density: NavigationMenuDensity::Standard,
            label: "Primary navigation".to_owned(),
            default_open: items
                .iter()
                .find(|item| item.is_panel())
                .map(|item| item.value.clone()),
            items,
            selected_value: None,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: NavigationMenuDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
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

    pub fn state(&self) -> NavigationMenuState {
        NavigationMenuState::new(self.default_open.clone(), None, self.selected_value.clone())
    }
}

impl NavigationMenuState {
    pub const fn new(
        open_item: Option<String>,
        focused_value: Option<String>,
        selected_value: Option<String>,
    ) -> Self {
        Self {
            open_item,
            focused_value,
            selected_value,
        }
    }

    pub const fn resting() -> Self {
        Self::new(None, None, None)
    }

    pub fn open_item(&self) -> Option<&str> {
        self.open_item.as_deref()
    }

    pub fn focused_value(&self) -> Option<&str> {
        self.focused_value.as_deref()
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    pub fn is_open(&self, value: &str) -> bool {
        self.open_item.as_deref() == Some(value)
    }

    pub fn is_focused(&self, value: &str) -> bool {
        self.focused_value.as_deref() == Some(value)
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: NavigationMenuIntent) -> NavigationMenuChange {
        match intent {
            NavigationMenuIntent::Open(value) => self.open(value),
            NavigationMenuIntent::Close => self.close(),
            NavigationMenuIntent::Toggle(value) => {
                if self.is_open(&value) {
                    self.close()
                } else {
                    self.open(value)
                }
            }
            NavigationMenuIntent::Focus(value) => self.focus(value),
            NavigationMenuIntent::Navigate(value) => self.navigate(value),
            NavigationMenuIntent::Clear => self.clear(),
        }
    }

    fn open(&mut self, value: String) -> NavigationMenuChange {
        if value.is_empty() || self.open_item.as_ref() == Some(&value) {
            NavigationMenuChange::Unchanged
        } else {
            self.open_item = Some(value.clone());
            self.focused_value = Some(value.clone());
            NavigationMenuChange::Opened(value)
        }
    }

    fn close(&mut self) -> NavigationMenuChange {
        if self.open_item.is_some() {
            self.open_item = None;
            NavigationMenuChange::Closed
        } else {
            NavigationMenuChange::Unchanged
        }
    }

    fn focus(&mut self, value: String) -> NavigationMenuChange {
        if value.is_empty() || self.focused_value.as_ref() == Some(&value) {
            NavigationMenuChange::Unchanged
        } else {
            self.focused_value = Some(value.clone());
            NavigationMenuChange::Focused(value)
        }
    }

    fn navigate(&mut self, value: String) -> NavigationMenuChange {
        if value.is_empty() {
            NavigationMenuChange::Unchanged
        } else {
            self.selected_value = Some(value.clone());
            self.focused_value = Some(value.clone());
            NavigationMenuChange::Navigated(value)
        }
    }

    fn clear(&mut self) -> NavigationMenuChange {
        if self.open_item.is_none() && self.focused_value.is_none() && self.selected_value.is_none()
        {
            NavigationMenuChange::Unchanged
        } else {
            self.open_item = None;
            self.focused_value = None;
            self.selected_value = None;
            NavigationMenuChange::Cleared
        }
    }
}

pub fn validate_navigation_menu_model(model: &NavigationMenuModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn navigation_menu_layout_metrics(
    model: &NavigationMenuModel,
    inline_size: f32,
) -> NavigationMenuLayoutMetrics {
    let dense = model.density == NavigationMenuDensity::Dense;
    let dense_list = dense && !model.loading;
    let emphasized = model.error.is_some() || model.loading || model.disabled;
    let dense_content = dense && !emphasized;
    let content_min_width = if dense_content {
        scale::space::xl3(inline_size)
    } else {
        scale::space::xl4(inline_size)
    };
    let content_padding = if dense_content {
        scale::space::xs(inline_size)
    } else {
        scale::space::s(inline_size)
    };
    let panel_link_padding = if dense {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let emphasized_panel_link_padding = scale::space::xs(inline_size);
    let title_font_size = scale::font_size::f0(inline_size);
    let detail_font_size = scale::font_size::f00(inline_size);
    let panel_content_width = model
        .items
        .iter()
        .filter(|item| item.is_panel())
        .flat_map(|item| {
            item.links.iter().map(|link| {
                let disabled = model.loading || model.disabled || item.disabled || link.disabled;
                let selected = model.selected_value.as_deref() == Some(link.value.as_str());
                let padding = if disabled || selected {
                    emphasized_panel_link_padding
                } else {
                    panel_link_padding
                };
                scale::estimate_inline_text_width(&link.label, title_font_size, 0.0)
                    .max(scale::estimate_inline_text_width(
                        &link.description,
                        detail_font_size,
                        0.0,
                    ))
                    .ceil()
                    + padding * 2.0
            })
        })
        .fold(0.0_f32, f32::max);
    // The two nested borders are added by `panel_outer_width` once their
    // renderer-specific width is known.
    let panel_width = content_min_width.max(panel_content_width + content_padding * 2.0);
    NavigationMenuLayoutMetrics {
        max_width: scale::container::NARROW,
        list_padding: if dense_list {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        list_gap: if dense_list {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        item_gap: scale::space::xs2(inline_size),
        panel_width,
        control_min_height: if dense {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        control_padding_inline: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        control_padding_block: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        control_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        emphasized_control_min_height: scale::space::FIELD,
        emphasized_control_padding_inline: scale::space::xs(inline_size),
        emphasized_control_padding_block: scale::space::xs2(inline_size),
        emphasized_control_font_size: scale::font_size::f0(inline_size),
        content_min_width,
        content_padding,
        content_gap: if dense_content {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        panel_link_padding,
        emphasized_panel_link_padding,
        panel_link_gap: scale::space::xs3(inline_size),
        title_font_size,
        title_line_height: scale::line_height::LH0,
        detail_font_size,
        detail_line_height: scale::line_height::LH0,
    }
}

pub fn navigation_menu_item_starts_new_row(
    model: &NavigationMenuModel,
    item_index: usize,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> bool {
    let metrics = navigation_menu_layout_metrics(model, inline_size);
    let border_width = border_width.max(0.0);
    let inner_width = (available_width - (metrics.list_padding + border_width) * 2.0).max(1.0);
    let mut row_width = 0.0;
    for (index, item) in model.items.iter().enumerate() {
        let disabled = model.loading || model.disabled || item.disabled;
        let emphasized = model.error.is_some() || disabled;
        let item_width = if item.is_panel() {
            metrics.panel_outer_width(border_width)
        } else {
            let font_size = if emphasized {
                metrics.emphasized_control_font_size
            } else {
                metrics.control_font_size
            };
            let padding = if emphasized {
                metrics.emphasized_control_padding_inline
            } else {
                metrics.control_padding_inline
            };
            scale::estimate_inline_text_width(&item.label, font_size, 0.0)
                + padding * 2.0
                + border_width * 2.0
        };
        let required = if row_width > 0.0 {
            metrics.list_gap + item_width
        } else {
            item_width
        };
        let starts_new_row = row_width > 0.0 && row_width + required > inner_width;
        if index == item_index {
            return starts_new_row;
        }
        row_width = if starts_new_row {
            item_width
        } else {
            row_width + required
        };
    }
    false
}

pub fn navigation_menu_render_nodes(
    model: &NavigationMenuModel,
    state: &NavigationMenuState,
) -> Vec<NavigationMenuRenderNode> {
    let blocked = model.loading || model.disabled;
    let invalid = model.error.is_some();
    let root_detail = model
        .error
        .clone()
        .unwrap_or_else(|| "Navigation menu".to_owned());
    let mut nodes = Vec::with_capacity(2 + model.items.len().saturating_mul(4));
    nodes.push(navigation_menu_node(
        model,
        state,
        NavigationMenuNodeDraft {
            part: NavigationMenuPart::Root,
            item_index: 0,
            link_index: 0,
            value: "navigation-menu",
            label: &model.label,
            detail: &root_detail,
            href: "",
            item_value: "",
            item_label: "",
            open: state.open_item().is_some(),
            visible: true,
            actionable: false,
            disabled: model.disabled,
        },
    ));
    nodes.push(navigation_menu_node(
        model,
        state,
        NavigationMenuNodeDraft {
            part: NavigationMenuPart::List,
            item_index: 0,
            link_index: 0,
            value: "navigation-menu-list",
            label: &model.label,
            detail: "Top-level navigation list",
            href: "",
            item_value: "",
            item_label: "",
            open: state.open_item().is_some(),
            visible: true,
            actionable: false,
            disabled: blocked,
        },
    ));

    for (item_index, item) in model.items.iter().enumerate() {
        let item_open = state.is_open(&item.value);
        let item_disabled = blocked || item.disabled;
        nodes.push(navigation_menu_node(
            model,
            state,
            NavigationMenuNodeDraft {
                part: NavigationMenuPart::Item,
                item_index,
                link_index: 0,
                value: &item.value,
                label: &item.label,
                detail: &item.description,
                href: item.href.as_deref().unwrap_or(""),
                item_value: &item.value,
                item_label: &item.label,
                open: item_open,
                visible: true,
                actionable: !item_disabled,
                disabled: item_disabled,
            },
        ));

        if item.is_panel() {
            nodes.push(navigation_menu_node(
                model,
                state,
                NavigationMenuNodeDraft {
                    part: NavigationMenuPart::Trigger,
                    item_index,
                    link_index: 0,
                    value: &item.value,
                    label: &item.label,
                    detail: &item.description,
                    href: "",
                    item_value: &item.value,
                    item_label: &item.label,
                    open: item_open,
                    visible: true,
                    actionable: !item_disabled,
                    disabled: item_disabled,
                },
            ));
            nodes.push(navigation_menu_node(
                model,
                state,
                NavigationMenuNodeDraft {
                    part: NavigationMenuPart::Content,
                    item_index,
                    link_index: 0,
                    value: &item.value,
                    label: &item.label,
                    detail: &item.description,
                    href: "",
                    item_value: &item.value,
                    item_label: &item.label,
                    open: item_open,
                    visible: item_open,
                    actionable: false,
                    disabled: item_disabled || !item_open,
                },
            ));
            for (link_index, link) in item.links.iter().enumerate() {
                nodes.push(navigation_menu_node(
                    model,
                    state,
                    NavigationMenuNodeDraft {
                        part: NavigationMenuPart::Link,
                        item_index,
                        link_index,
                        value: &link.value,
                        label: &link.label,
                        detail: &link.description,
                        href: &link.href,
                        item_value: &item.value,
                        item_label: &item.label,
                        open: item_open,
                        visible: item_open,
                        actionable: !item_disabled && !link.disabled,
                        disabled: item_disabled || link.disabled || !item_open,
                    },
                ));
            }
        } else {
            nodes.push(navigation_menu_node(
                model,
                state,
                NavigationMenuNodeDraft {
                    part: NavigationMenuPart::Link,
                    item_index,
                    link_index: 0,
                    value: &item.value,
                    label: &item.label,
                    detail: &item.description,
                    href: item.href.as_deref().unwrap_or(""),
                    item_value: &item.value,
                    item_label: &item.label,
                    open: false,
                    visible: true,
                    actionable: !item_disabled,
                    disabled: item_disabled || item.href.is_none(),
                },
            ));
        }
    }
    if invalid && nodes.len() > 1 {
        nodes[1].disabled = blocked;
    }
    nodes
}

pub fn default_navigation_menu_model() -> NavigationMenuModel {
    NavigationMenuModel::new(vec![
        NavigationMenuItem::link(
            "Docs",
            "docs",
            "/docs",
            "Read implementation guides and architectural notes.",
        ),
        NavigationMenuItem::panel(
            "Components",
            "components",
            "Browse token-only primitives and shared render contracts.",
            vec![
                NavigationMenuLink::new(
                    "Button",
                    "button",
                    "/components/button",
                    "Actions rendered from shared button tokens.",
                ),
                NavigationMenuLink::new(
                    "Navigation Menu",
                    "navigation-menu",
                    "/components/navigation-menu",
                    "Responsive navigation with local panel state.",
                ),
                NavigationMenuLink::new(
                    "Theme",
                    "theme",
                    "/components/theme",
                    "Shared palettes for Leptos and Bevy renderers.",
                ),
            ],
        ),
        NavigationMenuItem::link(
            "Stories",
            "stories",
            "/stories",
            "Open the story harness for reusable UI proofs.",
        ),
    ])
    .with_default_open("components")
    .with_selected_value("button")
}

struct NavigationMenuNodeDraft<'a> {
    part: NavigationMenuPart,
    item_index: usize,
    link_index: usize,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    href: &'a str,
    item_value: &'a str,
    item_label: &'a str,
    open: bool,
    visible: bool,
    actionable: bool,
    disabled: bool,
}

fn navigation_menu_node(
    model: &NavigationMenuModel,
    state: &NavigationMenuState,
    draft: NavigationMenuNodeDraft<'_>,
) -> NavigationMenuRenderNode {
    NavigationMenuRenderNode {
        part: draft.part,
        item_index: draft.item_index,
        link_index: draft.link_index,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        href: draft.href.to_owned(),
        item_value: draft.item_value.to_owned(),
        item_label: draft.item_label.to_owned(),
        density: model.density,
        open: draft.open,
        focused: state.is_focused(draft.value),
        selected: state.is_selected(draft.value),
        invalid: model.error.is_some(),
        visible: draft.visible,
        actionable: draft.actionable,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn navigation_menu_items_are_valid(
    items: &Vec<NavigationMenuItem>,
    _context: &(),
) -> garde::Result {
    let mut values = HashSet::with_capacity(items.len());
    for item in items {
        if !values.insert(item.value.as_str()) {
            return Err(garde::Error::new(
                "navigation menu item values must be unique",
            ));
        }
        if item.href.is_some() == item.is_panel() {
            return Err(garde::Error::new(
                "navigation menu items must be either a direct link or a panel trigger",
            ));
        }
        for link in &item.links {
            if !values.insert(link.value.as_str()) {
                return Err(garde::Error::new(
                    "navigation menu link values must be unique across the menu",
                ));
            }
        }
    }
    Ok(())
}

fn navigation_menu_panel_value_references_enabled_item<'a>(
    items: &'a [NavigationMenuItem],
    message: &'static str,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !items
                .iter()
                .any(|item| item.value == *value && item.is_panel() && !item.disabled)
        {
            return Err(garde::Error::new(message));
        }
        Ok(())
    }
}

fn navigation_menu_value_references_enabled_target<'a>(
    items: &'a [NavigationMenuItem],
    message: &'static str,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !navigation_menu_contains_enabled_value(items, value)
        {
            return Err(garde::Error::new(message));
        }
        Ok(())
    }
}

fn navigation_menu_contains_enabled_value(items: &[NavigationMenuItem], value: &str) -> bool {
    items.iter().any(|item| {
        if item.disabled {
            return false;
        }
        if item.value == value && item.href.is_some() {
            return true;
        }
        item.links
            .iter()
            .any(|link| link.value == value && !link.disabled)
    })
}

fn validate_optional_navigation_menu_href(href: &Option<String>, _context: &()) -> garde::Result {
    if let Some(href) = href {
        validate_navigation_menu_href(href, _context)?;
    }
    Ok(())
}

fn validate_navigation_menu_href(href: &str, _context: &()) -> garde::Result {
    if href.is_empty() || href.len() > 256 {
        return Err(garde::Error::new(
            "navigation menu href must be 1..=256 characters",
        ));
    }
    if !(href.starts_with('/') || href.starts_with('#') || href.starts_with("https://")) {
        return Err(garde::Error::new(
            "navigation menu href must be app-relative, hash, or https",
        ));
    }
    Ok(())
}

fn validate_optional_navigation_menu_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "navigation menu error must be 1..=240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_navigation_menu_model(&default_navigation_menu_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_items() {
        let model = NavigationMenuModel::new(Vec::new());
        assert!(validate_navigation_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_item_values() {
        let model = NavigationMenuModel::new(vec![
            NavigationMenuItem::link("Docs", "docs", "/docs", "Read docs."),
            NavigationMenuItem::link("Guides", "docs", "/guides", "Read guides."),
        ]);
        assert!(validate_navigation_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_link_values() {
        let model = NavigationMenuModel::new(vec![NavigationMenuItem::panel(
            "Components",
            "components",
            "Browse components.",
            vec![
                NavigationMenuLink::new("Button", "button", "/button", "Button docs."),
                NavigationMenuLink::new("Button copy", "button", "/button-copy", "Duplicate."),
            ],
        )]);
        assert!(validate_navigation_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_item_with_both_href_and_links() {
        let model = NavigationMenuModel::new(vec![
            NavigationMenuItem::link("Docs", "docs", "/docs", "Read docs.").with_links(vec![
                NavigationMenuLink::new("Intro", "intro", "/docs/intro", "Intro."),
            ]),
        ]);
        assert!(validate_navigation_menu_model(&model).is_ok());

        let mut invalid = NavigationMenuItem::panel(
            "Components",
            "components",
            "Browse components.",
            vec![NavigationMenuLink::new(
                "Button",
                "button",
                "/button",
                "Button docs.",
            )],
        );
        invalid.href = Some("/components".to_owned());
        let model = NavigationMenuModel::new(vec![invalid]);
        assert!(validate_navigation_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_href() {
        let model = NavigationMenuModel::new(vec![NavigationMenuItem::link(
            "Docs",
            "docs",
            "mailto:team@example.com",
            "Read docs.",
        )]);
        assert!(validate_navigation_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_default_open() {
        let model = NavigationMenuModel::new(vec![
            NavigationMenuItem::panel(
                "Components",
                "components",
                "Browse components.",
                vec![NavigationMenuLink::new(
                    "Button",
                    "button",
                    "/button",
                    "Button docs.",
                )],
            )
            .disabled(),
        ])
        .with_default_open("components");
        assert!(validate_navigation_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_value() {
        let model = default_navigation_menu_model().with_selected_value("unknown");
        assert!(validate_navigation_menu_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_navigation_menu_model().with_error("");
        assert!(validate_navigation_menu_model(&model).is_err());
    }

    #[test]
    fn state_opens_focuses_navigates_closes_and_clears() {
        let mut state = NavigationMenuState::resting();
        assert_eq!(
            state.apply(NavigationMenuIntent::Open("components".to_owned())),
            NavigationMenuChange::Opened("components".to_owned())
        );
        assert!(state.is_open("components"));
        assert_eq!(
            state.apply(NavigationMenuIntent::Focus("button".to_owned())),
            NavigationMenuChange::Focused("button".to_owned())
        );
        assert!(state.is_focused("button"));
        assert_eq!(
            state.apply(NavigationMenuIntent::Navigate("button".to_owned())),
            NavigationMenuChange::Navigated("button".to_owned())
        );
        assert_eq!(state.selected_value(), Some("button"));
        assert_eq!(
            state.apply(NavigationMenuIntent::Close),
            NavigationMenuChange::Closed
        );
        assert!(!state.is_open("components"));
        assert_eq!(
            state.apply(NavigationMenuIntent::Clear),
            NavigationMenuChange::Cleared
        );
        assert_eq!(state.selected_value(), None);
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_navigation_menu_model();
        let nodes = navigation_menu_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(NavigationMenuPart::Root)
        );
        for part in NavigationMenuPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == NavigationMenuPart::Link)
                .count(),
            5
        );
    }

    #[test]
    fn closed_panel_hides_content_links() {
        let model = default_navigation_menu_model().without_default_open();
        let nodes = navigation_menu_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == NavigationMenuPart::Content && !node.visible)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == NavigationMenuPart::Link
                    && node.item_value == "components"
                    && !node.visible)
        );
    }

    #[test]
    fn loading_disables_trigger_and_links() {
        let model = default_navigation_menu_model().loading();
        let nodes = navigation_menu_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == NavigationMenuPart::Trigger && node.disabled)
        );
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == NavigationMenuPart::Link)
                .all(|node| node.disabled)
        );
    }

    #[test]
    fn layout_metrics_preserve_dense_and_state_precedence() {
        let standard = navigation_menu_layout_metrics(&default_navigation_menu_model(), 1_280.0);
        let dense = navigation_menu_layout_metrics(
            &default_navigation_menu_model().with_density(NavigationMenuDensity::Dense),
            1_280.0,
        );
        let dense_loading = navigation_menu_layout_metrics(
            &default_navigation_menu_model()
                .with_density(NavigationMenuDensity::Dense)
                .loading(),
            1_280.0,
        );

        assert!(dense.list_padding < standard.list_padding);
        assert!(dense.control_min_height < standard.control_min_height);
        assert!(dense.panel_width < standard.panel_width);
        assert_eq!(standard.panel_outer_width(1.0), standard.panel_width + 4.0);
        assert!(dense.content_padding < standard.content_padding);
        assert_eq!(dense_loading.list_padding, standard.list_padding);
        assert_eq!(dense_loading.panel_width, standard.panel_width);
        assert_eq!(dense_loading.content_padding, standard.content_padding);

        let mut responsive = default_navigation_menu_model();
        responsive.items[1].links = vec![
            NavigationMenuLink::new(
                "Accordion",
                "accordion",
                "/components/accordion",
                "Disclosure state rendered through shared Rust nodes.",
            ),
            NavigationMenuLink::new(
                "Native Select",
                "native-select",
                "/components/native-select",
                "Browser-native select with shared option validation.",
            ),
            NavigationMenuLink::new(
                "Navigation Menu",
                "navigation-menu",
                "/components/navigation-menu",
                "Top-level menus with local panel state.",
            ),
        ];
        responsive.selected_value = Some("native-select".to_owned());
        assert!(navigation_menu_item_starts_new_row(
            &responsive,
            1,
            472.0,
            1_000.0,
            1.0,
        ));
        responsive.density = NavigationMenuDensity::Dense;
        responsive.selected_value = Some("docs".to_owned());
        assert!(!navigation_menu_item_starts_new_row(
            &responsive,
            1,
            472.0,
            1_000.0,
            1.0,
        ));
        assert!(navigation_menu_item_starts_new_row(
            &default_navigation_menu_model(),
            2,
            502.0,
            1_280.0,
            1.0,
        ));
        assert!(!navigation_menu_item_starts_new_row(
            &default_navigation_menu_model(),
            2,
            768.0,
            1_280.0,
            1.0,
        ));
    }
}
