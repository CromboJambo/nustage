# Nustage - Terminal-Native Spreadsheet with Power Query-Style Transformations

**Nustage** is a powerful terminal-native spreadsheet application that combines **IronCalc**'s spreadsheet engine with nustage's Power Query-style data transformation layer. This combination creates a lightweight, keyboard-driven spreadsheet solution with powerful data manipulation capabilities.

## 🎯 Key Features

### Spreadsheet Engine (IronCalc)
- Full spreadsheet UI rendering and navigation
- Cell-based formula evaluation
- Data persistence (.xlsx, .xls formats)
- Cell formatting and styling
- Grid-based interface operations

### Power Query Layer (Nustage)
- Step-based data transformation model
- Schema-aware data loading
- Transformation pipeline management
- DuckDB-powered data processing
- Reversible, immutable transformations

### Integration Benefits
- **Keyboard-driven interface**: Full mouse-free operation
- **Lightweight and fast**: Rust-based performance
- **Scriptable and composable**: Easy automation
- **Terminal-native UX**: Seamless integration with CLI workflows
- **Step-based model**: Clear, reversible transformations
- **Versionable pipelines**: Save and replay transformations
- **Schema awareness**: Smart completion and validation

## 📦 Architecture

```
IronCalc (UI & Spreadsheet Engine)
  ↓
Nustage (Power Query Layer)
  ↓
DuckDB (Data Processing)
  ↓
Data Sources (CSV, Excel, Parquet, etc.)
```

## 🚀 Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/nustage.git
cd nustage

# Build the project
cargo build --release

# Run the application
cargo run --release
```

### Installing Dependencies

Nustage uses the following main dependencies:

```toml
[dependencies]
# Spreadsheet Engine
ironcalc = { git = "https://github.com/ironcalc/ironcalc", features = ["serde"] }

# Data Processing
polars = { version = "0.53", features = ["parquet", "csv"] }
duckdb = { version = "1.1", features = ["bundled"] }
calamine = "0.33"

# TUI Framework
ratatui = "0.29"
crossterm = "0.27"

# CLI Argument Parsing
clap = { version = "4.5", features = ["derive"] }

# Utilities
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.10", features = ["serde", "v4"] }
chrono = { version = "0.4", features = ["serde"] }
```

## 📖 Quick Start

### Basic Usage

```rust
use nustage::{
    ironcalc::{IronCalcIntegration, Transformation},
    transformations::{TransformationFactory, TransformationPipeline},
};
use polars::prelude::*;

// Load data into IronCalc
let df = DataFrame::from_csv("data.csv")?;
let ironcalc = IronCalcIntegration::from_dataframe(df)?;

// Apply transformations
let mut pipeline = TransformationPipeline::new("Analysis".to_string());
pipeline.add_step(TransformationFactory::select_columns(
    "SelectColumns".to_string(),
    vec!["A".to_string(), "B".to_string()],
))?;

// Save to Excel
ironcalc.save("output.xlsx")?;
```

### Running Examples

```bash
# Run the IronCalc integration example
cargo run --example ironcalc_integration

# Run the transformation pipeline example
cargo run --example transformation_pipeline

# Run the TUI example
cargo run --example tui_demo
```

## 🔧 API Reference

### IronCalcIntegration

Main integration struct that bridges IronCalc and nustage.

**Methods:**

- `from_dataframe(df: DataFrame) -> Result<Self, IronCalcError>`
  - Creates integration from a Polars DataFrame
  
- `from_file(file_path: &str) -> Result<Self, IronCalcError>`
  - Loads integration from an Excel file
  
- `get_schema() -> &[ColumnSchema]`
  - Returns the schema of the current data
  
- `get_dataframe() -> &DataFrame`
  - Returns the underlying DataFrame
  
- `save(file_path: &str) -> Result<(), IronCalcError>`
  - Saves workbook to file
  
- `get_sheet_cells(...) -> Result<Vec<SpreadsheetCell>, IronCalcError>`
  - Retrieves cells from a sheet
  
- `get_sheet_columns(...) -> Result<Vec<SpreadsheetColumn>, IronCalcError>`
  - Retrieves columns from a sheet

### Transformation Types

```rust
pub enum Transformation {
    SelectColumns(Vec<String>),
    FilterRows(String, String),
    GroupBy(Vec<String>, Vec<Aggregation>),
    SortBy(Vec<String>, bool),
    RenameColumn(String, String),
    DropColumns(Vec<String>),
    CustomSql(String),
    AddColumn(String, String),
    RemoveDuplicates(bool),
}
```

### Aggregation Operations

```rust
pub enum AggregationOperation {
    Sum,
    Mean,
    Count,
    Min,
    Max,
    First,
    Last,
    StdDev,
    Variance,
}
```

## 📚 Usage Examples

### Example 1: Data Loading and Transformation

```rust
use nustage::{
    ironcalc::{IronCalcIntegration, Transformation},
    transformations::{TransformationFactory, TransformationPipeline},
};
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data from CSV
    let df = DataFrame::from_csv("sales_data.csv")?;
    
    // Create IronCalc integration
    let ironcalc = IronCalcIntegration::from_dataframe(df)?;
    
    // Create transformation pipeline
    let mut pipeline = TransformationPipeline::new("Sales Analysis".to_string());
    
    // Add transformations
    pipeline.add_step(TransformationFactory::select_columns(
        "SelectColumns".to_string(),
        vec!["Product".to_string(), "Region".to_string(), "Sales".to_string()],
    ))?;
    
    pipeline.add_step(TransformationFactory::filter_rows(
        "Filter".to_string(),
        "Sales".to_string(),
        "> 1000".to_string(),
    ))?;
    
    // Apply transformations
    let transformed_df = pipeline.apply(&df)?;
    
    // Save to Excel
    ironcalc.save("sales_analysis.xlsx")?;
    
    Ok(())
}
```

### Example 2: Schema-Autocomplete

```rust
// Get available fields for formula completion
let fields = get_field_names(&sheet);
// Returns: ["Name", "Age", "Salary", "Department"]

// Get resolved values
let value = get_resolved_value(&sheet, row, col)?;
```

### Example 3: Transformation Pipeline

```rust
// Create a transformation pipeline
let mut pipeline = TransformationPipeline::new("Data Processing");

// Add transformations
pipeline.add_step(TransformationFactory::filter_rows(
    "Filter".to_string(),
    "Amount".to_string(),
    "> 1000".to_string(),
))?;

pipeline.add_step(TransformationFactory::group_by(
    "Group".to_string(),
    vec!["Department".to_string()],
    vec![Aggregation {
        column: "Salary".to_string(),
        operation: AggregationOperation::Sum,
    }],
))?;

// Apply transformations
let transformed_df = pipeline.apply(&original_df)?;
```

## 🎨 TUI Features

Nustage includes a terminal user interface with:

- **Grid-based spreadsheet display**: Navigate and edit cells
- **Formula assistance**: Autocomplete and validation
- **Keyboard navigation**: Full mouse-free operation
- **Transformation viewer**: See transformation steps
- **Status bar**: Show current operation and progress

## 🚀 Future Enhancements

1. **Expression Language Integration**
   - Integrate Nushell expression language
   - Support complex formula parsing
   - Rich formula editor

2. **Advanced Transforms**
   - Joins between datasets
   - Pivots and unpivots
   - Custom SQL queries
   - Conditional logic

3. **Richer Autocomplete**
   - IronCalc context-aware suggestions
   - Formula template library
   - Variable suggestions

4. **Performance Optimizations**
   - Lazy evaluation for large datasets
   - Incremental updates
   - Memory-efficient operations

5. **Export Capabilities**
   - Multiple file formats
   - Custom formatting
   - Charts and visualizations

## 📁 Project Structure

```
nustage/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── main.rs             # CLI entry point
│   ├── data/               # Data loading module
│   ├── ironcalc/           # IronCalc integration module
│   ├── transformations/    # Power Query-style transformations
│   ├── cli/                # CLI argument parsing
│   ├── tui.rs              # TUI implementation
│   └── tui_grid.rs         # Grid display utilities
├── examples/               # Usage examples
├── docs/                   # Documentation
├── test_data/              # Test data files
├── Cargo.toml              # Project configuration
└── README.md               # This file
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Code**: Submit pull requests with well-tested code
2. **Documentation**: Improve or add documentation
3. **Examples**: Create useful examples
4. **Testing**: Add tests for new features
5. **Issues**: Report bugs and suggest features

### Contribution Guidelines

- Follow the existing code style
- Add tests for new features
- Update documentation
- Consider backward compatibility
- Provide usage examples

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- **IronCalc** - The spreadsheet engine
- **Polars** - The DataFrame library
- **DuckDB** - The database engine
- **Ratatui** - The TUI framework
- **Nushell** - The expression language inspiration

## 📧 Support

For issues, questions, or contributions:

- Check the [examples directory](examples/) for usage patterns
- Review the [API documentation](docs/)
- Open an issue on the project repository
- Submit a pull request with tests

---

**Version**: 0.1.0  
**Status**: In Development  
**Last Updated**: 2024