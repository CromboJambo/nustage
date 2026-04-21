# nustage-state.md

> Generated: Tue Apr 21 2026
> Source: Cargo.toml, README.md, AGENTS.md, CHANGELOG.md, src/lib.rs, src/data.rs, src/transformations.rs, src/main.rs, examples/simple_demo.rs, examples/power_query_workflow.rs
> Purpose: Human-level review for stateful memory approximation → SQLite indexing

---

## 1. Overview

Nustage is a terminal-native Rust toolkit for tabular workflows. It loads CSV and Parquet data via Polars, extracts schema as serializable Rust structs, defines named transformation pipelines and steps, and outputs CSV reports. Version 0.1.3. The codebase has been cut back to a smaller core — TUI modules, sidecar persistence, DuckDB execution, Excel/IronCalc integration, and Nushell embedding are not present in the current tree.

---

## 2. Architecture

### 2.1 Workspace Layout

```
Cargo.toml              Crate manifest (edition 2024, binary + library)
src/
  main.rs               nustage binary — CLI entry point (clap)
  lib.rs                Library root — re-exports data, transformations, core
  data.rs               Data loading, schema inference (Polars)
  transformations.rs    Transformation pipeline (steps, aggregation, serialization)
  test_data_loader.rs   Standalone test data helpers (not wired into lib)
examples/
  simple_demo.rs        Sample data creation and schema extraction
  power_query_workflow.rs Pipeline construction and CSV report output
test_data/              Sample CSV inputs (not present in current tree)
docs/                   Integration guides and release notes (not present in current tree)
bin/                    Shell scripts for domain workflows (FINMC ops analysis)
ServiceOrderMargin/     Standalone GL tie-out Rust script
.github/workflows/ci.yml CI pipeline
```

### 2.2 Core Components

| Component | Role | Status |
|---|---|---|
| `src/main.rs` | CLI entry point with clap Parser | Implemented |
| `src/lib.rs` | Library root, re-exports + core structs | Implemented |
| `src/data.rs` | DataFrame schema extraction via Polars | Implemented |
| `src/transformations.rs` | Pipeline and step types, TransformationFactory | Implemented |
| `examples/simple_demo.rs` | Demo data creation, schema inspection | Implemented |
| `examples/power_query_workflow.rs` | Power Query-style workflow (input → transforms → summary → report) | Implemented |
| `bin/find_finmc_ops.sh` | Domain workflow shell script | Implemented |
| `ServiceOrderMargin/gl_tie_out.rs` | Standalone GL tie-out script | Implemented |

### 2.3 Core Pipeline or Primary Function

Input file (CSV or Parquet) → schema extraction via `data::get_schema` → named transformation pipeline construction via `TransformationPipeline` → step application via `TransformationFactory` (select columns, filter rows, group by) → CSV report output via `CsvWriter`.

### 2.4 CLI Command Surface

| Command | Purpose |
|---|---|
| `cargo run -- <FILE>` | CLI with input CSV/Parquet file |
| `cargo run` | Demo mode fallback (no input file) |
| `cargo run --example simple_demo` | Sample data creation + schema extraction |
| `cargo run --example power_query_workflow` | Pipeline construction + CSV report output |
| `cargo build` | Debug build |
| `cargo build --release` | Optimized release build |

### 2.5 Dependencies

| Dependency | Version | Purpose |
|---|---|---|
| polars | 0.53 | Data loading, schema inference, DataFrame operations |
| clap | 4.5 | CLI argument parsing |
| thiserror | 2.0 | Library error types |
| anyhow | 1.0 | Application-level errors |
| serde | 1.0 | Serialization of schema and pipeline structs |
| serde_json | 1.0 | JSON serialization |
| chrono | 0.4 | Temporal utilities |
| uuid | 1.10 | UUID generation |

---

## 3. Build & Test

| Command | Purpose |
|---|---|
| `cargo build` | Compile debug build |
| `cargo build --release` | Optimized release build |
| `cargo test --all-features` | Run all unit, integration, and doc tests |
| `cargo clippy -- -D warnings` | Lint — must pass with zero warnings |
| `cargo fmt -- --check` | Verify formatting (CI enforced) |
| `cargo fmt` | Auto-format all source files |
| `cargo audit` | Security audit via rustsec/audit-action |

---

## 4. Code Quality & Style

- Formatter: `rustfmt` (default settings). Run `cargo fmt` before every commit.
- Linter: `clippy` with `-D warnings`. All warnings treated as errors in CI.
- Naming: `snake_case` for functions/variables, `CamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for constants.
- Error handling: `thiserror` for library error types, `anyhow` for application-level errors. Return `Result` instead of panicking.
- Documentation: `///` doc comments on all public functions, structs, and modules. Include `# Arguments`, `# Returns`, `# Errors` sections where appropriate.
- Testing: `#[test]` with `cargo test`. `test_<function_or_behavior>` inside `#[cfg(test)] mod tests` per module.
- Commit messages: imperative, descriptive summaries. Conventional-commit prefixes (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`) encouraged.
- PR gate: `cargo fmt -- --check && cargo clippy -- -D warnings && cargo test --all-features` before merge.

---

## 5. Product Boundary

Nustage owns workflow-level features: named transformation steps, schema-aware operations, sidecar state, provenance, pipeline serialization, execution planning, SQL / M-code generation, and domain workflows that operate on fields, tables, and reproducible intent.

Nustage does **not** own generic spreadsheet-viewer mechanics. If a feature is primarily about raw grid rendering, cursor movement and scrolling, cell/range addressing, in-place sheet browsing/editing, plugin-specific UX, or terminal spreadsheet interaction without pipeline semantics — it belongs in `zellij-sheets`, not here.

---

## 6. Crabjar Context

### 6.1 Architecture Alignment

| Component | Crabjar Role |
|---|---|
| `src/main.rs` | Pure observer — CLI entry point, no stateful modification |
| `src/lib.rs` | Pure observer — library root, structural reference |
| `src/data.rs` | Pure observer — schema extraction, read-only data inspection |
| `src/transformations.rs` | Append-only — pipeline definitions, step types, serializable structs |
| `examples/simple_demo.rs` | Pure observer — demo example, no production use |
| `examples/power_query_workflow.rs` | Append-only — workflow example demonstrating pipeline semantics |
| `bin/find_finmc_ops.sh` | Gated — domain workflow shell script, external to Rust crate |
| `ServiceOrderMargin/gl_tie_out.rs` | Gated — standalone GL tie-out script, separate crate |
| `Cargo.toml` | Pure observer — crate manifest, dependency surface |
| `CHANGELOG.md` | Append-only — version history, structural evolution |

### 6.2 State Docs Surface

Crabjar's state-docs commands: `create-state-doc`, `update-state-doc`. Overlay system tracks component status, dependency drift, and boundary alignment.

### 6.3 Knowledge Bridge

Nustage's schema serialization (`ColumnSchema`, `DataFrameSchema`) and pipeline serialization (`TransformationPipeline`, `TransformationStep`) are structurally compatible with Crabjar's state-doc indexing. Serde-derived structs provide a bridge between Rust types and SQLite schema representation.

### 6.4 Project Config

No `.crabjar_config.toml` present. Default Crabjar alignment applies: all Rust source modules treated as Pure observer, examples as Append-only or Gated, external scripts as Gated.

### 6.5 Integration Points

- Pipeline serialization via serde could adopt Crabjar's content-addressed sidecar pattern for pipeline identity.
- TransformationFactory step creation could adopt Crabjar's gated execution model for step validation.
- Schema extraction (`data::get_schema`) could adopt Crabjar's state-doc indexing for schema snapshots.
- Power Query workflow pattern (input → transforms → summary → report) could adopt Crabjar's reproducible intent tracking.
- Domain workflow scripts (`bin/`, `ServiceOrderMargin/`) could adopt Crabjar's external script integration.

---

## 7. Confidence Assessment

### 7.1 What This Review Captures

- Crate manifest and dependency surface from Cargo.toml
- CLI command surface from main.rs and README.md
- Core module structure from src/lib.rs, src/data.rs, src/transformations.rs
- Pipeline semantics from examples/power_query_workflow.rs
- Version history from CHANGELOG.md
- Product boundary from AGENTS.md and README.md
- Build/test commands from AGENTS.md
- Code quality rules from AGENTS.md
- External scripts from bin/ and ServiceOrderMargin/

### 7.2 What This Review Might Have Missed

- test_data_loader.rs content (read but not analyzed in detail)
- docs/integration/README.md content (docs directory empty in current tree)
- test_data/ content (test_data directory empty in current tree)
- .zed/ configuration
- web/ directory content
- nushell-sandbox-spe.md content
- PSV.md content
- QUICKSTART.md content
- CONTRIBUTING.md content
- IMPLEMENTATION_SUMMARY.md content
- LICENSE and LICENSES/ content
- CI pipeline specifics from .github/workflows/ci.yml
- Cargo.lock dependency resolution

### 7.3 Assumptions

- test_data_loader.rs is standalone test helpers not wired into lib (per AGENTS.md)
- docs/ and test_data/ directories are empty in current tree (per README.md)
- TUI, sidecar, DuckDB, Excel, Nushell modules are removed (per CHANGELOG.md and README.md)
- ServiceOrderMargin is a separate standalone Rust script
- bin/ contains domain workflow shell scripts
- Crabjar alignment defaults apply since no .crabjar_config.toml exists

### 7.4 Blind Spots

- No access to docs/integration/README.md (docs directory empty)
- No access to test_data/ files (test_data directory empty)
- No verification of CI pipeline specifics (.github/workflows/ci.yml)
- No verification of Cargo.lock vs Cargo.toml dependency alignment
- No verification of actual build/test pass status
- No verification of clippy/lint pass status
- No access to IMPLEMENTATION_SUMMARY.md for removed vs remaining clarification

### 7.5 Stale After

- New version release (0.1.4 or higher)
- Addition of new source modules to src/
- Addition of new examples to examples/
- Restoration of removed modules (TUI, sidecar, DuckDB, Excel, Nushell)
- Dependency changes in Cargo.toml
- Product boundary changes in AGENTS.md
- Pipeline semantics changes in transformations.rs
- CLI command surface changes in main.rs
- External script additions to bin/ or ServiceOrderMargin/

---

## 9. Key Takeaways

1. Nustage is a terminal-native Rust toolkit for tabular workflows at version 0.1.3.
2. Core scope is data loading via Polars, schema extraction, transformation pipeline definitions, and CSV report output.
3. TUI, sidecar, DuckDB, Excel/IronCalc, and Nushell modules are removed from the current tree.
4. Product boundary separates workflow-level features from spreadsheet-viewer mechanics — the latter belongs in zellij-sheets.
5. Pipeline serialization via serde provides a bridge to Crabjar's state-doc indexing.
6. TransformationFactory step creation and schema extraction patterns are adoption candidates for Crabjar.
7. Code quality rules enforce rustfmt, clippy -D warnings, and cargo test --all-features as CI gates.
8. External scripts in bin/ and ServiceOrderMargin/ are gated Crabjar components.
9. docs/ and test_data/ directories are empty in the current tree.
10. Version 0.1.3 marks the transition from aspirational claims to transparent documentation of implemented vs planned features.

---

*End of review.*
