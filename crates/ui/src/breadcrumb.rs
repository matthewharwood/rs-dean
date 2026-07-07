use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BreadcrumbDensity {
    Standard,
    Dense,
}

impl BreadcrumbDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

impl BreadcrumbPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Breadcrumb",
            Self::List => "BreadcrumbList",
            Self::Item => "BreadcrumbItem",
            Self::Link => "BreadcrumbLink",
            Self::Separator => "BreadcrumbSeparator",
            Self::Page => "BreadcrumbPage",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BreadcrumbEntry {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(custom(validate_optional_breadcrumb_href))]
    pub href: Option<String>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BreadcrumbModel {
    #[garde(skip)]
    pub density: BreadcrumbDensity,
    #[garde(length(min = 1, max = 12), dive)]
    pub entries: Vec<BreadcrumbEntry>,
    #[garde(length(min = 1, max = 8))]
    pub separator: String,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreadcrumbState {
    active_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreadcrumbIntent {
    Focus(String),
    Navigate(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreadcrumbChange {
    Focused(String),
    Navigated(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreadcrumbPart {
    Root,
    List,
    Item,
    Link,
    Separator,
    Page,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreadcrumbRenderNode {
    pub part: BreadcrumbPart,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub index: usize,
    pub current: bool,
    pub density: BreadcrumbDensity,
    pub active: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl BreadcrumbEntry {
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
            disabled: false,
        }
    }

    pub fn page(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn value(&self) -> String {
        self.href.clone().unwrap_or_else(|| self.label.clone())
    }
}

impl BreadcrumbModel {
    pub fn new(entries: Vec<BreadcrumbEntry>) -> Self {
        Self {
            density: BreadcrumbDensity::Standard,
            entries,
            separator: "/".to_owned(),
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: BreadcrumbDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
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

    pub const fn state(&self) -> BreadcrumbState {
        let _ = self;
        BreadcrumbState::idle()
    }
}

impl BreadcrumbState {
    pub const fn idle() -> Self {
        Self { active_value: None }
    }

    pub fn active_value(&self) -> Option<&str> {
        self.active_value.as_deref()
    }

    pub fn is_active(&self, value: &str) -> bool {
        self.active_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: BreadcrumbIntent) -> BreadcrumbChange {
        match intent {
            BreadcrumbIntent::Focus(value) => self.focus(value),
            BreadcrumbIntent::Navigate(value) => self.navigate(value),
            BreadcrumbIntent::Clear => self.clear(),
        }
    }

    fn focus(&mut self, value: String) -> BreadcrumbChange {
        if value.is_empty() || self.active_value.as_ref() == Some(&value) {
            BreadcrumbChange::Unchanged
        } else {
            self.active_value = Some(value.clone());
            BreadcrumbChange::Focused(value)
        }
    }

    fn navigate(&mut self, value: String) -> BreadcrumbChange {
        if value.is_empty() {
            BreadcrumbChange::Unchanged
        } else {
            self.active_value = Some(value.clone());
            BreadcrumbChange::Navigated(value)
        }
    }

    fn clear(&mut self) -> BreadcrumbChange {
        if self.active_value.is_some() {
            self.active_value = None;
            BreadcrumbChange::Cleared
        } else {
            BreadcrumbChange::Unchanged
        }
    }
}

pub fn validate_breadcrumb_model(model: &BreadcrumbModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn breadcrumb_render_nodes(
    model: &BreadcrumbModel,
    state: &BreadcrumbState,
) -> Vec<BreadcrumbRenderNode> {
    let mut nodes = Vec::with_capacity(2 + model.entries.len().saturating_mul(3));
    let root_disabled = model.disabled || model.loading;
    nodes.push(BreadcrumbRenderNode {
        part: BreadcrumbPart::Root,
        value: "breadcrumb".to_owned(),
        label: "Breadcrumb".to_owned(),
        detail: "Route trail".to_owned(),
        index: 0,
        current: false,
        density: model.density,
        active: false,
        loading: model.loading,
        disabled: root_disabled,
    });
    nodes.push(BreadcrumbRenderNode {
        part: BreadcrumbPart::List,
        value: "breadcrumb-list".to_owned(),
        label: "Location trail".to_owned(),
        detail: "Ancestor list".to_owned(),
        index: 0,
        current: false,
        density: model.density,
        active: false,
        loading: model.loading,
        disabled: root_disabled,
    });

    let last_index = model.entries.len().saturating_sub(1);
    for (index, entry) in model.entries.iter().enumerate() {
        let current = index == last_index;
        let value = entry.value();
        let item_disabled = root_disabled || entry.disabled;
        let active = state.is_active(&value);
        nodes.push(BreadcrumbRenderNode {
            part: BreadcrumbPart::Item,
            value: value.clone(),
            label: entry.label.clone(),
            detail: "Trail item".to_owned(),
            index,
            current,
            density: model.density,
            active,
            loading: model.loading,
            disabled: item_disabled,
        });

        if current {
            nodes.push(BreadcrumbRenderNode {
                part: BreadcrumbPart::Page,
                value,
                label: entry.label.clone(),
                detail: "Current page".to_owned(),
                index,
                current: true,
                density: model.density,
                active,
                loading: model.loading,
                disabled: item_disabled,
            });
        } else {
            nodes.push(BreadcrumbRenderNode {
                part: BreadcrumbPart::Link,
                value,
                label: entry.label.clone(),
                detail: "Ancestor link".to_owned(),
                index,
                current: false,
                density: model.density,
                active,
                loading: model.loading,
                disabled: item_disabled || entry.href.is_none(),
            });
            nodes.push(BreadcrumbRenderNode {
                part: BreadcrumbPart::Separator,
                value: model.separator.clone(),
                label: model.separator.clone(),
                detail: "Separator".to_owned(),
                index,
                current: false,
                density: model.density,
                active: false,
                loading: model.loading,
                disabled: root_disabled,
            });
        }
    }

    nodes
}

pub fn default_breadcrumb_model() -> BreadcrumbModel {
    BreadcrumbModel::new(vec![
        BreadcrumbEntry::link("Library", "#library"),
        BreadcrumbEntry::link("Components", "#components"),
        BreadcrumbEntry::page("Breadcrumb"),
    ])
}

fn validate_optional_breadcrumb_href(href: &Option<String>, _context: &()) -> garde::Result {
    if let Some(href) = href
        && !(1..=512).contains(&href.chars().count())
    {
        return Err(garde::Error::new(
            "breadcrumb href must be 1 to 512 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_breadcrumb_model(&default_breadcrumb_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_entries() {
        let model = BreadcrumbModel::new(Vec::new());
        assert!(validate_breadcrumb_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = BreadcrumbModel::new(vec![BreadcrumbEntry::page("")]);
        assert!(validate_breadcrumb_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_href() {
        let model = BreadcrumbModel::new(vec![BreadcrumbEntry::link("Library", "")]);
        assert!(validate_breadcrumb_model(&model).is_err());
    }

    #[test]
    fn local_state_tracks_focus_and_navigation() {
        let mut state = BreadcrumbState::idle();
        assert_eq!(state.active_value(), None);
        assert_eq!(
            state.apply(BreadcrumbIntent::Focus("#library".to_owned())),
            BreadcrumbChange::Focused("#library".to_owned())
        );
        assert_eq!(state.active_value(), Some("#library"));
        assert_eq!(
            state.apply(BreadcrumbIntent::Navigate("#components".to_owned())),
            BreadcrumbChange::Navigated("#components".to_owned())
        );
        assert_eq!(state.active_value(), Some("#components"));
        assert_eq!(
            state.apply(BreadcrumbIntent::Clear),
            BreadcrumbChange::Cleared
        );
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_breadcrumb_model();
        let state = model.state();
        let nodes = breadcrumb_render_nodes(&model, &state);
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(BreadcrumbPart::Root)
        );
        for part in [
            BreadcrumbPart::List,
            BreadcrumbPart::Item,
            BreadcrumbPart::Link,
            BreadcrumbPart::Separator,
            BreadcrumbPart::Page,
        ] {
            assert!(
                nodes.iter().any(|node| node.part == part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == BreadcrumbPart::Item)
                .count(),
            3
        );
    }

    #[test]
    fn loading_disables_navigation_nodes() {
        let model = default_breadcrumb_model().loading();
        let state = model.state();
        let nodes = breadcrumb_render_nodes(&model, &state);
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == BreadcrumbPart::Link)
                .all(|node| node.disabled)
        );
    }
}
