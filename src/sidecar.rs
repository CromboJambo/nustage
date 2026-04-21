//! Sidecar persistence module
//!
//! Stores pipeline provenance, step history, schema snapshots alongside data output.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sidecar entry for a single pipeline step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarEntry {
    pub step_name: String,
    pub input_columns: Vec<String>,
    pub output_columns: Vec<String>,
    pub transformation_type: String,
}

/// Sidecar state — pipeline provenance and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarState {
    pub pipeline_name: String,
    pub uuid: Uuid,
    pub source_file: String,
    pub output_file: String,
    pub entries: Vec<SidecarEntry>,
}

impl SidecarState {
    /// Create a new sidecar state for a pipeline
    pub fn new(pipeline_name: String, source_file: String, output_file: String) -> Self {
        SidecarState {
            pipeline_name,
            uuid: Uuid::new_v4(),
            source_file,
            output_file,
            entries: Vec::new(),
        }
    }

    /// Add a step entry to the sidecar
    pub fn add_entry(&mut self, entry: SidecarEntry) -> Result<(), Box<dyn std::error::Error>> {
        self.entries.push(entry);
        Ok(())
    }
}

/// Save sidecar state to a JSON file
pub fn save_sidecar(state: &SidecarState, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(state)?;
    std::fs::write(path, content)?;
    Ok(())
}

/// Load sidecar state from a JSON file
pub fn load_sidecar(path: &str) -> Result<SidecarState, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let state = serde_json::from_str(&content)?;
    Ok(state)
}
