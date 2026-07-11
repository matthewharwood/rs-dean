use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::input::InputDensity;
use crate::scale;

const MAX_INPUT_OTP_LENGTH: usize = 12;
const MAX_INPUT_OTP_GROUP_SIZE: usize = 6;
const DEFAULT_INPUT_OTP_LENGTH: usize = 6;
const DEFAULT_INPUT_OTP_GROUP_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputOtpPart {
    Root,
    Group,
    Slot,
    Separator,
}

impl InputOtpPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Group, Self::Slot, Self::Separator];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "InputOtp",
            Self::Group => "InputOtpGroup",
            Self::Slot => "InputOtpSlot",
            Self::Separator => "InputOtpSeparator",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct InputOtpModel {
    #[garde(skip)]
    pub density: InputDensity,
    #[garde(custom(validate_input_otp_length))]
    pub length: usize,
    #[garde(custom(validate_input_otp_group_size_for_length(&self.length)))]
    pub group_size: usize,
    #[garde(length(min = 1, max = 160))]
    pub label: String,
    #[garde(custom(validate_input_otp_value_for_contract(
        &self.length,
        &self.numeric_only
    )))]
    pub value: String,
    #[garde(length(min = 1, max = 8))]
    pub placeholder: String,
    #[garde(length(min = 1, max = 8))]
    pub separator: String,
    #[garde(skip)]
    pub numeric_only: bool,
    #[garde(custom(validate_optional_input_otp_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputOtpState {
    length: usize,
    numeric_only: bool,
    slots: Vec<Option<char>>,
    focused_slot: Option<usize>,
    active_part: Option<InputOtpPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputOtpIntent {
    FocusSlot(usize),
    Blur,
    InputSlot { index: usize, value: String },
    Paste(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputOtpChange {
    Focused(usize),
    Blurred,
    Input { index: usize, value: String },
    Pasted(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputOtpRenderNode {
    pub part: InputOtpPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: InputDensity,
    pub index: Option<usize>,
    pub focused: bool,
    pub active: bool,
    pub filled: bool,
    pub invalid: bool,
    pub visible: bool,
    pub actionable: bool,
    pub required: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InputOtpLayoutMetrics {
    pub group_gap: f32,
    pub slot_size: f32,
    pub slot_font_size: f32,
    pub slot_line_height: f32,
    pub separator_min_height: f32,
    pub separator_padding_inline: f32,
    pub separator_font_size: f32,
    pub separator_line_height: f32,
    pub shadow_offset: f32,
    pub shadow_blur: f32,
}

impl InputOtpModel {
    pub fn new(length: usize) -> Self {
        let group_size = length
            .min(DEFAULT_INPUT_OTP_GROUP_SIZE)
            .clamp(1, MAX_INPUT_OTP_GROUP_SIZE);
        Self {
            density: InputDensity::Standard,
            length,
            group_size,
            label: "One-time code".to_owned(),
            value: String::new(),
            placeholder: "•".to_owned(),
            separator: "-".to_owned(),
            numeric_only: true,
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

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        if self.group_size > length && length > 0 {
            self.group_size = length;
        }
        self
    }

    pub fn with_group_size(mut self, group_size: usize) -> Self {
        self.group_size = group_size;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
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

    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    pub const fn numeric_only(mut self) -> Self {
        self.numeric_only = true;
        self
    }

    pub const fn alphanumeric(mut self) -> Self {
        self.numeric_only = false;
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

    pub fn state(&self) -> InputOtpState {
        InputOtpState::new(&self.value, self.length, self.numeric_only)
    }
}

impl InputOtpState {
    pub fn new(value: impl AsRef<str>, length: usize, numeric_only: bool) -> Self {
        let length = length.clamp(1, MAX_INPUT_OTP_LENGTH);
        let mut slots = vec![None; length];
        for (index, character) in sanitize_input_otp_value(value.as_ref(), length, numeric_only)
            .chars()
            .enumerate()
        {
            slots[index] = Some(character);
        }
        Self {
            length,
            numeric_only,
            slots,
            focused_slot: None,
            active_part: None,
        }
    }

    pub const fn length(&self) -> usize {
        self.length
    }

    pub const fn focused_slot(&self) -> Option<usize> {
        self.focused_slot
    }

    pub const fn is_active(&self, part: InputOtpPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub const fn is_slot_focused(&self, index: usize) -> bool {
        matches!(self.focused_slot, Some(focused) if focused == index)
    }

    pub fn is_complete(&self) -> bool {
        self.slots.iter().all(Option::is_some)
    }

    pub fn slot_value(&self, index: usize) -> Option<char> {
        self.slots.get(index).copied().flatten()
    }

    pub fn value(&self) -> String {
        self.slots.iter().filter_map(|slot| *slot).collect()
    }

    pub fn apply(&mut self, intent: InputOtpIntent) -> InputOtpChange {
        match intent {
            InputOtpIntent::FocusSlot(index) => self.focus_slot(index),
            InputOtpIntent::Blur => self.blur(),
            InputOtpIntent::InputSlot { index, value } => self.input_slot(index, &value),
            InputOtpIntent::Paste(value) => self.paste(&value),
            InputOtpIntent::Clear => self.clear(),
        }
    }

    fn focus_slot(&mut self, index: usize) -> InputOtpChange {
        if index >= self.length {
            return InputOtpChange::Unchanged;
        }
        if self.focused_slot == Some(index) && self.active_part == Some(InputOtpPart::Slot) {
            InputOtpChange::Unchanged
        } else {
            self.focused_slot = Some(index);
            self.active_part = Some(InputOtpPart::Slot);
            InputOtpChange::Focused(index)
        }
    }

    fn blur(&mut self) -> InputOtpChange {
        if self.focused_slot.is_some() || self.active_part.is_some() {
            self.focused_slot = None;
            self.active_part = None;
            InputOtpChange::Blurred
        } else {
            InputOtpChange::Unchanged
        }
    }

    fn input_slot(&mut self, index: usize, value: &str) -> InputOtpChange {
        if index >= self.length {
            return InputOtpChange::Unchanged;
        }
        let next = value
            .chars()
            .find(|character| input_otp_character_allowed(*character, self.numeric_only));
        if self.slots[index] == next && self.focused_slot == Some(index) {
            return InputOtpChange::Unchanged;
        }
        self.slots[index] = next;
        self.active_part = Some(InputOtpPart::Slot);
        self.focused_slot = match next {
            Some(_) if index.saturating_add(1) < self.length => Some(index + 1),
            _ => Some(index),
        };
        InputOtpChange::Input {
            index,
            value: self.value(),
        }
    }

    fn paste(&mut self, value: &str) -> InputOtpChange {
        let start = self.focused_slot.unwrap_or(0).min(self.length - 1);
        let mut index = start;
        let before = self.value();
        for character in value
            .chars()
            .filter(|character| input_otp_character_allowed(*character, self.numeric_only))
        {
            if index >= self.length {
                break;
            }
            self.slots[index] = Some(character);
            index += 1;
        }
        let after = self.value();
        if before == after {
            return InputOtpChange::Unchanged;
        }
        self.active_part = Some(InputOtpPart::Slot);
        self.focused_slot = Some(index.min(self.length - 1));
        InputOtpChange::Pasted(after)
    }

    fn clear(&mut self) -> InputOtpChange {
        if self.slots.iter().all(Option::is_none) {
            InputOtpChange::Unchanged
        } else {
            self.slots.fill(None);
            self.focused_slot = None;
            self.active_part = None;
            InputOtpChange::Cleared
        }
    }
}

pub fn validate_input_otp_model(model: &InputOtpModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn input_otp_layout_metrics(
    density: InputDensity,
    loading: bool,
    disabled: bool,
    inline_size: f32,
) -> InputOtpLayoutMetrics {
    let dense_slot = density == InputDensity::Dense && !loading && !disabled;
    let dense_separator = density == InputDensity::Dense;
    InputOtpLayoutMetrics {
        group_gap: scale::space::xs2(inline_size),
        slot_size: if dense_slot {
            scale::space::s(inline_size)
        } else {
            scale::space::l(inline_size)
        },
        slot_font_size: if dense_slot {
            scale::font_size::f0(inline_size)
        } else {
            scale::font_size::f1(inline_size)
        },
        slot_line_height: if dense_slot {
            scale::line_height::LH0
        } else {
            scale::line_height::LH2
        },
        separator_min_height: if dense_separator {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        separator_padding_inline: if dense_separator {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        separator_font_size: if dense_separator {
            scale::font_size::f0(inline_size)
        } else {
            scale::font_size::f1(inline_size)
        },
        separator_line_height: if dense_separator {
            scale::line_height::LH0
        } else {
            scale::line_height::LH2
        },
        shadow_offset: scale::space::xs3(inline_size),
        shadow_blur: scale::space::xs2(inline_size),
    }
}

pub fn input_otp_render_nodes(
    model: &InputOtpModel,
    state: &InputOtpState,
) -> Vec<InputOtpRenderNode> {
    let blocked = model.loading || model.disabled;
    let control_detail = model
        .error
        .as_deref()
        .unwrap_or("Fixed-length one-time code");
    let root_value = state.value();
    let group_detail = format!("{} slots grouped by {}", model.length, model.group_size);
    let mut nodes = vec![
        input_otp_node(
            model,
            InputOtpNodeDraft {
                part: InputOtpPart::Root,
                index: None,
                value: &root_value,
                label: &model.label,
                detail: control_detail,
                visible: true,
                actionable: false,
                filled: false,
                disabled: model.disabled,
            },
            state,
        ),
        input_otp_node(
            model,
            InputOtpNodeDraft {
                part: InputOtpPart::Group,
                index: None,
                value: "input-otp-group",
                label: "InputOtpGroup",
                detail: &group_detail,
                visible: true,
                actionable: false,
                filled: false,
                disabled: model.disabled,
            },
            state,
        ),
    ];

    let mut separator_rendered = false;
    for index in 0..model.length {
        let slot_value = state
            .slot_value(index)
            .map(|character| character.to_string())
            .unwrap_or_default();
        let slot_label = if slot_value.is_empty() {
            model.placeholder.clone()
        } else {
            slot_value.clone()
        };
        let slot_detail = format!("One-time code slot {} of {}", index + 1, model.length);
        nodes.push(input_otp_node(
            model,
            InputOtpNodeDraft {
                part: InputOtpPart::Slot,
                index: Some(index),
                value: &slot_value,
                label: &slot_label,
                detail: &slot_detail,
                visible: true,
                actionable: true,
                filled: !slot_value.is_empty(),
                disabled: blocked,
            },
            state,
        ));

        if index + 1 < model.length && (index + 1) % model.group_size == 0 {
            separator_rendered = true;
            nodes.push(input_otp_node(
                model,
                InputOtpNodeDraft {
                    part: InputOtpPart::Separator,
                    index: Some(index),
                    value: &model.separator,
                    label: &model.separator,
                    detail: "Input OTP separator",
                    visible: true,
                    actionable: false,
                    filled: false,
                    disabled: blocked,
                },
                state,
            ));
        }
    }

    if !separator_rendered {
        nodes.push(input_otp_node(
            model,
            InputOtpNodeDraft {
                part: InputOtpPart::Separator,
                index: None,
                value: &model.separator,
                label: &model.separator,
                detail: "Input OTP separator",
                visible: false,
                actionable: false,
                filled: false,
                disabled: true,
            },
            state,
        ));
    }

    nodes
}

pub fn default_input_otp_model() -> InputOtpModel {
    InputOtpModel::new(DEFAULT_INPUT_OTP_LENGTH)
        .with_group_size(DEFAULT_INPUT_OTP_GROUP_SIZE)
        .with_value("123")
        .required()
}

struct InputOtpNodeDraft<'a> {
    part: InputOtpPart,
    index: Option<usize>,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    actionable: bool,
    filled: bool,
    disabled: bool,
}

fn input_otp_node(
    model: &InputOtpModel,
    draft: InputOtpNodeDraft<'_>,
    state: &InputOtpState,
) -> InputOtpRenderNode {
    let focused = draft
        .index
        .map(|index| state.is_slot_focused(index))
        .unwrap_or(false);
    let active = if draft.part == InputOtpPart::Slot {
        focused
    } else {
        state.is_active(draft.part)
    };
    InputOtpRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        index: draft.index,
        focused,
        active,
        filled: draft.filled,
        invalid: model.error.is_some(),
        visible: draft.visible,
        actionable: draft.actionable,
        required: model.required,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn sanitize_input_otp_value(value: &str, length: usize, numeric_only: bool) -> String {
    value
        .chars()
        .filter(|character| input_otp_character_allowed(*character, numeric_only))
        .take(length)
        .collect()
}

fn input_otp_character_allowed(character: char, numeric_only: bool) -> bool {
    if numeric_only {
        character.is_ascii_digit()
    } else {
        character.is_ascii_alphanumeric()
    }
}

fn validate_input_otp_length(length: &usize, _context: &()) -> garde::Result {
    if !(1..=MAX_INPUT_OTP_LENGTH).contains(length) {
        return Err(garde::Error::new(
            "input OTP length must be between 1 and 12 slots",
        ));
    }
    Ok(())
}

fn validate_input_otp_group_size_for_length(
    length: &usize,
) -> impl FnOnce(&usize, &()) -> garde::Result + '_ {
    move |group_size, _context| {
        if !(1..=MAX_INPUT_OTP_GROUP_SIZE).contains(group_size) {
            return Err(garde::Error::new(
                "input OTP group size must be between 1 and 6 slots",
            ));
        }
        if group_size > length {
            return Err(garde::Error::new(
                "input OTP group size cannot exceed length",
            ));
        }
        Ok(())
    }
}

fn validate_input_otp_value_for_contract<'a>(
    length: &'a usize,
    numeric_only: &'a bool,
) -> impl FnOnce(&String, &()) -> garde::Result + 'a {
    move |value, _context| {
        if value.chars().count() > *length {
            return Err(garde::Error::new(
                "input OTP value cannot exceed configured length",
            ));
        }
        if value
            .chars()
            .any(|character| !input_otp_character_allowed(character, *numeric_only))
        {
            return Err(garde::Error::new(
                "input OTP value contains characters outside the configured mode",
            ));
        }
        Ok(())
    }
}

fn validate_optional_input_otp_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && (copy.is_empty() || copy.len() > 320)
    {
        return Err(garde::Error::new(
            "input OTP optional copy must be 1..=320 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_input_otp_model(&default_input_otp_model()).is_ok());
    }

    #[test]
    fn layout_metrics_use_fluid_slot_and_separator_tokens() {
        let standard = input_otp_layout_metrics(InputDensity::Standard, false, false, 952.0);
        let dense = input_otp_layout_metrics(InputDensity::Dense, false, false, 952.0);
        let loading_dense = input_otp_layout_metrics(InputDensity::Dense, true, false, 952.0);

        assert!(standard.slot_size < scale::space::L);
        assert!(dense.slot_size < standard.slot_size);
        assert!(dense.slot_font_size < standard.slot_font_size);
        assert_eq!(loading_dense.slot_size, standard.slot_size);
        assert!(loading_dense.separator_min_height < standard.separator_min_height);
    }

    #[test]
    fn garde_rejects_zero_length() {
        let model = InputOtpModel::new(0);
        assert!(validate_input_otp_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_group_size_greater_than_length() {
        let model = InputOtpModel::new(4).with_group_size(5);
        assert!(validate_input_otp_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_value_longer_than_length() {
        let model = InputOtpModel::new(4).with_value("12345");
        assert!(validate_input_otp_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_non_numeric_default_value() {
        let model = InputOtpModel::new(6).with_value("12a");
        assert!(validate_input_otp_model(&model).is_err());
    }

    #[test]
    fn state_tracks_focus_input_paste_clear_and_blur() {
        let mut state = InputOtpState::new("", 6, true);
        assert_eq!(
            state.apply(InputOtpIntent::FocusSlot(0)),
            InputOtpChange::Focused(0)
        );
        assert_eq!(
            state.apply(InputOtpIntent::InputSlot {
                index: 0,
                value: "4".to_owned(),
            }),
            InputOtpChange::Input {
                index: 0,
                value: "4".to_owned(),
            }
        );
        assert_eq!(state.focused_slot(), Some(1));
        assert_eq!(
            state.apply(InputOtpIntent::Paste("56789".to_owned())),
            InputOtpChange::Pasted("456789".to_owned())
        );
        assert!(state.is_complete());
        assert_eq!(state.apply(InputOtpIntent::Clear), InputOtpChange::Cleared);
        assert_eq!(state.apply(InputOtpIntent::Blur), InputOtpChange::Unchanged);
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_input_otp_model();
        let nodes = input_otp_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(InputOtpPart::Root)
        );
        for part in InputOtpPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == InputOtpPart::Slot)
                .count(),
            model.length
        );
        assert!(nodes.iter().any(|node| {
            node.part == InputOtpPart::Separator && node.visible && node.label == "-"
        }));
    }

    #[test]
    fn ungrouped_model_keeps_disabled_separator_node() {
        let model = InputOtpModel::new(4).with_group_size(4);
        let nodes = input_otp_render_nodes(&model, &model.state());
        let separator = nodes
            .iter()
            .find(|node| node.part == InputOtpPart::Separator)
            .expect("input OTP render nodes preserve separator anatomy");
        assert!(!separator.visible);
        assert!(separator.disabled);
    }

    #[test]
    fn error_marks_nodes_invalid() {
        let model = default_input_otp_model().with_error("Code is required.");
        let nodes = input_otp_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(
            nodes
                .iter()
                .any(|node| node.part == InputOtpPart::Root && node.detail == "Code is required.")
        );
    }
}
