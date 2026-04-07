# GL Tie-Out Script

This Rust script ties out GL posted transactions to service order transactions for cost and sell analysis.

## Files

- `gl_tie_out.rs` - Main Rust script that reads and processes CSV files
- `Cargo.toml` - Rust project configuration
- `README.md` - This file

## CSV Files Required

The script expects CSV files in the `data/` directory:

1. `GLPostedTransactions_51120.csv` - General Ledger posted transactions
2. `LABOR_ServiceOrderTransactions.csv` - Labor service order transactions
3. `MATERIAL_ServiceOrderTransactions.csv` - Material service order transactions
4. `MISC_ServiceOrderTransactions.csv` - Miscellaneous service order transactions

## Setup

1. Ensure you have Rust installed on your system
2. Navigate to the rust_scripts directory
3. Run the following command to build the project:

```bash
cargo build --release
```

## Usage

1. Place your CSV files in the `data/` directory
2. Run the compiled binary:

```bash
cargo run --release
```

Or if you've already built it:

```bash
./target/release/gl_tie_out
```

## Output

The script generates a tie-out report at `output/gl_tie_out_report.txt` that includes:

- Summary statistics for all transaction types
- Detailed tie-out between GL transactions and service orders
- Cost and sell analysis for each transaction
- Matching information between GL vouchers and service order invoices

## Features

- Parses CSV files with complex data structures
- Handles quoted fields and special characters
- Provides detailed error handling
- Generates comprehensive reports for analysis
- Supports multiple transaction types (labor, material, misc, GL)

## Error Handling

The script will provide detailed error messages if:

- CSV files are not found in the data directory
- CSV files are malformed
- Required fields are missing
- Parsing fails

## Troubleshooting

If you encounter issues:

1. Check that all CSV files are present in the data/ directory
2. Verify CSV file format matches expected structure
3. Review error messages for specific issues
4. Ensure Rust and Cargo are properly installed