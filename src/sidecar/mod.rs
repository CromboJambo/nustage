//! Sidecar File Handling (.nustage.json)
//!
//! Manages the companion JSON file that stores pipeline definitions,
//! schema information, and transformation history alongside data files.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

/// The main sidecar file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarFile {
    /// Version of the sidecar format (for future compatibility)
    pub version: u32,

    /// Reference to the original data source file
    pub source: String,

    /// Pipeline steps that transform the data
    pub pipeline: Vec<TransformationStep>,

    /// Schema information at each step for diffing
    #[serde(default)]
    pub schema_history: HashMap<String, ColumnSchema>,

    /// Metadata about when this sidecar was created/modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SidecarMetadata>,
}

/// Transformation step in the pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationStep {
    /// Unique identifier for this step
    pub id: String,

    /// Human-readable name (e.g., "filter_revenue", "group_by_region")
    pub name: String,

    /// Type of transformation (filter, group_by, etc.)
    #[serde(rename = "type")]
    pub step_type: StepType,

    /// Additional parameters for this step
    #[serde(default)]
    pub params: serde_json::Value,
}

/// Types of transformations available
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum StepType {
    SelectColumns {
        columns: Vec<String>,
    },
    FilterRows {
        column: String,
        condition: String,
    },
    GroupBy {
        columns: Vec<String>,
        aggregations: Vec<Aggregation>,
    },
    SortBy {
        columns: Vec<String>,
        descending: bool,
    },
    RenameColumn {
        old_name: String,
        new_name: String,
    },
    DropColumns {
        columns: Vec<String>,
    },
    AddColumn {
        name: String,
        expression: String,
    },
    RemoveDuplicates {
        columns: Option<Vec<String>>,
    },
}

/// Aggregation operation for group by steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregation {
    pub column: String,
    #[serde(rename = "op")]
    pub operation: AggOperation,
}

/// Available aggregation operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggOperation {
    Sum,
    Mean,
    Count,
    Min,
    Max,
    First,
    Last,
}

/// Metadata about the sidecar file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarMetadata {
    /// When this sidecar was first created
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Last time it was modified
    pub modified_at: chrono::DateTime<chrono::Utc>,

    /// User who last modified it (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Column schema for diffing purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
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
    /// Create a new sidecar file pointing to a data source
    pub fn new(source: &str) -> Self {
        Self {
            version: 1,
            source: source.to_string(),
            pipeline: Vec::new(),
            schema_history: HashMap::new(),
            metadata: Some(SidecarMetadata {
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
                user: None,
            }),
        }
    }

    /// Load sidecar file from disk (if it exists)
    pub fn load(source_path: &str) -> Result<Self, SidecarError> {
        let sidecar_path = Self::sidecar_path_for_source(source_path);

        if !sidecar_path.exists() {
            return Err(SidecarError::NotFound(sidecar_path));
        }

        let content = fs::read_to_string(&sidecar_path)
            .map_err(|e| SidecarError::Io(e, sidecar_path.clone()))?;

        serde_json::from_str(&content).map_err(|e| SidecarError::Parse(e, sidecar_path))
    }

    /// Save sidecar file to disk
    pub fn save(&self) -> Result<(), SidecarError> {
        let sidecar_path = Self::sidecar_path_for_source(&self.source);

        // Ensure parent directory exists
        if let Some(parent) = sidecar_path.parent() {
            fs::create_dir_all(parent).map_err(|e| SidecarError::Io(e, sidecar_path.clone()))?;
        }

        let content = serde_json::to_string_pretty(self).map_err(SidecarError::Serialize)?;

        fs::write(&sidecar_path, &content).map_err(|e| SidecarError::Io(e, sidecar_path))
    }

    /// Get the path for a sidecar file given the source data file path
    pub fn sidecar_path_for_source(source_path: &str) -> PathBuf {
        let mut path = PathBuf::from(source_path);

        // Remove extension if present
        path.set_extension("");

        // Add .nustage.json extension
        path.set_extension("nustage.json");

        path
    }

    /// Get the original source file path from sidecar
    pub fn source_path(&self) -> PathBuf {
        PathBuf::from(&self.source)
    }

    /// Add a transformation step to the pipeline
    pub fn add_step(
        &mut self,
        name: &str,
        step_type: StepType,
        params: serde_json::Value,
    ) -> Result<(), SidecarError> {
        let id = Uuid::new_v4().to_string();

        self.pipeline.push(TransformationStep {
            id,
            name: name.to_string(),
            step_type,
            params,
        });

        // Update metadata
        if let Some(ref mut meta) = self.metadata {
            meta.modified_at = chrono::Utc::now();
        } else {
            self.metadata = Some(SidecarMetadata {
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
                user: None,
            });
        }

        Ok(())
    }

    /// Remove a transformation step by ID
    pub fn remove_step(&mut self, step_id: &str) -> Result<(), SidecarError> {
        let index = self
            .pipeline
            .iter()
            .position(|s| s.id == step_id)
            .ok_or_else(|| SidecarError::InvalidStep(step_id.to_string()))?;

        self.pipeline.remove(index);

        if let Some(ref mut meta) = self.metadata {
            meta.modified_at = chrono::Utc::now();
        }

        Ok(())
    }

    /// Get the last step in the pipeline
    pub fn last_step(&self) -> Option<&TransformationStep> {
        self.pipeline.last()
    }

    /// Generate a human-readable diff from original to current state
    pub fn generate_diff(
        &self,
        original_schema: &[ColumnSchema],
        current_schema: &[ColumnSchema],
    ) -> String {
        let mut lines = vec![String::from("=== Schema Changes ===")];

        // Find removed columns (in original but not in current)
        let original_names: std::collections::HashSet<&str> =
            original_schema.iter().map(|c| c.name.as_str()).collect();

        for current in &current_schema {
            if !original_names.contains(current.name.as_str()) {
                lines.push(format!("  + Added column: {}", current.name));
            }
        }

        // Find new columns (in original but not in current)
        let current_names: std::collections::HashSet<&str> =
            current_schema.iter().map(|c| c.name.as_str()).collect();

        for original in original_schema {
            if !current_names.contains(original.name.as_str()) {
                lines.push(format!("  - Removed column: {}", original.name));
            }
        }

        // Find type changes
        let orig_map: std::collections::HashMap<&str, &str> = original_schema
            .iter()
            .map(|c| (c.name.as_str(), c.data_type.as_str()))
            .collect();

        for current in &current_schema {
            if let Some(orig) = orig_map.get(current.name.as_str()) {
                if *orig != current.data_type.as_str() {
                    lines.push(format!(
                        "  ~ Type changed: {} from {} to {}",
                        current.name, orig, current.data_type
                    ));
                }
            }
        }

        lines.join("\n")
    }

    /// Get pipeline steps as plain text for human reading
    pub fn pipeline_to_text(&self) -> String {
        let mut lines = vec![String::from("=== Pipeline Steps ===")];

        for (index, step) in self.pipeline.iter().enumerate() {
            lines.push(format!(
                "{}. {} ({})",
                index + 1,
                step.name,
                match &step.step_type {
                    StepType::SelectColumns { columns } => format!("select {}", columns.join(", ")),
                    StepType::FilterRows { column, condition } => {
                        format!("filter {}: {}", column, condition)
                    }
                    StepType::GroupBy { columns, .. } =>
                        format!("group by: {}", columns.join(", ")),
                    StepType::SortBy {
                        columns,
                        descending,
                    } => {
                        let dir = if *descending { "desc" } else { "asc" };
                        format!("sort {}: {}", columns.join(", "), dir)
                    }
                    StepType::RenameColumn { old_name, new_name } => {
                        format!("rename: {} -> {}", old_name, new_name)
                    }
                    StepType::DropColumns { columns } => format!("drop: {}", columns.join(", ")),
                    StepType::AddColumn { name, expression } => {
                        format!("add column {}: {}", name, expression)
                    }
                    StepType::RemoveDuplicates { .. } => "remove duplicates".to_string(),
                }
            ));
        }

        lines.join("\n")
    }
}

/// Error types for sidecar file operations
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
            .add_step(
                "filter_revenue",
                StepType::FilterRows {
                    column: "Revenue".to_string(),
                    condition: "> 1000".to_string(),
                },
                serde_json::json!({}),
            )
            .unwrap();

        let text = sidecar.pipeline_to_text();
        assert!(text.contains("filter Revenue"));
        assert!(text.contains("> 1000"));
    }

    #[test]
    fn test_generate_diff() {
        let original = vec![
            ColumnSchema {
                name: "A".to_string(),
                data_type: "int".to_string(),
            },
            ColumnSchema {
                name: "B".to_string(),
                data_type: "str".to_string(),
            },
        ];

        let current = vec![
            ColumnSchema {
                name: "A".to_string(),
                data_type: "int".to_string(),
            },
            ColumnSchema {
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
