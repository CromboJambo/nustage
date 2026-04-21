# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2026-04-21

### Added
- **Sidecar persistence module** (`src/sidecar.rs`) — pipeline provenance storage with step history, schema snapshots, and UUID-based identity
- **Excel CSV export module** (`src/export.rs`) — workbook-to-CSV conversion with multi-sheet and single-sheet export
- `csv` crate dependency for CSV writing
- `calamine` dependency updated from `0.33` to `0.34.0`

### Removed
- QUICKSTART.md
- PSV.md
- IMPLEMENTATION_SUMMARY.md

### Changed
- Updated `calamine` dependency version
- Ordered core modules in `lib.rs`
- Updated `power_query_workflow.rs` example
- Updated repository guidelines to reflect current project structure
- Bumped the crate and CLI version from `0.1.3` to `0.1.4`

## [0.1.3] - 2026-04-07

### Changed
- Updated documentation to reflect honest current implementation status
- Removed "Demo Status: READY" claims that overstated implemented features
- Clearly marked aspirational features that are documented but not yet built
- Separated implemented capabilities from future work in all docs

### Documentation Updates
- **README.md** — Cleaned up duplicate sections; added status indicators for each feature
- **QUICKSTART.md** — Removed code examples referencing unbuilt features
- **IMPLEMENTATION_SUMMARY.md** — Clarified what was removed vs what remains
- **CHANGELOG.md** — Updated from 0.1.1 to 0.1.2; removed unreleased 0.1.0 entry

## [0.1.1] - 2026-03-03

### Added
- Initial public release of Nustage
- Terminal-native pipeline orchestration layer for tabular data
- DuckDB-powered data processing engine
- Schema-aware data loading and inference
- Reversible, immutable transformation steps
- TUI (Terminal User Interface) with grid display capabilities
- CLI argument parsing with Clap
- Comprehensive error handling with Thiserror and Anyhow
- Data serialization/deserialization with Serde
- Support for CSV and Parquet file formats
- IronCalc integration for spreadsheet operations
- Transformation factory with common operations
- Transformation pipeline management system
- Sample DataFrame creation for examples
- IronCalc integration example
- Simple demo example

### Changed
- Updated README to focus on technical capabilities and use cases
- Removed marketing language and focused on product features
- Improved documentation structure and clarity

### Dependencies
- ratatui = "0.29"
- crossterm = "0.27"
- duckdb = { version = "1.1", features = ["bundled"] }
- calamine = "0.33"
- polars = { version = "0.53", features = ["parquet", "csv", "lazy", "temporal", "round_series"] }
- ironcalc = "0.7.1"
- clap = { version = "4.5", features = ["derive"] }
- thiserror = "2.0"
- anyhow = "1.0"
- serde = { version = "1.0", features = ["derive"] }
- serde_json = "1.0"
- chrono = { version = "0.4", features = ["serde"] }
- uuid = { version = "1.10", features = ["serde", "v4"] }

### Architecture
- Core module structure with lib.rs, main.rs, and modular source organization
- CLI module for argument parsing
- Data module for file loading and schema inference
- IronCalc module for spreadsheet integration
- Transformations module for pipeline management
- TUI module for terminal interface
- TUI grid module for grid rendering

### Documentation
- README with comprehensive feature documentation
- API reference with module exports
- Usage examples demonstrating IronCalc integration
- Contributing guidelines
- Project roadmap documentation

### Known Limitations (Aspirational Features)
The following features are documented but not yet implemented:

1. **Step list panel in TUI** — Left sidebar showing named transformation steps
2. **Sidecar read/write** — `.nustage.json` file format for pipeline serialization
3. **SQL transparency** — Display generated DuckDB queries in TUI
4. **Real Excel loader** — IronCalc currently has partial Excel reading support
5. **Charts and visualization** — Not scope for MVP
6. **Nushell integration** — Scripting layer (aspirational)
7. **Collaboration features** — Not part of core problem domain
8. **Content-addressed sidecars** — Hash-based pipeline identity (long-term vision)
9. **Custom expression language** — DuckDB SQL is sufficient for MVP
10. **Richer autocomplete features** — Basic field awareness exists
11. **Performance optimizations for large datasets** — Core functionality tested on sample data
12. **Real cell rendering in TUI** — Grid viewing handled by Tabiew (external tool)
13. **WASM/web frontend** — Deferred until core API stabilizes
14. **Joins, pivots, unpivots** — Advanced transforms not yet built

## [0.1.0] — Unreleased (Deprecated)

### Planned Features
- Expression language integration
- Advanced data transformations
- Richer autocomplete
- Performance optimizations
- Export capabilities
- Custom formatting
- Charts and visualizations

### Project Status
Initial development phase. Core functionality implemented but not yet fully tested or released to the public.

**Note:** Version 0.1.0 was replaced by 0.1.2 with honest documentation that distinguishes between implemented features and aspirational work.

---

**Version 0.1.4** adds sidecar pipeline provenance and Excel CSV export capabilities to the core library.
