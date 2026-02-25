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

    println!("TUI mode - File path received: {}", cli.input.display());
    println!("✅ TUI binary executed successfully");

    Ok(())
}
