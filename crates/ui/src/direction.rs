use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DirectionValue {
    Ltr,
    Rtl,
}

impl DirectionValue {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Ltr => "Left to right",
            Self::Rtl => "Right to left",
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Ltr => Self::Rtl,
            Self::Rtl => Self::Ltr,
        }
    }

    pub const fn is_rtl(self) -> bool {
        matches!(self, Self::Rtl)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DirectionPart {
    Provider,
    Scope,
    AwareContent,
}

impl DirectionPart {
    pub const ALL: &'static [Self] = &[Self::Provider, Self::Scope, Self::AwareContent];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Provider => "DirectionProvider",
            Self::Scope => "DirectionScope",
            Self::AwareContent => "DirectionAwareContent",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DirectionModel {
    #[garde(skip)]
    pub default_direction: DirectionValue,
    #[garde(skip)]
    pub scope_direction: DirectionValue,
    #[garde(length(min = 1, max = 128))]
    pub provider_label: String,
    #[garde(length(min = 1, max = 128))]
    pub scope_label: String,
    #[garde(length(min = 1, max = 600))]
    pub content: String,
    #[garde(skip)]
    pub default_scope_active: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectionState {
    direction: DirectionValue,
    scope_active: bool,
    focused: Option<DirectionPart>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionIntent {
    Set(DirectionValue),
    Toggle,
    ActivateScope,
    DeactivateScope,
    ToggleScope,
    Focus(DirectionPart),
    Blur,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionChange {
    Changed(DirectionValue),
    ScopeActivated,
    ScopeDeactivated,
    Focused(DirectionPart),
    Blurred,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectionRenderNode {
    pub part: DirectionPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub direction: DirectionValue,
    pub inherited_direction: DirectionValue,
    pub scope_direction: DirectionValue,
    pub scope_active: bool,
    pub focused: bool,
    pub actionable: bool,
    pub selected: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl DirectionModel {
    pub fn new(
        provider_label: impl Into<String>,
        scope_label: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            default_direction: DirectionValue::Ltr,
            scope_direction: DirectionValue::Rtl,
            provider_label: provider_label.into(),
            scope_label: scope_label.into(),
            content: content.into(),
            default_scope_active: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_direction(mut self, direction: DirectionValue) -> Self {
        self.default_direction = direction;
        self
    }

    pub const fn with_scope_direction(mut self, direction: DirectionValue) -> Self {
        self.scope_direction = direction;
        self
    }

    pub const fn with_default_scope_active(mut self, active: bool) -> Self {
        self.default_scope_active = active;
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

    pub const fn state(&self) -> DirectionState {
        DirectionState::new(self.default_direction, self.default_scope_active)
    }
}

impl DirectionState {
    pub const fn new(direction: DirectionValue, scope_active: bool) -> Self {
        Self {
            direction,
            scope_active,
            focused: None,
        }
    }

    pub const fn direction(self) -> DirectionValue {
        self.direction
    }

    pub const fn scope_active(self) -> bool {
        self.scope_active
    }

    pub const fn focused_part(self) -> Option<DirectionPart> {
        self.focused
    }

    pub const fn effective_direction(self, model: &DirectionModel) -> DirectionValue {
        if self.scope_active {
            model.scope_direction
        } else {
            self.direction
        }
    }

    pub fn apply(&mut self, intent: DirectionIntent) -> DirectionChange {
        match intent {
            DirectionIntent::Set(direction) => self.set_direction(direction),
            DirectionIntent::Toggle => self.set_direction(self.direction.next()),
            DirectionIntent::ActivateScope => self.activate_scope(),
            DirectionIntent::DeactivateScope => self.deactivate_scope(),
            DirectionIntent::ToggleScope => {
                if self.scope_active {
                    self.deactivate_scope()
                } else {
                    self.activate_scope()
                }
            }
            DirectionIntent::Focus(part) => self.focus(part),
            DirectionIntent::Blur => self.blur(),
        }
    }

    fn set_direction(&mut self, direction: DirectionValue) -> DirectionChange {
        if self.direction == direction {
            DirectionChange::Unchanged
        } else {
            self.direction = direction;
            DirectionChange::Changed(direction)
        }
    }

    fn activate_scope(&mut self) -> DirectionChange {
        if self.scope_active {
            DirectionChange::Unchanged
        } else {
            self.scope_active = true;
            DirectionChange::ScopeActivated
        }
    }

    fn deactivate_scope(&mut self) -> DirectionChange {
        if self.scope_active {
            self.scope_active = false;
            DirectionChange::ScopeDeactivated
        } else {
            DirectionChange::Unchanged
        }
    }

    fn focus(&mut self, part: DirectionPart) -> DirectionChange {
        if self.focused == Some(part) {
            DirectionChange::Unchanged
        } else {
            self.focused = Some(part);
            DirectionChange::Focused(part)
        }
    }

    fn blur(&mut self) -> DirectionChange {
        if self.focused.is_some() {
            self.focused = None;
            DirectionChange::Blurred
        } else {
            DirectionChange::Unchanged
        }
    }
}

pub fn validate_direction_model(model: &DirectionModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn direction_render_nodes(
    model: &DirectionModel,
    state: &DirectionState,
) -> Vec<DirectionRenderNode> {
    let blocked = model.loading || model.disabled;
    let provider_direction = state.direction();
    let effective_direction = state.effective_direction(model);
    vec![
        DirectionRenderNode {
            part: DirectionPart::Provider,
            index: 0,
            value: provider_direction.label().to_owned(),
            label: model.provider_label.clone(),
            detail: format!("Document direction is {}", provider_direction.name()),
            direction: provider_direction,
            inherited_direction: provider_direction,
            scope_direction: model.scope_direction,
            scope_active: state.scope_active(),
            focused: state.focused_part() == Some(DirectionPart::Provider),
            actionable: true,
            selected: provider_direction.is_rtl(),
            loading: model.loading,
            disabled: blocked,
        },
        DirectionRenderNode {
            part: DirectionPart::Scope,
            index: 0,
            value: model.scope_direction.label().to_owned(),
            label: model.scope_label.clone(),
            detail: if state.scope_active() {
                format!("Nested scope overrides to {}", model.scope_direction.name())
            } else {
                format!("Nested scope is ready for {}", model.scope_direction.name())
            },
            direction: model.scope_direction,
            inherited_direction: provider_direction,
            scope_direction: model.scope_direction,
            scope_active: state.scope_active(),
            focused: state.focused_part() == Some(DirectionPart::Scope),
            actionable: true,
            selected: state.scope_active(),
            loading: model.loading,
            disabled: blocked,
        },
        DirectionRenderNode {
            part: DirectionPart::AwareContent,
            index: 0,
            value: effective_direction.label().to_owned(),
            label: "Direction-aware content".to_owned(),
            detail: model.content.clone(),
            direction: effective_direction,
            inherited_direction: provider_direction,
            scope_direction: model.scope_direction,
            scope_active: state.scope_active(),
            focused: state.focused_part() == Some(DirectionPart::AwareContent),
            actionable: false,
            selected: effective_direction.is_rtl(),
            loading: model.loading,
            disabled: model.disabled,
        },
    ]
}

pub fn default_direction_model() -> DirectionModel {
    DirectionModel::new(
        "Application direction",
        "RTL content scope",
        "This content reads from the nearest direction provider so Leptos DOM and Bevy primitive projections agree on the active flow.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_direction_model(&default_direction_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_provider_label() {
        let model = DirectionModel::new("", "RTL scope", "Content");
        assert!(validate_direction_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_content() {
        let model = DirectionModel::new("Provider", "RTL scope", "");
        assert!(validate_direction_model(&model).is_err());
    }

    #[test]
    fn state_toggles_direction_and_scope() {
        let mut state = default_direction_model().state();
        assert_eq!(state.direction(), DirectionValue::Ltr);
        assert_eq!(
            state.apply(DirectionIntent::Toggle),
            DirectionChange::Changed(DirectionValue::Rtl)
        );
        assert_eq!(state.direction(), DirectionValue::Rtl);
        assert!(!state.scope_active());
        assert_eq!(
            state.apply(DirectionIntent::ActivateScope),
            DirectionChange::ScopeActivated
        );
        assert!(state.scope_active());
        assert_eq!(
            state.apply(DirectionIntent::DeactivateScope),
            DirectionChange::ScopeDeactivated
        );
    }

    #[test]
    fn render_nodes_cover_direction_anatomy() {
        let model = default_direction_model();
        let nodes = direction_render_nodes(&model, &model.state());
        for part in DirectionPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn scope_controls_effective_direction() {
        let model = default_direction_model().with_scope_direction(DirectionValue::Rtl);
        let mut state = DirectionState::new(DirectionValue::Ltr, false);
        let content = direction_render_nodes(&model, &state)
            .into_iter()
            .find(|node| node.part == DirectionPart::AwareContent)
            .expect("direction content node");
        assert_eq!(content.direction, DirectionValue::Ltr);
        let _ = state.apply(DirectionIntent::ActivateScope);
        let content = direction_render_nodes(&model, &state)
            .into_iter()
            .find(|node| node.part == DirectionPart::AwareContent)
            .expect("direction content node");
        assert_eq!(content.direction, DirectionValue::Rtl);
    }

    #[test]
    fn loading_disables_actionable_nodes() {
        let model = default_direction_model().loading();
        let nodes = direction_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| node.actionable)
                .all(|node| node.disabled)
        );
    }
}
