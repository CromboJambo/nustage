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
    ironcalc::{IronCalcIntegration, Transformation, apply_transformation},
    transformations::{TransformationFactory, TransformationPipeline},
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

    // Add select columns transformation
    let select_step = TransformationFactory::select_columns(
        "Select Relevant Columns".to_string(),
        vec![
            "Product".to_string(),
            "Region".to_string(),
            "Sales".to_string(),
            "Date".to_string(),
        ],
    );
    pipeline.add_step(select_step)?;

    // Add filter transformation
    let filter_step = TransformationFactory::filter_rows(
        "Filter High Sales".to_string(),
        "Sales".to_string(),
        "> 1000".to_string(),
    );
    pipeline.add_step(filter_step)?;

    // Add group by transformation
    let group_step = TransformationFactory::group_by(
        "Group by Region".to_string(),
        vec!["Region".to_string()],
        vec![nustage::transformations::Aggregation {
            column: "Sales".to_string(),
            operation: nustage::transformations::AggregationOperation::Sum,
        }],
    );
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
    let sales_data = vec![
        ("Product", "Region", "Sales", "Date"),
        ("Widget", "North", "1200", "2024-01-01"),
        ("Widget", "South", "800", "2024-01-02"),
        ("Gadget", "North", "1500", "2024-01-03"),
        ("Gadget", "East", "950", "2024-01-04"),
        ("Widget", "West", "1100", "2024-01-05"),
        ("Gadget", "South", "1050", "2024-01-06"),
        ("Widget", "East", "1300", "2024-01-07"),
        ("Gadget", "West", "850", "2024-01-08"),
        ("Widget", "North", "1400", "2024-01-09"),
        ("Gadget", "South", "950", "2024-01-10"),
    ];

    let product_series = Series::new(
        "Product",
        sales_data.iter().skip(1).map(|r| r.0).collect::<Vec<_>>(),
    );
    let region_series = Series::new(
        "Region",
        sales_data.iter().skip(1).map(|r| r.1).collect::<Vec<_>>(),
    );
    let sales_series = Series::new(
        "Sales",
        sales_data.iter().skip(1).map(|r| r.2).collect::<Vec<_>>(),
    );
    let date_series = Series::new(
        "Date",
        sales_data.iter().skip(1).map(|r| r.3).collect::<Vec<_>>(),
    );

    DataFrame::new(vec![
        product_series,
        region_series,
        sales_series,
        date_series,
    ])
}
