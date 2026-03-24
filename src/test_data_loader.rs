// Simple test to verify that data loading works properly

use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "FILE")]
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Testing data loading from: {}", cli.input);

    // Test if file exists
    let path = Path::new(&cli.input);
    if !path.exists() {
        eprintln!("File not found: {}", cli.input);
        return Err("File not found".into());
    }

    // Try to load the data using your existing function
    match nustage::data::load_data(&cli.input) {
        Ok(df) => {
            println!(
                "Successfully loaded {} rows with {} columns",
                df.height(),
                df.width()
            );

            // Show first few rows of schema
            let schema = nustage::data::get_schema(&df).unwrap();
            println!("\nSchema:");
            for col in &schema {
                println!("  - {}: {}", col.name, col.data_type);
            }

            // Show sample data if not too large
            if df.height() > 0 {
                println!("\nFirst row data:");
                for col in &schema {
                    let column_name = &col.name;
                    match df.column(column_name) {
                        Ok(col_data) => {
                            if !col_data.is_empty() {
                                // Just show first value
                                let first_value =
                                    col_data.get(0).unwrap_or(polars::prelude::AnyValue::Null);
                                println!("  {}: {:?}", column_name, first_value);
                            }
                        }
                        Err(_) => println!("  {}: Error reading column", column_name),
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to load data: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
