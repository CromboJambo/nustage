// Simple Nustage Demo
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Nustage Simple Demo ===");

    // Create columns using Series with PlSmallStr names (polars 0.53 API)
    let product = Series::new("Product".into(), &["Widget", "Gadget", "Sprocket"]);
    let region = Series::new("Region".into(), &["North", "South", "East", "West"]);
    let sales = Series::new(
        "Sales".into(),
        &[1200i32, 800, 1500, 950, 1100, 1050, 1300, 850],
    );

    // Convert Series to Column and create DataFrame
    let columns: Vec<Column> = vec![product.into(), region.into(), sales.into()];
    let df = DataFrame::new(4, columns)?;

    println!(
        "Created DataFrame with {} rows and {} columns",
        df.height(),
        df.width()
    );
    Ok(())
}
