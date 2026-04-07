//! Nustage - A data processing and analysis toolkit for Rust
//!
//! This is the test data loader module for the nustage project.

use polars::prelude::*;
use std::fs::File;
use std::io::BufReader;

/// Load CSV data from a file
pub fn load_csv_data(path: &str) -> Result<DataFrame, PolarsError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    CsvReader::from_reader(reader).has_header(true).finish()
}

/// Load Parquet data from a file
pub fn load_parquet_data(path: &str) -> Result<DataFrame, PolarsError> {
    let file = File::open(path)?;
    ParquetReader::from_reader(file).finish()
}

/// Create sample sales data for testing
pub fn create_sample_sales_data() -> Result<DataFrame, PolarsError> {
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

/// Create sample expenses data for testing
pub fn create_sample_expenses_data() -> Result<DataFrame, PolarsError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sample_sales_data() {
        let df = create_sample_sales_data().unwrap();
        assert_eq!(df.height(), 10);
        assert_eq!(df.width(), 4);
    }

    #[test]
    fn test_create_sample_expenses_data() {
        let df = create_sample_expenses_data().unwrap();
        assert_eq!(df.height(), 10);
        assert_eq!(df.width(), 3);
    }
}
