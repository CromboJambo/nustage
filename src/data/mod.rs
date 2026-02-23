use crate::PipelineError;
use calamine::{Reader, Xls, Xlsx, open_workbook};
use chrono::{NaiveDate, NaiveDateTime};
use polars::chunked_array::cast::CastOptions;
use polars::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;

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
    let mut workbook: Xlsx<_> = open_workbook(file_path)
        .map_err(|e: calamine::XlsxError| PipelineError::DataLoadingError(e.to_string()))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| PipelineError::DataLoadingError("No sheets found".to_string()))?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    range_to_dataframe(&range)
}

/// Load Excel file (.xls) - legacy format
fn load_excel_legacy(file_path: &str) -> Result<DataFrame, PipelineError> {
    let mut workbook: Xls<_> = open_workbook(file_path)
        .map_err(|e: calamine::XlsError| PipelineError::DataLoadingError(e.to_string()))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| PipelineError::DataLoadingError("No sheets found".to_string()))?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    range_to_dataframe(&range)
}

/// Convert an Excel range into a DataFrame, treating each cell as text.
fn range_to_dataframe(range: &calamine::Range<calamine::Data>) -> Result<DataFrame, PipelineError> {
    let rows: Vec<Vec<String>> = range
        .rows()
        .map(|row| row.iter().map(ToString::to_string).collect())
        .collect();

    if rows.is_empty() {
        return Ok(DataFrame::default());
    }

    let raw_headers = rows[0].clone();
    let width = raw_headers.len();
    let height = rows.len().saturating_sub(1);

    let mut seen = std::collections::HashMap::<String, usize>::new();
    let headers: Vec<String> = raw_headers
        .iter()
        .enumerate()
        .map(|(idx, name)| {
            let base = if name.trim().is_empty() {
                format!("column_{}", idx + 1)
            } else {
                name.clone()
            };

            let count = seen.entry(base.clone()).or_insert(0);
            let resolved = if *count == 0 {
                base
            } else {
                format!("{}_{}", base, *count + 1)
            };
            *count += 1;
            resolved
        })
        .collect();

    let columns: Vec<Column> = (0..width)
        .map(|col_idx| {
            let values: Vec<String> = rows
                .iter()
                .skip(1)
                .map(|row| row.get(col_idx).cloned().unwrap_or_default())
                .collect();
            Column::new(headers[col_idx].as_str().into(), values)
        })
        .collect();

    DataFrame::new(height, columns).map_err(|e| PipelineError::DataLoadingError(e.to_string()))
}

/// Load Parquet file
fn load_parquet(file_path: &str) -> Result<DataFrame, PipelineError> {
    let file = File::open(file_path).map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    ParquetReader::new(file)
        .finish()
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))
}

#[derive(Debug, Clone)]
pub struct TypeDetectionChange {
    pub column: String,
    pub from_type: String,
    pub to_type: String,
}

/// Detect and apply column types for String columns.
///
/// This is conservative by default:
/// - only non-empty values are considered for detection
/// - all considered values must parse successfully for a cast to be applied
pub fn detect_and_apply_types(
    df: &DataFrame,
) -> Result<(DataFrame, Vec<TypeDetectionChange>), PipelineError> {
    let mut out = df.clone();
    let mut changes = Vec::new();

    let names = out.get_column_names_owned();
    for name in names {
        let name_str = name.to_string();

        let col = out
            .column(&name_str)
            .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?
            .clone();

        if col.dtype() != &DataType::String {
            continue;
        }

        let inferred = infer_string_column_type(&col)?;
        let Some(target_type) = inferred else {
            continue;
        };

        let new_col = cast_string_column(&col, &target_type)?;
        out.replace(&name_str, new_col)
            .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

        changes.push(TypeDetectionChange {
            column: name_str,
            from_type: DataType::String.to_string(),
            to_type: target_type.to_string(),
        });
    }

    Ok((out, changes))
}

fn infer_string_column_type(col: &Column) -> Result<Option<DataType>, PipelineError> {
    let mut values = Vec::new();

    for idx in 0..col.len() {
        let av = col
            .get(idx)
            .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

        if matches!(av, AnyValue::Null) {
            continue;
        }

        let v = av.str_value();
        let trimmed = v.trim();
        if trimmed.is_empty() {
            continue;
        }
        values.push(trimmed.to_string());
    }

    if values.is_empty() {
        return Ok(None);
    }

    if values.iter().all(|v| parse_bool(v).is_some()) {
        return Ok(Some(DataType::Boolean));
    }
    if values.iter().all(|v| parse_i64(v).is_some()) {
        return Ok(Some(DataType::Int64));
    }
    if values.iter().all(|v| parse_f64(v).is_some()) {
        return Ok(Some(DataType::Float64));
    }
    if values.iter().all(|v| parse_datetime(v).is_some()) {
        return Ok(Some(DataType::Datetime(TimeUnit::Microseconds, None)));
    }
    if values.iter().all(|v| parse_date(v).is_some()) {
        return Ok(Some(DataType::Date));
    }

    Ok(None)
}

fn cast_string_column(col: &Column, target: &DataType) -> Result<Column, PipelineError> {
    let name = col.name().clone();

    match target {
        DataType::Boolean => {
            let values = (0..col.len())
                .map(|idx| {
                    let av = col
                        .get(idx)
                        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

                    if matches!(av, AnyValue::Null) {
                        return Ok(None);
                    }

                    let text = av.str_value();
                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        Ok(None)
                    } else {
                        parse_bool(trimmed)
                            .ok_or_else(|| {
                                PipelineError::DataLoadingError(format!(
                                    "Failed bool parse for value '{}' in column '{}'",
                                    trimmed, name
                                ))
                            })
                            .map(Some)
                    }
                })
                .collect::<Result<Vec<Option<bool>>, PipelineError>>()?;

            Ok(Column::new(name, values))
        }
        DataType::Int64 => {
            let values = (0..col.len())
                .map(|idx| {
                    let av = col
                        .get(idx)
                        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

                    if matches!(av, AnyValue::Null) {
                        return Ok(None);
                    }

                    let text = av.str_value();
                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        Ok(None)
                    } else {
                        parse_i64(trimmed)
                            .ok_or_else(|| {
                                PipelineError::DataLoadingError(format!(
                                    "Failed int parse for value '{}' in column '{}'",
                                    trimmed, name
                                ))
                            })
                            .map(Some)
                    }
                })
                .collect::<Result<Vec<Option<i64>>, PipelineError>>()?;

            Ok(Column::new(name, values))
        }
        DataType::Float64 => {
            let values = (0..col.len())
                .map(|idx| {
                    let av = col
                        .get(idx)
                        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

                    if matches!(av, AnyValue::Null) {
                        return Ok(None);
                    }

                    let text = av.str_value();
                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        Ok(None)
                    } else {
                        parse_f64(trimmed)
                            .ok_or_else(|| {
                                PipelineError::DataLoadingError(format!(
                                    "Failed float parse for value '{}' in column '{}'",
                                    trimmed, name
                                ))
                            })
                            .map(Some)
                    }
                })
                .collect::<Result<Vec<Option<f64>>, PipelineError>>()?;

            Ok(Column::new(name, values))
        }
        DataType::Date => {
            let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
                .ok_or_else(|| PipelineError::DataLoadingError("Invalid epoch date".to_string()))?;

            let values = (0..col.len())
                .map(|idx| {
                    let av = col
                        .get(idx)
                        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

                    if matches!(av, AnyValue::Null) {
                        return Ok(None);
                    }

                    let text = av.str_value();
                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        Ok(None)
                    } else {
                        let date = parse_date(trimmed).ok_or_else(|| {
                            PipelineError::DataLoadingError(format!(
                                "Failed date parse for value '{}' in column '{}'",
                                trimmed, name
                            ))
                        })?;
                        let days = date.signed_duration_since(epoch).num_days() as i32;
                        Ok(Some(days))
                    }
                })
                .collect::<Result<Vec<Option<i32>>, PipelineError>>()?;

            let series = Series::new(name.clone(), values)
                .cast_with_options(&DataType::Date, CastOptions::Strict)
                .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

            Ok(series.into())
        }
        DataType::Datetime(TimeUnit::Microseconds, _) => {
            let values = (0..col.len())
                .map(|idx| {
                    let av = col
                        .get(idx)
                        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

                    if matches!(av, AnyValue::Null) {
                        return Ok(None);
                    }

                    let text = av.str_value();
                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        Ok(None)
                    } else {
                        let dt = parse_datetime(trimmed).ok_or_else(|| {
                            PipelineError::DataLoadingError(format!(
                                "Failed datetime parse for value '{}' in column '{}'",
                                trimmed, name
                            ))
                        })?;
                        Ok(Some(dt.and_utc().timestamp_micros()))
                    }
                })
                .collect::<Result<Vec<Option<i64>>, PipelineError>>()?;

            let series = Series::new(name.clone(), values)
                .cast_with_options(
                    &DataType::Datetime(TimeUnit::Microseconds, None),
                    CastOptions::Strict,
                )
                .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

            Ok(series.into())
        }
        _ => Err(PipelineError::DataLoadingError(format!(
            "Unsupported inferred target type: {}",
            target
        ))),
    }
}

fn parse_bool(input: &str) -> Option<bool> {
    match input.trim().to_ascii_lowercase().as_str() {
        "true" | "t" | "yes" | "y" | "1" => Some(true),
        "false" | "f" | "no" | "n" | "0" => Some(false),
        _ => None,
    }
}

fn normalize_number(input: &str) -> String {
    input.trim().replace(',', "")
}

fn parse_i64(input: &str) -> Option<i64> {
    let n = normalize_number(input);
    if n.contains('.') || n.contains('e') || n.contains('E') {
        return None;
    }
    n.parse::<i64>().ok()
}

fn parse_f64(input: &str) -> Option<f64> {
    let n = normalize_number(input);
    n.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_date(input: &str) -> Option<NaiveDate> {
    const FORMATS: &[&str] = &["%Y-%m-%d", "%m/%d/%Y", "%d/%m/%Y", "%Y/%m/%d"];

    let matches: Vec<&str> = FORMATS
        .iter()
        .filter_map(|fmt| NaiveDate::parse_from_str(input, fmt).ok().map(|_| *fmt))
        .collect();

    if matches.len() > 1 {
        eprintln!(
            "Warning: Date '{}' matches multiple formats ({:?}), using first match",
            input, matches
        );
    }

    matches
        .first()
        .copied()
        .and_then(|fmt| NaiveDate::parse_from_str(input, fmt).ok())
}

fn parse_datetime(input: &str) -> Option<NaiveDateTime> {
    const FORMATS: &[&str] = &[
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%m/%d/%Y %H:%M:%S",
        "%m/%d/%Y %H:%M",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
    ];

    let matches: Vec<&str> = FORMATS
        .iter()
        .filter_map(|fmt| NaiveDateTime::parse_from_str(input, fmt).ok().map(|_| *fmt))
        .collect();

    if matches.len() > 1 {
        eprintln!(
            "Warning: Datetime '{}' matches multiple formats ({:?}), using first match",
            input, matches
        );
    }

    matches
        .first()
        .copied()
        .and_then(|fmt| NaiveDateTime::parse_from_str(input, fmt).ok())
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
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    let mut unique = HashSet::new();
    for idx in 0..col.len() {
        let value = col
            .get(idx)
            .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

        if !matches!(value, AnyValue::Null) {
            unique.insert(value.str_value().to_string());
        }
    }

    let mut values: Vec<String> = unique.into_iter().collect();
    values.sort();
    Ok(values)
}

/// Get column statistics
pub fn get_column_stats(df: &DataFrame, column: &str) -> Result<ColumnStats, PipelineError> {
    let col = df
        .column(column)
        .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

    let series = col.as_materialized_series();
    let dtype = series.dtype();

    if dtype.is_numeric() {
        let casted = series
            .cast(&DataType::Float64)
            .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;
        let values = casted
            .f64()
            .map_err(|e| PipelineError::DataLoadingError(e.to_string()))?;

        Ok(ColumnStats {
            data_type: dtype.to_string(),
            min: values.min(),
            max: values.max(),
            sum: values.sum(),
            count: series.len(),
        })
    } else {
        Ok(ColumnStats {
            data_type: dtype.to_string(),
            min: None,
            max: None,
            sum: None,
            count: series.len(),
        })
    }
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
