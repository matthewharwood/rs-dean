use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmptyDensity {
    Standard,
    Dense,
}

impl EmptyDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmptyPart {
    Root,
    Header,
    Title,
    Description,
    Content,
    Action,
}

impl EmptyPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Header,
        Self::Title,
        Self::Description,
        Self::Content,
        Self::Action,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Empty",
            Self::Header => "EmptyHeader",
            Self::Title => "EmptyTitle",
            Self::Description => "EmptyDescription",
            Self::Content => "EmptyContent",
            Self::Action => "EmptyAction",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct EmptyAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct EmptyModel {
    #[garde(skip)]
    pub density: EmptyDensity,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 320))]
    pub description: String,
    #[garde(length(min = 1, max = 2_000))]
    pub content: String,
    #[garde(custom(validate_optional_empty_illustration_label))]
    pub illustration_label: Option<String>,
    #[garde(custom(validate_optional_empty_action))]
    pub action: Option<EmptyAction>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyState {
    active_part: Option<EmptyPart>,
    activated_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmptyIntent {
    Focus(EmptyPart),
    Blur(EmptyPart),
    Activate(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmptyChange {
    Focused(EmptyPart),
    Blurred(EmptyPart),
    Activated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyRenderNode {
    pub part: EmptyPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: EmptyDensity,
    pub active: bool,
    pub actionable: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmptyLayoutMetrics {
    pub max_width: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub header_gap: f32,
    pub title_font_size: f32,
    pub title_line_height: f32,
    pub description_font_size: f32,
    pub description_line_height: f32,
    pub content_padding: f32,
    pub content_gap: f32,
    pub marker_size: f32,
    pub marker_font_size: f32,
    pub marker_line_height: f32,
    pub content_font_size: f32,
    pub content_line_height: f32,
    pub action_min_height: f32,
    pub action_padding_inline: f32,
    pub action_padding_block: f32,
    pub action_font_size: f32,
    pub action_line_height: f32,
}

impl EmptyAction {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl EmptyModel {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            density: EmptyDensity::Standard,
            title: title.into(),
            description: description.into(),
            content: "Adjust filters, create a record, or restore saved state to continue."
                .to_owned(),
            illustration_label: Some("No data".to_owned()),
            action: Some(EmptyAction::new("Create item", "create-item")),
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: EmptyDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn with_illustration_label(mut self, illustration_label: impl Into<String>) -> Self {
        self.illustration_label = Some(illustration_label.into());
        self
    }

    pub fn without_illustration(mut self) -> Self {
        self.illustration_label = None;
        self
    }

    pub fn with_action(mut self, action: EmptyAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn without_action(mut self) -> Self {
        self.action = None;
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

    pub const fn state(&self) -> EmptyState {
        let _ = self;
        EmptyState::resting()
    }
}

impl EmptyState {
    pub const fn resting() -> Self {
        Self {
            active_part: None,
            activated_value: None,
        }
    }

    pub const fn is_active(&self, part: EmptyPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn activated_value(&self) -> Option<&str> {
        self.activated_value.as_deref()
    }

    pub fn apply(&mut self, intent: EmptyIntent) -> EmptyChange {
        match intent {
            EmptyIntent::Focus(part) => self.focus(part),
            EmptyIntent::Blur(part) => self.blur(part),
            EmptyIntent::Activate(value) => self.activate(value),
            EmptyIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, part: EmptyPart) -> EmptyChange {
        if self.is_active(part) {
            EmptyChange::Unchanged
        } else {
            self.active_part = Some(part);
            EmptyChange::Focused(part)
        }
    }

    fn blur(&mut self, part: EmptyPart) -> EmptyChange {
        if self.is_active(part) {
            self.active_part = None;
            EmptyChange::Blurred(part)
        } else {
            EmptyChange::Unchanged
        }
    }

    fn activate(&mut self, value: String) -> EmptyChange {
        if value.is_empty() {
            EmptyChange::Unchanged
        } else {
            self.active_part = Some(EmptyPart::Action);
            self.activated_value = Some(value.clone());
            EmptyChange::Activated(value)
        }
    }

    fn clear(&mut self) -> EmptyChange {
        if self.active_part.is_none() && self.activated_value.is_none() {
            EmptyChange::Unchanged
        } else {
            self.active_part = None;
            self.activated_value = None;
            EmptyChange::Cleared
        }
    }
}

pub fn validate_empty_model(model: &EmptyModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn empty_layout_metrics(
    density: EmptyDensity,
    loading: bool,
    disabled: bool,
    inline_size: f32,
) -> EmptyLayoutMetrics {
    let dense = density == EmptyDensity::Dense;
    let dense_root = dense && !loading && !disabled;
    let dense_action = dense && !loading && !disabled;

    EmptyLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_padding: if dense_root {
            scale::space::s(inline_size)
        } else {
            scale::space::m(inline_size)
        },
        root_gap: if dense_root {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        header_gap: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
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
        description_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        description_line_height: scale::line_height::LH0,
        content_padding: if dense {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        content_gap: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        marker_size: if dense {
            scale::space::l(inline_size)
        } else {
            scale::space::xl(inline_size)
        },
        marker_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        marker_line_height: scale::line_height::LH0,
        content_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        content_line_height: scale::line_height::LH0,
        action_min_height: if dense_action {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        action_padding_inline: if dense_action {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        action_padding_block: if dense_action {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        action_font_size: if dense_action {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        action_line_height: scale::line_height::LH0,
    }
}

pub fn empty_render_nodes(model: &EmptyModel, state: &EmptyState) -> Vec<EmptyRenderNode> {
    let action = model.action.as_ref();
    let action_disabled = action.is_none_or(|action| action.disabled);
    vec![
        empty_node(
            model,
            state,
            EmptyNodeDraft::new(
                EmptyPart::Root,
                "empty",
                "Empty",
                &model.title,
                false,
                model.disabled,
            ),
        ),
        empty_node(
            model,
            state,
            EmptyNodeDraft::new(
                EmptyPart::Header,
                "empty-header",
                "Empty header",
                &model.title,
                false,
                model.disabled,
            ),
        ),
        empty_node(
            model,
            state,
            EmptyNodeDraft::new(
                EmptyPart::Title,
                "empty-title",
                &model.title,
                "Empty title",
                false,
                model.disabled,
            ),
        ),
        empty_node(
            model,
            state,
            EmptyNodeDraft::new(
                EmptyPart::Description,
                "empty-description",
                "Empty description",
                &model.description,
                false,
                model.disabled,
            ),
        ),
        empty_node(
            model,
            state,
            EmptyNodeDraft::new(
                EmptyPart::Content,
                "empty-content",
                model
                    .illustration_label
                    .as_deref()
                    .unwrap_or("Empty content"),
                &model.content,
                false,
                model.disabled,
            ),
        ),
        empty_node(
            model,
            state,
            EmptyNodeDraft::new(
                EmptyPart::Action,
                action
                    .map(|action| action.value.as_str())
                    .unwrap_or("empty-action"),
                action
                    .map(|action| action.label.as_str())
                    .unwrap_or("Empty action"),
                action
                    .map(|_| "Recovery action")
                    .unwrap_or("No recovery action configured"),
                action.is_some(),
                model.disabled || model.loading || action_disabled,
            ),
        ),
    ]
}

pub fn default_empty_model() -> EmptyModel {
    EmptyModel::new(
        "No components yet",
        "Create a component to start exercising the shared UI contract.",
    )
}

struct EmptyNodeDraft<'a> {
    part: EmptyPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    actionable: bool,
    disabled: bool,
}

impl<'a> EmptyNodeDraft<'a> {
    const fn new(
        part: EmptyPart,
        value: &'a str,
        label: &'a str,
        detail: &'a str,
        actionable: bool,
        disabled: bool,
    ) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            actionable,
            disabled,
        }
    }
}

fn empty_node(
    model: &EmptyModel,
    state: &EmptyState,
    draft: EmptyNodeDraft<'_>,
) -> EmptyRenderNode {
    EmptyRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        active: state.is_active(draft.part),
        actionable: draft.actionable,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn validate_optional_empty_illustration_label(
    illustration_label: &Option<String>,
    _context: &(),
) -> garde::Result {
    if let Some(illustration_label) = illustration_label
        && (illustration_label.is_empty() || illustration_label.len() > 64)
    {
        return Err(garde::Error::new(
            "empty illustration label must be 1..=64 characters",
        ));
    }
    Ok(())
}

fn validate_optional_empty_action(action: &Option<EmptyAction>, _context: &()) -> garde::Result {
    if let Some(action) = action {
        action.validate().map_err(|report| {
            garde::Error::new(format!("invalid empty action contract: {report}"))
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_empty_model(&default_empty_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = EmptyModel::new("", "Description");
        assert!(validate_empty_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_description() {
        let model = EmptyModel::new("Nothing here", "");
        assert!(validate_empty_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_content() {
        let model = default_empty_model().with_content("");
        assert!(validate_empty_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_action_contract() {
        let model = default_empty_model().with_action(EmptyAction::new("", "invalid-action"));
        assert!(validate_empty_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_activation_and_clear() {
        let mut state = EmptyState::resting();
        assert!(!state.is_active(EmptyPart::Action));
        assert_eq!(
            state.apply(EmptyIntent::Focus(EmptyPart::Action)),
            EmptyChange::Focused(EmptyPart::Action)
        );
        assert!(state.is_active(EmptyPart::Action));
        assert_eq!(
            state.apply(EmptyIntent::Activate("create-item".to_owned())),
            EmptyChange::Activated("create-item".to_owned())
        );
        assert_eq!(state.activated_value(), Some("create-item"));
        assert_eq!(state.apply(EmptyIntent::Clear), EmptyChange::Cleared);
        assert_eq!(state.activated_value(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_empty_model();
        let nodes = empty_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), EmptyPart::ALL.len());
        assert_eq!(nodes.first().map(|node| node.part), Some(EmptyPart::Root));
        for part in EmptyPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn missing_action_keeps_disabled_action_node() {
        let model = default_empty_model().without_action();
        let nodes = empty_render_nodes(&model, &model.state());
        let action = nodes
            .into_iter()
            .find(|node| node.part == EmptyPart::Action)
            .expect("empty render nodes include action");
        assert!(!action.actionable);
        assert!(action.disabled);
    }

    #[test]
    fn loading_disables_action_node() {
        let model = default_empty_model().loading();
        let nodes = empty_render_nodes(&model, &model.state());
        let action = nodes
            .into_iter()
            .find(|node| node.part == EmptyPart::Action)
            .expect("empty render nodes include action");
        assert!(action.disabled);
    }

    #[test]
    fn layout_metrics_preserve_density_and_blocked_state_token_scales() {
        let standard = empty_layout_metrics(EmptyDensity::Standard, false, false, 1_024.0);
        let dense = empty_layout_metrics(EmptyDensity::Dense, false, false, 1_024.0);
        let loading_dense = empty_layout_metrics(EmptyDensity::Dense, true, false, 1_024.0);

        assert_eq!(standard.max_width, scale::container::CONTROL);
        assert!(dense.root_padding < standard.root_padding);
        assert!(dense.marker_size < standard.marker_size);
        assert!(dense.action_font_size < standard.action_font_size);
        assert_eq!(loading_dense.root_padding, standard.root_padding);
        assert_eq!(loading_dense.action_min_height, standard.action_min_height);
        assert_eq!(loading_dense.title_font_size, dense.title_font_size);
    }
}
