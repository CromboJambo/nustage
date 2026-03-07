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
        Some("xlsx") => load_excel(file_path),
        Some("xls") => load_excel_legacy(file_path),
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

/// Load Excel file (.xlsx) using calamine
fn load_excel(file_path: &str) -> Result<DataFrame, PipelineError> {
    use calamine::open_workbook;

    let mut workbook: calamine::Xlsx<_> = open_workbook(file_path)
        .map_err(|e| PipelineError::DataLoadingError(format!("Excel open failed: {}", e)))?
        .ok_or_else(|| PipelineError::DataLoadingError("Failed to open Excel file".to_string()))?;

    // Read the first sheet with data
    let (range, _sheet_name) = workbook
        .sheets()
        .iter()
        .find_map(|(name, range)| {
            if !range.is_empty() {
                Some((range.clone(), name.as_str()))
            } else {
                None
            }
        })
        .ok_or_else(|| {
            PipelineError::DataLoadingError("No data found in Excel file".to_string())
        })?;

    // Collect headers from first row
    let mut headers: Vec<String> = range
        .rows()
        .next()
        .iter()
        .map(|cell| cell.to_string())
        .collect();

    if headers.is_empty() {
        return Err(PipelineError::DataLoadingError(
            "No header row found".to_string(),
        ));
    }

    // Collect all rows into a 2D vector for processing
    let mut rows: Vec<Vec<calamine::Cell>> = range.rows().collect();

    if rows.len() < 2 {
        return Err(PipelineError::DataLoadingError(
            "No data rows found".to_string(),
        ));
    }

    // Skip header row, process data rows
    let data_rows: Vec<Vec<calamine::Cell>> = rows.drain(1..).collect();

    // Build columns - determine types by scanning all values first
    let num_cols = headers.len().max(1);
    let mut col_values: Vec<Vec<String>> = (0..num_cols).map(|_| Vec::new()).collect();

    for row in &data_rows {
        for (col_idx, cell) in row.iter().enumerate() {
            if col_idx < num_cols {
                col_values[col_idx].push(cell.to_string());
            }
        }
    }

    // Build DataFrame from collected data
    let mut columns: Vec<Series> = Vec::new();

    for (col_idx, values) in col_values.iter().enumerate() {
        if headers.is_empty() || col_idx >= headers.len() {
            continue;
        }

        let name = &headers[col_idx];

        // Try to infer type - start with numeric parsing
        let mut all_numeric = true;
        let mut has_floats = false;

        for val in values {
            if let Ok(_) = val.parse::<i64>() {
                continue;
            } else if let Ok(f) = val.parse::<f64>() {
                has_floats = true;
                continue;
            } else if val.is_empty() {
                continue;
            }
            all_numeric = false;
            break;
        }

        let series: Series = if all_numeric {
            // Create numeric series - try i64 first, fall back to f64
            let int_values: Vec<Option<i64>> = values
                .iter()
                .map(|v| v.parse::<i64>().ok().or_else(|| val_to_i64(v)))
                .collect();

            Series::new(name.as_str(), &int_values)
        } else if !values.is_empty() && has_floats {
            // Float series
            let float_values: Vec<Option<f64>> = values
                .iter()
                .map(|v| v.parse::<f64>().ok().or_else(|| val_to_f64(v)))
                .collect();

            Series::new(name.as_str(), &float_values)
        } else {
            // String series - keep as strings
            let str_values: Vec<&str> = values.iter().map(|v| v.as_str()).collect();
            Series::new(name.as_str(), str_values.as_slice())
        };

        columns.push(series);
    }

    DataFrame::new(columns).map_err(|e| PipelineError::DataLoadingError(e.to_string()))
}

/// Try to convert a value string to i64 (handles various formats)
fn val_to_i64(val: &str) -> Option<i64> {
    // Handle currency symbols, commas in numbers, etc.
    let cleaned = val.replace('$', "").replace(',', "");
    cleaned.parse::<i64>().ok()
}

/// Try to convert a value string to f64 (handles various formats)
fn val_to_f64(val: &str) -> Option<f64> {
    // Handle currency symbols, commas in numbers, etc.
    let cleaned = val.replace('$', "").replace(',', "");
    cleaned.parse::<f64>().ok()
}

/// Load legacy Excel file (.xls) using calamine
fn load_excel_legacy(file_path: &str) -> Result<DataFrame, PipelineError> {
    use calamine::open_workbook;

    let mut workbook = open_workbook(file_path)
        .map_err(|e| PipelineError::DataLoadingError(format!("Excel open failed: {}", e)))?
        .ok_or_else(|| PipelineError::DataLoadingError("Failed to open Excel file".to_string()))?;

    // Read the first sheet with data
    let (range, _sheet_name) = workbook
        .sheets()
        .iter()
        .find_map(|(name, range)| {
            if !range.is_empty() {
                Some((range.clone(), name.as_str()))
            } else {
                None
            }
        })
        .ok_or_else(|| {
            PipelineError::DataLoadingError("No data found in Excel file".to_string())
        })?;

    // Collect headers from first row
    let mut headers: Vec<String> = range
        .rows()
        .next()
        .iter()
        .map(|cell| cell.to_string())
        .collect();

    if headers.is_empty() {
        return Err(PipelineError::DataLoadingError(
            "No header row found".to_string(),
        ));
    }

    // Collect all rows into a 2D vector for processing
    let mut rows: Vec<Vec<calamine::Cell>> = range.rows().collect();

    if rows.len() < 2 {
        return Err(PipelineError::DataLoadingError(
            "No data rows found".to_string(),
        ));
    }

    // Skip header row, process data rows
    let data_rows: Vec<Vec<calamine::Cell>> = rows.drain(1..).collect();

    // Build columns - determine types by scanning all values first
    let num_cols = headers.len().max(1);
    let mut col_values: Vec<Vec<String>> = (0..num_cols).map(|_| Vec::new()).collect();

    for row in &data_rows {
        for (col_idx, cell) in row.iter().enumerate() {
            // Convert cell to string value - calamine Cell has value() method
            let str_val = match cell.value() {
                calamine::CellValue::Int(v) => v.to_string(),
                calamine::CellValue::Float(v) => format!("{:.2}", v),
                calamine::CellValue::String(s) => s.clone(),
                _ => "".to_string(),
            };

            // Store in column values vector
            if col_idx < num_cols {
                col_values[col_idx].push(str_val);
            }
        }
    }

    // Build DataFrame from collected data
    let mut columns: Vec<Series> = Vec::new();

    for (col_idx, values) in col_values.iter().enumerate() {
        if headers.is_empty() || col_idx >= headers.len() {
            continue;
        }

        let name = &headers[col_idx];

        // Try to infer type - start with numeric parsing
        let mut all_numeric = true;
        let mut has_floats = false;

        for val in values {
            if let Ok(_) = val.parse::<i64>() {
                continue;
            } else if let Ok(f) = val.parse::<f64>() {
                has_floats = true;
                continue;
            } else if val.is_empty() {
                continue;
            }
            all_numeric = false;
            break;
        }

        let series: Series = if all_numeric {
            // Create numeric series - try i64 first, fall back to f64
            let int_values: Vec<Option<i64>> = values
                .iter()
                .map(|v| v.parse::<i64>().ok().or_else(|| val_to_i64(v)))
                .collect();

            Series::new(name.as_str(), &int_values)
        } else if !values.is_empty() && has_floats {
            // Float series
            let float_values: Vec<Option<f64>> = values
                .iter()
                .map(|v| v.parse::<f64>().ok().or_else(|| val_to_f64(v)))
                .collect();

            Series::new(name.as_str(), &float_values)
        } else {
            // String series - keep as strings
            let str_values: Vec<&str> = values.iter().map(|v| v.as_str()).collect();
            Series::new(name.as_str(), str_values.as_slice())
        };

        columns.push(series);
    }

    DataFrame::new(columns).map_err(|e| PipelineError::DataLoadingError(e.to_string()))
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
