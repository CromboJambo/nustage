use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationStep {
    pub id: String,
    pub name: String,
    pub step_type: StepType,
    pub parameters: HashMap<String, String>,
    pub output_schema: Vec<ColumnSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepType {
    SelectColumns(Vec<String>),
    FilterRows(String, String),
    GroupBy(Vec<String>, Vec<Aggregation>),
    SortBy(Vec<String>, bool),
    RenameColumn(String, String),
    DropColumns(Vec<String>),
    CustomSql(String),
    AddColumn(String, String),
    RemoveDuplicates(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregation {
    pub column: String,
    pub operation: AggregationOperation,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub index: usize,
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationPipeline {
    pub name: String,
    pub steps: Vec<TransformationStep>,
    /// Tracks the schema as it evolves through each applied step.
    pub current_schema: Vec<ColumnSchema>,
}

impl TransformationPipeline {
    pub fn new(name: String) -> Self {
        Self {
            name,
            steps: Vec::new(),
            current_schema: Vec::new(),
        }
    }

    pub fn add_step(&mut self, mut step: TransformationStep) -> Result<(), TransformationError> {
        self.validate_step(&step)?;
        step.output_schema = self.compute_output_schema(&step.step_type)?;
        self.current_schema = step.output_schema.clone();
        self.steps.push(step);
        Ok(())
    }

    pub fn apply(&self, df: &DataFrame) -> Result<DataFrame, TransformationError> {
        let mut result = df.clone();
        for step in &self.steps {
            result = self.apply_step(&result, step)?;
        }
        Ok(result)
    }

    /// Returns the schema as it stands after all applied steps.
    pub fn output_schema(&self) -> Vec<ColumnSchema> {
        self.current_schema.clone()
    }

    pub fn get_step(&self, id: &str) -> Option<&TransformationStep> {
        self.steps.iter().find(|s| s.id == id)
    }

    pub fn remove_step(&mut self, id: &str) {
        self.steps.retain(|s| s.id != id);
        // Recompute current_schema from the last remaining step.
        self.current_schema = self
            .steps
            .last()
            .map(|s| s.output_schema.clone())
            .unwrap_or_default();
    }

    fn validate_step(&self, step: &TransformationStep) -> Result<(), TransformationError> {
        if step.output_schema.is_empty()
            && !matches!(
                &step.step_type,
                StepType::SelectColumns(_) | StepType::DropColumns(_) | StepType::FilterRows(_, _)
            )
        {
            return Err(TransformationError::InvalidStep(
                "Output schema must be set for this step type".to_string(),
            ));
        }
        Ok(())
    }

    fn apply_step(
        &self,
        df: &DataFrame,
        step: &TransformationStep,
    ) -> Result<DataFrame, TransformationError> {
        match &step.step_type {
            StepType::SelectColumns(columns) => df
                .clone()
                .select(columns.as_slice())
                .map_err(|e| TransformationError::DataError(e.to_string())),

            StepType::FilterRows(column, condition) => {
                let filter_column = column.as_str();
                if let Some((op, value)) = Self::parse_condition(condition) {
                    let result = match op {
                        "gt" => df
                            .clone()
                            .lazy()
                            .filter(col(filter_column).gt(lit(value)))
                            .collect(),
                        "gte" => df
                            .clone()
                            .lazy()
                            .filter(col(filter_column).gt_eq(lit(value)))
                            .collect(),
                        "lt" => df
                            .clone()
                            .lazy()
                            .filter(col(filter_column).lt(lit(value)))
                            .collect(),
                        "lte" => df
                            .clone()
                            .lazy()
                            .filter(col(filter_column).lt_eq(lit(value)))
                            .collect(),
                        _ => Ok(df.clone()),
                    };
                    result.map_err(|e| TransformationError::DataError(e.to_string()))
                } else {
                    Ok(df.clone())
                }
            }

            StepType::GroupBy(group_columns, aggregations) => {
                let group_exprs = group_columns.iter().map(|c| col(c)).collect::<Vec<_>>();

                let agg_exprs = aggregations
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
                    .collect::<Vec<_>>();

                df.clone()
                    .lazy()
                    .group_by(group_exprs)
                    .agg(agg_exprs)
                    .collect()
                    .map_err(|e| TransformationError::DataError(e.to_string()))
            }

            StepType::SortBy(columns, descending) => {
                let descending_flags = vec![*descending; columns.len()];
                df.clone()
                    .sort(
                        columns.as_slice(),
                        SortMultipleOptions {
                            descending: descending_flags,
                            ..Default::default()
                        },
                    )
                    .map_err(|e| TransformationError::DataError(e.to_string()))
            }

            StepType::RenameColumn(old_name, new_name) => {
                let mut result = df.clone();
                result
                    .rename(old_name.as_str(), new_name.clone().into())
                    .map_err(|e| TransformationError::DataError(e.to_string()))?;
                Ok(result)
            }

            StepType::DropColumns(columns) => {
                if columns.len() == 1 {
                    df.clone()
                        .drop(&columns[0])
                        .map_err(|e| TransformationError::DataError(e.to_string()))
                } else {
                    let mut result = df.clone();
                    for c in columns {
                        result
                            .drop_in_place(c.as_str())
                            .map_err(|e| TransformationError::DataError(e.to_string()))?;
                    }
                    Ok(result)
                }
            }

            StepType::CustomSql(sql) => Err(TransformationError::InvalidStep(format!(
                "CustomSql is not yet implemented in the Polars pipeline. SQL: {sql}"
            ))),

            StepType::AddColumn(name, expr) => Err(TransformationError::InvalidStep(format!(
                "AddColumn '{name}' with expression '{expr}' is not yet implemented"
            ))),

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

    fn parse_condition(condition: &str) -> Option<(&'static str, f64)> {
        let trimmed = condition.trim();
        if let Some(num_str) = trimmed.strip_prefix(">= ") {
            return num_str.parse().ok().map(|v| ("gte", v));
        }
        if let Some(num_str) = trimmed.strip_prefix("> ") {
            return num_str.parse().ok().map(|v| ("gt", v));
        }
        if let Some(num_str) = trimmed.strip_prefix("<= ") {
            return num_str.parse().ok().map(|v| ("lte", v));
        }
        if let Some(num_str) = trimmed.strip_prefix("< ") {
            return num_str.parse().ok().map(|v| ("lt", v));
        }
        None
    }

    fn compute_output_schema(
        &self,
        step_type: &StepType,
    ) -> Result<Vec<ColumnSchema>, TransformationError> {
        let input_schema = &self.current_schema;

        match step_type {
            StepType::SelectColumns(columns) => {
                if input_schema.is_empty() {
                    return Ok(Vec::new());
                }
                let output = columns
                    .iter()
                    .filter_map(|col_name| input_schema.iter().find(|c| c.name == *col_name))
                    .cloned()
                    .collect();
                Ok(output)
            }
            StepType::DropColumns(columns) => {
                if input_schema.is_empty() {
                    return Ok(Vec::new());
                }
                Ok(input_schema
                    .iter()
                    .filter(|c| !columns.contains(&c.name))
                    .cloned()
                    .collect())
            }
            StepType::RenameColumn(old_name, new_name) => {
                if input_schema.is_empty() {
                    return Ok(Vec::new());
                }
                Ok(input_schema
                    .iter()
                    .map(|c| {
                        if c.name == *old_name {
                            ColumnSchema {
                                name: new_name.clone(),
                                ..c.clone()
                            }
                        } else {
                            c.clone()
                        }
                    })
                    .collect())
            }
            StepType::AddColumn(name, _) => {
                let mut output = input_schema.clone();
                output.push(ColumnSchema {
                    index: output.len(),
                    name: name.clone(),
                    data_type: "Unknown".to_string(),
                });
                Ok(output)
            }
            StepType::GroupBy(group_columns, aggregations) => {
                let mut output = Vec::new();
                for group_col in group_columns {
                    if let Some(c) = input_schema.iter().find(|c| c.name == *group_col) {
                        output.push(c.clone());
                    }
                }
                for agg in aggregations {
                    let op_name = match agg.operation {
                        AggregationOperation::Sum => "sum",
                        AggregationOperation::Mean => "mean",
                        AggregationOperation::Count => "count",
                        AggregationOperation::Min => "min",
                        AggregationOperation::Max => "max",
                        AggregationOperation::First => "first",
                        AggregationOperation::Last => "last",
                        AggregationOperation::StdDev => "stddev",
                        AggregationOperation::Variance => "variance",
                    };
                    output.push(ColumnSchema {
                        index: output.len(),
                        name: format!("{op_name}_{}", agg.column),
                        data_type: "Unknown".to_string(),
                    });
                }
                Ok(output)
            }
            StepType::FilterRows(_, _)
            | StepType::SortBy(_, _)
            | StepType::CustomSql(_)
            | StepType::RemoveDuplicates(_) => Ok(input_schema.clone()),
        }
    }
}

pub struct TransformationFactory;

impl TransformationFactory {
    pub fn select_columns(
        name: String,
        columns: Vec<String>,
    ) -> Result<TransformationStep, TransformationError> {
        Ok(TransformationStep {
            id: new_id("select"),
            name,
            step_type: StepType::SelectColumns(columns),
            parameters: HashMap::new(),
            output_schema: Vec::new(),
        })
    }

    pub fn filter_rows(
        name: String,
        column: String,
        condition: String,
    ) -> Result<TransformationStep, TransformationError> {
        Ok(TransformationStep {
            id: new_id("filter"),
            name,
            step_type: StepType::FilterRows(column, condition),
            parameters: HashMap::new(),
            output_schema: Vec::new(),
        })
    }

    pub fn group_by(
        name: String,
        columns: Vec<String>,
        aggregations: Vec<Aggregation>,
    ) -> Result<TransformationStep, TransformationError> {
        Ok(TransformationStep {
            id: new_id("group"),
            name,
            step_type: StepType::GroupBy(columns, aggregations),
            parameters: HashMap::new(),
            output_schema: Vec::new(),
        })
    }

    pub fn sort_by(
        name: String,
        columns: Vec<String>,
        descending: bool,
    ) -> Result<TransformationStep, TransformationError> {
        Ok(TransformationStep {
            id: new_id("sort"),
            name,
            step_type: StepType::SortBy(columns, descending),
            parameters: HashMap::new(),
            output_schema: Vec::new(),
        })
    }

    pub fn rename_column(
        name: String,
        old_name: String,
        new_name: String,
    ) -> Result<TransformationStep, TransformationError> {
        Ok(TransformationStep {
            id: new_id("rename"),
            name,
            step_type: StepType::RenameColumn(old_name, new_name),
            parameters: HashMap::new(),
            output_schema: Vec::new(),
        })
    }

    pub fn drop_columns(
        name: String,
        columns: Vec<String>,
    ) -> Result<TransformationStep, TransformationError> {
        Ok(TransformationStep {
            id: new_id("drop"),
            name,
            step_type: StepType::DropColumns(columns),
            parameters: HashMap::new(),
            output_schema: Vec::new(),
        })
    }
}

/// Generate a unique step ID using the `uuid` crate.
fn new_id(prefix: &str) -> String {
    format!("{}_{}", prefix, uuid::Uuid::new_v4())
}

pub fn serialize_pipeline(pipeline: &TransformationPipeline) -> Result<String, serde_json::Error> {
    serde_json::to_string(pipeline)
}

pub fn deserialize_pipeline(json: &str) -> Result<TransformationPipeline, serde_json::Error> {
    serde_json::from_str(json)
}

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
            StepType::SelectColumns(cols) => write!(f, "SelectColumns({cols:?})"),
            StepType::FilterRows(col, cond) => write!(f, "FilterRows({col}, {cond})"),
            StepType::GroupBy(cols, aggs) => write!(f, "GroupBy({cols:?}, {aggs:?})"),
            StepType::SortBy(cols, desc) => write!(f, "SortBy({cols:?}, {desc})"),
            StepType::RenameColumn(old, new) => write!(f, "RenameColumn({old}, {new})"),
            StepType::DropColumns(cols) => write!(f, "DropColumns({cols:?})"),
            StepType::CustomSql(sql) => write!(f, "CustomSql({sql})"),
            StepType::AddColumn(col, formula) => write!(f, "AddColumn({col}, {formula})"),
            StepType::RemoveDuplicates(keep_first) => {
                write!(f, "RemoveDuplicates({keep_first})")
            }
        }
    }
}
