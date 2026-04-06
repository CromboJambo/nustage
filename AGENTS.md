# Repository Guidelines

Nustage is a terminal-native data processing and analysis toolkit built in Rust. It loads CSV, Excel, and Parquet files, provides a TUI for interactive exploration, and supports transformation pipelines, IronCalc spreadsheet integration, and Power Query M code generation.

## Product Boundary

Nustage owns workflow-level features: named transformation steps, schema-aware operations, sidecar state, provenance, pipeline serialization, execution planning, SQL / M-code generation, and domain workflows that operate on fields, tables, and reproducible intent.

Nustage does **not** own generic spreadsheet-viewer mechanics unless they directly support pipeline state. If a feature is primarily about:

- raw grid rendering
- cursor movement and scrolling
- cell/range addressing
- in-place sheet browsing/editing
- plugin-specific UX
- terminal spreadsheet interaction without pipeline semantics

it likely belongs in `zellij-sheets`, not here.

Use this rule:

- `zellij-sheets` answers: "How do I inspect and navigate tabular data interactively?"
- `nustage` answers: "How do I define, execute, persist, and understand tabular workflows?"

When in doubt, keep Nustage focused on step pipelines, witness-layer visibility, and reproducible transformations.

## Project Structure & Module Organization

```
Cargo.toml              # Crate manifest (edition 2024, two binaries + library)
target/src/             # Source code (see note below)
  main.rs               # `nustage` binary — CLI entry point (clap)
  test_data_loader.rs   # `data_test` binary
  lib.rs                # Library root — re-exports all public modules
  cli/mod.rs            # CLI argument parsing (clap derive)
  data/mod.rs           # Data loading and execution inputs (Polars, Calamine, DuckDB)
  tui.rs / tui_grid.rs  # Witness-layer TUI for pipeline state and result inspection
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

## Feature Ownership Rules

- Put pipeline definitions, step graphs, lineage, sidecar persistence, and execution metadata in `transformations/`, `sidecar/`, or adjacent library modules.
- Put file-format loading, schema inference, and execution-engine glue in `data/`.
- Keep `tui.rs` and `tui_grid.rs` focused on the witness layer: step list, schema, preview, query transparency, status, and execution feedback.
- Avoid rebuilding a general spreadsheet app inside Nustage. If a TUI feature would still make sense with no pipeline, it probably belongs in `zellij-sheets`.
- Prefer field-based and range-of-records abstractions over cell-addressed UX. Nustage is step-oriented, not formula-grid-oriented.
- IronCalc integration belongs here only when it supports import/export, interoperability, or pipeline-adjacent spreadsheet logic. Generic sheet viewing/editing still belongs elsewhere.

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
