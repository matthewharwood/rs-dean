use std::collections::HashSet;

use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TableDensity {
    Standard,
    Dense,
}

impl TableDensity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TablePart {
    Root,
    Header,
    Body,
    Row,
    Head,
    Cell,
    Caption,
}

impl TablePart {
    pub const ALL: &'static [Self] = &[
        Self::Root,
        Self::Header,
        Self::Body,
        Self::Row,
        Self::Head,
        Self::Cell,
        Self::Caption,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "Table",
            Self::Header => "TableHeader",
            Self::Body => "TableBody",
            Self::Row => "TableRow",
            Self::Head => "TableHead",
            Self::Cell => "TableCell",
            Self::Caption => "TableCaption",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct TableColumn {
    #[garde(length(min = 1, max = 96))]
    pub label: String,
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(skip)]
    pub numeric: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct TableRow {
    #[garde(length(min = 1, max = 128))]
    pub value: String,
    #[garde(length(min = 1, max = 24))]
    pub cells: Vec<String>,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct TableModel {
    #[garde(skip)]
    pub density: TableDensity,
    #[garde(length(min = 1, max = 160))]
    pub caption: String,
    #[garde(length(min = 1, max = 12), dive, custom(table_columns_are_valid))]
    pub columns: Vec<TableColumn>,
    #[garde(length(min = 1, max = 100), dive, custom(table_rows_are_valid(&self.columns)))]
    pub rows: Vec<TableRow>,
    #[garde(custom(table_selected_row_references_row(&self.rows)))]
    pub selected_row: Option<String>,
    #[garde(custom(validate_optional_table_copy))]
    pub error: Option<String>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableState {
    selected_row: Option<String>,
    focused_part: Option<TablePart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableIntent {
    SelectRow(String),
    Focus(TablePart),
    Blur,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableChange {
    RowSelected(String),
    Focused(TablePart),
    Blurred,
    Cleared,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRenderNode {
    pub part: TablePart,
    pub row_index: usize,
    pub column_index: usize,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub row_value: String,
    pub column_value: String,
    pub density: TableDensity,
    pub numeric: bool,
    pub selected: bool,
    pub active: bool,
    pub visible: bool,
    pub invalid: bool,
    pub loading: bool,
    pub disabled: bool,
    pub actionable: bool,
}

impl TableColumn {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            numeric: false,
        }
    }

    pub const fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }
}

impl TableRow {
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
}

impl TableModel {
    pub fn new(columns: Vec<TableColumn>, rows: Vec<TableRow>) -> Self {
        Self {
            density: TableDensity::Standard,
            caption: "Shared component status table".to_owned(),
            columns,
            rows,
            selected_row: None,
            error: None,
            loading: false,
            disabled: false,
        }
    }

    pub const fn with_density(mut self, density: TableDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = caption.into();
        self
    }

    pub fn with_selected_row(mut self, row: impl Into<String>) -> Self {
        self.selected_row = Some(row.into());
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
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

    pub fn state(&self) -> TableState {
        TableState::new(self.selected_row.clone(), None)
    }
}

impl TableState {
    pub const fn new(selected_row: Option<String>, focused_part: Option<TablePart>) -> Self {
        Self {
            selected_row,
            focused_part,
        }
    }

    pub fn selected_row(&self) -> Option<&str> {
        self.selected_row.as_deref()
    }

    pub const fn focused_part(&self) -> Option<TablePart> {
        self.focused_part
    }

    pub fn is_selected(&self, row: &str) -> bool {
        self.selected_row.as_deref() == Some(row)
    }

    pub const fn is_focused(&self, part: TablePart) -> bool {
        matches!(self.focused_part, Some(active) if active as u8 == part as u8)
    }

    pub fn apply(&mut self, intent: TableIntent) -> TableChange {
        match intent {
            TableIntent::SelectRow(row) => {
                if row.is_empty() || self.selected_row.as_ref() == Some(&row) {
                    TableChange::Unchanged
                } else {
                    self.selected_row = Some(row.clone());
                    TableChange::RowSelected(row)
                }
            }
            TableIntent::Focus(part) => {
                if self.focused_part == Some(part) {
                    TableChange::Unchanged
                } else {
                    self.focused_part = Some(part);
                    TableChange::Focused(part)
                }
            }
            TableIntent::Blur => {
                if self.focused_part.is_none() {
                    TableChange::Unchanged
                } else {
                    self.focused_part = None;
                    TableChange::Blurred
                }
            }
            TableIntent::Clear => {
                if self.selected_row.is_none() && self.focused_part.is_none() {
                    TableChange::Unchanged
                } else {
                    self.selected_row = None;
                    self.focused_part = None;
                    TableChange::Cleared
                }
            }
        }
    }
}

pub fn validate_table_model(model: &TableModel) -> Result<(), garde::Report> {
    model.validate()
}

pub fn table_render_nodes(model: &TableModel, state: &TableState) -> Vec<TableRenderNode> {
    let blocked = model.disabled || model.loading;
    let invalid = model.error.is_some();
    let mut nodes = Vec::with_capacity(
        model
            .columns
            .len()
            .saturating_mul(model.rows.len().saturating_add(1))
            .saturating_add(model.rows.len())
            .saturating_add(4),
    );
    nodes.push(TableRenderNode {
        part: TablePart::Root,
        row_index: 0,
        column_index: 0,
        value: state.selected_row().unwrap_or("none").to_owned(),
        label: "Table".to_owned(),
        detail: format!(
            "{} rows across {} columns",
            model.rows.len(),
            model.columns.len()
        ),
        row_value: String::new(),
        column_value: String::new(),
        density: model.density,
        numeric: false,
        selected: state.selected_row().is_some(),
        active: state.is_focused(TablePart::Root),
        visible: true,
        invalid,
        loading: model.loading,
        disabled: blocked,
        actionable: false,
    });
    nodes.push(TableRenderNode {
        part: TablePart::Header,
        row_index: 0,
        column_index: 0,
        value: "header".to_owned(),
        label: "Table header".to_owned(),
        detail: format!("{} column headers", model.columns.len()),
        row_value: String::new(),
        column_value: String::new(),
        density: model.density,
        numeric: false,
        selected: false,
        active: state.is_focused(TablePart::Header),
        visible: true,
        invalid,
        loading: model.loading,
        disabled: blocked,
        actionable: false,
    });
    for (column_index, column) in model.columns.iter().enumerate() {
        nodes.push(TableRenderNode {
            part: TablePart::Head,
            row_index: 0,
            column_index,
            value: column.value.clone(),
            label: column.label.clone(),
            detail: if column.numeric {
                "Numeric table heading".to_owned()
            } else {
                "Table heading".to_owned()
            },
            row_value: String::new(),
            column_value: column.value.clone(),
            density: model.density,
            numeric: column.numeric,
            selected: false,
            active: state.is_focused(TablePart::Head),
            visible: true,
            invalid,
            loading: model.loading,
            disabled: blocked,
            actionable: false,
        });
    }
    nodes.push(TableRenderNode {
        part: TablePart::Body,
        row_index: 0,
        column_index: 0,
        value: "body".to_owned(),
        label: "Table body".to_owned(),
        detail: format!("{} table rows", model.rows.len()),
        row_value: String::new(),
        column_value: String::new(),
        density: model.density,
        numeric: false,
        selected: false,
        active: state.is_focused(TablePart::Body),
        visible: true,
        invalid,
        loading: model.loading,
        disabled: blocked,
        actionable: false,
    });
    for (row_index, row) in model.rows.iter().enumerate() {
        let row_blocked = blocked || row.disabled;
        let selected = state.is_selected(&row.value);
        nodes.push(TableRenderNode {
            part: TablePart::Row,
            row_index,
            column_index: 0,
            value: row.value.clone(),
            label: format!("Row {}", row_index.saturating_add(1)),
            detail: row.value.clone(),
            row_value: row.value.clone(),
            column_value: String::new(),
            density: model.density,
            numeric: false,
            selected,
            active: state.is_focused(TablePart::Row),
            visible: true,
            invalid,
            loading: model.loading,
            disabled: row_blocked,
            actionable: !row_blocked,
        });
        for (column_index, column) in model.columns.iter().enumerate() {
            let cell = row.cells.get(column_index).cloned().unwrap_or_default();
            nodes.push(TableRenderNode {
                part: TablePart::Cell,
                row_index,
                column_index,
                value: format!("{}:{}", row.value, column.value),
                label: cell.clone(),
                detail: format!("{}: {cell}", column.label),
                row_value: row.value.clone(),
                column_value: column.value.clone(),
                density: model.density,
                numeric: column.numeric,
                selected,
                active: state.is_focused(TablePart::Cell),
                visible: true,
                invalid,
                loading: model.loading,
                disabled: row_blocked,
                actionable: false,
            });
        }
    }
    nodes.push(TableRenderNode {
        part: TablePart::Caption,
        row_index: 0,
        column_index: 0,
        value: "caption".to_owned(),
        label: "Table caption".to_owned(),
        detail: model.error.clone().unwrap_or_else(|| model.caption.clone()),
        row_value: String::new(),
        column_value: String::new(),
        density: model.density,
        numeric: false,
        selected: false,
        active: state.is_focused(TablePart::Caption),
        visible: true,
        invalid,
        loading: model.loading,
        disabled: blocked,
        actionable: false,
    });
    nodes
}

pub fn default_table_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("Component", "component"),
        TableColumn::new("Surface", "surface"),
        TableColumn::new("Ready", "ready").numeric(),
    ]
}

pub fn default_table_rows() -> Vec<TableRow> {
    vec![
        TableRow::new(
            "accordion",
            vec![
                "Accordion".to_owned(),
                "Disclosure".to_owned(),
                "100".to_owned(),
            ],
        ),
        TableRow::new(
            "switch",
            vec!["Switch".to_owned(), "Control".to_owned(), "100".to_owned()],
        ),
        TableRow::new(
            "table",
            vec!["Table".to_owned(), "Data".to_owned(), "100".to_owned()],
        ),
    ]
}

pub fn default_table_model() -> TableModel {
    TableModel::new(default_table_columns(), default_table_rows())
        .with_caption("Concrete UI components rendered from shared Rust table data.")
        .with_selected_row("switch")
}

fn table_columns_are_valid(columns: &Vec<TableColumn>, _context: &()) -> garde::Result {
    let mut values = HashSet::with_capacity(columns.len());
    for column in columns {
        if !values.insert(column.value.as_str()) {
            return Err(garde::Error::new("table column values must be unique"));
        }
    }
    Ok(())
}

fn table_rows_are_valid<'a>(
    columns: &'a [TableColumn],
) -> impl FnOnce(&Vec<TableRow>, &()) -> garde::Result + 'a {
    move |rows, _context| {
        let mut values = HashSet::with_capacity(rows.len());
        for row in rows {
            if !values.insert(row.value.as_str()) {
                return Err(garde::Error::new("table row values must be unique"));
            }
            if row.cells.len() != columns.len() {
                return Err(garde::Error::new(
                    "table row cells must match the column count",
                ));
            }
            for cell in &row.cells {
                if !(1..=160).contains(&cell.chars().count()) {
                    return Err(garde::Error::new("table cells must be 1 to 160 characters"));
                }
            }
        }
        Ok(())
    }
}

fn table_selected_row_references_row<'a>(
    rows: &'a [TableRow],
) -> impl FnOnce(&Option<String>, &()) -> garde::Result + 'a {
    move |selected_row, _context| {
        if let Some(selected_row) = selected_row
            && !rows.iter().any(|row| &row.value == selected_row)
        {
            return Err(garde::Error::new(
                "selected table row must reference an existing row",
            ));
        }
        Ok(())
    }
}

fn validate_optional_table_copy(copy: &Option<String>, _context: &()) -> garde::Result {
    if let Some(copy) = copy
        && !(1..=240).contains(&copy.chars().count())
    {
        return Err(garde::Error::new("table copy must be 1 to 240 characters"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_validates_with_garde() {
        assert!(validate_table_model(&default_table_model()).is_ok());
    }

    #[test]
    fn garde_rejects_empty_columns() {
        let model = TableModel::new(Vec::new(), default_table_rows());
        assert!(validate_table_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_duplicate_column_values() {
        let model = TableModel::new(
            vec![
                TableColumn::new("Component", "same"),
                TableColumn::new("Surface", "same"),
            ],
            vec![TableRow::new(
                "accordion",
                vec!["Accordion".to_owned(), "Disclosure".to_owned()],
            )],
        );
        assert!(validate_table_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_mismatched_cell_count() {
        let model = TableModel::new(
            vec![TableColumn::new("Component", "component")],
            vec![TableRow::new(
                "accordion",
                vec!["Accordion".to_owned(), "Disclosure".to_owned()],
            )],
        );
        assert!(validate_table_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_unknown_selected_row() {
        let model = default_table_model().with_selected_row("missing");
        assert!(validate_table_model(&model).is_err());
    }

    #[test]
    fn garde_rejects_empty_error() {
        let model = default_table_model().with_error("");
        assert!(validate_table_model(&model).is_err());
    }

    #[test]
    fn state_selects_focuses_blurs_and_clears() {
        let mut state = TableState::new(None, None);
        assert_eq!(
            state.apply(TableIntent::SelectRow("switch".to_owned())),
            TableChange::RowSelected("switch".to_owned())
        );
        assert!(state.is_selected("switch"));
        assert_eq!(
            state.apply(TableIntent::Focus(TablePart::Row)),
            TableChange::Focused(TablePart::Row)
        );
        assert!(state.is_focused(TablePart::Row));
        assert_eq!(state.apply(TableIntent::Blur), TableChange::Blurred);
        assert_eq!(state.apply(TableIntent::Clear), TableChange::Cleared);
        assert_eq!(state.selected_row(), None);
    }

    #[test]
    fn render_nodes_cover_shadcn_anatomy() {
        let model = default_table_model();
        let nodes = table_render_nodes(&model, &model.state());
        assert_eq!(nodes.first().map(|node| node.part), Some(TablePart::Root));
        for part in TablePart::ALL {
            assert!(
                nodes.iter().any(|node| node.part == *part),
                "missing {}",
                part.label()
            );
        }
    }

    #[test]
    fn error_marks_caption_invalid() {
        let model = default_table_model().with_error("Table data failed validation.");
        let nodes = table_render_nodes(&model, &model.state());
        assert!(nodes.iter().all(|node| node.invalid));
        assert!(nodes.iter().any(|node| node.part == TablePart::Caption
            && node.detail == "Table data failed validation."));
    }

    #[test]
    fn loading_disables_rows() {
        let model = default_table_model().loading();
        let nodes = table_render_nodes(&model, &model.state());
        assert!(
            nodes
                .iter()
                .any(|node| node.part == TablePart::Row && node.disabled && !node.actionable)
        );
    }
}
