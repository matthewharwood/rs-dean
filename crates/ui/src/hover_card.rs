use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum HoverCardDensity {
    Standard,
    Dense,
}

impl HoverCardDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum HoverCardPart {
    Root,
    Trigger,
    Content,
    Arrow,
}

impl HoverCardPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Trigger, Self::Content, Self::Arrow];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "HoverCard",
            Self::Trigger => "HoverCardTrigger",
            Self::Content => "HoverCardContent",
            Self::Arrow => "HoverCardArrow",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct HoverCardModel {
    #[garde(skip)]
    pub density: HoverCardDensity,
    #[garde(length(min = 1, max = 128))]
    pub trigger_label: String,
    #[garde(length(min = 1, max = 160))]
    pub preview_title: String,
    #[garde(length(min = 1, max = 480))]
    pub preview_description: String,
    #[garde(length(min = 1, max = 160))]
    pub metadata: String,
    #[garde(length(min = 1, max = 96))]
    pub arrow_label: String,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoverCardState {
    open: bool,
    active_part: Option<HoverCardPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverCardIntent {
    Open(HoverCardPart),
    Close,
    Toggle,
    Focus(HoverCardPart),
    Blur,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverCardChange {
    Opened(HoverCardPart),
    Closed,
    Focused(HoverCardPart),
    Blurred,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoverCardRenderNode {
    pub part: HoverCardPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: HoverCardDensity,
    pub open: bool,
    pub active: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl HoverCardModel {
    pub fn new(
        trigger_label: impl Into<String>,
        preview_title: impl Into<String>,
        preview_description: impl Into<String>,
    ) -> Self {
        Self {
            density: HoverCardDensity::Standard,
            trigger_label: trigger_label.into(),
            preview_title: preview_title.into(),
            preview_description: preview_description.into(),
            metadata: "Leptos + Bevy".to_owned(),
            arrow_label: "Preview arrow".to_owned(),
            default_open: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: HoverCardDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = metadata.into();
        self
    }

    pub fn with_arrow_label(mut self, arrow_label: impl Into<String>) -> Self {
        self.arrow_label = arrow_label.into();
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

    pub const fn state(&self) -> HoverCardState {
        HoverCardState::new(self.default_open)
    }
}

impl HoverCardState {
    pub const fn new(open: bool) -> Self {
        Self {
            open,
            active_part: if open {
                Some(HoverCardPart::Trigger)
            } else {
                None
            },
        }
    }

    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub const fn is_active(&self, part: HoverCardPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn apply(&mut self, intent: HoverCardIntent) -> HoverCardChange {
        match intent {
            HoverCardIntent::Open(part) => self.open(part),
            HoverCardIntent::Close => self.close(),
            HoverCardIntent::Toggle => self.toggle(),
            HoverCardIntent::Focus(part) => self.focus(part),
            HoverCardIntent::Blur => self.blur(),
        }
    }

    fn open(&mut self, part: HoverCardPart) -> HoverCardChange {
        if self.open && self.is_active(part) {
            HoverCardChange::Unchanged
        } else {
            self.open = true;
            self.active_part = Some(part);
            HoverCardChange::Opened(part)
        }
    }

    fn close(&mut self) -> HoverCardChange {
        if self.open || self.active_part.is_some() {
            self.open = false;
            self.active_part = None;
            HoverCardChange::Closed
        } else {
            HoverCardChange::Unchanged
        }
    }

    fn toggle(&mut self) -> HoverCardChange {
        if self.open {
            self.close()
        } else {
            self.open(HoverCardPart::Trigger)
        }
    }

    fn focus(&mut self, part: HoverCardPart) -> HoverCardChange {
        if self.open {
            if self.is_active(part) {
                HoverCardChange::Unchanged
            } else {
                self.active_part = Some(part);
                HoverCardChange::Focused(part)
            }
        } else {
            self.open(part)
        }
    }

    fn blur(&mut self) -> HoverCardChange {
        if self.open || self.active_part.is_some() {
            self.open = false;
            self.active_part = None;
            HoverCardChange::Blurred
        } else {
            HoverCardChange::Unchanged
        }
    }
}

pub fn validate_hover_card_model(model: &HoverCardModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn hover_card_render_nodes(
    model: &HoverCardModel,
    state: &HoverCardState,
) -> Vec<HoverCardRenderNode> {
    let blocked = model.loading || model.disabled;
    vec![
        hover_card_node(
            model,
            state,
            HoverCardNodeDraft::new(
                HoverCardPart::Root,
                "hover-card",
                "HoverCard",
                &model.preview_title,
                true,
                model.disabled,
            ),
        ),
        hover_card_node(
            model,
            state,
            HoverCardNodeDraft::new(
                HoverCardPart::Trigger,
                "hover-card-trigger",
                &model.trigger_label,
                "Hover or focus to reveal preview content.",
                true,
                blocked,
            ),
        ),
        hover_card_node(
            model,
            state,
            HoverCardNodeDraft::new(
                HoverCardPart::Content,
                &model.metadata,
                &model.preview_title,
                &model.preview_description,
                state.is_open(),
                !state.is_open() || model.disabled,
            ),
        ),
        hover_card_node(
            model,
            state,
            HoverCardNodeDraft::new(
                HoverCardPart::Arrow,
                "hover-card-arrow",
                "arrow",
                &model.arrow_label,
                state.is_open(),
                !state.is_open() || model.disabled,
            ),
        ),
    ]
}

pub fn default_hover_card_model() -> HoverCardModel {
    HoverCardModel::new(
        "Design token",
        "Shared semantic preview",
        "Hover content is renderer-local while the same typed model feeds Leptos DOM and Bevy primitives.",
    )
    .with_metadata("rs-dean-ui")
}

struct HoverCardNodeDraft<'a> {
    part: HoverCardPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    disabled: bool,
}

impl<'a> HoverCardNodeDraft<'a> {
    const fn new(
        part: HoverCardPart,
        value: &'a str,
        label: &'a str,
        detail: &'a str,
        visible: bool,
        disabled: bool,
    ) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            visible,
            disabled,
        }
    }
}

fn hover_card_node(
    model: &HoverCardModel,
    state: &HoverCardState,
    draft: HoverCardNodeDraft<'_>,
) -> HoverCardRenderNode {
    HoverCardRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        open: state.is_open(),
        active: state.is_active(draft.part),
        visible: draft.visible,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_hover_card_model(&default_hover_card_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_trigger_label() {
        let model = HoverCardModel::new("", "Preview", "Description");
        assert!(validate_hover_card_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_preview_title() {
        let model = HoverCardModel::new("Trigger", "", "Description");
        assert!(validate_hover_card_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_preview_description() {
        let model = HoverCardModel::new("Trigger", "Preview", "");
        assert!(validate_hover_card_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_metadata() {
        let model = default_hover_card_model().with_metadata("");
        assert!(validate_hover_card_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_arrow_label() {
        let model = default_hover_card_model().with_arrow_label("");
        assert!(validate_hover_card_model(&model).is_err());
    }

    #[test]
    fn state_tracks_open_focus_toggle_and_close() {
        let mut state = HoverCardState::new(false);
        assert!(!state.is_open());
        assert_eq!(
            state.apply(HoverCardIntent::Open(HoverCardPart::Trigger)),
            HoverCardChange::Opened(HoverCardPart::Trigger)
        );
        assert!(state.is_open());
        assert!(state.is_active(HoverCardPart::Trigger));
        assert_eq!(
            state.apply(HoverCardIntent::Focus(HoverCardPart::Content)),
            HoverCardChange::Focused(HoverCardPart::Content)
        );
        assert!(state.is_active(HoverCardPart::Content));
        assert_eq!(
            state.apply(HoverCardIntent::Toggle),
            HoverCardChange::Closed
        );
        assert!(!state.is_open());
        assert_eq!(
            state.apply(HoverCardIntent::Blur),
            HoverCardChange::Unchanged
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_hover_card_model();
        let nodes = hover_card_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), HoverCardPart::ALL.len());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(HoverCardPart::Root)
        );
        for part in HoverCardPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn closed_state_keeps_content_and_arrow_hidden() {
        let model = default_hover_card_model();
        let nodes = hover_card_render_nodes(&model, &model.state());
        for part in [HoverCardPart::Content, HoverCardPart::Arrow] {
            let node = nodes
                .iter()
                .find(|node| node.part == part)
                .expect("hover card includes hidden overlay parts");
            assert!(!node.visible);
            assert!(node.disabled);
        }
    }

    #[test]
    fn default_open_model_reveals_content_and_arrow() {
        let model = default_hover_card_model().default_open();
        let nodes = hover_card_render_nodes(&model, &model.state());
        for part in [HoverCardPart::Content, HoverCardPart::Arrow] {
            let node = nodes
                .iter()
                .find(|node| node.part == part)
                .expect("hover card includes overlay parts");
            assert!(node.visible);
            assert!(!node.disabled);
        }
    }
}
