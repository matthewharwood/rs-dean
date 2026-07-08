use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SeparatorDensity {
    Standard,
    Dense,
}

impl SeparatorDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

impl SeparatorOrientation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SeparatorPart {
    Root,
    Line,
    Label,
}

impl SeparatorPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Line, Self::Label];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Separator",
            Self::Line => "SeparatorLine",
            Self::Label => "SeparatorLabel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SeparatorModel {
    #[garde(skip)]
    pub density: SeparatorDensity,
    #[garde(skip)]
    pub orientation: SeparatorOrientation,
    #[garde(custom(validate_optional_separator_label))]
    pub label: Option<String>,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(custom(validate_optional_separator_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub decorative: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparatorState {
    focused: bool,
    hovered: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeparatorIntent {
    Focus,
    Blur,
    Hover,
    Leave,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeparatorChange {
    Focused,
    Hovered,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparatorRenderNode {
    pub part: SeparatorPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: SeparatorDensity,
    pub orientation: SeparatorOrientation,
    pub decorative: bool,
    pub active: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl SeparatorModel {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            density: SeparatorDensity::Standard,
            orientation: SeparatorOrientation::Horizontal,
            label: Some(label.into()),
            detail: "Separates related content without owning durable state.".to_owned(),
            error: None,
            decorative: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SeparatorDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self.decorative = false;
        self
    }

    pub fn without_label(mut self) -> Self {
        self.label = None;
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
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

    pub fn decorative(mut self) -> Self {
        self.decorative = true;
        self.label = None;
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

    pub const fn state(&self) -> SeparatorState {
        SeparatorState::new()
    }
}

impl SeparatorState {
    pub const fn new() -> Self {
        Self {
            focused: false,
            hovered: false,
        }
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub const fn is_hovered(&self) -> bool {
        self.hovered
    }

    pub const fn is_active(&self) -> bool {
        self.focused || self.hovered
    }

    pub fn apply(&mut self, intent: SeparatorIntent) -> SeparatorChange {
        match intent {
            SeparatorIntent::Focus => self.focus(),
            SeparatorIntent::Hover => self.hover(),
            SeparatorIntent::Blur | SeparatorIntent::Leave | SeparatorIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self) -> SeparatorChange {
        if self.focused {
            SeparatorChange::Unchanged
        } else {
            self.focused = true;
            SeparatorChange::Focused
        }
    }

    fn hover(&mut self) -> SeparatorChange {
        if self.hovered {
            SeparatorChange::Unchanged
        } else {
            self.hovered = true;
            SeparatorChange::Hovered
        }
    }

    fn clear(&mut self) -> SeparatorChange {
        if self.focused || self.hovered {
            self.focused = false;
            self.hovered = false;
            SeparatorChange::Cleared
        } else {
            SeparatorChange::Unchanged
        }
    }
}

impl Default for SeparatorState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn validate_separator_model(model: &SeparatorModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn separator_render_nodes(
    model: &SeparatorModel,
    state: &SeparatorState,
) -> Vec<SeparatorRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let active = state.is_active();
    let root_label = model
        .label
        .clone()
        .unwrap_or_else(|| "Decorative separator".to_owned());
    let label_visible = model.label.is_some();

    vec![
        SeparatorRenderNode {
            part: SeparatorPart::Root,
            value: model.orientation.label().to_owned(),
            label: root_label.clone(),
            detail: model.error.clone().unwrap_or_else(|| model.detail.clone()),
            density: model.density,
            orientation: model.orientation,
            decorative: model.decorative,
            active,
            visible: true,
            actionable: !blocked && !model.decorative,
            invalid,
            loading: model.loading,
            disabled: model.disabled,
        },
        SeparatorRenderNode {
            part: SeparatorPart::Line,
            value: format!("line-{}", model.orientation.label()),
            label: format!("{} line", root_label),
            detail: "Theme-colored separator line derived from shared tokens.".to_owned(),
            density: model.density,
            orientation: model.orientation,
            decorative: model.decorative,
            active,
            visible: true,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: blocked,
        },
        SeparatorRenderNode {
            part: SeparatorPart::Label,
            value: model.label.clone().unwrap_or_else(|| "none".to_owned()),
            label: model
                .label
                .clone()
                .unwrap_or_else(|| "Unlabeled separator".to_owned()),
            detail: if label_visible {
                "Optional separator label rendered from the shared model.".to_owned()
            } else {
                "Label node is present for anatomy parity but hidden.".to_owned()
            },
            density: model.density,
            orientation: model.orientation,
            decorative: model.decorative,
            active,
            visible: label_visible,
            actionable: false,
            invalid,
            loading: model.loading,
            disabled: blocked || !label_visible,
        },
    ]
}

pub fn default_separator_model() -> SeparatorModel {
    SeparatorModel::new("Component boundary")
        .with_detail("Token-backed divider shared by Leptos DOM and Bevy primitives.")
}

fn validate_optional_separator_label(value: &Option<String>, _: &()) -> garde::Result {
    validate_optional_text(value, 96, "separator label")
}

fn validate_optional_separator_error(value: &Option<String>, _: &()) -> garde::Result {
    validate_optional_text(value, 240, "separator error")
}

fn validate_optional_text(value: &Option<String>, max: usize, field: &str) -> garde::Result {
    if let Some(value) = value
        && (value.is_empty() || value.len() > max)
    {
        return Err(garde::Error::new(format!(
            "{field} must be between 1 and {max} characters when present"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_separator_model(&default_separator_model()).is_ok());
    }

    #[test]
    fn decorative_model_hides_label_and_validates() {
        let model = default_separator_model().decorative();
        assert!(validate_separator_model(&model).is_ok());

        let state = model.state();
        let nodes = separator_render_nodes(&model, &state);
        let label = nodes
            .iter()
            .find(|node| node.part == SeparatorPart::Label)
            .expect("separator label node should exist");

        assert!(!label.visible);
        assert!(label.disabled);
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = default_separator_model().with_label("");
        assert!(validate_separator_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_detail() {
        let model = default_separator_model().with_detail("");
        assert!(validate_separator_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_separator_model().with_error("");
        assert!(validate_separator_model(&model).is_err());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_separator_model();
        let state = model.state();
        let nodes = separator_render_nodes(&model, &state);

        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(SeparatorPart::Root)
        );
        for part in SeparatorPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn state_tracks_focus_hover_and_clear() {
        let model = default_separator_model();
        let mut state = model.state();

        assert!(!state.is_active());
        assert_eq!(
            state.apply(SeparatorIntent::Focus),
            SeparatorChange::Focused
        );
        assert!(state.is_focused());
        assert_eq!(
            state.apply(SeparatorIntent::Hover),
            SeparatorChange::Hovered
        );
        assert!(state.is_hovered());
        assert_eq!(
            state.apply(SeparatorIntent::Clear),
            SeparatorChange::Cleared
        );
        assert!(!state.is_active());
    }

    #[test]
    fn vertical_orientation_reaches_every_node() {
        let model = default_separator_model().with_orientation(SeparatorOrientation::Vertical);
        let state = model.state();

        for node in separator_render_nodes(&model, &state) {
            assert_eq!(node.orientation, SeparatorOrientation::Vertical);
        }
    }
}
