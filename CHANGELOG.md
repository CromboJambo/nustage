# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-03-03

### Added
- Initial public release of Nustage
- Terminal-native spreadsheet engine integration with IronCalc
- Power Query-style transformation pipeline system
- DuckDB-powered data processing engine
- Schema-aware data loading and inference
- Reversible, immutable transformation steps
- Version control ready sidecar (.nustage.json) format
- TUI (Terminal User Interface) with grid display capabilities
- CLI argument parsing with Clap
- Comprehensive error handling with Thiserror and Anyhow
- Data serialization/deserialization with Serde
- Support for CSV, Excel, and Parquet file formats
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

### Fixed
- Package version updated to 0.1.1 in Cargo.toml
- Project structure organized for public release

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

### Future Work (Not Implemented)
- Nushell integration
- Collaboration features
- WASM/web frontend
- Charts and visualization
- Custom expression language
- Advanced transforms (joins, pivots, unpivots)
- Richer autocomplete features
- Performance optimizations for large datasets
- Real cell rendering in TUI
- Fix Model lifetime in ironcalc/mod.rs
- Real Excel loader in data/mod.rs
- Step list panel in TUI
- Sidecar read/write
- SQL transparency

## [0.1.0] - [Unreleased]

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

---

**Note**: Version 0.1.0 was the initial development version. Version 0.1.1 is the first public release with documented features and improvements.