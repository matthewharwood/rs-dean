use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChartDensity {
    Standard,
    Dense,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChartTone {
    Brand,
    Info,
    Success,
    Warning,
    Danger,
    Muted,
}

impl ChartDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl ChartTone {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Brand => "brand",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Muted => "muted",
        }
    }
}

impl ChartPart {
    pub const ALL: &'static [Self] = &[
        Self::Container,
        Self::Series,
        Self::Legend,
        Self::Tooltip,
        Self::Axis,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Container => "ChartContainer",
            Self::Series => "ChartSeries",
            Self::Legend => "ChartLegend",
            Self::Tooltip => "ChartTooltip",
            Self::Axis => "ChartAxis",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ChartSeries {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(range(min = 0, max = 100))]
    pub amount: u8,
    #[garde(skip)]
    pub tone: ChartTone,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ChartModel {
    #[garde(skip)]
    pub density: ChartDensity,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 96))]
    pub axis_label: String,
    #[garde(length(min = 1, max = 16))]
    pub unit: String,
    #[garde(length(min = 1, max = 8), dive, custom(chart_series_values_are_unique))]
    pub series: Vec<ChartSeries>,
    #[garde(custom(chart_selected_value_exists(&self.series)))]
    pub selected_value: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChartState {
    selected_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChartIntent {
    Select(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChartChange {
    Selected(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChartPart {
    Container,
    Series,
    Legend,
    Tooltip,
    Axis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChartRenderNode {
    pub part: ChartPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub amount: u8,
    pub density: ChartDensity,
    pub tone: ChartTone,
    pub selected: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl ChartSeries {
    pub fn new(label: impl Into<String>, value: impl Into<String>, amount: u8) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            amount,
            tone: ChartTone::Brand,
            disabled: false,
        }
    }

    pub const fn with_tone(mut self, tone: ChartTone) -> Self {
        self.tone = tone;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl ChartModel {
    pub fn new(
        title: impl Into<String>,
        axis_label: impl Into<String>,
        unit: impl Into<String>,
        series: Vec<ChartSeries>,
    ) -> Self {
        Self {
            density: ChartDensity::Standard,
            title: title.into(),
            axis_label: axis_label.into(),
            unit: unit.into(),
            series,
            selected_value: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: ChartDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_selected_value(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
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

    pub fn state(&self) -> ChartState {
        ChartState::new(
            self.selected_value
                .clone()
                .or_else(|| self.series.first().map(|series| series.value.clone())),
        )
    }
}

impl ChartState {
    pub fn new(selected_value: Option<String>) -> Self {
        Self { selected_value }
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: ChartIntent) -> ChartChange {
        match intent {
            ChartIntent::Select(value) => {
                if value.is_empty() || self.selected_value.as_deref() == Some(value.as_str()) {
                    ChartChange::Unchanged
                } else {
                    self.selected_value = Some(value.clone());
                    ChartChange::Selected(value)
                }
            }
            ChartIntent::Clear => {
                if self.selected_value.is_none() {
                    ChartChange::Unchanged
                } else {
                    self.selected_value = None;
                    ChartChange::Cleared
                }
            }
        }
    }
}

pub fn validate_chart_model(model: &ChartModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn chart_render_nodes(model: &ChartModel, state: &ChartState) -> Vec<ChartRenderNode> {
    let blocked = model.loading || model.disabled;
    let selected = selected_series(model, state);
    let mut nodes = Vec::with_capacity(model.series.len().saturating_mul(2) + 3);
    nodes.push(chart_node(
        model,
        ChartNodeDraft {
            part: ChartPart::Container,
            index: 0,
            value: "chart-container".to_owned(),
            label: model.title.clone(),
            detail: "Themed data visualization frame".to_owned(),
            amount: 0,
            tone: ChartTone::Muted,
            selected: false,
            disabled: model.disabled,
        },
    ));

    for (index, series) in model.series.iter().enumerate() {
        nodes.push(chart_node(
            model,
            ChartNodeDraft {
                part: ChartPart::Series,
                index,
                value: series.value.clone(),
                label: series.label.clone(),
                detail: format!("{}{} {}", series.amount, model.unit, series.label),
                amount: series.amount,
                tone: series.tone,
                selected: state.is_selected(&series.value),
                disabled: blocked || series.disabled,
            },
        ));
    }

    for (index, series) in model.series.iter().enumerate() {
        nodes.push(chart_node(
            model,
            ChartNodeDraft {
                part: ChartPart::Legend,
                index,
                value: series.value.clone(),
                label: series.label.clone(),
                detail: series.tone.label().to_owned(),
                amount: series.amount,
                tone: series.tone,
                selected: state.is_selected(&series.value),
                disabled: model.disabled || series.disabled,
            },
        ));
    }

    nodes.push(chart_node(
        model,
        ChartNodeDraft {
            part: ChartPart::Tooltip,
            index: selected.map(|(_, index)| index).unwrap_or(0),
            value: selected
                .map(|(series, _)| series.value.clone())
                .unwrap_or_else(|| "chart-tooltip".to_owned()),
            label: selected
                .map(|(series, _)| series.label.clone())
                .unwrap_or_else(|| "No series".to_owned()),
            detail: selected
                .map(|(series, _)| format!("{}{}", series.amount, model.unit))
                .unwrap_or_else(|| "0".to_owned()),
            amount: selected.map(|(series, _)| series.amount).unwrap_or(0),
            tone: selected
                .map(|(series, _)| series.tone)
                .unwrap_or(ChartTone::Muted),
            selected: selected.is_some(),
            disabled: model.disabled,
        },
    ));
    nodes.push(chart_node(
        model,
        ChartNodeDraft {
            part: ChartPart::Axis,
            index: 0,
            value: "chart-axis".to_owned(),
            label: model.axis_label.clone(),
            detail: model.unit.clone(),
            amount: 0,
            tone: ChartTone::Muted,
            selected: false,
            disabled: model.disabled,
        },
    ));
    nodes
}

pub fn default_chart_model() -> ChartModel {
    ChartModel::new(
        "Component progress",
        "Implementation status",
        "%",
        default_chart_series(),
    )
    .with_selected_value("implemented")
}

pub fn default_chart_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("Implemented", "implemented", 23).with_tone(ChartTone::Success),
        ChartSeries::new("In sweep", "in-sweep", 8).with_tone(ChartTone::Info),
        ChartSeries::new("Remaining", "remaining", 69).with_tone(ChartTone::Muted),
    ]
}

#[derive(Debug, Clone)]
struct ChartNodeDraft {
    part: ChartPart,
    index: usize,
    value: String,
    label: String,
    detail: String,
    amount: u8,
    tone: ChartTone,
    selected: bool,
    disabled: bool,
}

fn chart_node(model: &ChartModel, draft: ChartNodeDraft) -> ChartRenderNode {
    ChartRenderNode {
        part: draft.part,
        index: draft.index,
        value: draft.value,
        label: draft.label,
        detail: draft.detail,
        amount: draft.amount,
        density: model.density,
        tone: draft.tone,
        selected: draft.selected,
        loading: model.loading,
        disabled: draft.disabled,
    }
}

fn selected_series<'a>(
    model: &'a ChartModel,
    state: &ChartState,
) -> Option<(&'a ChartSeries, usize)> {
    state
        .selected_value()
        .and_then(|selected| {
            model
                .series
                .iter()
                .enumerate()
                .find(|(_, series)| series.value == selected)
        })
        .map(|(index, series)| (series, index))
        .or_else(|| model.series.first().map(|series| (series, 0)))
}

fn chart_selected_value_exists<'a>(
    series: &'a [ChartSeries],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |selected_value, _context| {
        let Some(selected_value) = selected_value else {
            return Ok(());
        };
        if selected_value.is_empty() {
            return Err(garde::Error::new(
                "selected chart series value cannot be empty",
            ));
        }
        if series.iter().any(|series| series.value == *selected_value) {
            Ok(())
        } else {
            Err(garde::Error::new(format!(
                "selected chart series `{selected_value}` does not exist"
            )))
        }
    }
}

fn chart_series_values_are_unique(series: &[ChartSeries], _context: &()) -> garde::Result {
    for (index, item) in series.iter().enumerate() {
        if series
            .iter()
            .skip(index + 1)
            .any(|other| other.value == item.value)
        {
            return Err(garde::Error::new(format!(
                "duplicate chart series value `{}`",
                item.value
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_chart_model(&default_chart_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_series() {
        let model = ChartModel::new("Chart", "Axis", "%", Vec::new());
        assert!(validate_chart_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_series_values() {
        let model = ChartModel::new(
            "Chart",
            "Axis",
            "%",
            vec![
                ChartSeries::new("One", "same", 10),
                ChartSeries::new("Two", "same", 20),
            ],
        );
        assert!(validate_chart_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_value() {
        let model = default_chart_model().with_selected_value("missing");
        assert!(validate_chart_model(&model).is_err());
    }

    #[test]
    fn state_selects_and_clears_series() {
        let mut state = default_chart_model().state();
        assert!(state.is_selected("implemented"));
        assert_eq!(
            state.apply(ChartIntent::Select("remaining".to_owned())),
            ChartChange::Selected("remaining".to_owned())
        );
        assert!(state.is_selected("remaining"));
        assert_eq!(state.apply(ChartIntent::Clear), ChartChange::Cleared);
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_chart_model();
        let nodes = chart_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(ChartPart::Container)
        );
        for part in ChartPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == ChartPart::Series)
                .count(),
            model.series.len()
        );
    }
}
