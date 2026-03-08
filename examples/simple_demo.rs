//! Nustage - A data processing and analysis toolkit for Rust
//!
//! This example demonstrates Nustage's core data processing capabilities.

//! # Usage
//! ```ignore
//! cargo run --example simple_demo
//! ```

use polars::prelude::*;

/// Create a sample sales DataFrame for demonstration
fn create_sample_sales_data() -> Result<DataFrame, PolarsError> {
    let product = Series::new(
        "Product".into(),
        &[
            "Widget", "Widget", "Gadget", "Gadget", "Sprocket", "Sprocket", "Bracket", "Bracket",
            "Widget", "Gadget",
        ],
    )
    .into();

    let region = Series::new(
        "Region".into(),
        &[
            "North", "South", "East", "West", "North", "South", "East", "West", "North", "South",
        ],
    )
    .into();

    let quantity = Series::new("Quantity".into(), &[10, 20, 15, 25, 12, 18, 8, 10, 14, 22]).into();

    let revenue = Series::new(
        "Revenue".into(),
        &[
            1200.0, 800.0, 1500.0, 950.0, 1100.0, 1050.0, 1300.0, 850.0, 1400.0, 950.0,
        ],
    )
    .into();

    DataFrame::new(10, vec![product, region, quantity, revenue])
}

/// Create a sample expenses DataFrame for demonstration
fn create_sample_expenses_data() -> Result<DataFrame, PolarsError> {
    let category = Series::new(
        "Category".into(),
        &[
            "Travel", "Meals", "Office", "Travel", "Meals", "Office", "Travel", "Meals", "Office",
            "Travel",
        ],
    )
    .into();

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

    let amount = Series::new(
        "Amount".into(),
        &[
            150.0, 85.5, 200.0, 275.0, 120.0, 190.0, 320.0, 95.0, 250.0, 180.0,
        ],
    )
    .into();

    DataFrame::new(10, vec![category, date, amount])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Nustage Simple Demo ===\n");

    // Create and display sales data
    println!("1. Sales Data Demo");
    println!("-------------------");
    {
        let df = create_sample_sales_data()?;
        println!(
            "Created sales DataFrame with {} rows and {} columns",
            df.height(),
            df.width()
        );
        println!("\nSchema:");
        let schema = df.schema();
        for (i, (name, dtype)) in schema.iter().enumerate() {
            println!("  [{}] {}: {}", i, name, dtype);
        }
    }

    // Create and display expenses data
    println!("\n2. Expenses Data Demo");
    println!("---------------------");
    {
        let df = create_sample_expenses_data()?;
        println!(
            "Created expenses DataFrame with {} rows and {} columns",
            df.height(),
            df.width()
        );
        println!("\nSchema:");
        let schema = df.schema();
        for (i, (name, dtype)) in schema.iter().enumerate() {
            println!("  [{}] {}: {}", i, name, dtype);
        }
    }

    // Demonstrate schema inspection
    println!("\n3. Schema Inspection");
    println!("-------------------");
    {
        let df = create_sample_sales_data()?;
        let schema = nustage::data::get_schema(&df)?;
        println!(
            "Schema extracted into {} ColumnSchema entries",
            schema.len()
        );
        for col in &schema {
            println!("  - {}: {}", col.name, col.data_type);
        }
    }

    println!("\n=== Demo Complete ===");

    Ok(())
}
