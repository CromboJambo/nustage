use crate::Pipeline;
use crate::data::{get_schema, load_data};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table},
};
use std::io;
use std::time::Duration;

pub struct App {
    pub pipeline: Pipeline,
    pub current_step_index: usize,
    pub selected_column: Option<String>,
    pub scroll_offset: usize,
    pub rows_per_page: usize,
}

impl App {
    pub fn new(pipeline: Pipeline) -> Self {
        Self {
            pipeline,
            current_step_index: 0,
            selected_column: None,
            scroll_offset: 0,
            rows_per_page: 20,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::Terminal<impl Backend>) -> io::Result<()> {
        // Enable raw mode for key input
        enable_raw_mode()?;

        // Enter alternate screen
        execute!(terminal.backend_mut(), EnterAlternateScreen)?;

        loop {
            // Render the UI
            terminal.draw(|f| self.render(f))?;

            // Handle key events with timeout to avoid blocking indefinitely
            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Down => {
                            self.scroll_offset = self.scroll_offset.saturating_add(1);
                        }
                        KeyCode::Up => {
                            self.scroll_offset = self.scroll_offset.saturating_sub(1);
                        }
                        KeyCode::PageDown => {
                            self.scroll_offset =
                                self.scroll_offset.saturating_add(self.rows_per_page);
                        }
                        KeyCode::PageUp => {
                            self.scroll_offset =
                                self.scroll_offset.saturating_sub(self.rows_per_page);
                        }
                        KeyCode::Home => {
                            self.scroll_offset = 0;
                        }
                        KeyCode::End => {
                            // We'll handle this properly in the render function
                            self.scroll_offset = 0;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        // Cleanup terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }

    fn render(&self, f: &mut Frame) {
        let area = f.area();

        // Create layout with main area and sidebar
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        // Render sidebar (steps)
        self.render_sidebar(f, chunks[0]);

        // Render main data area
        self.render_data_area(f, chunks[1]);
    }

    fn render_sidebar(&self, f: &mut Frame, area: Rect) {
        let steps: Vec<Row> = self
            .pipeline
            .steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                let selected = i == self.current_step_index;
                let style = if selected {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };

                Row::new(vec![
                    Cell::from(format!("{}", i)),
                    Cell::from(step.name.clone()),
                    Cell::from(step.description.clone()),
                ])
                .style(style)
            })
            .collect();

        let step_table = Table::new(
            steps,
            [
                Constraint::Length(5),
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ],
        )
        .header(
            Row::new(vec!["#", "Step", "Description"]).style(Style::default().fg(Color::Yellow)),
        )
        .block(Block::new().title("Pipeline Steps").borders(Borders::ALL));

        f.render_widget(step_table, area);
    }

    fn render_data_area(&self, f: &mut Frame, area: Rect) {
        // Get current dataframe
        let df = self.get_current_dataframe();

        if df.height() == 0 {
            let block = Block::new().title("No Data").borders(Borders::ALL);
            f.render_widget(block, area);
            return;
        }

        // Get schema for column information
        match get_schema(&df) {
            Ok(schema) => {
                // Create header row with proper column names
                let headers: Vec<Cell> = schema
                    .iter()
                    .map(|col| Cell::from(col.name.clone()))
                    .collect();

                let header_row = Row::new(headers).style(Style::default().fg(Color::Yellow));

                // Create data rows with scroll support
                let mut rows: Vec<Row> = vec![];

                // Calculate visible rows (limit to what fits in area)
                let max_visible_rows = area.height.saturating_sub(3) as usize; // Leave space for header and status
                let visible_rows = self.rows_per_page.min(max_visible_rows).min(df.height());
                let start = self.scroll_offset.min(df.height().saturating_sub(visible_rows));

                for i in start..start + visible_rows {
                    if i >= df.height() {
                        break;
                    }

                    let cells: Vec<Cell> = schema
                        .iter()
                        .map(|col| {
                            // Try to get the actual value from this row and column
                            let cell_value = match df.column(&col.name) {
                                Ok(column) => {
                                    if i < column.len() {
                                        match column.get(i) {
                                            Ok(value) => match value {
                                                polars::prelude::AnyValue::Null => "NULL".to_string(),
                                                polars::prelude::AnyValue::Int64(v) => v.to_string(),
                                                polars::prelude::AnyValue::Float64(v) => v.to_string(),
                                                polars::prelude::AnyValue::String(s) => s.to_string(),
                                                polars::prelude::AnyValue::Bool(v) => v.to_string(),
                                                polars::prelude::AnyValue::Date(v) => format!("{}", v),
                                                polars::prelude::AnyValue::Datetime(v, _, _) => {
                                                    format!("{}", v)
                                                }
                                                _ => "N/A".to_string(),
                                            }
                                        }
                                        Err(_) => "ERROR".to_string(),
                                    }
                                }
                                Err(_) => "ERROR".to_string(),
                            };

                            Cell::from(cell_value)
                        })
                        .collect();

                    rows.push(Row::new(cells));
                }

                // Calculate column widths based on content and headers
                let mut column_widths: Vec<usize> = schema
                    .iter()
                    .map(|col| col.name.len())
                    .collect();

                // Adjust for actual data content
                for row in &rows {
                    for (i, cell) in row.cells.iter().enumerate() {
                        if i < column_widths.len() {
                            let text_len = cell.content.to_string().len();
                            column_widths[i] = column_widths[i].max(text_len);
                        }
                    }
                }

                // Add padding to columns
                for width in &mut column_widths {
                    *width += 2; // Add some padding
                }

                let table = Table::new(
                    rows,
                    column_widths
                        .iter()
                        .map(|&w| Constraint::Length(w as u16))
                        .collect::<Vec<_>>(),
                )
                .header(header_row)
                .block(Block::new().title("Data Preview").borders(Borders::ALL));

                f.render_widget(table, area);

                // Add status bar
                let status_text = format!(
                    "Rows: {} | Scroll: {}/{} | Step: {}",
                    df.height(),
                    self.scroll_offset + 1,
                    df.height(),
                    self.current_step_index + 1
                );
                let status = Line::from(Span::styled(
                    status_text,
                    Style::default().fg(Color::DarkGray),
                ));
                f.render_widget(status, area);
            }
            Err(_) => {
                let block = Block::new().title("Error").borders(Borders::ALL);
                f.render_widget(block, area);
            }
        }
    }

    fn get_current_dataframe(&self) -> polars::prelude::DataFrame {
        // Return the original source data for now
        match &self.pipeline.source {
            source if !source.is_empty() => {
                let mut df = polars::prelude::DataFrame::default();
                if let Ok(data) = load_data(source) {
                    df = data;
                }
                df
            }
            _ => polars::prelude::DataFrame::default(),
        }
    }
}
