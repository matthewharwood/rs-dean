use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FieldDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FieldInputKind {
    Text,
    Email,
    Password,
    Search,
}

impl FieldDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl FieldInputKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Search => "search",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FieldPart {
    Root,
    Label,
    Control,
    Description,
    Error,
}

impl FieldPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Label,
        Self::Control,
        Self::Description,
        Self::Error,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Field",
            Self::Label => "FieldLabel",
            Self::Control => "FieldControl",
            Self::Description => "FieldDescription",
            Self::Error => "FieldError",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct FieldModel {
    #[garde(skip)]
    pub density: FieldDensity,
    #[garde(skip)]
    pub input_kind: FieldInputKind,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(max = 240))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub placeholder: String,
    #[garde(length(min = 1, max = 320))]
    pub description: String,
    #[garde(custom(validate_optional_field_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldState {
    focused: bool,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldIntent {
    Focus,
    Blur,
    Input(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldChange {
    Focused,
    Blurred,
    Input(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldRenderNode {
    pub part: FieldPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: FieldDensity,
    pub input_kind: FieldInputKind,
    pub focused: bool,
    pub invalid: bool,
    pub required: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl FieldModel {
    pub fn new(label: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            density: FieldDensity::Standard,
            input_kind: FieldInputKind::Text,
            label: label.into(),
            value: String::new(),
            placeholder: "Enter a value".to_owned(),
            description: description.into(),
            error: None,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: FieldDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_input_kind(mut self, input_kind: FieldInputKind) -> Self {
        self.input_kind = input_kind;
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
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

    pub fn state(&self) -> FieldState {
        FieldState::new(self.value.clone())
    }
}

impl FieldState {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            focused: false,
            value: value.into(),
        }
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn apply(&mut self, intent: FieldIntent) -> FieldChange {
        match intent {
            FieldIntent::Focus => self.focus(),
            FieldIntent::Blur => self.blur(),
            FieldIntent::Input(value) => self.input(value),
            FieldIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self) -> FieldChange {
        if self.focused {
            FieldChange::Unchanged
        } else {
            self.focused = true;
            FieldChange::Focused
        }
    }

    fn blur(&mut self) -> FieldChange {
        if self.focused {
            self.focused = false;
            FieldChange::Blurred
        } else {
            FieldChange::Unchanged
        }
    }

    fn input(&mut self, value: String) -> FieldChange {
        if self.value == value {
            FieldChange::Unchanged
        } else {
            self.value = value.clone();
            FieldChange::Input(value)
        }
    }

    fn clear(&mut self) -> FieldChange {
        if self.value.is_empty() {
            FieldChange::Unchanged
        } else {
            self.value.clear();
            FieldChange::Cleared
        }
    }
}

pub fn validate_field_model(model: &FieldModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn field_render_nodes(model: &FieldModel, state: &FieldState) -> Vec<FieldRenderNode> {
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let error_detail = model.error.as_deref().unwrap_or("No field error");
    vec![
        field_node(
            model,
            state,
            FieldNodeDraft::new(
                FieldPart::Root,
                "field",
                "Field",
                &model.label,
                true,
                model.disabled,
            ),
        ),
        field_node(
            model,
            state,
            FieldNodeDraft::new(
                FieldPart::Label,
                "field-label",
                &model.label,
                "Field label",
                true,
                model.disabled,
            ),
        ),
        field_node(
            model,
            state,
            FieldNodeDraft::new(
                FieldPart::Control,
                state.value(),
                &model.placeholder,
                "Field control",
                true,
                blocked,
            ),
        ),
        field_node(
            model,
            state,
            FieldNodeDraft::new(
                FieldPart::Description,
                "field-description",
                "Field description",
                &model.description,
                true,
                model.disabled,
            ),
        ),
        field_node(
            model,
            state,
            FieldNodeDraft::new(
                FieldPart::Error,
                "field-error",
                "Field error",
                error_detail,
                invalid,
                !invalid || model.disabled,
            ),
        ),
    ]
}

pub fn default_field_model() -> FieldModel {
    FieldModel::new(
        "Project name",
        "Use a short name that can be shared across Leptos and Bevy surfaces.",
    )
    .with_placeholder("rs-dean-ui")
}

struct FieldNodeDraft<'a> {
    part: FieldPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    disabled: bool,
}

impl<'a> FieldNodeDraft<'a> {
    const fn new(
        part: FieldPart,
        value: &'a str,
        label: &'a str,
        detail: &'a str,
        visible: bool,
        disabled: bool,
    ) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            visible,
            disabled,
        }
    }
}

fn field_node(
    model: &FieldModel,
    state: &FieldState,
    draft: FieldNodeDraft<'_>,
) -> FieldRenderNode {
    FieldRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        input_kind: model.input_kind,
        focused: state.is_focused(),
        invalid: model.error.is_some(),
        required: model.required,
        visible: draft.visible,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn validate_optional_field_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && (copy.is_empty() || copy.len() > 320)
    {
        return Err(garde::Error::new(
            "field optional copy must be 1..=320 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_field_model(&default_field_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = FieldModel::new("", "Description");
        assert!(validate_field_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_description() {
        let model = FieldModel::new("Project name", "");
        assert!(validate_field_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_placeholder() {
        let model = default_field_model().with_placeholder("");
        assert!(validate_field_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_field_model().with_error("");
        assert!(validate_field_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_input_blur_and_clear() {
        let mut state = FieldState::new("rs-dean");
        assert!(!state.is_focused());
        assert_eq!(state.apply(FieldIntent::Focus), FieldChange::Focused);
        assert!(state.is_focused());
        assert_eq!(
            state.apply(FieldIntent::Input("rs-dean-ui".to_owned())),
            FieldChange::Input("rs-dean-ui".to_owned())
        );
        assert_eq!(state.value(), "rs-dean-ui");
        assert_eq!(state.apply(FieldIntent::Blur), FieldChange::Blurred);
        assert_eq!(state.apply(FieldIntent::Clear), FieldChange::Cleared);
        assert_eq!(state.value(), "");
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_field_model();
        let nodes = field_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), FieldPart::ALL.len());
        assert_eq!(nodes.first().map(|node| node.part), Some(FieldPart::Root));
        for part in FieldPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn missing_error_keeps_disabled_error_node() {
        let model = default_field_model().without_error();
        let nodes = field_render_nodes(&model, &model.state());
        let error = nodes
            .into_iter()
            .find(|node| node.part == FieldPart::Error)
            .expect("field render nodes include error");
        assert!(!error.visible);
        assert!(error.disabled);
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_field_model().with_error("Required");
        let nodes = field_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(
            nodes
                .iter()
                .any(|node| node.part == FieldPart::Error && node.visible)
        );
    }
}
