# IronCalc Integration Documentation

## Overview

**Version:** 0.1.2 — First honest public release

This document describes the IronCalc compatibility layer integration with Nustage. This provides read-only Excel support without claiming full spreadsheet engine capabilities.

## Current Status

### Implemented ✅

- **Read-only Excel support** — IronCalc can load .xlsx files as input sources
- **Schema introspection** — Field names and types available for transformations
- **Data loading pipeline** — CSV → Polars → transformation → output

### Aspirational / Not Yet Implemented ⏳

- **Spreadsheet UI rendering** — Grid viewing handled by Tabiew (external tool)
- **Cell-based formula evaluation** — Not part of core Nustage model
- **Data persistence** — Transformed data exported, not persisted in Excel sheets
- **Cell formatting and styling** — Export formatting only
- **Grid-based interface operations** — Nustage is pipeline-focused, not grid-focused

## Architecture

```
IronCalc Compatibility Layer (Read-Only)
  ↓
Nustage (Power Query Layer)
  ↓
DuckDB (Data Processing)
  ↓
Data Sources (CSV, Excel, Parquet, etc.)
  ↓
Output (CSV, Parquet, Excel, TSV)
```

## How It Works

### 1. IronCalc as the Compatibility Layer

IronCalc is used for:
- **Read operations only** — Open existing Excel/CSV files as input sources
- **Data extraction** — Convert Excel sheets to Polars DataFrames
- **Export format** — Save transformed data back to Excel format

**Note:** IronCalc is not the computational layer. Data never mutates in source files.

### 2. Nustage as the Core Engine

Nustage provides:
- Step-based data transformation model (implemented)
- Schema-aware data loading (implemented)
- Transformation pipeline management (implemented)
- DuckDB-powered data processing (implemented)
- Reversible, immutable transformations (implemented)

### 3. Integration Points

#### Data Loading

```rust
// Load from CSV (recommended for demo)
let df = DataFrame::from_csv("data.csv")?;

// Load from Parquet (full support)
let df = DataFrame::from_parquet("data.parquet")?;

// Load from Excel via IronCalc (read-only support)
// Currently aspirational — see limitations below
```

#### Transformation Pipeline

```rust
// Core transformations are implemented:
let pipeline = TransformationPipeline::new("Analysis");

// Filter (implemented)
pipeline.add_step(TransformationFactory::filter_rows("Amount", "> 1000")?);

// Add Column (implemented)
pipeline.add_step(TransformationFactory::add_column("Profit", "@Amount - @Cost")?);

// Group By (implemented)
pipeline.add_step(TransformationFactory::group_by("Region")?);

// Select Columns (implemented)
pipeline.add_step(TransformationFactory::select_columns(vec!["Product", "Region"])?);

// Sort (implemented)
pipeline.add_step(TransformationFactory::sort_by("Amount", true)?);
```

#### Schema Awareness

**Implemented:**
- Field name introspection from loaded data
- Type information from Polars schema
- Basic field metadata display
- Schema validation against transform operations

**Aspirational:**
- Autocomplete for field names (basic awareness exists)
- Type information for formula assistance
- Advanced field metadata display
- Schema validation

## Key Benefits

### For Terminal Users

- **Keyboard-driven interface** — Full mouse-free operation
- **Lightweight and fast** — Rust-based performance
- **Scriptable and composable** — Easy automation
- **Terminal-native UX** — Seamless integration with CLI workflows

### For Power Query Users

- **Step-based model** — Clear, reversible transformations (implemented)
- **Immutable operations** — No side effects (implemented)
- **Versionable pipelines** — Plan for sidecar format (aspirational)
- **Schema awareness** — Smart completion and validation (basic implemented)

### For Developers

- **Rust-based** — High performance and safety
- **Embeddable** — Can be integrated into other applications
- **Open architecture** — Easy to extend with custom transformations
- **Type-safe** — Compile-time error checking

## Usage Examples

### Basic Pipeline (Working)

```rust
use nustage::transformations::{TransformationFactory, TransformationPipeline};
use polars::prelude::*;

// Load data (CSV recommended for demo)
let df = DataFrame::from_csv("data.csv")?;

// Create pipeline
let mut pipeline = TransformationPipeline::new("Sales Analysis");

// Add filter
pipeline.add_step(TransformationFactory::filter_rows("Amount", "> 1000")?)?;

// Add column
pipeline.add_step(TransformationFactory::add_column("Profit", "@Amount - @Cost")?)?;

// Group by and aggregate
pipeline.add_step(TransformationFactory::group_by("Region")?)?;

// Select columns
pipeline.add_step(TransformationFactory::select_columns(vec!["Region", "Profit"])?);

// Run pipeline
let result_df = pipeline.apply(&df)?;

// Preview result
println!("{:?}", result_df);
```

### Running from CLI

```bash
# Build
cargo build --release

# Run with CSV (recommended for demo)
./target/release/nustage test_data/sales.csv

# Run with Parquet
./target/release/nustage test_data/*.parquet

# Run TUI mode (grid preview via Tabiew)
./target/release/nustage --tui test_data/sales.csv
```

### Examples Directory

```bash
# Simple demo example
cargo run --release --example simple_demo

# IronCalc integration example
cargo run --release --example ironcalc_integration
```

## Data Flow

### Loading Data

1. User provides file path (CSV, Excel, or Parquet)
2. Nustage loads data into Polars DataFrame
3. Schema is inferred and displayed
4. Pipeline steps are defined

### Transformations

1. User defines transformation steps
2. Nustage validates steps against schema
3. Steps are applied to DataFrame
4. Resulting DataFrame is displayed in TUI (row count, shape)

### Saving Data

1. Export transformed data to desired format (CSV, Parquet, Excel, TSV)
2. IronCalc handles Excel format conversion (aspirational write support)
3. User can open in Excel for further analysis

## API Reference

### Transformation Pipeline

#### `TransformationPipeline`

Main pipeline struct for managing transformation steps.

**Methods:**
- `new(name: &str) -> Self` — Create a new pipeline with given name
- `add_step(step: TransformationStep) -> Result<Self, Error>` — Add transformation step
- `apply(source: &DataFrame) -> Result<DataFrame, Error>` — Execute pipeline on source data
- `get_schema() -> &Schema` — Get current pipeline schema

#### `TransformationFactory`

Static factory for creating transformation steps.

**Methods:**
- `filter_rows(column: &str, condition: &str) -> Result<TransformationStep, Error>`
- `select_columns(columns: Vec<String>) -> Result<TransformationStep, Error>`
- `add_column(name: &str, expression: &str) -> Result<TransformationStep, Error>`
- `group_by(columns: Vec<String>) -> Result<TransformationStep, Error>`
- `sort_by(column: &str, descending: bool) -> Result<TransformationStep, Error>`

### IronCalc Compatibility Layer (Aspirational)

#### `IronCalcIntegration`

Main compatibility struct for Excel I/O.

**Implemented Methods:**
- Load Excel file and convert to Polars DataFrame

**Aspirational Methods:**
- `from_dataframe(df: DataFrame) -> Result<Self, Error>` — Create from DataFrame
- `get_sheet_cells(...) -> Result<Vec<Cell>, Error>` — Retrieve cells from sheet
- `get_sheet_columns(...) -> Result<Vec<Column>, Error>` — Retrieve column metadata
- `save(file_path: &str) -> Result<(), Error>` — Save workbook to file

## Known Limitations

### Read-Only Excel Support

- **Current state:** IronCalc supports reading Excel files, not writing with full fidelity
- **Workaround:** Export transformed data as CSV or Parquet
- **Future:** Full read/write support when IronCalc capabilities mature

### Sidecar Format

- **Current state:** Pipeline definitions stored in code or CLI arguments
- **Aspirational:** `.nustage.json` sidecar files for version control

### SQL Transparency

- **Current state:** Generated SQL not displayed in TUI
- **Aspirational:** SQL transparency display for learning and debugging

### Real Cell Rendering

- **Current state:** Grid viewing handled by Tabiew (external tool)
- **Aspirational:** In-process cell rendering in TUI

### Charts and Visualization

- **Current state:** Not implemented
- **Future:** Export charts to external tools or files

## Error Handling

All integration functions return `Result` types with descriptive errors:

```rust
pub enum NustageError {
    DataError(String),
    SchemaError(String),
    TransformError(String),
    FileError(String),
}
```

Example:
```rust
let df = DataFrame::from_csv("data.csv").map_err(|e| NustageError::DataError(e.to_string()))?;
```

## Best Practices

1. **Use CSV/Parquet for demo** — Excel support is aspirational for writing
2. **Use transformation pipelines** — For complex data processing
3. **Leverage schema awareness** — For autocomplete and validation
4. **Handle errors gracefully** — Use `?` operator for clean error propagation
5. **Export to desired format** — CSV, Parquet, or Excel as needed
6. **Plan for sidecar** — When version control is needed

## Contributing

When contributing to the IronCalc integration:

1. Follow the existing code style
2. Add tests for new features
3. Update this documentation
4. Consider backward compatibility
5. Provide usage examples

## See Also

- [`README.md`](../README.md) — Project overview and current implementation status
- [`QUICKSTART.md`](../QUICKSTART.md) — Build and run instructions
- [`ROADMAP.md`](../ROADMAP.md) — Prioritized feature list
- [`COMPREHENSIVE_ROADMAP.md`](../COMPREHENSIVE_ROADMAP.md) — Detailed feature specifications

## Version History

- **0.1.2** — First honest release with clear separation of implemented vs aspirational features
- **0.1.1** — Initial public release (now superseded)

## Support

For issues, questions, or contributions:
- Check the examples directory for usage patterns
- Review the API documentation in `src/transformations/`
- Open an issue on the project repository
- Submit a pull request with tests

---

**Version:** 0.1.2  
**Status:** Honest Documentation Release  
**Note:** Features marked as "Aspirational" are documented but not yet implemented.