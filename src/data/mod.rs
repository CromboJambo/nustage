//! Data loading and processing module for Nustage
//!
//! This module provides functionality for loading, processing, and analyzing
//! data from various formats including CSV, Excel, and Parquet files.

use polars::prelude::*;
use std::path::Path;

// Define the error type for data loading operations
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Data loading error: {0}")]
    DataLoadingError(String),
}

/// Column schema information
#[derive(Debug, Clone)]
pub struct ColumnSchema {
    pub index: usize,
    pub name: String,
    pub data_type: String,
}

/// Load data from various file formats
pub fn load_data(file_path: &str) -> Result<DataFrame, PipelineError> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(PipelineError::FileNotFound(file_path.to_string()));
    }

    match path.extension().and_then(|s| s.to_str()) {
        Some("csv") => load_csv(file_path),
        Some("xlsx") | Some("xls") => load_excel_placeholder(),
        Some("parquet") => load_parquet(file_path),
        _ => Err(PipelineError::UnsupportedFormat(file_path.to_string())),
    }
}

/// Load CSV file
fn load_csv(file_path: &str) -> Result<DataFrame, PipelineError> {
    CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(file_path.into()))
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?
        .finish()
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))
}

/// Load Excel file (.xlsx or .xls) - placeholder implementation
fn load_excel_placeholder() -> Result<DataFrame, PipelineError> {
    Err(PipelineError::UnsupportedFormat(
        "Excel loading not implemented".to_string(),
    ))
}

/// Load Parquet file using polars
fn load_parquet(file_path: &str) -> Result<DataFrame, PipelineError> {
    use polars::prelude::*;

    let file = std::fs::File::open(file_path).map_err(|e| {
        PipelineError::DataLoadingError(format!("Failed to open parquet file: {}", e))
    })?;

    ParquetReader::new(file)
        .finish()
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))
}

/// Get schema information for a DataFrame
pub fn get_schema(df: &DataFrame) -> Result<Vec<ColumnSchema>, PipelineError> {
    let schema = df.schema();

    let columns: Vec<ColumnSchema> = schema
        .iter()
        .enumerate()
        .map(|(i, (name, dtype))| ColumnSchema {
            index: i,
            name: name.to_string(),
            data_type: dtype.to_string(),
        })
        .collect();

    Ok(columns)
}
