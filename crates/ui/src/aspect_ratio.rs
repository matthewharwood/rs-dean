use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AspectRatioFit {
    Cover,
    Contain,
}

impl AspectRatioFit {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cover => "cover",
            Self::Contain => "contain",
        }
    }
}

impl AspectRatioPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "AspectRatio",
            Self::Frame => "AspectRatioFrame",
            Self::Media => "AspectRatioMedia",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AspectRatioModel {
    #[garde(length(min = 1, max = 160))]
    pub label: String,
    #[garde(length(min = 1, max = 2_000))]
    pub description: String,
    #[garde(custom(validate_ratio_side))]
    pub width: u16,
    #[garde(custom(validate_ratio_side))]
    pub height: u16,
    #[garde(skip)]
    pub fit: AspectRatioFit,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectRatioPart {
    Root,
    Frame,
    Media,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectRatioRenderNode {
    pub part: AspectRatioPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub width: u16,
    pub height: u16,
    pub fit: AspectRatioFit,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AspectRatioLayoutMetrics {
    pub width: f32,
    pub height: f32,
    pub frame_height: f32,
    pub gap: f32,
    pub frame_padding: f32,
    pub media_padding: f32,
    pub marker_size: f32,
    pub caption_height: f32,
    pub caption_text_gap: f32,
    pub label_font_size: f32,
    pub detail_font_size: f32,
    pub badge_height: f32,
    pub badge_padding_inline: f32,
    pub badge_padding_block: f32,
}

impl AspectRatioModel {
    pub fn new(label: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: description.into(),
            width: 16,
            height: 9,
            fit: AspectRatioFit::Cover,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_ratio(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub const fn with_fit(mut self, fit: AspectRatioFit) -> Self {
        self.fit = fit;
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

    pub fn ratio_label(&self) -> String {
        format!("{}:{}", self.width, self.height)
    }

    pub fn aspect_ratio_style(&self) -> String {
        format!("aspect-ratio: {} / {};", self.width, self.height)
    }
}

pub fn validate_aspect_ratio_model(model: &AspectRatioModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn aspect_ratio_render_nodes(model: &AspectRatioModel) -> Vec<AspectRatioRenderNode> {
    vec![
        AspectRatioRenderNode {
            part: AspectRatioPart::Root,
            value: "aspect-ratio".to_owned(),
            label: "Aspect Ratio".to_owned(),
            detail: "Stable media frame".to_owned(),
            width: model.width,
            height: model.height,
            fit: model.fit,
            loading: model.loading,
            disabled: model.disabled,
        },
        AspectRatioRenderNode {
            part: AspectRatioPart::Frame,
            value: model.ratio_label(),
            label: model.label.clone(),
            detail: "Aspect ratio frame".to_owned(),
            width: model.width,
            height: model.height,
            fit: model.fit,
            loading: model.loading,
            disabled: model.disabled,
        },
        AspectRatioRenderNode {
            part: AspectRatioPart::Media,
            value: model.fit.label().to_owned(),
            label: model.label.clone(),
            detail: model.description.clone(),
            width: model.width,
            height: model.height,
            fit: model.fit,
            loading: model.loading,
            disabled: model.disabled,
        },
    ]
}

pub fn aspect_ratio_layout_metrics(
    model: &AspectRatioModel,
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> AspectRatioLayoutMetrics {
    let width = available_width.max(1.0);
    let border_width = border_width.max(0.0);
    let frame_height = width * f32::from(model.height) / f32::from(model.width);
    let gap = scale::space::xs2(inline_size);
    let frame_padding = gap;
    let media_padding = scale::space::s(inline_size);
    let marker_size = scale::space::xl(inline_size);
    let caption_text_gap = scale::space::xs3(inline_size);
    let label_font_size = scale::font_size::f0(inline_size);
    let detail_font_size = scale::font_size::f00(inline_size);
    let caption_text_height = label_font_size * scale::line_height::LH0
        + caption_text_gap
        + detail_font_size * scale::line_height::LH0;
    let badge_padding_inline = gap;
    let badge_padding_block = caption_text_gap;
    let badge_height =
        detail_font_size * scale::line_height::LH0 + badge_padding_block * 2.0 + border_width * 2.0;
    let caption_height = caption_text_height.max(badge_height);
    let height = frame_height + gap + caption_height;

    AspectRatioLayoutMetrics {
        width,
        height,
        frame_height,
        gap,
        frame_padding,
        media_padding,
        marker_size,
        caption_height,
        caption_text_gap,
        label_font_size,
        detail_font_size,
        badge_height,
        badge_padding_inline,
        badge_padding_block,
    }
}

pub fn default_aspect_ratio_model() -> AspectRatioModel {
    AspectRatioModel::new(
        "Course preview",
        "A stable 16:9 frame for image, canvas, or embedded media content.",
    )
}

fn validate_ratio_side(side: &u16, _context: &()) -> garde::Result {
    if (1..=64).contains(side) {
        Ok(())
    } else {
        Err(garde::Error::new(
            "aspect ratio sides must be between 1 and 64",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_aspect_ratio_model(&default_aspect_ratio_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = AspectRatioModel::new("", "Description");
        assert!(validate_aspect_ratio_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_zero_ratio_side() {
        let model = AspectRatioModel::new("Preview", "Description").with_ratio(0, 9);
        assert!(validate_aspect_ratio_model(&model).is_err());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let nodes = aspect_ratio_render_nodes(&default_aspect_ratio_model());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(AspectRatioPart::Root)
        );
        assert!(nodes.iter().any(|node| node.part == AspectRatioPart::Frame));
        assert!(nodes.iter().any(|node| node.part == AspectRatioPart::Media));
    }

    #[test]
    fn ratio_label_and_style_stay_stable() {
        let model = AspectRatioModel::new("Square", "Avatar crop").with_ratio(1, 1);
        assert_eq!(model.ratio_label(), "1:1");
        assert_eq!(model.aspect_ratio_style(), "aspect-ratio: 1 / 1;");
    }

    #[test]
    fn layout_metrics_preserve_the_ratio_without_a_renderer_cap() {
        let model = default_aspect_ratio_model();
        let metrics = aspect_ratio_layout_metrics(&model, 860.0, 1_000.0, 1.0);

        assert!((metrics.frame_height - 483.75).abs() < 0.01);
        assert!(metrics.frame_height > 260.0);
        assert!(
            (metrics.height - metrics.frame_height - metrics.gap - metrics.caption_height).abs()
                < 0.01
        );
    }
}
