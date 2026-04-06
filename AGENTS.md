# Repository Guidelines

Nustage is a terminal-native data processing and analysis toolkit built in Rust. It loads CSV, Excel, and Parquet files, provides a TUI for interactive exploration, and supports transformation pipelines, IronCalc spreadsheet integration, and Power Query M code generation.

## Project Structure & Module Organization

```
Cargo.toml              # Crate manifest (edition 2024, two binaries + library)
target/src/             # Source code (see note below)
  main.rs               # `nustage` binary — CLI entry point (clap)
  test_data_loader.rs   # `data_test` binary
  lib.rs                # Library root — re-exports all public modules
  cli/mod.rs            # CLI argument parsing (clap derive)
  data/mod.rs           # Data loading (Polars, Calamine, DuckDB)
  tui.rs / tui_grid.rs  # Terminal UI (ratatui + crossterm)
  ironcalc/mod.rs       # IronCalc spreadsheet engine integration
  transformations/mod.rs# Transformation pipeline (steps, aggregation, serialization)
  mcode/mod.rs          # Power Query M code generation
  sidecar/mod.rs        # .nustage sidecar file format
examples/               # Runnable examples (simple_demo, ironcalc, power_query)
test_data/              # Sample CSV/Excel/Parquet files and .nustage pipelines
docs/                   # Integration guides and release notes
.github/workflows/ci.yml  # CI pipeline
```

> **Note:** Source files currently reside under `target/src/` as referenced by `Cargo.toml`. The canonical paths in `Cargo.toml` are `src/main.rs` and `src/lib.rs`; ensure any new modules follow the same layout.

## Build, Test & Development Commands

| Command | Purpose |
|---|---|
| `cargo build` | Compile debug build |
| `cargo build --release` | Optimized release build |
| `cargo test --all-features` | Run all unit, integration, and doc tests |
| `cargo clippy -- -D warnings` | Lint — must pass with zero warnings |
| `cargo fmt -- --check` | Verify formatting (CI enforced) |
| `cargo fmt` | Auto-format all source files |
| `cargo run -- <FILE>` | Run the CLI with a data file |
| `cargo run -- --tui <FILE>` | Launch interactive TUI mode |
| `cargo run --example simple_demo` | Run an example |

## Coding Style & Naming Conventions

- **Formatter:** `rustfmt` (default settings). Run `cargo fmt` before every commit.
- **Linter:** `clippy` with `-D warnings`. All warnings are treated as errors in CI.
- **Naming:** Follow standard Rust conventions — `snake_case` for functions/variables, `CamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for constants.
- **Error handling:** Use `thiserror` for library error types, `anyhow` for application-level errors. Return `Result` instead of panicking.
- **Documentation:** Add `///` doc comments to all public functions, structs, and modules. Include `# Arguments`, `# Returns`, and `# Errors` sections where appropriate.

## Testing Guidelines

- **Framework:** Built-in `#[test]` with `cargo test`.
- **Naming:** `test_<function_or_behavior>` inside a `#[cfg(test)] mod tests` block per module.
- **Coverage:** Add unit tests for new functions and integration tests for new features. Test with varied data inputs (empty, malformed, large).
- **CI gate:** `cargo test --all-features` must pass before merge.

## Commit & Pull Request Guidelines

**Commit messages** — use imperative, descriptive summaries (e.g., `Add column filtering to transformation pipeline`, `Refactor pipeline steps to use variables`). Conventional-commit prefixes (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`) are encouraged.

**Pull requests:**

1. Run the full check suite locally: `cargo fmt -- --check && cargo clippy -- -D warnings && cargo test --all-features`
2. Provide a clear title and description of changes.
3. Link related issues.
4. Keep PRs focused — one logical change per PR.
5. Target the `main` or `develop` branch.

## Security

CI runs `cargo audit` via the `rustsec/audit-action` on every push and PR. Address any reported advisories before merging.
