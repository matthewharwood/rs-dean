use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawerSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl DrawerSide {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    pub const fn axis(self) -> &'static str {
        match self {
            Self::Top | Self::Bottom => "y",
            Self::Right | Self::Left => "x",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawerPart {
    Root,
    Trigger,
    Content,
    Header,
    Footer,
    Handle,
}

impl DrawerPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Trigger,
        Self::Content,
        Self::Header,
        Self::Footer,
        Self::Handle,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Drawer",
            Self::Trigger => "DrawerTrigger",
            Self::Content => "DrawerContent",
            Self::Header => "DrawerHeader",
            Self::Footer => "DrawerFooter",
            Self::Handle => "DrawerHandle",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DrawerAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub close_drawer: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DrawerModel {
    #[garde(skip)]
    pub side: DrawerSide,
    #[garde(length(min = 1, max = 96))]
    pub trigger_label: String,
    #[garde(length(min = 1, max = 128))]
    pub title: String,
    #[garde(length(min = 1, max = 240))]
    pub description: String,
    #[garde(length(min = 1, max = 800))]
    pub body: String,
    #[garde(length(max = 3), dive, custom(drawer_actions_are_valid))]
    pub actions: Vec<DrawerAction>,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawerState {
    open: bool,
    focused: Option<DrawerPart>,
    dragging: bool,
    active_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawerIntent {
    Open,
    Close,
    Toggle,
    Focus(DrawerPart),
    Blur,
    StartDrag,
    EndDrag,
    ActivateFooter(String),
    ClearAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawerChange {
    Opened,
    Closed,
    Focused(DrawerPart),
    Blurred,
    DragStarted,
    DragEnded,
    FooterActivated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawerRenderNode {
    pub part: DrawerPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub side: DrawerSide,
    pub open: bool,
    pub focused: bool,
    pub dragging: bool,
    pub actionable: bool,
    pub close_drawer: bool,
    pub selected: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl DrawerAction {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            close_drawer: true,
        }
    }

    pub const fn keep_open(mut self) -> Self {
        self.close_drawer = false;
        self
    }
}

impl DrawerModel {
    pub fn new(
        trigger_label: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            side: DrawerSide::Bottom,
            trigger_label: trigger_label.into(),
            title: title.into(),
            description: description.into(),
            body: body.into(),
            actions: vec![
                DrawerAction::new("Submit", "submit"),
                DrawerAction::new("Cancel", "cancel"),
            ],
            default_open: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    pub fn with_actions(mut self, actions: Vec<DrawerAction>) -> Self {
        self.actions = actions;
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

    pub const fn state(&self) -> DrawerState {
        DrawerState {
            open: self.default_open,
            focused: None,
            dragging: false,
            active_action: None,
        }
    }
}

impl DrawerState {
    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub const fn focused_part(&self) -> Option<DrawerPart> {
        self.focused
    }

    pub const fn is_dragging(&self) -> bool {
        self.dragging
    }

    pub fn active_action(&self) -> Option<&str> {
        self.active_action.as_deref()
    }

    pub fn apply(&mut self, intent: DrawerIntent) -> DrawerChange {
        match intent {
            DrawerIntent::Open => self.open(),
            DrawerIntent::Close => self.close(),
            DrawerIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            DrawerIntent::Focus(part) => self.focus(part),
            DrawerIntent::Blur => self.blur(),
            DrawerIntent::StartDrag => self.start_drag(),
            DrawerIntent::EndDrag => self.end_drag(),
            DrawerIntent::ActivateFooter(value) => self.activate_footer(value),
            DrawerIntent::ClearAction => self.clear_action(),
        }
    }

    fn open(&mut self) -> DrawerChange {
        if self.open {
            DrawerChange::Unchanged
        } else {
            self.open = true;
            DrawerChange::Opened
        }
    }

    fn close(&mut self) -> DrawerChange {
        if self.open {
            self.open = false;
            self.dragging = false;
            DrawerChange::Closed
        } else {
            DrawerChange::Unchanged
        }
    }

    fn focus(&mut self, part: DrawerPart) -> DrawerChange {
        if self.focused == Some(part) {
            DrawerChange::Unchanged
        } else {
            self.focused = Some(part);
            DrawerChange::Focused(part)
        }
    }

    fn blur(&mut self) -> DrawerChange {
        if self.focused.is_some() {
            self.focused = None;
            DrawerChange::Blurred
        } else {
            DrawerChange::Unchanged
        }
    }

    fn start_drag(&mut self) -> DrawerChange {
        if self.dragging || !self.open {
            DrawerChange::Unchanged
        } else {
            self.dragging = true;
            DrawerChange::DragStarted
        }
    }

    fn end_drag(&mut self) -> DrawerChange {
        if self.dragging {
            self.dragging = false;
            DrawerChange::DragEnded
        } else {
            DrawerChange::Unchanged
        }
    }

    fn activate_footer(&mut self, value: String) -> DrawerChange {
        if value.is_empty() || self.active_action.as_ref() == Some(&value) {
            return DrawerChange::Unchanged;
        }
        self.active_action = Some(value.clone());
        DrawerChange::FooterActivated(value)
    }

    fn clear_action(&mut self) -> DrawerChange {
        if self.active_action.is_none() {
            DrawerChange::Unchanged
        } else {
            self.active_action = None;
            DrawerChange::Cleared
        }
    }
}

pub fn validate_drawer_model(model: &DrawerModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn drawer_render_nodes(model: &DrawerModel, state: &DrawerState) -> Vec<DrawerRenderNode> {
    let blocked = model.disabled || model.loading;
    let visible = state.is_open();
    let mut nodes = Vec::with_capacity(model.actions.len().saturating_add(5));
    nodes.push(DrawerRenderNode {
        part: DrawerPart::Root,
        index: 0,
        value: model.side.label().to_owned(),
        label: model.title.clone(),
        detail: format!(
            "{} drawer on the {} axis",
            model.side.label(),
            model.side.axis()
        ),
        side: model.side,
        open: state.is_open(),
        focused: false,
        dragging: state.is_dragging(),
        actionable: false,
        close_drawer: false,
        selected: state.is_open(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DrawerRenderNode {
        part: DrawerPart::Trigger,
        index: 0,
        value: model.trigger_label.clone(),
        label: model.trigger_label.clone(),
        detail: "Open drawer".to_owned(),
        side: model.side,
        open: state.is_open(),
        focused: state.focused_part() == Some(DrawerPart::Trigger),
        dragging: false,
        actionable: true,
        close_drawer: false,
        selected: state.is_open(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DrawerRenderNode {
        part: DrawerPart::Content,
        index: 0,
        value: model.title.clone(),
        label: model.title.clone(),
        detail: model.body.clone(),
        side: model.side,
        open: state.is_open(),
        focused: false,
        dragging: state.is_dragging(),
        actionable: false,
        close_drawer: false,
        selected: false,
        visible,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DrawerRenderNode {
        part: DrawerPart::Header,
        index: 0,
        value: model.title.clone(),
        label: model.title.clone(),
        detail: model.description.clone(),
        side: model.side,
        open: state.is_open(),
        focused: false,
        dragging: false,
        actionable: false,
        close_drawer: false,
        selected: false,
        visible,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DrawerRenderNode {
        part: DrawerPart::Handle,
        index: 0,
        value: model.side.axis().to_owned(),
        label: "Drawer handle".to_owned(),
        detail: if state.is_dragging() {
            "Release drawer handle".to_owned()
        } else {
            "Drag drawer handle".to_owned()
        },
        side: model.side,
        open: state.is_open(),
        focused: state.focused_part() == Some(DrawerPart::Handle),
        dragging: state.is_dragging(),
        actionable: true,
        close_drawer: false,
        selected: state.is_dragging(),
        visible,
        loading: model.loading,
        disabled: blocked || !visible,
    });
    for (index, action) in model.actions.iter().enumerate() {
        nodes.push(DrawerRenderNode {
            part: DrawerPart::Footer,
            index,
            value: action.value.clone(),
            label: action.label.clone(),
            detail: if action.close_drawer {
                "Close drawer action".to_owned()
            } else {
                "Keep drawer open action".to_owned()
            },
            side: model.side,
            open: state.is_open(),
            focused: state.focused_part() == Some(DrawerPart::Footer),
            dragging: false,
            actionable: true,
            close_drawer: action.close_drawer,
            selected: state.active_action() == Some(action.value.as_str()),
            visible,
            loading: model.loading,
            disabled: blocked || !visible,
        });
    }
    if model.actions.is_empty() {
        nodes.push(DrawerRenderNode {
            part: DrawerPart::Footer,
            index: 0,
            value: "empty".to_owned(),
            label: "No actions".to_owned(),
            detail: "Footer has no actions".to_owned(),
            side: model.side,
            open: state.is_open(),
            focused: false,
            dragging: false,
            actionable: false,
            close_drawer: false,
            selected: false,
            visible,
            loading: model.loading,
            disabled: true,
        });
    }
    nodes
}

pub fn default_drawer_model() -> DrawerModel {
    DrawerModel::new(
        "Open drawer",
        "Mobile workflow",
        "Complete a focused task without leaving the page.",
        "Drawer content is side-aware, token-rendered, and portable across Leptos DOM and Bevy primitive projections.",
    )
}

fn drawer_actions_are_valid(actions: &Vec<DrawerAction>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(actions.len());
    for action in actions {
        if !values.insert(action.value.as_str()) {
            return Err(garde::Error::new("drawer action values must be unique"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_drawer_model(&default_drawer_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = DrawerModel::new("Open", "", "Description", "Body");
        assert!(validate_drawer_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_action_values() {
        let model = DrawerModel::new("Open", "Title", "Description", "Body").with_actions(vec![
            DrawerAction::new("Submit", "same"),
            DrawerAction::new("Cancel", "same"),
        ]);
        assert!(validate_drawer_model(&model).is_err());
    }

    #[test]
    fn state_opens_drags_activates_and_closes() {
        let mut state = default_drawer_model().state();
        assert_eq!(
            state.apply(DrawerIntent::StartDrag),
            DrawerChange::Unchanged
        );
        assert_eq!(state.apply(DrawerIntent::Open), DrawerChange::Opened);
        assert_eq!(
            state.apply(DrawerIntent::StartDrag),
            DrawerChange::DragStarted
        );
        assert!(state.is_dragging());
        assert_eq!(
            state.apply(DrawerIntent::ActivateFooter("submit".to_owned())),
            DrawerChange::FooterActivated("submit".to_owned())
        );
        assert_eq!(state.active_action(), Some("submit"));
        assert_eq!(state.apply(DrawerIntent::Close), DrawerChange::Closed);
        assert!(!state.is_dragging());
    }

    #[test]
    fn render_nodes_cover_repeatable_drawer_anatomy() {
        let model = default_drawer_model().with_default_open(true);
        let nodes = drawer_render_nodes(&model, &model.state());
        for part in DrawerPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == DrawerPart::Footer)
                .count(),
            model.actions.len()
        );
    }

    #[test]
    fn closed_state_hides_content_nodes() {
        let model = default_drawer_model();
        let nodes = drawer_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| matches!(
                    node.part,
                    DrawerPart::Content
                        | DrawerPart::Header
                        | DrawerPart::Footer
                        | DrawerPart::Handle
                ))
                .all(|node| !node.visible)
        );
    }

    #[test]
    fn loading_disables_actionable_nodes() {
        let model = default_drawer_model().with_default_open(true).loading();
        let nodes = drawer_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| node.actionable)
                .all(|node| node.disabled)
        );
    }
}
