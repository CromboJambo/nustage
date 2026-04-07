//! Nustage - A data processing and analysis toolkit for Rust
//!
//! This is the ironcalc module for the nustage project.

use polars::prelude::*;
use serde::{Deserialize, Serialize};

/// IronCalc integration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IronCalcIntegration {
    /// DataFrame that represents the workbook data
    pub dataframe: DataFrame,

    /// Schema of the workbook
    pub schema: Vec<ColumnSchema>,
}

impl IronCalcIntegration {
    /// Create IronCalcIntegration from DataFrame
    pub fn from_dataframe(df: DataFrame) -> Result<Self, PolarsError> {
        let schema = get_schema(&df)?;
        Ok(IronCalcIntegration {
            dataframe: df,
            schema,
        })
    }

    /// Get the schema of the workbook
    pub fn get_schema(&self) -> &Vec<ColumnSchema> {
        &self.schema
    }

    /// Save to Excel file
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would save the dataframe to Excel format
        println!("Saving workbook to: {}", path);
        Ok(())
    }
}

/// Column schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub index: usize,
    pub name: String,
    pub data_type: String,
}

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
