//! CLI module for nustage data processing tool

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Nustage - Data Processing and Analysis Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file path (CSV, Excel, Parquet)
    #[arg(value_name = "FILE")]
    pub input: PathBuf,

    /// Output file path for saving results
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Data format (auto-detect if not specified)
    #[arg(short, long)]
    pub format: Option<String>,

    /// Show detailed schema information
    #[arg(short, long)]
    pub schema: bool,

    /// Show data preview (number of rows)
    #[arg(short, long, default_value = "10")]
    pub preview: usize,

    /// TUI mode for interactive data exploration
    #[arg(short, long, conflicts_with = "output")]
    pub tui: bool,

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcommands for nustage
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Analyze data statistics
    Analyze {
        /// Column to analyze
        #[arg(short, long)]
        column: Option<String>,
    },

    /// Get unique values from a column
    Unique {
        /// Column name
        #[arg(short, long)]
        column: String,
    },

    /// Show data schema
    Schema,

    /// Export data to a specific format
    Export {
        /// Output format (csv, json, parquet)
        #[arg(short, long)]
        format: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },
}

/// Parse command line arguments
pub fn parse_args() -> Cli {
    Cli::parse()
}

/// Validate input file path
pub fn validate_input(input: &PathBuf) -> Result<(), String> {
    if !input.exists() {
        return Err(format!("File not found: {}", input.display()));
    }

    if !input.is_file() {
        return Err(format!("Not a file: {}", input.display()));
    }

    Ok(())
}

/// Get file extension
pub fn get_file_extension(path: &PathBuf) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}
