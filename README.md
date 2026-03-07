# Nustage
> **Piping commands over ranges, not formulas in cells.**

---

## What This Is

A terminal-native pipeline orchestration layer for tabular data. Not a spreadsheet. Not a grid editor. The thing that sits between your data and your understanding of it — making every transformation explicit, reversible, and reproducible.

Built because copy-paste-values-only from a CSV export is rational behavior given current alternatives. Nustage makes the step-based model accessible enough that an accounting colleague would actually use it.

---

## What This Is Not

- Not a grid viewer — use Tabiew for that
- Not a formula editor — pipes over ranges, not cells
- Not Excel with a terminal skin
- Not a Nushell wrapper (yet)
- Not a collaboration tool (yet)

---

## The Core Insight

Power Query is honest engineering that almost nobody uses because the abstraction gap is too wide. Excel versioning is recoverable but not readable — snapshots, not intent. The formula in F7 that references a sheet from 2019 that someone renamed is the failure mode of cell-based computation.

The step list is the product. Everything else serves it.

---

## Architecture

```
Data Source (CSV / Parquet / Excel)
  ↓
Schema Inference
  ↓
Step Pipeline (immutable, ordered, named)
  ↓
DuckDB Execution Engine
  ↓
Tabular Result
  ↓
Witness Layer (TUI — pipeline state, not pixel-perfect grid)
  ↓
Sidecar (.nustage.json — versionable, diffable, readable)
```

### Layers and Responsibilities

| Layer | Tool | Responsibility |
|-------|------|----------------|
| Grid viewing | Tabiew | Raw data display, navigation |
| Pipeline orchestration | Nustage | Steps, schema, transforms |
| Execution | DuckDB | SQL generation, query engine |
| Scripting (future) | Nushell | Complex expressions, macros |
| Version control | Git + sidecar | Diff pipeline definitions |

---

## The Step Model

Each transformation is:
- **Named** — readable by a human
- **Immutable** — applying a step never mutates the source
- **Ordered** — steps form a reproducible chain
- **Reversible** — delete or reorder without starting over
- **Transparent** — generates visible SQL for learning and debugging

```
1. Source: sales.csv
2. Filter: Revenue > 1000
3. Add Column: Margin = Revenue - Cost
4. Group By: Region | Sum(Margin)
5. Sort: Margin descending
```

This is the whole product. The TUI makes it visible. DuckDB makes it fast. The sidecar makes it versionable.

---

## Expression Syntax

Pipes over ranges. Field references, not cell addresses.

```nu
@Revenue - @Cost
@Date >= 2024-01-01
@Region in ["North", "South"]
@data | group-by Region | sum Revenue
```

No `=SUM(F7:F23)`. No cell coordinates. No silent breakage when someone inserts a column.

---

## The Sidecar

Lives alongside the data file. Plain JSON. Diffs cleanly in git.

```json
{
  "version": 1,
  "source": "sales.csv",
  "pipeline": [
    { "step": "filter", "column": "Revenue", "condition": "> 1000" },
    { "step": "add_column", "name": "Margin", "expr": "@Revenue - @Cost" },
    { "step": "group_by", "columns": ["Region"], "agg": [{"col": "Margin", "op": "sum"}] }
  ],
  "schema": {
    "Revenue": "f64",
    "Cost": "f64",
    "Region": "str",
    "Date": "date"
  }
}
```

The sidecar is the artifact. The CSV is just the data.

---

## Domain Advantages

Built by a cost accountant in manufacturing. The problems Nustage solves are real and daily:

- **BOM structures** — hierarchical, box in box in box, maps cleanly to recursive pipeline steps
- **Standard vs actual variance** — two-range comparison as a named pipeline, not a copy-paste ritual  
- **Period-end allocations** — reproducible, auditable, not locked in one person's head
- **Schema drift** — when columns change, the sidecar tells you exactly what broke and where

The target user is not a data engineer. It is the person who currently exports to CSV, opens in Excel, copies, pastes values only, and formats manually — because that is the rational path given current tools.

---

## The Witness Layer (TUI)

The TUI is not the product. It is the feedback loop that makes the pipeline tangible.

Minimum viable witness:
- **Step list** — left panel, named steps, current active step highlighted
- **Schema panel** — column names and types at current step
- **Preview** — row count, shape, sample output
- **SQL transparency** — generated query visible on demand
- **Status bar** — rows in / rows out, active filters

The grid is Tabiew's job. Nustage shows pipeline state.

---

## Tech Stack

| Concern | Crate | Why |
|---------|-------|-----|
| Data processing | Polars | Columnar, fast, Rust-native |
| Query engine | DuckDB | Embedded SQL, no server |
| Excel I/O | IronCalc + Calamine | Read/write .xlsx natively |
| TUI | Ratatui + Crossterm | Proven, keyboard-driven |
| CLI | Clap | Standard, derive-based |
| Serialization | Serde + serde_json | Sidecar format |
| Error handling | Thiserror + Anyhow | Clean propagation |

---

## What To Build Next (Prioritized)

1. **Real cell rendering in tui_grid.rs** — actual data, not placeholders
2. **Fix Model lifetime in ironcalc/mod.rs** — `'static` is wrong, scope it properly  
3. **Real Excel loader in data/mod.rs** — replace empty DataFrame placeholder
4. **Step list panel in TUI** — the thing that makes Nustage distinct from Tabiew
5. **Sidecar read/write** — serialize/deserialize pipeline to `.nustage.json`
6. **SQL transparency** — show generated DuckDB query per step

Everything else is future.

---

## What To Ignore For Now

- Nushell integration — aspirational, not load-bearing yet
- Collaboration features — not the problem
- Charts and visualization — not the problem
- Custom expression language — DuckDB SQL is sufficient for MVP

---



---

## Web Interface & PowerQuery Integration

### The Companion Tool Vision

Nustage doesn't replace Excel — it **augments** it by providing a structured way to build transformations that can be exported as M code for PowerQuery.

```
┌─────────────────────────────────────────────────────────────┐
│  Nustage Web (WASM)                                         │
│  ─────────────────────────────────────────────────────────   │
│  - Import any file from browser                             │
│  - Build pipeline visually                                  │
│  - Preview changes in sidecar                               │
│  - Export M code to PowerQuery                              │
└─────────────────────────────────────────────────────────────┘
                    ↓ syncs with
┌─────────────────────────────────────────────────────────────┐
│  Original File (untouched) + Sidecar (.nustage.json)        │
│  ─────────────────────────────────────────────────────────   │
│  - Human-readable diff in plain text                        │
│  - Versionable, git-friendly                                │
└─────────────────────────────────────────────────────────────┘
                    ↓ generates
┌─────────────────────────────────────────────────────────────┐
│  PowerQuery M Code                                          │
│  ─────────────────────────────────────────────────────────   │
│  - Copy-paste into Excel's Advanced Editor                  │
│  - Or apply directly via this tool                          │
└─────────────────────────────────────────────────────────────┘
```

### Key Features

1. **Import any file** — CSV, Excel (.xlsx/.xls), Parquet from browser
2. **Build transformations visually** — Filter, Group, Sort, Add Calculated Columns
3. **Preview changes in sidecar** — Human-readable diff showing what changed
4. **Export to PowerQuery M** — Copy-paste ready code for Excel's Advanced Editor
5. **Keep original file unchanged** — Sidecar lives alongside your data

### Example Workflow

1. Open `sales_data.xlsx` in Nustage Web
2. Add step: Filter rows where `Revenue > 1000`
3. Add step: Group by `Region`, sum `Revenue`
4. Add step: Sort by `Sum(Revenue)` descending
5. Click "Export to PowerQuery M" → Copy code
6. Paste into Excel's Advanced Editor → Done!

### M Code Output Example

```powerquery-m
let
    Source = Excel.CurrentWorkbook(){[Name="Table1"]}[Content],
    FilteredRows = Table.SelectRows(Source, each [Revenue] > 1000),
    GroupedRows = Table.Group(FilteredRows, {"Region"}, {{"Sum(Revenue)", each List.Sum([Revenue]), type nullable number}}),
    SortedRows = Table.Sort(GroupedRows,{{"Sum(Revenue)", Order.Descending}})
in
    SortedRows
```

### Why This Works Better Than PowerQuery UI

| Feature | PowerQuery UI | Nustage Web |
|---------|--------------|-------------|
| **Transparency** | Hidden behind clicks | Explicit SQL generation |
| **Version control** | No native support | Git-friendly sidecars |
| **Reproducibility** | Manual copy-paste | Replay pipeline on new data |
| **Diff readability** | Hard to compare steps | Plain text changes visible |

---

## Building Nustage Web

### Tech Stack for WASM

```toml
[dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["console"] }
serde-wasm-bindgen = "0.6"
```

### Build Commands

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for web
wasm-pack build --target web --release

# Run local server
wasm-pack serve
```

### File Structure

```
nustage/
├── src/
│   ├── wasm.rs           # WASM bindings (new)
│   └── mcode/            # PowerQuery M generator (new)
│       └── mod.rs
├── sidecar/              # Sidecar file handling (new)
│   └── mod.rs
└── web/                  # Web interface (new)
    ├── index.html
    ├── styles.css
    └── app.js
```

### Next Steps for Web Interface

1. **Set up WASM build pipeline** — Configure wasm-pack in Cargo.toml
2. **Implement M code generator** — Convert Nustage steps to PowerQuery M
3. **Build sidecar file handling** — Load, save, diff .nustage.json files
4. **Create web UI** — Drag-drop file upload, step builder, preview panels
5. **Export functionality** — Copy M code to clipboard, download sidecar

---

## Quick Start Guide

### For Terminal Users (Rust CLI)

```bash
# Build from source
cargo build --release

# Run with a data file
./target/release/nustage test_data/sales.csv

# Or run the TUI version
./target/release/nustage --tui test_data/sales.csv
```

### For Web Users (WASM)

```bash
# Build WASM package
wasm-pack build --target web --release

# Open in browser
wasm-pack serve
```

Visit `http://localhost:8000` to start building your pipeline!
