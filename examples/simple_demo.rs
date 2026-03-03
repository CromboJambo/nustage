# Simple Nustage Demo
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Nustage Simple Demo ===");
    let df = DataFrame::new(vec![
        Series::new("Product", &["Widget", "Gadget", "Sprocket"]),
        Series::new("Region", &["North", "South", "East", "West"]),
        Series::new("Sales", &[1200i32, 800, 1500, 950, 1100, 1050, 1300, 850]),
    ])?;
    println!("Created DataFrame with {} rows and {} columns", df.height(), df.width());
    Ok(())
}
