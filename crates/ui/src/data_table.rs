use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataTableDensity {
    Standard,
    Dense,
}

impl DataTableDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataTableSortDirection {
    Ascending,
    Descending,
}

impl DataTableSortDirection {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
        }
    }

    pub const fn toggled(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataTablePart {
    Root,
    Toolbar,
    Header,
    Row,
    Cell,
    Pagination,
}

impl DataTablePart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Toolbar,
        Self::Header,
        Self::Row,
        Self::Cell,
        Self::Pagination,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "DataTable",
            Self::Toolbar => "DataTableToolbar",
            Self::Header => "DataTableHeader",
            Self::Row => "DataTableRow",
            Self::Cell => "DataTableCell",
            Self::Pagination => "DataTablePagination",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DataTableColumn {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub sortable: bool,
    #[garde(skip)]
    pub numeric: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DataTableRow {
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 24))]
    pub cells: Vec<String>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct DataTableModel {
    #[garde(skip)]
    pub density: DataTableDensity,
    #[garde(length(min = 1, max = 128))]
    pub title: String,
    #[garde(length(min = 1, max = 128))]
    pub filter_placeholder: String,
    #[garde(length(min = 1, max = 160))]
    pub empty_label: String,
    #[garde(length(min = 1, max = 12), dive, custom(data_table_columns_are_valid))]
    pub columns: Vec<DataTableColumn>,
    #[garde(length(min = 1, max = 100), dive, custom(data_table_rows_are_valid(&self.columns)))]
    pub rows: Vec<DataTableRow>,
    #[garde(custom(data_table_sort_references_column(&self.columns)))]
    pub default_sort_column: Option<String>,
    #[garde(skip)]
    pub default_sort_direction: DataTableSortDirection,
    #[garde(range(min = 1, max = 50))]
    pub page_size: usize,
    #[garde(custom(data_table_selected_row_references_row(&self.rows)))]
    pub selected_row: Option<String>,
    #[garde(length(max = 128))]
    pub default_filter: String,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataTableState {
    filter: String,
    sort_column: Option<String>,
    sort_direction: DataTableSortDirection,
    page_index: usize,
    selected_row: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTableIntent {
    Filter(String),
    Sort(String),
    NextPage,
    PreviousPage,
    SelectRow(String),
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTableChange {
    Filtered(String),
    Sorted(String, DataTableSortDirection),
    PageChanged(usize),
    RowSelected(String),
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataTableRenderNode {
    pub part: DataTablePart,
    pub row_index: usize,
    pub column_index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub row_value: String,
    pub column_value: String,
    pub density: DataTableDensity,
    pub sort_direction: Option<DataTableSortDirection>,
    pub selected: bool,
    pub visible: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl DataTableColumn {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            sortable: true,
            numeric: false,
        }
    }

    pub const fn unsortable(mut self) -> Self {
        self.sortable = false;
        self
    }

    pub const fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }
}

impl DataTableRow {
    pub fn new(value: impl Into<String>, cells: Vec<String>) -> Self {
        Self {
            value: value.into(),
            cells,
            disabled: false,
        }
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    fn matches_filter(&self, filter: &str) -> bool {
        if filter.is_empty() {
            return true;
        }
        let filter = filter.to_ascii_lowercase();
        self.value.to_ascii_lowercase().contains(&filter)
            || self
                .cells
                .iter()
                .any(|cell| cell.to_ascii_lowercase().contains(&filter))
    }
}

impl DataTableModel {
    pub fn new(columns: Vec<DataTableColumn>, rows: Vec<DataTableRow>) -> Self {
        Self {
            density: DataTableDensity::Standard,
            title: "Data table".to_owned(),
            filter_placeholder: "Filter rows".to_owned(),
            empty_label: "No rows found.".to_owned(),
            columns,
            rows,
            default_sort_column: None,
            default_sort_direction: DataTableSortDirection::Ascending,
            page_size: 3,
            selected_row: None,
            default_filter: String::new(),
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: DataTableDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_filter_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.filter_placeholder = placeholder.into();
        self
    }

    pub fn with_empty_label(mut self, label: impl Into<String>) -> Self {
        self.empty_label = label.into();
        self
    }

    pub fn with_sort(
        mut self,
        column: impl Into<String>,
        direction: DataTableSortDirection,
    ) -> Self {
        self.default_sort_column = Some(column.into());
        self.default_sort_direction = direction;
        self
    }

    pub const fn with_page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn with_selected_row(mut self, value: impl Into<String>) -> Self {
        self.selected_row = Some(value.into());
        self
    }

    pub fn with_default_filter(mut self, filter: impl Into<String>) -> Self {
        self.default_filter = filter.into();
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

    pub fn state(&self) -> DataTableState {
        DataTableState::new(
            self.default_filter.clone(),
            self.default_sort_column.clone(),
            self.default_sort_direction,
            0,
            self.selected_row.clone(),
        )
    }
}

impl DataTableState {
    pub fn new(
        filter: String,
        sort_column: Option<String>,
        sort_direction: DataTableSortDirection,
        page_index: usize,
        selected_row: Option<String>,
    ) -> Self {
        Self {
            filter,
            sort_column,
            sort_direction,
            page_index,
            selected_row,
        }
    }

    pub fn filter(&self) -> &str {
        &self.filter
    }

    pub fn sort_column(&self) -> Option<&str> {
        self.sort_column.as_deref()
    }

    pub const fn sort_direction(&self) -> DataTableSortDirection {
        self.sort_direction
    }

    pub const fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn selected_row(&self) -> Option<&str> {
        self.selected_row.as_deref()
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_row.as_deref() == Some(value)
    }

    pub fn apply(&mut self, intent: DataTableIntent, max_page_index: usize) -> DataTableChange {
        match intent {
            DataTableIntent::Filter(filter) => self.update_filter(filter),
            DataTableIntent::Sort(column) => self.sort(column),
            DataTableIntent::NextPage => self.next_page(max_page_index),
            DataTableIntent::PreviousPage => self.previous_page(),
            DataTableIntent::SelectRow(row) => self.select_row(row),
            DataTableIntent::Clear => self.clear(),
        }
    }

    fn update_filter(&mut self, filter: String) -> DataTableChange {
        if self.filter == filter {
            DataTableChange::Unchanged
        } else {
            self.filter = filter.clone();
            self.page_index = 0;
            DataTableChange::Filtered(filter)
        }
    }

    fn sort(&mut self, column: String) -> DataTableChange {
        if column.is_empty() {
            return DataTableChange::Unchanged;
        }
        if self.sort_column.as_ref() == Some(&column) {
            self.sort_direction = self.sort_direction.toggled();
        } else {
            self.sort_column = Some(column.clone());
            self.sort_direction = DataTableSortDirection::Ascending;
        }
        self.page_index = 0;
        DataTableChange::Sorted(column, self.sort_direction)
    }

    fn next_page(&mut self, max_page_index: usize) -> DataTableChange {
        if self.page_index >= max_page_index {
            DataTableChange::Unchanged
        } else {
            self.page_index = self.page_index.saturating_add(1);
            DataTableChange::PageChanged(self.page_index)
        }
    }

    fn previous_page(&mut self) -> DataTableChange {
        if self.page_index == 0 {
            DataTableChange::Unchanged
        } else {
            self.page_index = self.page_index.saturating_sub(1);
            DataTableChange::PageChanged(self.page_index)
        }
    }

    fn select_row(&mut self, row: String) -> DataTableChange {
        if row.is_empty() || self.selected_row.as_ref() == Some(&row) {
            return DataTableChange::Unchanged;
        }
        self.selected_row = Some(row.clone());
        DataTableChange::RowSelected(row)
    }

    fn clear(&mut self) -> DataTableChange {
        if self.filter.is_empty()
            && self.sort_column.is_none()
            && self.page_index == 0
            && self.selected_row.is_none()
        {
            return DataTableChange::Unchanged;
        }
        self.filter.clear();
        self.sort_column = None;
        self.page_index = 0;
        self.selected_row = None;
        DataTableChange::Cleared
    }
}

pub fn validate_data_table_model(model: &DataTableModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn data_table_render_nodes(
    model: &DataTableModel,
    state: &DataTableState,
) -> Vec<DataTableRenderNode> {
    let blocked = model.disabled || model.loading;
    let sorted_rows = visible_data_table_rows(model, state);
    let max_page = max_data_table_page_index(model, state);
    let page_index = state.page_index().min(max_page);
    let start = page_index.saturating_mul(model.page_size);
    let page_rows = sorted_rows
        .iter()
        .skip(start)
        .take(model.page_size)
        .copied()
        .collect::<Vec<_>>();
    let mut nodes = Vec::with_capacity(
        model
            .columns
            .len()
            .saturating_mul(page_rows.len().saturating_add(1))
            .saturating_add(page_rows.len())
            .saturating_add(3),
    );
    nodes.push(DataTableRenderNode {
        part: DataTablePart::Root,
        row_index: 0,
        column_index: 0,
        value: state.selected_row().unwrap_or("none").to_owned(),
        label: model.title.clone(),
        detail: format!("{} rows", sorted_rows.len()),
        row_value: String::new(),
        column_value: String::new(),
        density: model.density,
        sort_direction: None,
        selected: state.selected_row().is_some(),
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes.push(DataTableRenderNode {
        part: DataTablePart::Toolbar,
        row_index: 0,
        column_index: 0,
        value: state.filter().to_owned(),
        label: model.filter_placeholder.clone(),
        detail: format!("{} filtered rows", sorted_rows.len()),
        row_value: String::new(),
        column_value: String::new(),
        density: model.density,
        sort_direction: None,
        selected: false,
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    for (column_index, column) in model.columns.iter().enumerate() {
        nodes.push(DataTableRenderNode {
            part: DataTablePart::Header,
            row_index: 0,
            column_index,
            value: column.value.clone(),
            label: column.label.clone(),
            detail: if column.sortable {
                "Sortable column".to_owned()
            } else {
                "Static column".to_owned()
            },
            row_value: String::new(),
            column_value: column.value.clone(),
            density: model.density,
            sort_direction: state
                .sort_column()
                .filter(|sort| *sort == column.value)
                .map(|_| state.sort_direction()),
            selected: state.sort_column() == Some(column.value.as_str()),
            visible: true,
            loading: model.loading,
            disabled: blocked || !column.sortable,
        });
    }
    if page_rows.is_empty() {
        nodes.push(DataTableRenderNode {
            part: DataTablePart::Row,
            row_index: 0,
            column_index: 0,
            value: "empty".to_owned(),
            label: model.empty_label.clone(),
            detail: state.filter().to_owned(),
            row_value: String::new(),
            column_value: String::new(),
            density: model.density,
            sort_direction: None,
            selected: false,
            visible: true,
            loading: model.loading,
            disabled: true,
        });
    } else {
        for (row_index, row) in page_rows.iter().enumerate() {
            let selected = state.is_selected(&row.value);
            nodes.push(DataTableRenderNode {
                part: DataTablePart::Row,
                row_index,
                column_index: 0,
                value: row.value.clone(),
                label: row
                    .cells
                    .first()
                    .cloned()
                    .unwrap_or_else(|| row.value.clone()),
                detail: format!("{} cells", row.cells.len()),
                row_value: row.value.clone(),
                column_value: String::new(),
                density: model.density,
                sort_direction: None,
                selected,
                visible: true,
                loading: model.loading,
                disabled: blocked || row.disabled,
            });
            for (column_index, cell) in row.cells.iter().enumerate() {
                let column = &model.columns[column_index];
                nodes.push(DataTableRenderNode {
                    part: DataTablePart::Cell,
                    row_index,
                    column_index,
                    value: cell.clone(),
                    label: cell.clone(),
                    detail: column.label.clone(),
                    row_value: row.value.clone(),
                    column_value: column.value.clone(),
                    density: model.density,
                    sort_direction: None,
                    selected,
                    visible: true,
                    loading: model.loading,
                    disabled: blocked || row.disabled,
                });
            }
        }
    }
    nodes.push(DataTableRenderNode {
        part: DataTablePart::Pagination,
        row_index: page_index,
        column_index: max_page,
        value: page_index.saturating_add(1).to_string(),
        label: format!(
            "Page {} of {}",
            page_index.saturating_add(1),
            max_page.saturating_add(1)
        ),
        detail: sorted_rows.len().to_string(),
        row_value: String::new(),
        column_value: String::new(),
        density: model.density,
        sort_direction: None,
        selected: page_index > 0,
        visible: true,
        loading: model.loading,
        disabled: blocked,
    });
    nodes
}

pub fn visible_data_table_rows<'a>(
    model: &'a DataTableModel,
    state: &DataTableState,
) -> Vec<&'a DataTableRow> {
    let mut rows = model
        .rows
        .iter()
        .filter(|row| row.matches_filter(state.filter()))
        .collect::<Vec<_>>();
    if let Some(sort_column) = state.sort_column()
        && let Some(column_index) = model
            .columns
            .iter()
            .position(|column| column.value == sort_column)
    {
        rows.sort_by(|left, right| left.cells[column_index].cmp(&right.cells[column_index]));
        if state.sort_direction() == DataTableSortDirection::Descending {
            rows.reverse();
        }
    }
    rows
}

pub fn max_data_table_page_index(model: &DataTableModel, state: &DataTableState) -> usize {
    let rows = visible_data_table_rows(model, state).len();
    if rows == 0 {
        0
    } else {
        rows.saturating_sub(1) / model.page_size
    }
}

pub fn default_data_table_model() -> DataTableModel {
    DataTableModel::new(
        vec![
            DataTableColumn::new("Component", "component"),
            DataTableColumn::new("Surface", "surface"),
            DataTableColumn::new("Status", "status"),
        ],
        vec![
            DataTableRow::new(
                "accordion",
                vec![
                    "Accordion".to_owned(),
                    "Disclosure".to_owned(),
                    "Complete".to_owned(),
                ],
            ),
            DataTableRow::new(
                "combobox",
                vec![
                    "Combobox".to_owned(),
                    "Input".to_owned(),
                    "Complete".to_owned(),
                ],
            ),
            DataTableRow::new(
                "command",
                vec![
                    "Command".to_owned(),
                    "Overlay".to_owned(),
                    "Complete".to_owned(),
                ],
            ),
            DataTableRow::new(
                "context-menu",
                vec![
                    "Context Menu".to_owned(),
                    "Overlay".to_owned(),
                    "Complete".to_owned(),
                ],
            ),
        ],
    )
    .with_title("Component status")
    .with_sort("component", DataTableSortDirection::Ascending)
    .with_selected_row("command")
}

fn data_table_columns_are_valid(columns: &Vec<DataTableColumn>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(columns.len());
    for column in columns {
        if !values.insert(column.value.as_str()) {
            return Err(garde::Error::new("data table column values must be unique"));
        }
    }
    Ok(())
}

fn data_table_rows_are_valid<'a>(
    columns: &'a [DataTableColumn],
) -> impl FnOnce(&Vec<DataTableRow>, &()) -> garde::Result + 'a {
    move |rows, _context| {
        let mut values = HashSet::with_capacity(rows.len());
        for row in rows {
            if !values.insert(row.value.as_str()) {
                return Err(garde::Error::new("data table row values must be unique"));
            }
            if row.cells.len() != columns.len() {
                return Err(garde::Error::new(
                    "data table row cells must match column count",
                ));
            }
            if row
                .cells
                .iter()
                .any(|cell| !(1..=160).contains(&cell.chars().count()))
            {
                return Err(garde::Error::new(
                    "data table cells must be 1 to 160 characters",
                ));
            }
        }
        Ok(())
    }
}

fn data_table_sort_references_column<'a>(
    columns: &'a [DataTableColumn],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |sort_column, _context| {
        if let Some(sort_column) = sort_column
            && !columns
                .iter()
                .any(|column| column.value == *sort_column && column.sortable)
        {
            return Err(garde::Error::new(
                "data table sort column must reference a sortable column",
            ));
        }
        Ok(())
    }
}

fn data_table_selected_row_references_row<'a>(
    rows: &'a [DataTableRow],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |selected, _context| {
        if let Some(selected) = selected
            && !rows
                .iter()
                .any(|row| row.value == *selected && !row.disabled)
        {
            return Err(garde::Error::new(
                "selected data table row must reference an enabled row",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_data_table_model(&default_data_table_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_columns() {
        let model = DataTableModel::new(Vec::new(), Vec::new());
        assert!(validate_data_table_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_column_values() {
        let model = DataTableModel::new(
            vec![
                DataTableColumn::new("Name", "same"),
                DataTableColumn::new("Status", "same"),
            ],
            vec![DataTableRow::new(
                "row",
                vec!["Alice".to_owned(), "Ready".to_owned()],
            )],
        );
        assert!(validate_data_table_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_mismatched_cell_count() {
        let model = DataTableModel::new(
            vec![DataTableColumn::new("Name", "name")],
            vec![DataTableRow::new(
                "row",
                vec!["Alice".to_owned(), "Ready".to_owned()],
            )],
        );
        assert!(validate_data_table_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_row() {
        let model = default_data_table_model().with_selected_row("unknown");
        assert!(validate_data_table_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unsortable_default_sort() {
        let model = DataTableModel::new(
            vec![DataTableColumn::new("Actions", "actions").unsortable()],
            vec![DataTableRow::new("row", vec!["Open".to_owned()])],
        )
        .with_sort("actions", DataTableSortDirection::Ascending);
        assert!(validate_data_table_model(&model).is_err());
    }

    #[test]
    fn state_filters_sorts_pages_selects_and_clears() {
        let model = default_data_table_model().with_page_size(2);
        let mut state = model.state();
        let max_page = max_data_table_page_index(&model, &state);
        assert_eq!(
            state.apply(DataTableIntent::Filter("co".to_owned()), max_page),
            DataTableChange::Filtered("co".to_owned())
        );
        assert_eq!(state.filter(), "co");
        let max_page = max_data_table_page_index(&model, &state);
        assert_eq!(
            state.apply(DataTableIntent::Sort("component".to_owned()), max_page),
            DataTableChange::Sorted("component".to_owned(), DataTableSortDirection::Descending)
        );
        let max_page = max_data_table_page_index(&model, &state);
        assert_eq!(
            state.apply(DataTableIntent::SelectRow("combobox".to_owned()), max_page),
            DataTableChange::RowSelected("combobox".to_owned())
        );
        assert_eq!(state.selected_row(), Some("combobox"));
        let max_page = max_data_table_page_index(&model, &state);
        assert_eq!(
            state.apply(DataTableIntent::Clear, max_page),
            DataTableChange::Cleared
        );
    }

    #[test]
    fn render_nodes_cover_repeatable_shadcn_anatomy() {
        let model = default_data_table_model();
        let nodes = data_table_render_nodes(&model, &model.state());
        assert_eq!(
            nodes.first().map(|node| node.part),
            Some(DataTablePart::Root)
        );
        for part in DataTablePart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|node| node.part == DataTablePart::Header)
                .count(),
            3
        );
    }

    #[test]
    fn empty_filter_keeps_empty_row_node() {
        let model = default_data_table_model().with_default_filter("missing");
        let nodes = data_table_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == DataTablePart::Row && node.disabled)
        );
    }

    #[test]
    fn loading_disables_row_nodes() {
        let model = default_data_table_model().loading();
        let nodes = data_table_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == DataTablePart::Row && node.disabled)
        );
    }
}
