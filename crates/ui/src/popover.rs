use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PopoverDensity {
    Standard,
    Dense,
}

impl PopoverDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PopoverPart {
    Root,
    Trigger,
    Content,
    Arrow,
}

impl PopoverPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Trigger, Self::Content, Self::Arrow];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Popover",
            Self::Trigger => "PopoverTrigger",
            Self::Content => "PopoverContent",
            Self::Arrow => "PopoverArrow",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct PopoverModel {
    #[garde(skip)]
    pub density: PopoverDensity,
    #[garde(length(min = 1, max = 128))]
    pub trigger_label: String,
    #[garde(length(min = 1, max = 96))]
    pub eyebrow: String,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 480))]
    pub description: String,
    #[garde(length(min = 1, max = 96))]
    pub arrow_label: String,
    #[garde(custom(validate_optional_popover_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopoverState {
    open: bool,
    active_part: Option<PopoverPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PopoverIntent {
    Open(PopoverPart),
    Close,
    Toggle,
    Focus(PopoverPart),
    ClearFocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PopoverChange {
    Opened(PopoverPart),
    Closed,
    Focused(PopoverPart),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopoverRenderNode {
    pub part: PopoverPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: PopoverDensity,
    pub open: bool,
    pub active: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverLayoutMetrics {
    pub max_width: f32,
    pub root_gap: f32,
    pub trigger_min_height: f32,
    pub trigger_padding_inline: f32,
    pub trigger_padding_block: f32,
    pub trigger_font_size: f32,
    pub trigger_line_height: f32,
    pub content_offset: f32,
    pub content_padding: f32,
    pub content_gap: f32,
    pub arrow_size: f32,
    pub meta_font_size: f32,
    pub title_font_size: f32,
    pub title_line_height: f32,
    pub detail_font_size: f32,
    pub detail_line_height: f32,
    pub error_font_size: f32,
}

impl PopoverLayoutMetrics {
    pub fn trigger_outer_height(self, border_width: f32) -> f32 {
        self.trigger_min_height.max(
            self.trigger_font_size * self.trigger_line_height
                + self.trigger_padding_block * 2.0
                + border_width.max(0.0) * 2.0,
        )
    }
}

impl PopoverModel {
    pub fn new(
        trigger_label: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            density: PopoverDensity::Standard,
            trigger_label: trigger_label.into(),
            eyebrow: "Renderer local".to_owned(),
            title: title.into(),
            description: description.into(),
            arrow_label: "Popover arrow".to_owned(),
            error: None,
            default_open: true,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: PopoverDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = eyebrow.into();
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

    pub const fn closed(mut self) -> Self {
        self.default_open = false;
        self
    }

    pub const fn default_open(mut self) -> Self {
        self.default_open = true;
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

    pub const fn state(&self) -> PopoverState {
        PopoverState::new(self.default_open)
    }
}

impl PopoverState {
    pub const fn new(open: bool) -> Self {
        Self {
            open,
            active_part: if open {
                Some(PopoverPart::Trigger)
            } else {
                None
            },
        }
    }

    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub const fn is_active(&self, part: PopoverPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn apply(&mut self, intent: PopoverIntent) -> PopoverChange {
        match intent {
            PopoverIntent::Open(part) => self.open(part),
            PopoverIntent::Close => self.close(),
            PopoverIntent::Toggle => self.toggle(),
            PopoverIntent::Focus(part) => self.focus(part),
            PopoverIntent::ClearFocus => self.clear_focus(),
        }
    }

    fn open(&mut self, part: PopoverPart) -> PopoverChange {
        if self.open && self.is_active(part) {
            PopoverChange::Unchanged
        } else {
            self.open = true;
            self.active_part = Some(part);
            PopoverChange::Opened(part)
        }
    }

    fn close(&mut self) -> PopoverChange {
        if self.open || self.active_part.is_some() {
            self.open = false;
            self.active_part = None;
            PopoverChange::Closed
        } else {
            PopoverChange::Unchanged
        }
    }

    fn toggle(&mut self) -> PopoverChange {
        if self.open {
            self.close()
        } else {
            self.open(PopoverPart::Trigger)
        }
    }

    fn focus(&mut self, part: PopoverPart) -> PopoverChange {
        if self.is_active(part) {
            return PopoverChange::Unchanged;
        }
        if part != PopoverPart::Trigger {
            self.open = true;
        }
        self.active_part = Some(part);
        PopoverChange::Focused(part)
    }

    fn clear_focus(&mut self) -> PopoverChange {
        if self.active_part.is_none() {
            PopoverChange::Unchanged
        } else {
            self.active_part = None;
            PopoverChange::Cleared
        }
    }
}

pub fn validate_popover_model(model: &PopoverModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn popover_layout_metrics(model: &PopoverModel, inline_size: f32) -> PopoverLayoutMetrics {
    let dense = model.density == PopoverDensity::Dense;
    let trigger_emphasized = model.loading || model.disabled || model.error.is_some();
    let dense_trigger = dense && !trigger_emphasized;
    let dense_content = dense && !trigger_emphasized;
    PopoverLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_gap: scale::space::xs2(inline_size),
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
        content_offset: scale::space::xs2(inline_size),
        content_padding: if dense_content {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        content_gap: if dense_content {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        arrow_size: scale::space::s(inline_size),
        meta_font_size: scale::font_size::f00(inline_size),
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
        detail_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        detail_line_height: scale::line_height::LH0,
        error_font_size: scale::font_size::f00(inline_size),
    }
}

pub fn popover_render_nodes(model: &PopoverModel, state: &PopoverState) -> Vec<PopoverRenderNode> {
    let blocked = model.loading || model.disabled;
    let invalid = model.error.is_some();
    vec![
        popover_node(
            model,
            state,
            PopoverNodeDraft::new(
                PopoverPart::Root,
                "popover",
                "Popover",
                if state.is_open() {
                    "Popover open"
                } else {
                    "Popover closed"
                },
                true,
                false,
                model.disabled,
            ),
        ),
        popover_node(
            model,
            state,
            PopoverNodeDraft::new(
                PopoverPart::Trigger,
                "popover-trigger",
                &model.trigger_label,
                "Toggle popover content.",
                true,
                true,
                blocked,
            ),
        ),
        popover_node(
            model,
            state,
            PopoverNodeDraft::new(
                PopoverPart::Content,
                &model.eyebrow,
                &model.title,
                &model.description,
                state.is_open(),
                false,
                !state.is_open() || model.disabled,
            ),
        ),
        popover_node(
            model,
            state,
            PopoverNodeDraft::new(
                PopoverPart::Arrow,
                "popover-arrow",
                "arrow",
                &model.arrow_label,
                state.is_open(),
                false,
                !state.is_open() || model.disabled,
            ),
        ),
    ]
    .into_iter()
    .map(|mut node| {
        node.invalid = invalid;
        node
    })
    .collect()
}

pub fn default_popover_model() -> PopoverModel {
    PopoverModel::new(
        "Open controls",
        "Shared overlay controls",
        "Popover state is renderer-local while durable choices stay in the consuming app state layer.",
    )
    .with_eyebrow("Issue 43")
}

struct PopoverNodeDraft<'a> {
    part: PopoverPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    actionable: bool,
    disabled: bool,
}

impl<'a> PopoverNodeDraft<'a> {
    const fn new(
        part: PopoverPart,
        value: &'a str,
        label: &'a str,
        detail: &'a str,
        visible: bool,
        actionable: bool,
        disabled: bool,
    ) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            visible,
            actionable,
            disabled,
        }
    }
}

fn popover_node(
    model: &PopoverModel,
    state: &PopoverState,
    draft: PopoverNodeDraft<'_>,
) -> PopoverRenderNode {
    PopoverRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        open: state.is_open(),
        active: state.is_active(draft.part),
        visible: draft.visible,
        actionable: draft.actionable,
        invalid: false,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn validate_optional_popover_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "popover error must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_popover_model(&default_popover_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_trigger_label() {
        let model = PopoverModel::new("", "Title", "Description");
        assert!(validate_popover_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = PopoverModel::new("Open", "", "Description");
        assert!(validate_popover_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_description() {
        let model = PopoverModel::new("Open", "Title", "");
        assert!(validate_popover_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_eyebrow() {
        let model = default_popover_model().with_eyebrow("");
        assert!(validate_popover_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_arrow_label() {
        let model = default_popover_model().with_arrow_label("");
        assert!(validate_popover_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_popover_model().with_error("");
        assert!(validate_popover_model(&model).is_err());
    }

    #[test]
    fn state_tracks_toggle_focus_clear_and_close() {
        let mut state = PopoverState::new(false);
        assert!(!state.is_open());
        assert_eq!(
            state.apply(PopoverIntent::Toggle),
            PopoverChange::Opened(PopoverPart::Trigger)
        );
        assert!(state.is_open());
        assert!(state.is_active(PopoverPart::Trigger));
        assert_eq!(
            state.apply(PopoverIntent::Focus(PopoverPart::Content)),
            PopoverChange::Focused(PopoverPart::Content)
        );
        assert!(state.is_active(PopoverPart::Content));
        assert_eq!(
            state.apply(PopoverIntent::ClearFocus),
            PopoverChange::Cleared
        );
        assert!(state.is_open());
        assert_eq!(state.apply(PopoverIntent::Close), PopoverChange::Closed);
        assert!(!state.is_open());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_popover_model();
        let nodes = popover_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), PopoverPart::ALL.len());
        assert_eq!(nodes.first().map(|node| node.part), Some(PopoverPart::Root));
        for part in PopoverPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn closed_state_keeps_content_and_arrow_hidden() {
        let model = default_popover_model().closed();
        let nodes = popover_render_nodes(&model, &model.state());
        for part in [PopoverPart::Content, PopoverPart::Arrow] {
            let node = nodes
                .iter()
                .find(|node| node.part == part)
                .expect("popover includes hidden overlay parts");
            assert!(!node.visible);
            assert!(node.disabled);
        }
    }

    #[test]
    fn loading_disables_trigger_but_preserves_open_content() {
        let model = default_popover_model().loading();
        let nodes = popover_render_nodes(&model, &model.state());
        let trigger = nodes
            .iter()
            .find(|node| node.part == PopoverPart::Trigger)
            .expect("popover includes trigger");
        let content = nodes
            .iter()
            .find(|node| node.part == PopoverPart::Content)
            .expect("popover includes content");
        assert!(trigger.disabled);
        assert!(content.visible);
        assert!(content.loading);
    }

    #[test]
    fn error_marks_render_nodes_invalid() {
        let model = default_popover_model().with_error("Unavailable configuration");
        let nodes = popover_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
    }

    #[test]
    fn layout_metrics_preserve_dense_and_state_precedence() {
        let standard = popover_layout_metrics(&default_popover_model(), 1_000.0);
        let dense = popover_layout_metrics(
            &default_popover_model().with_density(PopoverDensity::Dense),
            1_000.0,
        );
        let dense_loading = popover_layout_metrics(
            &default_popover_model()
                .with_density(PopoverDensity::Dense)
                .loading(),
            1_000.0,
        );

        assert!(dense.trigger_min_height < standard.trigger_min_height);
        assert!(dense.content_padding < standard.content_padding);
        assert!(dense.title_font_size < standard.title_font_size);
        assert_eq!(
            dense_loading.trigger_min_height,
            standard.trigger_min_height
        );
        assert_eq!(dense_loading.content_padding, standard.content_padding);
        assert!(standard.trigger_outer_height(1.0) >= standard.trigger_min_height);
    }
}
