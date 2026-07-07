use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SheetDensity {
    Standard,
    Dense,
}

impl SheetDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SheetSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SheetPart {
    Root,
    Trigger,
    Content,
    Header,
    Footer,
    Close,
}

impl SheetPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Trigger,
        Self::Content,
        Self::Header,
        Self::Footer,
        Self::Close,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Sheet",
            Self::Trigger => "SheetTrigger",
            Self::Content => "SheetContent",
            Self::Header => "SheetHeader",
            Self::Footer => "SheetFooter",
            Self::Close => "SheetClose",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SheetAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub close_sheet: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SheetModel {
    #[garde(skip)]
    pub density: SheetDensity,
    #[garde(skip)]
    pub side: SheetSide,
    #[garde(length(min = 1, max = 96))]
    pub trigger_label: String,
    #[garde(length(min = 1, max = 128))]
    pub title: String,
    #[garde(length(min = 1, max = 240))]
    pub description: String,
    #[garde(length(min = 1, max = 800))]
    pub body: String,
    #[garde(length(min = 1, max = 3), dive, custom(sheet_actions_are_valid))]
    pub actions: Vec<SheetAction>,
    #[garde(length(min = 1, max = 96))]
    pub close_label: String,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetState {
    open: bool,
    focused: Option<SheetPart>,
    active_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SheetIntent {
    Open,
    Close,
    Toggle,
    Focus(SheetPart),
    Blur,
    ActivateFooter(String),
    ClearAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SheetChange {
    Opened,
    Closed,
    Focused(SheetPart),
    Blurred,
    FooterActivated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetRenderNode {
    pub part: SheetPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: SheetDensity,
    pub side: SheetSide,
    pub open: bool,
    pub focused: bool,
    pub actionable: bool,
    pub close_sheet: bool,
    pub selected: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl SheetAction {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            close_sheet: false,
        }
    }

    pub const fn close_sheet(mut self) -> Self {
        self.close_sheet = true;
        self
    }
}

impl SheetModel {
    pub fn new(
        trigger_label: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            density: SheetDensity::Standard,
            side: SheetSide::Right,
            trigger_label: trigger_label.into(),
            title: title.into(),
            description: description.into(),
            body: body.into(),
            actions: vec![
                SheetAction::new("Save changes", "save"),
                SheetAction::new("Cancel", "cancel").close_sheet(),
            ],
            close_label: "Close sheet".to_owned(),
            default_open: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SheetDensity) -> Self {
        self.density = density;
        self
    }

    pub const fn with_side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    pub fn with_actions(mut self, actions: Vec<SheetAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn with_close_label(mut self, close_label: impl Into<String>) -> Self {
        self.close_label = close_label.into();
        self
    }

    pub const fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
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

    pub const fn state(&self) -> SheetState {
        SheetState {
            open: self.default_open,
            focused: None,
            active_action: None,
        }
    }
}

impl SheetState {
    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub const fn focused_part(&self) -> Option<SheetPart> {
        self.focused
    }

    pub fn active_action(&self) -> Option<&str> {
        self.active_action.as_deref()
    }

    pub fn apply(&mut self, intent: SheetIntent) -> SheetChange {
        match intent {
            SheetIntent::Open => self.open(),
            SheetIntent::Close => self.close(),
            SheetIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            SheetIntent::Focus(part) => self.focus(part),
            SheetIntent::Blur => self.blur(),
            SheetIntent::ActivateFooter(value) => self.activate_footer(value),
            SheetIntent::ClearAction => self.clear_action(),
        }
    }

    fn open(&mut self) -> SheetChange {
        if self.open {
            SheetChange::Unchanged
        } else {
            self.open = true;
            SheetChange::Opened
        }
    }

    fn close(&mut self) -> SheetChange {
        if self.open {
            self.open = false;
            self.focused = None;
            SheetChange::Closed
        } else {
            SheetChange::Unchanged
        }
    }

    fn focus(&mut self, part: SheetPart) -> SheetChange {
        if self.focused == Some(part) {
            SheetChange::Unchanged
        } else {
            self.focused = Some(part);
            SheetChange::Focused(part)
        }
    }

    fn blur(&mut self) -> SheetChange {
        if self.focused.is_some() {
            self.focused = None;
            SheetChange::Blurred
        } else {
            SheetChange::Unchanged
        }
    }

    fn activate_footer(&mut self, value: String) -> SheetChange {
        if value.is_empty() || self.active_action.as_ref() == Some(&value) {
            return SheetChange::Unchanged;
        }
        self.active_action = Some(value.clone());
        SheetChange::FooterActivated(value)
    }

    fn clear_action(&mut self) -> SheetChange {
        if self.active_action.is_none() {
            SheetChange::Unchanged
        } else {
            self.active_action = None;
            SheetChange::Cleared
        }
    }
}

pub fn validate_sheet_model(model: &SheetModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn sheet_render_nodes(model: &SheetModel, state: &SheetState) -> Vec<SheetRenderNode> {
    let blocked = model.disabled || model.loading;
    let visible = state.is_open();
    let mut nodes = Vec::with_capacity(model.actions.len().saturating_add(5));
    nodes.push(SheetRenderNode {
        part: SheetPart::Root,
        index: 0,
        value: model.side.label().to_owned(),
        label: model.title.clone(),
        detail: format!("{} sheet", model.side.label()),
        density: model.density,
        side: model.side,
        open: state.is_open(),
        focused: false,
        actionable: false,
        close_sheet: false,
        selected: state.is_open(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(SheetRenderNode {
        part: SheetPart::Trigger,
        index: 0,
        value: model.trigger_label.clone(),
        label: model.trigger_label.clone(),
        detail: "Open sheet".to_owned(),
        density: model.density,
        side: model.side,
        open: state.is_open(),
        focused: state.focused_part() == Some(SheetPart::Trigger),
        actionable: true,
        close_sheet: false,
        selected: state.is_open(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(SheetRenderNode {
        part: SheetPart::Content,
        index: 0,
        value: model.title.clone(),
        label: model.title.clone(),
        detail: model.body.clone(),
        density: model.density,
        side: model.side,
        open: state.is_open(),
        focused: state.focused_part() == Some(SheetPart::Content),
        actionable: false,
        close_sheet: false,
        selected: false,
        visible,
        loading: model.loading,
        disabled: blocked || !visible,
    });
    nodes.push(SheetRenderNode {
        part: SheetPart::Header,
        index: 0,
        value: model.title.clone(),
        label: model.title.clone(),
        detail: model.description.clone(),
        density: model.density,
        side: model.side,
        open: state.is_open(),
        focused: false,
        actionable: false,
        close_sheet: false,
        selected: false,
        visible,
        loading: model.loading,
        disabled: blocked || !visible,
    });
    for (index, action) in model.actions.iter().enumerate() {
        nodes.push(SheetRenderNode {
            part: SheetPart::Footer,
            index,
            value: action.value.clone(),
            label: action.label.clone(),
            detail: if action.close_sheet {
                "Close sheet action".to_owned()
            } else {
                "Keep sheet open action".to_owned()
            },
            density: model.density,
            side: model.side,
            open: state.is_open(),
            focused: state.focused_part() == Some(SheetPart::Footer),
            actionable: true,
            close_sheet: action.close_sheet,
            selected: state.active_action() == Some(action.value.as_str()),
            visible,
            loading: model.loading,
            disabled: blocked || !visible,
        });
    }
    nodes.push(SheetRenderNode {
        part: SheetPart::Close,
        index: 0,
        value: "close".to_owned(),
        label: model.close_label.clone(),
        detail: "Close sheet".to_owned(),
        density: model.density,
        side: model.side,
        open: state.is_open(),
        focused: state.focused_part() == Some(SheetPart::Close),
        actionable: true,
        close_sheet: true,
        selected: false,
        visible,
        loading: model.loading,
        disabled: blocked || !visible,
    });
    nodes
}

pub fn default_sheet_model() -> SheetModel {
    SheetModel::new(
        "Open sheet",
        "Project settings",
        "Update scoped settings without leaving the current workflow.",
        "Sheet content is edge-aware, token-rendered, and portable across Leptos DOM and Bevy primitive projections.",
    )
}

fn sheet_actions_are_valid(actions: &Vec<SheetAction>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(actions.len());
    for action in actions {
        if !values.insert(action.value.as_str()) {
            return Err(garde::Error::new("sheet action values must be unique"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_sheet_model(&default_sheet_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = SheetModel::new("Open", "", "Description", "Body");
        assert!(validate_sheet_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_close_label() {
        let model = default_sheet_model().with_close_label("");
        assert!(validate_sheet_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_action_values() {
        let model = SheetModel::new("Open", "Title", "Description", "Body").with_actions(vec![
            SheetAction::new("Submit", "same"),
            SheetAction::new("Cancel", "same"),
        ]);
        assert!(validate_sheet_model(&model).is_err());
    }

    #[test]
    fn closed_state_hides_content_header_footer_and_close() {
        let model = default_sheet_model();
        let state = model.state();
        let nodes = sheet_render_nodes(&model, &state);

        for part in [
            SheetPart::Content,
            SheetPart::Header,
            SheetPart::Footer,
            SheetPart::Close,
        ] {
            assert!(
                nodes
                    .iter()
                    .filter(|node| node.part == part)
                    .all(|node| !node.visible && node.disabled),
                "{part:?} should be hidden and disabled while closed",
            );
        }
    }

    #[test]
    fn loading_disables_actionable_nodes() {
        let model = default_sheet_model().loading().with_default_open(true);
        let state = model.state();

        for node in sheet_render_nodes(&model, &state) {
            if node.actionable {
                assert!(node.disabled);
            }
        }
    }

    #[test]
    fn render_nodes_cover_repeatable_sheet_anatomy() {
        let model = default_sheet_model().with_default_open(true);
        let state = model.state();
        let nodes = sheet_render_nodes(&model, &state);

        assert_eq!(nodes.first().map(|node| node.part), Some(SheetPart::Root));
        for part in SheetPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn state_opens_focuses_activates_and_closes() {
        let model = default_sheet_model();
        let mut state = model.state();

        assert_eq!(state.apply(SheetIntent::Open), SheetChange::Opened);
        assert!(state.is_open());
        assert_eq!(
            state.apply(SheetIntent::Focus(SheetPart::Trigger)),
            SheetChange::Focused(SheetPart::Trigger)
        );
        assert_eq!(
            state.apply(SheetIntent::ActivateFooter("save".to_owned())),
            SheetChange::FooterActivated("save".to_owned())
        );
        assert_eq!(state.active_action(), Some("save"));
        assert_eq!(state.apply(SheetIntent::Close), SheetChange::Closed);
        assert!(!state.is_open());
    }

    #[test]
    fn side_reaches_every_render_node() {
        let model = default_sheet_model().with_side(SheetSide::Left);
        let state = model.state();

        for node in sheet_render_nodes(&model, &state) {
            assert_eq!(node.side, SheetSide::Left);
        }
    }
}
