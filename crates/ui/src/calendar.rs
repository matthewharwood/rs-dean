use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CalendarSelectionMode {
    Single,
    Range,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Validate)]
pub struct CalendarDate {
    #[garde(range(min = 1900, max = 2200))]
    pub year: u16,
    #[garde(range(min = 1, max = 12))]
    pub month: u8,
    #[garde(range(min = 1, max = 31), custom(calendar_day_exists(&self.year, &self.month)))]
    pub day: u8,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CalendarRange {
    #[garde(dive)]
    pub start: CalendarDate,
    #[garde(dive, custom(calendar_range_end_after_start(&self.start)))]
    pub end: CalendarDate,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CalendarModel {
    #[garde(range(min = 1900, max = 2200))]
    pub visible_year: u16,
    #[garde(range(min = 1, max = 12))]
    pub visible_month: u8,
    #[garde(skip)]
    pub mode: CalendarSelectionMode,
    #[garde(dive)]
    pub selected: Option<CalendarDate>,
    #[garde(dive)]
    pub range: Option<CalendarRange>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarState {
    visible_year: u16,
    visible_month: u8,
    selected: Option<CalendarDate>,
    range_anchor: Option<CalendarDate>,
    range: Option<CalendarRange>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarIntent {
    PreviousMonth,
    NextMonth,
    Select(CalendarDate),
    SelectRange(CalendarDate),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarChange {
    Moved { year: u16, month: u8 },
    Selected(CalendarDate),
    RangeStarted(CalendarDate),
    RangeSelected(CalendarRange),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarPart {
    Root,
    Header,
    Grid,
    Day,
    Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarRenderNode {
    pub part: CalendarPart,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub date: Option<CalendarDate>,
    pub mode: CalendarSelectionMode,
    pub current_month: bool,
    pub selected: bool,
    pub in_range: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl CalendarSelectionMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Range => "range",
        }
    }
}

impl CalendarPart {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Calendar",
            Self::Header => "CalendarHeader",
            Self::Grid => "CalendarGrid",
            Self::Day => "CalendarDay",
            Self::Range => "CalendarRange",
        }
    }
}

impl CalendarDate {
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn value(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn label(self) -> String {
        format!("{} {}", month_short_name(self.month), self.day)
    }
}

impl CalendarRange {
    pub const fn new(start: CalendarDate, end: CalendarDate) -> Self {
        Self { start, end }
    }

    pub fn between(first: CalendarDate, second: CalendarDate) -> Self {
        if first <= second {
            Self::new(first, second)
        } else {
            Self::new(second, first)
        }
    }

    pub fn label(self) -> String {
        format!("{} - {}", self.start.label(), self.end.label())
    }

    pub fn contains(self, date: CalendarDate) -> bool {
        self.start <= date && date <= self.end
    }
}

impl CalendarModel {
    pub const fn new(visible_year: u16, visible_month: u8) -> Self {
        Self {
            visible_year,
            visible_month,
            mode: CalendarSelectionMode::Single,
            selected: None,
            range: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_mode(mut self, mode: CalendarSelectionMode) -> Self {
        self.mode = mode;
        self
    }

    pub const fn with_selected(mut self, selected: CalendarDate) -> Self {
        self.selected = Some(selected);
        self
    }

    pub const fn with_range(mut self, range: CalendarRange) -> Self {
        self.range = Some(range);
        self.mode = CalendarSelectionMode::Range;
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

    pub const fn state(&self) -> CalendarState {
        CalendarState {
            visible_year: self.visible_year,
            visible_month: self.visible_month,
            selected: self.selected,
            range_anchor: None,
            range: self.range,
        }
    }
}

impl CalendarState {
    pub const fn visible_year(&self) -> u16 {
        self.visible_year
    }

    pub const fn visible_month(&self) -> u8 {
        self.visible_month
    }

    pub const fn selected(&self) -> Option<CalendarDate> {
        self.selected
    }

    pub const fn range(&self) -> Option<CalendarRange> {
        self.range
    }

    pub fn is_selected(&self, date: CalendarDate) -> bool {
        self.selected == Some(date)
    }

    pub fn is_in_range(&self, date: CalendarDate) -> bool {
        self.range.is_some_and(|range| range.contains(date))
    }

    pub fn apply(&mut self, intent: CalendarIntent) -> CalendarChange {
        match intent {
            CalendarIntent::PreviousMonth => {
                let (year, month) = previous_month(self.visible_year, self.visible_month);
                self.visible_year = year;
                self.visible_month = month;
                CalendarChange::Moved { year, month }
            }
            CalendarIntent::NextMonth => {
                let (year, month) = next_month(self.visible_year, self.visible_month);
                self.visible_year = year;
                self.visible_month = month;
                CalendarChange::Moved { year, month }
            }
            CalendarIntent::Select(date) => {
                if self.selected == Some(date) {
                    CalendarChange::Unchanged
                } else {
                    self.selected = Some(date);
                    self.range_anchor = None;
                    CalendarChange::Selected(date)
                }
            }
            CalendarIntent::SelectRange(date) => match self.range_anchor.take() {
                Some(anchor) => {
                    let range = CalendarRange::between(anchor, date);
                    self.range = Some(range);
                    self.selected = Some(date);
                    CalendarChange::RangeSelected(range)
                }
                None => {
                    self.range_anchor = Some(date);
                    self.range = None;
                    self.selected = Some(date);
                    CalendarChange::RangeStarted(date)
                }
            },
            CalendarIntent::Clear => {
                if self.selected.is_some() || self.range.is_some() || self.range_anchor.is_some() {
                    self.selected = None;
                    self.range = None;
                    self.range_anchor = None;
                    CalendarChange::Cleared
                } else {
                    CalendarChange::Unchanged
                }
            }
        }
    }
}

pub fn validate_calendar_model(model: &CalendarModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn calendar_render_nodes(
    model: &CalendarModel,
    state: &CalendarState,
) -> Vec<CalendarRenderNode> {
    let mut nodes = Vec::with_capacity(46);
    let visible_label = format!(
        "{} {}",
        month_name(state.visible_month()),
        state.visible_year()
    );
    nodes.push(CalendarRenderNode {
        part: CalendarPart::Root,
        index: 0,
        value: visible_label.clone(),
        label: "Calendar".to_owned(),
        detail: format!("{} selection calendar", model.mode.label()),
        date: None,
        mode: model.mode,
        current_month: true,
        selected: state.selected().is_some() || state.range().is_some(),
        in_range: false,
        loading: model.loading,
        disabled: model.disabled || model.loading,
    });
    nodes.push(CalendarRenderNode {
        part: CalendarPart::Header,
        index: 0,
        value: visible_label.clone(),
        label: visible_label,
        detail: "Visible month".to_owned(),
        date: None,
        mode: model.mode,
        current_month: true,
        selected: false,
        in_range: false,
        loading: model.loading,
        disabled: model.disabled || model.loading,
    });
    nodes.push(CalendarRenderNode {
        part: CalendarPart::Grid,
        index: 0,
        value: "Date grid".to_owned(),
        label: "Date grid".to_owned(),
        detail: "Six-week calendar grid".to_owned(),
        date: None,
        mode: model.mode,
        current_month: true,
        selected: false,
        in_range: false,
        loading: model.loading,
        disabled: model.disabled || model.loading,
    });

    for (index, date) in month_grid_dates(state.visible_year(), state.visible_month())
        .into_iter()
        .enumerate()
    {
        let current_month = date.month == state.visible_month();
        let selected = state.is_selected(date);
        let in_range = state.is_in_range(date);
        nodes.push(CalendarRenderNode {
            part: CalendarPart::Day,
            index,
            value: date.value(),
            label: date.day.to_string(),
            detail: date.label(),
            date: Some(date),
            mode: model.mode,
            current_month,
            selected,
            in_range,
            loading: model.loading,
            disabled: model.disabled || model.loading || !current_month,
        });
    }

    let range = state.range();
    nodes.push(CalendarRenderNode {
        part: CalendarPart::Range,
        index: 0,
        value: range.map_or_else(|| "none".to_owned(), CalendarRange::label),
        label: "Selected range".to_owned(),
        detail: range.map_or_else(|| "No range selected".to_owned(), CalendarRange::label),
        date: None,
        mode: model.mode,
        current_month: true,
        selected: range.is_some(),
        in_range: false,
        loading: model.loading,
        disabled: model.disabled || model.loading,
    });

    nodes
}

pub fn default_calendar_model() -> CalendarModel {
    CalendarModel::new(2026, 7).with_selected(CalendarDate::new(2026, 7, 7))
}

pub fn month_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Invalid month",
    }
}

pub fn month_short_name(month: u8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}

fn month_grid_dates(year: u16, month: u8) -> [CalendarDate; 42] {
    let first_weekday = weekday_index(year, month, 1) as i16;
    let (previous_year, previous_month) = previous_month(year, month);
    let (next_year, next_month) = next_month(year, month);
    let previous_days = days_in_month(previous_year, previous_month) as i16;
    let current_days = days_in_month(year, month) as i16;

    core::array::from_fn(|index| {
        let day = index as i16 - first_weekday + 1;
        if day < 1 {
            CalendarDate::new(previous_year, previous_month, (previous_days + day) as u8)
        } else if day > current_days {
            CalendarDate::new(next_year, next_month, (day - current_days) as u8)
        } else {
            CalendarDate::new(year, month, day as u8)
        }
    })
}

fn calendar_day_exists<'a>(
    year: &'a u16,
    month: &'a u8,
) -> impl FnOnce(&u8, &()) -> garde::Result + 'a {
    move |day, _context| {
        if *day <= days_in_month(*year, *month) {
            Ok(())
        } else {
            Err(garde::Error::new("calendar date day is outside the month"))
        }
    }
}

fn calendar_range_end_after_start(
    start: &CalendarDate,
) -> impl FnOnce(&CalendarDate, &()) -> garde::Result + '_ {
    move |end, _context| {
        if *start <= *end {
            Ok(())
        } else {
            Err(garde::Error::new(
                "calendar range end must be on or after start",
            ))
        }
    }
}

const fn previous_month(year: u16, month: u8) -> (u16, u8) {
    if month == 1 {
        (year - 1, 12)
    } else {
        (year, month - 1)
    }
}

const fn next_month(year: u16, month: u8) -> (u16, u8) {
    if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    }
}

const fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

const fn is_leap_year(year: u16) -> bool {
    year.is_multiple_of(4) && !year.is_multiple_of(100) || year.is_multiple_of(400)
}

fn weekday_index(year: u16, month: u8, day: u8) -> u8 {
    const OFFSETS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut year = i32::from(year);
    let month_index = usize::from(month.saturating_sub(1));
    if month < 3 {
        year -= 1;
    }
    ((year + year / 4 - year / 100 + year / 400 + OFFSETS[month_index] + i32::from(day)) % 7) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_calendar_model(&default_calendar_model()).is_ok());
    }

    #[test]
    fn garde_rejects_invalid_month() {
        let model = CalendarModel::new(2026, 13);
        assert!(validate_calendar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_invalid_day_for_month() {
        let model = CalendarModel::new(2026, 2).with_selected(CalendarDate::new(2026, 2, 30));
        assert!(validate_calendar_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_reversed_range() {
        let model = CalendarModel::new(2026, 7).with_range(CalendarRange::new(
            CalendarDate::new(2026, 7, 10),
            CalendarDate::new(2026, 7, 6),
        ));
        assert!(validate_calendar_model(&model).is_err());
    }

    #[test]
    fn state_selects_single_date() {
        let mut state = CalendarModel::new(2026, 7).state();
        let date = CalendarDate::new(2026, 7, 8);
        assert_eq!(
            state.apply(CalendarIntent::Select(date)),
            CalendarChange::Selected(date)
        );
        assert!(state.is_selected(date));
    }

    #[test]
    fn state_selects_date_range() {
        let mut state = CalendarModel::new(2026, 7)
            .with_mode(CalendarSelectionMode::Range)
            .state();
        let start = CalendarDate::new(2026, 7, 6);
        let end = CalendarDate::new(2026, 7, 10);
        assert_eq!(
            state.apply(CalendarIntent::SelectRange(start)),
            CalendarChange::RangeStarted(start)
        );
        assert_eq!(
            state.apply(CalendarIntent::SelectRange(end)),
            CalendarChange::RangeSelected(CalendarRange::new(start, end))
        );
        assert!(state.is_in_range(CalendarDate::new(2026, 7, 8)));
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_calendar_model();
        let nodes = calendar_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(CalendarPart::Root)
        );
        assert!(nodes.iter().any(|node| node.part == CalendarPart::Header));
        assert!(nodes.iter().any(|node| node.part == CalendarPart::Grid));
        assert!(nodes.iter().any(|node| node.part == CalendarPart::Day));
        assert!(nodes.iter().any(|node| node.part == CalendarPart::Range));
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == CalendarPart::Day)
                .count(),
            42
        );
    }
}
