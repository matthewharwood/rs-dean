use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SidebarDensity {
    Standard,
    Dense,
}

impl SidebarDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SidebarPart {
    Root,
    Header,
    Content,
    Group,
    Menu,
    Footer,
    Rail,
}

impl SidebarPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Header,
        Self::Content,
        Self::Group,
        Self::Menu,
        Self::Footer,
        Self::Rail,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Sidebar",
            Self::Header => "SidebarHeader",
            Self::Content => "SidebarContent",
            Self::Group => "SidebarGroup",
            Self::Menu => "SidebarMenu",
            Self::Footer => "SidebarFooter",
            Self::Rail => "SidebarRail",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SidebarItem {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(custom(validate_optional_sidebar_badge))]
    pub badge: Option<String>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SidebarGroup {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 12), dive)]
    pub items: Vec<SidebarItem>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SidebarModel {
    #[garde(skip)]
    pub density: SidebarDensity,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 96))]
    pub header_label: String,
    #[garde(length(min = 1, max = 240))]
    pub header_detail: String,
    #[garde(length(min = 1, max = 8), dive, custom(sidebar_groups_are_valid))]
    pub groups: Vec<SidebarGroup>,
    #[garde(custom(sidebar_active_value_references_enabled_item(&self.groups)))]
    pub active_value: Option<String>,
    #[garde(length(min = 1, max = 96))]
    pub footer_label: String,
    #[garde(length(min = 1, max = 240))]
    pub footer_detail: String,
    #[garde(length(min = 1, max = 96))]
    pub rail_label: String,
    #[garde(custom(validate_optional_sidebar_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub default_collapsed: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidebarState {
    collapsed: bool,
    focused_value: Option<String>,
    active_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidebarIntent {
    Expand,
    Collapse,
    ToggleCollapse,
    Focus(String),
    Activate(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidebarChange {
    Expanded,
    Collapsed,
    Focused(String),
    Activated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidebarRenderNode {
    pub part: SidebarPart,
    pub group_index: usize,
    pub item_index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub badge: String,
    pub density: SidebarDensity,
    pub collapsed: bool,
    pub focused: bool,
    pub selected: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarControlMetrics {
    pub min_height: f32,
    pub padding_inline: f32,
    pub padding_block: f32,
    pub font_size: f32,
    pub line_height: f32,
}

impl SidebarControlMetrics {
    pub fn content_min_height(self, border_width: f32) -> f32 {
        (self.min_height - self.padding_block * 2.0 - border_width.max(0.0) * 2.0).max(0.0)
    }

    pub fn outer_height(self, border_width: f32) -> f32 {
        self.min_height.max(
            self.font_size * self.line_height
                + self.padding_block * 2.0
                + border_width.max(0.0) * 2.0,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarLayoutMetrics {
    pub max_width: f32,
    pub rail_width: f32,
    pub rail_min_height: f32,
    pub rail_font_size: f32,
    pub panel_padding: f32,
    pub panel_gap: f32,
    pub header_gap: f32,
    pub header_padding_bottom: f32,
    pub title_font_size: f32,
    pub title_line_height: f32,
    pub detail_font_size: f32,
    pub detail_line_height: f32,
    pub content_gap: f32,
    pub group_gap: f32,
    pub group_label_font_size: f32,
    pub group_label_line_height: f32,
    pub group_label_tracking_em: f32,
    pub menu_gap: f32,
    pub menu_content_gap: f32,
    pub badge_padding_inline: f32,
    pub badge_padding_block: f32,
    pub badge_font_size: f32,
    pub badge_line_height: f32,
    pub footer_gap: f32,
    pub footer_padding_top: f32,
    pub footer_label_font_size: f32,
    pub footer_detail_font_size: f32,
    pub footer_line_height: f32,
    pub error_padding: f32,
    pub error_font_size: f32,
    pub error_line_height: f32,
    pub shadow_level: u8,
    standard_menu: SidebarControlMetrics,
    dense_menu: SidebarControlMetrics,
    dense: bool,
}

impl SidebarLayoutMetrics {
    pub const fn menu_control(
        self,
        active: bool,
        focused: bool,
        disabled: bool,
    ) -> SidebarControlMetrics {
        if sidebar_menu_uses_standard_metrics(self.dense, active, focused, disabled) {
            self.standard_menu
        } else {
            self.dense_menu
        }
    }
}

impl SidebarItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            badge: None,
            disabled: false,
        }
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl SidebarGroup {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        items: Vec<SidebarItem>,
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

impl SidebarModel {
    pub fn new(groups: Vec<SidebarGroup>) -> Self {
        Self {
            density: SidebarDensity::Standard,
            label: "Application navigation".to_owned(),
            header_label: "rs-dean".to_owned(),
            header_detail: "Rust/WASM workspace".to_owned(),
            groups,
            active_value: None,
            footer_label: "Local first".to_owned(),
            footer_detail: "Durable navigation state remains app-owned.".to_owned(),
            rail_label: "Toggle sidebar".to_owned(),
            error: None,
            default_collapsed: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SidebarDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_header(mut self, label: impl Into<String>, detail: impl Into<String>) -> Self {
        self.header_label = label.into();
        self.header_detail = detail.into();
        self
    }

    pub fn with_active_value(mut self, value: impl Into<String>) -> Self {
        self.active_value = Some(value.into());
        self
    }

    pub fn without_active_value(mut self) -> Self {
        self.active_value = None;
        self
    }

    pub fn with_footer(mut self, label: impl Into<String>, detail: impl Into<String>) -> Self {
        self.footer_label = label.into();
        self.footer_detail = detail.into();
        self
    }

    pub fn with_rail_label(mut self, label: impl Into<String>) -> Self {
        self.rail_label = label.into();
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

    pub const fn collapsed(mut self) -> Self {
        self.default_collapsed = true;
        self
    }

    pub const fn expanded(mut self) -> Self {
        self.default_collapsed = false;
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

    pub fn state(&self) -> SidebarState {
        SidebarState {
            collapsed: self.default_collapsed,
            focused_value: None,
            active_value: self.active_value.clone(),
        }
    }
}

impl SidebarState {
    pub const fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    pub fn focused_value(&self) -> Option<&str> {
        self.focused_value.as_deref()
    }

    pub fn active_value(&self) -> Option<&str> {
        self.active_value.as_deref()
    }

    pub fn is_focused(&self, value: &str) -> bool {
        self.focused_value.as_deref() == Some(value)
    }

    pub fn is_active(&self, value: &str) -> bool {
        self.active_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: SidebarIntent) -> SidebarChange {
        match intent {
            SidebarIntent::Expand => self.expand(),
            SidebarIntent::Collapse => self.collapse(),
            SidebarIntent::ToggleCollapse => {
                if self.collapsed {
                    self.expand()
                } else {
                    self.collapse()
                }
            }
            SidebarIntent::Focus(value) => self.focus(value),
            SidebarIntent::Activate(value) => self.activate(value),
            SidebarIntent::Clear => self.clear(),
        }
    }

    fn expand(&mut self) -> SidebarChange {
        if self.collapsed {
            self.collapsed = false;
            SidebarChange::Expanded
        } else {
            SidebarChange::Unchanged
        }
    }

    fn collapse(&mut self) -> SidebarChange {
        if self.collapsed {
            SidebarChange::Unchanged
        } else {
            self.collapsed = true;
            SidebarChange::Collapsed
        }
    }

    fn focus(&mut self, value: String) -> SidebarChange {
        if value.is_empty() || self.focused_value.as_ref() == Some(&value) {
            SidebarChange::Unchanged
        } else {
            self.focused_value = Some(value.clone());
            SidebarChange::Focused(value)
        }
    }

    fn activate(&mut self, value: String) -> SidebarChange {
        if value.is_empty() || self.active_value.as_ref() == Some(&value) {
            SidebarChange::Unchanged
        } else {
            self.active_value = Some(value.clone());
            SidebarChange::Activated(value)
        }
    }

    fn clear(&mut self) -> SidebarChange {
        if self.focused_value.is_some() || self.active_value.is_some() {
            self.focused_value = None;
            self.active_value = None;
            SidebarChange::Cleared
        } else {
            SidebarChange::Unchanged
        }
    }
}

pub fn validate_sidebar_model(model: &SidebarModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn sidebar_layout_metrics(model: &SidebarModel, inline_size: f32) -> SidebarLayoutMetrics {
    let dense = model.density == SidebarDensity::Dense;
    let panel_space = if dense {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let standard_menu = SidebarControlMetrics {
        min_height: scale::space::FIELD,
        padding_inline: scale::space::xs(inline_size),
        padding_block: scale::space::xs2(inline_size),
        font_size: scale::font_size::f0(inline_size),
        line_height: scale::line_height::LH0,
    };
    let dense_menu = SidebarControlMetrics {
        min_height: scale::space::s(inline_size),
        padding_inline: scale::space::xs2(inline_size),
        padding_block: scale::space::xs3(inline_size),
        font_size: scale::font_size::f00(inline_size),
        line_height: scale::line_height::LH0,
    };

    SidebarLayoutMetrics {
        max_width: scale::container::CONTROL,
        rail_width: scale::space::s(inline_size),
        rail_min_height: scale::space::xl(inline_size),
        rail_font_size: scale::font_size::f00(inline_size),
        panel_padding: panel_space,
        panel_gap: panel_space,
        header_gap: scale::space::xs3(inline_size),
        header_padding_bottom: panel_space,
        title_font_size: if dense {
            scale::font_size::f0(inline_size)
        } else {
            scale::font_size::f1(inline_size)
        },
        title_line_height: if dense {
            scale::line_height::LH0
        } else {
            scale::line_height::LH2
        },
        detail_font_size: scale::font_size::f00(inline_size),
        detail_line_height: scale::line_height::LH0,
        content_gap: panel_space,
        group_gap: scale::space::xs3(inline_size),
        group_label_font_size: scale::font_size::f00(inline_size),
        group_label_line_height: scale::line_height::LH0,
        group_label_tracking_em: 0.08,
        menu_gap: panel_space,
        menu_content_gap: scale::space::xs2(inline_size),
        badge_padding_inline: scale::space::xs2(inline_size),
        badge_padding_block: scale::space::xs3(inline_size),
        badge_font_size: scale::font_size::f00(inline_size),
        badge_line_height: scale::line_height::LH0,
        footer_gap: scale::space::xs3(inline_size),
        footer_padding_top: scale::space::xs(inline_size),
        footer_label_font_size: scale::font_size::f00(inline_size),
        footer_detail_font_size: scale::font_size::f00(inline_size),
        footer_line_height: scale::line_height::LH0,
        error_padding: scale::space::s(inline_size),
        error_font_size: scale::font_size::f0(inline_size),
        error_line_height: scale::line_height::LH0,
        shadow_level: 1,
        standard_menu,
        dense_menu,
        dense,
    }
}

pub const fn sidebar_menu_uses_standard_metrics(
    dense: bool,
    active: bool,
    focused: bool,
    disabled: bool,
) -> bool {
    !dense || active || focused || disabled
}

#[derive(Debug, Clone)]
struct SidebarNodeDraft {
    part: SidebarPart,
    group_index: usize,
    item_index: usize,
    value: String,
    label: String,
    detail: String,
    badge: String,
    actionable: bool,
    visible: bool,
    selected: bool,
    invalid: bool,
    disabled: bool,
}

impl SidebarNodeDraft {
    fn new(
        part: SidebarPart,
        value: impl Into<String>,
        label: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            part,
            group_index: 0,
            item_index: 0,
            value: value.into(),
            label: label.into(),
            detail: detail.into(),
            badge: String::new(),
            actionable: false,
            visible: true,
            selected: false,
            invalid: false,
            disabled: false,
        }
    }

    const fn with_index(mut self, group_index: usize, item_index: usize) -> Self {
        self.group_index = group_index;
        self.item_index = item_index;
        self
    }

    fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = badge.into();
        self
    }

    const fn actionable(mut self, actionable: bool) -> Self {
        self.actionable = actionable;
        self
    }

    const fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
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

pub fn sidebar_render_nodes(model: &SidebarModel, state: &SidebarState) -> Vec<SidebarRenderNode> {
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let mut nodes = Vec::with_capacity(
        model
            .groups
            .iter()
            .map(|group| group.items.len().saturating_add(1))
            .sum::<usize>()
            .saturating_add(4),
    );
    nodes.push(sidebar_node(
        SidebarNodeDraft::new(
            SidebarPart::Root,
            model.label.clone(),
            model.label.clone(),
            model
                .error
                .clone()
                .unwrap_or_else(|| model.header_detail.clone()),
        )
        .invalid(invalid)
        .disabled(blocked),
        model,
        state,
    ));
    nodes.push(sidebar_node(
        SidebarNodeDraft::new(
            SidebarPart::Header,
            model.header_label.clone(),
            model.header_label.clone(),
            model.header_detail.clone(),
        )
        .invalid(invalid)
        .disabled(blocked),
        model,
        state,
    ));
    nodes.push(sidebar_node(
        SidebarNodeDraft::new(
            SidebarPart::Content,
            "content",
            "Sidebar content",
            "Grouped navigation menu content",
        )
        .invalid(invalid)
        .disabled(blocked),
        model,
        state,
    ));
    for (group_index, group) in model.groups.iter().enumerate() {
        let group_blocked = blocked || group.disabled;
        nodes.push(sidebar_node(
            SidebarNodeDraft::new(
                SidebarPart::Group,
                group.value.clone(),
                group.label.clone(),
                format!("{} navigation group", group.label),
            )
            .with_index(group_index, 0)
            .invalid(invalid)
            .disabled(group_blocked),
            model,
            state,
        ));
        for (item_index, item) in group.items.iter().enumerate() {
            let disabled = group_blocked || item.disabled;
            nodes.push(sidebar_node(
                SidebarNodeDraft::new(
                    SidebarPart::Menu,
                    item.value.clone(),
                    item.label.clone(),
                    format!("{} navigation item", item.label),
                )
                .with_index(group_index, item_index)
                .with_badge(item.badge.clone().unwrap_or_default())
                .actionable(!disabled)
                .selected(state.is_active(&item.value))
                .invalid(invalid)
                .disabled(disabled),
                model,
                state,
            ));
        }
    }
    nodes.push(sidebar_node(
        SidebarNodeDraft::new(
            SidebarPart::Footer,
            model.footer_label.clone(),
            model.footer_label.clone(),
            model.footer_detail.clone(),
        )
        .with_index(model.groups.len(), 0)
        .visible(!state.is_collapsed())
        .invalid(invalid)
        .disabled(blocked),
        model,
        state,
    ));
    nodes.push(sidebar_node(
        SidebarNodeDraft::new(
            SidebarPart::Rail,
            model.rail_label.clone(),
            model.rail_label.clone(),
            if state.is_collapsed() {
                "Expand sidebar"
            } else {
                "Collapse sidebar"
            },
        )
        .with_index(model.groups.len(), 0)
        .actionable(!blocked)
        .selected(state.is_collapsed())
        .invalid(invalid)
        .disabled(blocked),
        model,
        state,
    ));
    nodes
}

fn sidebar_node(
    draft: SidebarNodeDraft,
    model: &SidebarModel,
    state: &SidebarState,
) -> SidebarRenderNode {
    let focused = state.is_focused(&draft.value);
    SidebarRenderNode {
        part: draft.part,
        group_index: draft.group_index,
        item_index: draft.item_index,
        value: draft.value,
        label: draft.label,
        detail: draft.detail,
        badge: draft.badge,
        density: model.density,
        collapsed: state.is_collapsed(),
        focused,
        selected: draft.selected,
        visible: draft.visible,
        actionable: draft.actionable,
        invalid: draft.invalid,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

pub fn default_sidebar_groups() -> Vec<SidebarGroup> {
    vec![
        SidebarGroup::new(
            "Workspace",
            "workspace",
            vec![
                SidebarItem::new("Overview", "overview"),
                SidebarItem::new("Stories", "stories").with_badge("64"),
                SidebarItem::new("Gate", "gate"),
            ],
        ),
        SidebarGroup::new(
            "Build",
            "build",
            vec![
                SidebarItem::new("Components", "components"),
                SidebarItem::new("Themes", "themes"),
                SidebarItem::new("Bevy scenes", "bevy"),
            ],
        ),
    ]
}

pub fn default_sidebar_model() -> SidebarModel {
    SidebarModel::new(default_sidebar_groups()).with_active_value("overview")
}

fn validate_optional_sidebar_badge(value: &Option<String>, _: &()) -> garde::Result {
    validate_optional_sidebar_text(value, 24, "sidebar badge")
}

fn validate_optional_sidebar_error(value: &Option<String>, _: &()) -> garde::Result {
    validate_optional_sidebar_text(value, 240, "sidebar error")
}

fn validate_optional_sidebar_text(
    value: &Option<String>,
    max: usize,
    field: &str,
) -> garde::Result {
    if let Some(value) = value
        && (value.is_empty() || value.len() > max)
    {
        return Err(garde::Error::new(format!(
            "{field} must be between 1 and {max} characters when present"
        )));
    }
    Ok(())
}

fn sidebar_groups_are_valid(groups: &Vec<SidebarGroup>, _: &()) -> garde::Result {
    let mut group_values = HashSet::with_capacity(groups.len());
    let mut item_values = HashSet::new();
    for group in groups {
        if !group_values.insert(group.value.as_str()) {
            return Err(garde::Error::new("sidebar group values must be unique"));
        }
        for item in &group.items {
            if !item_values.insert(item.value.as_str()) {
                return Err(garde::Error::new("sidebar item values must be unique"));
            }
        }
    }
    Ok(())
}

fn sidebar_active_value_references_enabled_item(
    groups: &[SidebarGroup],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + '_ {
    move |value, _| {
        let Some(value) = value.as_deref() else {
            return Ok(());
        };
        let found = groups.iter().any(|group| {
            !group.disabled
                && group
                    .items
                    .iter()
                    .any(|item| !item.disabled && item.value == value)
        });
        if found {
            Ok(())
        } else {
            Err(garde::Error::new(
                "active sidebar value must reference an enabled item",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_sidebar_model(&default_sidebar_model()).is_ok());
    }

    #[test]
    fn garde_rejects_duplicate_group_values() {
        let model = SidebarModel::new(vec![
            SidebarGroup::new("One", "same", vec![SidebarItem::new("A", "a")]),
            SidebarGroup::new("Two", "same", vec![SidebarItem::new("B", "b")]),
        ]);
        assert!(validate_sidebar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_item_values() {
        let model = SidebarModel::new(vec![SidebarGroup::new(
            "One",
            "one",
            vec![SidebarItem::new("A", "same"), SidebarItem::new("B", "same")],
        )]);
        assert!(validate_sidebar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_groups() {
        let model = SidebarModel::new(Vec::new());
        assert!(validate_sidebar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_active_value() {
        let model = SidebarModel::new(vec![SidebarGroup::new(
            "One",
            "one",
            vec![SidebarItem::new("A", "a").disabled()],
        )])
        .with_active_value("a");
        assert!(validate_sidebar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_sidebar_model().with_error("");
        assert!(validate_sidebar_model(&model).is_err());
    }

    #[test]
    fn layout_metrics_follow_fluid_tokens_and_dense_state_precedence() {
        let standard = default_sidebar_model();
        let dense = default_sidebar_model().with_density(SidebarDensity::Dense);
        let standard_metrics = sidebar_layout_metrics(&standard, 1_000.0);
        let dense_metrics = sidebar_layout_metrics(&dense, 1_000.0);
        let dense_resting = dense_metrics.menu_control(false, false, false);
        let dense_active = dense_metrics.menu_control(true, false, false);

        assert_eq!(standard_metrics.max_width, scale::container::CONTROL);
        assert_eq!(standard_metrics.rail_width, scale::space::s(1_000.0));
        assert!(dense_metrics.panel_padding < standard_metrics.panel_padding);
        assert!(dense_resting.min_height < dense_active.min_height);
        assert_eq!(
            dense_active,
            standard_metrics.menu_control(false, false, false)
        );
        assert_eq!(dense_metrics.menu_control(false, true, false), dense_active);
        assert_eq!(dense_metrics.menu_control(false, false, true), dense_active);
    }

    #[test]
    fn menu_outer_height_accounts_for_runtime_theme_border_width() {
        let metrics = sidebar_layout_metrics(&default_sidebar_model(), 1_000.0)
            .menu_control(false, false, false);
        let thin = metrics.outer_height(1.0);
        let thick = metrics.outer_height(2.0);

        assert_eq!(
            thin,
            metrics.font_size * metrics.line_height + metrics.padding_block * 2.0 + 2.0
        );
        assert_eq!(thick - thin, 2.0);
    }

    #[test]
    fn render_nodes_cover_repeatable_sidebar_anatomy() {
        let model = default_sidebar_model();
        let state = model.state();
        let nodes = sidebar_render_nodes(&model, &state);

        assert_eq!(nodes.first().map(|node| node.part), Some(SidebarPart::Root));
        for part in SidebarPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn collapsed_state_hides_footer_but_keeps_rail() {
        let model = default_sidebar_model().collapsed();
        let state = model.state();
        let nodes = sidebar_render_nodes(&model, &state);
        let footer = nodes
            .iter()
            .find(|node| node.part == SidebarPart::Footer)
            .expect("footer node should exist");
        let rail = nodes
            .iter()
            .find(|node| node.part == SidebarPart::Rail)
            .expect("rail node should exist");

        assert!(!footer.visible);
        assert!(rail.visible);
        assert!(rail.selected);
    }

    #[test]
    fn state_toggles_focuses_activates_and_clears() {
        let model = default_sidebar_model();
        let mut state = model.state();

        assert!(!state.is_collapsed());
        assert_eq!(
            state.apply(SidebarIntent::ToggleCollapse),
            SidebarChange::Collapsed
        );
        assert!(state.is_collapsed());
        assert_eq!(
            state.apply(SidebarIntent::Focus("gate".to_owned())),
            SidebarChange::Focused("gate".to_owned())
        );
        assert_eq!(
            state.apply(SidebarIntent::Activate("gate".to_owned())),
            SidebarChange::Activated("gate".to_owned())
        );
        assert_eq!(state.active_value(), Some("gate"));
        assert_eq!(state.apply(SidebarIntent::Clear), SidebarChange::Cleared);
        assert_eq!(state.active_value(), None);
    }
}
