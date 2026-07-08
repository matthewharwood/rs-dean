use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AlertTone {
    Default,
    Info,
    Success,
    Warning,
    Destructive,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AlertDensity {
    Standard,
    Dense,
}

impl AlertTone {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Destructive => "destructive",
        }
    }
}

impl AlertDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl AlertPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Alert",
            Self::Title => "AlertTitle",
            Self::Description => "AlertDescription",
            Self::Action => "AlertAction",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AlertAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AlertModel {
    #[garde(skip)]
    pub tone: AlertTone,
    #[garde(skip)]
    pub density: AlertDensity,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 2_000))]
    pub description: String,
    #[garde(custom(validate_optional_alert_action))]
    pub action: Option<AlertAction>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertPart {
    Root,
    Title,
    Description,
    Action,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertIntent {
    Activate(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlertRenderNode {
    pub part: AlertPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub tone: AlertTone,
    pub density: AlertDensity,
    pub loading: bool,
    pub disabled: bool,
}

impl AlertAction {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl AlertModel {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            tone: AlertTone::Default,
            density: AlertDensity::Standard,
            title: title.into(),
            description: description.into(),
            action: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_tone(mut self, tone: AlertTone) -> Self {
        self.tone = tone;
        self
    }

    pub const fn with_density(mut self, density: AlertDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_action(mut self, action: AlertAction) -> Self {
        self.action = Some(action);
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
}

pub fn validate_alert_model(model: &AlertModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn alert_render_nodes(model: &AlertModel) -> Vec<AlertRenderNode> {
    let mut nodes = Vec::with_capacity(if model.action.is_some() { 4 } else { 3 });
    nodes.push(AlertRenderNode {
        part: AlertPart::Root,
        value: "alert".to_owned(),
        label: "Alert".to_owned(),
        detail: match model.tone {
            AlertTone::Default => "Default status callout",
            AlertTone::Info => "Informational status callout",
            AlertTone::Success => "Successful status callout",
            AlertTone::Warning => "Warning status callout",
            AlertTone::Destructive => "Destructive status callout",
        }
        .to_owned(),
        tone: model.tone,
        density: model.density,
        loading: model.loading,
        disabled: model.disabled,
    });
    nodes.push(AlertRenderNode {
        part: AlertPart::Title,
        value: "alert-title".to_owned(),
        label: model.title.clone(),
        detail: "Alert title".to_owned(),
        tone: model.tone,
        density: model.density,
        loading: model.loading,
        disabled: model.disabled,
    });
    nodes.push(AlertRenderNode {
        part: AlertPart::Description,
        value: "alert-description".to_owned(),
        label: "Alert description".to_owned(),
        detail: model.description.clone(),
        tone: model.tone,
        density: model.density,
        loading: model.loading,
        disabled: model.disabled,
    });

    if let Some(action) = &model.action {
        nodes.push(AlertRenderNode {
            part: AlertPart::Action,
            value: action.value.clone(),
            label: action.label.clone(),
            detail: "Alert action".to_owned(),
            tone: model.tone,
            density: model.density,
            loading: model.loading,
            disabled: model.disabled || model.loading || action.disabled,
        });
    }

    nodes
}

pub fn default_alert_model() -> AlertModel {
    AlertModel::new(
        "Build completed",
        "The latest design-token bundle is ready for review.",
    )
    .with_tone(AlertTone::Success)
    .with_action(AlertAction::new("Open report", "open-report"))
}

fn validate_optional_alert_action(action: &Option<AlertAction>, _context: &()) -> garde::Result {
    if let Some(action) = action {
        action.validate().map_err(|report| {
            garde::Error::new(format!("invalid alert action contract: {report}"))
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_alert_model(&default_alert_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = AlertModel::new("", "A useful description");
        assert!(validate_alert_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_description() {
        let model = AlertModel::new("Title", "");
        assert!(validate_alert_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_action_contract() {
        let model = AlertModel::new("Title", "Description").with_action(AlertAction::new("", ""));
        assert!(validate_alert_model(&model).is_err());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let nodes = alert_render_nodes(&default_alert_model());
        assert_eq!(nodes.first().map(|node| node.part), Some(AlertPart::Root));
        assert_eq!(nodes.first().map(|node| node.label.as_str()), Some("Alert"));
        assert!(nodes.iter().any(|node| node.part == AlertPart::Title));
        assert!(nodes.iter().any(|node| node.part == AlertPart::Description));
        assert!(nodes.iter().any(|node| node.part == AlertPart::Action));
    }

    #[test]
    fn loading_alert_disables_action_node() {
        let model = default_alert_model().loading();
        let nodes = alert_render_nodes(&model);
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AlertPart::Action && node.disabled)
        );
    }
}
