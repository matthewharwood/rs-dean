use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum KbdDensity {
    Standard,
    Dense,
}

impl KbdDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum KbdPart {
    Root,
    Key,
    Chord,
}

impl KbdPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Key, Self::Chord];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Kbd",
            Self::Key => "KbdKey",
            Self::Chord => "KbdChord",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct KbdKey {
    #[garde(length(min = 1, max = 24))]
    pub label: String,
    #[garde(length(min = 1, max = 64))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct KbdModel {
    #[garde(skip)]
    pub density: KbdDensity,
    #[garde(custom(validate_kbd_keys))]
    pub keys: Vec<KbdKey>,
    #[garde(length(min = 1, max = 12))]
    pub separator: String,
    #[garde(custom(validate_optional_kbd_copy))]
    pub aria_label: Option<String>,
    #[garde(custom(validate_optional_kbd_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KbdState {
    key_count: usize,
    active_part: Option<KbdPart>,
    focused_key: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KbdIntent {
    FocusChord,
    FocusKey(usize),
    Blur,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KbdChange {
    ChordFocused,
    KeyFocused(usize),
    Blurred,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KbdRenderNode {
    pub part: KbdPart,
    pub index: Option<usize>,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: KbdDensity,
    pub focused: bool,
    pub invalid: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl KbdKey {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        let value = kbd_key_value(&label);
        Self {
            label,
            value,
            disabled: false,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl KbdModel {
    pub fn new(keys: Vec<KbdKey>) -> Self {
        Self {
            density: KbdDensity::Standard,
            keys,
            separator: "+".to_owned(),
            aria_label: Some("Keyboard shortcut".to_owned()),
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: KbdDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_keys(mut self, keys: Vec<KbdKey>) -> Self {
        self.keys = keys;
        self
    }

    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn without_aria_label(mut self) -> Self {
        self.aria_label = None;
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

    pub fn state(&self) -> KbdState {
        KbdState::new(self.keys.len())
    }
}

impl KbdState {
    pub const fn new(key_count: usize) -> Self {
        Self {
            key_count,
            active_part: None,
            focused_key: None,
        }
    }

    pub const fn is_focused(&self, part: KbdPart) -> bool {
        matches!(self.active_part, Some(active) if active as u8 == part as u8)
    }

    pub const fn is_key_focused(&self, index: usize) -> bool {
        matches!(self.focused_key, Some(focused) if focused == index)
    }

    pub fn apply(&mut self, intent: KbdIntent) -> KbdChange {
        match intent {
            KbdIntent::FocusChord => self.focus_chord(),
            KbdIntent::FocusKey(index) => self.focus_key(index),
            KbdIntent::Blur => self.blur(),
            KbdIntent::Clear => self.clear(),
        }
    }

    fn focus_chord(&mut self) -> KbdChange {
        if self.is_focused(KbdPart::Chord) && self.focused_key.is_none() {
            KbdChange::Unchanged
        } else {
            self.active_part = Some(KbdPart::Chord);
            self.focused_key = None;
            KbdChange::ChordFocused
        }
    }

    fn focus_key(&mut self, index: usize) -> KbdChange {
        if index >= self.key_count {
            return KbdChange::Unchanged;
        }
        if self.is_key_focused(index) {
            KbdChange::Unchanged
        } else {
            self.active_part = Some(KbdPart::Key);
            self.focused_key = Some(index);
            KbdChange::KeyFocused(index)
        }
    }

    fn blur(&mut self) -> KbdChange {
        if self.active_part.is_none() && self.focused_key.is_none() {
            KbdChange::Unchanged
        } else {
            self.active_part = None;
            self.focused_key = None;
            KbdChange::Blurred
        }
    }

    fn clear(&mut self) -> KbdChange {
        if self.active_part.is_none() && self.focused_key.is_none() {
            KbdChange::Unchanged
        } else {
            self.active_part = None;
            self.focused_key = None;
            KbdChange::Cleared
        }
    }
}

pub fn validate_kbd_model(model: &KbdModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn kbd_render_nodes(model: &KbdModel, state: &KbdState) -> Vec<KbdRenderNode> {
    let invalid = model.error.is_some();
    let chord = kbd_chord_label(model);
    let aria_label = model.aria_label.as_deref().unwrap_or("Keyboard shortcut");
    let chord_detail = model.error.as_deref().unwrap_or(&chord);
    let mut nodes = Vec::with_capacity(model.keys.len() + 2);

    nodes.push(kbd_node(
        model,
        state,
        KbdNodeDraft::new(KbdPart::Root, "kbd", "Kbd", &chord).invalid(invalid),
    ));
    nodes.push(kbd_node(
        model,
        state,
        KbdNodeDraft::new(KbdPart::Chord, "kbd-chord", aria_label, chord_detail).invalid(invalid),
    ));

    for (index, key) in model.keys.iter().enumerate() {
        nodes.push(kbd_node(
            model,
            state,
            KbdNodeDraft::new(KbdPart::Key, &key.value, &key.label, "Keyboard key")
                .index(index)
                .invalid(invalid)
                .disabled(model.disabled || model.loading || key.disabled),
        ));
    }

    nodes
}

pub fn kbd_chord_label(model: &KbdModel) -> String {
    model
        .keys
        .iter()
        .map(|key| key.label.as_str())
        .collect::<Vec<_>>()
        .join(&model.separator)
}

pub fn default_kbd_model() -> KbdModel {
    KbdModel::new(vec![KbdKey::new("Cmd"), KbdKey::new("K")])
        .with_separator(" + ")
        .with_aria_label("Open command menu")
}

struct KbdNodeDraft<'a> {
    part: KbdPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    index: Option<usize>,
    visible: bool,
    invalid: bool,
    disabled: bool,
}

impl<'a> KbdNodeDraft<'a> {
    const fn new(part: KbdPart, value: &'a str, label: &'a str, detail: &'a str) -> Self {
        Self {
            part,
            value,
            label,
            detail,
            index: None,
            visible: true,
            invalid: false,
            disabled: false,
        }
    }

    const fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
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

fn kbd_node(model: &KbdModel, state: &KbdState, draft: KbdNodeDraft<'_>) -> KbdRenderNode {
    let focused = if draft.part == KbdPart::Key {
        draft.index.is_some_and(|index| state.is_key_focused(index))
    } else {
        state.is_focused(draft.part)
    };
    KbdRenderNode {
        part: draft.part,
        index: draft.index,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        focused,
        invalid: draft.invalid,
        visible: draft.visible,
        loading: model.loading,
        disabled: draft.disabled || model.disabled,
    }
}

fn kbd_key_value(label: &str) -> String {
    let value = label
        .trim()
        .to_ascii_lowercase()
        .replace(|character: char| !character.is_ascii_alphanumeric(), "-");
    value.trim_matches('-').to_owned()
}

fn validate_kbd_keys(keys: &Vec<KbdKey>, _context: &()) -> garde::Result {
    if keys.is_empty() || keys.len() > 6 {
        return Err(garde::Error::new("kbd keys must contain 1..=6 keys"));
    }

    let mut values = HashSet::with_capacity(keys.len());
    for key in keys {
        key.validate()
            .map_err(|report| garde::Error::new(format!("invalid kbd key: {report}")))?;
        if !values.insert(key.value.as_str()) {
            return Err(garde::Error::new(
                "kbd key values must be unique within the chord",
            ));
        }
    }

    Ok(())
}

fn validate_optional_kbd_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && (copy.is_empty() || copy.len() > 240)
    {
        return Err(garde::Error::new(
            "kbd optional copy must be 1..=240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_kbd_model(&default_kbd_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_keys() {
        let model = KbdModel::new(Vec::new());
        assert!(validate_kbd_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_key_label() {
        let model = KbdModel::new(vec![KbdKey::new("")]);
        assert!(validate_kbd_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_key_values() {
        let model = KbdModel::new(vec![
            KbdKey::new("Command").with_value("mod"),
            KbdKey::new("Control").with_value("mod"),
        ]);
        assert!(validate_kbd_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_separator() {
        let model = default_kbd_model().with_separator("");
        assert!(validate_kbd_model(&model).is_err());
    }

    #[test]
    fn state_tracks_key_focus_blur_and_clear() {
        let mut state = KbdState::new(2);
        assert_eq!(state.apply(KbdIntent::FocusKey(9)), KbdChange::Unchanged);
        assert_eq!(
            state.apply(KbdIntent::FocusKey(1)),
            KbdChange::KeyFocused(1)
        );
        assert!(state.is_key_focused(1));
        assert_eq!(state.apply(KbdIntent::Blur), KbdChange::Blurred);
        assert!(!state.is_key_focused(1));
        assert_eq!(state.apply(KbdIntent::FocusChord), KbdChange::ChordFocused);
        assert!(state.is_focused(KbdPart::Chord));
        assert_eq!(state.apply(KbdIntent::Clear), KbdChange::Cleared);
        assert!(!state.is_focused(KbdPart::Chord));
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_kbd_model();
        let nodes = kbd_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), model.keys.len() + 2);
        assert_eq!(nodes.first().map(|node| node.part), Some(KbdPart::Root));
        for part in KbdPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        let key_nodes = nodes
            .iter()
            .filter(|node| node.part == KbdPart::Key)
            .collect::<Vec<_>>();
        assert_eq!(key_nodes[0].index, Some(0));
        assert_eq!(key_nodes[1].index, Some(1));
    }

    #[test]
    fn error_marks_chord_and_keys_invalid() {
        let model = default_kbd_model().with_error("Shortcut unavailable.");
        let nodes = kbd_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == KbdPart::Chord && node.invalid)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == KbdPart::Key && node.invalid)
        );
    }

    #[test]
    fn loading_disables_key_nodes() {
        let model = default_kbd_model().loading();
        let nodes = kbd_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == KbdPart::Key)
                .all(|node| node.disabled)
        );
    }
}
