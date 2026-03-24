//! TUI module for Nustage
//!
//! This module implements the terminal user interface for exploring and
//! manipulating data. Call [`run`] from `main.rs` to launch the interactive
//! session.

use nustage::data::{get_schema, load_data};
use polars::prelude::*;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    frame::Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::{
    io::stdout,
    path::PathBuf,
};

/// Main TUI application structure.
pub struct App {
    /// Data frame being displayed.
    data: DataFrame,
    /// Schema information.
    schema: Vec<nustage::data::ColumnSchema>,
    /// Current grid configuration.
    grid_config: nustage::tui_grid::GridConfig,
    /// Current grid state.
    grid_state: nustage::tui_grid::GridState,
    /// Is the application running?
    running: bool,
}

impl App {
    /// Create a new app instance from a file path.
    pub fn new(input_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let data = load_data(&input_path.to_string_lossy())?;
        let schema = get_schema(&data)?;

        let grid_config = nustage::tui_grid::GridConfig::default();
        let grid_state = nustage::tui_grid::GridState::default();

        Ok(Self {
            data,
            schema,
            grid_config,
            grid_state,
            running: true,
        })
    }

    /// Handle a terminal event.
    pub fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                KeyCode::Down => self.grid_state.table_state.select_next(),
                KeyCode::Up => self.grid_state.table_state.select_previous(),
                _ => {}
            }
        }
        Ok(())
    }

    /// Render the UI into the given frame.
    pub fn render(&mut self, frame: &mut Frame) {
        let size = frame.area();

        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage(80), // Main grid area
                Constraint::Percentage(20), // Status bar
            ])
            .split(size);

        nustage::tui_grid::render_grid_display(
            frame,
            chunks[0],
            &self.data,
            &self.grid_config,
            &self.grid_state,
        );

        let status_text = format!(
            "Data: {} rows × {} columns | Schema: {} fields",
            self.data.height(),
            self.data.width(),
            self.schema.len()
        );

        let status_paragraph = Paragraph::new(status_text)
            .block(Block::default().title("Status").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));

        frame.render_widget(status_paragraph, chunks[1]);
    }
}

/// Run the TUI application with the given input file.
pub fn run(input_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;

    let mut app = App::new(input_path)?;

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|frame| app.render(frame))?;

            let ev = event::read()?;
            app.handle_event(ev)?;

            if !app.running {
                break;
            }
        }
        Ok(())
    })();

    // Always restore the terminal, even on error.
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    result
}
