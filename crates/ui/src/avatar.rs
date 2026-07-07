use garde::Validate;
use serde::{Deserialize, Serialize};

const DEFAULT_AVATAR_IMAGE_SRC: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 64 64'%3E%3Crect width='64' height='64' fill='%230f766e'/%3E%3Ctext x='32' y='39' text-anchor='middle' font-size='22' fill='white' font-family='Arial'%3EMH%3C/text%3E%3C/svg%3E";

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AvatarVisual {
    Image,
    Fallback,
}

impl AvatarSize {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
        }
    }
}

impl AvatarVisual {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Fallback => "fallback",
        }
    }
}

impl AvatarPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Avatar",
            Self::Image => "AvatarImage",
            Self::Fallback => "AvatarFallback",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AvatarImage {
    #[garde(length(min = 1, max = 2_048))]
    pub src: String,
    #[garde(length(min = 1, max = 160))]
    pub alt: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AvatarModel {
    #[garde(skip)]
    pub size: AvatarSize,
    #[garde(length(min = 1, max = 160))]
    pub name: String,
    #[garde(length(min = 1, max = 8))]
    pub fallback: String,
    #[garde(custom(validate_optional_avatar_image))]
    pub image: Option<AvatarImage>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvatarState {
    visual: AvatarVisual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AvatarIntent {
    ImageLoaded,
    ImageFailed,
    ShowFallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AvatarChange {
    ShowingImage,
    ShowingFallback,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarPart {
    Root,
    Image,
    Fallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvatarRenderNode {
    pub part: AvatarPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub size: AvatarSize,
    pub visual: AvatarVisual,
    pub loading: bool,
    pub disabled: bool,
}

impl AvatarImage {
    pub fn new(src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            alt: alt.into(),
        }
    }
}

impl AvatarModel {
    pub fn new(name: impl Into<String>, fallback: impl Into<String>) -> Self {
        Self {
            size: AvatarSize::Medium,
            name: name.into(),
            fallback: fallback.into(),
            image: Some(AvatarImage::new(
                DEFAULT_AVATAR_IMAGE_SRC,
                "Matthew Harwood profile image",
            )),
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_image(mut self, image: AvatarImage) -> Self {
        self.image = Some(image);
        self
    }

    pub fn without_image(mut self) -> Self {
        self.image = None;
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

    pub fn state(&self) -> AvatarState {
        if self.image.is_some() && !self.loading {
            AvatarState::showing_image()
        } else {
            AvatarState::showing_fallback()
        }
    }
}

impl AvatarState {
    pub const fn showing_image() -> Self {
        Self {
            visual: AvatarVisual::Image,
        }
    }

    pub const fn showing_fallback() -> Self {
        Self {
            visual: AvatarVisual::Fallback,
        }
    }

    pub const fn visual(self) -> AvatarVisual {
        self.visual
    }

    pub fn apply(&mut self, intent: AvatarIntent) -> AvatarChange {
        match intent {
            AvatarIntent::ImageLoaded => self.show_image(),
            AvatarIntent::ImageFailed | AvatarIntent::ShowFallback => self.show_fallback(),
        }
    }

    fn show_image(&mut self) -> AvatarChange {
        if self.visual == AvatarVisual::Image {
            AvatarChange::Unchanged
        } else {
            self.visual = AvatarVisual::Image;
            AvatarChange::ShowingImage
        }
    }

    fn show_fallback(&mut self) -> AvatarChange {
        if self.visual == AvatarVisual::Fallback {
            AvatarChange::Unchanged
        } else {
            self.visual = AvatarVisual::Fallback;
            AvatarChange::ShowingFallback
        }
    }
}

pub fn validate_avatar_model(model: &AvatarModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn avatar_render_nodes(model: &AvatarModel, state: AvatarState) -> Vec<AvatarRenderNode> {
    let image_value = model
        .image
        .as_ref()
        .map(|image| image.src.clone())
        .unwrap_or_else(|| "missing-image".to_owned());
    let image_label = model
        .image
        .as_ref()
        .map(|image| image.alt.clone())
        .unwrap_or_else(|| format!("{} image", model.name));

    vec![
        AvatarRenderNode {
            part: AvatarPart::Root,
            value: "avatar".to_owned(),
            label: model.name.clone(),
            detail: "Identity avatar".to_owned(),
            size: model.size,
            visual: state.visual(),
            loading: model.loading,
            disabled: model.disabled,
        },
        AvatarRenderNode {
            part: AvatarPart::Image,
            value: image_value,
            label: image_label,
            detail: "Avatar image".to_owned(),
            size: model.size,
            visual: state.visual(),
            loading: model.loading,
            disabled: model.disabled || model.loading || model.image.is_none(),
        },
        AvatarRenderNode {
            part: AvatarPart::Fallback,
            value: model.fallback.clone(),
            label: model.name.clone(),
            detail: "Avatar fallback".to_owned(),
            size: model.size,
            visual: state.visual(),
            loading: model.loading,
            disabled: model.disabled,
        },
    ]
}

pub fn default_avatar_model() -> AvatarModel {
    AvatarModel::new("Matthew Harwood", "MH")
}

fn validate_optional_avatar_image(image: &Option<AvatarImage>, _context: &()) -> garde::Result {
    if let Some(image) = image {
        image
            .validate()
            .map_err(|report| garde::Error::new(format!("invalid avatar image: {report}")))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_avatar_model(&default_avatar_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_name() {
        let model = AvatarModel::new("", "MH");
        assert!(validate_avatar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_fallback() {
        let model = AvatarModel::new("Matthew Harwood", "");
        assert!(validate_avatar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_image_contract() {
        let model = AvatarModel::new("Matthew Harwood", "MH").with_image(AvatarImage::new("", ""));
        assert!(validate_avatar_model(&model).is_err());
    }

    #[test]
    fn state_transitions_between_image_and_fallback() {
        let mut state = AvatarState::showing_image();
        assert_eq!(state.visual(), AvatarVisual::Image);
        assert_eq!(
            state.apply(AvatarIntent::ImageFailed),
            AvatarChange::ShowingFallback
        );
        assert_eq!(state.visual(), AvatarVisual::Fallback);
        assert_eq!(
            state.apply(AvatarIntent::ImageLoaded),
            AvatarChange::ShowingImage
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_avatar_model();
        let nodes = avatar_render_nodes(&model, model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(AvatarPart::Root));
        assert!(nodes.iter().any(|node| node.part == AvatarPart::Image));
        assert!(nodes.iter().any(|node| node.part == AvatarPart::Fallback));
    }

    #[test]
    fn missing_image_starts_with_fallback() {
        let model = default_avatar_model().without_image();
        assert_eq!(model.state().visual(), AvatarVisual::Fallback);
        let nodes = avatar_render_nodes(&model, model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AvatarPart::Image && node.disabled)
        );
    }
}
