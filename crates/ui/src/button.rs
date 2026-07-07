use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
    Icon,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ButtonKind {
    Button,
    Submit,
    Reset,
    Link,
}

impl ButtonVariant {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Destructive => "destructive",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Link => "link",
        }
    }
}

impl ButtonSize {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
            Self::Icon => "icon",
        }
    }
}

impl ButtonKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
            Self::Link => "link",
        }
    }

    pub const fn button_type(self) -> &'static str {
        match self {
            Self::Button | Self::Link => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

impl ButtonPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Button",
            Self::Icon => "ButtonIcon",
            Self::Label => "ButtonLabel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ButtonModel {
    #[garde(skip)]
    pub variant: ButtonVariant,
    #[garde(skip)]
    pub size: ButtonSize,
    #[garde(skip)]
    pub kind: ButtonKind,
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(custom(validate_button_icon_for_size(&self.size)))]
    pub icon: Option<String>,
    #[garde(custom(validate_button_href_for_kind(&self.kind)))]
    pub href: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonState {
    pressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonIntent {
    Press,
    Release,
    Activate(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonChange {
    Pressed,
    Released,
    Activated(String),
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonPart {
    Root,
    Icon,
    Label,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonRenderNode {
    pub part: ButtonPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub kind: ButtonKind,
    pub pressed: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl ButtonModel {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            kind: ButtonKind::Button,
            label: label.into(),
            value: value.into(),
            icon: Some("Go".to_owned()),
            href: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub const fn with_kind(mut self, kind: ButtonKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn without_icon(mut self) -> Self {
        self.icon = None;
        self
    }

    pub fn as_link(mut self, href: impl Into<String>) -> Self {
        self.kind = ButtonKind::Link;
        self.href = Some(href.into());
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

    pub const fn state(&self) -> ButtonState {
        let _ = self;
        ButtonState::idle()
    }
}

impl ButtonState {
    pub const fn idle() -> Self {
        Self { pressed: false }
    }

    pub const fn is_pressed(self) -> bool {
        self.pressed
    }

    pub fn apply(&mut self, intent: ButtonIntent) -> ButtonChange {
        match intent {
            ButtonIntent::Press => self.press(),
            ButtonIntent::Release => self.release(),
            ButtonIntent::Activate(value) => {
                if value.is_empty() {
                    ButtonChange::Unchanged
                } else {
                    ButtonChange::Activated(value)
                }
            }
        }
    }

    fn press(&mut self) -> ButtonChange {
        if self.pressed {
            ButtonChange::Unchanged
        } else {
            self.pressed = true;
            ButtonChange::Pressed
        }
    }

    fn release(&mut self) -> ButtonChange {
        if self.pressed {
            self.pressed = false;
            ButtonChange::Released
        } else {
            ButtonChange::Unchanged
        }
    }
}

pub fn validate_button_model(model: &ButtonModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn button_render_nodes(model: &ButtonModel, state: ButtonState) -> Vec<ButtonRenderNode> {
    let icon_value = model
        .icon
        .clone()
        .unwrap_or_else(|| "button-icon-missing".to_owned());
    vec![
        ButtonRenderNode {
            part: ButtonPart::Root,
            value: model.value.clone(),
            label: model.label.clone(),
            detail: format!("{} {} button", model.size.label(), model.variant.label()),
            variant: model.variant,
            size: model.size,
            kind: model.kind,
            pressed: state.is_pressed(),
            loading: model.loading,
            disabled: model.disabled || model.loading,
        },
        ButtonRenderNode {
            part: ButtonPart::Icon,
            value: icon_value.clone(),
            label: icon_value,
            detail: "Button icon".to_owned(),
            variant: model.variant,
            size: model.size,
            kind: model.kind,
            pressed: state.is_pressed(),
            loading: model.loading,
            disabled: model.disabled || model.loading || model.icon.is_none(),
        },
        ButtonRenderNode {
            part: ButtonPart::Label,
            value: model.label.clone(),
            label: model.label.clone(),
            detail: "Button label".to_owned(),
            variant: model.variant,
            size: model.size,
            kind: model.kind,
            pressed: state.is_pressed(),
            loading: model.loading,
            disabled: model.disabled || model.loading,
        },
    ]
}

pub fn default_button_model() -> ButtonModel {
    ButtonModel::new("Continue", "continue")
}

fn validate_button_icon_for_size(
    size: &ButtonSize,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + '_ {
    move |icon, _context| {
        if let Some(icon) = icon
            && !(1..=16).contains(&icon.chars().count())
        {
            return Err(garde::Error::new("button icon must be 1 to 16 characters"));
        }
        if *size == ButtonSize::Icon && icon.is_none() {
            return Err(garde::Error::new(
                "icon-sized button must include an icon label",
            ));
        }
        Ok(())
    }
}

fn validate_button_href_for_kind(
    kind: &ButtonKind,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + '_ {
    move |href, _context| {
        if let Some(href) = href
            && !(1..=512).contains(&href.chars().count())
        {
            return Err(garde::Error::new("button href must be 1 to 512 characters"));
        }
        if *kind == ButtonKind::Link && href.is_none() {
            return Err(garde::Error::new("link button must include an href"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_button_model(&default_button_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = ButtonModel::new("", "continue");
        assert!(validate_button_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = ButtonModel::new("Continue", "");
        assert!(validate_button_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_icon() {
        let model = ButtonModel::new("Continue", "continue").with_icon("");
        assert!(validate_button_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_icon_size_without_icon() {
        let model = ButtonModel::new("Continue", "continue")
            .with_size(ButtonSize::Icon)
            .without_icon();
        assert!(validate_button_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_link_kind_without_href() {
        let model = ButtonModel::new("Continue", "continue").with_kind(ButtonKind::Link);
        assert!(validate_button_model(&model).is_err());
    }

    #[test]
    fn state_tracks_press_release() {
        let mut state = ButtonState::idle();
        assert!(!state.is_pressed());
        assert_eq!(state.apply(ButtonIntent::Press), ButtonChange::Pressed);
        assert!(state.is_pressed());
        assert_eq!(state.apply(ButtonIntent::Release), ButtonChange::Released);
        assert!(!state.is_pressed());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_button_model();
        let nodes = button_render_nodes(&model, model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(ButtonPart::Root));
        assert!(nodes.iter().any(|node| node.part == ButtonPart::Icon));
        assert!(nodes.iter().any(|node| node.part == ButtonPart::Label));
    }

    #[test]
    fn missing_icon_keeps_disabled_icon_node() {
        let model = default_button_model().without_icon();
        let nodes = button_render_nodes(&model, model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == ButtonPart::Icon && node.disabled)
        );
    }
}
