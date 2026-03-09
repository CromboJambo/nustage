//! IronCalc Integration Example
//!
//! This example demonstrates how to integrate IronCalc with nustage's
//! Power Query-style data transformation layer.
//!
//! # Usage
//! ```
//! cargo run --example ironcalc_integration
//! ```

use nustage::{
    ironcalc::IronCalcIntegration,
    transformations::{
        Aggregation, AggregationOperation, ColumnSchema, TransformationFactory,
        TransformationPipeline,
    },
};
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== IronCalc Integration Example ===\n");

    // Step 1: Create a sample DataFrame
    println!("1. Creating sample DataFrame...");
    let df = create_sample_dataframe()?;
    println!(
        "   Created DataFrame with {} rows and {} columns",
        df.height(),
        df.width()
    );

    // Step 2: Load data into IronCalc
    println!("\n2. Loading DataFrame into IronCalc...");
    let ironcalc = IronCalcIntegration::from_dataframe(df.clone())?;
    println!("   Workbook created with schema:");
    for schema in ironcalc.get_schema() {
        println!("     - {}: {}", schema.name, schema.data_type);
    }

    // Step 3: Apply Power Query-style transformations
    println!("\n3. Applying transformations...");
    let mut pipeline = TransformationPipeline::new("Sales Analysis".to_string());
    pipeline.input_schema = ironcalc
        .get_schema()
        .iter()
        .map(|c| ColumnSchema {
            index: c.index,
            name: c.name.clone(),
            data_type: c.data_type.clone(),
        })
        .collect();

    // Add select columns transformation
    let mut select_step = TransformationFactory::select_columns(
        "Select Relevant Columns".to_string(),
        vec![
            "Product".to_string(),
            "Region".to_string(),
            "Sales".to_string(),
            "Date".to_string(),
        ],
    )?;
    select_step.output_schema = pipeline
        .input_schema
        .iter()
        .filter(|c| matches!(c.name.as_str(), "Product" | "Region" | "Sales" | "Date"))
        .cloned()
        .collect();
    pipeline.add_step(select_step)?;

    // Add filter transformation
    let mut filter_step = TransformationFactory::filter_rows(
        "Filter High Sales".to_string(),
        "Sales".to_string(),
        "> 1000".to_string(),
    )?;
    filter_step.output_schema = pipeline.input_schema.clone();
    pipeline.add_step(filter_step)?;

    // Add group by transformation
    let mut group_step = TransformationFactory::group_by(
        "Group by Region".to_string(),
        vec!["Region".to_string()],
        vec![Aggregation {
            column: "Sales".to_string(),
            operation: AggregationOperation::Sum,
        }],
    )?;
    group_step.output_schema = vec![
        ColumnSchema {
            index: 0,
            name: "Region".to_string(),
            data_type: "str".to_string(),
        },
        ColumnSchema {
            index: 1,
            name: "Sales".to_string(),
            data_type: "i32".to_string(),
        },
    ];
    pipeline.add_step(group_step)?;

    println!("   Added {} transformation steps", pipeline.steps.len());
    for step in &pipeline.steps {
        println!("     - {}", step.name);
    }

    // Step 4: Apply transformations to DataFrame
    println!("\n4. Applying transformations to DataFrame...");
    let transformed_df = pipeline.apply(&df)?;
    println!(
        "   Transformed DataFrame has {} rows and {} columns",
        transformed_df.height(),
        transformed_df.width()
    );

    // Step 5: Save to Excel file
    println!("\n5. Saving to Excel file...");
    let output_path = "example_output.xlsx";

    // Remove existing file if it exists (demo mode)
    if std::path::Path::new(output_path).exists() {
        let _ = std::fs::remove_file(output_path);
        println!("   Removed existing file: {}", output_path);
    }

    ironcalc.save(output_path)?;
    println!("   Workbook saved to: {}", output_path);

    // Step 6: Demonstrate schema awareness
    println!("\n6. Schema awareness features:");
    println!("   Available fields for autocomplete:");
    for field in ironcalc.get_schema() {
        println!("     - {}", field.name);
    }

    println!("\n=== Example Complete ===");
    Ok(())
}

/// Create a sample DataFrame for demonstration
fn create_sample_dataframe() -> Result<DataFrame, polars::prelude::PolarsError> {
    let product_series = Series::new(
        "Product".into(),
        &[
            "Widget", "Widget", "Gadget", "Gadget", "Widget", "Gadget", "Widget", "Gadget",
            "Widget", "Gadget",
        ],
    );
    let region_series = Series::new(
        "Region".into(),
        &[
            "North", "South", "North", "East", "West", "South", "East", "West", "North", "South",
        ],
    );
    let sales_series = Series::new(
        "Sales".into(),
        &[1200i32, 800, 1500, 950, 1100, 1050, 1300, 850, 1400, 950],
    );
    let date_series = Series::new(
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
    );

    let columns: Vec<Column> = vec![
        product_series.into(),
        region_series.into(),
        sales_series.into(),
        date_series.into(),
    ];
    DataFrame::new(10, columns)
}
