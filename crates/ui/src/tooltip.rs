use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::dom::ui_dom_id;

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
}
