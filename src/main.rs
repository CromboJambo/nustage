use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file path (CSV, Excel, Parquet)
    #[arg(value_name = "FILE")]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Testing data loading from: {:?}", cli.input);

    // Load the data using your existing function
    match nustage::data::load_data(&cli.input.to_string_lossy()) {
        Ok(df) => {
            println!(
                "✅ Successfully loaded {} rows with {} columns",
                df.height(),
                df.width()
            );

            // Show schema information
            let schema = nustage::data::get_schema(&df).unwrap();
            println!("\nSchema:");
            for col in &schema {
                println!("  - {}: {}", col.name, col.data_type);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to load data: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
