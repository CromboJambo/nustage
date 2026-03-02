# IronCalc Integration Documentation

## Overview

This document describes the integration between **IronCalc** (spreadsheet engine) and **nustage** (Power Query-style data transformation layer). This combination creates a terminal-native spreadsheet solution with powerful data manipulation capabilities.

## Architecture

```
IronCalc (UI & Spreadsheet Engine)
  ↓
Nustage (Power Query Layer)
  ↓
DuckDB (Data Processing)
  ↓
Data Sources (CSV, Excel, Parquet, etc.)
```

## How It Works

### 1. IronCalc as the Spreadsheet Engine

IronCalc handles:
- Spreadsheet UI rendering and navigation
- Cell-based formula evaluation
- Data persistence (.xlsx, .xls formats)
- Cell formatting and styling
- Grid-based interface operations

### 2. Nustage as the Power Query Layer

Nustage provides:
- Step-based data transformation model
- Schema-aware data loading
- Transformation pipeline management
- DuckDB-powered data processing
- Reversible, immutable transformations

### 3. Integration Points

#### Data Loading

```rust
// Nustage loads data into IronCalc
let df = load_data("data.xlsx")?;
model.load_data_frame(df)?;
```

#### Transformation Pipeline

```rust
// Nustage manages transformation steps
add_filter_step("Amount > 1000")?;
add_select_columns(["Product", "Region", "Date"])?;
add_group_by("Region", ["TotalSales", "AvgPrice"])?;
```

#### Schema Awareness

- Autocomplete for field names
- Type information for formula assistance
- Field metadata display
- Schema validation

## Key Benefits

### For Terminal Users

- **Keyboard-driven interface**: Full mouse-free operation
- **Lightweight and fast**: Rust-based performance
- **Scriptable and composable**: Easy automation
- **Terminal-native UX**: Seamless integration with CLI workflows

### For Power Query Users

- **Step-based model**: Clear, reversible transformations
- **Immutable operations**: No side effects
- **Versionable pipelines**: Save and replay transformations
- **Schema awareness**: Smart completion and validation

### For Developers

- **Rust-based**: High performance and safety
- **Embeddable**: Can be integrated into other applications
- **Open architecture**: Easy to extend with custom transformations
- **Type-safe**: Compile-time error checking

## Usage Examples

### Basic Integration

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

### Schema-Autocomplete

```rust
// Get available fields for formula completion
let fields = get_field_names(&sheet);
// Returns: ["Name", "Age", "Salary", "Department"]

// Get resolved values
let value = get_resolved_value(&sheet, row, col)?;
```

### Transformation Pipeline

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

## Data Flow

### Loading Data

1. User provides file path (CSV, Excel, Parquet)
2. Nustage loads data into Polars DataFrame
3. IronCalcIntegration converts DataFrame to Workbook
4. Workbook is displayed in terminal UI

### Transformations

1. User defines transformation steps
2. Nustage validates steps against schema
3. Steps are applied to DataFrame
4. Resulting DataFrame is displayed in IronCalc
5. Workbook can be saved to file

### Saving Data

1. IronCalc handles file format conversion
2. Workbook is serialized to .xlsx or .xls
3. User can open in Excel for further analysis

## API Reference

### IronCalcIntegration

Main integration struct that bridges IronCalc and nustage.

#### Methods

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

## Integration Scenarios

### 1. Data Loading

Load data from files into IronCalc sheets:

```rust
let ironcalc = IronCalcIntegration::from_file("data.xlsx")?;
```

### 2. Transformation Pipeline

Apply Power Query-style transformations:

```rust
let mut pipeline = TransformationPipeline::new("MyPipeline");
pipeline.add_step(TransformationFactory::select_columns(...))?;
```

### 3. Formula Assistance

Use IronCalc's formula engine with schema awareness:

```rust
let fields = get_field_names(&sheet);
// Provides autocomplete for formulas
```

### 4. Export

Save transformed data back to various formats:

```rust
ironcalc.save("output.xlsx")?;
```

### 5. Scripting

Use nustage's step model for automation:

```rust
let pipeline = TransformationPipeline::deserialize(json)?;
let result = pipeline.apply(&df)?;
```

## Implementation Considerations

### Immediate Steps

1. ✅ Add IronCalc as a dependency in nustage
2. ✅ Create integration functions to load data from IronCalc models
3. ✅ Implement transformation pipeline integration
4. ✅ Add schema awareness for IronCalc cells

### Future Enhancements

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

## Error Handling

All integration functions return `Result` types with descriptive errors:

```rust
pub enum IronCalcError {
    IronCalcError(String),
    DataConversionError(String),
    SheetError(#[from] SheetError),
    InvalidCellReference(String),
    FormulaError(String),
}
```

## Best Practices

1. **Always validate input**: Check file paths and data schemas
2. **Use transformation pipelines**: For complex data processing
3. **Leverage schema awareness**: For autocomplete and validation
4. **Handle errors gracefully**: Use `?` operator for clean error propagation
5. **Save intermediate results**: For debugging and recovery

## Contributing

When contributing to the IronCalc integration:

1. Follow the existing code style
2. Add tests for new features
3. Update this documentation
4. Consider backward compatibility
5. Provide usage examples

## License

This integration is part of the nustage project and follows its licensing terms.

## Support

For issues, questions, or contributions:
- Check the examples directory for usage patterns
- Review the API documentation
- Open an issue on the project repository
- Submit a pull request with tests

---

**Last Updated**: 2024
**Version**: 0.1.0
**Status**: In Development