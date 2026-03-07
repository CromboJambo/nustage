//! Nustage - A data processing and analysis toolkit for Rust
//!
//! This library provides functionality for loading, processing, and analyzing
//! data from various formats including CSV, Excel, and Parquet files.

pub mod cli;
pub mod data;
pub mod ironcalc;
pub mod mcode;
pub mod sidecar;
pub mod transformations;
pub mod tui_grid;

// Re-export commonly used types for convenience
pub use cli::Cli;
pub use data::{ColumnSchema, PipelineError, get_schema, load_data};
pub use ironcalc::{
    IronCalcError, IronCalcIntegration, SpreadsheetCell, SpreadsheetColumn, SpreadsheetRow,
    Transformation, apply_transformation, get_field_names, get_resolved_value,
    load_dataframe_to_ironcalc,
};
pub use mcode::generate_m_code;
pub use sidecar::{SidecarError, SidecarFile, SidecarMetadata};

pub use transformations::{
    Aggregation, AggregationOperation, ColumnSchema as TransformColumnSchema, StepType,
    TransformationError, TransformationFactory, TransformationPipeline, TransformationStep,
    deserialize_pipeline, get_available_transformations, serialize_pipeline,
};
pub use tui_grid::{GridConfig, GridState, create_basic_grid_display, render_grid_display};
