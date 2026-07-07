use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommandDensity {
    Standard,
    Dense,
}

impl CommandDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommandPart {
    Root,
    Input,
    List,
    Group,
    Item,
    Shortcut,
}

impl CommandPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Input,
        Self::List,
        Self::Group,
        Self::Item,
        Self::Shortcut,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Command",
            Self::Input => "CommandInput",
            Self::List => "CommandList",
            Self::Group => "CommandGroup",
            Self::Item => "CommandItem",
            Self::Shortcut => "CommandShortcut",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CommandItem {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(max = 160))]
    pub detail: String,
    #[garde(custom(validate_optional_command_shortcut))]
    pub shortcut: Option<String>,
    #[garde(skip)]
    pub keywords: Vec<String>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CommandGroup {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 24), dive)]
    pub items: Vec<CommandItem>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CommandModel {
    #[garde(skip)]
    pub density: CommandDensity,
    #[garde(length(min = 1, max = 128))]
    pub placeholder: String,
    #[garde(length(min = 1, max = 160))]
    pub empty_label: String,
    #[garde(length(min = 1, max = 12), dive, custom(command_groups_are_valid))]
    pub groups: Vec<CommandGroup>,
    #[garde(custom(command_value_references_enabled_item(&self.groups, "selected command value must reference an enabled item")))]
    pub selected_value: Option<String>,
    #[garde(custom(command_value_references_enabled_item(&self.groups, "highlighted command value must reference an enabled item")))]
    pub highlighted_value: Option<String>,
    #[garde(length(max = 128))]
    pub default_query: String,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandState {
    open: bool,
    query: String,
    highlighted_value: Option<String>,
    selected_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandIntent {
    Open,
    Close,
    Toggle,
    Input(String),
    Highlight(String),
    Select(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandChange {
    Opened,
    Closed,
    Input(String),
    Highlighted(String),
    Selected(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandRenderNode {
    pub part: CommandPart,
    pub group_index: usize,
    pub item_index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub shortcut: String,
    pub group_value: String,
    pub group_label: String,
    pub density: CommandDensity,
    pub highlighted: bool,
    pub selected: bool,
    pub visible: bool,
    pub open: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandFilteredItem<'a> {
    pub group_index: usize,
    pub item_index: usize,
    pub group: &'a CommandGroup,
    pub item: &'a CommandItem,
}

impl CommandItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            detail: String::new(),
            shortcut: None,
            keywords: Vec::new(),
            disabled: false,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    fn matches_query(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let query = query.to_ascii_lowercase();
        self.label.to_ascii_lowercase().contains(&query)
            || self.value.to_ascii_lowercase().contains(&query)
            || self.detail.to_ascii_lowercase().contains(&query)
            || self
                .shortcut
                .as_ref()
                .is_some_and(|shortcut| shortcut.to_ascii_lowercase().contains(&query))
            || self
                .keywords
                .iter()
                .any(|keyword| keyword.to_ascii_lowercase().contains(&query))
    }
}

impl CommandGroup {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        items: Vec<CommandItem>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            items,
        }
    }
}

impl CommandModel {
    pub fn new(groups: Vec<CommandGroup>) -> Self {
        Self {
            density: CommandDensity::Standard,
            placeholder: "Search commands".to_owned(),
            empty_label: "No command found.".to_owned(),
            groups,
            selected_value: None,
            highlighted_value: None,
            default_query: String::new(),
            default_open: true,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: CommandDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_empty_label(mut self, empty_label: impl Into<String>) -> Self {
        self.empty_label = empty_label.into();
        self
    }

    pub fn with_selected_value(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    pub fn with_highlighted_value(mut self, value: impl Into<String>) -> Self {
        self.highlighted_value = Some(value.into());
        self
    }

    pub fn with_default_query(mut self, query: impl Into<String>) -> Self {
        self.default_query = query.into();
        self
    }

    pub const fn closed(mut self) -> Self {
        self.default_open = false;
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

    pub fn state(&self) -> CommandState {
        CommandState::new(
            self.default_open,
            self.default_query.clone(),
            self.highlighted_value.clone(),
            self.selected_value.clone(),
        )
    }
}

impl CommandState {
    pub fn new(
        open: bool,
        query: String,
        highlighted_value: Option<String>,
        selected_value: Option<String>,
    ) -> Self {
        Self {
            open,
            query,
            highlighted_value,
            selected_value,
        }
    }

    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn highlighted_value(&self) -> Option<&str> {
        self.highlighted_value.as_deref()
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    pub fn is_highlighted(&self, value: &str) -> bool {
        self.highlighted_value.as_deref() == Some(value)
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: CommandIntent) -> CommandChange {
        match intent {
            CommandIntent::Open => self.open(),
            CommandIntent::Close => self.close(),
            CommandIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            CommandIntent::Input(query) => self.input(query),
            CommandIntent::Highlight(value) => self.highlight(value),
            CommandIntent::Select(value) => self.select(value),
            CommandIntent::Clear => self.clear(),
        }
    }

    fn open(&mut self) -> CommandChange {
        if self.open {
            CommandChange::Unchanged
        } else {
            self.open = true;
            CommandChange::Opened
        }
    }

    fn close(&mut self) -> CommandChange {
        if self.open {
            self.open = false;
            CommandChange::Closed
        } else {
            CommandChange::Unchanged
        }
    }

    fn input(&mut self, query: String) -> CommandChange {
        if self.query == query && self.open {
            CommandChange::Unchanged
        } else {
            self.query = query.clone();
            self.highlighted_value = None;
            self.open = true;
            CommandChange::Input(query)
        }
    }

    fn highlight(&mut self, value: String) -> CommandChange {
        if value.is_empty() || self.highlighted_value.as_ref() == Some(&value) {
            return CommandChange::Unchanged;
        }
        self.highlighted_value = Some(value.clone());
        CommandChange::Highlighted(value)
    }

    fn select(&mut self, value: String) -> CommandChange {
        if value.is_empty() || self.selected_value.as_ref() == Some(&value) {
            return CommandChange::Unchanged;
        }
        self.highlighted_value = Some(value.clone());
        self.selected_value = Some(value.clone());
        self.open = false;
        CommandChange::Selected(value)
    }

    fn clear(&mut self) -> CommandChange {
        if self.query.is_empty()
            && self.highlighted_value.is_none()
            && self.selected_value.is_none()
            && !self.open
        {
            return CommandChange::Unchanged;
        }
        self.query.clear();
        self.highlighted_value = None;
        self.selected_value = None;
        self.open = false;
        CommandChange::Cleared
    }
}

pub fn validate_command_model(model: &CommandModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn command_render_nodes(model: &CommandModel, state: &CommandState) -> Vec<CommandRenderNode> {
    let blocked = model.disabled || model.loading;
    let visible_items = filtered_command_items(model, state.query());
    let selected = state.selected_value().unwrap_or("none").to_owned();
    let highlighted = state.highlighted_value().unwrap_or("none").to_owned();
    let mut nodes = Vec::with_capacity(
        visible_items
            .len()
            .saturating_mul(2)
            .saturating_add(model.groups.len())
            .saturating_add(3),
    );
    nodes.push(CommandRenderNode {
        part: CommandPart::Root,
        group_index: 0,
        item_index: 0,
        value: selected.clone(),
        label: "Command".to_owned(),
        detail: if state.is_open() {
            "Command palette open".to_owned()
        } else {
            "Command palette closed".to_owned()
        },
        shortcut: String::new(),
        group_value: String::new(),
        group_label: String::new(),
        density: model.density,
        highlighted: highlighted != "none",
        selected: selected != "none",
        visible: true,
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(CommandRenderNode {
        part: CommandPart::Input,
        group_index: 0,
        item_index: 0,
        value: state.query().to_owned(),
        label: model.placeholder.clone(),
        detail: selected_command_label(model, state)
            .unwrap_or_else(|| "No command selected".to_owned()),
        shortcut: String::new(),
        group_value: String::new(),
        group_label: String::new(),
        density: model.density,
        highlighted: false,
        selected: selected != "none",
        visible: true,
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(CommandRenderNode {
        part: CommandPart::List,
        group_index: 0,
        item_index: 0,
        value: "commands".to_owned(),
        label: if visible_items.is_empty() {
            model.empty_label.clone()
        } else {
            "Command results".to_owned()
        },
        detail: format!("{} visible commands", visible_items.len()),
        shortcut: String::new(),
        group_value: String::new(),
        group_label: String::new(),
        density: model.density,
        highlighted: false,
        selected: false,
        visible: state.is_open(),
        open: state.is_open(),
        loading: model.loading,
        disabled: blocked,
    });

    let mut emitted_groups = HashSet::with_capacity(model.groups.len());
    for visible in visible_items {
        if emitted_groups.insert(visible.group.value.as_str()) {
            let group_count = filtered_command_items_for_group(model, state.query(), visible.group)
                .len()
                .to_string();
            nodes.push(CommandRenderNode {
                part: CommandPart::Group,
                group_index: visible.group_index,
                item_index: 0,
                value: visible.group.value.clone(),
                label: visible.group.label.clone(),
                detail: format!("{group_count} commands"),
                shortcut: String::new(),
                group_value: visible.group.value.clone(),
                group_label: visible.group.label.clone(),
                density: model.density,
                highlighted: false,
                selected: false,
                visible: state.is_open(),
                open: state.is_open(),
                loading: model.loading,
                disabled: blocked,
            });
        }

        let highlighted = state.is_highlighted(&visible.item.value);
        let selected = state.is_selected(&visible.item.value);
        let shortcut = visible.item.shortcut.clone().unwrap_or_default();
        nodes.push(CommandRenderNode {
            part: CommandPart::Item,
            group_index: visible.group_index,
            item_index: visible.item_index,
            value: visible.item.value.clone(),
            label: visible.item.label.clone(),
            detail: visible.item.detail.clone(),
            shortcut: shortcut.clone(),
            group_value: visible.group.value.clone(),
            group_label: visible.group.label.clone(),
            density: model.density,
            highlighted,
            selected,
            visible: state.is_open(),
            open: state.is_open(),
            loading: model.loading,
            disabled: blocked || visible.item.disabled,
        });
        if !shortcut.is_empty() {
            nodes.push(CommandRenderNode {
                part: CommandPart::Shortcut,
                group_index: visible.group_index,
                item_index: visible.item_index,
                value: visible.item.value.clone(),
                label: shortcut,
                detail: visible.item.label.clone(),
                shortcut: String::new(),
                group_value: visible.group.value.clone(),
                group_label: visible.group.label.clone(),
                density: model.density,
                highlighted,
                selected,
                visible: state.is_open(),
                open: state.is_open(),
                loading: model.loading,
                disabled: blocked || visible.item.disabled,
            });
        }
    }

    nodes
}

pub fn filtered_command_items<'a>(
    model: &'a CommandModel,
    query: &str,
) -> Vec<CommandFilteredItem<'a>> {
    model
        .groups
        .iter()
        .enumerate()
        .flat_map(|(group_index, group)| {
            group
                .items
                .iter()
                .enumerate()
                .filter(move |(_, item)| item.matches_query(query))
                .map(move |(item_index, item)| CommandFilteredItem {
                    group_index,
                    item_index,
                    group,
                    item,
                })
        })
        .collect()
}

pub fn selected_command_label(model: &CommandModel, state: &CommandState) -> Option<String> {
    state.selected_value().and_then(|selected| {
        model
            .groups
            .iter()
            .flat_map(|group| group.items.iter())
            .find(|item| item.value == selected)
            .map(|item| item.label.clone())
    })
}

pub fn default_command_model() -> CommandModel {
    CommandModel::new(vec![
        CommandGroup::new(
            "Build",
            "build",
            vec![
                CommandItem::new("Run gate", "gate")
                    .with_detail("Run the one-pass workspace gate.")
                    .with_shortcut("G")
                    .with_keywords(vec!["check".to_owned(), "ci".to_owned()]),
                CommandItem::new("Open stories", "stories")
                    .with_detail("Launch the reusable UI story harness.")
                    .with_shortcut("S")
                    .with_keywords(vec!["ui".to_owned(), "gallery".to_owned()]),
            ],
        ),
        CommandGroup::new(
            "Frameworks",
            "frameworks",
            vec![
                CommandItem::new("Leptos surface", "leptos")
                    .with_detail("Render command UI in the DOM.")
                    .with_shortcut("L"),
                CommandItem::new("Bevy primitive", "bevy")
                    .with_detail("Project the same command model into scene primitives.")
                    .with_shortcut("B"),
            ],
        ),
    ])
    .with_selected_value("gate")
    .with_highlighted_value("stories")
}

fn filtered_command_items_for_group<'a>(
    model: &'a CommandModel,
    query: &str,
    group: &'a CommandGroup,
) -> Vec<CommandFilteredItem<'a>> {
    filtered_command_items(model, query)
        .into_iter()
        .filter(|visible| visible.group.value == group.value)
        .collect()
}

fn validate_optional_command_shortcut(shortcut: &Option<String>, _context: &()) -> garde::Result {
    if let Some(shortcut) = shortcut
        && !(1..=32).contains(&shortcut.chars().count())
    {
        return Err(garde::Error::new(
            "command shortcut must be 1 to 32 characters",
        ));
    }
    Ok(())
}

fn command_groups_are_valid(groups: &Vec<CommandGroup>, _context: &()) -> garde::Result {
    let mut group_values = HashSet::with_capacity(groups.len());
    let mut item_values = HashSet::new();
    for group in groups {
        if !group_values.insert(group.value.as_str()) {
            return Err(garde::Error::new("command group values must be unique"));
        }
        for item in &group.items {
            if !item_values.insert(item.value.as_str()) {
                return Err(garde::Error::new("command item values must be unique"));
            }
            if item.keywords.len() > 8 {
                return Err(garde::Error::new(
                    "command items may include at most 8 keywords",
                ));
            }
            if item
                .keywords
                .iter()
                .any(|keyword| !(1..=64).contains(&keyword.chars().count()))
            {
                return Err(garde::Error::new(
                    "command item keywords must be 1 to 64 characters",
                ));
            }
        }
    }
    Ok(())
}

fn command_value_references_enabled_item<'a>(
    groups: &'a [CommandGroup],
    message: &'static str,
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !groups
                .iter()
                .flat_map(|group| group.items.iter())
                .any(|item| item.value == *value && !item.disabled)
        {
            return Err(garde::Error::new(message));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_command_model(&default_command_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_groups() {
        let model = CommandModel::new(Vec::new());
        assert!(validate_command_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_group_values() {
        let model = CommandModel::new(vec![
            CommandGroup::new("Build", "same", vec![CommandItem::new("Run gate", "gate")]),
            CommandGroup::new(
                "Browse",
                "same",
                vec![CommandItem::new("Open stories", "stories")],
            ),
        ]);
        assert!(validate_command_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_item_values() {
        let model = CommandModel::new(vec![CommandGroup::new(
            "Build",
            "build",
            vec![
                CommandItem::new("Run gate", "same"),
                CommandItem::new("Open stories", "same"),
            ],
        )]);
        assert!(validate_command_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_item_label() {
        let model = CommandModel::new(vec![CommandGroup::new(
            "Build",
            "build",
            vec![CommandItem::new("", "gate")],
        )]);
        assert!(validate_command_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_shortcut() {
        let model = CommandModel::new(vec![CommandGroup::new(
            "Build",
            "build",
            vec![CommandItem::new("Run gate", "gate").with_shortcut("")],
        )]);
        assert!(validate_command_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_value() {
        let model = default_command_model().with_selected_value("unknown");
        assert!(validate_command_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_highlighted_value() {
        let model = CommandModel::new(vec![CommandGroup::new(
            "Build",
            "build",
            vec![CommandItem::new("Run gate", "gate").disabled()],
        )])
        .with_highlighted_value("gate");
        assert!(validate_command_model(&model).is_err());
    }

    #[test]
    fn state_filters_highlights_selects_and_clears() {
        let mut state = CommandState::new(false, String::new(), None, None);
        assert_eq!(
            state.apply(CommandIntent::Input("sto".to_owned())),
            CommandChange::Input("sto".to_owned())
        );
        assert!(state.is_open());
        assert_eq!(state.query(), "sto");
        assert_eq!(
            state.apply(CommandIntent::Highlight("stories".to_owned())),
            CommandChange::Highlighted("stories".to_owned())
        );
        assert_eq!(state.highlighted_value(), Some("stories"));
        assert_eq!(
            state.apply(CommandIntent::Select("stories".to_owned())),
            CommandChange::Selected("stories".to_owned())
        );
        assert_eq!(state.selected_value(), Some("stories"));
        assert!(!state.is_open());
        assert_eq!(state.apply(CommandIntent::Clear), CommandChange::Cleared);
        assert_eq!(state.selected_value(), None);
        assert_eq!(state.highlighted_value(), None);
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_command_model();
        let nodes = command_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(CommandPart::Root));
        for part in CommandPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == CommandPart::Group)
                .count(),
            2
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == CommandPart::Item)
                .count(),
            4
        );
    }

    #[test]
    fn no_matches_marks_list_as_empty() {
        let model = default_command_model().with_default_query("zzzz");
        let state = model.state();
        let nodes = command_render_nodes(&model, &state);
        assert!(nodes.iter().any(|node| {
            node.part == CommandPart::List && node.visible && node.label == model.empty_label
        }));
        assert!(
            !nodes
                .iter()
                .any(|node| node.part == CommandPart::Item && node.visible)
        );
    }

    #[test]
    fn loading_disables_item_nodes() {
        let model = default_command_model().loading();
        let nodes = command_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == CommandPart::Item && node.disabled)
        );
    }
}
