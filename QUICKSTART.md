# Nustage Quick Start Guide

## Getting Started in 5 Minutes

### Option A: Terminal Version (Existing)

```bash
# Clone or navigate to your nustage directory
cd /path/to/nustage

# Build the CLI tool
cargo build --release

# Run with a sample data file
./target/release/nustage test_data/sales.csv

# Or run the TUI version
./target/release/nustage --tui test_data/sales.csv
```

### Option B: Web Version (New!)

```bash
# Install wasm-pack if you haven't already
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the WASM package for web
wasm-pack build --target web --release

# Run a local server
wasm-pack serve

# Open http://localhost:8000 in your browser
```

---

## Understanding the Workflow

### The Core Concept

Nustage doesn't modify your original data file. Instead, it creates a **sidecar file** (`.nustage.json`) that lives alongside your data and contains all your transformations:

```
sales.csv                    ← Your original data (never modified)
sales.nustage.json           ← Transformation pipeline (versionable)
```

### Step-by-Step Example

Let's transform sales data to show top-performing regions:

1. **Upload File** → Drag `sales.csv` into the browser or run CLI
2. **Add Filter** → Keep only rows where `Revenue > 1000`
3. **Group By** → Group by `Region`, sum `Revenue`
4. **Sort** → Order by summed revenue descending
5. **Export M Code** → Copy to clipboard for PowerQuery

### What the Sidecar File Looks Like

```json
{
  "version": 1,
  "source": "sales.csv",
  "pipeline": [
    {
      "id": "uuid-here",
      "name": "filter_revenue",
      "op": "filter_rows",
      "params": {"column": "Revenue", "condition": "> 1000"}
    },
    {
      "id": "uuid-here", 
      "name": "group_region",
      "op": "group_by",
      "params": {
        "columns": ["Region"],
        "aggregations": [{"column": "Revenue", "op": "sum"}]
      }
    },
    {
      "id": "uuid-here",
      "name": "sort_revenue",
      "op": "sort_by", 
      "params": {"columns": ["Sum(Revenue)"], "descending": true}
    }
  ]
}
```

---

## Available Transformations

### Filter Rows
Keep only rows matching a condition:
- **Nustage syntax:** `@Revenue > 1000`
- **M code output:** `Table.SelectRows(Source, each [Revenue] > 1000)`

### Select Columns
Choose which columns to keep:
- **Input:** `Name, Revenue, Date`
- **M code output:** `Table.SelectColumns(Source, {"Name", "Revenue", "Date"})`

### Add Calculated Column
Create new column from formula:
- **Nustage syntax:** `Margin = @Revenue - @Cost`
- **M code output:** `Table.AddColumn(Source, "Margin", each [Revenue] - [Cost])`

### Group & Aggregate
Summarize data by category:
- **Input:** Group by `Region`, sum `Revenue`
- **M code output:** `Table.Group(Source, {"Region"}, {{"Total Revenue", each List.Sum([Revenue]), type number}})`

### Sort
Order rows by column:
- **Input:** `@Revenue desc`
- **M code output:** `Table.Sort(Source, {{"Revenue", Order.Descending}})`

### Rename Column
Change column names:
- **Input:** `Total_Revenue → Revenue`
- **M code output:** `Table.RenameColumns(Source, {{"Total_Revenue", "Revenue"}})`

### Drop Columns
Remove unwanted columns:
- **Input:** `TempHelper, InternalID`
- **M code output:** `Table.RemoveColumns(Source, {"TempHelper", "InternalID"})`

---

## Exporting to PowerQuery

After building your pipeline in Nustage Web:

1. Click **"Export to PowerQuery M"**
2. Review the generated code (syntax-highlighted)
3. Click **"Copy to Clipboard"**
4. Open Excel → Data tab → Get Data → From Other Sources → Blank Query
5. Click **"Advanced Editor"** → Paste the code
6. Click **Done** → Your transformed data appears!

### Example M Code Output

```powerquery-m
let
    Source = Excel.CurrentWorkbook(){[Name="Table1"]}[Content],
    FilteredRows = Table.SelectRows(Source, each [Revenue] > 1000),
    GroupedRows = Table.Group(FilteredRows, {"Region"}, {{"Total Revenue", each List.Sum([Revenue]), type nullable number}}),
    SortedRows = Table.Sort(GroupedRows,{{"Total Revenue", Order.Descending}})
in
    SortedRows
```

---

## Viewing Schema Changes

Nustage shows you exactly what changed:

```
=== Schema Changes ===
  + Added column: Total_Revenue
  - Removed column: TempHelper
  ~ Type changed: Date from str to date
```

This makes it easy to understand impact before applying changes.

---

## Common Use Cases

### Financial Reporting
1. Import monthly sales CSV
2. Filter for current quarter dates
3. Calculate profit margin (`= Revenue - Cost`)
4. Group by product category, sum revenue
5. Export M code → Refresh next month with new data

### Data Cleaning
1. Load raw export from legacy system
2. Drop columns marked "deprecated"
3. Rename confusing column names
4. Filter out null/empty rows
5. Save sidecar → Reuse for future exports

### Variance Analysis (BOM Comparison)
1. Import standard BOM and actual costs as separate files
2. Join on part number
3. Calculate variance (`= Actual - Standard`)
4. Filter for variances outside tolerance threshold
5. Export M code → Apply to new monthly data automatically

---

## Troubleshooting

### "Unsupported file format" error
- Nustage supports: `.csv`, `.xlsx`, `.xls`, `.parquet`
- Ensure file extension is lowercase (`.XLSX` may not work)

### WASM build too large (>5MB)
```bash
# Optimize for size
wasm-pack build --target web --release --features minimal
```

### M code doesn't load in Excel
- Check for special characters in column names (use quotes: `"Column Name"`)
- Ensure all referenced columns exist in source data
- Try pasting into Excel's Advanced Editor manually first

---

## Tips & Best Practices

1. **Name your steps meaningfully** → `filter_revenue` is better than `step_3`
2. **Keep original files untouched** → Sidecar is where changes live
3. **Version control sidecars** → Git-friendly diffs, easy collaboration
4. **Test with small datasets first** → Validate pipeline before scaling up
5. **Document complex expressions** → Add comments in sidecar metadata (future feature)

---

## Need Help?

- Check `roadmap/WASM_POWERQUERY_ROADMAP.md` for detailed implementation
- See `IMPLEMENTATION_SUMMARY.md` for what was built this session
- Open an issue on the repository for bugs or feature requests

---

*Happy transforming!* 🚀