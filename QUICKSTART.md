# Nustage Quick Start

This quick start reflects the current repository scope.

## Build

```bash
cargo build
```

For an optimized binary:

```bash
cargo build --release
```

## Run the CLI

Run against a file:

```bash
cargo run -- test_data/sales.csv
```

Run without an input file to use the current demo path:

```bash
cargo run
```

## Run Examples

```bash
cargo run --example simple_demo
cargo run --example power_query_workflow
```

## Run Tests

```bash
cargo test --all-features
```

## Current Scope

Implemented in this repo today:

- CSV and Parquet-oriented data handling through Polars types
- Schema extraction with `nustage::data::get_schema`
- Lightweight transformation pipeline structs
- Minimal CLI and runnable examples

Removed or not currently present:

- `--tui` mode
- TUI/grid rendering
- Sidecar files
- Excel/IronCalc integration
- DuckDB-backed execution

## Read Next

- [`README.md`](/home/crombo/nustage/README.md)
- [`CONTRIBUTING.md`](/home/crombo/nustage/CONTRIBUTING.md)
