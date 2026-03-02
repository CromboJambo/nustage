//! IronCalc Integration Module
//!
//! This module provides integration between nustage's Power Query-style
//! data transformation layer and IronCalc's spreadsheet engine.
//!
//! # Overview
//! - IronCalc handling: Spreadsheet UI, formulas, calculations
//! - Nustage handling: Data loading, transformation pipelines
//! - Integration: Loading data between systems, schema awareness

use ironcalc::model::{CellType, Formula, Model, Sheet, Workbook};
use ironcalc::sheet::SheetError;
use polars::prelude::*;
use std::collections::HashMap;
use std::path::Path;

/// Error types for IronCalc integration
#[derive(Debug, thiserror::Error)]
pub enum IronCalcError {
    #[error("IronCalc model error: {0}")]
    IronCalcError(String),

    #[error("Data conversion error: {0}")]
    DataConversionError(String),

    #[error("Sheet error: {0}")]
    SheetError(#[from] SheetError),

    #[error("Invalid cell reference: {0}")]
    InvalidCellReference(String),

    #[error("Formula parsing error: {0}")]
    FormulaError(String),
}

/// Represents a cell in the spreadsheet with schema information
#[derive(Debug, Clone)]
pub struct SpreadsheetCell {
    pub row: usize,
    pub col: usize,
    pub value: String,
    pub data_type: String,
    pub formula: Option<String>,
    pub is_formula: bool,
}

/// Represents a column in the spreadsheet
#[derive(Debug, Clone)]
pub struct SpreadsheetColumn {
    pub index: usize,
    pub name: String,
    pub data_type: String,
    pub cells: Vec<String>,
}

/// Represents a row in the spreadsheet
#[derive(Debug, Clone)]
pub struct SpreadsheetRow {
    pub index: usize,
    pub cells: Vec<String>,
}

/// IronCalc integration for loading and saving spreadsheet data
pub struct IronCalcIntegration {
    workbook: Workbook,
    sheets: HashMap<String, Sheet>,
    schema: Vec<ColumnSchema>,
    data_frame: DataFrame,
}

impl IronCalcIntegration {
    /// Create a new IronCalc integration from a DataFrame
    pub fn from_dataframe(df: DataFrame) -> Result<Self, IronCalcError> {
        let workbook = Workbook::new();
        let mut sheets = HashMap::new();

        // Create a sheet for the data
        let sheet_name = "Data";
        let mut sheet = Sheet::new(sheet_name.to_string());
        sheet.set_name(sheet_name.to_string());

        // Add header row
        let schema = df.schema();
        let columns: Vec<_> = schema.iter().collect();

        for (col_idx, (col_name, dtype)) in columns.iter().enumerate() {
            let cell = ironcalc::model::Cell::new(
                col_name.to_string(),
                CellType::String,
                None,
                None,
            );
            sheet.add_cell(0, col_idx, cell);
        }

        // Add data rows
        for row_idx in 1..=df.height() {
            for (col_idx, (col_name, dtype)) in columns.iter().enumerate() {
                let value = df.get(row_idx - 1, col_idx)
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "".to_string());

                let cell = ironcalc::model::Cell::new(
                    value.clone(),
                    CellType::String,
                    None,
                    None,
                );
                sheet.add_cell(row_idx, col_idx, cell);
            }
        }

        sheets.insert(sheet_name.to_string(), sheet);
        workbook.add_sheet(sheet_name.to_string());

        Ok(Self {
            workbook,
            sheets,
            schema: columns
                .iter()
                .enumerate()
                .map(|(i, (name, dtype))| ColumnSchema {
                    index: i,
                    name: name.to_string(),
                    data_type: dtype.to_string(),
                })
                .collect(),
            data_frame: df,
        })
    }

    /// Create a new IronCalc integration from a file
    pub fn from_file(file_path: &str) -> Result<Self, IronCalcError> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err(IronCalcError::IronCalcError(format!(
                "File not found: {}",
                file_path
            )));
        }

        // Load workbook from file
        let workbook = Workbook::open(file_path)
            .map_err(|e| IronCalcError::IronCalcError(e.to_string()))?;

        // Load data into DataFrame
        let mut data_frame = DataFrame::empty();

        // Get all sheets
        let sheet_names = workbook.sheet_names();
        for sheet_name in sheet_names {
            if let Some(sheet) = workbook.sheet(sheet_name) {
                let sheet_df = self::sheet_to_dataframe(sheet)?;
                data_frame = data_frame.vstack(&sheet_df).unwrap_or_else(|_| {
                    data_frame.vstack(&sheet_df).unwrap_or(DataFrame::empty())
                });
            }
        }

        Ok(Self {
            workbook,
            sheets: workbook.sheets().collect(),
            schema: Vec::new(), // Will be populated when needed
            data_frame,
        })
    }

    /// Get the schema of the current data
    pub fn get_schema(&self) -> &[ColumnSchema] {
        &self.schema
    }

    /// Get the DataFrame
    pub fn get_dataframe(&self) -> &DataFrame {
        &self.data_frame
    }

    /// Save the workbook to a file
    pub fn save(&self, file_path: &str) -> Result<(), IronCalcError> {
        self.workbook.save(file_path)
            .map_err(|e| IronCalcError::IronCalcError(e.to_string()))
    }

    /// Get cells for a specific sheet
    pub fn get_sheet_cells(
        &self,
        sheet_name: &str,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> Result<Vec<SpreadsheetCell>, IronCalcError> {
        let mut cells = Vec::new();

        if let Some(sheet) = self.sheets.get(sheet_name) {
            for row in start_row..=end_row {
                for col in start_col..=end_col {
                    if let Some(cell) = sheet.get_cell(row, col) {
                        let cell_type = match cell.get_cell_type() {
                            CellType::String => "String",
                            CellType::Number => "Number",
                            CellType::Boolean => "Boolean",
                            CellType::DateTime => "DateTime",
                            CellType::Error => "Error",
                            CellType::Empty => "Empty",
                        };

                        let formula = cell.get_formula()
                            .map(|f| f.to_string())
                            .filter(|f| !f.is_empty());

                        cells.push(SpreadsheetCell {
                            row,
                            col,
                            value: cell.get_value().unwrap_or_else(|| "".to_string()),
                            data_type: cell_type.to_string(),
                            formula,
                            is_formula: formula.is_some(),
                        });
                    }
                }
            }
        }

        Ok(cells)
    }

    /// Get all columns from a sheet
    pub fn get_sheet_columns(
        &self,
        sheet_name: &str,
        start_row: usize,
        end_row: usize,
    ) -> Result<Vec<SpreadsheetColumn>, IronCalcError> {
        let mut columns = Vec::new();

        if let Some(sheet) = self.sheets.get(sheet_name) {
            let schema = sheet.get_schema();
            let num_cols = schema.len();

            for col_idx in 0..num_cols {
                let mut cells = Vec::new();
                for row in start_row..=end_row {
                    if let Some(cell) = sheet.get_cell(row, col_idx) {
                        cells.push(cell.get_value().unwrap_or_else(|| "".to_string()));
                    }
                }

                let column_name = schema.get(col_idx)
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| format!("Column{}", col_idx));

                columns.push(SpreadsheetColumn {
                    index: col_idx,
                    name: column_name,
                    data_type: "Unknown".to_string(),
                    cells,
                });
            }
        }

        Ok(columns)
    }

    /// Convert a sheet to a DataFrame
    fn sheet_to_dataframe(sheet: &Sheet) -> Result<DataFrame, IronCalcError> {
        // Get schema
        let schema = sheet.get_schema();

        // Get column data
        let mut columns: Vec<Vec<String>> = Vec::new();
        let mut column_names: Vec<String> = Vec::new();

        for (col_idx, col_name) in schema.iter().enumerate() {
            column_names.push(col_name.clone());

            let mut col_data = Vec::new();
            for row in 0..=sheet.row_count() {
                if let Some(cell) = sheet.get_cell(row, col_idx) {
                    col_data.push(cell.get_value().unwrap_or_else(|| "".to_string()));
                }
            }
            columns.push(col_data);
        }

        // Create DataFrame
        let df = DataFrame::new(columns
            .into_iter()
            .zip(column_names.into_iter())
            .map(|(data, name)| Series::new(name, data))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| IronCalcError::DataConversionError(e.to_string()))?;

        Ok(df)
    }
}

/// Column schema information
#[derive(Debug, Clone)]
pub struct ColumnSchema {
    pub index: usize,
    pub name: String,
    pub data_type: String,
}

/// Load data into IronCalc from a DataFrame
pub fn load_dataframe_to_ironcalc(df: DataFrame) -> Result<Workbook, IronCalcError> {
    let workbook = Workbook::new();
    let sheet_name = "Data";
    let mut sheet = Sheet::new(sheet_name.to_string());
    sheet.set_name(sheet_name.to_string());

    // Add header row
    let schema = df.schema();
    let columns: Vec<_> = schema.iter().collect();

    for (col_idx, (col_name, dtype)) in columns.iter().enumerate() {
        let cell = ironcalc::model::Cell::new(
            col_name.to_string(),
            CellType::String,
            None,
            None,
        );
        sheet.add_cell(0, col_idx, cell);
    }

    // Add data rows
    for row_idx in 1..=df.height() {
        for (col_idx, (col_name, dtype)) in columns.iter().enumerate() {
            let value = df.get(row_idx - 1, col_idx)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "".to_string());

            let cell = ironcalc::model::Cell::new(
                value.clone(),
                CellType::String,
                None,
                None,
            );
            sheet.add_cell(row_idx, col_idx, cell);
        }
    }

    workbook.add_sheet(sheet_name.to_string());

    Ok(workbook)
}

/// Apply a Power Query-style transformation to IronCalc data
pub fn apply_transformation(
    workbook: &mut Workbook,
    transformation: Transformation,
) -> Result<(), IronCalcError> {
    match transformation {
        Transformation::SelectColumns(columns) => {
            // Get the first sheet
            let sheet_name = workbook.sheet_names().first()
                .ok_or_else(|| IronCalcError::IronCalcError("No sheets found".to_string()))?;

            if let Some(sheet) = workbook.sheet_mut(sheet_name) {
                // Get current schema
                let schema = sheet.get_schema();
                let column_indices: Vec<usize> = schema
                    .iter()
                    .enumerate()
                    .filter(|(idx, name)| columns.contains(name))
                    .map(|(idx, _)| idx)
                    .collect();

                // Rebuild sheet with only selected columns
                let mut new_sheet = Sheet::new(sheet_name.clone());
                new_sheet.set_name(sheet_name.clone());

                // Add header row
                for (col_idx, col_name) in column_indices.iter() {
                    let cell = ironcalc::model::Cell::new(
                        col_name.clone(),
                        CellType::String,
                        None,
                        None,
                    );
                    new_sheet.add_cell(0, *col_idx, cell);
                }

                // Add data rows
                for row_idx in 1..=sheet.row_count() {
                    for (new_col_idx, old_col_idx) in column_indices.iter().enumerate() {
                        if let Some(cell) = sheet.get_cell(*row_idx, *old_col_idx) {
                            new_sheet.add_cell(row_idx, new_col_idx, cell.clone());
                        }
                    }
                }

                // Replace the sheet
                workbook.replace_sheet(sheet_name.clone(), new_sheet);
            }
        }
    }

    Ok(())
}

/// Power Query-style transformation types
#[derive(Debug, Clone)]
pub enum Transformation {
    SelectColumns(Vec<String>),
    FilterRows(String, String),
    GroupBy(String, Vec<String>),
}

/// Get field names from IronCalc for autocomplete
pub fn get_field_names(sheet: &Sheet) -> Vec<String> {
    let schema = sheet.get_schema();
    schema.iter().map(|name| name.clone()).collect()
}

/// Get cell value with formula resolution
pub fn get_resolved_value(sheet: &Sheet, row: usize, col: usize) -> Result<String, IronCalcError> {
    if let Some(cell) = sheet.get_cell(row, col) {
        let formula = cell.get_formula();
        if let Some(formula) = formula {
            // Try to resolve the formula
            let result = sheet.evaluate_formula(formula)
                .map_err(|e| IronCalcError::FormulaError(e.to_string()))?;
            Ok(result.to_string())
        } else {
            Ok(cell.get_value().unwrap_or_else(|| "".to_string()))
        }
    } else {
        Err(IronCalcError::InvalidCellReference(format!("({},{})", row, col)))
    }
}
