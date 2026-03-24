//! Sidecar File Handling (.nustage.json)
//!
//! Stores the canonical transformation pipeline alongside the source data.

use crate::transformations::{ColumnSchema, StepType, TransformationStep};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

/// The main sidecar file structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarFile {
    /// Version of the sidecar format.
    pub version: u32,
    /// Reference to the original data source file.
    pub source: String,
    /// Canonical transformation steps for this source.
    #[serde(default)]
    pub pipeline: Vec<TransformationStep>,
    /// Optional schema snapshots keyed by step id.
    #[serde(default)]
    pub schema_history: HashMap<String, Vec<ColumnSchema>>,
    /// Metadata about when this sidecar was created or modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SidecarMetadata>,
}

/// Metadata about the sidecar file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for SidecarFile {
    fn default() -> Self {
        Self {
            version: 1,
            source: String::new(),
            pipeline: Vec::new(),
            schema_history: HashMap::new(),
            metadata: None,
        }
    }
}

impl SidecarFile {
    /// Create a new sidecar file pointing to a data source.
    pub fn new(source: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            version: 1,
            source: source.to_string(),
            pipeline: Vec::new(),
            schema_history: HashMap::new(),
            metadata: Some(SidecarMetadata {
                created_at: now,
                modified_at: now,
                user: None,
            }),
        }
    }

    /// Load sidecar file from disk.
    pub fn load(source_path: &str) -> Result<Self, SidecarError> {
        let sidecar_path = Self::sidecar_path_for_source(source_path);

        if !sidecar_path.exists() {
            return Err(SidecarError::NotFound(sidecar_path));
        }

        let content = fs::read_to_string(&sidecar_path)
            .map_err(|e| SidecarError::Io(e, sidecar_path.clone()))?;

        serde_json::from_str(&content).map_err(|e| SidecarError::Parse(e, sidecar_path))
    }

    /// Save sidecar file to disk.
    pub fn save(&self) -> Result<(), SidecarError> {
        let sidecar_path = Self::sidecar_path_for_source(&self.source);

        if let Some(parent) = sidecar_path.parent() {
            fs::create_dir_all(parent).map_err(|e| SidecarError::Io(e, sidecar_path.clone()))?;
        }

        let content = serde_json::to_string_pretty(self).map_err(SidecarError::Serialize)?;

        fs::write(&sidecar_path, &content).map_err(|e| SidecarError::Io(e, sidecar_path))
    }

    /// Get the sidecar path associated with a source data file path.
    pub fn sidecar_path_for_source(source_path: &str) -> PathBuf {
        let mut path = PathBuf::from(source_path);
        path.set_extension("");
        path.set_extension("nustage.json");
        path
    }

    /// Get the original source file path from this sidecar.
    pub fn source_path(&self) -> PathBuf {
        PathBuf::from(&self.source)
    }

    /// Add a canonical transformation step to the pipeline.
    pub fn add_step(&mut self, mut step: TransformationStep) -> Result<(), SidecarError> {
        if step.id.is_empty() {
            step.id = Uuid::new_v4().to_string();
        }

        self.touch_metadata();
        self.pipeline.push(step);
        Ok(())
    }

    /// Create and add a transformation step with minimal boilerplate.
    pub fn add_step_from_parts(
        &mut self,
        name: &str,
        step_type: StepType,
    ) -> Result<&TransformationStep, SidecarError> {
        let step = TransformationStep {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            step_type,
            parameters: HashMap::new(),
            output_schema: Vec::new(),
        };

        self.touch_metadata();
        self.pipeline.push(step);
        self.pipeline
            .last()
            .ok_or_else(|| SidecarError::InvalidStep("Pipeline unexpectedly empty".to_string()))
    }

    /// Remove a transformation step by id.
    pub fn remove_step(&mut self, step_id: &str) -> Result<(), SidecarError> {
        let index = self
            .pipeline
            .iter()
            .position(|s| s.id == step_id)
            .ok_or_else(|| SidecarError::InvalidStep(step_id.to_string()))?;

        self.pipeline.remove(index);
        self.schema_history.remove(step_id);
        self.touch_metadata();
        Ok(())
    }

    /// Get the last step in the pipeline.
    pub fn last_step(&self) -> Option<&TransformationStep> {
        self.pipeline.last()
    }

    /// Store a schema snapshot for a step.
    pub fn record_schema(&mut self, step_id: &str, schema: Vec<ColumnSchema>) {
        self.schema_history.insert(step_id.to_string(), schema);
        self.touch_metadata();
    }

    /// Generate a human-readable diff from original to current state.
    pub fn generate_diff(
        &self,
        original_schema: &[ColumnSchema],
        current_schema: &[ColumnSchema],
    ) -> String {
        let mut lines = vec![String::from("=== Schema Changes ===")];

        let original_names: std::collections::HashSet<&str> =
            original_schema.iter().map(|c| c.name.as_str()).collect();
        let current_names: std::collections::HashSet<&str> =
            current_schema.iter().map(|c| c.name.as_str()).collect();

        for current in current_schema {
            if !original_names.contains(current.name.as_str()) {
                lines.push(format!("  + Added column: {}", current.name));
            }
        }

        for original in original_schema {
            if !current_names.contains(original.name.as_str()) {
                lines.push(format!("  - Removed column: {}", original.name));
            }
        }

        let original_types: std::collections::HashMap<&str, &str> = original_schema
            .iter()
            .map(|c| (c.name.as_str(), c.data_type.as_str()))
            .collect();

        for current in current_schema {
            if let Some(original_type) = original_types.get(current.name.as_str())
                && *original_type != current.data_type.as_str()
            {
                lines.push(format!(
                    "  ~ Type changed: {} from {} to {}",
                    current.name, original_type, current.data_type
                ));
            }
        }

        lines.join("\n")
    }

    /// Get pipeline steps as plain text for human reading.
    pub fn pipeline_to_text(&self) -> String {
        let mut lines = vec![String::from("=== Pipeline Steps ===")];

        for (index, step) in self.pipeline.iter().enumerate() {
            lines.push(format!(
                "{}. {} ({})",
                index + 1,
                step.name,
                describe_step_type(&step.step_type)
            ));
        }

        lines.join("\n")
    }

    fn touch_metadata(&mut self) {
        let now = chrono::Utc::now();
        if let Some(ref mut meta) = self.metadata {
            meta.modified_at = now;
        } else {
            self.metadata = Some(SidecarMetadata {
                created_at: now,
                modified_at: now,
                user: None,
            });
        }
    }
}

fn describe_step_type(step_type: &StepType) -> String {
    match step_type {
        StepType::SelectColumns(columns) => format!("select {}", columns.join(", ")),
        StepType::FilterRows(column, condition) => format!("filter {}: {}", column, condition),
        StepType::GroupBy(columns, aggregations) => format!(
            "group by {} -> {}",
            columns.join(", "),
            aggregations
                .iter()
                .map(|agg| format!("{:?}({})", agg.operation, agg.column))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        StepType::SortBy(columns, descending) => format!(
            "sort {}: {}",
            columns.join(", "),
            if *descending { "desc" } else { "asc" }
        ),
        StepType::RenameColumn(old_name, new_name) => {
            format!("rename: {} -> {}", old_name, new_name)
        }
        StepType::DropColumns(columns) => format!("drop: {}", columns.join(", ")),
        StepType::CustomSql(sql) => format!("sql: {}", sql),
        StepType::AddColumn(name, expression) => format!("add column {}: {}", name, expression),
        StepType::RemoveDuplicates(all_columns) => {
            if *all_columns {
                "remove duplicates: all columns".to_string()
            } else {
                "remove duplicates".to_string()
            }
        }
    }
}

/// Error types for sidecar file operations.
#[derive(Debug, thiserror::Error)]
pub enum SidecarError {
    #[error("Sidecar file not found: {}", _0.display())]
    NotFound(PathBuf),
    #[error("IO error: {} (file: {})", _0, _1.display())]
    Io(std::io::Error, PathBuf),
    #[error("Parse error: {} (file: {})", _0, _1.display())]
    Parse(serde_json::Error, PathBuf),
    #[error("Serialization error: {}", _0)]
    Serialize(serde_json::Error),
    #[error("Invalid step ID: {}", _0)]
    InvalidStep(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{Aggregation, AggregationOperation};

    #[test]
    fn test_sidecar_path_for_source() {
        let paths = vec![
            ("/path/to/data.csv", "/path/to/data.nustage.json"),
            ("data.xlsx", "data.nustage.json"),
            ("/home/user/file.parquet", "/home/user/file.nustage.json"),
        ];

        for (source, expected) in paths {
            let result = SidecarFile::sidecar_path_for_source(source);
            assert_eq!(result.to_str().unwrap(), expected);
        }
    }

    #[test]
    fn test_new_sidecar() {
        let sidecar = SidecarFile::new("test_data.csv");
        assert_eq!(sidecar.source, "test_data.csv");
        assert!(sidecar.metadata.is_some());
        assert!(sidecar.pipeline.is_empty());
    }

    #[test]
    fn test_pipeline_to_text() {
        let mut sidecar = SidecarFile::new("test.csv");

        sidecar
            .add_step_from_parts(
                "filter_revenue",
                StepType::FilterRows("Revenue".to_string(), "> 1000".to_string()),
            )
            .unwrap();

        let text = sidecar.pipeline_to_text();
        assert!(text.contains("filter Revenue"));
        assert!(text.contains("> 1000"));
    }

    #[test]
    fn test_group_by_render() {
        let mut sidecar = SidecarFile::new("test.csv");

        sidecar
            .add_step_from_parts(
                "group_revenue",
                StepType::GroupBy(
                    vec!["Region".to_string()],
                    vec![Aggregation {
                        column: "Revenue".to_string(),
                        operation: AggregationOperation::Sum,
                    }],
                ),
            )
            .unwrap();

        let text = sidecar.pipeline_to_text();
        assert!(text.contains("Sum(Revenue)"));
    }

    #[test]
    fn test_generate_diff() {
        let original = vec![
            ColumnSchema {
                index: 0,
                name: "A".to_string(),
                data_type: "int".to_string(),
            },
            ColumnSchema {
                index: 1,
                name: "B".to_string(),
                data_type: "str".to_string(),
            },
        ];

        let current = vec![
            ColumnSchema {
                index: 0,
                name: "A".to_string(),
                data_type: "int".to_string(),
            },
            ColumnSchema {
                index: 1,
                name: "C".to_string(),
                data_type: "float".to_string(),
            },
        ];

        let sidecar = SidecarFile::new("test.csv");
        let diff = sidecar.generate_diff(&original, &current);

        assert!(diff.contains("+ Added column: C"));
        assert!(diff.contains("- Removed column: B"));
    }
}
