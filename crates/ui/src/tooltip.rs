use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{dom::ui_dom_id, scale};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TooltipDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TooltipPlacement {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TooltipPart {
    Root,
    Trigger,
    Content,
    Arrow,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct TooltipModel {
    #[garde(skip)]
    pub density: TooltipDensity,
    #[garde(skip)]
    pub placement: TooltipPlacement,
    #[garde(length(min = 1, max = 128))]
    pub trigger_label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 480))]
    pub content: String,
    #[garde(length(min = 1, max = 96))]
    pub arrow_label: String,
    #[garde(custom(validate_optional_tooltip_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub show_arrow: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooltipState {
    open: bool,
    active_part: Option<TooltipPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TooltipIntent {
    Open(TooltipPart),
    Close,
    Toggle,
    Focus(TooltipPart),
    Blur,
    Hover(TooltipPart),
    Leave,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TooltipChange {
    Opened(TooltipPart),
    Closed,
    Focused(TooltipPart),
    Blurred,
    Hovered(TooltipPart),
    Left,
    Reset,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TooltipRenderNode {
    pub part: TooltipPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: TooltipDensity,
    pub placement: TooltipPlacement,
    pub open: bool,
    pub active: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TooltipLayoutMetrics {
    pub stage_height: f32,
    pub stage_padding: f32,
    pub trigger_width: f32,
    pub trigger_height: f32,
    pub trigger_padding_inline: f32,
    pub trigger_padding_block: f32,
    pub trigger_font_size: f32,
    pub trigger_line_height: f32,
    pub content_width: f32,
    pub content_height: f32,
    pub content_padding: f32,
    pub content_gap: f32,
    pub content_font_size: f32,
    pub content_line_height: f32,
    pub arrow_edge: f32,
    pub placement_gap: f32,
    pub placement: TooltipPlacement,
    pub content_visible: bool,
    pub arrow_visible: bool,
}

impl TooltipDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl TooltipPlacement {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

impl TooltipPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Trigger, Self::Content, Self::Arrow];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Tooltip",
            Self::Trigger => "TooltipTrigger",
            Self::Content => "TooltipContent",
            Self::Arrow => "TooltipArrow",
        }
    }
}

impl TooltipModel {
    pub fn new(
        trigger_label: impl Into<String>,
        value: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            density: TooltipDensity::Standard,
            placement: TooltipPlacement::Top,
            trigger_label: trigger_label.into(),
            value: value.into(),
            content: content.into(),
            arrow_label: "Tooltip arrow".to_owned(),
            error: None,
            default_open: true,
            show_arrow: true,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: TooltipDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_trigger_label(mut self, trigger_label: impl Into<String>) -> Self {
        self.trigger_label = trigger_label.into();
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn with_arrow_label(mut self, arrow_label: impl Into<String>) -> Self {
        self.arrow_label = arrow_label.into();
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

    pub const fn open(mut self) -> Self {
        self.default_open = true;
        self
    }

    pub const fn closed(mut self) -> Self {
        self.default_open = false;
        self
    }

    pub const fn with_arrow(mut self) -> Self {
        self.show_arrow = true;
        self
    }

    pub const fn without_arrow(mut self) -> Self {
        self.show_arrow = false;
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

    pub const fn state(&self) -> TooltipState {
        TooltipState::new(self.default_open)
    }
}

impl TooltipState {
    pub const fn new(open: bool) -> Self {
        Self {
            open,
            active_part: if open {
                Some(TooltipPart::Trigger)
            } else {
                None
            },
        }
    }

    pub const fn is_open(self) -> bool {
        self.open
    }

    pub const fn active_part(self) -> Option<TooltipPart> {
        self.active_part
    }

    pub const fn is_active(self, part: TooltipPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn apply(&mut self, intent: TooltipIntent) -> TooltipChange {
        match intent {
            TooltipIntent::Open(part) => self.open_part(part),
            TooltipIntent::Close => self.close(),
            TooltipIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open_part(TooltipPart::Trigger)
                }
            }
            TooltipIntent::Focus(part) => self.focus(part),
            TooltipIntent::Blur => self.blur(),
            TooltipIntent::Hover(part) => self.hover(part),
            TooltipIntent::Leave => self.leave(),
            TooltipIntent::Reset => self.reset(),
        }
    }

    fn open_part(&mut self, part: TooltipPart) -> TooltipChange {
        if self.open && self.active_part == Some(part) {
            TooltipChange::Unchanged
        } else {
            self.open = true;
            self.active_part = Some(part);
            TooltipChange::Opened(part)
        }
    }

    fn close(&mut self) -> TooltipChange {
        if self.open || self.active_part.is_some() {
            self.open = false;
            self.active_part = None;
            TooltipChange::Closed
        } else {
            TooltipChange::Unchanged
        }
    }

    fn focus(&mut self, part: TooltipPart) -> TooltipChange {
        if self.open && self.active_part == Some(part) {
            TooltipChange::Unchanged
        } else {
            self.open = true;
            self.active_part = Some(part);
            TooltipChange::Focused(part)
        }
    }

    fn blur(&mut self) -> TooltipChange {
        if self.open || self.active_part.is_some() {
            self.open = false;
            self.active_part = None;
            TooltipChange::Blurred
        } else {
            TooltipChange::Unchanged
        }
    }

    fn hover(&mut self, part: TooltipPart) -> TooltipChange {
        if self.open && self.active_part == Some(part) {
            TooltipChange::Unchanged
        } else {
            self.open = true;
            self.active_part = Some(part);
            TooltipChange::Hovered(part)
        }
    }

    fn leave(&mut self) -> TooltipChange {
        if self.open || self.active_part.is_some() {
            self.open = false;
            self.active_part = None;
            TooltipChange::Left
        } else {
            TooltipChange::Unchanged
        }
    }

    fn reset(&mut self) -> TooltipChange {
        if !self.open && self.active_part.is_none() {
            TooltipChange::Unchanged
        } else {
            self.open = false;
            self.active_part = None;
            TooltipChange::Reset
        }
    }
}

pub fn validate_tooltip_model(model: &TooltipModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn tooltip_render_nodes(model: &TooltipModel, state: &TooltipState) -> Vec<TooltipRenderNode> {
    let blocked = model.loading || model.disabled;
    let invalid = model.error.is_some();
    let open = state.is_open() && !model.disabled;
    let content_detail = model.error.clone().unwrap_or_else(|| model.content.clone());
    vec![
        TooltipRenderNode {
            part: TooltipPart::Root,
            value: model.value.clone(),
            label: "Tooltip".to_owned(),
            detail: if open {
                "Tooltip open".to_owned()
            } else {
                "Tooltip closed".to_owned()
            },
            density: model.density,
            placement: model.placement,
            open,
            active: state.is_active(TooltipPart::Root),
            visible: true,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: model.disabled,
        },
        TooltipRenderNode {
            part: TooltipPart::Trigger,
            value: model.value.clone(),
            label: model.trigger_label.clone(),
            detail: "Show tooltip content on focus or hover.".to_owned(),
            density: model.density,
            placement: model.placement,
            open,
            active: state.is_active(TooltipPart::Trigger),
            visible: true,
            actionable: !blocked,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        TooltipRenderNode {
            part: TooltipPart::Content,
            value: model.value.clone(),
            label: "Tooltip content".to_owned(),
            detail: content_detail,
            density: model.density,
            placement: model.placement,
            open,
            active: state.is_active(TooltipPart::Content),
            visible: open,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: !open || model.disabled,
        },
        TooltipRenderNode {
            part: TooltipPart::Arrow,
            value: model.value.clone(),
            label: "arrow".to_owned(),
            detail: model.arrow_label.clone(),
            density: model.density,
            placement: model.placement,
            open,
            active: state.is_active(TooltipPart::Arrow),
            visible: open && model.show_arrow,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: !open || !model.show_arrow || model.disabled,
        },
    ]
}

pub fn tooltip_layout_metrics(
    model: &TooltipModel,
    state: &TooltipState,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> TooltipLayoutMetrics {
    let border_width = border_width.max(0.0);
    let blocked = model.loading || model.disabled;
    let invalid = model.error.is_some();
    let dense = model.density == TooltipDensity::Dense && !blocked && !invalid;
    let trigger_padding_inline = if dense {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let trigger_padding_block = if dense {
        scale::space::xs3(inline_size)
    } else {
        scale::space::xs2(inline_size)
    };
    let trigger_font_size = if dense {
        scale::font_size::f00(inline_size)
    } else {
        scale::font_size::f0(inline_size)
    };
    let trigger_copy = tooltip_trigger_copy(model);
    let trigger_width = border_width * 2.0
        + trigger_padding_inline * 2.0
        + scale::estimate_inline_text_width(&trigger_copy, trigger_font_size, 0.0);
    let minimum_trigger_height = if dense {
        scale::space::s(inline_size)
    } else {
        scale::space::L
    };
    let trigger_line_height = scale::line_height::LH0;
    let trigger_height = minimum_trigger_height.max(
        border_width * 2.0 + trigger_padding_block * 2.0 + trigger_font_size * trigger_line_height,
    );
    let content_padding = if dense {
        scale::space::xs2(inline_size)
    } else {
        scale::space::xs(inline_size)
    };
    let content_gap = if dense {
        scale::space::xs3(inline_size)
    } else {
        scale::space::xs2(inline_size)
    };
    let content_font_size = scale::font_size::f00(inline_size);
    let content_line_height = scale::line_height::LH0;
    let arrow_edge = scale::space::xs(inline_size);
    let content_copy = tooltip_content_copy(model);
    let maximum_content_width = available_width.clamp(scale::space::L, scale::container::CONTROL);
    let natural_copy_width =
        scale::estimate_inline_text_width(&content_copy, content_font_size, 0.0)
            + scale::space::xs3(inline_size);
    let natural_content_width = border_width * 2.0
        + content_padding * 2.0
        + natural_copy_width.max(if model.show_arrow { arrow_edge } else { 0.0 });
    let content_width = natural_content_width.min(maximum_content_width);
    let copy_width = (content_width - border_width * 2.0 - content_padding * 2.0).max(1.0);
    let copy_height = if natural_content_width <= maximum_content_width {
        content_font_size * content_line_height
    } else {
        scale::estimate_precise_text_block_height(
            &content_copy,
            copy_width,
            content_font_size,
            content_line_height,
            0.0,
        )
    };
    let arrow_visible = state.is_open() && model.show_arrow && !model.disabled;
    let content_height = border_width * 2.0
        + content_padding * 2.0
        + copy_height
        + if arrow_visible {
            arrow_edge + content_gap
        } else {
            0.0
        };

    TooltipLayoutMetrics {
        stage_height: scale::space::xl4(inline_size),
        stage_padding: scale::space::s(inline_size),
        trigger_width,
        trigger_height,
        trigger_padding_inline,
        trigger_padding_block,
        trigger_font_size,
        trigger_line_height,
        content_width,
        content_height,
        content_padding,
        content_gap,
        content_font_size,
        content_line_height,
        arrow_edge,
        placement_gap: scale::space::xs2(inline_size),
        placement: tooltip_effective_placement(model),
        content_visible: state.is_open() && !model.disabled,
        arrow_visible,
    }
}

pub fn tooltip_trigger_copy(model: &TooltipModel) -> String {
    if model.loading {
        "Loading".to_owned()
    } else {
        model.trigger_label.clone()
    }
}

pub fn tooltip_content_copy(model: &TooltipModel) -> String {
    if model.loading {
        "Loading tooltip content.".to_owned()
    } else {
        model.error.clone().unwrap_or_else(|| model.content.clone())
    }
}

pub const fn tooltip_effective_placement(model: &TooltipModel) -> TooltipPlacement {
    if model.loading || model.error.is_some() || model.disabled {
        TooltipPlacement::Bottom
    } else {
        model.placement
    }
}

pub fn default_tooltip_model() -> TooltipModel {
    TooltipModel::new(
        "Save",
        "save-command",
        "Writes the current draft to durable app state when the consumer accepts the action.",
    )
}

pub fn tooltip_dom_id(prefix: &str, value: &str) -> String {
    ui_dom_id(prefix, value)
}

fn validate_optional_tooltip_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new(
            "tooltip copy must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_tooltip_model(&default_tooltip_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_trigger_label() {
        let model = TooltipModel::new("", "save", "Save the current draft.");
        assert!(validate_tooltip_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = TooltipModel::new("Save", "", "Save the current draft.");
        assert!(validate_tooltip_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_content() {
        let model = TooltipModel::new("Save", "save", "");
        assert!(validate_tooltip_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_arrow_label() {
        let model = default_tooltip_model().with_arrow_label("");
        assert!(validate_tooltip_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_tooltip_model().with_error("");
        assert!(validate_tooltip_model(&model).is_err());
    }

    #[test]
    fn state_opens_on_focus_and_hover_then_closes() {
        let mut state = TooltipState::new(false);
        assert!(!state.is_open());
        assert_eq!(
            state.apply(TooltipIntent::Focus(TooltipPart::Trigger)),
            TooltipChange::Focused(TooltipPart::Trigger)
        );
        assert!(state.is_open());
        assert!(state.is_active(TooltipPart::Trigger));
        assert_eq!(state.apply(TooltipIntent::Blur), TooltipChange::Blurred);
        assert!(!state.is_open());
        assert_eq!(
            state.apply(TooltipIntent::Hover(TooltipPart::Trigger)),
            TooltipChange::Hovered(TooltipPart::Trigger)
        );
        assert!(state.is_open());
        assert_eq!(state.apply(TooltipIntent::Leave), TooltipChange::Left);
        assert!(!state.is_open());
    }

    #[test]
    fn state_toggles_and_resets() {
        let mut state = TooltipState::new(false);
        assert_eq!(
            state.apply(TooltipIntent::Toggle),
            TooltipChange::Opened(TooltipPart::Trigger)
        );
        assert!(state.is_open());
        assert_eq!(state.apply(TooltipIntent::Reset), TooltipChange::Reset);
        assert!(!state.is_open());
        assert_eq!(state.active_part(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_tooltip_model();
        let nodes = tooltip_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(TooltipPart::Root));
        for part in TooltipPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn closed_state_hides_content_and_arrow() {
        let model = default_tooltip_model().closed();
        let nodes = tooltip_render_nodes(&model, &model.state());
        for part in [TooltipPart::Content, TooltipPart::Arrow] {
            let node = nodes
                .iter()
                .find(|node| node.part == part)
                .expect("tooltip includes hidden popup anatomy");
            assert!(!node.visible);
            assert!(node.disabled);
        }
    }

    #[test]
    fn loading_disables_trigger_but_keeps_open_content() {
        let model = default_tooltip_model().loading();
        let nodes = tooltip_render_nodes(&model, &model.state());
        let trigger = nodes
            .iter()
            .find(|node| node.part == TooltipPart::Trigger)
            .expect("tooltip includes trigger");
        let content = nodes
            .iter()
            .find(|node| node.part == TooltipPart::Content)
            .expect("tooltip includes content");
        assert!(trigger.disabled);
        assert!(content.visible);
        assert!(content.loading);
    }

    #[test]
    fn disabled_model_hides_content_even_when_default_open() {
        let model = default_tooltip_model().disabled();
        let nodes = tooltip_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .all(|node| node.disabled || node.part == TooltipPart::Root)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == TooltipPart::Content && !node.visible)
        );
    }

    #[test]
    fn error_marks_render_nodes_invalid() {
        let model = default_tooltip_model().with_error("Tooltip content is unavailable.");
        let nodes = tooltip_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(nodes.iter().any(|node| node.part == TooltipPart::Content
            && node.detail == "Tooltip content is unavailable."));
    }

    #[test]
    fn hidden_arrow_keeps_arrow_node_but_marks_it_inactive() {
        let model = default_tooltip_model().without_arrow();
        let nodes = tooltip_render_nodes(&model, &model.state());
        let arrow = nodes
            .iter()
            .find(|node| node.part == TooltipPart::Arrow)
            .expect("tooltip includes arrow node");
        assert!(!arrow.visible);
        assert!(arrow.disabled);
    }

    #[test]
    fn dom_ids_are_stable_and_ascii() {
        assert_eq!(
            tooltip_dom_id("tooltip-content", "Save & Publish"),
            "tooltip-content-save-publish"
        );
    }

    #[test]
    fn layout_metrics_use_shared_copy_placement_and_density_tokens() {
        let standard = TooltipModel::new("Save", "save", "Save draft.");
        let dense = standard.clone().with_density(TooltipDensity::Dense);
        let standard_metrics =
            tooltip_layout_metrics(&standard, &standard.state(), 448.0, 1_000.0, 1.0);
        let dense_metrics = tooltip_layout_metrics(&dense, &dense.state(), 448.0, 1_000.0, 1.0);

        assert!(dense_metrics.trigger_height < standard_metrics.trigger_height);
        assert!(dense_metrics.content_height < standard_metrics.content_height);
        assert_eq!(standard_metrics.placement, TooltipPlacement::Top);
        assert!(standard_metrics.content_visible);
        assert!(standard_metrics.arrow_visible);
        assert_eq!(
            standard_metrics.trigger_line_height,
            scale::line_height::LH0
        );
        assert_eq!(
            standard_metrics.content_line_height,
            scale::line_height::LH0
        );
        let expected_content_height = 2.0
            + standard_metrics.content_padding * 2.0
            + standard_metrics.arrow_edge
            + standard_metrics.content_gap
            + standard_metrics.content_font_size * standard_metrics.content_line_height;
        assert!((standard_metrics.content_height - expected_content_height).abs() < 0.001);
    }

    #[test]
    fn loading_metrics_use_bottom_placement_and_shared_loading_copy() {
        let model = default_tooltip_model()
            .with_placement(TooltipPlacement::Left)
            .loading();
        let metrics = tooltip_layout_metrics(&model, &model.state(), 448.0, 1_000.0, 1.0);

        assert_eq!(tooltip_trigger_copy(&model), "Loading");
        assert_eq!(tooltip_content_copy(&model), "Loading tooltip content.");
        assert_eq!(metrics.placement, TooltipPlacement::Bottom);
    }
}
