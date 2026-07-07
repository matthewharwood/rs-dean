use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AttachmentKind {
    Pdf,
    Image,
    Archive,
    Data,
}

impl AttachmentKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Image => "image",
            Self::Archive => "archive",
            Self::Data => "data",
        }
    }

    pub const fn marker(self) -> &'static str {
        match self {
            Self::Pdf => "PDF",
            Self::Image => "IMG",
            Self::Archive => "ZIP",
            Self::Data => "CSV",
        }
    }
}

impl AttachmentPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Attachment",
            Self::Preview => "AttachmentPreview",
            Self::Title => "AttachmentTitle",
            Self::Meta => "AttachmentMeta",
            Self::Action => "AttachmentAction",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AttachmentAction {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct AttachmentModel {
    #[garde(skip)]
    pub kind: AttachmentKind,
    #[garde(length(min = 1, max = 32))]
    pub preview_label: String,
    #[garde(length(min = 1, max = 160))]
    pub title: String,
    #[garde(length(min = 1, max = 160))]
    pub meta: String,
    #[garde(custom(validate_optional_attachment_action))]
    pub action: Option<AttachmentAction>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentState {
    activated: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachmentIntent {
    Activate(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachmentChange {
    Activated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentPart {
    Root,
    Preview,
    Title,
    Meta,
    Action,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentRenderNode {
    pub part: AttachmentPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub kind: AttachmentKind,
    pub loading: bool,
    pub disabled: bool,
}

impl AttachmentAction {
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

impl AttachmentModel {
    pub fn new(title: impl Into<String>, meta: impl Into<String>) -> Self {
        Self {
            kind: AttachmentKind::Pdf,
            preview_label: AttachmentKind::Pdf.marker().to_owned(),
            title: title.into(),
            meta: meta.into(),
            action: Some(AttachmentAction::new("Download", "download-attachment")),
            loading: false,
            disabled: false,
        }
    }

    pub fn with_kind(mut self, kind: AttachmentKind) -> Self {
        self.kind = kind;
        self.preview_label = kind.marker().to_owned();
        self
    }

    pub fn with_preview_label(mut self, preview_label: impl Into<String>) -> Self {
        self.preview_label = preview_label.into();
        self
    }

    pub fn with_action(mut self, action: AttachmentAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn without_action(mut self) -> Self {
        self.action = None;
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

    pub fn state(&self) -> AttachmentState {
        let _ = self;
        AttachmentState::idle()
    }
}

impl AttachmentState {
    pub const fn idle() -> Self {
        Self { activated: None }
    }

    pub fn activated_value(&self) -> Option<&str> {
        self.activated.as_deref()
    }

    pub fn apply(&mut self, intent: AttachmentIntent) -> AttachmentChange {
        match intent {
            AttachmentIntent::Activate(value) => self.activate(value),
            AttachmentIntent::Clear => self.clear(),
        }
    }

    fn activate(&mut self, value: String) -> AttachmentChange {
        if value.is_empty() || self.activated.as_ref() == Some(&value) {
            AttachmentChange::Unchanged
        } else {
            self.activated = Some(value.clone());
            AttachmentChange::Activated(value)
        }
    }

    fn clear(&mut self) -> AttachmentChange {
        if self.activated.is_some() {
            self.activated = None;
            AttachmentChange::Cleared
        } else {
            AttachmentChange::Unchanged
        }
    }
}

pub fn validate_attachment_model(model: &AttachmentModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn attachment_render_nodes(model: &AttachmentModel) -> Vec<AttachmentRenderNode> {
    let mut nodes = Vec::with_capacity(if model.action.is_some() { 5 } else { 4 });
    nodes.push(AttachmentRenderNode {
        part: AttachmentPart::Root,
        value: "attachment".to_owned(),
        label: "Attachment".to_owned(),
        detail: "Message attachment".to_owned(),
        kind: model.kind,
        loading: model.loading,
        disabled: model.disabled,
    });
    nodes.push(AttachmentRenderNode {
        part: AttachmentPart::Preview,
        value: model.kind.label().to_owned(),
        label: model.preview_label.clone(),
        detail: "Attachment preview".to_owned(),
        kind: model.kind,
        loading: model.loading,
        disabled: model.disabled,
    });
    nodes.push(AttachmentRenderNode {
        part: AttachmentPart::Title,
        value: model.title.clone(),
        label: model.title.clone(),
        detail: "Attachment title".to_owned(),
        kind: model.kind,
        loading: model.loading,
        disabled: model.disabled,
    });
    nodes.push(AttachmentRenderNode {
        part: AttachmentPart::Meta,
        value: model.meta.clone(),
        label: "Attachment metadata".to_owned(),
        detail: model.meta.clone(),
        kind: model.kind,
        loading: model.loading,
        disabled: model.disabled,
    });

    if let Some(action) = &model.action {
        nodes.push(AttachmentRenderNode {
            part: AttachmentPart::Action,
            value: action.value.clone(),
            label: action.label.clone(),
            detail: "Attachment action".to_owned(),
            kind: model.kind,
            loading: model.loading,
            disabled: model.disabled || model.loading || action.disabled,
        });
    }

    nodes
}

pub fn default_attachment_model() -> AttachmentModel {
    AttachmentModel::new("roadmap-notes.pdf", "2.4 MB")
}

fn validate_optional_attachment_action(
    action: &Option<AttachmentAction>,
    _context: &(),
) -> garde::Result {
    if let Some(action) = action {
        action.validate().map_err(|report| {
            garde::Error::new(format!("invalid attachment action contract: {report}"))
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_attachment_model(&default_attachment_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_title() {
        let model = AttachmentModel::new("", "2.4 MB");
        assert!(validate_attachment_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_meta() {
        let model = AttachmentModel::new("roadmap-notes.pdf", "");
        assert!(validate_attachment_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_action_contract() {
        let model = AttachmentModel::new("roadmap-notes.pdf", "2.4 MB")
            .with_action(AttachmentAction::new("", ""));
        assert!(validate_attachment_model(&model).is_err());
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let nodes = attachment_render_nodes(&default_attachment_model());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(AttachmentPart::Root)
        );
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AttachmentPart::Preview)
        );
        assert!(nodes.iter().any(|node| node.part == AttachmentPart::Title));
        assert!(nodes.iter().any(|node| node.part == AttachmentPart::Meta));
        assert!(nodes.iter().any(|node| node.part == AttachmentPart::Action));
    }

    #[test]
    fn loading_attachment_disables_action_node() {
        let model = default_attachment_model().loading();
        let nodes = attachment_render_nodes(&model);
        assert!(
            nodes
                .iter()
                .any(|node| node.part == AttachmentPart::Action && node.disabled)
        );
    }

    #[test]
    fn local_state_tracks_activation() {
        let mut state = AttachmentState::idle();
        assert_eq!(state.activated_value(), None);
        assert_eq!(
            state.apply(AttachmentIntent::Activate("download".to_owned())),
            AttachmentChange::Activated("download".to_owned())
        );
        assert_eq!(state.activated_value(), Some("download"));
        assert_eq!(
            state.apply(AttachmentIntent::Clear),
            AttachmentChange::Cleared
        );
        assert_eq!(state.activated_value(), None);
    }
}
