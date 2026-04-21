//! Nustage - A data processing and analysis toolkit for Rust
//!
//! This is the main entry point for the nustage CLI application.

use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "nustage")]
#[command(version = "0.1.3")]
#[command(author = "Nustage Team")]
#[command(about = "Terminal-native pipeline orchestration layer for tabular data")]
struct Cli {
    /// Input data file (CSV or Parquet)
    #[arg(value_name = "FILE")]
    input: Option<String>,

    /// Output file path
    #[arg(long = "output", value_name = "OUTPUT_FILE")]
    output: Option<String>,

    /// Sidecar metadata file path
    #[arg(long = "sidecar", value_name = "SIDECAR_FILE")]
    sidecar: Option<String>,

    /// Export xlsx sheets to CSV files
    #[arg(long = "export", value_name = "OUTPUT_DIR")]
    export: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Nustage CLI started");

    if let Some(input_file) = cli.input {
        if Path::new(&input_file).exists() {
            println!("Processing input file: {}", input_file);

            let ext = input_file.rsplit('.').next().unwrap_or_default();
            match ext {
                "xlsx" | "xlsm" | "xlsb" | "xls" | "xla" | "xlam" => {
                    let workbook = nustage::data::open_excel(&input_file)?;
                    println!("Sheets: {:?}", nustage::data::sheet_names(&workbook));
                    let formulas = nustage::data::defined_names(&workbook);
                    println!("Defined names: {:?}", formulas);
                    println!("Excel loaded");
                }
                "csv" => {
                    println!("CSV loading path");
                }
                "parquet" => {
                    println!("Parquet loading path");
                }
                _ => {
                    println!("Unknown file type");
                }
            }

            if let Some(sidecar_path) = cli.sidecar {
                let pipeline_name = "Pipeline".to_string();
                let output_file = cli
                    .output
                    .clone()
                    .unwrap_or_else(|| "default_output".to_string());
                let state = nustage::sidecar::SidecarState::new(
                    pipeline_name.clone(),
                    input_file.clone(),
                    output_file,
                );
                println!("Sidecar state initialized for {}", pipeline_name);
                nustage::sidecar::save_sidecar(&state, &sidecar_path)?;
                println!("Sidecar saved to {}", sidecar_path);
            }

            if let Some(output_dir) = cli.export {
                fs::create_dir_all(&output_dir)?;
                let csv_paths = nustage::export::export_xlsx(&input_file, &output_dir)?;
                println!("Exported {} sheets to CSV:", csv_paths.len());
                for p in csv_paths {
                    println!("  {}", p);
                }
            }

            println!("File processing completed");
        } else {
            eprintln!("Error: Input file '{}' does not exist", input_file);
        }
    } else {
        println!("No input file specified. Running demo mode.");

        // Run simple demo
        run_demo()?;
    }

    Ok(())
}

fn run_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Nustage Demo ===");

    // This would be replaced with actual demo logic
    println!("Demo completed successfully");

    Ok(())
}
