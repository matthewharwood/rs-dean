use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProgressDensity {
    Standard,
    Dense,
}

impl ProgressDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProgressPart {
    Root,
    Track,
    Indicator,
    Label,
}

impl ProgressPart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Track, Self::Indicator, Self::Label];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Progress",
            Self::Track => "ProgressTrack",
            Self::Indicator => "ProgressIndicator",
            Self::Label => "ProgressLabel",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ProgressModel {
    #[garde(skip)]
    pub density: ProgressDensity,
    #[garde(custom(validate_optional_progress_value))]
    pub value: Option<u8>,
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(custom(validate_optional_progress_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressState {
    highlighted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgressIntent {
    Focus,
    Hover,
    Blur,
    Leave,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgressChange {
    Focused,
    Hovered,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressRenderNode {
    pub part: ProgressPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: ProgressDensity,
    pub percent: u8,
    pub determinate: bool,
    pub highlighted: bool,
    pub visible: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl ProgressModel {
    pub fn new(value: u8) -> Self {
        Self {
            density: ProgressDensity::Standard,
            value: Some(value),
            label: "Progress".to_owned(),
            detail: "Task progress is controlled by the consuming app.".to_owned(),
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub fn indeterminate() -> Self {
        Self {
            value: None,
            ..Self::new(0)
        }
    }

    pub const fn with_density(mut self, density: ProgressDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_value(mut self, value: u8) -> Self {
        self.value = Some(value);
        self
    }

    pub const fn without_value(mut self) -> Self {
        self.value = None;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
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

    pub const fn loading(mut self) -> Self {
        self.loading = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn state(&self) -> ProgressState {
        ProgressState::new()
    }
}

impl ProgressState {
    pub const fn new() -> Self {
        Self { highlighted: false }
    }

    pub const fn is_highlighted(&self) -> bool {
        self.highlighted
    }

    pub fn apply(&mut self, intent: ProgressIntent) -> ProgressChange {
        match intent {
            ProgressIntent::Focus => self.highlight(ProgressChange::Focused),
            ProgressIntent::Hover => self.highlight(ProgressChange::Hovered),
            ProgressIntent::Blur | ProgressIntent::Leave | ProgressIntent::Clear => self.clear(),
        }
    }

    fn highlight(&mut self, change: ProgressChange) -> ProgressChange {
        if self.highlighted {
            ProgressChange::Unchanged
        } else {
            self.highlighted = true;
            change
        }
    }

    fn clear(&mut self) -> ProgressChange {
        if self.highlighted {
            self.highlighted = false;
            ProgressChange::Cleared
        } else {
            ProgressChange::Unchanged
        }
    }
}

impl Default for ProgressState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn validate_progress_model(model: &ProgressModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn progress_render_nodes(
    model: &ProgressModel,
    state: &ProgressState,
) -> Vec<ProgressRenderNode> {
    let invalid = model.error.is_some();
    let percent = progress_percent(model);
    let determinate = progress_is_determinate(model);
    let value_label = progress_value_label(model);
    vec![
        progress_node(
            model,
            state,
            ProgressNodeDraft {
                part: ProgressPart::Root,
                value: &value_label,
                label: &model.label,
                detail: if determinate {
                    "Determinate progress"
                } else {
                    "Indeterminate progress"
                },
                visible: true,
                invalid,
                percent,
                determinate,
            },
        ),
        progress_node(
            model,
            state,
            ProgressNodeDraft {
                part: ProgressPart::Track,
                value: "progress-track",
                label: "Track",
                detail: "Progress track",
                visible: true,
                invalid,
                percent,
                determinate,
            },
        ),
        progress_node(
            model,
            state,
            ProgressNodeDraft {
                part: ProgressPart::Indicator,
                value: &value_label,
                label: if determinate {
                    &value_label
                } else {
                    "In progress"
                },
                detail: "Progress indicator",
                visible: true,
                invalid,
                percent,
                determinate,
            },
        ),
        progress_node(
            model,
            state,
            ProgressNodeDraft {
                part: ProgressPart::Label,
                value: &value_label,
                label: &model.label,
                detail: model.error.as_deref().unwrap_or(&model.detail),
                visible: true,
                invalid,
                percent,
                determinate,
            },
        ),
    ]
}

pub fn default_progress_model() -> ProgressModel {
    ProgressModel::new(64)
        .with_label("Upload")
        .with_detail("64 percent complete")
}

pub fn progress_value_label(model: &ProgressModel) -> String {
    if model.loading {
        "Loading".to_owned()
    } else if let Some(value) = model.value {
        format!("{value}% complete")
    } else {
        "In progress".to_owned()
    }
}

pub const fn progress_percent(model: &ProgressModel) -> u8 {
    if model.loading || model.value.is_none() {
        100
    } else {
        match model.value {
            Some(value) => value,
            None => 100,
        }
    }
}

pub const fn progress_is_determinate(model: &ProgressModel) -> bool {
    !model.loading && model.value.is_some()
}

struct ProgressNodeDraft<'a> {
    part: ProgressPart,
    value: &'a str,
    label: &'a str,
    detail: &'a str,
    visible: bool,
    invalid: bool,
    percent: u8,
    determinate: bool,
}

fn progress_node(
    model: &ProgressModel,
    state: &ProgressState,
    draft: ProgressNodeDraft<'_>,
) -> ProgressRenderNode {
    ProgressRenderNode {
        part: draft.part,
        value: draft.value.to_owned(),
        label: draft.label.to_owned(),
        detail: draft.detail.to_owned(),
        density: model.density,
        percent: draft.percent,
        determinate: draft.determinate,
        highlighted: state.is_highlighted(),
        visible: draft.visible,
        invalid: draft.invalid,
        loading: model.loading,
        disabled: model.disabled,
    }
}

fn validate_optional_progress_value(value: &Option<u8>, _context: &()) -> garde::Result {
    if let Some(value) = value
        && *value > 100
    {
        return Err(garde::Error::new(
            "progress value must be between 0 and 100",
        ));
    }
    Ok(())
}

fn validate_optional_progress_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "progress error must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_progress_model(&default_progress_model()).is_ok());
    }

    #[test]
    fn garde_rejects_value_over_one_hundred() {
        let model = ProgressModel::new(101);
        assert!(validate_progress_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = default_progress_model().with_label("");
        assert!(validate_progress_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_detail() {
        let model = default_progress_model().with_detail("");
        assert!(validate_progress_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_progress_model().with_error("");
        assert!(validate_progress_model(&model).is_err());
    }

    #[test]
    fn state_highlights_and_clears() {
        let mut state = ProgressState::new();
        assert!(!state.is_highlighted());
        assert_eq!(state.apply(ProgressIntent::Focus), ProgressChange::Focused);
        assert!(state.is_highlighted());
        assert_eq!(
            state.apply(ProgressIntent::Hover),
            ProgressChange::Unchanged
        );
        assert_eq!(state.apply(ProgressIntent::Leave), ProgressChange::Cleared);
        assert!(!state.is_highlighted());
        assert_eq!(
            state.apply(ProgressIntent::Clear),
            ProgressChange::Unchanged
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_progress_model();
        let nodes = progress_render_nodes(&model, &model.state());
        assert_eq!(nodes.len(), ProgressPart::ALL.len());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(ProgressPart::Root)
        );
        for part in ProgressPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn indeterminate_model_uses_full_indicator_without_value() {
        let model = ProgressModel::indeterminate();
        let nodes = progress_render_nodes(&model, &model.state());
        let indicator = nodes
            .iter()
            .find(|node| node.part == ProgressPart::Indicator)
            .expect("progress includes indicator");
        assert_eq!(indicator.percent, 100);
        assert!(!indicator.determinate);
        assert_eq!(indicator.value, "In progress");
    }

    #[test]
    fn loading_model_is_indeterminate() {
        let model = default_progress_model().loading();
        let nodes = progress_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.loading));
        assert!(nodes.iter().all(|node| !node.determinate));
    }

    #[test]
    fn error_marks_render_nodes_invalid() {
        let model = default_progress_model().with_error("Upload failed");
        let nodes = progress_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
    }
}
