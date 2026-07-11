use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{
    CalendarDate, CalendarIntent, CalendarLayoutMetrics, CalendarModel, CalendarPart,
    calendar_layout_metrics, calendar_render_nodes, month_name, scale,
};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DatePickerDensity {
    Standard,
    Dense,
}

impl DatePickerDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DatePickerPart {
    Root,
    Trigger,
    Popover,
    Calendar,
    Value,
}

impl DatePickerPart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Trigger,
        Self::Popover,
        Self::Calendar,
        Self::Value,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "DatePicker",
            Self::Trigger => "DatePickerTrigger",
            Self::Popover => "DatePickerPopover",
            Self::Calendar => "DatePickerCalendar",
            Self::Value => "DatePickerValue",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DatePickerModel {
    #[garde(skip)]
    pub density: DatePickerDensity,
    #[garde(length(min = 1, max = 128))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub placeholder: String,
    #[garde(range(min = 1900, max = 2200))]
    pub visible_year: u16,
    #[garde(range(min = 1, max = 12))]
    pub visible_month: u8,
    #[garde(dive)]
    pub selected: Option<CalendarDate>,
    #[garde(skip)]
    pub default_open: bool,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatePickerState {
    open: bool,
    focused: bool,
    visible_year: u16,
    visible_month: u8,
    selected: Option<CalendarDate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatePickerIntent {
    Open,
    Close,
    Toggle,
    Focus,
    Blur,
    PreviousMonth,
    NextMonth,
    Select(CalendarDate),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatePickerChange {
    Opened,
    Closed,
    Focused,
    Blurred,
    Moved { year: u16, month: u8 },
    Selected(CalendarDate),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatePickerRenderNode {
    pub part: DatePickerPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub date: Option<CalendarDate>,
    pub density: DatePickerDensity,
    pub open: bool,
    pub focused: bool,
    pub current_month: bool,
    pub selected: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DatePickerLayoutMetrics {
    pub max_width: f32,
    pub root_padding: f32,
    pub root_gap: f32,
    pub trigger_min_height: f32,
    pub trigger_padding_inline: f32,
    pub trigger_padding_block: f32,
    pub trigger_gap: f32,
    pub trigger_font_size: f32,
    pub trigger_line_height: f32,
    pub marker_size: f32,
    pub marker_font_size: f32,
    pub popover_padding: f32,
    pub popover_gap: f32,
    pub clear_min_height: f32,
    pub clear_padding_inline: f32,
    pub clear_padding_block: f32,
    pub clear_font_size: f32,
    pub clear_line_height: f32,
    pub calendar: CalendarLayoutMetrics,
}

impl DatePickerModel {
    pub fn new(visible_year: u16, visible_month: u8) -> Self {
        Self {
            density: DatePickerDensity::Standard,
            label: "Pick a date".to_owned(),
            placeholder: "Select date".to_owned(),
            visible_year,
            visible_month,
            selected: None,
            default_open: false,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: DatePickerDensity) -> Self {
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

    pub const fn with_selected(mut self, selected: CalendarDate) -> Self {
        self.visible_year = selected.year;
        self.visible_month = selected.month;
        self.selected = Some(selected);
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

    pub const fn state(&self) -> DatePickerState {
        DatePickerState {
            open: self.default_open,
            focused: false,
            visible_year: self.visible_year,
            visible_month: self.visible_month,
            selected: self.selected,
        }
    }
}

impl DatePickerState {
    pub const fn is_open(&self) -> bool {
        self.open
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub const fn visible_year(&self) -> u16 {
        self.visible_year
    }

    pub const fn visible_month(&self) -> u8 {
        self.visible_month
    }

    pub const fn selected(&self) -> Option<CalendarDate> {
        self.selected
    }

    pub fn apply(&mut self, intent: DatePickerIntent) -> DatePickerChange {
        match intent {
            DatePickerIntent::Open => self.open(),
            DatePickerIntent::Close => self.close(),
            DatePickerIntent::Toggle => {
                if self.open {
                    self.close()
                } else {
                    self.open()
                }
            }
            DatePickerIntent::Focus => self.focus(),
            DatePickerIntent::Blur => self.blur(),
            DatePickerIntent::PreviousMonth => self.move_month(CalendarIntent::PreviousMonth),
            DatePickerIntent::NextMonth => self.move_month(CalendarIntent::NextMonth),
            DatePickerIntent::Select(date) => self.select(date),
            DatePickerIntent::Clear => self.clear(),
        }
    }

    fn open(&mut self) -> DatePickerChange {
        if self.open {
            DatePickerChange::Unchanged
        } else {
            self.open = true;
            DatePickerChange::Opened
        }
    }

    fn close(&mut self) -> DatePickerChange {
        if self.open {
            self.open = false;
            DatePickerChange::Closed
        } else {
            DatePickerChange::Unchanged
        }
    }

    fn focus(&mut self) -> DatePickerChange {
        if self.focused {
            DatePickerChange::Unchanged
        } else {
            self.focused = true;
            DatePickerChange::Focused
        }
    }

    fn blur(&mut self) -> DatePickerChange {
        if self.focused {
            self.focused = false;
            DatePickerChange::Blurred
        } else {
            DatePickerChange::Unchanged
        }
    }

    fn move_month(&mut self, intent: CalendarIntent) -> DatePickerChange {
        let mut calendar_state = CalendarModel::new(self.visible_year, self.visible_month).state();
        let _ = calendar_state.apply(intent);
        self.visible_year = calendar_state.visible_year();
        self.visible_month = calendar_state.visible_month();
        DatePickerChange::Moved {
            year: self.visible_year,
            month: self.visible_month,
        }
    }

    fn select(&mut self, date: CalendarDate) -> DatePickerChange {
        if self.selected == Some(date) && !self.open {
            return DatePickerChange::Unchanged;
        }
        self.selected = Some(date);
        self.visible_year = date.year;
        self.visible_month = date.month;
        self.open = false;
        DatePickerChange::Selected(date)
    }

    fn clear(&mut self) -> DatePickerChange {
        if self.selected.is_none() {
            DatePickerChange::Unchanged
        } else {
            self.selected = None;
            DatePickerChange::Cleared
        }
    }
}

pub fn validate_date_picker_model(model: &DatePickerModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn date_picker_layout_metrics(
    density: DatePickerDensity,
    open: bool,
    disabled: bool,
    inline_size: f32,
) -> DatePickerLayoutMetrics {
    let dense_root = density == DatePickerDensity::Dense && !disabled;
    let dense_trigger = density == DatePickerDensity::Dense && !open;
    DatePickerLayoutMetrics {
        max_width: scale::container::CONTROL,
        root_padding: if dense_root {
            scale::space::xs(inline_size)
        } else {
            scale::space::s(inline_size)
        },
        root_gap: scale::space::xs2(inline_size),
        trigger_min_height: if dense_trigger {
            scale::space::s(inline_size)
        } else {
            scale::space::FIELD
        },
        trigger_padding_inline: if dense_trigger {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        trigger_padding_block: if dense_trigger {
            scale::space::xs3(inline_size)
        } else {
            scale::space::xs2(inline_size)
        },
        trigger_gap: if dense_trigger {
            scale::space::xs2(inline_size)
        } else {
            scale::space::xs(inline_size)
        },
        trigger_font_size: if dense_trigger {
            scale::font_size::f00(inline_size)
        } else {
            scale::font_size::f0(inline_size)
        },
        trigger_line_height: scale::line_height::LH0,
        marker_size: scale::space::s(inline_size),
        marker_font_size: scale::font_size::f00(inline_size),
        popover_padding: scale::space::xs(inline_size),
        popover_gap: scale::space::xs2(inline_size),
        clear_min_height: scale::space::s(inline_size),
        clear_padding_inline: scale::space::xs2(inline_size),
        clear_padding_block: scale::space::xs3(inline_size),
        clear_font_size: scale::font_size::f00(inline_size),
        clear_line_height: scale::line_height::LH0,
        calendar: calendar_layout_metrics(inline_size),
    }
}

pub fn date_picker_render_nodes(
    model: &DatePickerModel,
    state: &DatePickerState,
) -> Vec<DatePickerRenderNode> {
    let blocked = model.disabled || model.loading;
    let selected_label = date_picker_value_label(model, state);
    let visible_label = format!(
        "{} {}",
        month_name(state.visible_month()),
        state.visible_year()
    );
    let mut nodes = Vec::with_capacity(47);
    nodes.push(DatePickerRenderNode {
        part: DatePickerPart::Root,
        index: 0,
        value: state
            .selected()
            .map_or_else(|| "none".to_owned(), CalendarDate::value),
        label: model.label.clone(),
        detail: selected_label.clone(),
        date: state.selected(),
        density: model.density,
        open: state.is_open(),
        focused: state.is_focused(),
        current_month: true,
        selected: state.selected().is_some(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DatePickerRenderNode {
        part: DatePickerPart::Trigger,
        index: 0,
        value: state
            .selected()
            .map_or_else(|| "none".to_owned(), CalendarDate::value),
        label: model.label.clone(),
        detail: "Toggle date picker".to_owned(),
        date: state.selected(),
        density: model.density,
        open: state.is_open(),
        focused: state.is_focused(),
        current_month: true,
        selected: state.selected().is_some(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DatePickerRenderNode {
        part: DatePickerPart::Popover,
        index: 0,
        value: visible_label.clone(),
        label: visible_label.clone(),
        detail: "Date picker popover".to_owned(),
        date: None,
        density: model.density,
        open: state.is_open(),
        focused: false,
        current_month: true,
        selected: false,
        visible: state.is_open(),
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DatePickerRenderNode {
        part: DatePickerPart::Calendar,
        index: 0,
        value: visible_label,
        label: "Calendar".to_owned(),
        detail: "Six-week date grid".to_owned(),
        date: None,
        density: model.density,
        open: state.is_open(),
        focused: false,
        current_month: true,
        selected: state.selected().is_some(),
        visible: state.is_open(),
        loading: model.loading,
        disabled: blocked,
    });

    let mut calendar_model = CalendarModel::new(state.visible_year(), state.visible_month());
    calendar_model.selected = state.selected();
    calendar_model.loading = model.loading;
    calendar_model.disabled = model.disabled;
    let calendar_state = calendar_model.state();
    for node in calendar_render_nodes(&calendar_model, &calendar_state)
        .into_iter()
        .filter(|node| node.part == CalendarPart::Day)
    {
        nodes.push(DatePickerRenderNode {
            part: DatePickerPart::Calendar,
            index: node.index.saturating_add(1),
            value: node.value,
            label: node.label,
            detail: node.detail,
            date: node.date,
            density: model.density,
            open: state.is_open(),
            focused: false,
            current_month: node.current_month,
            selected: node.selected,
            visible: state.is_open(),
            loading: model.loading,
            disabled: blocked || node.disabled,
        });
    }

    nodes.push(DatePickerRenderNode {
        part: DatePickerPart::Value,
        index: 0,
        value: state
            .selected()
            .map_or_else(|| "none".to_owned(), CalendarDate::value),
        label: selected_label,
        detail: if state.selected().is_some() {
            "Selected date".to_owned()
        } else {
            "No date selected".to_owned()
        },
        date: state.selected(),
        density: model.density,
        open: state.is_open(),
        focused: state.is_focused(),
        current_month: true,
        selected: state.selected().is_some(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes
}

pub fn date_picker_value_label(model: &DatePickerModel, state: &DatePickerState) -> String {
    state
        .selected()
        .map_or_else(|| model.placeholder.clone(), date_picker_date_label)
}

pub fn date_picker_date_label(date: CalendarDate) -> String {
    format!("{} {}, {}", month_name(date.month), date.day, date.year)
}

pub fn default_date_picker_model() -> DatePickerModel {
    DatePickerModel::new(2026, 7)
        .with_label("Published on")
        .with_placeholder("Choose date")
        .with_selected(CalendarDate::new(2026, 7, 7))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_date_picker_model(&default_date_picker_model()).is_ok());
    }

    #[test]
    fn layout_metrics_share_calendar_and_fluid_tailwind_tokens() {
        let compact = date_picker_layout_metrics(DatePickerDensity::Standard, false, false, 320.0);
        let wide = date_picker_layout_metrics(DatePickerDensity::Standard, false, false, 1_000.0);
        let dense = date_picker_layout_metrics(DatePickerDensity::Dense, false, false, 1_000.0);
        let dense_open = date_picker_layout_metrics(DatePickerDensity::Dense, true, false, 1_000.0);
        let disabled_dense =
            date_picker_layout_metrics(DatePickerDensity::Dense, false, true, 1_000.0);

        assert!(compact.root_padding < wide.root_padding);
        assert!(dense.root_padding < wide.root_padding);
        assert_eq!(disabled_dense.root_padding, wide.root_padding);
        assert!(dense.trigger_font_size < dense_open.trigger_font_size);
        assert_eq!(wide.calendar.max_width, wide.max_width);
    }

    #[test]
    fn garde_rejects_empty_label() {
        let model = DatePickerModel::new(2026, 7).with_label("");
        assert!(validate_date_picker_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_visible_month() {
        let mut model = DatePickerModel::new(2026, 7);
        model.visible_month = 13;
        assert!(validate_date_picker_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_selected_date() {
        let model = DatePickerModel::new(2026, 2).with_selected(CalendarDate::new(2026, 2, 30));
        assert!(validate_date_picker_model(&model).is_err());
    }

    #[test]
    fn state_opens_moves_selects_closes_and_clears() {
        let mut state = DatePickerModel::new(2026, 7).state();
        assert_eq!(
            state.apply(DatePickerIntent::Open),
            DatePickerChange::Opened
        );
        assert!(state.is_open());
        assert_eq!(
            state.apply(DatePickerIntent::NextMonth),
            DatePickerChange::Moved {
                year: 2026,
                month: 8
            }
        );
        let date = CalendarDate::new(2026, 8, 10);
        assert_eq!(
            state.apply(DatePickerIntent::Select(date)),
            DatePickerChange::Selected(date)
        );
        assert!(!state.is_open());
        assert_eq!(state.selected(), Some(date));
        assert_eq!(
            state.apply(DatePickerIntent::Clear),
            DatePickerChange::Cleared
        );
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn render_nodes_cover_repeatable_picker_anatomy() {
        let model = default_date_picker_model().with_default_open(true);
        let nodes = date_picker_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(DatePickerPart::Root)
        );
        for part in DatePickerPart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == DatePickerPart::Calendar && node.date.is_some())
                .count(),
            42
        );
    }

    #[test]
    fn closed_state_hides_calendar_day_nodes() {
        let model = default_date_picker_model();
        let nodes = date_picker_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .filter(|node| node.part == DatePickerPart::Calendar && node.date.is_some())
                .all(|node| !node.visible)
        );
    }

    #[test]
    fn loading_disables_calendar_day_nodes() {
        let model = default_date_picker_model()
            .with_default_open(true)
            .loading();
        let nodes = date_picker_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == DatePickerPart::Calendar
                    && node.date.is_some()
                    && node.disabled)
        );
    }

    #[test]
    fn value_label_uses_selected_date_or_placeholder() {
        let empty = DatePickerModel::new(2026, 7);
        assert_eq!(
            date_picker_value_label(&empty, &empty.state()),
            empty.placeholder
        );
        let selected = default_date_picker_model();
        assert_eq!(
            date_picker_value_label(&selected, &selected.state()),
            "July 7, 2026"
        );
    }
}
