//! Simple TUI Grid Display Module for Nustage
//!
//! This module implements the grid-based display functionality for the terminal UI,
//! allowing users to visualize tabular data in a structured format.

use polars::prelude::*;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

/// Grid display configuration
#[derive(Debug, Clone)]
pub struct GridConfig {
    /// Number of rows to display in the grid
    pub row_count: usize,
    /// Column width constraints
    pub column_constraints: Vec<Constraint>,
    /// Whether to show headers
    pub show_headers: bool,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            row_count: 20,
            column_constraints: vec![Constraint::Percentage(20); 5],
            show_headers: true,
        }
    }
}

/// Grid display state
#[derive(Debug, Clone, Default)]
pub struct GridState {
    /// Current table state for scrolling
    pub table_state: TableState,
    /// Current view offset
    pub offset: usize,
    /// Selected column index
    pub selected_column: Option<usize>,
    /// Filter applied to the data
    pub filter: Option<String>,
}

/// Create a basic grid display for initial exploration
pub fn create_basic_grid_display(df: &DataFrame) -> (GridConfig, GridState) {
    let config = GridConfig {
        row_count: 15.min(df.height()),
        column_constraints: vec![Constraint::Percentage(20); df.width()],
        show_headers: true,
    };

    let state = GridState::default();

    (config, state)
}

/// Render grid display in the TUI frame
pub fn render_grid_display(
    frame: &mut Frame,
    area: Rect,
    df: &DataFrame,
    config: &GridConfig,
    state: &GridState,
) {
    let mut rows = vec![];

    // Add header row if enabled
    if config.show_headers {
        let headers: Vec<Cell> = df
            .get_column_names()
            .iter()
            .map(|name| Cell::from(name.to_string()))
            .collect();
        rows.push(Row::new(headers).style(Style::default().fg(Color::Yellow)));
    }

    // Add data rows with actual values
    let max_rows = config.row_count.min(df.height());

    for row_idx in state.offset..(state.offset + max_rows) {
        if row_idx >= df.height() {
            break;
        }

        let cells: Vec<Cell> = (0..df.width())
            .map(|col_idx| {
                let col_name = df.get_column_names()[col_idx];
                match df.column(col_name) {
                    Ok(series) => series
                        .get(row_idx)
                        .map(|av| av.to_string().into())
                        .unwrap_or(Cell::from("")),
                    Err(_) => Cell::from(""),
                }
            })
            .collect();

        rows.push(Row::new(cells));
    }

    // Create the table widget
    let table = Table::new(rows, config.column_constraints.clone())
        .block(Block::default().title("Data Grid").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .row_highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol(">> ");

    // Render the table
    frame.render_widget(table, area);
}

/// Extract cell value as string for a specific row and column
pub fn get_cell_value(df: &DataFrame, _row_index: usize, col_index: usize) -> String {
    let col_name = df.get_column_names()[col_index];

    match df.column(col_name) {
        Ok(series) => series
            .get(_row_index)
            .map(|av| strip_quotes(&av.to_string()))
            .unwrap_or_default(),
        Err(_) => String::new(),
    }
}

/// Calculate the current view window for grid display
pub fn calculate_view_window(df: &DataFrame, state: &GridState, max_rows: usize) -> (usize, usize) {
    let start_row = state.offset;
    let end_row = (start_row + max_rows).min(df.height());
    (start_row, end_row)
}

/// Get all cell values for a row as strings
pub fn get_row_values(df: &DataFrame, row_index: usize) -> Vec<String> {
    (0..df.width())
        .map(|col_idx| {
            let col_name = df.get_column_names()[col_idx];

            match df.column(col_name) {
                Ok(series) => series
                    .get(row_index)
                    .map(|av| strip_quotes(&av.to_string()))
                    .unwrap_or_default(),
                Err(_) => String::new(),
            }
        })
        .collect()
}

/// Strip surrounding quotes from string representations
fn strip_quotes(s: &str) -> String {
    s.trim_matches('"').trim_matches('\'').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row_values() {
        let a: Column = Series::new("A".into(), &[1i32, 2, 3]).into();
        let b: Column = Series::new("B".into(), &["x", "y", "z"]).into();
        let df = DataFrame::new(3, vec![a, b]).unwrap();

        let values = get_row_values(&df, 0);
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], "1");
        assert_eq!(values[1], "x");
    }

    #[test]
    fn test_get_cell_value() {
        let a: Column = Series::new("A".into(), &[1i32, 2, 3]).into();
        let b: Column = Series::new("B".into(), &["x", "y", "z"]).into();
        let df = DataFrame::new(3, vec![a, b]).unwrap();

        assert_eq!(get_cell_value(&df, 0, 0), "1");
        assert_eq!(get_cell_value(&df, 1, 1), "y");
    }
}
