use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::scale;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectDensity {
    Standard,
    Dense,
}

impl SelectDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectPart {
    Root,
    Trigger,
    Value,
    Content,
    Item,
    Group,
}

impl SelectPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Trigger,
        Self::Value,
        Self::Content,
        Self::Item,
        Self::Group,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Select",
            Self::Trigger => "SelectTrigger",
            Self::Value => "SelectValue",
            Self::Content => "SelectContent",
            Self::Item => "SelectItem",
            Self::Group => "SelectGroup",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SelectOption {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SelectGroup {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 12), dive)]
    pub options: Vec<SelectOption>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SelectModel {
    #[garde(skip)]
    pub density: SelectDensity,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 160))]
    pub placeholder: String,
    #[garde(length(min = 1, max = 8), dive, custom(select_groups_are_valid))]
    pub groups: Vec<SelectGroup>,
    #[garde(custom(select_selected_value_references_enabled_option(&self.groups)))]
    pub selected_value: Option<String>,
    #[garde(custom(validate_optional_select_error))]
    pub error: Option<String>,
    #[garde(skip)]
    pub required: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectState {
    open: bool,
    focused_value: Option<String>,
    selected_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectIntent {
    Open,
    Close,
    Toggle,
    Focus(String),
    Select(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectChange {
    Opened,
    Closed,
    Focused(String),
    Selected(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectRenderNode {
    pub part: SelectPart,
    pub index: usize,
    pub group_index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub density: SelectDensity,
    pub open: bool,
    pub selected: bool,
    pub focused: bool,
    pub visible: bool,
    pub actionable: bool,
    pub invalid: bool,
    pub required: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectLayoutMetrics {
    pub max_width: f32,
    pub root_gap: f32,
    pub label_font_size: f32,
    pub label_tracking_em: f32,
    pub trigger_min_height: f32,
    pub trigger_padding_inline: f32,
    pub trigger_padding_block: f32,
    pub trigger_gap: f32,
    pub trigger_font_size: f32,
    pub chevron_font_size: f32,
    pub content_margin_top: f32,
    pub content_padding: f32,
    pub content_gap: f32,
    pub group_gap: f32,
    pub group_padding_top: f32,
    pub item_padding: f32,
    pub emphasized_item_padding: f32,
    pub item_gap: f32,
    pub item_title_font_size: f32,
    pub disabled_item_title_font_size: f32,
    pub item_detail_font_size: f32,
    pub error_font_size: f32,
    pub line_height: f32,
}

impl SelectLayoutMetrics {
    pub fn trigger_outer_height(self, border_width: f32) -> f32 {
        self.trigger_min_height.max(
            self.trigger_font_size * self.line_height
                + self.trigger_padding_block * 2.0
                + border_width.max(0.0) * 2.0,
        )
    }

    pub fn content_offset(self, border_width: f32) -> f32 {
        self.label_font_size * self.line_height
            + self.root_gap
            + self.trigger_outer_height(border_width)
            + self.content_margin_top
    }
}

impl SelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            detail: format!("{label} option"),
            label,
            value: value.into(),
            disabled: false,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl SelectGroup {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        options: Vec<SelectOption>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            options,
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl SelectModel {
    pub fn new(groups: Vec<SelectGroup>) -> Self {
        Self {
            density: SelectDensity::Standard,
            label: "Renderer".to_owned(),
            placeholder: "Choose renderer".to_owned(),
            groups,
            selected_value: None,
            error: None,
            required: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: SelectDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_selected_value(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    pub fn without_selected_value(mut self) -> Self {
        self.selected_value = None;
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

    pub const fn required(mut self) -> Self {
        self.required = true;
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

    pub fn state(&self) -> SelectState {
        SelectState::new(self.selected_value.clone())
    }
}

impl SelectState {
    pub fn new(selected_value: Option<String>) -> Self {
        Self {
            open: false,
            focused_value: None,
            selected_value,
        }
    }

    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub fn focused_value(&self) -> Option<&str> {
        self.focused_value.as_deref()
    }

    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    pub fn is_focused(&self, value: &str) -> bool {
        self.focused_value.as_deref() == Some(value)
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: SelectIntent) -> SelectChange {
        match intent {
            SelectIntent::Open => self.open(),
            SelectIntent::Close => self.close(),
            SelectIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            SelectIntent::Focus(value) => self.focus(value),
            SelectIntent::Select(value) => self.select(value),
            SelectIntent::Clear => self.clear(),
        }
    }

    fn open(&mut self) -> SelectChange {
        if self.open {
            SelectChange::Unchanged
        } else {
            self.open = true;
            SelectChange::Opened
        }
    }

    fn close(&mut self) -> SelectChange {
        if self.open || self.focused_value.is_some() {
            self.open = false;
            self.focused_value = None;
            SelectChange::Closed
        } else {
            SelectChange::Unchanged
        }
    }

    fn focus(&mut self, value: String) -> SelectChange {
        if value.is_empty() || self.focused_value.as_ref() == Some(&value) {
            return SelectChange::Unchanged;
        }
        self.open = true;
        self.focused_value = Some(value.clone());
        SelectChange::Focused(value)
    }

    fn select(&mut self, value: String) -> SelectChange {
        if value.is_empty() || self.selected_value.as_ref() == Some(&value) {
            self.open = false;
            self.focused_value = None;
            return SelectChange::Unchanged;
        }
        self.open = false;
        self.focused_value = None;
        self.selected_value = Some(value.clone());
        SelectChange::Selected(value)
    }

    fn clear(&mut self) -> SelectChange {
        if !self.open && self.focused_value.is_none() && self.selected_value.is_none() {
            return SelectChange::Unchanged;
        }
        self.open = false;
        self.focused_value = None;
        self.selected_value = None;
        SelectChange::Cleared
    }
}

pub fn validate_select_model(model: &SelectModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn select_layout_metrics(model: &SelectModel, inline_size: f32) -> SelectLayoutMetrics {
    select_layout_metrics_for_density(model.density, inline_size)
}

pub fn select_trigger_layout_metrics(
    model: &SelectModel,
    open: bool,
    inline_size: f32,
) -> SelectLayoutMetrics {
    let density = if select_trigger_uses_standard_metrics(
        open,
        model.error.is_some(),
        model.loading || model.disabled,
    ) {
        SelectDensity::Standard
    } else {
        model.density
    };
    select_layout_metrics_for_density(density, inline_size)
}

fn select_layout_metrics_for_density(
    density: SelectDensity,
    inline_size: f32,
) -> SelectLayoutMetrics {
    let dense = density == SelectDensity::Dense;
    SelectLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_gap: scale::space::xs2(inline_size),
        label_font_size: scale::font_size::f00(inline_size),
        label_tracking_em: 0.08,
        trigger_min_height: if dense {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        trigger_padding_inline: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        trigger_padding_block: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        trigger_gap: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        trigger_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        chevron_font_size: scale::font_size::f00(inline_size),
        content_margin_top: scale::space::xs2(inline_size),
        content_padding: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        content_gap: if dense {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        group_gap: scale::space::xs3(inline_size),
        group_padding_top: scale::space::xs2(inline_size),
        item_padding: if dense {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        emphasized_item_padding: scale::space::xs(inline_size),
        item_gap: scale::space::xs3(inline_size),
        item_title_font_size: if dense {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        disabled_item_title_font_size: scale::font_size::f0(inline_size),
        item_detail_font_size: scale::font_size::f00(inline_size),
        error_font_size: scale::font_size::f00(inline_size),
        line_height: scale::line_height::LH0,
    }
}

pub const fn select_trigger_uses_standard_metrics(
    open: bool,
    invalid: bool,
    blocked: bool,
) -> bool {
    open || invalid || blocked
}

pub const fn select_item_uses_emphasized_metrics(
    selected: bool,
    focused: bool,
    disabled: bool,
) -> bool {
    selected || focused || disabled
}

pub fn select_render_nodes(model: &SelectModel, state: &SelectState) -> Vec<SelectRenderNode> {
    let invalid = model.error.is_some();
    let blocked = model.loading || model.disabled;
    let mut item_count = 0usize;
    let mut nodes = Vec::with_capacity(
        model
            .groups
            .iter()
            .map(|group| group.options.len())
            .sum::<usize>()
            + model.groups.len()
            + 4,
    );
    nodes.push(SelectRenderNode {
        part: SelectPart::Root,
        index: 0,
        group_index: 0,
        value: state.selected_value().unwrap_or("none").to_owned(),
        label: model.label.clone(),
        detail: model
            .error
            .clone()
            .unwrap_or_else(|| selected_select_label(model, state)),
        density: model.density,
        open: state.is_open(),
        selected: state.selected_value().is_some(),
        focused: state.focused_value().is_some(),
        visible: true,
        actionable: false,
        invalid,
        required: model.required,
        loading: model.loading,
        disabled: model.disabled,
    });
    nodes.push(SelectRenderNode {
        part: SelectPart::Trigger,
        index: 0,
        group_index: 0,
        value: state.selected_value().unwrap_or("trigger").to_owned(),
        label: model.label.clone(),
        detail: "Open select options".to_owned(),
        density: model.density,
        open: state.is_open(),
        selected: state.selected_value().is_some(),
        focused: false,
        visible: true,
        actionable: !blocked,
        invalid,
        required: model.required,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(SelectRenderNode {
        part: SelectPart::Value,
        index: 0,
        group_index: 0,
        value: state.selected_value().unwrap_or("placeholder").to_owned(),
        label: selected_select_label(model, state),
        detail: model.placeholder.clone(),
        density: model.density,
        open: state.is_open(),
        selected: state.selected_value().is_some(),
        focused: false,
        visible: true,
        actionable: false,
        invalid,
        required: model.required,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(SelectRenderNode {
        part: SelectPart::Content,
        index: 0,
        group_index: 0,
        value: "content".to_owned(),
        label: "Select content".to_owned(),
        detail: "Grouped listbox options".to_owned(),
        density: model.density,
        open: state.is_open(),
        selected: false,
        focused: state.focused_value().is_some(),
        visible: state.is_open(),
        actionable: false,
        invalid,
        required: model.required,
        loading: model.loading,
        disabled: blocked,
    });

    for (group_index, group) in model.groups.iter().enumerate() {
        let group_disabled = blocked || group.disabled;
        nodes.push(SelectRenderNode {
            part: SelectPart::Group,
            index: group_index,
            group_index,
            value: group.value.clone(),
            label: group.label.clone(),
            detail: format!("{} options", group.options.len()),
            density: model.density,
            open: state.is_open(),
            selected: false,
            focused: false,
            visible: state.is_open(),
            actionable: false,
            invalid,
            required: model.required,
            loading: model.loading,
            disabled: group_disabled,
        });
        for option in &group.options {
            let option_disabled = group_disabled || option.disabled;
            nodes.push(SelectRenderNode {
                part: SelectPart::Item,
                index: item_count,
                group_index,
                value: option.value.clone(),
                label: option.label.clone(),
                detail: option.detail.clone(),
                density: model.density,
                open: state.is_open(),
                selected: state.is_selected(&option.value),
                focused: state.is_focused(&option.value),
                visible: state.is_open(),
                actionable: !option_disabled,
                invalid,
                required: model.required,
                loading: model.loading,
                disabled: option_disabled,
            });
            item_count += 1;
        }
    }
    nodes
}

pub fn default_select_model() -> SelectModel {
    SelectModel::new(default_select_groups())
        .with_label("Renderer")
        .with_selected_value("leptos")
}

pub fn default_select_groups() -> Vec<SelectGroup> {
    vec![
        SelectGroup::new(
            "Application surfaces",
            "apps",
            vec![
                SelectOption::new("Leptos DOM", "leptos")
                    .with_detail("Render interactive DOM controls."),
                SelectOption::new("Bevy WebGPU", "bevy")
                    .with_detail("Render scene primitives from the same contract."),
            ],
        ),
        SelectGroup::new(
            "Shared contracts",
            "shared",
            vec![
                SelectOption::new("Design tokens", "tokens")
                    .with_detail("Use semantic theme tokens across renderers."),
            ],
        ),
    ]
}

pub fn selected_select_label(model: &SelectModel, state: &SelectState) -> String {
    state
        .selected_value()
        .and_then(|value| {
            model
                .groups
                .iter()
                .flat_map(|group| group.options.iter())
                .find(|option| option.value == value)
                .map(|option| option.label.clone())
        })
        .unwrap_or_else(|| model.placeholder.clone())
}

fn select_groups_are_valid(groups: &Vec<SelectGroup>, _context: &()) -> garde::Result {
    let mut group_values = HashSet::with_capacity(groups.len());
    let mut option_values = HashSet::new();
    for group in groups {
        if !group_values.insert(group.value.as_str()) {
            return Err(garde::Error::new("select group values must be unique"));
        }
        for option in &group.options {
            if !option_values.insert(option.value.as_str()) {
                return Err(garde::Error::new("select option values must be unique"));
            }
        }
    }
    Ok(())
}

fn select_selected_value_references_enabled_option<'a>(
    groups: &'a [SelectGroup],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |value, _context| {
        if let Some(value) = value
            && !groups.iter().any(|group| {
                !group.disabled
                    && group
                        .options
                        .iter()
                        .any(|option| option.value == *value && !option.disabled)
            })
        {
            return Err(garde::Error::new(
                "selected select value must reference an enabled option",
            ));
        }
        Ok(())
    }
}

fn validate_optional_select_error(error: &Option<String>, _context: &()) -> garde::Result {
    if let Some(error) = error
        && !(1..=240).contains(&error.chars().count())
    {
        return Err(garde::Error::new(
            "select error must be 1 to 240 characters",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_select_model(&default_select_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_groups() {
        let model = SelectModel::new(Vec::new());
        assert!(validate_select_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_option_values() {
        let model = SelectModel::new(vec![SelectGroup::new(
            "Group",
            "group",
            vec![
                SelectOption::new("A", "same"),
                SelectOption::new("B", "same"),
            ],
        )]);
        assert!(validate_select_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_disabled_selected_value() {
        let model = SelectModel::new(vec![SelectGroup::new(
            "Group",
            "group",
            vec![SelectOption::new("A", "a").disabled()],
        )])
        .with_selected_value("a");
        assert!(validate_select_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_select_model().with_error("");
        assert!(validate_select_model(&model).is_err());
    }

    #[test]
    fn state_opens_focuses_selects_and_clears() {
        let mut state = SelectState::new(None);
        assert_eq!(state.apply(SelectIntent::Open), SelectChange::Opened);
        assert!(state.is_open());
        assert_eq!(
            state.apply(SelectIntent::Focus("bevy".to_owned())),
            SelectChange::Focused("bevy".to_owned())
        );
        assert!(state.is_focused("bevy"));
        assert_eq!(
            state.apply(SelectIntent::Select("bevy".to_owned())),
            SelectChange::Selected("bevy".to_owned())
        );
        assert!(state.is_selected("bevy"));
        assert!(!state.is_open());
        assert_eq!(state.apply(SelectIntent::Clear), SelectChange::Cleared);
        assert!(state.selected_value().is_none());
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_select_model();
        let nodes = select_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(SelectPart::Root));
        for part in SelectPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn closed_state_hides_content_groups_and_items() {
        let model = default_select_model();
        let nodes = select_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| {
                    matches!(
                        node.part,
                        SelectPart::Content | SelectPart::Group | SelectPart::Item
                    )
                })
                .all(|node| !node.visible)
        );
    }

    #[test]
    fn selected_label_uses_selected_option_or_placeholder() {
        let model = default_select_model().with_selected_value("tokens");
        assert_eq!(
            selected_select_label(&model, &model.state()),
            "Design tokens"
        );
        let model = default_select_model().without_selected_value();
        assert_eq!(
            selected_select_label(&model, &model.state()),
            "Choose renderer"
        );
    }

    #[test]
    fn layout_metrics_preserve_token_width_and_density() {
        let inline_size = 448.0;
        let standard = select_layout_metrics(&default_select_model(), inline_size);
        let dense = select_layout_metrics(
            &default_select_model().with_density(SelectDensity::Dense),
            inline_size,
        );

        assert_eq!(standard.max_width, scale::container::CONTROL);
        assert_eq!(standard.trigger_min_height, scale::space::FIELD);
        assert!(dense.trigger_min_height < standard.trigger_min_height);
        assert!(dense.trigger_padding_inline < standard.trigger_padding_inline);
        assert!(dense.item_padding < standard.item_padding);
        assert_eq!(dense.emphasized_item_padding, standard.item_padding);
    }

    #[test]
    fn emphasized_trigger_states_use_standard_metrics() {
        let inline_size = 448.0;
        let dense = default_select_model().with_density(SelectDensity::Dense);
        let closed = select_trigger_layout_metrics(&dense, false, inline_size);
        let open = select_trigger_layout_metrics(&dense, true, inline_size);
        let invalid = select_trigger_layout_metrics(
            &dense.clone().with_error("Choose a renderer"),
            false,
            inline_size,
        );
        let blocked = select_trigger_layout_metrics(&dense.disabled(), false, inline_size);

        assert!(closed.trigger_min_height < open.trigger_min_height);
        assert_eq!(open.trigger_min_height, scale::space::FIELD);
        assert_eq!(invalid.trigger_min_height, open.trigger_min_height);
        assert_eq!(blocked.trigger_min_height, open.trigger_min_height);
        assert!(select_trigger_uses_standard_metrics(true, false, false));
        assert!(select_trigger_uses_standard_metrics(false, true, false));
        assert!(select_trigger_uses_standard_metrics(false, false, true));
    }

    #[test]
    fn emphasized_item_metrics_follow_shared_state_flags() {
        assert!(!select_item_uses_emphasized_metrics(false, false, false));
        assert!(select_item_uses_emphasized_metrics(true, false, false));
        assert!(select_item_uses_emphasized_metrics(false, true, false));
        assert!(select_item_uses_emphasized_metrics(false, false, true));
    }
}
