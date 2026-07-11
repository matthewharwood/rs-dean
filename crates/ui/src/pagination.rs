use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PaginationDensity {
    Standard,
    Dense,
}

impl PaginationDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PaginationPart {
    Root,
    Content,
    Item,
    Previous,
    Link,
    Next,
}

impl PaginationPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Content,
        Self::Item,
        Self::Previous,
        Self::Link,
        Self::Next,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Pagination",
            Self::Content => "PaginationContent",
            Self::Item => "PaginationItem",
            Self::Previous => "PaginationPrevious",
            Self::Link => "PaginationLink",
            Self::Next => "PaginationNext",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct PaginationModel {
    #[garde(skip)]
    pub density: PaginationDensity,
    #[garde(range(min = 1, max = 999))]
    pub page_count: u16,
    #[garde(custom(pagination_current_page_is_in_range(&self.page_count)))]
    pub current_page: u16,
    #[garde(range(max = 3))]
    pub sibling_count: u8,
    #[garde(length(min = 1, max = 96))]
    pub previous_label: String,
    #[garde(length(min = 1, max = 96))]
    pub next_label: String,
    #[garde(custom(validate_optional_pagination_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginationState {
    current_page: u16,
    focused_page: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaginationIntent {
    Previous,
    Next,
    GoTo(u16),
    Focus(u16),
    ClearFocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaginationChange {
    PageChanged(u16),
    Focused(u16),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginationRenderNode {
    pub part: PaginationPart,
    pub index: usize,
    pub page: u16,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: PaginationDensity,
    pub current: bool,
    pub focused: bool,
    pub edge: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaginationLayoutMetrics {
    pub max_width: f32,
    pub root_gap: f32,
    pub content_padding: f32,
    pub content_gap: f32,
    pub control_min_size: f32,
    pub control_padding_inline: f32,
    pub control_padding_block: f32,
    pub control_font_size: f32,
    pub emphasized_control_min_size: f32,
    pub emphasized_control_padding_inline: f32,
    pub emphasized_control_padding_block: f32,
    pub emphasized_control_font_size: f32,
    pub line_height: f32,
    pub error_font_size: f32,
}

impl PaginationModel {
    pub fn new(page_count: u16, current_page: u16) -> Self {
        Self {
            density: PaginationDensity::Standard,
            page_count,
            current_page,
            sibling_count: 1,
            previous_label: "Previous".to_owned(),
            next_label: "Next".to_owned(),
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: PaginationDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_sibling_count(mut self, sibling_count: u8) -> Self {
        self.sibling_count = sibling_count;
        self
    }

    pub fn with_previous_label(mut self, label: impl Into<String>) -> Self {
        self.previous_label = label.into();
        self
    }

    pub fn with_next_label(mut self, label: impl Into<String>) -> Self {
        self.next_label = label.into();
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

    pub const fn state(&self) -> PaginationState {
        PaginationState::new(self.current_page)
    }
}

impl PaginationState {
    pub const fn new(current_page: u16) -> Self {
        Self {
            current_page,
            focused_page: None,
        }
    }

    pub const fn current_page(&self) -> u16 {
        self.current_page
    }

    pub const fn focused_page(&self) -> Option<u16> {
        self.focused_page
    }

    pub const fn is_current(&self, page: u16) -> bool {
        self.current_page == page
    }

    pub const fn is_focused(&self, page: u16) -> bool {
        matches!(self.focused_page, Some(focused) if focused == page)
    }

    pub fn apply(&mut self, intent: PaginationIntent, page_count: u16) -> PaginationChange {
        match intent {
            PaginationIntent::Previous => {
                self.go_to(self.current_page.saturating_sub(1), page_count)
            }
            PaginationIntent::Next => self.go_to(self.current_page.saturating_add(1), page_count),
            PaginationIntent::GoTo(page) => self.go_to(page, page_count),
            PaginationIntent::Focus(page) => self.focus(page, page_count),
            PaginationIntent::ClearFocus => self.clear_focus(),
        }
    }

    fn go_to(&mut self, page: u16, page_count: u16) -> PaginationChange {
        if page == 0 || page > page_count || page == self.current_page {
            PaginationChange::Unchanged
        } else {
            self.current_page = page;
            self.focused_page = Some(page);
            PaginationChange::PageChanged(page)
        }
    }

    fn focus(&mut self, page: u16, page_count: u16) -> PaginationChange {
        if page == 0 || page > page_count || self.focused_page == Some(page) {
            PaginationChange::Unchanged
        } else {
            self.focused_page = Some(page);
            PaginationChange::Focused(page)
        }
    }

    fn clear_focus(&mut self) -> PaginationChange {
        if self.focused_page.is_some() {
            self.focused_page = None;
            PaginationChange::Cleared
        } else {
            PaginationChange::Unchanged
        }
    }
}

pub fn validate_pagination_model(model: &PaginationModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn pagination_layout_metrics(
    model: &PaginationModel,
    inline_size: f32,
) -> PaginationLayoutMetrics {
    let dense = model.density == PaginationDensity::Dense;
    let dense_content = dense && !model.loading;
    PaginationLayoutMetrics {
        max_width: scale::container::CONTENT,
        root_gap: scale::space::xs2(inline_size),
        content_padding: if dense_content {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        content_gap: if dense_content {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        control_min_size: if dense {
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
        emphasized_control_min_size: scale::space::FIELD,
        emphasized_control_padding_inline: scale::space::xs(inline_size),
        emphasized_control_padding_block: scale::space::xs2(inline_size),
        emphasized_control_font_size: scale::font_size::f0(inline_size),
        line_height: scale::line_height::LH0,
        error_font_size: scale::font_size::f00(inline_size),
    }
}

pub const fn pagination_control_uses_emphasized_metrics(
    current: bool,
    invalid: bool,
    disabled: bool,
) -> bool {
    (disabled && !current) || invalid
}

pub fn pagination_render_nodes(
    model: &PaginationModel,
    state: &PaginationState,
) -> Vec<PaginationRenderNode> {
    let blocked = model.loading || model.disabled;
    let invalid = model.error.is_some();
    let pages =
        visible_pagination_pages(model.page_count, state.current_page(), model.sibling_count);
    let mut nodes = Vec::with_capacity(pages.len().saturating_mul(2).saturating_add(4));
    nodes.push(pagination_node(
        model,
        state,
        PaginationNodeDraft {
            part: PaginationPart::Root,
            index: 0,
            page: state.current_page(),
            value: "pagination",
            label: "Pagination",
            detail: model.error.as_deref().unwrap_or("Collection pagination"),
            edge: false,
            visible: true,
            actionable: false,
            disabled: model.disabled,
        },
    ));
    nodes.push(pagination_node(
        model,
        state,
        PaginationNodeDraft {
            part: PaginationPart::Content,
            index: 0,
            page: state.current_page(),
            value: "pagination-content",
            label: "Pagination pages",
            detail: "Page controls",
            edge: false,
            visible: true,
            actionable: false,
            disabled: blocked,
        },
    ));
    nodes.push(pagination_node(
        model,
        state,
        PaginationNodeDraft {
            part: PaginationPart::Item,
            index: 0,
            page: state.current_page().saturating_sub(1),
            value: "previous-item",
            label: &model.previous_label,
            detail: "Previous page item",
            edge: true,
            visible: true,
            actionable: !blocked && state.current_page() > 1,
            disabled: blocked || state.current_page() <= 1,
        },
    ));
    nodes.push(pagination_node(
        model,
        state,
        PaginationNodeDraft {
            part: PaginationPart::Previous,
            index: 0,
            page: state.current_page().saturating_sub(1),
            value: "previous",
            label: &model.previous_label,
            detail: "Previous page",
            edge: true,
            visible: true,
            actionable: !blocked && state.current_page() > 1,
            disabled: blocked || state.current_page() <= 1,
        },
    ));
    for (index, page) in pages.iter().copied().enumerate() {
        let value = page.to_string();
        let label = format!("{page}");
        let detail = if state.is_current(page) {
            "Current page"
        } else {
            "Page"
        };
        nodes.push(pagination_node(
            model,
            state,
            PaginationNodeDraft {
                part: PaginationPart::Item,
                index: index.saturating_add(1),
                page,
                value: &value,
                label: &label,
                detail,
                edge: page == 1 || page == model.page_count,
                visible: true,
                actionable: !blocked && !state.is_current(page),
                disabled: blocked,
            },
        ));
        nodes.push(pagination_node(
            model,
            state,
            PaginationNodeDraft {
                part: PaginationPart::Link,
                index,
                page,
                value: &value,
                label: &label,
                detail,
                edge: page == 1 || page == model.page_count,
                visible: true,
                actionable: !blocked && !state.is_current(page),
                disabled: blocked,
            },
        ));
    }
    nodes.push(pagination_node(
        model,
        state,
        PaginationNodeDraft {
            part: PaginationPart::Item,
            index: pages.len().saturating_add(1),
            page: state.current_page().saturating_add(1),
            value: "next-item",
            label: &model.next_label,
            detail: "Next page item",
            edge: true,
            visible: true,
            actionable: !blocked && state.current_page() < model.page_count,
            disabled: blocked || state.current_page() >= model.page_count,
        },
    ));
    nodes.push(pagination_node(
        model,
        state,
        PaginationNodeDraft {
            part: PaginationPart::Next,
            index: pages.len().saturating_add(1),
            page: state.current_page().saturating_add(1),
            value: "next",
            label: &model.next_label,
            detail: "Next page",
            edge: true,
            visible: true,
            actionable: !blocked && state.current_page() < model.page_count,
            disabled: blocked || state.current_page() >= model.page_count,
        },
    ));
    if invalid && nodes.len() > 1 {
        nodes[1].disabled = blocked;
    }
    nodes
}

pub fn visible_pagination_pages(page_count: u16, current_page: u16, sibling_count: u8) -> Vec<u16> {
    if page_count == 0 {
        return Vec::new();
    }
    let mut pages = Vec::new();
    let sibling_count = u16::from(sibling_count);
    let start = current_page.saturating_sub(sibling_count).max(1);
    let end = current_page.saturating_add(sibling_count).min(page_count);
    pages.push(1);
    for page in start..=end {
        if !pages.contains(&page) {
            pages.push(page);
        }
    }
    if !pages.contains(&page_count) {
        pages.push(page_count);
    }
    pages.sort_unstable();
    pages
}

pub fn default_pagination_model() -> PaginationModel {
    PaginationModel::new(8, 3)
}

struct PaginationNodeDraft<'a> {
    part: PaginationPart,
    index: usize,
    page: u16,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    edge: bool,
    visible: bool,
    actionable: bool,
    disabled: bool,
}

fn pagination_node(
    model: &PaginationModel,
    state: &PaginationState,
    draft: PaginationNodeDraft<'_>,
) -> PaginationRenderNode {
    PaginationRenderNode {
        part: draft.part,
        index: draft.index,
        page: draft.page,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        current: state.is_current(draft.page),
        focused: state.is_focused(draft.page),
        edge: draft.edge,
        visible: draft.visible,
        actionable: draft.actionable,
        invalid: model.error.is_some(),
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn pagination_current_page_is_in_range(
    page_count: &u16,
) -> impl FnOnce(&u16, &()) -> garde::Result + '_ {
    move |current_page, _context| {
        if *current_page == 0 || *current_page > *page_count {
            return Err(garde::Error::new(
                "pagination current page must be within page count",
            ));
        }
        Ok(())
    }
}

fn validate_optional_pagination_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "pagination error must be 1..=240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_pagination_model(&default_pagination_model()).is_ok());
    }

    #[test]
    fn garde_rejects_zero_page_count() {
        let model = PaginationModel::new(0, 1);
        assert!(validate_pagination_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_current_page_out_of_range() {
        let model = PaginationModel::new(3, 4);
        assert!(validate_pagination_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_labels() {
        let model = default_pagination_model().with_previous_label("");
        assert!(validate_pagination_model(&model).is_err());
        let model = default_pagination_model().with_next_label("");
        assert!(validate_pagination_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_pagination_model().with_error("");
        assert!(validate_pagination_model(&model).is_err());
    }

    #[test]
    fn state_moves_between_pages_and_focuses() {
        let mut state = PaginationState::new(3);
        assert_eq!(
            state.apply(PaginationIntent::Previous, 8),
            PaginationChange::PageChanged(2)
        );
        assert_eq!(state.current_page(), 2);
        assert_eq!(
            state.apply(PaginationIntent::Next, 8),
            PaginationChange::PageChanged(3)
        );
        assert_eq!(
            state.apply(PaginationIntent::GoTo(8), 8),
            PaginationChange::PageChanged(8)
        );
        assert_eq!(
            state.apply(PaginationIntent::Next, 8),
            PaginationChange::Unchanged
        );
        assert_eq!(
            state.apply(PaginationIntent::Focus(4), 8),
            PaginationChange::Focused(4)
        );
        assert_eq!(state.focused_page(), Some(4));
        assert_eq!(
            state.apply(PaginationIntent::ClearFocus, 8),
            PaginationChange::Cleared
        );
    }

    #[test]
    fn visible_pages_include_edges_and_siblings() {
        assert_eq!(visible_pagination_pages(8, 4, 1), vec![1, 3, 4, 5, 8]);
        assert_eq!(visible_pagination_pages(3, 2, 1), vec![1, 2, 3]);
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_pagination_model();
        let nodes = pagination_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(PaginationPart::Root)
        );
        for part in PaginationPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == PaginationPart::Link)
                .count()
                >= 3
        );
    }

    #[test]
    fn first_page_disables_previous_and_last_page_disables_next() {
        let model = PaginationModel::new(4, 1);
        let nodes = pagination_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == PaginationPart::Previous && node.disabled)
        );

        let model = PaginationModel::new(4, 4);
        let nodes = pagination_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == PaginationPart::Next && node.disabled)
        );
    }

    #[test]
    fn loading_disables_page_links() {
        let model = default_pagination_model().loading();
        let nodes = pagination_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| {
                    matches!(
                        node.part,
                        PaginationPart::Previous | PaginationPart::Link | PaginationPart::Next
                    )
                })
                .all(|node| node.disabled)
        );
    }

    #[test]
    fn layout_metrics_preserve_dense_and_state_precedence() {
        let standard = pagination_layout_metrics(&default_pagination_model(), 1_000.0);
        let dense = pagination_layout_metrics(
            &default_pagination_model().with_density(PaginationDensity::Dense),
            1_000.0,
        );
        let dense_loading = pagination_layout_metrics(
            &default_pagination_model()
                .with_density(PaginationDensity::Dense)
                .loading(),
            1_000.0,
        );

        assert!(dense.content_padding < standard.content_padding);
        assert!(dense.control_min_size < standard.control_min_size);
        assert!(dense.control_font_size < standard.control_font_size);
        assert_eq!(dense_loading.content_padding, standard.content_padding);
        assert!(pagination_control_uses_emphasized_metrics(
            false, false, true
        ));
        assert!(pagination_control_uses_emphasized_metrics(
            true, true, false
        ));
        assert!(!pagination_control_uses_emphasized_metrics(
            true, false, true
        ));
    }
}
