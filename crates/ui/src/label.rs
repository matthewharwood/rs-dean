use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LabelDensity {
    Standard,
    Dense,
}

impl LabelDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LabelRequirement {
    None,
    Optional,
    Required,
}

impl LabelRequirement {
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Optional => "optional",
            Self::Required => "required",
        }
    }

    pub const fn marker(self) -> &'static str {
        match self {
            Self::None => "No requirement",
            Self::Optional => "Optional",
            Self::Required => "Required",
        }
    }

    pub const fn visible(self) -> bool {
        !matches!(self, Self::None)
    }

    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LabelPart {
    Root,
    Text,
    Requirement,
}

impl LabelPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Text, Self::Requirement];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Label",
            Self::Text => "LabelText",
            Self::Requirement => "LabelRequirement",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct LabelModel {
    #[garde(skip)]
    pub density: LabelDensity,
    #[garde(length(min = 1, max = 160))]
    pub text: String,
    #[garde(custom(validate_optional_label_control_id))]
    pub control_id: Option<String>,
    #[garde(skip)]
    pub requirement: LabelRequirement,
    #[garde(custom(validate_optional_label_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelState {
    active_part: Option<LabelPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelIntent {
    Focus(LabelPart),
    Hover(LabelPart),
    Blur,
    Leave,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelChange {
    Focused(LabelPart),
    Hovered(LabelPart),
    Blurred,
    Left,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelRenderNode {
    pub part: LabelPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: LabelDensity,
    pub requirement: LabelRequirement,
    pub active: bool,
    pub invalid: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl LabelModel {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            density: LabelDensity::Standard,
            text: text.into(),
            control_id: None,
            requirement: LabelRequirement::None,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: LabelDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_for(mut self, control_id: impl Into<String>) -> Self {
        self.control_id = Some(control_id.into());
        self
    }

    pub fn without_for(mut self) -> Self {
        self.control_id = None;
        self
    }

    pub const fn with_requirement(mut self, requirement: LabelRequirement) -> Self {
        self.requirement = requirement;
        self
    }

    pub const fn required(mut self) -> Self {
        self.requirement = LabelRequirement::Required;
        self
    }

    pub const fn optional(mut self) -> Self {
        self.requirement = LabelRequirement::Optional;
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

    pub const fn state(&self) -> LabelState {
        let _ = self;
        LabelState::resting()
    }
}

impl LabelState {
    pub const fn resting() -> Self {
        Self { active_part: None }
    }

    pub const fn is_active(&self, part: LabelPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub fn apply(&mut self, intent: LabelIntent) -> LabelChange {
        match intent {
            LabelIntent::Focus(part) => self.focus(part),
            LabelIntent::Hover(part) => self.hover(part),
            LabelIntent::Blur => self.blur(),
            LabelIntent::Leave => self.leave(),
            LabelIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, part: LabelPart) -> LabelChange {
        if self.is_active(part) {
            LabelChange::Unchanged
        } else {
            self.active_part = Some(part);
            LabelChange::Focused(part)
        }
    }

    fn hover(&mut self, part: LabelPart) -> LabelChange {
        if self.is_active(part) {
            LabelChange::Unchanged
        } else {
            self.active_part = Some(part);
            LabelChange::Hovered(part)
        }
    }

    fn blur(&mut self) -> LabelChange {
        if self.active_part.is_some() {
            self.active_part = None;
            LabelChange::Blurred
        } else {
            LabelChange::Unchanged
        }
    }

    fn leave(&mut self) -> LabelChange {
        if self.active_part.is_some() {
            self.active_part = None;
            LabelChange::Left
        } else {
            LabelChange::Unchanged
        }
    }

    fn clear(&mut self) -> LabelChange {
        if self.active_part.is_some() {
            self.active_part = None;
            LabelChange::Cleared
        } else {
            LabelChange::Unchanged
        }
    }
}

pub fn validate_label_model(model: &LabelModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn label_render_nodes(model: &LabelModel, state: &LabelState) -> Vec<LabelRenderNode> {
    let invalid = model.error.is_some();
    let root_detail = model.error.as_deref().unwrap_or(&model.text);
    let requirement_visible = model.requirement.visible();
    vec![
        label_node(
            model,
            state,
            LabelNodeDraft::new(LabelPart::Root, "label", "Label", root_detail).invalid(invalid),
        ),
        label_node(
            model,
            state,
            LabelNodeDraft::new(LabelPart::Text, "label-text", &model.text, "Label text")
                .invalid(invalid),
        ),
        label_node(
            model,
            state,
            LabelNodeDraft::new(
                LabelPart::Requirement,
                model.requirement.label(),
                model.requirement.marker(),
                "Requirement marker",
            )
            .visible(requirement_visible)
            .disabled(!requirement_visible || model.disabled || model.loading),
        ),
    ]
}

pub fn default_label_model() -> LabelModel {
    LabelModel::new("Email").with_for("email").required()
}

struct LabelNodeDraft<'a> {
    part: LabelPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    invalid: bool,
    disabled: bool,
}

impl<'a> LabelNodeDraft<'a> {
    const fn new(part: LabelPart, value: &'a str, label: &'a str, detail: &'a str) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            visible: true,
            invalid: false,
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

    const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

fn label_node(
    model: &LabelModel,
    state: &LabelState,
    draft: LabelNodeDraft<'_>,
) -> LabelRenderNode {
    LabelRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        requirement: model.requirement,
        active: state.is_active(draft.part),
        invalid: draft.invalid,
        visible: draft.visible,
        loading: model.loading,
        disabled: draft.disabled || model.disabled,
    }
}

fn validate_optional_label_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && (copy.is_empty() || copy.len() > 240)
    {
        return Err(garde::Error::new(
            "label optional copy must be 1..=240 characters",
        ));
    }
    Ok(())
}

fn validate_optional_label_control_id(control_id: &Option<String>, _context: &()) -> garde::Result {
    if let Some(control_id) = control_id {
        if control_id.is_empty() || control_id.len() > 128 {
            return Err(garde::Error::new(
                "label control id must be 1..=128 characters",
            ));
        }
        if !control_id
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
        {
            return Err(garde::Error::new(
                "label control id must use ascii letters, numbers, hyphen, or underscore",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_label_model(&default_label_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_text() {
        let model = LabelModel::new("");
        assert!(validate_label_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_control_id() {
        let model = default_label_model().with_for("");
        assert!(validate_label_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_control_id() {
        let model = default_label_model().with_for("email.field");
        assert!(validate_label_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_label_model().with_error("");
        assert!(validate_label_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_hover_leave_and_clear() {
        let mut state = LabelState::resting();
        assert_eq!(
            state.apply(LabelIntent::Focus(LabelPart::Text)),
            LabelChange::Focused(LabelPart::Text)
        );
        assert!(state.is_active(LabelPart::Text));
        assert_eq!(state.apply(LabelIntent::Blur), LabelChange::Blurred);
        assert_eq!(
            state.apply(LabelIntent::Hover(LabelPart::Requirement)),
            LabelChange::Hovered(LabelPart::Requirement)
        );
        assert!(state.is_active(LabelPart::Requirement));
        assert_eq!(state.apply(LabelIntent::Leave), LabelChange::Left);
        assert_eq!(
            state.apply(LabelIntent::Focus(LabelPart::Root)),
            LabelChange::Focused(LabelPart::Root)
        );
        assert_eq!(state.apply(LabelIntent::Clear), LabelChange::Cleared);
        assert!(!state.is_active(LabelPart::Root));
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_label_model();
        let nodes = label_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), LabelPart::ALL.len());
        assert_eq!(nodes.first().map(|node| node.part), Some(LabelPart::Root));
        for part in LabelPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn no_requirement_keeps_hidden_disabled_requirement_node() {
        let model = default_label_model().with_requirement(LabelRequirement::None);
        let nodes = label_render_nodes(&model, &model.state());
        let requirement = nodes
            .into_iter()
            .find(|node| node.part == LabelPart::Requirement)
            .expect("label render nodes include requirement");
        assert!(!requirement.visible);
        assert!(requirement.disabled);
    }

    #[test]
    fn error_marks_root_and_text_invalid() {
        let model = default_label_model().with_error("Email is required.");
        let nodes = label_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == LabelPart::Root && node.invalid)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == LabelPart::Text && node.invalid)
        );
    }
}
