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

/// Load data from various file formats (simplified for now)
pub fn load_data(file_path: &str) -> Result<DataFrame, PipelineError> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(PipelineError::FileNotFound(file_path.to_string()));
    }

    match path.extension().and_then(|s| s.to_str()) {
        Some("csv") => load_csv(file_path),
        Some("xlsx") | Some("xls") => load_excel(file_path),
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

/// Load Excel file (.xlsx or .xls) using calamine
fn load_excel(file_path: &str) -> Result<DataFrame, PipelineError> {
    use calamine::{Reader, Xls, Xlsx};

    // Try to open as xlsx first
    let mut workbook: Option<Xlsx<_>> = match Xlsx::open(file_path) {
        Ok(wb) => Some(wb),
        Err(_) => None,
    };

    // If that fails, try xls
    if workbook.is_none() {
        workbook = match Xls::open(file_path) {
            Ok(wb) => Some(wb),
            Err(_) => {
                return Err(PipelineError::DataLoadingError(
                    "Failed to open Excel file - not a valid xlsx or xls".to_string(),
                ));
            }
        };
    }

    let mut workbook = workbook.unwrap();

    let mut headers: Vec<String> = Vec::new();
    let mut data_rows: Vec<Vec<String>> = Vec::new();

    // Read first worksheet range
    if let Some(range) = workbook.worksheet_range_at(0)? {
        for (row_idx, row) in range.rows().enumerate() {
            if row_idx == 0 {
                headers = row.iter().map(|cell| cell.to_string()).collect();
            } else {
                data_rows.push(row.iter().map(|cell| cell.to_string()).collect());
            }
        }
    }

    if headers.is_empty() || data_rows.is_empty() {
        return Err(PipelineError::DataLoadingError(
            "Empty Excel file".to_string(),
        ));
    }

    // Build series for each column (same logic as xlsx)
    let mut columns: Vec<Column> = Vec::new();
    for (i, header) in headers.iter().enumerate() {
        let values: Vec<Option<&str>> = data_rows
            .iter()
            .map(|row| row.get(i).map(|s| s.as_str()))
            .collect();

        // Try to infer type from first non-empty value
        let series: Column = if values
            .iter()
            .all(|v| v.is_none() || v.unwrap().parse::<i64>().is_ok())
        {
            let ints: Vec<i64> = values
                .iter()
                .map(|v| v.and_then(|s| s.parse().ok()).unwrap_or(0))
                .collect();
            Series::new(header.as_str(), &ints).into()
        } else if values
            .iter()
            .all(|v| v.is_none() || v.unwrap().parse::<f64>().is_ok())
        {
            let floats: Vec<f64> = values
                .iter()
                .map(|v| v.and_then(|s| s.parse().ok()).unwrap_or(0.0))
                .collect();
            Series::new(header.as_str(), &floats).into()
        } else {
            Series::new(
                header.as_str(),
                &values.iter().map(|v| v.unwrap_or("")).collect::<Vec<_>>(),
            )
            .into()
        };

        columns.push(series);
    }

    DataFrame::new(data_rows.len(), columns)
        .map_err(|e| PipelineError::DataLoadingError(format!("Failed to create DataFrame: {}", e)))
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

/// Helper function to parse string values from Excel cells

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

pub fn get_unique_values(
    _df: &DataFrame,
    _column_name: &str,
) -> Result<Vec<String>, PipelineError> {
    // For now, we'll return a placeholder result to avoid complex implementation issues
    Ok(vec!["placeholder".to_string()])
}

pub fn get_column_stats(_df: &DataFrame, _column_name: &str) -> Result<ColumnStats, PipelineError> {
    // For now, we'll return a placeholder result to avoid complex implementation issues
    Ok(ColumnStats {
        data_type: "placeholder".to_string(),
        min: "N/A".to_string(),
        max: "N/A".to_string(),
        sum: "N/A".to_string(),
        count: 0,
    })
}

#[derive(Debug, Clone)]
pub struct ColumnSchema {
    pub index: usize,
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Clone)]
pub struct ColumnStats {
    pub data_type: String,
    pub min: String,
    pub max: String,
    pub sum: String,
    pub count: usize,
}
