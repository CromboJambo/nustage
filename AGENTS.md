# Repository Guidelines

Nustage is a terminal-native Rust toolkit for tabular workflows: CSV/Parquet loading, schema inference, named transformation pipelines, and Power Query M code generation.

## Product Boundary

Nustage owns workflow-level features: named steps, schema-aware operations, provenance, pipeline serialization, execution planning, SQL / M-code generation.

Nustage does **not** own grid rendering, cursor navigation, cell addressing, or interactive spreadsheet browsing. Those belong in `zellij-sheets`.

When in doubt: keep Nustage focused on step pipelines, schema visibility, and reproducible transformations.

## Project Structure

```
Cargo.toml              # edition 2024, binary + library
src/
  main.rs               CLI entry point (clap)
  lib.rs                Library root — re-exports
  data.rs               Data loading, schema inference (Polars)
  transformations.rs    Pipeline steps, aggregation, serialization
  test_data_loader.rs   Standalone test helpers (not wired into lib)
examples/
  simple_demo.rs
  power_query_workflow.rs
test_data/              Sample CSV inputs
docs/                   Integration guides, release notes
bin/                    Shell scripts for domain workflows
ServiceOrderMargin/     Standalone GL tie-out Rust script
```

## Build, Test & Development Commands

| Command | Purpose |
|---|---|
| `cargo build` | Compile debug build |
| `cargo build --release` | Optimized release build |
| `cargo test --all-features` | Run all unit, integration, and doc tests |
| `cargo clippy -- -D warnings` | Lint — must pass with zero warnings |
| `cargo fmt -- --check` | Verify formatting (CI enforced) |
| `cargo fmt` | Auto-format all source files |
| `cargo run -- <FILE>` | Run CLI with a data file |
| `cargo run` | Run CLI in demo mode (no input file) |
| `cargo run --example simple_demo` | Run an example |

## Coding Style

- `rustfmt` (default). Run `cargo fmt` before every commit.
- `clippy` with `-D warnings`. All warnings are errors in CI.
- `snake_case` for functions/variables, `CamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for constants.
- `thiserror` for library error types, `anyhow` for application-level errors. Return `Result` instead of panicking.
- `///` doc comments on all public functions, structs, and modules.

## Testing

- Built-in `#[test]` with `cargo test`.
- `test_<function_or_behavior>` inside a `#[cfg(test)] mod tests` block per module.
- Test with varied inputs (empty, malformed, large).
- `cargo test --all-features` must pass before merge.

## Commit & PR Guidelines

Commit messages use imperative, descriptive summaries. Conventional prefixes (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`) encouraged.

PR checklist:
1. `cargo fmt -- --check && cargo clippy -- -D warnings && cargo test --all-features`
2. Clear title and description.
3. Link related issues.
4. One logical change per PR.
5. Target `main`.

## Security

CI runs `cargo audit` via `rustsec/audit-action` on every push and PR. Address advisories before merging.
