use calamine::{Reader, Xls, Xlsx, open_workbook};
use chrono::{NaiveDate, NaiveDateTime};
use polars::chunked_array::cast::CastOptions;
use polars::prelude::*;
use std::collections::HashSet;
use std::fs::File;
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

/// Load data from various file formats
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

/// Load Excel file (.xlsx)
fn load_excel(file_path: &str) -> Result<DataFrame, PipelineError> {
    let mut workbook: Xlsx<_> =
        open_workbook(file_path).map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| PipelineError::DataLoadingError("No sheets found".to_string()))?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    // Convert calamine Range to Polars DataFrame
    let df = range_to_dataframe(range)?;
    Ok(df)
}

/// Load legacy Excel file (.xls)
fn load_excel_legacy(file_path: &str) -> Result<DataFrame, PipelineError> {
    let mut workbook: Xls<_> =
        open_workbook(file_path).map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| PipelineError::DataLoadingError("No sheets found".to_string()))?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    // Convert calamine Range to Polars DataFrame
    let df = range_to_dataframe(range)?;
    Ok(df)
}

/// Convert calamine Range to Polars DataFrame
fn range_to_dataframe(range: calamine::Range) -> Result<DataFrame, PipelineError> {
    use calamine::DataType;

    if range.height() == 0 || range.width() == 0 {
        return Err(PipelineError::DataLoadingError("Empty sheet".to_string()));
    }

    let mut columns: Vec<Column> = vec![];
    let num_rows = range.height() as usize;
    let num_cols = range.width() as usize;

    // Get column names from the first row
    let col_names: Vec<String> = (0..num_cols)
        .map(|col| {
            if let Some(cell) = range.get_value(0, col) {
                match cell {
                    DataType::String(s) => s.clone(),
                    _ => format!("column_{}", col),
                }
            } else {
                format!("column_{}", col)
            }
        })
        .collect();

    // Process each column
    for col in 0..num_cols {
        let mut values: Vec<String> = vec![];

        for row in 1..num_rows {
            if let Some(cell) = range.get_value(row, col) {
                match cell {
                    DataType::String(s) => values.push(s.clone()),
                    DataType::Float(f) => values.push(f.to_string()),
                    DataType::Int(i) => values.push(i.to_string()),
                    DataType::Bool(b) => values.push(b.to_string()),
                    _ => values.push("".to_string()),
                }
            } else {
                values.push("".to_string());
            }
        }

        // Create Polars column from strings
        let series = Series::new(&col_names[col], &values);
        columns.push(series);
    }

    DataFrame::from_columns(columns).map_err(|e| PipelineError::DataLoadingError(e.to_string()))
}

/// Load Parquet file
fn load_parquet(file_path: &str) -> Result<DataFrame, PipelineError> {
    let mut file = File::open(file_path).map_err(|e| PipelineError::FileNotFound(e.to_string()))?;

    let df = DataFrame::read_parquet(&mut file, ParquetReaderOptions::default())
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    Ok(df)
}

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

pub fn get_unique_values(df: &DataFrame, column_name: &str) -> Result<Vec<String>, PipelineError> {
    let series = df
        .column(column_name)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    // Get unique values from the series
    let mut unique_values: HashSet<String> = HashSet::new();

    match series.dtype() {
        DataType::Utf8 => {
            for i in 0..series.len() {
                if let Ok(value) = series.get(i) {
                    match value {
                        AnyValue::String(s) => {
                            unique_values.insert(s.to_string());
                        }
                        _ => {
                            unique_values.insert("NULL".to_string());
                        }
                    }
                }
            }
        }
        _ => {
            // For other types, we'll just convert to string
            for i in 0..series.len() {
                if let Ok(value) = series.get(i) {
                    match value {
                        AnyValue::Null => unique_values.insert("NULL".to_string()),
                        _ => unique_values.insert(format!("{}", value)),
                    };
                }
            }
        }
    }

    // Convert HashSet to Vec
    let mut result: Vec<String> = unique_values.into_iter().collect();
    result.sort(); // Sort for consistent output

    Ok(result)
}

pub fn get_column_stats(df: &DataFrame, column_name: &str) -> Result<ColumnStats, PipelineError> {
    let series = df
        .column(column_name)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    match series.dtype() {
        DataType::Utf8 => {
            // For string columns
            let mut count = 0;
            let mut unique_count = 0;

            let mut unique_values: HashSet<String> = HashSet::new();

            for i in 0..series.len() {
                if let Ok(value) = series.get(i) {
                    match value {
                        AnyValue::String(s) => {
                            count += 1;
                            unique_values.insert(s.to_string());
                        }
                        _ => {
                            count += 1; // Count non-null values
                        }
                    }
                }
            }

            unique_count = unique_values.len();

            Ok(ColumnStats {
                data_type: series.dtype().to_string(),
                min: "N/A".to_string(),
                max: "N/A".to_string(),
                sum: "N/A".to_string(),
                count,
            })
        }
        _ => {
            // For numeric columns, we can get more stats
            Ok(ColumnStats {
                data_type: series.dtype().to_string(),
                min: "N/A".to_string(),
                max: "N/A".to_string(),
                sum: "N/A".to_string(),
                count: series.len(),
            })
        }
    }
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
