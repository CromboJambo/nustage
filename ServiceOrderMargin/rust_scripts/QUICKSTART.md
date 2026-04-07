ServiceOrderMargin\rust_scripts\QUICKSTART.md
```

# Quick Start Guide - GL Tie-Out Script

## Overview
This guide will help you get the GL tie-out script up and running in 5 minutes.

## Prerequisites
- Rust installed on your system
- CSV files in the `data/` directory

## Setup Steps

### 1. Navigate to the rust_scripts directory
```bash
cd "C:\Users\dgrover\OneDrive - Delkor Systems Inc\Desktop\Development\ServiceOrderMargin\rust_scripts"
```

### 2. Verify CSV files exist
Ensure you have these files in the `data/` subdirectory:
- `GLPostedTransactions_51120.csv`
- `LABOR_ServiceOrderTransactions.csv`
- `MATERIAL_ServiceOrderTransactions.csv`
- `MISC_ServiceOrderTransactions.csv`

### 3. Build the project
**Windows:**
```bash
build.bat
```

**Linux/macOS:**
```bash
chmod +x build.sh
./build.sh
```

**Manual build:**
```bash
cargo build --release
```

### 4. Run the script
**Windows:**
```bash
target\release\gl_tie_out.exe
```

**Linux/macOS:**
```bash
./target/release/gl_tie_out
```

## Expected Output

After running the script, you'll find:
1. A console output showing:
   - Number of GL transactions processed
   - Number of service order transactions processed
   - Summary statistics

2. A detailed report at `output/gl_tie_out_report.txt` containing:
   - Transaction summaries
   - Cost and sell analysis
   - GL vs. Service Order tie-out details

## Troubleshooting

### "data directory not found"
**Solution:** Ensure you have a `data/` directory with all required CSV files.

### "cargo not found"
**Solution:** Install Rust from https://rustup.rs/

### Build errors
**Solution:** Make sure you have a stable version of Rust installed:
```bash
rustup update stable
```

## Next Steps

1. Review the generated report in `output/gl_tie_out_report.txt`
2. Customize the script for your specific needs
3. Check the `README.md` for detailed information
4. Adjust the CSV file paths if needed

## Tips

- The script handles large CSV files efficiently
- All amounts are parsed with proper decimal handling
- Quoted fields and special characters are supported
- Detailed error messages are provided for debugging

## Support

For more details, see:
- `README.md` - Complete documentation
- `gl_tie_out.rs` - Source code with detailed comments

Happy tying out! 🚀
