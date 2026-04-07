# Nustage

Nustage is a terminal-native Rust toolkit for tabular workflows.

Current scope in this repository:

- Load CSV and Parquet data with Polars
- Inspect schema as serializable Rust structs
- Define named transformation pipelines and steps
- Run small examples that demonstrate the current API surface

Nustage does not own grid viewing, cursor movement, cell addressing, or spreadsheet-style interaction. Those concerns belong outside this repo.

## Current Status

This codebase has been cut back to a smaller core.

What is present today:

- `src/data.rs` for schema extraction from Polars `DataFrame`s
- `src/transformations.rs` for lightweight pipeline and step definitions
- `src/main.rs` for a minimal CLI entry point
- `examples/simple_demo.rs` and `examples/power_query_workflow.rs`

What is not present in the current tree:

- TUI modules
- Sidecar persistence
- DuckDB execution
- Excel/IronCalc integration
- Nushell embedding

The docs should be read with that boundary in mind.

## Project Layout

```text
src/
  main.rs             CLI entry point
  lib.rs              Library exports
  data.rs             DataFrame schema extraction
  transformations.rs  Pipeline and transformation step types
examples/
  simple_demo.rs
  power_query_workflow.rs
test_data/
  Sample CSV inputs
```

## Build

```bash
cargo build
cargo build --release
```

## Run

The CLI is currently a thin entry point:

```bash
cargo run -- test_data/sales.csv
```

If no input file is provided, it falls back to a demo mode:

```bash
cargo run
```

## Examples

```bash
cargo run --example simple_demo
cargo run --example power_query_workflow
```

`simple_demo` shows sample data creation and schema extraction.  
`power_query_workflow` shows pipeline construction and CSV report output.

## Tests and Checks

```bash
cargo test --all-features
cargo clippy -- -D warnings
cargo fmt -- --check
```

## Product Boundary

Nustage is step-oriented. It is for reproducible tabular workflows, named transformations, and schema-aware operations.

It is not a spreadsheet UI. If a feature is mainly about:

- raw grid rendering
- cursor navigation
- cell or range addressing
- interactive sheet browsing

it should not be added here.

## Related Docs

- [`QUICKSTART.md`](/home/crombo/nustage/QUICKSTART.md)
- [`CONTRIBUTING.md`](/home/crombo/nustage/CONTRIBUTING.md)
- [`docs/integration/README.md`](/home/crombo/nustage/docs/integration/README.md)
