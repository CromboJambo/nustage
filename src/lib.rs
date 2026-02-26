//! Nustage - A data processing and analysis toolkit for Rust
//!
//! This library provides functionality for loading, processing, and analyzing
//! data from various formats including CSV, Excel, and Parquet files.

pub mod cli;
pub mod data;
pub mod tui_grid;

// Re-export commonly used types for convenience
pub use cli::Cli;
pub use data::{ColumnSchema, PipelineError, get_schema, load_data};
pub use tui_grid::{GridConfig, GridState, create_basic_grid_display, render_grid_display};
