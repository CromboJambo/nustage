use crate::Pipeline;
use crate::data::{get_schema, load_data};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};
use std::io;
use std::time::Duration;

pub struct App {
    pub pipeline: Pipeline,
    pub current_step_index: usize,
    pub selected_column: Option<String>,
}

impl App {
    pub fn new(pipeline: Pipeline) -> Self {
        Self {
            pipeline,
            current_step_index: 0,
            selected_column: None,
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
                    Event::Key(key) => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Down => {
                            if self.current_step_index < self.pipeline.steps.len() - 1 {
                                self.current_step_index += 1;
                            }
                        }
                        KeyCode::Up => {
                            if self.current_step_index > 0 {
                                self.current_step_index -= 1;
                            }
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
        // For now we'll just show the current step's data
        if self.pipeline.steps.is_empty() {
            let block = Block::new().title("No Data").borders(Borders::ALL);

            f.render_widget(block, area);
            return;
        }

        // Get schema for column information
        match get_schema(&self.get_current_dataframe()) {
            Ok(schema) => {
                // Create header row
                let headers: Vec<Cell> = schema
                    .iter()
                    .map(|col| Cell::from(col.name.clone()))
                    .collect();

                let header_row = Row::new(headers).style(Style::default().fg(Color::Yellow));

                // Create data rows (just first few rows for demo)
                let mut rows: Vec<Row> = vec![];

                // For simplicity, just show column names and some sample data
                if !schema.is_empty() {
                    let sample_rows = 10.min(self.get_current_dataframe().height());
                    for i in 0..sample_rows {
                        let cells: Vec<Cell> = schema
                            .iter()
                            .map(|col| {
                                // Just show a placeholder value for now
                                Cell::from(format!("Row{}", i))
                            })
                            .collect();

                        rows.push(Row::new(cells));
                    }
                }

                let table = Table::new(
                    rows,
                    schema
                        .iter()
                        .map(|_| Constraint::Percentage(20))
                        .collect::<Vec<_>>(),
                )
                .header(header_row)
                .block(Block::new().title("Data Preview").borders(Borders::ALL));

                f.render_widget(table, area);
            }
            Err(_) => {
                let block = Block::new().title("Error").borders(Borders::ALL);

                f.render_widget(block, area);
            }
        }
    }

    fn get_current_dataframe(&self) -> polars::prelude::DataFrame {
        // Try to load original source file if available
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
