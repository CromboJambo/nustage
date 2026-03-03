//! Power Query-Style Transformations Module
//!
//! This module provides a step-based data transformation model inspired by Power Query.
//! It allows for immutable, reversible transformation steps with schema awareness.

use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use thiserror::Error;

/// Error types for transformation operations
#[derive(Debug, Error)]
pub enum TransformationError {
    #[error("Invalid transformation step: {0}")]
    InvalidStep(String),

    #[error("Column not found: {0}")]
    ColumnNotFound(String),

    #[error("Schema mismatch: {0}")]
    SchemaMismatch(String),

    #[error("Data error: {0}")]
    DataError(String),
}

/// Represents a single transformation step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationStep {
    pub id: String,
    pub name: String,
    pub step_type: StepType,
    pub parameters: HashMap<String, String>,
    pub output_schema: Vec<ColumnSchema>,
}

/// Type of transformation step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepType {
    /// Select specific columns
    SelectColumns(Vec<String>),

    /// Filter rows based on condition
    FilterRows(String, String),

    /// Group by columns and aggregate
    GroupBy(Vec<String>, Vec<Aggregation>),

    /// Sort by columns
    SortBy(Vec<String>, bool),

    /// Rename columns
    RenameColumn(String, String),

    /// Drop columns
    DropColumns(Vec<String>),

    /// Custom SQL query
    CustomSql(String),

    /// Add column with formula
    AddColumn(String, String),

    /// Remove duplicates
    RemoveDuplicates(bool),
}

/// Aggregation operation for group by
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregation {
    pub column: String,
    pub operation: AggregationOperation,
}

/// Aggregation operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Column schema information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub index: usize,
    pub name: String,
    pub data_type: String,
}

/// Represents a transformation pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationPipeline {
    pub name: String,
    pub steps: Vec<TransformationStep>,
    pub input_schema: Vec<ColumnSchema>,
}

impl TransformationPipeline {
    /// Create a new transformation pipeline
    pub fn new(name: String) -> Self {
        Self {
            name,
            steps: Vec::new(),
            input_schema: Vec::new(),
        }
    }

    /// Add a transformation step
    pub fn add_step(&mut self, step: TransformationStep) -> Result<(), TransformationError> {
        // Validate the step
        self.validate_step(&step)?;

        // Update the schema
        self.update_schema(&step);

        // Add the step
        self.steps.push(step);

        Ok(())
    }

    /// Apply all transformations to a DataFrame
    pub fn apply(&self, df: &DataFrame) -> Result<DataFrame, TransformationError> {
        let mut result = df.clone();

        for step in &self.steps {
            result = self.apply_step(&result, step)?;
        }

        Ok(result)
    }

    /// Get the output schema
    pub fn output_schema(&self) -> &[ColumnSchema] {
        if let Some(last_step) = self.steps.last() {
            &last_step.output_schema
        } else {
            &self.input_schema
        }
    }

    /// Get step by ID
    pub fn get_step(&self, id: &str) -> Option<&TransformationStep> {
        self.steps.iter().find(|s| s.id == id)
    }

    /// Remove a step by ID
    pub fn remove_step(&mut self, id: &str) -> Result<(), TransformationError> {
        let index =
            self.steps.iter().position(|s| s.id == id).ok_or_else(|| {
                TransformationError::InvalidStep(format!("Step {} not found", id))
            })?;

        // Remove the step
        self.steps.remove(index);

        // Rebuild schema from remaining steps
        self.rebuild_schema()?;

        Ok(())
    }

    /// Rebuild schema from steps
    fn rebuild_schema(&mut self) -> Result<(), TransformationError> {
        for step in &self.steps {
            // Clone the schema we need to update
            let step_schema = step.output_schema.clone();

            // Update self's input schema
            self.input_schema = step_schema;
        }

        Ok(())
    }

    /// Validate a transformation step
    fn validate_step(&self, step: &TransformationStep) -> Result<(), TransformationError> {
        match &step.step_type {
            StepType::SelectColumns(columns) => {
                if columns.is_empty() {
                    return Err(TransformationError::InvalidStep(
                        "Select columns must specify at least one column".to_string(),
                    ));
                }
                // Check if columns exist
                let available_columns: HashSet<_> =
                    self.input_schema.iter().map(|s| s.name.as_str()).collect();

                for column in columns {
                    if !available_columns.contains(column.as_str()) {
                        return Err(TransformationError::ColumnNotFound(column.clone()));
                    }
                }
            }
            StepType::FilterRows(column, condition) => {
                if column.is_empty() || condition.is_empty() {
                    return Err(TransformationError::InvalidStep(
                        "Filter rows requires column and condition".to_string(),
                    ));
                }
                // Check if column exists
                if !self.input_schema.iter().any(|s| s.name == *column) {
                    return Err(TransformationError::ColumnNotFound(column.clone()));
                }
            }
            StepType::GroupBy(group_columns, _aggregations) => {
                if group_columns.is_empty() {
                    return Err(TransformationError::InvalidStep(
                        "Group by requires at least one group column".to_string(),
                    ));
                }
                // Check if columns exist
                let available_columns: HashSet<_> =
                    self.input_schema.iter().map(|s| s.name.as_str()).collect();

                for column in group_columns {
                    if !available_columns.contains(column.as_str()) {
                        return Err(TransformationError::ColumnNotFound(column.clone()));
                    }
                }
            }
            StepType::DropColumns(columns) => {
                if columns.is_empty() {
                    return Err(TransformationError::InvalidStep(
                        "Drop columns must specify at least one column".to_string(),
                    ));
                }
                // Check if columns exist
                let available_columns: HashSet<_> =
                    self.input_schema.iter().map(|s| s.name.as_str()).collect();

                for column in columns {
                    if !available_columns.contains(column.as_str()) {
                        return Err(TransformationError::ColumnNotFound(column.clone()));
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Apply a single step to a DataFrame
    fn apply_step(
        &self,
        df: &DataFrame,
        step: &TransformationStep,
    ) -> Result<DataFrame, TransformationError> {
        match &step.step_type {
            StepType::SelectColumns(columns) => {
                let selected_columns: Vec<String> = columns.iter().cloned().collect();

                df.select(&selected_columns)
                    .map_err(|e| TransformationError::DataError(e.to_string()))
            }
            StepType::FilterRows(column, condition) => {
                // Parse condition and filter - simplified version for demo
                let filter_column = column.as_str();

                // Try to parse numeric comparison (simplified)
                if let Some((op, value)) = Self::parse_condition(condition) {
                    match op {
                        "gt" => df
                            .clone()
                            .lazy()
                            .filter(col(filter_column).gt(lit(value)))
                            .collect()
                            .map_err(|e| TransformationError::DataError(e.to_string())),
                        "gte" => df
                            .clone()
                            .lazy()
                            .filter(col(filter_column).gt_eq(lit(value)))
                            .collect()
                            .map_err(|e| TransformationError::DataError(e.to_string())),
                        "lt" => df
                            .clone()
                            .lazy()
                            .filter(col(filter_column).lt(lit(value)))
                            .collect()
                            .map_err(|e| TransformationError::DataError(e.to_string())),
                        "lte" => df
                            .clone()
                            .lazy()
                            .filter(col(filter_column).lt_eq(lit(value)))
                            .collect()
                            .map_err(|e| TransformationError::DataError(e.to_string())),
                        _ => Ok(df.clone()),
                    }
                } else {
                    // Default to keeping all rows if condition can't be parsed
                    Ok(df.clone())
                }
            }
            StepType::GroupBy(group_columns, aggregations) => {
                // Apply group by with aggregations using polars lazy API
                let mut lf = df.clone().lazy();

                for group_col in group_columns {
                    lf = lf.group_by([group_col.as_str()]).agg(
                        &aggregations
                            .iter()
                            .map(|agg| match agg.operation {
                                AggregationOperation::Sum => col(&agg.column).sum(),
                                AggregationOperation::Mean => col(&agg.column).mean(),
                                AggregationOperation::Count => col(&agg.column).count(),
                                AggregationOperation::Min => col(&agg.column).min(),
                                AggregationOperation::Max => col(&agg.column).max(),
                                AggregationOperation::First => col(&agg.column).first(),
                                AggregationOperation::Last => col(&agg.column).last(),
                                AggregationOperation::StdDev => col(&agg.column).std(0),
                                AggregationOperation::Variance => col(&agg.column).var(0),
                            })
                            .collect::<Vec<_>>(),
                    );
                }

                lf.collect()
                    .map_err(|e| TransformationError::DataError(e.to_string()))
            }
            StepType::SortBy(columns, _descending) => {
                let sort_columns: Vec<String> = columns.iter().cloned().collect();

                df.clone()
                    .sort(&sort_columns, SortMultipleOptions::default())
                    .map_err(|e| TransformationError::DataError(e.to_string()))
            }
            StepType::RenameColumn(old_name, new_name) => {
                let mut result = df.clone();
                // Use to_owned() and PlSmallStr::from instead of from_static
                result
                    .rename(old_name.as_str(), PlSmallStr::from(new_name.to_string()))
                    .map_err(|e| TransformationError::DataError(e.to_string()))?;
                Ok(result)
            }
            StepType::DropColumns(columns) => {
                let columns_to_drop: Vec<String> = columns.iter().cloned().collect();

                if columns_to_drop.len() == 1 {
                    df.clone()
                        .drop(&columns_to_drop[0])
                        .map_err(|e| TransformationError::DataError(e.to_string()))
                } else {
                    let mut result = df.clone();
                    for col in &columns_to_drop {
                        result
                            .drop_in_place(col.as_str())
                            .map_err(|e| TransformationError::DataError(e.to_string()))?;
                    }
                    Ok(result)
                }
            }
            StepType::CustomSql(_) => {
                // For now, we'll skip custom SQL implementation
                Ok(df.clone())
            }
            StepType::AddColumn(_, _) => {
                // For now, we'll skip add column implementation
                Ok(df.clone())
            }
            StepType::RemoveDuplicates(keep_first) => {
                let strategy = if *keep_first {
                    UniqueKeepStrategy::First
                } else {
                    UniqueKeepStrategy::Last
                };
                df.clone()
                    .unique_stable(None, strategy, None)
                    .map_err(|e| TransformationError::DataError(e.to_string()))
            }
        }
    }

    /// Parse a simple condition string into components
    fn parse_condition(condition: &str) -> Option<(&'static str, f64)> {
        let trimmed = condition.trim();

        // Try to match patterns like "> 1000", ">= 500", "< 100", etc.
        if let Some(num_str) = trimmed.strip_prefix("> ") {
            return num_str.parse().ok().map(|v| ("gt", v));
        }
        if let Some(num_str) = trimmed.strip_prefix(">= ") {
            return num_str.parse().ok().map(|v| ("gte", v));
        }
        if let Some(num_str) = trimmed.strip_prefix("< ") {
            return num_str.parse().ok().map(|v| ("lt", v));
        }
        if let Some(num_str) = trimmed.strip_prefix("<= ") {
            return num_str.parse().ok().map(|v| ("lte", v));
        }

        None
    }

    /// Update schema based on step
    fn update_schema(&mut self, step: &TransformationStep) {
        self.input_schema = step.output_schema.clone();
    }
}

/// Factory functions for creating common transformations
pub struct TransformationFactory;

impl TransformationFactory {
    /// Create a select columns transformation
    pub fn select_columns(
        name: String,
        columns: Vec<String>,
    ) -> Result<TransformationStep, TransformationError> {
        let step_type = StepType::SelectColumns(columns);

        Ok(TransformationStep {
            id: format!("select_{}", uuid()),
            name,
            step_type,
            parameters: HashMap::new(),
            output_schema: Vec::new(), // Will be set by pipeline
        })
    }

    /// Create a filter rows transformation
    pub fn filter_rows(
        name: String,
        column: String,
        condition: String,
    ) -> Result<TransformationStep, TransformationError> {
        let step_type = StepType::FilterRows(column, condition);

        Ok(TransformationStep {
            id: format!("filter_{}", uuid()),
            name,
            step_type,
            parameters: HashMap::new(),
            output_schema: Vec::new(), // Will be set by pipeline
        })
    }

    /// Create a group by transformation
    pub fn group_by(
        name: String,
        columns: Vec<String>,
        aggregations: Vec<Aggregation>,
    ) -> Result<TransformationStep, TransformationError> {
        let step_type = StepType::GroupBy(columns, aggregations);

        Ok(TransformationStep {
            id: format!("group_{}", uuid()),
            name,
            step_type,
            parameters: HashMap::new(),
            output_schema: Vec::new(), // Will be set by pipeline
        })
    }

    /// Create a sort by transformation
    pub fn sort_by(
        name: String,
        columns: Vec<String>,
        descending: bool,
    ) -> Result<TransformationStep, TransformationError> {
        let step_type = StepType::SortBy(columns, descending);

        Ok(TransformationStep {
            id: format!("sort_{}", uuid()),
            name,
            step_type,
            parameters: HashMap::new(),
            output_schema: Vec::new(), // Will be set by pipeline
        })
    }

    /// Create a rename column transformation
    pub fn rename_column(
        name: String,
        old_name: String,
        new_name: String,
    ) -> Result<TransformationStep, TransformationError> {
        let step_type = StepType::RenameColumn(old_name, new_name);

        Ok(TransformationStep {
            id: format!("rename_{}", uuid()),
            name,
            step_type,
            parameters: HashMap::new(),
            output_schema: Vec::new(), // Will be set by pipeline
        })
    }

    /// Create a drop columns transformation
    pub fn drop_columns(
        name: String,
        columns: Vec<String>,
    ) -> Result<TransformationStep, TransformationError> {
        let step_type = StepType::DropColumns(columns);

        Ok(TransformationStep {
            id: format!("drop_{}", uuid()),
            name,
            step_type,
            parameters: HashMap::new(),
            output_schema: Vec::new(), // Will be set by pipeline
        })
    }
}

/// Generate a simple unique ID
fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}{}", duration.as_secs(), duration.subsec_nanos())
}

/// Serialize pipeline to JSON
pub fn serialize_pipeline(pipeline: &TransformationPipeline) -> Result<String, serde_json::Error> {
    serde_json::to_string(pipeline)
}

/// Deserialize pipeline from JSON
pub fn deserialize_pipeline(json: &str) -> Result<TransformationPipeline, serde_json::Error> {
    serde_json::from_str(json)
}

/// Get available transformations for autocomplete
pub fn get_available_transformations() -> Vec<String> {
    vec![
        "SelectColumns".to_string(),
        "FilterRows".to_string(),
        "GroupBy".to_string(),
        "SortBy".to_string(),
        "RenameColumn".to_string(),
        "DropColumns".to_string(),
        "CustomSql".to_string(),
        "AddColumn".to_string(),
        "RemoveDuplicates".to_string(),
    ]
}

impl fmt::Display for TransformationStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.name, self.step_type)
    }
}

impl fmt::Display for StepType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepType::SelectColumns(cols) => write!(f, "SelectColumns({:?})", cols),
            StepType::FilterRows(col, cond) => write!(f, "FilterRows({}, {})", col, cond),
            StepType::GroupBy(cols, aggs) => write!(f, "GroupBy({:?}, {:?})", cols, aggs),
            StepType::SortBy(cols, desc) => write!(f, "SortBy({:?}, {})", cols, desc),
            StepType::RenameColumn(old, new) => write!(f, "RenameColumn({}, {})", old, new),
            StepType::DropColumns(cols) => write!(f, "DropColumns({:?})", cols),
            StepType::CustomSql(sql) => write!(f, "CustomSql({})", sql),
            StepType::AddColumn(col, formula) => write!(f, "AddColumn({}, {})", col, formula),
            StepType::RemoveDuplicates(keep_first) => {
                write!(f, "RemoveDuplicates({})", keep_first)
            }
        }
    }
}
