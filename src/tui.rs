//! TUI Main Entry Point for Nustage
//!
//! This module implements the terminal user interface for exploring and manipulating data.

use clap::Parser;
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
    layout::{Constraint, Layout, Rect},
    terminal::Terminal,
    widgets::{Block, Borders, Paragraph, Table, TableState},
};
use std::{
    io::{self, stdout},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file path (CSV, Excel, Parquet)
    #[arg(value_name = "FILE")]
    input: PathBuf,
}

/// Main TUI application structure
pub struct App {
    /// Data frame being displayed
    data: DataFrame,
    /// Schema information
    schema: Vec<nustage::data::ColumnSchema>,
    /// Current grid configuration
    grid_config: nustage::tui_grid::GridConfig,
    /// Current grid state
    grid_state: nustage::tui_grid::GridState,
    /// Is the application running?
    running: bool,
}

impl App {
    /// Create a new app instance
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

    /// Handle key events
    pub fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                    KeyCode::Down => {
                        // Scroll down in grid
                        self.grid_state.table_state.select_next();
                    }
                    KeyCode::Up => {
                        // Scroll up in grid
                        self.grid_state.table_state.select_previous();
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Render the UI
    pub fn render(&mut self, frame: &mut Frame) {
        let size = frame.size();

        // Create layout
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage(80), // Main grid area
                Constraint::Percentage(20), // Status bar
            ])
            .split(size);

        // Render grid display
        nustage::tui_grid::render_grid_display(
            frame,
            chunks[0],
            &self.data,
            &self.grid_config,
            &self.grid_state,
        );

        // Render status bar
        let status_text = format!(
            "Data: {} rows × {} columns | Schema: {} fields",
            self.data.height(),
            self.data.width(),
            self.schema.len()
        );

        let status_block = Block::default().title("Status").borders(Borders::ALL);

        let status_paragraph = Paragraph::new(status_text).block(status_block);

        frame.render_widget(status_paragraph, chunks[1]);
    }
}

/// Run the TUI application
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize terminal
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Create app instance
    let mut app = App::new(&cli.input)?;

    // Main loop
    loop {
        terminal.draw(|frame| app.render(frame))?;

        if let Event::Key(key) = event::read()? {
            app.handle_event(key)?;

            if !app.running {
                break;
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("TUI mode - File path received: {}", cli.input.display());

    // Run the TUI application
    run()
}
