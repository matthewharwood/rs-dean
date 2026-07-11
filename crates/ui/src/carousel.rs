use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CarouselDensity {
    Standard,
    Dense,
}

impl CarouselDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl CarouselPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Content,
        Self::Item,
        Self::Previous,
        Self::Next,
        Self::Indicator,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Carousel",
            Self::Content => "CarouselContent",
            Self::Item => "CarouselItem",
            Self::Previous => "CarouselPrevious",
            Self::Next => "CarouselNext",
            Self::Indicator => "CarouselIndicator",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CarouselSlide {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 600))]
    pub detail: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CarouselModel {
    #[garde(skip)]
    pub density: CarouselDensity,
    #[garde(length(min = 1, max = 12), dive, custom(carousel_slides_are_unique))]
    pub slides: Vec<CarouselSlide>,
    #[garde(custom(carousel_index_in_bounds(&self.slides)))]
    pub default_index: usize,
    #[garde(skip)]
    pub looping: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CarouselState {
    current_index: usize,
    total: usize,
    looping: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarouselIntent {
    Previous,
    Next,
    Select(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarouselChange {
    Moved { from: usize, to: usize },
    Selected(usize),
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CarouselPart {
    Root,
    Content,
    Item,
    Previous,
    Next,
    Indicator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarouselRenderNode {
    pub part: CarouselPart,
    pub index: usize,
    pub total: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: CarouselDensity,
    pub selected: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarouselLayoutMetrics {
    pub max_width: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub content_gap: f32,
    pub item_min_height: f32,
    pub item_padding: f32,
    pub item_gap: f32,
    pub title_font_size: f32,
    pub title_line_height: f32,
    pub detail_font_size: f32,
    pub detail_line_height: f32,
    pub controls_gap: f32,
    pub nav_size: f32,
    pub nav_font_size: f32,
    pub nav_line_height: f32,
    pub indicator_padding_inline: f32,
    pub indicator_padding_block: f32,
    pub indicator_font_size: f32,
    pub indicator_line_height: f32,
}

impl CarouselSlide {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            detail: detail.into(),
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl CarouselModel {
    pub fn new(slides: Vec<CarouselSlide>) -> Self {
        Self {
            density: CarouselDensity::Standard,
            slides,
            default_index: 0,
            looping: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: CarouselDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_default_index(mut self, default_index: usize) -> Self {
        self.default_index = default_index;
        self
    }

    pub const fn looping(mut self) -> Self {
        self.looping = true;
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

    pub const fn state(&self) -> CarouselState {
        CarouselState::new(self.default_index, self.slides.len(), self.looping)
    }
}

impl CarouselState {
    pub const fn new(current_index: usize, total: usize, looping: bool) -> Self {
        Self {
            current_index: if total == 0 || current_index >= total {
                0
            } else {
                current_index
            },
            total,
            looping,
        }
    }

    pub const fn current_index(self) -> usize {
        self.current_index
    }

    pub const fn total(self) -> usize {
        self.total
    }

    pub const fn is_selected(self, index: usize) -> bool {
        self.current_index == index
    }

    pub fn apply(&mut self, intent: CarouselIntent) -> CarouselChange {
        match intent {
            CarouselIntent::Previous => self.previous(),
            CarouselIntent::Next => self.next(),
            CarouselIntent::Select(index) => self.select(index),
        }
    }

    fn previous(&mut self) -> CarouselChange {
        if self.total <= 1 {
            return CarouselChange::Unchanged;
        }
        let from = self.current_index;
        let Some(to) = previous_index(from, self.total, self.looping) else {
            return CarouselChange::Unchanged;
        };
        self.current_index = to;
        CarouselChange::Moved { from, to }
    }

    fn next(&mut self) -> CarouselChange {
        if self.total <= 1 {
            return CarouselChange::Unchanged;
        }
        let from = self.current_index;
        let Some(to) = next_index(from, self.total, self.looping) else {
            return CarouselChange::Unchanged;
        };
        self.current_index = to;
        CarouselChange::Moved { from, to }
    }

    fn select(&mut self, index: usize) -> CarouselChange {
        if index >= self.total || index == self.current_index {
            CarouselChange::Unchanged
        } else {
            self.current_index = index;
            CarouselChange::Selected(index)
        }
    }
}

pub fn validate_carousel_model(model: &CarouselModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn carousel_layout_metrics(
    density: CarouselDensity,
    disabled: bool,
    inline_size: f32,
) -> CarouselLayoutMetrics {
    let dense_root = density == CarouselDensity::Dense && !disabled;
    let root_padding = if dense_root {
        scale::space::xs(inline_size)
    } else {
        scale::space::s(inline_size)
    };
    let root_gap = root_padding;
    let content_gap = if density == CarouselDensity::Dense {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let (title_font_size, title_line_height, detail_font_size) =
        if density == CarouselDensity::Dense {
            (
                scale::font_size::f0(inline_size),
                scale::line_height::LH0,
                scale::font_size::f00(inline_size),
            )
        } else {
            (
                scale::font_size::f1(inline_size),
                scale::line_height::LH2,
                scale::font_size::f0(inline_size),
            )
        };

    CarouselLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_padding,
        root_gap,
        content_gap,
        item_min_height: scale::space::xl(inline_size),
        item_padding: scale::space::s(inline_size),
        item_gap: scale::space::xs2(inline_size),
        title_font_size,
        title_line_height,
        detail_font_size,
        detail_line_height: scale::line_height::LH0,
        controls_gap: scale::space::xs(inline_size),
        nav_size: scale::space::l(inline_size),
        nav_font_size: scale::font_size::f0(inline_size),
        nav_line_height: scale::line_height::LH0,
        indicator_padding_inline: scale::space::xs(inline_size),
        indicator_padding_block: scale::space::xs3(inline_size),
        indicator_font_size: scale::font_size::f00(inline_size),
        indicator_line_height: scale::line_height::LH00,
    }
}

pub fn carousel_render_nodes(
    model: &CarouselModel,
    state: &CarouselState,
) -> Vec<CarouselRenderNode> {
    let total = model.slides.len();
    let current = state.current_index();
    let blocked = model.loading || model.disabled;
    let mut nodes = Vec::with_capacity(total + 5);
    nodes.push(carousel_node(
        model,
        CarouselNodeDraft {
            part: CarouselPart::Root,
            index: 0,
            total,
            value: "carousel".to_owned(),
            label: "Carousel".to_owned(),
            detail: "Paged content carousel".to_owned(),
            selected: false,
            disabled: model.disabled,
        },
    ));
    nodes.push(carousel_node(
        model,
        CarouselNodeDraft {
            part: CarouselPart::Content,
            index: current,
            total,
            value: "carousel-content".to_owned(),
            label: "Carousel content".to_owned(),
            detail: "Paged slide strip".to_owned(),
            selected: false,
            disabled: model.disabled,
        },
    ));

    for (index, slide) in model.slides.iter().enumerate() {
        nodes.push(carousel_node(
            model,
            CarouselNodeDraft {
                part: CarouselPart::Item,
                index,
                total,
                value: slide.value.clone(),
                label: slide.label.clone(),
                detail: slide.detail.clone(),
                selected: state.is_selected(index),
                disabled: blocked || slide.disabled,
            },
        ));
    }

    nodes.push(carousel_node(
        model,
        CarouselNodeDraft {
            part: CarouselPart::Previous,
            index: current,
            total,
            value: "previous".to_owned(),
            label: "Previous".to_owned(),
            detail: "Previous slide".to_owned(),
            selected: false,
            disabled: blocked || previous_index(current, total, model.looping).is_none(),
        },
    ));
    nodes.push(carousel_node(
        model,
        CarouselNodeDraft {
            part: CarouselPart::Next,
            index: current,
            total,
            value: "next".to_owned(),
            label: "Next".to_owned(),
            detail: "Next slide".to_owned(),
            selected: false,
            disabled: blocked || next_index(current, total, model.looping).is_none(),
        },
    ));
    nodes.push(carousel_node(
        model,
        CarouselNodeDraft {
            part: CarouselPart::Indicator,
            index: current,
            total,
            value: "indicator".to_owned(),
            label: format!("{} of {}", current.saturating_add(1), total),
            detail: model
                .slides
                .get(current)
                .map(|slide| slide.label.as_str())
                .unwrap_or("No slide")
                .to_owned(),
            selected: true,
            disabled: model.disabled,
        },
    ));
    nodes
}

pub fn default_carousel_model() -> CarouselModel {
    CarouselModel::new(default_carousel_slides())
        .with_default_index(0)
        .looping()
}

pub fn default_carousel_slides() -> Vec<CarouselSlide> {
    vec![
        CarouselSlide::new(
            "Theme preview",
            "theme-preview",
            "Semantic tokens resolve through Leptos and Bevy from the same palette.",
        ),
        CarouselSlide::new(
            "Component contract",
            "component-contract",
            "Typed render nodes keep content, controls, and indicators portable.",
        ),
        CarouselSlide::new(
            "Story proof",
            "story-proof",
            "The story harness validates the component before app integration.",
        ),
    ]
}

struct CarouselNodeDraft {
    part: CarouselPart,
    index: usize,
    total: usize,
    value: String,
    label: String,
    detail: String,
    selected: bool,
    disabled: bool,
}

fn carousel_node(model: &CarouselModel, draft: CarouselNodeDraft) -> CarouselRenderNode {
    CarouselRenderNode {
        part: draft.part,
        index: draft.index,
        total: draft.total,
        value: draft.value,
        label: draft.label,
        detail: draft.detail,
        density: model.density,
        selected: draft.selected,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn previous_index(current: usize, total: usize, looping: bool) -> Option<usize> {
    if total <= 1 {
        None
    } else if current == 0 {
        looping.then_some(total - 1)
    } else {
        Some(current - 1)
    }
}

fn next_index(current: usize, total: usize, looping: bool) -> Option<usize> {
    if total <= 1 {
        None
    } else if current.saturating_add(1) >= total {
        looping.then_some(0)
    } else {
        Some(current + 1)
    }
}

fn carousel_index_in_bounds<'a>(
    slides: &'a [CarouselSlide],
) -> impl FnOnce(&usize, &()) -> garde::Result + 'a {
    move |index, _context| {
        if slides.is_empty() || *index < slides.len() {
            Ok(())
        } else {
            Err(garde::Error::new(format!(
                "default carousel index {index} is outside {} slides",
                slides.len()
            )))
        }
    }
}

fn carousel_slides_are_unique(slides: &[CarouselSlide], _context: &()) -> garde::Result {
    for (index, slide) in slides.iter().enumerate() {
        if slides
            .iter()
            .skip(index + 1)
            .any(|other| other.value == slide.value)
        {
            return Err(garde::Error::new(format!(
                "duplicate carousel slide value `{}`",
                slide.value
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_carousel_model(&default_carousel_model()).is_ok());
    }

    #[test]
    fn layout_metrics_share_fluid_tailwind_tokens() {
        let compact = carousel_layout_metrics(CarouselDensity::Standard, false, 320.0);
        let wide = carousel_layout_metrics(CarouselDensity::Standard, false, 1_000.0);
        let dense = carousel_layout_metrics(CarouselDensity::Dense, false, 1_000.0);
        let disabled_dense = carousel_layout_metrics(CarouselDensity::Dense, true, 1_000.0);

        assert!(compact.root_padding < wide.root_padding);
        assert!(dense.root_padding < wide.root_padding);
        assert_eq!(disabled_dense.root_padding, wide.root_padding);
        assert!(dense.detail_font_size < wide.detail_font_size);
        assert_eq!(wide.max_width, scale::container::CONTROL);
    }

    #[test]
    fn garde_rejects_empty_slides() {
        assert!(validate_carousel_model(&CarouselModel::new(Vec::new())).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_slide_values() {
        let model = CarouselModel::new(vec![
            CarouselSlide::new("One", "same", "First"),
            CarouselSlide::new("Two", "same", "Second"),
        ]);
        assert!(validate_carousel_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_out_of_bounds_default_index() {
        let model = default_carousel_model().with_default_index(4);
        assert!(validate_carousel_model(&model).is_err());
    }

    #[test]
    fn state_moves_and_selects_slides() {
        let model = default_carousel_model();
        let mut state = model.state();
        assert_eq!(state.current_index(), 0);
        assert_eq!(
            state.apply(CarouselIntent::Next),
            CarouselChange::Moved { from: 0, to: 1 }
        );
        assert_eq!(
            state.apply(CarouselIntent::Previous),
            CarouselChange::Moved { from: 1, to: 0 }
        );
        assert_eq!(
            state.apply(CarouselIntent::Select(2)),
            CarouselChange::Selected(2)
        );
        assert_eq!(state.current_index(), 2);
    }

    #[test]
    fn non_looping_edges_do_not_move() {
        let model = CarouselModel::new(default_carousel_slides());
        let mut state = model.state();
        assert_eq!(
            state.apply(CarouselIntent::Previous),
            CarouselChange::Unchanged
        );
        assert_eq!(
            state.apply(CarouselIntent::Select(2)),
            CarouselChange::Selected(2)
        );
        assert_eq!(state.apply(CarouselIntent::Next), CarouselChange::Unchanged);
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_carousel_model();
        let nodes = carousel_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(CarouselPart::Root)
        );
        for part in CarouselPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == CarouselPart::Item)
                .count(),
            model.slides.len()
        );
    }
}
