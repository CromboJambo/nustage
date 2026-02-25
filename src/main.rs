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
    println!("File path received: {}", cli.input.display());

    // At this point, we just verify the CLI parsing works
    println!("✅ Main binary executed successfully");

    Ok(())
}
