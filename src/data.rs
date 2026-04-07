//! Nustage - A data processing and analysis toolkit for Rust
//!
//! This is the data module for the nustage project.

use polars::prelude::*;
use serde::{Deserialize, Serialize};

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
