use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScrollAreaDensity {
    Standard,
    Dense,
}

impl ScrollAreaDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScrollAreaOverflow {
    Vertical,
    Horizontal,
    Both,
}

impl ScrollAreaOverflow {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Both => "both",
        }
    }

    pub const fn has_vertical(self) -> bool {
        matches!(self, Self::Vertical | Self::Both)
    }

    pub const fn has_horizontal(self) -> bool {
        matches!(self, Self::Horizontal | Self::Both)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScrollAreaAxis {
    Vertical,
    Horizontal,
}

impl ScrollAreaAxis {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScrollAreaPart {
    Root,
    Viewport,
    Content,
    Bar,
    Corner,
}

impl ScrollAreaPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Viewport,
        Self::Content,
        Self::Bar,
        Self::Corner,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "ScrollArea",
            Self::Viewport => "ScrollViewport",
            Self::Content => "ScrollContent",
            Self::Bar => "ScrollBar",
            Self::Corner => "ScrollCorner",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ScrollAreaItem {
    #[garde(length(min = 1, max = 96))]
    pub title: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 320))]
    pub detail: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ScrollAreaModel {
    #[garde(skip)]
    pub density: ScrollAreaDensity,
    #[garde(skip)]
    pub overflow: ScrollAreaOverflow,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 32), dive, custom(scroll_area_items_are_unique))]
    pub items: Vec<ScrollAreaItem>,
    #[garde(custom(scroll_area_active_item_references_enabled_item(&self.items)))]
    pub active_item: Option<String>,
    #[garde(custom(validate_optional_scroll_area_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollAreaState {
    viewport_focused: bool,
    active_item: Option<String>,
    hovered_axis: Option<ScrollAreaAxis>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrollAreaIntent {
    FocusViewport,
    BlurViewport,
    HoverBar(ScrollAreaAxis),
    LeaveBar,
    ScrollTo(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrollAreaChange {
    ViewportFocused,
    ViewportBlurred,
    BarHovered(ScrollAreaAxis),
    BarLeft,
    ScrolledTo(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollAreaRenderNode {
    pub part: ScrollAreaPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: ScrollAreaDensity,
    pub overflow: ScrollAreaOverflow,
    pub axis: Option<ScrollAreaAxis>,
    pub active: bool,
    pub focused: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollAreaLayoutMetrics {
    pub max_width: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub header_gap: f32,
    pub title_font_size: f32,
    pub status_font_size: f32,
    pub frame_padding: f32,
    pub frame_gap: f32,
    pub viewport_max_height: f32,
    pub viewport_padding: f32,
    pub content_gap: f32,
    pub item_padding: f32,
    pub item_gap: f32,
    pub emphasized_item_padding: f32,
    pub emphasized_item_gap: f32,
    pub item_title_font_size: f32,
    pub disabled_item_title_font_size: f32,
    pub item_detail_font_size: f32,
    pub bar_row_gap: f32,
    pub vertical_bar_width: f32,
    pub vertical_bar_height: f32,
    pub horizontal_bar_width: f32,
    pub horizontal_bar_height: f32,
    pub corner_size: f32,
    pub line_height: f32,
}

impl ScrollAreaItem {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            detail: format!("{title} scroll item"),
            title,
            value: value.into(),
            disabled: false,
        }
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

impl ScrollAreaModel {
    pub fn new(items: Vec<ScrollAreaItem>) -> Self {
        Self {
            density: ScrollAreaDensity::Standard,
            overflow: ScrollAreaOverflow::Vertical,
            label: "Scroll area".to_owned(),
            items,
            active_item: None,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: ScrollAreaDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_overflow(mut self, overflow: ScrollAreaOverflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_active_item(mut self, value: impl Into<String>) -> Self {
        self.active_item = Some(value.into());
        self
    }

    pub fn without_active_item(mut self) -> Self {
        self.active_item = None;
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

    pub fn state(&self) -> ScrollAreaState {
        ScrollAreaState::new(self.active_item.clone())
    }
}

impl ScrollAreaState {
    pub const fn new(active_item: Option<String>) -> Self {
        Self {
            viewport_focused: false,
            active_item,
            hovered_axis: None,
        }
    }

    pub const fn is_viewport_focused(&self) -> bool {
        self.viewport_focused
    }

    pub fn active_item(&self) -> Option<&str> {
        self.active_item.as_deref()
    }

    pub const fn hovered_axis(&self) -> Option<ScrollAreaAxis> {
        self.hovered_axis
    }

    pub fn is_active_item(&self, value: &str) -> bool {
        self.active_item.as_deref() == Some(value)
    }

    pub fn is_hovering_axis(&self, axis: ScrollAreaAxis) -> bool {
        self.hovered_axis == Some(axis)
    }

    pub fn apply(&mut self, intent: ScrollAreaIntent) -> ScrollAreaChange {
        match intent {
            ScrollAreaIntent::FocusViewport => self.focus_viewport(),
            ScrollAreaIntent::BlurViewport => self.blur_viewport(),
            ScrollAreaIntent::HoverBar(axis) => self.hover_bar(axis),
            ScrollAreaIntent::LeaveBar => self.leave_bar(),
            ScrollAreaIntent::ScrollTo(value) => self.scroll_to(value),
            ScrollAreaIntent::Clear => self.clear(),
        }
    }

    fn focus_viewport(&mut self) -> ScrollAreaChange {
        if self.viewport_focused {
            ScrollAreaChange::Unchanged
        } else {
            self.viewport_focused = true;
            ScrollAreaChange::ViewportFocused
        }
    }

    fn blur_viewport(&mut self) -> ScrollAreaChange {
        if self.viewport_focused {
            self.viewport_focused = false;
            ScrollAreaChange::ViewportBlurred
        } else {
            ScrollAreaChange::Unchanged
        }
    }

    fn hover_bar(&mut self, axis: ScrollAreaAxis) -> ScrollAreaChange {
        if self.hovered_axis == Some(axis) {
            ScrollAreaChange::Unchanged
        } else {
            self.hovered_axis = Some(axis);
            ScrollAreaChange::BarHovered(axis)
        }
    }

    fn leave_bar(&mut self) -> ScrollAreaChange {
        if self.hovered_axis.take().is_some() {
            ScrollAreaChange::BarLeft
        } else {
            ScrollAreaChange::Unchanged
        }
    }

    fn scroll_to(&mut self, value: String) -> ScrollAreaChange {
        if value.is_empty() || self.active_item.as_ref() == Some(&value) {
            return ScrollAreaChange::Unchanged;
        }
        self.active_item = Some(value.clone());
        ScrollAreaChange::ScrolledTo(value)
    }

    fn clear(&mut self) -> ScrollAreaChange {
        if !self.viewport_focused && self.active_item.is_none() && self.hovered_axis.is_none() {
            return ScrollAreaChange::Unchanged;
        }
        self.viewport_focused = false;
        self.active_item = None;
        self.hovered_axis = None;
        ScrollAreaChange::Cleared
    }
}

pub fn validate_scroll_area_model(model: &ScrollAreaModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn scroll_area_layout_metrics(
    model: &ScrollAreaModel,
    inline_size: f32,
) -> ScrollAreaLayoutMetrics {
    let dense = model.density == ScrollAreaDensity::Dense;
    let dense_root = dense && model.error.is_none() && !model.disabled;
    let emphasized_viewport = model.error.is_some() || model.loading || model.disabled;
    ScrollAreaLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_padding: if dense_root {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        root_gap: if dense_root {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        header_gap: scale::space::xs2(inline_size),
        title_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        status_font_size: scale::font_size::f00(inline_size),
        frame_padding: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        frame_gap: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        viewport_max_height: if dense && !emphasized_viewport {
            scale::space::xl(inline_size)
        } else {
            scale::space::xl2(inline_size)
        },
        viewport_padding: if dense && !emphasized_viewport {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        content_gap: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        item_padding: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        item_gap: scale::space::xs3(inline_size),
        emphasized_item_padding: scale::space::xs(inline_size),
        emphasized_item_gap: scale::space::xs3(inline_size),
        item_title_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        disabled_item_title_font_size: scale::font_size::f0(inline_size),
        item_detail_font_size: scale::font_size::f00(inline_size),
        bar_row_gap: scale::space::xs2(inline_size),
        vertical_bar_width: scale::space::xs2(inline_size),
        vertical_bar_height: scale::space::m(inline_size),
        horizontal_bar_width: scale::space::m(inline_size),
        horizontal_bar_height: scale::space::xs2(inline_size),
        corner_size: scale::space::xs2(inline_size),
        line_height: scale::line_height::LH0,
    }
}

pub const fn scroll_area_item_uses_emphasized_metrics(
    active: bool,
    disabled: bool,
    invalid: bool,
) -> bool {
    active || disabled || invalid
}

pub fn scroll_area_render_nodes(
    model: &ScrollAreaModel,
    state: &ScrollAreaState,
) -> Vec<ScrollAreaRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let mut nodes = Vec::with_capacity(model.items.len().saturating_add(5));
    nodes.push(ScrollAreaRenderNode {
        part: ScrollAreaPart::Root,
        index: 0,
        value: state.active_item().unwrap_or("top").to_owned(),
        label: model.label.clone(),
        detail: model
            .error
            .clone()
            .unwrap_or_else(|| format!("{} scroll items", model.items.len())),
        density: model.density,
        overflow: model.overflow,
        axis: None,
        active: state.active_item().is_some(),
        focused: state.is_viewport_focused(),
        visible: true,
        actionable: false,
        invalid,
        loading: model.loading,
        disabled: model.disabled,
    });
    nodes.push(ScrollAreaRenderNode {
        part: ScrollAreaPart::Viewport,
        index: 0,
        value: state.active_item().unwrap_or("viewport").to_owned(),
        label: model.label.clone(),
        detail: "Scrollable viewport".to_owned(),
        density: model.density,
        overflow: model.overflow,
        axis: None,
        active: state.active_item().is_some(),
        focused: state.is_viewport_focused(),
        visible: true,
        actionable: !blocked,
        invalid,
        loading: model.loading,
        disabled: blocked,
    });
    for (index, item) in model.items.iter().enumerate() {
        let disabled = blocked || item.disabled;
        nodes.push(ScrollAreaRenderNode {
            part: ScrollAreaPart::Content,
            index,
            value: item.value.clone(),
            label: item.title.clone(),
            detail: item.detail.clone(),
            density: model.density,
            overflow: model.overflow,
            axis: None,
            active: state.is_active_item(&item.value),
            focused: state.is_viewport_focused(),
            visible: true,
            actionable: !disabled,
            invalid,
            loading: model.loading,
            disabled,
        });
    }
    if model.overflow.has_vertical() {
        nodes.push(scroll_area_bar_node(
            model,
            state,
            ScrollAreaAxis::Vertical,
            blocked,
            invalid,
        ));
    }
    if model.overflow.has_horizontal() {
        nodes.push(scroll_area_bar_node(
            model,
            state,
            ScrollAreaAxis::Horizontal,
            blocked,
            invalid,
        ));
    }
    nodes.push(ScrollAreaRenderNode {
        part: ScrollAreaPart::Corner,
        index: 0,
        value: "corner".to_owned(),
        label: "Scroll corner".to_owned(),
        detail: "Intersection of scrollbars".to_owned(),
        density: model.density,
        overflow: model.overflow,
        axis: None,
        active: state.hovered_axis().is_some(),
        focused: state.is_viewport_focused(),
        visible: model.overflow == ScrollAreaOverflow::Both,
        actionable: false,
        invalid,
        loading: model.loading,
        disabled: blocked,
    });
    nodes
}

pub fn default_scroll_area_model() -> ScrollAreaModel {
    ScrollAreaModel::new(default_scroll_area_items())
        .with_label("Activity")
        .with_active_item("sync")
}

pub fn default_scroll_area_items() -> Vec<ScrollAreaItem> {
    vec![
        ScrollAreaItem::new("Draft lesson", "draft").with_detail("Outline the next study session."),
        ScrollAreaItem::new("Sync state", "sync").with_detail("Persist local progress."),
        ScrollAreaItem::new("Review notes", "review").with_detail("Read previous corrections."),
        ScrollAreaItem::new("Publish", "publish").with_detail("Prepare the static artifact."),
    ]
}

fn scroll_area_bar_node(
    model: &ScrollAreaModel,
    state: &ScrollAreaState,
    axis: ScrollAreaAxis,
    blocked: bool,
    invalid: bool,
) -> ScrollAreaRenderNode {
    ScrollAreaRenderNode {
        part: ScrollAreaPart::Bar,
        index: match axis {
            ScrollAreaAxis::Vertical => 0,
            ScrollAreaAxis::Horizontal => 1,
        },
        value: axis.label().to_owned(),
        label: format!("{} scroll bar", axis.label()),
        detail: format!("{} overflow affordance", model.overflow.label()),
        density: model.density,
        overflow: model.overflow,
        axis: Some(axis),
        active: state.is_hovering_axis(axis),
        focused: state.is_viewport_focused(),
        visible: true,
        actionable: !blocked,
        invalid,
        loading: model.loading,
        disabled: blocked,
    }
}

fn scroll_area_items_are_unique(items: &Vec<ScrollAreaItem>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(items.len());
    for item in items {
        if !values.insert(item.value.as_str()) {
            return Err(garde::Error::new("scroll area item values must be unique"));
        }
    }
    Ok(())
}

fn scroll_area_active_item_references_enabled_item<'a>(
    items: &'a [ScrollAreaItem],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !items
                .iter()
                .any(|item| item.value == *value && !item.disabled)
        {
            return Err(garde::Error::new(
                "active scroll area item must reference an enabled item",
            ));
        }
        Ok(())
    }
}

fn validate_optional_scroll_area_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "scroll area error must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_scroll_area_model(&default_scroll_area_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_items() {
        let model = ScrollAreaModel::new(Vec::new());
        assert!(validate_scroll_area_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_item_values() {
        let model = ScrollAreaModel::new(vec![
            ScrollAreaItem::new("First", "item"),
            ScrollAreaItem::new("Second", "item"),
        ]);
        assert!(validate_scroll_area_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_active_item() {
        let model = ScrollAreaModel::new(vec![
            ScrollAreaItem::new("First", "first").disabled(),
            ScrollAreaItem::new("Second", "second"),
        ])
        .with_active_item("first");
        assert!(validate_scroll_area_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_scroll_area_model().with_error("");
        assert!(validate_scroll_area_model(&model).is_err());
    }

    #[test]
    fn state_tracks_viewport_bars_scroll_and_clear() {
        let mut state = ScrollAreaState::new(None);
        assert_eq!(
            state.apply(ScrollAreaIntent::FocusViewport),
            ScrollAreaChange::ViewportFocused
        );
        assert!(state.is_viewport_focused());
        assert_eq!(
            state.apply(ScrollAreaIntent::HoverBar(ScrollAreaAxis::Vertical)),
            ScrollAreaChange::BarHovered(ScrollAreaAxis::Vertical)
        );
        assert!(state.is_hovering_axis(ScrollAreaAxis::Vertical));
        assert_eq!(
            state.apply(ScrollAreaIntent::ScrollTo("review".to_owned())),
            ScrollAreaChange::ScrolledTo("review".to_owned())
        );
        assert!(state.is_active_item("review"));
        assert_eq!(
            state.apply(ScrollAreaIntent::Clear),
            ScrollAreaChange::Cleared
        );
        assert!(!state.is_viewport_focused());
        assert!(state.active_item().is_none());
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_scroll_area_model();
        let nodes = scroll_area_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(ScrollAreaPart::Root)
        );
        for part in ScrollAreaPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn horizontal_overflow_keeps_corner_hidden() {
        let model = default_scroll_area_model().with_overflow(ScrollAreaOverflow::Horizontal);
        let nodes = scroll_area_render_nodes(&model, &model.state());
        assert!(nodes.iter().any(|node| node.part == ScrollAreaPart::Bar
            && node.axis == Some(ScrollAreaAxis::Horizontal)));
        assert!(
            nodes
                .iter()
                .any(|node| node.part == ScrollAreaPart::Corner && !node.visible)
        );
    }

    #[test]
    fn loading_disables_viewport_and_bars() {
        let model = default_scroll_area_model().loading();
        let nodes = scroll_area_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| {
                    matches!(node.part, ScrollAreaPart::Viewport | ScrollAreaPart::Bar)
                })
                .all(|node| node.disabled && !node.actionable)
        );
    }

    #[test]
    fn layout_metrics_follow_density_and_shared_token_scales() {
        let standard = scroll_area_layout_metrics(&default_scroll_area_model(), 1_000.0);
        let dense = scroll_area_layout_metrics(
            &default_scroll_area_model().with_density(ScrollAreaDensity::Dense),
            1_000.0,
        );
        let loading_dense = scroll_area_layout_metrics(
            &default_scroll_area_model()
                .with_density(ScrollAreaDensity::Dense)
                .loading(),
            1_000.0,
        );

        assert!(dense.root_padding < standard.root_padding);
        assert!(dense.viewport_max_height < standard.viewport_max_height);
        assert_eq!(
            loading_dense.viewport_max_height,
            standard.viewport_max_height
        );
        assert_eq!(standard.line_height, scale::line_height::LH0);
        assert!(standard.vertical_bar_height > standard.vertical_bar_width);
        assert!(standard.horizontal_bar_width > standard.horizontal_bar_height);
    }

    #[test]
    fn emphasized_item_metrics_cover_css_state_precedence() {
        assert!(scroll_area_item_uses_emphasized_metrics(true, false, false));
        assert!(scroll_area_item_uses_emphasized_metrics(false, true, false));
        assert!(scroll_area_item_uses_emphasized_metrics(false, false, true));
        assert!(!scroll_area_item_uses_emphasized_metrics(
            false, false, false
        ));
    }
}
