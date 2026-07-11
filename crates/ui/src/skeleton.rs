use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SkeletonDensity {
    Standard,
    Dense,
}

impl SkeletonDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SkeletonPart {
    Root,
    Block,
    Text,
    Media,
}

impl SkeletonPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Block, Self::Text, Self::Media];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Skeleton",
            Self::Block => "SkeletonBlock",
            Self::Text => "SkeletonText",
            Self::Media => "SkeletonMedia",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SkeletonModel {
    #[garde(skip)]
    pub density: SkeletonDensity,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 96))]
    pub block_label: String,
    #[garde(length(min = 1, max = 96))]
    pub text_label: String,
    #[garde(length(min = 1, max = 96))]
    pub media_label: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(range(min = 1, max = 6))]
    pub text_lines: u8,
    #[garde(custom(validate_optional_skeleton_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub animated: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonState {
    active_part: Option<SkeletonPart>,
    animation_paused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkeletonIntent {
    Focus(SkeletonPart),
    Blur(SkeletonPart),
    PauseAnimation,
    ResumeAnimation,
    ToggleAnimation,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkeletonChange {
    Focused(SkeletonPart),
    Blurred(SkeletonPart),
    AnimationPaused,
    AnimationResumed,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonRenderNode {
    pub part: SkeletonPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: SkeletonDensity,
    pub text_lines: u8,
    pub animated: bool,
    pub animation_paused: bool,
    pub active: bool,
    pub visible: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkeletonLayoutMetrics {
    pub max_width: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub content_gap: f32,
    pub text_gap: f32,
    pub block_height: f32,
    pub line_height: f32,
    pub media_min_height: f32,
    pub status_font_size: f32,
    pub status_line_height: f32,
    pub status_tracking_em: f32,
    pub error_padding: f32,
    pub error_font_size: f32,
    pub error_line_height: f32,
    pub shadow_level: u8,
    pub dense_root: bool,
}

impl SkeletonModel {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            density: SkeletonDensity::Standard,
            label: label.into(),
            block_label: "Block placeholder".to_owned(),
            text_label: "Text placeholder".to_owned(),
            media_label: "Media placeholder".to_owned(),
            detail: "Preserves layout while content loads from local-first state.".to_owned(),
            text_lines: 3,
            error: None,
            animated: true,
            loading: true,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SkeletonDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_block_label(mut self, label: impl Into<String>) -> Self {
        self.block_label = label.into();
        self
    }

    pub fn with_text_label(mut self, label: impl Into<String>) -> Self {
        self.text_label = label.into();
        self
    }

    pub fn with_media_label(mut self, label: impl Into<String>) -> Self {
        self.media_label = label.into();
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub const fn with_text_lines(mut self, text_lines: u8) -> Self {
        self.text_lines = text_lines;
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

    pub const fn animated(mut self) -> Self {
        self.animated = true;
        self
    }

    pub const fn static_placeholder(mut self) -> Self {
        self.animated = false;
        self
    }

    pub const fn loading(mut self) -> Self {
        self.loading = true;
        self
    }

    pub const fn ready(mut self) -> Self {
        self.loading = false;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn state(&self) -> SkeletonState {
        let _ = self;
        SkeletonState::new()
    }
}

impl SkeletonState {
    pub const fn new() -> Self {
        Self {
            active_part: None,
            animation_paused: false,
        }
    }

    pub const fn is_active(&self, part: SkeletonPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub const fn animation_paused(&self) -> bool {
        self.animation_paused
    }

    pub fn apply(&mut self, intent: SkeletonIntent) -> SkeletonChange {
        match intent {
            SkeletonIntent::Focus(part) => self.focus(part),
            SkeletonIntent::Blur(part) => self.blur(part),
            SkeletonIntent::PauseAnimation => self.pause_animation(),
            SkeletonIntent::ResumeAnimation => self.resume_animation(),
            SkeletonIntent::ToggleAnimation => {
                if self.animation_paused {
                    self.resume_animation()
                } else {
                    self.pause_animation()
                }
            }
            SkeletonIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, part: SkeletonPart) -> SkeletonChange {
        if self.is_active(part) {
            SkeletonChange::Unchanged
        } else {
            self.active_part = Some(part);
            SkeletonChange::Focused(part)
        }
    }

    fn blur(&mut self, part: SkeletonPart) -> SkeletonChange {
        if self.is_active(part) {
            self.active_part = None;
            SkeletonChange::Blurred(part)
        } else {
            SkeletonChange::Unchanged
        }
    }

    fn pause_animation(&mut self) -> SkeletonChange {
        if self.animation_paused {
            SkeletonChange::Unchanged
        } else {
            self.animation_paused = true;
            SkeletonChange::AnimationPaused
        }
    }

    fn resume_animation(&mut self) -> SkeletonChange {
        if self.animation_paused {
            self.animation_paused = false;
            SkeletonChange::AnimationResumed
        } else {
            SkeletonChange::Unchanged
        }
    }

    fn clear(&mut self) -> SkeletonChange {
        if self.active_part.is_some() || self.animation_paused {
            self.active_part = None;
            self.animation_paused = false;
            SkeletonChange::Cleared
        } else {
            SkeletonChange::Unchanged
        }
    }
}

impl Default for SkeletonState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn validate_skeleton_model(model: &SkeletonModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn skeleton_layout_metrics(model: &SkeletonModel, inline_size: f32) -> SkeletonLayoutMetrics {
    let dense = model.density == SkeletonDensity::Dense;
    let dense_root = skeleton_uses_dense_root_metrics(
        dense,
        model.loading,
        model.error.is_some(),
        model.disabled,
    );
    SkeletonLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_padding: if dense_root {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        root_gap: if dense_root {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        content_gap: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        text_gap: scale::space::xs2(inline_size),
        block_height: if dense {
            scale::space::m(inline_size)
        } else {
            scale::space::l(inline_size)
        },
        line_height: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        media_min_height: if dense {
            scale::space::l(inline_size)
        } else {
            scale::space::xl(inline_size)
        },
        status_font_size: scale::font_size::f00(inline_size),
        status_line_height: scale::line_height::LH0,
        status_tracking_em: 0.08,
        error_padding: scale::space::xs(inline_size),
        error_font_size: scale::font_size::f0(inline_size),
        error_line_height: scale::line_height::LH0,
        shadow_level: u8::from(model.loading && !model.disabled),
        dense_root,
    }
}

pub const fn skeleton_uses_dense_root_metrics(
    dense: bool,
    loading: bool,
    invalid: bool,
    disabled: bool,
) -> bool {
    dense && loading && !invalid && !disabled
}

pub const fn skeleton_placeholder_uses_border(active: bool, invalid: bool, disabled: bool) -> bool {
    !disabled && (active || invalid)
}

pub fn skeleton_render_nodes(
    model: &SkeletonModel,
    state: &SkeletonState,
) -> Vec<SkeletonRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.disabled || !model.loading;
    let animated = model.animated && model.loading && !model.disabled && !state.animation_paused();
    let detail = model.error.clone().unwrap_or_else(|| model.detail.clone());

    vec![
        SkeletonRenderNode {
            part: SkeletonPart::Root,
            value: "root".to_owned(),
            label: model.label.clone(),
            detail,
            density: model.density,
            text_lines: model.text_lines,
            animated,
            animation_paused: state.animation_paused(),
            active: state.is_active(SkeletonPart::Root),
            visible: true,
            invalid,
            loading: model.loading,
            disabled: model.disabled,
        },
        SkeletonRenderNode {
            part: SkeletonPart::Block,
            value: "block".to_owned(),
            label: model.block_label.clone(),
            detail: "Block placeholder preserves the primary content height.".to_owned(),
            density: model.density,
            text_lines: model.text_lines,
            animated,
            animation_paused: state.animation_paused(),
            active: state.is_active(SkeletonPart::Block),
            visible: model.loading,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        SkeletonRenderNode {
            part: SkeletonPart::Text,
            value: "text".to_owned(),
            label: model.text_label.clone(),
            detail: format!(
                "{} text placeholder lines preserve copy rhythm.",
                model.text_lines
            ),
            density: model.density,
            text_lines: model.text_lines,
            animated,
            animation_paused: state.animation_paused(),
            active: state.is_active(SkeletonPart::Text),
            visible: model.loading,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        SkeletonRenderNode {
            part: SkeletonPart::Media,
            value: "media".to_owned(),
            label: model.media_label.clone(),
            detail: "Media placeholder preserves the final image aspect area.".to_owned(),
            density: model.density,
            text_lines: model.text_lines,
            animated,
            animation_paused: state.animation_paused(),
            active: state.is_active(SkeletonPart::Media),
            visible: model.loading,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
    ]
}

pub fn default_skeleton_model() -> SkeletonModel {
    SkeletonModel::new("Loading profile card")
        .with_block_label("Header block")
        .with_text_label("Body copy")
        .with_media_label("Preview media")
}

fn validate_optional_skeleton_error(value: &Option<String>, _: &()) -> garde::Result {
    if let Some(value) = value
        && (value.is_empty() || value.len() > 240)
    {
        return Err(garde::Error::new(
            "skeleton error must be between 1 and 240 characters when present",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_skeleton_model(&default_skeleton_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = default_skeleton_model().with_detail("");
        assert!(validate_skeleton_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_skeleton_model().with_error("");
        assert!(validate_skeleton_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_out_of_range_text_lines() {
        let model = default_skeleton_model().with_text_lines(0);
        assert!(validate_skeleton_model(&model).is_err());

        let model = default_skeleton_model().with_text_lines(7);
        assert!(validate_skeleton_model(&model).is_err());
    }

    #[test]
    fn layout_metrics_follow_fluid_density_and_root_state_precedence() {
        let standard = skeleton_layout_metrics(&default_skeleton_model(), 1_000.0);
        let dense_model = default_skeleton_model().with_density(SkeletonDensity::Dense);
        let dense = skeleton_layout_metrics(&dense_model, 1_000.0);
        let dense_ready = skeleton_layout_metrics(&dense_model.clone().ready(), 1_000.0);
        let dense_disabled = skeleton_layout_metrics(&dense_model.disabled(), 1_000.0);

        assert_eq!(standard.max_width, scale::container::CONTROL);
        assert_eq!(standard.block_height, scale::space::l(1_000.0));
        assert!(dense.block_height < standard.block_height);
        assert!(dense.root_padding < standard.root_padding);
        assert_eq!(dense_ready.root_padding, standard.root_padding);
        assert_eq!(dense_disabled.root_padding, standard.root_padding);
        assert_eq!(dense.shadow_level, 1);
        assert_eq!(dense_ready.shadow_level, 0);
        assert_eq!(dense_disabled.shadow_level, 0);
    }

    #[test]
    fn placeholder_border_precedence_matches_renderer_classes() {
        assert!(!skeleton_placeholder_uses_border(false, false, false));
        assert!(skeleton_placeholder_uses_border(true, false, false));
        assert!(skeleton_placeholder_uses_border(false, true, false));
        assert!(!skeleton_placeholder_uses_border(true, true, true));
    }

    #[test]
    fn ready_model_hides_placeholder_parts() {
        let model = default_skeleton_model().ready();
        let state = model.state();
        let nodes = skeleton_render_nodes(&model, &state);

        assert!(
            nodes
                .iter()
                .filter(|node| node.part != SkeletonPart::Root)
                .all(|node| !node.visible && node.disabled)
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_skeleton_model();
        let state = model.state();
        let nodes = skeleton_render_nodes(&model, &state);

        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(SkeletonPart::Root)
        );
        for part in SkeletonPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn state_tracks_focus_pause_resume_and_clear() {
        let model = default_skeleton_model();
        let mut state = model.state();

        assert!(!state.is_active(SkeletonPart::Text));
        assert_eq!(
            state.apply(SkeletonIntent::Focus(SkeletonPart::Text)),
            SkeletonChange::Focused(SkeletonPart::Text)
        );
        assert!(state.is_active(SkeletonPart::Text));
        assert_eq!(
            state.apply(SkeletonIntent::PauseAnimation),
            SkeletonChange::AnimationPaused
        );
        assert!(state.animation_paused());
        assert_eq!(state.apply(SkeletonIntent::Clear), SkeletonChange::Cleared);
        assert!(!state.is_active(SkeletonPart::Text));
        assert!(!state.animation_paused());
    }
}
