use clap::Parser;
use std::path::PathBuf;
use thiserror::Error;
mod data;

use crate::data::{detect_and_apply_types, get_schema, load_data};

/// Nustage: Terminal-first spreadsheet transformation tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file path (CSV, Excel, Parquet)
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Show generated SQL for transparency
    #[arg(short, long)]
    show_sql: bool,

    /// Run in interactive TUI mode
    #[arg(short, long)]
    tui: bool,
}

/// Pipeline state for transformations
#[derive(Debug, Clone)]
pub struct Pipeline {
    /// Original data source
    pub source: String,

    /// Step list (immutable transformations)
    pub steps: Vec<Step>,

    /// Stationary filters (slicer/timeline style)
    pub filters: Vec<Filter>,

    /// Current step index (for preview)
    pub current_step: usize,
}

/// Transformation step
#[derive(Debug, Clone)]
pub struct Step {
    pub name: String,
    pub description: String,
    pub transform_type: TransformType,
    pub parameters: Vec<Parameter>,
}

/// Type of transformation
#[derive(Debug, Clone)]
pub enum TransformType {
    Source,
    DetectTypes,
    Filter,
    AddColumn,
    SelectColumns,
    GroupBy,
    Sort,
    CustomSql,
}

/// Parameter for transformation
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

/// Stationary filter (slicer/timeline style)
#[derive(Debug, Clone)]
pub struct Filter {
    pub id: String,
    pub column: String,
    pub filter_type: FilterType,
    pub values: Vec<String>,
}

/// Type of filter
#[derive(Debug, Clone)]
pub enum FilterType {
    Slicer,
    Timeline,
    Custom,
}

/// Pipeline errors
#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("Data loading error: {0}")]
    DataLoadingError(String),

    #[error("SQL generation error: {0}")]
    SqlError(String),

    #[error("Invalid step: {0}")]
    InvalidStep(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Main entry point
fn main() -> Result<(), PipelineError> {
    let cli = Cli::parse();

    // Load initial pipeline
    let mut pipeline = Pipeline {
        source: cli.input.to_string_lossy().to_string(),
        steps: Vec::new(),
        filters: Vec::new(),
        current_step: 0,
    };

    // Load data and create initial source step
    println!("Loading data from: {}", pipeline.source);
    let loaded_df = load_data(&pipeline.source)?;

    println!(
        "Loaded {} rows with {} columns",
        loaded_df.height(),
        loaded_df.width()
    );

    // Create source step
    pipeline.steps.push(Step {
        name: "Source".to_string(),
        description: format!("Loaded from {}", pipeline.source),
        transform_type: TransformType::Source,
        parameters: vec![Parameter {
            name: "source_file".to_string(),
            value: pipeline.source.clone(),
        }],
    });

    // Detect and apply best-effort column types by default.
    let (df, type_changes) = detect_and_apply_types(&loaded_df)?;

    if type_changes.is_empty() {
        println!("Type detection: no changes");
    } else {
        println!("Type detection: {} column(s) updated", type_changes.len());
        for change in &type_changes {
            println!(
                "  - {}: {} -> {}",
                change.column, change.from_type, change.to_type
            );
        }
    }

    pipeline.steps.push(Step {
        name: "Detect Types".to_string(),
        description: if type_changes.is_empty() {
            "Auto type detection (no inferred changes)".to_string()
        } else {
            format!(
                "Auto type detection updated {} column(s)",
                type_changes.len()
            )
        },
        transform_type: TransformType::DetectTypes,
        parameters: vec![Parameter {
            name: "changes".to_string(),
            value: type_changes.len().to_string(),
        }],
    });

    // Get schema information
    let schema = get_schema(&df)?;
    println!("Schema:");
    for col in &schema {
        println!("  - {} ({})", col.name, col.data_type);
    }

    // TODO: Process commands
    // TODO: Generate SQL if requested

    if cli.tui {
        println!("TUI mode is not fully implemented yet");
        println!("Use CLI mode for now.");
    } else {
        // CLI mode
        println!("Steps: {}", pipeline.steps.len());
        println!("Filters: {}", pipeline.filters.len());
    }

    Ok(())
}
