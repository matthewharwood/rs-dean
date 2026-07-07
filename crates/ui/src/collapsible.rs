use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CollapsibleDensity {
    Standard,
    Dense,
}

impl CollapsibleDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl CollapsiblePart {
    pub const ALL: &'static [Self] = &[Self::Root, Self::Trigger, Self::Content];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Collapsible",
            Self::Trigger => "CollapsibleTrigger",
            Self::Content => "CollapsibleContent",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CollapsibleModel {
    #[garde(skip)]
    pub density: CollapsibleDensity,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 2_000))]
    pub content: String,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollapsibleState {
    open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsibleIntent {
    Toggle,
    Open,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsibleChange {
    Opened,
    Closed,
    Unchanged,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CollapsiblePart {
    Root,
    Trigger,
    Content,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollapsibleRenderNode {
    pub part: CollapsiblePart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: CollapsibleDensity,
    pub open: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl CollapsibleModel {
    pub fn new(
        value: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            density: CollapsibleDensity::Standard,
            value: value.into(),
            title: title.into(),
            content: content.into(),
            default_open: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: CollapsibleDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub const fn open(mut self) -> Self {
        self.default_open = true;
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

    pub const fn state(&self) -> CollapsibleState {
        CollapsibleState::new(self.default_open)
    }
}

impl CollapsibleState {
    pub const fn new(open: bool) -> Self {
        Self { open }
    }

    pub const fn is_open(self) -> bool {
        self.open
    }

    pub fn apply(&mut self, intent: CollapsibleIntent) -> CollapsibleChange {
        match intent {
            CollapsibleIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            CollapsibleIntent::Open => self.open(),
            CollapsibleIntent::Close => self.close(),
        }
    }

    fn open(&mut self) -> CollapsibleChange {
        if self.open {
            CollapsibleChange::Unchanged
        } else {
            self.open = true;
            CollapsibleChange::Opened
        }
    }

    fn close(&mut self) -> CollapsibleChange {
        if self.open {
            self.open = false;
            CollapsibleChange::Closed
        } else {
            CollapsibleChange::Unchanged
        }
    }
}

pub fn validate_collapsible_model(model: &CollapsibleModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn collapsible_render_nodes(
    model: &CollapsibleModel,
    state: &CollapsibleState,
) -> Vec<CollapsibleRenderNode> {
    let open = state.is_open();
    let blocked = model.loading || model.disabled;
    vec![
        CollapsibleRenderNode {
            part: CollapsiblePart::Root,
            value: model.value.clone(),
            label: model.title.clone(),
            detail: format!(
                "{} {} collapsible",
                model.density.label(),
                if open { "open" } else { "closed" },
            ),
            density: model.density,
            open,
            loading: model.loading,
            disabled: blocked,
        },
        CollapsibleRenderNode {
            part: CollapsiblePart::Trigger,
            value: model.value.clone(),
            label: model.title.clone(),
            detail: if open {
                "Collapse content".to_owned()
            } else {
                "Expand content".to_owned()
            },
            density: model.density,
            open,
            loading: model.loading,
            disabled: blocked,
        },
        CollapsibleRenderNode {
            part: CollapsiblePart::Content,
            value: model.value.clone(),
            label: "Collapsible content".to_owned(),
            detail: model.content.clone(),
            density: model.density,
            open,
            loading: model.loading,
            disabled: blocked || !open,
        },
    ]
}

pub fn default_collapsible_model() -> CollapsibleModel {
    CollapsibleModel::new(
        "token-stack",
        "Shared token stack",
        "The same Rust disclosure state feeds the Leptos DOM surface and Bevy primitive adapter.",
    )
    .open()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_collapsible_model(&default_collapsible_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_value() {
        let model = CollapsibleModel::new("", "Shared token stack", "Content");
        assert!(validate_collapsible_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = CollapsibleModel::new("token-stack", "", "Content");
        assert!(validate_collapsible_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_content() {
        let model = CollapsibleModel::new("token-stack", "Shared token stack", "");
        assert!(validate_collapsible_model(&model).is_err());
    }

    #[test]
    fn state_opens_closes_and_toggles() {
        let mut state = CollapsibleState::new(false);
        assert!(!state.is_open());
        assert_eq!(
            state.apply(CollapsibleIntent::Open),
            CollapsibleChange::Opened
        );
        assert!(state.is_open());
        assert_eq!(
            state.apply(CollapsibleIntent::Toggle),
            CollapsibleChange::Closed
        );
        assert!(!state.is_open());
        assert_eq!(
            state.apply(CollapsibleIntent::Close),
            CollapsibleChange::Unchanged
        );
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_collapsible_model();
        let nodes = collapsible_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(CollapsiblePart::Root)
        );
        for part in CollapsiblePart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn closed_content_node_is_disabled() {
        let model = default_collapsible_model().default_open(false);
        let nodes = collapsible_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == CollapsiblePart::Content && node.disabled)
        );
    }

    #[test]
    fn loading_disables_trigger_node() {
        let model = default_collapsible_model().loading();
        let nodes = collapsible_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == CollapsiblePart::Trigger && node.disabled)
        );
    }
}
