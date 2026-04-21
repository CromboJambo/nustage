//! Nustage - A data processing and analysis toolkit for Rust
//!
//! This is the data module for the nustage project.

use calamine::{Data, Range, Reader, Sheets, open_workbook_auto};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

/// Get schema from DataFrame
pub fn get_schema(df: &DataFrame) -> Result<Vec<ColumnSchema>, PolarsError> {
    let mut schema = Vec::new();
    for (i, (name, dtype)) in df.schema().iter().enumerate() {
        schema.push(ColumnSchema {
            index: i,
            name: name.to_string(),
            data_type: format!("{:?}", dtype),
        });
    }
    Ok(schema)
}

/// Column schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub index: usize,
    pub name: String,
    pub data_type: String,
}

/// Open Excel workbook from file path
pub fn open_excel(path: &str) -> Result<Sheets<BufReader<File>>, Box<dyn std::error::Error>> {
    let workbook = open_workbook_auto(path)?;
    Ok(workbook)
}

/// Get sheet names from workbook
pub fn sheet_names(workbook: &Sheets<BufReader<File>>) -> Vec<String> {
    workbook.sheet_names().to_owned()
}

/// Read worksheet range by name
pub fn worksheet_range(
    workbook: &mut Sheets<BufReader<File>>,
    sheet_name: &str,
) -> Result<Option<Range<Data>>, Box<dyn std::error::Error>> {
    Ok(workbook.worksheet_range(sheet_name).ok())
}

/// Get formulas from a worksheet
pub fn worksheet_formula(
    workbook: &mut Sheets<BufReader<File>>,
    sheet_name: &str,
) -> Result<Range<String>, Box<dyn std::error::Error>> {
    Ok(workbook.worksheet_formula(sheet_name)?)
}

/// Get defined names (formula references) from workbook
pub fn defined_names(workbook: &Sheets<BufReader<File>>) -> Vec<(String, String)> {
    workbook.defined_names().to_vec()
}

/// Convert worksheet range to DataFrame
pub fn range_to_dataframe(range: &Range<Data>) -> Result<DataFrame, PolarsError> {
    let rows: Vec<Vec<AnyValue<'static>>> = range
        .rows()
        .map(|row| {
            row.iter()
                .map(|v| match v {
                    Data::String(s) => AnyValue::StringOwned(s.into()),
                    Data::Empty => AnyValue::Null,
                    Data::Float(f) => AnyValue::Float64(*f),
                    Data::Int(i) => AnyValue::Int64(*i),
                    Data::Bool(b) => AnyValue::Boolean(*b),
                    Data::Error(_e) => AnyValue::Null,
                    Data::DateTime(_) => AnyValue::Null,
                    Data::DateTimeIso(_) => AnyValue::Null,
                    Data::DurationIso(_) => AnyValue::Null,
                })
                .collect()
        })
        .collect();
    let columns: Vec<Column> = rows
        .iter()
        .enumerate()
        .map(|(i, row)| Column::new(format!("col_{}", i).into(), row))
        .collect();
    DataFrame::new(range.get_size().0, columns)
}

/// Convert worksheet range to CSV
pub fn range_to_csv(range: &Range<Data>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path(path)?;
    let rows: Vec<Vec<String>> = range
        .rows()
        .map(|row| {
            row.iter()
                .map(|v| match v {
                    Data::String(s) => s.to_string(),
                    Data::Empty => "".to_string(),
                    Data::Float(f) => f.to_string(),
                    Data::Int(i) => i.to_string(),
                    Data::Bool(b) => b.to_string(),
                    Data::Error(e) => e.to_string(),
                    Data::DateTime(_) => "".to_string(),
                    Data::DateTimeIso(s) => s.to_string(),
                    Data::DurationIso(s) => s.to_string(),
                })
                .collect::<Vec<String>>()
        })
        .collect();
    if rows.is_empty() {
        return Ok(());
    }
    let header = rows[0].clone();
    writer.write_record(&header)?;
    for row in rows.iter() {
        writer.write_record(row)?;
    }
    Ok(())
}

/// Open Excel workbook from file path and export all sheets to CSV
pub fn export_excel_to_csv(
    path: &str,
    output_dir: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut workbook = open_workbook_auto(path)?;
    let names = workbook.sheet_names().to_owned();
    let mut csv_paths = Vec::new();
    for name in names {
        let sanitized = name.replace(' ', "_").replace('/', "-");
        let csv_path = format!("{}/{}.csv", output_dir, sanitized);
        if let Ok(range) = workbook.worksheet_range(&name) {
            range_to_csv(&range, &csv_path)?;
        }
        csv_paths.push(csv_path);
    }
    Ok(csv_paths)
}
