use clap::Parser;
use nustage::cli::{Cli, Commands, validate_input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    validate_input(&cli.input).map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

    if cli.tui {
        return nustage::tui::run(&cli.input);
    }

    let df = nustage::data::load_data(&cli.input.to_string_lossy())?;

    match &cli.command {
        Some(Commands::Schema) | None if cli.schema => {
            let schema = nustage::data::get_schema(&df)?;
            println!("Schema ({} columns):", schema.len());
            for col in &schema {
                println!("  [{:>2}] {}: {}", col.index, col.name, col.data_type);
            }
        }
        Some(Commands::Analyze { column }) => {
            let schema = nustage::data::get_schema(&df)?;
            match column {
                Some(col_name) => {
                    println!("Column analysis: {col_name}");
                    if let Ok(series) = df.column(col_name) {
                        println!("  null count : {}", series.null_count());
                        println!("  len        : {}", series.len());
                    }
                }
                None => {
                    println!(
                        "DataFrame: {} rows × {} columns",
                        df.height(),
                        df.width()
                    );
                    for col in &schema {
                        println!("  {}: {}", col.name, col.data_type);
                    }
                }
            }
        }
        Some(Commands::Unique { column }) => {
            let series = df.column(column)?;
            let unique = series.unique()?;
            println!("Unique values in '{column}' ({} total):", unique.len());
            for i in 0..unique.len().min(50) {
                println!("  {:?}", unique.get(i)?);
            }
        }
        Some(Commands::Schema) => {
            let schema = nustage::data::get_schema(&df)?;
            println!("Schema ({} columns):", schema.len());
            for col in &schema {
                println!("  [{:>2}] {}: {}", col.index, col.name, col.data_type);
            }
        }
        Some(Commands::Export { format, output }) => {
            match format.as_str() {
                "csv" => {
                    let mut file = std::fs::File::create(output)?;
                    polars::prelude::CsvWriter::new(&mut file).finish(&mut df.clone())?;
                    println!("Exported to CSV: {}", output.display());
                }
                "parquet" => {
                    let mut file = std::fs::File::create(output)?;
                    polars::prelude::ParquetWriter::new(&mut file).finish(&mut df.clone())?;
                    println!("Exported to Parquet: {}", output.display());
                }
                other => {
                    eprintln!("Unsupported export format: {other}. Use csv or parquet.");
                    std::process::exit(1);
                }
            }
        }
        None => {
            // Default: show a preview
            let preview_rows = cli.preview.min(df.height());
            println!(
                "Loaded {} rows × {} columns (showing {preview_rows}):",
                df.height(),
                df.width()
            );
            println!("{}", df.head(Some(preview_rows)));
        }
    }

    Ok(())
}
