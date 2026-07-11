use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MarkerDensity {
    Standard,
    Dense,
}

impl MarkerDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MarkerTone {
    Neutral,
    Brand,
    Info,
    Success,
    Warning,
    Danger,
}

impl MarkerTone {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Brand => "brand",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MarkerPart {
    Root,
    Dot,
    Label,
    Anchor,
}

impl MarkerPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Dot, Self::Label, Self::Anchor];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Marker",
            Self::Dot => "MarkerDot",
            Self::Label => "MarkerLabel",
            Self::Anchor => "MarkerAnchor",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct MarkerAnchor {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 512))]
    pub href: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct MarkerModel {
    #[garde(skip)]
    pub density: MarkerDensity,
    #[garde(skip)]
    pub tone: MarkerTone,
    #[garde(length(min = 1, max = 160))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub dot_label: String,
    #[garde(custom(validate_optional_marker_anchor))]
    pub anchor: Option<MarkerAnchor>,
    #[garde(custom(validate_optional_marker_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkerState {
    active_part: Option<MarkerPart>,
    navigated_href: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkerIntent {
    Focus(MarkerPart),
    Hover(MarkerPart),
    Navigate(String),
    Blur,
    Leave,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkerChange {
    Focused(MarkerPart),
    Hovered(MarkerPart),
    Navigated(String),
    Blurred,
    Left,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkerRenderNode {
    pub part: MarkerPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: MarkerDensity,
    pub tone: MarkerTone,
    pub active: bool,
    pub invalid: bool,
    pub visible: bool,
    pub actionable: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkerLayoutMetrics {
    pub root_gap: f32,
    pub root_padding_inline: f32,
    pub root_padding_block: f32,
    pub root_emphasis_gap: f32,
    pub root_emphasis_padding_inline: f32,
    pub dot_size: f32,
    pub dot_emphasis_size: f32,
    pub label_font_size: f32,
    pub label_line_height: f32,
    pub anchor_min_height: f32,
    pub anchor_padding_inline: f32,
    pub anchor_padding_block: f32,
    pub anchor_font_size: f32,
    pub anchor_line_height: f32,
}

impl MarkerAnchor {
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl MarkerModel {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            density: MarkerDensity::Standard,
            tone: MarkerTone::Brand,
            label: label.into(),
            value: "unread".to_owned(),
            dot_label: "Unread marker".to_owned(),
            anchor: Some(MarkerAnchor::new("Jump", "#latest")),
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: MarkerDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_tone(mut self, tone: MarkerTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_dot_label(mut self, dot_label: impl Into<String>) -> Self {
        self.dot_label = dot_label.into();
        self
    }

    pub fn with_anchor(mut self, anchor: MarkerAnchor) -> Self {
        self.anchor = Some(anchor);
        self
    }

    pub fn without_anchor(mut self) -> Self {
        self.anchor = None;
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

    pub const fn state(&self) -> MarkerState {
        let _ = self;
        MarkerState::resting()
    }
}

impl MarkerState {
    pub const fn resting() -> Self {
        Self {
            active_part: None,
            navigated_href: None,
        }
    }

    pub const fn is_active(&self, part: MarkerPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn navigated_href(&self) -> Option<&str> {
        self.navigated_href.as_deref()
    }

    pub fn apply(&mut self, intent: MarkerIntent) -> MarkerChange {
        match intent {
            MarkerIntent::Focus(part) => self.focus(part),
            MarkerIntent::Hover(part) => self.hover(part),
            MarkerIntent::Navigate(href) => self.navigate(href),
            MarkerIntent::Blur => self.blur(),
            MarkerIntent::Leave => self.leave(),
            MarkerIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, part: MarkerPart) -> MarkerChange {
        if self.is_active(part) {
            MarkerChange::Unchanged
        } else {
            self.active_part = Some(part);
            MarkerChange::Focused(part)
        }
    }

    fn hover(&mut self, part: MarkerPart) -> MarkerChange {
        if self.is_active(part) {
            MarkerChange::Unchanged
        } else {
            self.active_part = Some(part);
            MarkerChange::Hovered(part)
        }
    }

    fn navigate(&mut self, href: String) -> MarkerChange {
        if href.is_empty() || self.navigated_href.as_ref() == Some(&href) {
            MarkerChange::Unchanged
        } else {
            self.active_part = Some(MarkerPart::Anchor);
            self.navigated_href = Some(href.clone());
            MarkerChange::Navigated(href)
        }
    }

    fn blur(&mut self) -> MarkerChange {
        if self.active_part.is_some() {
            self.active_part = None;
            MarkerChange::Blurred
        } else {
            MarkerChange::Unchanged
        }
    }

    fn leave(&mut self) -> MarkerChange {
        if self.active_part.is_some() {
            self.active_part = None;
            MarkerChange::Left
        } else {
            MarkerChange::Unchanged
        }
    }

    fn clear(&mut self) -> MarkerChange {
        if self.active_part.is_some() || self.navigated_href.is_some() {
            self.active_part = None;
            self.navigated_href = None;
            MarkerChange::Cleared
        } else {
            MarkerChange::Unchanged
        }
    }
}

pub fn validate_marker_model(model: &MarkerModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn marker_layout_metrics(model: &MarkerModel, inline_size: f32) -> MarkerLayoutMetrics {
    let dense = model.density == MarkerDensity::Dense;
    let dense_root = dense && model.error.is_none() && !model.loading && !model.disabled;
    let dense_brand_dot = dense
        && model.tone == MarkerTone::Brand
        && model.error.is_none()
        && !model.loading
        && !model.disabled;
    MarkerLayoutMetrics {
        root_gap: if dense_root {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        root_padding_inline: if dense_root {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        root_padding_block: scale::space::xs3(inline_size),
        root_emphasis_gap: scale::space::xs2(inline_size),
        root_emphasis_padding_inline: scale::space::xs2(inline_size),
        dot_size: if dense_brand_dot {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        dot_emphasis_size: scale::space::xs2(inline_size),
        label_font_size: scale::font_size::f00(inline_size),
        label_line_height: scale::line_height::LH0,
        anchor_min_height: scale::space::s(inline_size),
        anchor_padding_inline: scale::space::xs2(inline_size),
        anchor_padding_block: scale::space::xs3(inline_size),
        anchor_font_size: scale::font_size::f00(inline_size),
        anchor_line_height: scale::line_height::LH0,
    }
}

pub fn marker_render_nodes(model: &MarkerModel, state: &MarkerState) -> Vec<MarkerRenderNode> {
    let invalid = model.error.is_some();
    let root_detail = model.error.as_deref().unwrap_or(&model.label);
    let anchor = model.anchor.as_ref();
    vec![
        marker_node(
            model,
            state,
            MarkerNodeDraft::new(MarkerPart::Root, &model.value, "Marker", root_detail)
                .invalid(invalid),
        ),
        marker_node(
            model,
            state,
            MarkerNodeDraft::new(
                MarkerPart::Dot,
                &model.value,
                &model.dot_label,
                "Marker dot",
            )
            .invalid(invalid),
        ),
        marker_node(
            model,
            state,
            MarkerNodeDraft::new(
                MarkerPart::Label,
                &model.value,
                &model.label,
                "Marker label",
            )
            .invalid(invalid),
        ),
        marker_node(
            model,
            state,
            MarkerNodeDraft::new(
                MarkerPart::Anchor,
                anchor.map_or("marker-anchor", |anchor| anchor.href.as_str()),
                anchor.map_or("No marker anchor", |anchor| anchor.label.as_str()),
                "Marker anchor",
            )
            .visible(anchor.is_some())
            .actionable(anchor.is_some())
            .disabled(anchor.is_none_or(|anchor| anchor.disabled)),
        ),
    ]
}

pub fn default_marker_model() -> MarkerModel {
    MarkerModel::new("3 new")
}

struct MarkerNodeDraft<'a> {
    part: MarkerPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    invalid: bool,
    actionable: bool,
    disabled: bool,
}

impl<'a> MarkerNodeDraft<'a> {
    const fn new(part: MarkerPart, value: &'a str, label: &'a str, detail: &'a str) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            visible: true,
            invalid: false,
            actionable: false,
            disabled: false,
        }
    }

    const fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    const fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    const fn actionable(mut self, actionable: bool) -> Self {
        self.actionable = actionable;
        self
    }

    const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

fn marker_node(
    model: &MarkerModel,
    state: &MarkerState,
    draft: MarkerNodeDraft<'_>,
) -> MarkerRenderNode {
    MarkerRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        tone: model.tone,
        active: state.is_active(draft.part),
        invalid: draft.invalid,
        visible: draft.visible,
        actionable: draft.actionable,
        loading: model.loading,
        disabled: draft.disabled || model.disabled || (model.loading && draft.actionable),
    }
}

fn validate_optional_marker_anchor(anchor: &Option<MarkerAnchor>, _context: &()) -> garde::Result {
    if let Some(anchor) = anchor {
        anchor.validate().map_err(|report| {
            garde::Error::new(format!("invalid marker anchor contract: {report}"))
        })?;
    }
    Ok(())
}

fn validate_optional_marker_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && (error.is_empty() || error.len() > 240)
    {
        return Err(garde::Error::new(
            "marker optional error must be 1..=240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_marker_model(&default_marker_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = MarkerModel::new("");
        assert!(validate_marker_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = default_marker_model().with_value("");
        assert!(validate_marker_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_dot_label() {
        let model = default_marker_model().with_dot_label("");
        assert!(validate_marker_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_anchor_contract() {
        let model = default_marker_model().with_anchor(MarkerAnchor::new("", ""));
        assert!(validate_marker_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_marker_model().with_error("");
        assert!(validate_marker_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_hover_navigation_and_clear() {
        let mut state = MarkerState::resting();
        assert_eq!(
            state.apply(MarkerIntent::Focus(MarkerPart::Dot)),
            MarkerChange::Focused(MarkerPart::Dot)
        );
        assert!(state.is_active(MarkerPart::Dot));
        assert_eq!(state.apply(MarkerIntent::Blur), MarkerChange::Blurred);
        assert_eq!(
            state.apply(MarkerIntent::Hover(MarkerPart::Label)),
            MarkerChange::Hovered(MarkerPart::Label)
        );
        assert!(state.is_active(MarkerPart::Label));
        assert_eq!(state.apply(MarkerIntent::Leave), MarkerChange::Left);
        assert_eq!(
            state.apply(MarkerIntent::Navigate("#latest".to_owned())),
            MarkerChange::Navigated("#latest".to_owned())
        );
        assert!(state.is_active(MarkerPart::Anchor));
        assert_eq!(state.navigated_href(), Some("#latest"));
        assert_eq!(state.apply(MarkerIntent::Clear), MarkerChange::Cleared);
        assert!(!state.is_active(MarkerPart::Anchor));
        assert_eq!(state.navigated_href(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_marker_model();
        let nodes = marker_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), MarkerPart::ALL.len());
        assert_eq!(nodes.first().map(|node| node.part), Some(MarkerPart::Root));
        for part in MarkerPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn missing_anchor_keeps_hidden_disabled_anchor_node() {
        let model = default_marker_model().without_anchor();
        let nodes = marker_render_nodes(&model, &model.state());
        let anchor = nodes
            .into_iter()
            .find(|node| node.part == MarkerPart::Anchor)
            .expect("marker render nodes include anchor");
        assert!(!anchor.visible);
        assert!(anchor.disabled);
        assert!(!anchor.actionable);
    }

    #[test]
    fn loading_disables_actionable_anchor_node() {
        let model = default_marker_model().loading();
        let nodes = marker_render_nodes(&model, &model.state());
        let anchor = nodes
            .into_iter()
            .find(|node| node.part == MarkerPart::Anchor)
            .expect("marker render nodes include anchor");
        assert!(anchor.visible);
        assert!(anchor.actionable);
        assert!(anchor.disabled);
    }

    #[test]
    fn error_marks_root_dot_and_label_invalid() {
        let model = default_marker_model().with_error("Unread position is stale.");
        let nodes = marker_render_nodes(&model, &model.state());
        for part in [MarkerPart::Root, MarkerPart::Dot, MarkerPart::Label] {
            assert!(
                nodes.iter().any(|node| node.part == part && node.invalid),
                "missing invalid state for {}",
                part.label()
            );
        }
    }

    #[test]
    fn layout_metrics_preserve_dense_tone_and_state_precedence() {
        let standard = marker_layout_metrics(&default_marker_model(), 1_000.0);
        let dense = marker_layout_metrics(
            &default_marker_model().with_density(MarkerDensity::Dense),
            1_000.0,
        );
        let dense_warning = marker_layout_metrics(
            &default_marker_model()
                .with_density(MarkerDensity::Dense)
                .with_tone(MarkerTone::Warning),
            1_000.0,
        );
        let dense_loading = marker_layout_metrics(
            &default_marker_model()
                .with_density(MarkerDensity::Dense)
                .loading(),
            1_000.0,
        );

        assert!(dense.root_gap < standard.root_gap);
        assert!(dense.root_padding_inline < standard.root_padding_inline);
        assert!(dense.dot_size < standard.dot_size);
        assert_eq!(dense_warning.dot_size, standard.dot_size);
        assert_eq!(dense_loading.root_gap, standard.root_gap);
        assert_eq!(dense_loading.dot_size, standard.dot_size);
    }
}
