use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToggleDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToggleVariant {
    Default,
    Outline,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TogglePressed {
    Unpressed,
    Pressed,
}

impl ToggleDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl ToggleVariant {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
        }
    }
}

impl TogglePressed {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unpressed => "unpressed",
            Self::Pressed => "pressed",
        }
    }

    pub const fn aria_pressed(self) -> &'static str {
        match self {
            Self::Unpressed => "false",
            Self::Pressed => "true",
        }
    }

    pub const fn is_pressed(self) -> bool {
        matches!(self, Self::Pressed)
    }

    pub const fn toggle_target(self) -> Self {
        match self {
            Self::Unpressed => Self::Pressed,
            Self::Pressed => Self::Unpressed,
        }
    }
}

impl TogglePart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Indicator, Self::Label];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Toggle",
            Self::Indicator => "ToggleIndicator",
            Self::Label => "ToggleLabel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ToggleModel {
    #[garde(skip)]
    pub density: ToggleDensity,
    #[garde(skip)]
    pub variant: ToggleVariant,
    #[garde(skip)]
    pub pressed: TogglePressed,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(length(min = 1, max = 48))]
    pub pressed_label: String,
    #[garde(length(min = 1, max = 48))]
    pub unpressed_label: String,
    #[garde(length(min = 1, max = 24))]
    pub pressed_indicator: String,
    #[garde(length(min = 1, max = 24))]
    pub unpressed_indicator: String,
    #[garde(custom(validate_optional_toggle_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToggleState {
    pressed: TogglePressed,
    active_part: Option<TogglePart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleIntent {
    Toggle,
    Set(TogglePressed),
    Focus(TogglePart),
    Blur,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleChange {
    Changed(TogglePressed),
    Focused(TogglePart),
    Blurred,
    Reset,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TogglePart {
    Root,
    Indicator,
    Label,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToggleRenderNode {
    pub part: TogglePart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: ToggleDensity,
    pub variant: ToggleVariant,
    pub pressed: TogglePressed,
    pub active: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
    pub actionable: bool,
}

impl ToggleModel {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            density: ToggleDensity::Standard,
            variant: ToggleVariant::Default,
            pressed: TogglePressed::Unpressed,
            label: label.into(),
            value: value.into(),
            detail: "Renderer-local pressed state until the consumer persists the tool choice."
                .to_owned(),
            pressed_label: "Pressed".to_owned(),
            unpressed_label: "Unpressed".to_owned(),
            pressed_indicator: "on".to_owned(),
            unpressed_indicator: "off".to_owned(),
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: ToggleDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_pressed(mut self, pressed: TogglePressed) -> Self {
        self.pressed = pressed;
        self
    }

    pub const fn pressed(mut self) -> Self {
        self.pressed = TogglePressed::Pressed;
        self
    }

    pub const fn unpressed(mut self) -> Self {
        self.pressed = TogglePressed::Unpressed;
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub fn with_pressed_label(mut self, label: impl Into<String>) -> Self {
        self.pressed_label = label.into();
        self
    }

    pub fn with_unpressed_label(mut self, label: impl Into<String>) -> Self {
        self.unpressed_label = label.into();
        self
    }

    pub fn with_pressed_indicator(mut self, indicator: impl Into<String>) -> Self {
        self.pressed_indicator = indicator.into();
        self
    }

    pub fn with_unpressed_indicator(mut self, indicator: impl Into<String>) -> Self {
        self.unpressed_indicator = indicator.into();
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
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

    pub const fn state(&self) -> ToggleState {
        ToggleState::new(self.pressed)
    }
}

impl ToggleState {
    pub const fn new(pressed: TogglePressed) -> Self {
        Self {
            pressed,
            active_part: None,
        }
    }

    pub const fn pressed(self) -> TogglePressed {
        self.pressed
    }

    pub const fn is_pressed(self) -> bool {
        self.pressed.is_pressed()
    }

    pub const fn active_part(self) -> Option<TogglePart> {
        self.active_part
    }

    pub const fn is_active(self, part: TogglePart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn apply(&mut self, intent: ToggleIntent) -> ToggleChange {
        match intent {
            ToggleIntent::Toggle => self.set(self.pressed.toggle_target()),
            ToggleIntent::Set(pressed) => self.set(pressed),
            ToggleIntent::Focus(part) => {
                if self.active_part == Some(part) {
                    ToggleChange::Unchanged
                } else {
                    self.active_part = Some(part);
                    ToggleChange::Focused(part)
                }
            }
            ToggleIntent::Blur => {
                if self.active_part.is_none() {
                    ToggleChange::Unchanged
                } else {
                    self.active_part = None;
                    ToggleChange::Blurred
                }
            }
            ToggleIntent::Reset => {
                if self.pressed == TogglePressed::Unpressed && self.active_part.is_none() {
                    ToggleChange::Unchanged
                } else {
                    self.pressed = TogglePressed::Unpressed;
                    self.active_part = None;
                    ToggleChange::Reset
                }
            }
        }
    }

    fn set(&mut self, pressed: TogglePressed) -> ToggleChange {
        if self.pressed == pressed {
            ToggleChange::Unchanged
        } else {
            self.pressed = pressed;
            ToggleChange::Changed(pressed)
        }
    }
}

pub fn validate_toggle_model(model: &ToggleModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn toggle_render_nodes(model: &ToggleModel, state: &ToggleState) -> Vec<ToggleRenderNode> {
    let pressed = state.pressed();
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let status = toggle_status_label(model, pressed);
    let indicator = toggle_indicator_label(model, pressed);
    let detail = model.error.clone().unwrap_or_else(|| model.detail.clone());
    vec![
        ToggleRenderNode {
            part: TogglePart::Root,
            value: model.value.clone(),
            label: model.label.clone(),
            detail: status.clone(),
            density: model.density,
            variant: model.variant,
            pressed,
            active: state.is_active(TogglePart::Root),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: !blocked,
        },
        ToggleRenderNode {
            part: TogglePart::Indicator,
            value: model.value.clone(),
            label: indicator,
            detail: status,
            density: model.density,
            variant: model.variant,
            pressed,
            active: state.is_active(TogglePart::Indicator),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: false,
        },
        ToggleRenderNode {
            part: TogglePart::Label,
            value: model.value.clone(),
            label: model.label.clone(),
            detail,
            density: model.density,
            variant: model.variant,
            pressed,
            active: state.is_active(TogglePart::Label),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: false,
        },
    ]
}

pub fn default_toggle_model() -> ToggleModel {
    ToggleModel::new("Bold", "bold")
        .with_detail("Pressing the toggle updates local tool state; the editor persists accepted formatting.")
        .with_pressed_label("Active")
        .with_unpressed_label("Inactive")
        .with_pressed_indicator("on")
        .with_unpressed_indicator("off")
        .pressed()
}

fn toggle_status_label(model: &ToggleModel, pressed: TogglePressed) -> String {
    match pressed {
        TogglePressed::Unpressed => model.unpressed_label.clone(),
        TogglePressed::Pressed => model.pressed_label.clone(),
    }
}

fn toggle_indicator_label(model: &ToggleModel, pressed: TogglePressed) -> String {
    match pressed {
        TogglePressed::Unpressed => model.unpressed_indicator.clone(),
        TogglePressed::Pressed => model.pressed_indicator.clone(),
    }
}

fn validate_optional_toggle_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new("toggle copy must be 1 to 240 characters"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_toggle_model(&default_toggle_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = ToggleModel::new("", "bold");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = ToggleModel::new("Bold", "");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_status_labels() {
        let model = default_toggle_model().with_pressed_label("");
        assert!(validate_toggle_model(&model).is_err());
        let model = default_toggle_model().with_unpressed_label("");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_indicator_labels() {
        let model = default_toggle_model().with_pressed_indicator("");
        assert!(validate_toggle_model(&model).is_err());
        let model = default_toggle_model().with_unpressed_indicator("");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_toggle_model().with_error("");
        assert!(validate_toggle_model(&model).is_err());
    }

    #[test]
    fn state_toggles_sets_focuses_and_resets() {
        let mut state = ToggleState::new(TogglePressed::Unpressed);
        assert!(!state.is_pressed());
        assert_eq!(
            state.apply(ToggleIntent::Toggle),
            ToggleChange::Changed(TogglePressed::Pressed)
        );
        assert!(state.is_pressed());
        assert_eq!(
            state.apply(ToggleIntent::Set(TogglePressed::Unpressed)),
            ToggleChange::Changed(TogglePressed::Unpressed)
        );
        assert_eq!(
            state.apply(ToggleIntent::Focus(TogglePart::Root)),
            ToggleChange::Focused(TogglePart::Root)
        );
        assert!(state.is_active(TogglePart::Root));
        assert_eq!(state.apply(ToggleIntent::Reset), ToggleChange::Reset);
        assert_eq!(state.pressed(), TogglePressed::Unpressed);
        assert_eq!(state.active_part(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_toggle_model();
        let nodes = toggle_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(TogglePart::Root));
        for part in TogglePart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_toggle_model().with_error("Toolbar state failed validation.");
        let nodes = toggle_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(nodes.iter().any(|node| node.part == TogglePart::Label
            && node.detail == "Toolbar state failed validation."));
    }

    #[test]
    fn loading_disables_root_action() {
        let model = default_toggle_model().loading();
        let nodes = toggle_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == TogglePart::Root && node.disabled && !node.actionable)
        );
    }
}
