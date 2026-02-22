use crate::PipelineError;
use calamine::{open_workbook, Range, Reader, Xls, Xlsx, XmlReader};
use polars::prelude::*;
use std::path::Path;

/// Load data from various file formats
pub fn load_data(file_path: &str) -> Result<DataFrame, PipelineError> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(PipelineError::FileNotFound(file_path.to_string()));
    }

    // Try different file formats
    if path.extension().and_then(|s| s.to_str()) == Some("csv") {
        load_csv(file_path)
    } else if path.extension().and_then(|s| s.to_str()) == Some("xlsx") {
        load_excel(file_path)
    } else if path.extension().and_then(|s| s.to_str()) == Some("xls") {
        load_excel_legacy(file_path)
    } else if path.extension().and_then(|s| s.to_str()) == Some("parquet") {
        load_parquet(file_path)
    } else {
        Err(PipelineError::UnsupportedFormat(file_path.to_string()))
    }
}

/// Load CSV file
fn load_csv(file_path: &str) -> Result<DataFrame, PipelineError> {
    let df = CsvReader::from_path(file_path)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?
        .infer_schema(true)
        .finish()
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    Ok(df)
}

/// Load Excel file (.xlsx)
fn load_excel(file_path: &str) -> Result<DataFrame, PipelineError> {
    let mut workbook: Xlsx<_> =
        open_workbook(file_path).map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .ok_or_else(|| PipelineError::DataLoadingError("No sheets found".to_string()))?;

    let range = workbook.worksheet_range(sheet_name).ok_or_else(|| {
        PipelineError::DataLoadingError(format!("Failed to read sheet: {}", sheet_name))
    })?;

    // Convert Excel range to Polars DataFrame
    let rows: Vec<Row> = range
        .rows()
        .map(|row| row.iter().map(|cell| cell.to_string()).collect())
        .collect();

    let headers: Vec<&str> = rows
        .first()
        .unwrap_or(&vec![])
        .iter()
        .map(|s| s.as_str())
        .collect();
    let data: Vec<Vec<&str>> = rows
        .iter()
        .skip(1)
        .map(|r| r.iter().map(|s| s.as_str()).collect())
        .collect();

    let df = DataFrame::from_rows(&data)
        .with_column_name("source", headers)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    Ok(df)
}

/// Load Excel file (.xls) - legacy format
fn load_excel_legacy(file_path: &str) -> Result<DataFrame, PipelineError> {
    let mut workbook: Xls<_> =
        open_workbook(file_path).map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .ok_or_else(|| PipelineError::DataLoadingError("No sheets found".to_string()))?;

    let range = workbook.worksheet_range(sheet_name).ok_or_else(|| {
        PipelineError::DataLoadingError(format!("Failed to read sheet: {}", sheet_name))
    })?;

    // Convert Excel range to Polars DataFrame
    let rows: Vec<Row> = range
        .rows()
        .map(|row| row.iter().map(|cell| cell.to_string()).collect())
        .collect();

    let headers: Vec<&str> = rows
        .first()
        .unwrap_or(&vec![])
        .iter()
        .map(|s| s.as_str())
        .collect();
    let data: Vec<Vec<&str>> = rows
        .iter()
        .skip(1)
        .map(|r| r.iter().map(|s| s.as_str()).collect())
        .collect();

    let df = DataFrame::from_rows(&data)
        .with_column_name("source", headers)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    Ok(df)
}

/// Load Parquet file
fn load_parquet(file_path: &str) -> Result<DataFrame, PipelineError> {
    let df = ParquetReader::new(file_path)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?
        .finish()
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    Ok(df)
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

/// Get unique values for a column
pub fn get_unique_values(df: &DataFrame, column: &str) -> Result<Vec<String>, PipelineError> {
    let col = df
        .column(column)
        .ok_or_else(|| PipelineError::DataLoadingError(format!("Column not found: {}", column)))?;

    let unique_values = col
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<&str>>()
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    Ok(unique_values)
}

/// Get column statistics
pub fn get_column_stats(df: &DataFrame, column: &str) -> Result<ColumnStats, PipelineError> {
    let col = df
        .column(column)
        .ok_or_else(|| PipelineError::DataLoadingError(format!("Column not found: {}", column)))?;

    let dtype = col.dtype();

    let stats = match dtype {
        DataType::Int32 => {
            let min = col.min().unwrap_or(0);
            let max = col.max().unwrap_or(0);
            let sum = col.sum().unwrap_or(0);
            ColumnStats {
                data_type: dtype.to_string(),
                min: Some(min),
                max: Some(max),
                sum: Some(sum),
                count: col.len(),
            }
        }
        DataType::Float64 => {
            let min = col.min().unwrap_or(0.0);
            let max = col.max().unwrap_or(0.0);
            let sum = col.sum().unwrap_or(0.0);
            ColumnStats {
                data_type: dtype.to_string(),
                min: Some(min),
                max: Some(max),
                sum: Some(sum),
                count: col.len(),
            }
        }
        DataType::String => ColumnStats {
            data_type: dtype.to_string(),
            min: None,
            max: None,
            sum: None,
            count: col.len(),
        },
        _ => ColumnStats {
            data_type: dtype.to_string(),
            min: None,
            max: None,
            sum: None,
            count: col.len(),
        },
    };

    Ok(stats)
}

/// Column schema information
#[derive(Debug, Clone)]
pub struct ColumnSchema {
    pub index: usize,
    pub name: String,
    pub data_type: String,
}

/// Column statistics
#[derive(Debug, Clone)]
pub struct ColumnStats {
    pub data_type: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub sum: Option<f64>,
    pub count: usize,
}
