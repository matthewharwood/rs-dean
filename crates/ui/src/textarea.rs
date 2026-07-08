use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::dom::ui_dom_id;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextareaDensity {
    Standard,
    Dense,
}

impl TextareaDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextareaPart {
    Root,
    Control,
    Counter,
    Hint,
}

impl TextareaPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Control, Self::Counter, Self::Hint];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Textarea",
            Self::Control => "TextareaControl",
            Self::Counter => "TextareaCounter",
            Self::Hint => "TextareaHint",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct TextareaModel {
    #[garde(skip)]
    pub density: TextareaDensity,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(max = 2_000), custom(textarea_value_within_max_length(&self.max_length)))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub placeholder: String,
    #[garde(length(min = 1, max = 320))]
    pub hint: String,
    #[garde(custom(validate_optional_textarea_copy))]
    pub error: Option<String>,
    #[garde(custom(validate_optional_textarea_max_length))]
    pub max_length: Option<u16>,
    #[garde(custom(validate_textarea_rows))]
    pub rows: u8,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextareaState {
    focused: bool,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextareaIntent {
    Focus,
    Blur,
    Input(String),
    Clear,
    Reset(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextareaChange {
    Focused,
    Blurred,
    Input(String),
    Cleared,
    Reset(String),
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextareaRenderNode {
    pub part: TextareaPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: TextareaDensity,
    pub rows: u8,
    pub current_length: usize,
    pub max_length: Option<u16>,
    pub focused: bool,
    pub invalid: bool,
    pub required: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
    pub actionable: bool,
}

impl TextareaModel {
    pub fn new(label: impl Into<String>, hint: impl Into<String>) -> Self {
        Self {
            density: TextareaDensity::Standard,
            label: label.into(),
            value: String::new(),
            placeholder: "Write a response".to_owned(),
            hint: hint.into(),
            error: None,
            max_length: Some(280),
            rows: 4,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: TextareaDensity) -> Self {
        self.density = density;
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

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = hint.into();
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub const fn with_max_length(mut self, max_length: u16) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub const fn without_max_length(mut self) -> Self {
        self.max_length = None;
        self
    }

    pub const fn with_rows(mut self, rows: u8) -> Self {
        self.rows = rows;
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

    pub fn state(&self) -> TextareaState {
        TextareaState::new(self.value.clone())
    }
}

impl TextareaState {
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

    pub fn current_length(&self) -> usize {
        self.value.chars().count()
    }

    pub fn apply(&mut self, intent: TextareaIntent) -> TextareaChange {
        match intent {
            TextareaIntent::Focus => self.focus(),
            TextareaIntent::Blur => self.blur(),
            TextareaIntent::Input(value) => self.input(value),
            TextareaIntent::Clear => self.clear(),
            TextareaIntent::Reset(value) => self.reset(value),
        }
    }

    fn focus(&mut self) -> TextareaChange {
        if self.focused {
            TextareaChange::Unchanged
        } else {
            self.focused = true;
            TextareaChange::Focused
        }
    }

    fn blur(&mut self) -> TextareaChange {
        if self.focused {
            self.focused = false;
            TextareaChange::Blurred
        } else {
            TextareaChange::Unchanged
        }
    }

    fn input(&mut self, value: String) -> TextareaChange {
        if self.value == value {
            TextareaChange::Unchanged
        } else {
            self.value = value.clone();
            TextareaChange::Input(value)
        }
    }

    fn clear(&mut self) -> TextareaChange {
        if self.value.is_empty() {
            TextareaChange::Unchanged
        } else {
            self.value.clear();
            TextareaChange::Cleared
        }
    }

    fn reset(&mut self, value: String) -> TextareaChange {
        if self.value == value && !self.focused {
            TextareaChange::Unchanged
        } else {
            self.value = value.clone();
            self.focused = false;
            TextareaChange::Reset(value)
        }
    }
}

pub fn validate_textarea_model(model: &TextareaModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn textarea_render_nodes(
    model: &TextareaModel,
    state: &TextareaState,
) -> Vec<TextareaRenderNode> {
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let current_length = state.current_length();
    let counter_label = textarea_counter_label(current_length, model.max_length);
    let hint_detail = model.error.clone().unwrap_or_else(|| model.hint.clone());
    vec![
        textarea_node(
            model,
            state,
            TextareaNodeDraft::new(
                TextareaPart::Root,
                "textarea",
                &model.label,
                &hint_detail,
                true,
                model.disabled,
                false,
            ),
        ),
        textarea_node(
            model,
            state,
            TextareaNodeDraft::new(
                TextareaPart::Control,
                state.value(),
                &model.placeholder,
                "Multi-line text entry",
                true,
                blocked,
                !blocked,
            ),
        ),
        textarea_node(
            model,
            state,
            TextareaNodeDraft::new(
                TextareaPart::Counter,
                "counter",
                &counter_label,
                textarea_counter_detail(model.max_length),
                true,
                false,
                false,
            ),
        ),
        textarea_node(
            model,
            state,
            TextareaNodeDraft::new(
                TextareaPart::Hint,
                "hint",
                if invalid { "Error" } else { "Hint" },
                &hint_detail,
                true,
                model.disabled,
                false,
            ),
        ),
    ]
}

pub fn default_textarea_model() -> TextareaModel {
    TextareaModel::new(
        "Implementation notes",
        "Draft locally; persist accepted notes through app state.",
    )
    .with_value("Shared models keep Leptos and Bevy aligned.")
    .with_placeholder("Add implementation notes")
}

pub fn textarea_dom_id(prefix: &str, value: &str) -> String {
    ui_dom_id(prefix, value)
}

fn textarea_node(
    model: &TextareaModel,
    state: &TextareaState,
    draft: TextareaNodeDraft<'_>,
) -> TextareaRenderNode {
    TextareaRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        rows: model.rows,
        current_length: state.current_length(),
        max_length: model.max_length,
        focused: state.is_focused() && draft.part == TextareaPart::Control,
        invalid: model.error.is_some(),
        required: model.required,
        visible: draft.visible,
        loading: model.loading,
        disabled: draft.disabled,
        actionable: draft.actionable,
    }
}

struct TextareaNodeDraft<'a> {
    part: TextareaPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    disabled: bool,
    actionable: bool,
}

impl<'a> TextareaNodeDraft<'a> {
    const fn new(
        part: TextareaPart,
        value: &'a str,
        label: &'a str,
        detail: &'a str,
        visible: bool,
        disabled: bool,
        actionable: bool,
    ) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            visible,
            disabled,
            actionable,
        }
    }
}

fn textarea_counter_label(current_length: usize, max_length: Option<u16>) -> String {
    match max_length {
        Some(max_length) => format!("{current_length} / {max_length}"),
        None => format!("{current_length} chars"),
    }
}

fn textarea_counter_detail(max_length: Option<u16>) -> &'static str {
    if max_length.is_some() {
        "Character limit"
    } else {
        "No character limit"
    }
}

fn textarea_value_within_max_length<'a>(
    max_length: &'a Option<u16>,
) -> impl FnOnce(&String, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(max_length) = max_length
            && value.chars().count() > usize::from(*max_length)
        {
            return Err(garde::Error::new(
                "textarea value must not exceed max length",
            ));
        }
        Ok(())
    }
}

fn validate_optional_textarea_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new(
            "textarea copy must be 1 to 240 characters",
        ));
    }
    Ok(())
}

fn validate_optional_textarea_max_length(max_length: &Option<u16>, _context: &()) -> garde::Result {
    if let Some(max_length) = max_length
        && !(16..=2_000).contains(max_length)
    {
        return Err(garde::Error::new(
            "textarea max length must be 16 to 2000 characters",
        ));
    }
    Ok(())
}

fn validate_textarea_rows(rows: &u8, _context: &()) -> garde::Result {
    if !(2..=12).contains(rows) {
        return Err(garde::Error::new("textarea rows must be 2 to 12"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_textarea_model(&default_textarea_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = default_textarea_model()
            .with_value("")
            .with_hint("Valid hint");
        let model = TextareaModel {
            label: String::new(),
            ..model
        };
        assert!(validate_textarea_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_placeholder() {
        let model = default_textarea_model().with_placeholder("");
        assert!(validate_textarea_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_hint() {
        let model = default_textarea_model().with_hint("");
        assert!(validate_textarea_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_textarea_model().with_error("");
        assert!(validate_textarea_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_rows() {
        let model = default_textarea_model().with_rows(1);
        assert!(validate_textarea_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_value_over_max_length() {
        let model = TextareaModel::new("Notes", "Keep it brief")
            .with_max_length(16)
            .with_value("This value is too long for the configured textarea.");
        assert!(validate_textarea_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_input_clear_blur_and_reset() {
        let mut state = TextareaState::new("Draft");
        assert_eq!(state.apply(TextareaIntent::Focus), TextareaChange::Focused);
        assert!(state.is_focused());
        assert_eq!(
            state.apply(TextareaIntent::Input("Updated draft".to_owned())),
            TextareaChange::Input("Updated draft".to_owned())
        );
        assert_eq!(state.value(), "Updated draft");
        assert_eq!(state.apply(TextareaIntent::Clear), TextareaChange::Cleared);
        assert_eq!(state.apply(TextareaIntent::Blur), TextareaChange::Blurred);
        assert_eq!(
            state.apply(TextareaIntent::Reset("Restored".to_owned())),
            TextareaChange::Reset("Restored".to_owned())
        );
        assert_eq!(state.value(), "Restored");
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_textarea_model();
        let nodes = textarea_render_nodes(&model, &model.state());
        for part in TextareaPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn counter_tracks_character_limit() {
        let model = TextareaModel::new("Comment", "Short comment")
            .with_value("abc")
            .with_max_length(16);
        let nodes = textarea_render_nodes(&model, &model.state());
        let counter = nodes
            .iter()
            .find(|node| node.part == TextareaPart::Counter)
            .expect("textarea has counter node");
        assert_eq!(counter.label, "3 / 16");
        assert_eq!(counter.max_length, Some(16));
    }

    #[test]
    fn loading_disables_control_node() {
        let model = default_textarea_model().loading();
        let nodes = textarea_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == TextareaPart::Control && node.disabled)
        );
    }

    #[test]
    fn no_max_length_keeps_counter_node() {
        let model = default_textarea_model().without_max_length();
        let nodes = textarea_render_nodes(&model, &model.state());
        let counter = nodes
            .iter()
            .find(|node| node.part == TextareaPart::Counter)
            .expect("textarea has counter node");
        assert_eq!(counter.detail, "No character limit");
    }

    #[test]
    fn dom_ids_are_stable_and_ascii() {
        assert_eq!(
            textarea_dom_id("textarea-hint", "Implementation Notes"),
            "textarea-hint-implementation-notes"
        );
    }
}
