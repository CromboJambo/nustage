# Nustage

> **Piping commands over ranges, not formulas in cells.**

---

## What This Is

A terminal-native pipeline orchestration layer for tabular data. It sits between your data and your understanding of it — making every transformation explicit, reversible, and reproducible.

Built because copy-paste-values-only from a CSV export is rational behavior given current alternatives. Nustage makes the step-based model accessible enough that an accounting colleague would actually use it.

---

## What This Is Not

- Not a grid viewer — use Tabiew for that
- Not a formula editor — pipes over ranges, not cells
- Not Excel with a terminal skin
- Not a Nushell wrapper (aspirational)
- Not a collaboration tool (aspirational)

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
Witness Layer (TUI — pipeline state)
  ↓
Sidecar (.nustage.json — aspirational)
```

### Layers and Responsibilities

| Layer | Tool | Status | Responsibility |
|-------|-------|--------|----------------|
| Grid viewing | Tabiew | ✅ Implemented | Raw data display, navigation |
| Pipeline orchestration | Nustage | ✅ Implemented | Steps, schema, transforms |
| Execution | DuckDB | ✅ Implemented | SQL generation, query engine |
| Scripting | Nushell | Aspirational | Complex expressions, macros |
| Version control | Git | ✅ Implemented | Track pipeline changes |

---

## The Step Model

Each transformation is:
- **Named** — readable by a human
- **Immutable** — applying a step never mutates the source
- **Ordered** — steps form a reproducible chain
- **Reversible** — delete or reorder without starting over

```
1. Source: sales.csv
2. Filter: Revenue > 1000
3. Add Column: Margin = Revenue - Cost
4. Group By: Region | Sum(Margin)
5. Sort: Margin descending
```

This is the whole product. DuckDB makes it fast.

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

## Domain Advantages

Built by a cost accountant in manufacturing. The problems Nustage solves are real and daily:

- **BOM structures** — hierarchical, box in box in box, maps cleanly to recursive pipeline steps
- **Standard vs actual variance** — two-range comparison as a named pipeline, not a copy-paste ritual
- **Period-end allocations** — reproducible, auditable, not locked in one person's head
- **Schema drift** — when columns change, the pipeline tells you exactly what broke

The target user is not a data engineer. It is the person who currently exports to CSV, opens in Excel, copies, pastes values only, and formats manually — because that is the rational path given current tools.

---

## The Witness Layer (TUI)

The TUI is the feedback loop that makes the pipeline tangible.

**Current TUI capabilities:**
- Preview row count and shape (working)

**Aspirational TUI features:**
- Step list panel
- Schema panel
- SQL transparency
- Status bar

The grid is Tabiew's job. Nustage shows pipeline state.

---

## Tech Stack

| Concern | Crate | Why | Status |
|---------|-------|-----|--------|
| Data processing | Polars | Columnar, fast, Rust-native | ✅ Implemented |
| Query engine | DuckDB | Embedded SQL, no server | ✅ Implemented |
| Excel I/O | IronCalc + Calamine | Read .xlsx (partial support) | Partial |
| TUI | Ratatui + Crossterm | Proven, keyboard-driven | ✅ Implemented |
| CLI | Clap | Standard, derive-based | ✅ Implemented |
| Serialization | Serde + serde_json | Sidecar format | Aspirational |
| Error handling | Thiserror + Anyhow | Clean propagation | ✅ Implemented |

---

## Current Implementation Status

### Implemented and Working

- **Main CLI binary** — Built with `--tui` flag support for interactive mode
- **Data loading** — CSV and Parquet files work reliably
- **Examples** — `simple_demo` and `ironcalc_integration` examples compile and run
- **Tests** — Unit tests pass cleanly
- **Release build** — Compiles without errors

### Not Yet Implemented (Aspirational)

The following features are documented but not yet built:

1. **Step list panel in TUI** — Add named steps to the witness layer
2. **Sidecar read/write** — Implement pipeline serialization to `.nustage.json`
3. **SQL transparency** — Show generated DuckDB queries per step
4. **Real Excel loader** — Excel support currently requires manual conversion to CSV
5. **Charts and visualization** — Not the problem domain for MVP
6. **Custom expression language** — DuckDB SQL is sufficient for MVP
7. **Richer autocomplete features** — Basic field awareness is available
8. **Performance optimizations for large datasets** — Core functionality tested on sample data
9. **Real cell rendering in TUI** — Grid viewing via Tabiew
10. **Nushell integration** — Scripting integration is aspirational
11. **Collaboration features** — Not the problem domain
12. **Content-addressed sidecars** — Long-term vision, not MVP scope

---

## Quick Start Guide

### For Terminal Users (Rust CLI)

```bash
# Build from source
cargo build --release

# Run with a CSV data file (CSV and Parquet recommended for demo)
./target/release/nustage test_data/sales.csv

# Run with Parquet file
./target/release/nustage test_data/*.parquet

# Run the TUI version (requires terminal)
./target/release/nustage --tui test_data/sales.csv
```

### Demo Examples

```bash
# Run the simple demo example
cargo run --release --example simple_demo

# Run the IronCalc integration example
cargo run --release --example ironcalc_integration
```

### Unit Tests

```bash
# Run all tests
cargo test --release
```

---

## See Also

- [`ROADMAP.md`](roadmap/ROADMAP.md) — Prioritized feature list
- [`COMPREHENSIVE_ROADMAP.md`](roadmap/COMPREHENSIVE_ROADMAP.md) — Detailed feature specifications

---

## License

This project is licensed under the terms in [LICENSE](../LICENSE).

---

**Note:** This is version 0.1.2 — the first public release with honest documentation. Features marked as "Aspirational" are documented but not yet implemented.
