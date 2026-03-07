//! WASM Bindings for Nustage Web Interface
//!
//! Provides JavaScript-accessible functions for the web version of Nustage.
//! This module compiles to WASM and serves as the bridge between Rust logic
//! and browser-based UI components.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// Re-export commonly used types for convenience
use crate::data::{ColumnSchema, PipelineError, get_schema, load_data};
use crate::mcode::generate_m_code;
use crate::sidecar::{SidecarFile, StepType as SidecarStepType, AggOperation};
use crate::transformations::{TransformationPipeline, TransformationStep, StepType};

/// JavaScript-accessible wrapper for loading data from a file path
#[wasm_bindgen]
pub fn load_data_js(file_path: &str) -> Result<JsValue, JsValue> {
    wasm_bindgen_futures::spawn_local(async move {
        match load_data(file_path) {
            Ok(df) => {
                let schema = get_schema(&df).unwrap_or_default();

                // Convert to JSON-serializable format
                let result = serde_json::json!({
                    "success": true,
                    "rows": df.height(),
                    "columns": df.width(),
                    "schema": schema.iter().map(|c| {
                        serde_json::json!({
                            "name": c.name,
                            "type": c.data_type
                        })
                    }).collect::<Vec<_>>()
                });

                JsValue::from_str(&serde_json::to_string(&result).unwrap())
            }
            Err(e) => {
                let error = serde_json::json!({
                    "success": false,
                    "error": format!("{}", e)
                });
                JsValue::from_str(&serde_json::to_string(&error).unwrap())
            }
        }
    }).await;

    Ok(JsValue::NULL) // Return immediately, result comes via callback or polling
}

/// JavaScript-accessible function to generate M code from a pipeline
#[wasm_bindgen]
pub fn generate_m_code_js(pipeline_json: &str, source_name: &str) -> Result<String, String> {
    let steps: Vec<TransformationStep> = serde_json::from_str(pipeline_json)
        .map_err(|e| format!("Failed to parse pipeline JSON: {}", e))?;

    Ok(generate_m_code(&steps, source_name))
}

/// JavaScript-accessible function to create a new sidecar file
#[wasm_bindgen]
pub fn create_sidecar(source_path: &str) -> Result<String, String> {
    let sidecar = SidecarFile::new(source_path);

    serde_json::to_string_pretty(&sidecar)
        .map_err(|e| format!("Failed to serialize sidecar: {}", e))
}

/// JavaScript-accessible function to load an existing sidecar file
#[wasm_bindgen]
pub fn load_sidecar(source_path: &str) -> Result<String, String> {
    match SidecarFile::load(source_path) {
        Ok(sidecar) => serde_json::to_string_pretty(&sidecar)
            .map_err(|e| format!("Failed to serialize sidecar: {}", e)),
        Err(e) => Err(format!("Failed to load sidecar: {}", e)),
    }
}

/// JavaScript-accessible function to save a sidecar file
#[wasm_bindgen]
pub fn save_sidecar_json(sidecar_json: &str) -> Result<String, String> {
    let mut sidecar: SidecarFile = serde_json::from_str(sidecar_json)
        .map_err(|e| format!("Failed to parse sidecar JSON: {}", e))?;

    match sidecar.save() {
        Ok(_) => Ok("Sidecar saved successfully".to_string()),
        Err(e) => Err(format!("Failed to save sidecar: {}", e)),
    }
}

/// JavaScript-accessible function to get the sidecar path for a source file
#[wasm_bindgen]
pub fn get_sidecar_path(source_path: &str) -> String {
    SidecarFile::sidecar_path_for_source(source_path).to_string_lossy().into_owned()
}

/// JavaScript-accessible function to generate human-readable diff
#[wasm_bindgen]
pub fn generate_diff_json(original_schema_json: &str, transformed_schema_json: &str) -> Result<String, String> {
    let original: Vec<ColumnSchema> = serde_json::from_str(original_schema_json)
        .map_err(|e| format!("Failed to parse original schema: {}", e))?;

    let transformed: Vec<ColumnSchema> = serde_json::from_str(transformed_schema_json)
        .map_err(|e| format!("Failed to parse transformed schema: {}", e))?;

    let sidecar = SidecarFile::new("temp");
    let diff_text = sidecar.generate_diff(&original, &transformed);

    Ok(diff_text)
}

/// JavaScript-accessible function to get pipeline as text
#[wasm_bindgen]
pub fn pipeline_to_text_json(sidecar_json: &str) -> Result<String, String> {
    let sidecar: SidecarFile = serde_json::from_str(sidecar_json)
        .map_err(|e| format!("Failed to parse sidecar JSON: {}", e))?;

    Ok(sidecar.pipeline_to_text())
}

/// JavaScript-accessible function to add a step to the pipeline (via sidecar)
#[wasm_bindgen]
pub fn add_step_to_sidecar(
    sidecar_json: &str,
    name: &str,
    step_type: &str,
    params_json: &str,
) -> Result<String, String> {
    let mut sidecar: SidecarFile = serde_json::from_str(sidecar_json)
        .map_err(|e| format!("Failed to parse sidecar JSON: {}", e))?;

    // Parse step type from string
    let parsed_step_type: SidecarStepType = match step_type {
        "select_columns" => SidecarStepType::SelectColumns {
            columns: serde_json::from_str(params_json)
                .map_err(|e| format!("Failed to parse columns: {}", e))?,
        },
        "filter_rows" => {
            let params: FilterParams = serde_json::from_str(params_json)
                .map_err(|e| format!("Failed to parse filter params: {}", e))?;
            SidecarStepType::FilterRows {
                column: params.column,
                condition: params.condition,
            }
        },
        "group_by" => {
            let params: GroupByParams = serde_json::from_str(params_json)
                .map_err(|e| format!("Failed to parse group by params: {}", e))?;
            SidecarStepType::GroupBy {
                columns: params.columns,
                aggregations: params.aggregations,
            }
        },
        "sort_by" => {
            let params: SortParams = serde_json::from_str(params_json)
                .map_err(|e| format!("Failed to parse sort params: {}", e))?;
            SidecarStepType::SortBy {
                columns: params.columns,
                descending: params.descending,
            }
        },
        "rename_column" => {
            let params: RenameParams = serde_json::from_str(params_json)
                .map_err(|e| format!("Failed to parse rename params: {}", e))?;
            SidecarStepType::RenameColumn {
                old_name: params.old_name,
                new_name: params.new_name,
            }
        },
        "drop_columns" => SidecarStepType::DropColumns {
            columns: serde_json::from_str(params_json)
                .map_err(|e| format!("Failed to parse drop columns: {}", e))?,
        },
        "add_column" => {
            let params: AddColumnParams = serde_json::from_str(params_json)
                .map_err(|e| format!("Failed to parse add column params: {}", e))?;
            SidecarStepType::AddColumn {
                name: params.name,
                expression: params.expression,
            }
        },
        "remove_duplicates" => SidecarStepType::RemoveDuplicates {
            columns: None, // Will be parsed from params if needed
        },
        _ => return Err(format!("Unknown step type: {}", step_type)),
    };

    match sidecar.add_step(name, parsed_step_type, serde_json::json!({})) {
        Ok(_) => serde_json::to_string_pretty(&sidecar)
            .map_err(|e| format!("Failed to serialize updated sidecar: {}", e)),
        Err(e) => Err(format!("Failed to add step: {}", e)),
    }
}

/// Helper structs for JSON parsing
#[derive(Debug, Deserialize)]
struct FilterParams {
    column: String,
    condition: String,
}

#[derive(Debug, Deserialize)]
struct GroupByParams {
    columns: Vec<String>,
    aggregations: Vec<AggregationParam>,
}

#[derive(Debug, Deserialize)]
struct AggregationParam {
    column: String,
    op: AggOperation,
}

#[derive(Debug, Deserialize)]
struct SortParams {
    columns: Vec<String>,
    descending: bool,
}

#[derive(Debug, Deserialize)]
struct RenameParams {
    old_name: String,
    new_name: String,
}

#[derive(Debug, Deserialize)]
struct AddColumnParams {
    name: String,
    expression: String,
}

/// JavaScript-accessible function to remove a step from the pipeline
#[wasm_bindgen]
pub fn remove_step_from_sidecar(
    sidecar_json: &str,
    step_id: &str,
) -> Result<String, String> {
    let mut sidecar: SidecarFile = serde_json::from_str(sidecar_json)
        .map_err(|e| format!("Failed to parse sidecar JSON: {}", e))?;

    match sidecar.remove_step(step_id) {
        Ok(_) => serde_json::to_string_pretty(&sidecar)
            .map_err(|e| format!("Failed to serialize updated sidecar: {}", e)),
        Err(e) => Err(format!("Failed to remove step: {}", e)),
    }
}

/// JavaScript-accessible function to export pipeline as M code
#[wasm_bindgen]
pub fn export_as_m_code(sidecar_json: &str, source_name: &str) -> Result<String, String> {
    let sidecar: SidecarFile = serde_json::from_str(sidecar_json)
        .map_err(|e| format!("Failed to parse sidecar JSON: {}", e))?;

    // Convert SidecarStepType to TransformationStep for M code generation
    let mut steps: Vec<TransformationStep> = Vec::new();

    for sc_step in &sidecar.pipeline {
        let ts_step = match &sc_step.step_type {
            SidecarStepType::SelectColumns { columns } => TransformationStep {
                id: sc_step.id.clone(),
                name: sc_step.name.clone(),
                step_type: StepType::SelectColumns(columns.clone()),
                parameters: serde_json::json!({}),
                output_schema: vec![],
            },
            SidecarStepType::FilterRows { column, condition } => TransformationStep {
                id: sc_step.id.clone(),
                name: sc_step.name.clone(),
                step_type: StepType::FilterRows(column.clone(), condition.clone()),
                parameters: serde_json::json!({}),
                output_schema: vec![],
            },
            SidecarStepType::GroupBy { columns, aggregations } => TransformationStep {
                id: sc_step.id.clone(),
                name: sc_step.name.clone(),
                step_type: StepType::GroupBy(
                    columns.clone(),
                    aggregations.iter().map(|a| crate::transformations::Aggregation {
                        column: a.column.clone(),
                        operation: match a.operation {
                            AggOperation::Sum => TransformationStep::sum_agg,
                            AggOperation::Mean => TransformationStep::mean_agg,
                            AggOperation::Count => TransformationStep::count_agg,
                            AggOperation::Min => TransformationStep::min_agg,
                            AggOperation::Max => TransformationStep::max_agg,
                            AggOperation::First => TransformationStep::first_agg,
                            AggOperation::Last => TransformationStep::last_agg,
                        },
                    }).collect(),
                ),
                parameters: serde_json::json!({}),
                output_schema: vec![],
            },
            // ... more conversions as needed
            _ => continue, // Skip unsupported types for now
        };

        steps.push(ts_step);
    }

    Ok(generate_m_code(&steps, source_name))
}

// Helper methods on TransformationStep for aggregation operations
impl TransformationStep {
    fn sum_agg() -> crate::transformations::AggregationOperation {
        crate::transformations::AggregationOperation::Sum
    }

    fn mean_agg() -> crate::transformations::AggregationOperation {
        crate::transformations::AggregationOperation::Mean
    }

    fn count_agg() -> crate::transformations::AggregationOperation {
        crate::transformations::AggregationOperation::Count
    }

    fn min_agg() -> crate::transformations::AggregationOperation {
        crate::transformations::AggregationOperation::Min
    }

    fn max_agg() -> crate::transformations::AggregationOperation {
        crate::transformations::AggregationOperation::Max
    }

    fn first_agg() -> crate::transformations::AggregationOperation {
        crate::transformations::AggregationOperation::First
    }

    fn last_agg() -> crate::transformations::AggregationOperation {
        crate::transformations::AggregationOperation::Last
    }
}

/// Initialize console logging for debugging in browser
#[wasm_bindgen(start)]
pub fn init_logging() {
    #[cfg(feature = "console_log")]
    console_error_panic_hook::set_once();

    // Enable verbose console logging if needed
    web_sys::console::log_1(&"Nustage WASM initialized".into());
}

/// Get version information
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

```


Now let me create a web interface structure and update the README to reflect this new direction:

<tool_call>
<function=create_directory>
<parameter=path>
nustage/web
</parameter>
</function>
</tool_call>
