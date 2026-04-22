//! Power Query-style workflow example
//!
//! This example mirrors a common Power Query flow:
//! input -> transforms -> summary -> report output.
//!
//! # Usage
//! ```ignore
//! cargo run --example power_query_workflow
//! ```

use std::fs::File;
use std::path::Path;

use nustage::data;
use nustage::sidecar;
use nustage::transformations::{
    Aggregation, AggregationOperation, ColumnSchema as TransformColumnSchema,
    TransformationFactory, TransformationPipeline,
};
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Power Query Workflow Example ===\n");

    // Input: create a sample dataset similar to sales data you would load in Power Query.
    let df = create_sample_sales_data()?;
    println!(
        "Input DataFrame: {} rows, {} columns",
        df.height(),
        df.width()
    );

    // Transforms: select, filter, and group-by summary.
    let mut pipeline = TransformationPipeline::new("Sales Summary".to_string());
    pipeline.input_schema = data::get_schema(&df)?
        .into_iter()
        .map(|c| TransformColumnSchema {
            index: c.index,
            name: c.name,
            data_type: c.data_type,
        })
        .collect();

    let select_step = TransformationFactory::select_columns(
        "Select columns".to_string(),
        vec![
            "Date".to_string(),
            "Region".to_string(),
            "Product".to_string(),
            "Units".to_string(),
            "Revenue".to_string(),
        ],
    )?;
    pipeline.add_step(select_step)?;

    let filter_step = TransformationFactory::filter_rows(
        "Filter high revenue".to_string(),
        "Revenue".to_string(),
        "> 1000".to_string(),
    )?;
    pipeline.add_step(filter_step)?;

    let group_step = TransformationFactory::group_by(
        "Group by region".to_string(),
        vec!["Region".to_string()],
        vec![
            Aggregation {
                column: "Revenue".to_string(),
                operation: AggregationOperation::Sum,
            },
            Aggregation {
                column: "Units".to_string(),
                operation: AggregationOperation::Sum,
            },
        ],
    )?;
    pipeline.add_step(group_step)?;

    // Summary: apply pipeline to produce aggregated output.
    let mut summary_df = pipeline.apply(&df)?;
    println!(
        "Summary DataFrame: {} rows, {} columns",
        summary_df.height(),
        summary_df.width()
    );

    // Report output: write summary as a CSV report.
    let output_path = "report_output.csv";
    if Path::new(output_path).exists() {
        let _ = std::fs::remove_file(output_path);
    }
    let mut file = File::create(output_path)?;
    CsvWriter::new(&mut file).finish(&mut summary_df)?;

    println!("Report written to {}", output_path);

    let sidecar_path = "sidecar_output.json";
    let mut sidecar_state = sidecar::SidecarState::new(
        "Sales Summary".to_string(),
        "sales.csv".to_string(),
        output_path.to_string(),
    );
    sidecar_state.add_entry(sidecar::SidecarEntry {
        step_name: "Select columns".to_string(),
        input_columns: Vec::new(),
        output_columns: vec![
            "Date".to_string(),
            "Region".to_string(),
            "Product".to_string(),
            "Units".to_string(),
            "Revenue".to_string(),
        ],
        transformation_type: "select".to_string(),
    })?;
    sidecar_state.add_entry(sidecar::SidecarEntry {
        step_name: "Filter high revenue".to_string(),
        input_columns: vec![
            "Date".to_string(),
            "Region".to_string(),
            "Product".to_string(),
            "Units".to_string(),
            "Revenue".to_string(),
        ],
        output_columns: vec![
            "Date".to_string(),
            "Region".to_string(),
            "Product".to_string(),
            "Units".to_string(),
            "Revenue".to_string(),
        ],
        transformation_type: "filter".to_string(),
    })?;
    sidecar_state.add_entry(sidecar::SidecarEntry {
        step_name: "Group by region".to_string(),
        input_columns: vec![
            "Date".to_string(),
            "Region".to_string(),
            "Product".to_string(),
            "Units".to_string(),
            "Revenue".to_string(),
        ],
        output_columns: vec![
            "Region".to_string(),
            "Revenue".to_string(),
            "Units".to_string(),
        ],
        transformation_type: "group_by".to_string(),
    })?;
    sidecar::save_sidecar(&sidecar_state, sidecar_path)?;
    println!("Sidecar saved to {}", sidecar_path);

    println!("\n=== Example Complete ===");

    Ok(())
}

fn create_sample_sales_data() -> Result<DataFrame, PolarsError> {
    let date = Series::new(
        "Date".into(),
        &[
            "2024-01-01",
            "2024-01-02",
            "2024-01-03",
            "2024-01-04",
            "2024-01-05",
            "2024-01-06",
            "2024-01-07",
            "2024-01-08",
            "2024-01-09",
            "2024-01-10",
        ],
    )
    .into();

    let region = Series::new(
        "Region".into(),
        &[
            "North", "South", "North", "East", "West", "South", "East", "West", "North", "South",
        ],
    )
    .into();

    let product = Series::new(
        "Product".into(),
        &[
            "Widget", "Widget", "Gadget", "Gadget", "Widget", "Gadget", "Widget", "Gadget",
            "Widget", "Gadget",
        ],
    )
    .into();

    let units = Series::new("Units".into(), &[10, 8, 14, 6, 12, 9, 7, 11, 13, 10]).into();

    let revenue = Series::new(
        "Revenue".into(),
        &[
            1200.0, 800.0, 1500.0, 950.0, 1100.0, 1050.0, 1300.0, 850.0, 1400.0, 950.0,
        ],
    )
    .into();

    DataFrame::new(10, vec![date, region, product, units, revenue])
}
