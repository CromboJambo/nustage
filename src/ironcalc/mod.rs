//! IronCalc Integration Module
//!
//! This module provides integration between nustage's Power Query-style
//! data transformation layer and IronCalc's spreadsheet engine.
//!
//! # Overview
//! - IronCalc handling: Spreadsheet UI, formulas, calculations
//! - Nustage handling: Data loading, transformation pipelines
//! - Integration: Loading data between systems, schema awareness

use polars::prelude::*;
use thiserror::Error;

/// Error types for IronCalc integration
#[derive(Debug, Error)]
pub enum IronCalcError {
    #[error("IronCalc model error: {0}")]
    IronCalcError(String),

    #[error("Data conversion error: {0}")]
    DataConversionError(String),

    #[error("Invalid cell reference: {0}")]
    InvalidCellReference(String),

    #[error("Formula parsing error: {0}")]
    FormulaError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
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
    /// Data schema information
    schema: Vec<ColumnSchema>,
    /// The underlying DataFrame
    data_frame: DataFrame,
    /// The underlying IronCalc model
    model: ironcalc::base::Model<'static>,
}

impl IronCalcIntegration {
    /// Create a new IronCalc integration from a DataFrame
    pub fn from_dataframe(df: DataFrame) -> Result<Self, IronCalcError> {
        // Extract schema from DataFrame
        let polars_schema = df.schema();
        let columns: Vec<ColumnSchema> = polars_schema
            .iter()
            .enumerate()
            .map(|(i, (name, dtype))| ColumnSchema {
                index: i,
                name: name.to_string(),
                data_type: dtype.to_string(),
            })
            .collect();

        // Create a new IronCalc model and populate it with the DataFrame data
        let model = Self::create_model_from_dataframe(&df)?;

        Ok(Self {
            schema: columns,
            data_frame: df,
            model,
        })
    }

    /// Create an IronCalc Model from a DataFrame
    fn create_model_from_dataframe(
        df: &DataFrame,
    ) -> Result<ironcalc::base::Model<'static>, IronCalcError> {
        let mut model = ironcalc::base::Model::new_empty("nustage-workbook", "en", "UTC", "en")
            .map_err(|e| IronCalcError::IronCalcError(e.to_string()))?;

        // Get dimensions
        let height = df.height();
        let width = df.width();

        if height == 0 {
            return Ok(model);
        }

        // Write header row (1-indexed in Excel)
        for schema_col in 0..width {
            let excel_col: i32 = (schema_col + 1).try_into().map_err(|_| {
                IronCalcError::InvalidCellReference(format!(
                    "Column index {} too large",
                    schema_col
                ))
            })?;

            // Get column name from schema
            let (col_name, _) = df.schema().iter().nth(schema_col).unwrap();
            model
                .set_user_input(0, 1, excel_col, col_name.to_string())
                .map_err(|e| IronCalcError::IronCalcError(e.to_string()))?;
        }

        // Write data rows (1-indexed in Excel)
        for row_idx in 0..height {
            let excel_row: i32 = (row_idx + 2).try_into().map_err(|_| {
                IronCalcError::InvalidCellReference(format!("Row index {} too large", row_idx))
            })?;

            // Get column names for this iteration
            let col_names: Vec<String> = df
                .schema()
                .iter()
                .map(|(name, _)| name.to_string())
                .collect();

            for schema_col in 0..width {
                let excel_col: i32 = (schema_col + 1).try_into().map_err(|_| {
                    IronCalcError::InvalidCellReference(format!(
                        "Column index {} too large",
                        schema_col
                    ))
                })?;

                let column = df
                    .column(&col_names[schema_col])
                    .map_err(|e| IronCalcError::DataConversionError(e.to_string()))?;
                // Convert Column to Series for type matching
                let series: &Series = column.as_series().ok_or_else(|| {
                    IronCalcError::DataConversionError(format!(
                        "Failed to get series from column {}",
                        col_names[schema_col]
                    ))
                })?;

                let value_str = Self::series_value_to_string(series, row_idx)?;

                model
                    .set_user_input(0, excel_row, excel_col, value_str)
                    .map_err(|e| IronCalcError::IronCalcError(e.to_string()))?;
            }
        }

        Ok(model)
    }

    /// Convert a DataFrame cell value to string representation for Excel
    fn series_value_to_string(series: &Series, row_idx: usize) -> Result<String, IronCalcError> {
        match series.dtype() {
            DataType::Int32 => {
                let s = series
                    .i32()
                    .map_err(|e| IronCalcError::DataConversionError(e.to_string()))?;
                Ok(s.get(row_idx).unwrap_or(0).to_string())
            }
            DataType::Int64 => {
                let s = series
                    .i64()
                    .map_err(|e| IronCalcError::DataConversionError(e.to_string()))?;
                Ok(s.get(row_idx).unwrap_or(0).to_string())
            }
            DataType::Float32 => {
                let s = series
                    .f32()
                    .map_err(|e| IronCalcError::DataConversionError(e.to_string()))?;
                Ok(s.get(row_idx).unwrap_or(0.0).to_string())
            }
            DataType::Float64 => {
                let s = series
                    .f64()
                    .map_err(|e| IronCalcError::DataConversionError(e.to_string()))?;
                Ok(s.get(row_idx).unwrap_or(0.0).to_string())
            }
            DataType::String => {
                let s = series
                    .str()
                    .map_err(|e| IronCalcError::DataConversionError(e.to_string()))?;
                Ok(s.get(row_idx).unwrap_or("").to_string())
            }
            DataType::Boolean => {
                let s = series
                    .bool()
                    .map_err(|e| IronCalcError::DataConversionError(e.to_string()))?;
                Ok(s.get(row_idx).unwrap_or(false).to_string())
            }
            _ => {
                // For other types, use the default formatting
                Ok(series.get(row_idx).unwrap().to_string())
            }
        }
    }

    /// Get the schema of the current data
    pub fn get_schema(&self) -> &[ColumnSchema] {
        &self.schema
    }

    /// Get the DataFrame
    pub fn get_dataframe(&self) -> &DataFrame {
        &self.data_frame
    }

    /// Load data from a file into IronCalc
    pub fn load_from_file(_file_path: &str) -> Result<Self, IronCalcError> {
        // For now, we'll just return a placeholder implementation
        let df = DataFrame::empty();

        let polars_schema = df.schema();
        let columns: Vec<ColumnSchema> = polars_schema
            .iter()
            .enumerate()
            .map(|(i, (name, dtype))| ColumnSchema {
                index: i,
                name: name.to_string(),
                data_type: dtype.to_string(),
            })
            .collect();

        Ok(Self {
            schema: columns,
            data_frame: df,
            model: ironcalc::base::Model::new_empty("loaded-workbook", "en", "UTC", "en")
                .map_err(|e| IronCalcError::IronCalcError(e.to_string()))?,
        })
    }

    /// Save the workbook to a file
    pub fn save(&self, file_path: &str) -> Result<(), IronCalcError> {
        use ironcalc::export::save_to_xlsx;

        // Use ironcalc's built-in save function
        save_to_xlsx(&self.model, file_path).map_err(|e| {
            IronCalcError::IronCalcError(format!("Failed to save Excel file: {}", e))
        })?;

        Ok(())
    }

    /// Update the underlying model with current DataFrame data
    pub fn update_model_from_dataframe(&mut self) -> Result<(), IronCalcError> {
        let new_model = Self::create_model_from_dataframe(&self.data_frame)?;
        self.model = new_model;
        Ok(())
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
pub fn load_dataframe_to_ironcalc(df: DataFrame) -> Result<IronCalcIntegration, IronCalcError> {
    IronCalcIntegration::from_dataframe(df)
}

/// Apply a Power Query-style transformation to IronCalc data
pub fn apply_transformation(
    _integration: &mut IronCalcIntegration,
    _transformation: Transformation,
) -> Result<(), IronCalcError> {
    // For now, we'll just return a placeholder implementation
    Ok(())
}

/// Power Query-style transformation types
#[derive(Debug, Clone)]
pub enum Transformation {
    SelectColumns(Vec<String>),
    FilterRows(String, String),
    GroupBy(Vec<String>, Vec<Aggregation>),
}

/// Aggregation operation for group by
#[derive(Debug, Clone)]
pub struct Aggregation {
    pub column: String,
    pub operation: AggregationOperation,
}

/// Aggregation operation types
#[derive(Debug, Clone)]
pub enum AggregationOperation {
    Sum,
    Mean,
    Count,
    Min,
    Max,
    First,
    Last,
    StdDev,
    Variance,
}

/// Get field names from IronCalc for autocomplete
pub fn get_field_names(integration: &IronCalcIntegration) -> Vec<String> {
    integration
        .get_schema()
        .iter()
        .map(|schema| schema.name.clone())
        .collect()
}

/// Get cell value with formula resolution
pub fn get_resolved_value(
    _integration: &IronCalcIntegration,
    _row: usize,
    _col: usize,
) -> Result<String, IronCalcError> {
    // For now, we'll just return a placeholder implementation
    Ok("".to_string())
}
