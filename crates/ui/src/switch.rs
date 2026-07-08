use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwitchDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwitchChecked {
    Off,
    On,
}

impl SwitchDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl SwitchChecked {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::On => "on",
        }
    }

    pub const fn aria_checked(self) -> &'static str {
        match self {
            Self::Off => "false",
            Self::On => "true",
        }
    }

    pub const fn is_on(self) -> bool {
        matches!(self, Self::On)
    }

    pub const fn toggle_target(self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

impl SwitchPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Track, Self::Thumb, Self::Label];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Switch",
            Self::Track => "SwitchTrack",
            Self::Thumb => "SwitchThumb",
            Self::Label => "SwitchLabel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SwitchModel {
    #[garde(skip)]
    pub density: SwitchDensity,
    #[garde(skip)]
    pub checked: SwitchChecked,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(length(min = 1, max = 48))]
    pub on_label: String,
    #[garde(length(min = 1, max = 48))]
    pub off_label: String,
    #[garde(custom(validate_optional_switch_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchState {
    checked: SwitchChecked,
    active_part: Option<SwitchPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchIntent {
    Toggle,
    Set(SwitchChecked),
    Focus(SwitchPart),
    Blur,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchChange {
    Changed(SwitchChecked),
    Focused(SwitchPart),
    Blurred,
    Reset,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwitchPart {
    Root,
    Track,
    Thumb,
    Label,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchRenderNode {
    pub part: SwitchPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: SwitchDensity,
    pub checked: SwitchChecked,
    pub required: bool,
    pub active: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
    pub actionable: bool,
}

impl SwitchModel {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            density: SwitchDensity::Standard,
            checked: SwitchChecked::Off,
            label: label.into(),
            value: value.into(),
            detail: "Renderer-local draft state until the consumer persists the choice.".to_owned(),
            on_label: "On".to_owned(),
            off_label: "Off".to_owned(),
            error: None,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SwitchDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_checked(mut self, checked: SwitchChecked) -> Self {
        self.checked = checked;
        self
    }

    pub const fn checked(mut self) -> Self {
        self.checked = SwitchChecked::On;
        self
    }

    pub const fn unchecked(mut self) -> Self {
        self.checked = SwitchChecked::Off;
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub fn with_on_label(mut self, label: impl Into<String>) -> Self {
        self.on_label = label.into();
        self
    }

    pub fn with_off_label(mut self, label: impl Into<String>) -> Self {
        self.off_label = label.into();
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub const fn required(mut self) -> Self {
        self.required = true;
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

    pub const fn state(&self) -> SwitchState {
        SwitchState::new(self.checked)
    }
}

impl SwitchState {
    pub const fn new(checked: SwitchChecked) -> Self {
        Self {
            checked,
            active_part: None,
        }
    }

    pub const fn checked(self) -> SwitchChecked {
        self.checked
    }

    pub const fn is_on(self) -> bool {
        self.checked.is_on()
    }

    pub const fn active_part(self) -> Option<SwitchPart> {
        self.active_part
    }

    pub const fn is_active(self, part: SwitchPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn apply(&mut self, intent: SwitchIntent) -> SwitchChange {
        match intent {
            SwitchIntent::Toggle => self.set(self.checked.toggle_target()),
            SwitchIntent::Set(checked) => self.set(checked),
            SwitchIntent::Focus(part) => {
                if self.active_part == Some(part) {
                    SwitchChange::Unchanged
                } else {
                    self.active_part = Some(part);
                    SwitchChange::Focused(part)
                }
            }
            SwitchIntent::Blur => {
                if self.active_part.is_none() {
                    SwitchChange::Unchanged
                } else {
                    self.active_part = None;
                    SwitchChange::Blurred
                }
            }
            SwitchIntent::Reset => {
                if self.checked == SwitchChecked::Off && self.active_part.is_none() {
                    SwitchChange::Unchanged
                } else {
                    self.checked = SwitchChecked::Off;
                    self.active_part = None;
                    SwitchChange::Reset
                }
            }
        }
    }

    fn set(&mut self, checked: SwitchChecked) -> SwitchChange {
        if self.checked == checked {
            SwitchChange::Unchanged
        } else {
            self.checked = checked;
            SwitchChange::Changed(checked)
        }
    }
}

pub fn validate_switch_model(model: &SwitchModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn switch_render_nodes(model: &SwitchModel, state: &SwitchState) -> Vec<SwitchRenderNode> {
    let checked = state.checked();
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let status = switch_status_label(model, checked);
    let detail = model.error.clone().unwrap_or_else(|| model.detail.clone());
    vec![
        SwitchRenderNode {
            part: SwitchPart::Root,
            value: model.value.clone(),
            label: model.label.clone(),
            detail: format!("{} {} switch", model.density.label(), checked.label()),
            density: model.density,
            checked,
            required: model.required,
            active: state.is_active(SwitchPart::Root),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: !blocked,
        },
        SwitchRenderNode {
            part: SwitchPart::Track,
            value: model.value.clone(),
            label: "Switch track".to_owned(),
            detail: status.clone(),
            density: model.density,
            checked,
            required: model.required,
            active: state.is_active(SwitchPart::Track),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: !blocked,
        },
        SwitchRenderNode {
            part: SwitchPart::Thumb,
            value: model.value.clone(),
            label: "Switch thumb".to_owned(),
            detail: status,
            density: model.density,
            checked,
            required: model.required,
            active: state.is_active(SwitchPart::Thumb),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: false,
        },
        SwitchRenderNode {
            part: SwitchPart::Label,
            value: model.value.clone(),
            label: model.label.clone(),
            detail,
            density: model.density,
            checked,
            required: model.required,
            active: state.is_active(SwitchPart::Label),
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: false,
        },
    ]
}

pub fn default_switch_model() -> SwitchModel {
    SwitchModel::new("Enable shared theme", "shared-theme")
        .with_detail("Persist this choice through app state while the renderer owns focus only.")
        .with_on_label("Enabled")
        .with_off_label("Disabled")
        .checked()
}

fn switch_status_label(model: &SwitchModel, checked: SwitchChecked) -> String {
    match checked {
        SwitchChecked::Off => model.off_label.clone(),
        SwitchChecked::On => model.on_label.clone(),
    }
}

fn validate_optional_switch_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new("switch copy must be 1 to 240 characters"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_switch_model(&default_switch_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = SwitchModel::new("", "shared-theme");
        assert!(validate_switch_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = SwitchModel::new("Enable shared theme", "");
        assert!(validate_switch_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_status_labels() {
        let model = default_switch_model().with_on_label("");
        assert!(validate_switch_model(&model).is_err());
        let model = default_switch_model().with_off_label("");
        assert!(validate_switch_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_switch_model().with_error("");
        assert!(validate_switch_model(&model).is_err());
    }

    #[test]
    fn state_toggles_sets_focuses_and_resets() {
        let mut state = SwitchState::new(SwitchChecked::Off);
        assert!(!state.is_on());
        assert_eq!(
            state.apply(SwitchIntent::Toggle),
            SwitchChange::Changed(SwitchChecked::On)
        );
        assert!(state.is_on());
        assert_eq!(
            state.apply(SwitchIntent::Set(SwitchChecked::Off)),
            SwitchChange::Changed(SwitchChecked::Off)
        );
        assert_eq!(
            state.apply(SwitchIntent::Focus(SwitchPart::Track)),
            SwitchChange::Focused(SwitchPart::Track)
        );
        assert!(state.is_active(SwitchPart::Track));
        assert_eq!(state.apply(SwitchIntent::Reset), SwitchChange::Reset);
        assert_eq!(state.checked(), SwitchChecked::Off);
        assert_eq!(state.active_part(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_switch_model();
        let nodes = switch_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(SwitchPart::Root));
        for part in SwitchPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn error_marks_label_invalid() {
        let model = default_switch_model().with_error("Choose a valid setting before continuing.");
        let nodes = switch_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(nodes.iter().any(|node| node.part == SwitchPart::Label
            && node.detail == "Choose a valid setting before continuing."));
    }

    #[test]
    fn loading_disables_actionable_nodes() {
        let model = default_switch_model().loading();
        let nodes = switch_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == SwitchPart::Track && node.disabled && !node.actionable)
        );
    }
}
