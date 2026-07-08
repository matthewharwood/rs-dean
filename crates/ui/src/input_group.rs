use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::input::{InputAction, InputDensity, InputKind};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputGroupPart {
    Root,
    Addon,
    Input,
    Button,
}

impl InputGroupPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Addon, Self::Input, Self::Button];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "InputGroup",
            Self::Addon => "InputGroupAddon",
            Self::Input => "InputGroupInput",
            Self::Button => "InputGroupButton",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct InputGroupModel {
    #[garde(skip)]
    pub density: InputDensity,
    #[garde(skip)]
    pub input_kind: InputKind,
    #[garde(length(max = 240))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub placeholder: String,
    #[garde(custom(validate_optional_input_group_copy))]
    pub addon: Option<String>,
    #[garde(custom(validate_optional_input_group_action))]
    pub button: Option<InputAction>,
    #[garde(custom(validate_optional_input_group_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputGroupState {
    focused: bool,
    value: String,
    active_part: Option<InputGroupPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputGroupIntent {
    Focus,
    Blur,
    Input(String),
    ActivateButton(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputGroupChange {
    Focused,
    Blurred,
    Input(String),
    ButtonActivated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputGroupRenderNode {
    pub part: InputGroupPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: InputDensity,
    pub input_kind: InputKind,
    pub focused: bool,
    pub active: bool,
    pub invalid: bool,
    pub visible: bool,
    pub actionable: bool,
    pub required: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl InputGroupModel {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            density: InputDensity::Standard,
            input_kind: InputKind::Text,
            value: String::new(),
            placeholder: placeholder.into(),
            addon: None,
            button: None,
            error: None,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: InputDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_input_kind(mut self, input_kind: InputKind) -> Self {
        self.input_kind = input_kind;
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_addon(mut self, addon: impl Into<String>) -> Self {
        self.addon = Some(addon.into());
        self
    }

    pub fn without_addon(mut self) -> Self {
        self.addon = None;
        self
    }

    pub fn with_button(mut self, button: InputAction) -> Self {
        self.button = Some(button);
        self
    }

    pub fn without_button(mut self) -> Self {
        self.button = None;
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

    pub fn state(&self) -> InputGroupState {
        InputGroupState::new(self.value.clone())
    }
}

impl InputGroupState {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            focused: false,
            value: value.into(),
            active_part: None,
        }
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub const fn is_active(&self, part: InputGroupPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn apply(&mut self, intent: InputGroupIntent) -> InputGroupChange {
        match intent {
            InputGroupIntent::Focus => self.focus(),
            InputGroupIntent::Blur => self.blur(),
            InputGroupIntent::Input(value) => self.input(value),
            InputGroupIntent::ActivateButton(value) => self.activate_button(value),
            InputGroupIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self) -> InputGroupChange {
        if self.focused {
            InputGroupChange::Unchanged
        } else {
            self.focused = true;
            self.active_part = Some(InputGroupPart::Input);
            InputGroupChange::Focused
        }
    }

    fn blur(&mut self) -> InputGroupChange {
        if self.focused || self.active_part.is_some() {
            self.focused = false;
            self.active_part = None;
            InputGroupChange::Blurred
        } else {
            InputGroupChange::Unchanged
        }
    }

    fn input(&mut self, value: String) -> InputGroupChange {
        if self.value == value {
            InputGroupChange::Unchanged
        } else {
            self.focused = true;
            self.active_part = Some(InputGroupPart::Input);
            self.value = value.clone();
            InputGroupChange::Input(value)
        }
    }

    fn activate_button(&mut self, value: String) -> InputGroupChange {
        if value.is_empty() {
            InputGroupChange::Unchanged
        } else {
            self.active_part = Some(InputGroupPart::Button);
            InputGroupChange::ButtonActivated(value)
        }
    }

    fn clear(&mut self) -> InputGroupChange {
        if self.value.is_empty() {
            InputGroupChange::Unchanged
        } else {
            self.value.clear();
            self.active_part = None;
            InputGroupChange::Cleared
        }
    }
}

pub fn validate_input_group_model(model: &InputGroupModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn input_group_render_nodes(
    model: &InputGroupModel,
    state: &InputGroupState,
) -> Vec<InputGroupRenderNode> {
    let blocked = model.loading || model.disabled;
    let addon = model.addon.as_deref().unwrap_or("No addon");
    let button = model.button.as_ref();
    let button_label = button
        .map(|button| button.label.as_str())
        .unwrap_or("No button");
    let button_value = button
        .map(|button| button.value.as_str())
        .unwrap_or("input-group-button");
    let button_disabled = button.map(|button| button.disabled).unwrap_or(true);
    let control_detail = model
        .error
        .as_deref()
        .unwrap_or("Grouped single-line input");

    vec![
        input_group_node(
            model,
            state,
            InputGroupNodeDraft::new(
                InputGroupPart::Root,
                "input-group",
                "InputGroup",
                control_detail,
                true,
                false,
                model.disabled,
            ),
        ),
        input_group_node(
            model,
            state,
            InputGroupNodeDraft::new(
                InputGroupPart::Addon,
                "input-group-addon",
                addon,
                "Input group addon",
                model.addon.is_some(),
                false,
                model.addon.is_none() || model.disabled,
            ),
        ),
        input_group_node(
            model,
            state,
            InputGroupNodeDraft::new(
                InputGroupPart::Input,
                state.value(),
                &model.placeholder,
                control_detail,
                true,
                false,
                blocked,
            ),
        ),
        input_group_node(
            model,
            state,
            InputGroupNodeDraft::new(
                InputGroupPart::Button,
                button_value,
                button_label,
                "Input group button",
                button.is_some(),
                button.is_some() && !button_disabled,
                button.is_none() || button_disabled || blocked,
            ),
        ),
    ]
}

pub fn default_input_group_model() -> InputGroupModel {
    InputGroupModel::new("42")
        .with_value("42")
        .with_addon("$")
        .with_button(InputAction::new("Apply", "apply-value"))
}

struct InputGroupNodeDraft<'a> {
    part: InputGroupPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    actionable: bool,
    disabled: bool,
}

impl<'a> InputGroupNodeDraft<'a> {
    const fn new(
        part: InputGroupPart,
        value: &'a str,
        label: &'a str,
        detail: &'a str,
        visible: bool,
        actionable: bool,
        disabled: bool,
    ) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            visible,
            actionable,
            disabled,
        }
    }
}

fn input_group_node(
    model: &InputGroupModel,
    state: &InputGroupState,
    draft: InputGroupNodeDraft<'_>,
) -> InputGroupRenderNode {
    InputGroupRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        input_kind: model.input_kind,
        focused: state.is_focused(),
        active: state.is_active(draft.part),
        invalid: model.error.is_some(),
        visible: draft.visible,
        actionable: draft.actionable,
        required: model.required,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn validate_optional_input_group_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && (copy.is_empty() || copy.len() > 320)
    {
        return Err(garde::Error::new(
            "input group optional copy must be 1..=320 characters",
        ));
    }
    Ok(())
}

fn validate_optional_input_group_action(
    action: &Option<InputAction>,
    _context: &(),
) -> garde::Result {
    if let Some(action) = action
        && action.validate().is_err()
    {
        return Err(garde::Error::new(
            "input group button must include non-empty label and value",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_input_group_model(&default_input_group_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_placeholder() {
        let model = InputGroupModel::new("");
        assert!(validate_input_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_addon() {
        let model = default_input_group_model().with_addon("");
        assert!(validate_input_group_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_button_contract() {
        let model = default_input_group_model().with_button(InputAction::new("", "apply"));
        assert!(validate_input_group_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_input_button_activation_and_clear() {
        let mut state = InputGroupState::new("42");
        assert_eq!(
            state.apply(InputGroupIntent::Focus),
            InputGroupChange::Focused
        );
        assert_eq!(
            state.apply(InputGroupIntent::Input("64".to_owned())),
            InputGroupChange::Input("64".to_owned())
        );
        assert_eq!(state.value(), "64");
        assert_eq!(
            state.apply(InputGroupIntent::ActivateButton("apply-value".to_owned())),
            InputGroupChange::ButtonActivated("apply-value".to_owned())
        );
        assert!(state.is_active(InputGroupPart::Button));
        assert_eq!(
            state.apply(InputGroupIntent::Clear),
            InputGroupChange::Cleared
        );
        assert_eq!(
            state.apply(InputGroupIntent::Blur),
            InputGroupChange::Blurred
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_input_group_model();
        let nodes = input_group_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), InputGroupPart::ALL.len());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(InputGroupPart::Root)
        );
        for part in InputGroupPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn missing_addon_and_button_keep_disabled_nodes() {
        let model = default_input_group_model().without_addon().without_button();
        let nodes = input_group_render_nodes(&model, &model.state());
        for part in [InputGroupPart::Addon, InputGroupPart::Button] {
            let node = nodes
                .iter()
                .find(|node| node.part == part)
                .expect("input group render nodes include optional anatomy");
            assert!(!node.visible);
            assert!(node.disabled);
        }
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_input_group_model().with_error("Amount is required.");
        let nodes = input_group_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(nodes.iter().any(|node| {
            node.part == InputGroupPart::Input && node.detail == "Amount is required."
        }));
    }
}
