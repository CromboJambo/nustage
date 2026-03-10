# NuStage: Honest Project Roadmap [Version 0.1.2]

> **Piping commands over ranges, not formulas in cells.**

---

## Current Project Status

**Version:** 0.1.2 — First honest public release

**Implemented and Working:**
- ✅ CLI binary with `--tui` flag for terminal UI
- ✅ CSV and Parquet data loading via Polars
- ✅ DuckDB execution engine for data processing
- ✅ TUI with grid preview (row count, shape display)
- ✅ Unit tests pass
- ✅ Examples: `simple_demo` and `ironcalc_integration`

**Aspirational (Not Yet Built):**
- ⏳ Step list panel in TUI
- ⏳ Sidecar read/write (`.nustage.json` format)
- ⏳ SQL transparency display
- ⏳ Real Excel loader (partial support only)
- ⏳ Charts and visualization
- ⏳ Nushell integration
- ⏳ Content-addressed sidecars
- ⏳ WASM/web frontend

See [`README.md`](../README.md) for current implementation status.

---

## Core Philosophy

### Step Model Over Cell-Based Formulas

Instead of cell-based formulas like `=SUM(F7:F23)` that break when someone moves a column, we use pipeline syntax:

```bash
data.csv | filter "region='West'" | group "product" | sum "revenue"
```

Each step is:
- **Named** — readable by a human
- **Immutable** — never mutates the source
- **Ordered** — forms a reproducible chain
- **Reversible** — delete or reorder without starting over

### Domain Advantage: Manufacturing Cost Data

The real edge is manufacturing cost structures that Excel struggles with:
- Bill of Materials: hierarchical, box-in-box hierarchies
- Standard vs actual variance tracking
- Multi-level rollups with clear provenance

The tool exists because copy-paste-values-only from CSV export is rational behavior today — the step-based model makes this accessible to accounting colleagues.

---

## What's Implemented (0.1.2)

### 1. CLI Binary

```bash
cargo build --release
./target/release/nustage test_data/sales.csv
```

Features:
- Accepts CSV and Parquet file paths
- Supports `--tui` flag for interactive mode
- Proper error handling and argument parsing

### 2. Data Loading

Via Polars:
- CSV files — full support
- Parquet files — full support
- Schema inference on load

### 3. TUI Grid Preview

```bash
./target/release/nustage --tui test_data/sales.csv
```

Features:
- Grid display via Tabiew (external tool)
- Row count and shape display
- Keyboard-driven interface

### 4. Examples

```bash
cargo run --release --example simple_demo
cargo run --release --example ironcalc_integration
```

Both examples compile and run successfully.

### 5. Testing

```bash
cargo test --release
```

Unit tests pass on core data loading and transformation pipeline.

### 6. Module Structure

```
src/
├── main.rs           — CLI entry point
├── lib.rs            — Library exports
├── cli/              — Argument parsing
├── data/             — File loading and schema inference
├── transformations/   — Core transform operations
├── sidecar/          — Pipeline serialization (aspirational)
├── mcode/            — Power Query M generation (library capability)
├── ironcalc/         — Excel compatibility layer (partial)
├── tui/              — Terminal UI rendering
└── tui_grid.rs       — Grid rendering helpers
```

---

## What's Aspirational (Not Yet Built)

### High Priority (0.1.3–0.1.4)

1. **Step List Panel** — Left sidebar showing named transformation steps with reorder/delete actions
2. **Sidecar Read/Write** — `.nustage.json` files for versioning pipeline definitions
3. **SQL Transparency** — Display generated DuckDB queries in the TUI
4. **Real Excel Loader** — IronCalc currently supports partial Excel reading

### Medium Priority (0.1.5–0.2.0)

5. **Charts and Visualization** — Not MVP scope
6. **Rich Autocomplete** — Basic field awareness exists; richer context-aware suggestions needed
7. **Performance Optimization** — Core functionality tested on sample data; large datasets untested
8. **Real Cell Rendering** — Grid viewing handled by Tabiew (external tool)

### Long-Term Vision (Future Releases)

9. **Nushell Integration** — Scripting layer (aspirational)
10. **Collaboration Features** — Not part of core problem domain
11. **Content-Addressed Sidecars** — Hash-based pipeline identity (long-term)
12. **WASM/Web Frontend** — Deferred until core API stabilizes
13. **Joins, Pivots, Unpivots** — Advanced transforms not yet built
14. **Custom Expression Language** — DuckDB SQL is sufficient for MVP

---

## Implementation Priorities

### Immediate (0.1.2 Release)

- ✅ Review and approve cleaned documentation
- ✅ Update Cargo.toml version to 0.1.2 (pending)
- ⏳ Review roadmap files for honest status markers

### Short Term (Next Sprint)

1. Build step list panel in TUI
2. Implement sidecar read/write
3. Add SQL transparency display
4. Fix Excel loader (current workaround requires CSV conversion)

### Medium Term (Next 1–2 Months)

5. Performance optimization for larger datasets
6. Richer autocomplete with context awareness
7. Test with real manufacturing cost data (BOM hierarchies)
8. Export capabilities (CSV, Parquet, Excel, TSV)

### Long Term (Future Releases)

9. Custom expression language with `$field` and `$row` syntax
10. Content-addressed sidecar files (hash-based identity)
11. Nushell scripting integration (if desired)
12. WASM frontend (if core API stabilizes)

---

## Success Metrics

### Usability

> Can accounting colleagues actually use the step-based model without reverting to copy-paste-values-only from CSV export?

**Current State:** Basic pipeline concept demonstrated via CLI and TUI preview.

### Performance

> Handle datasets that would choke Excel.

**Current State:** Core functionality tested on sample data in `test_data/`.

### Composability

> Operations are pure, reversible, and testable.

**Current State:** Implemented in Polars/DuckDB; step model exists but UI for managing steps is aspirational.

### Sovereignty

> Local-first, version-controlled, and transparent.

**Current State:** Works entirely offline; Git tracks changes; sidecar versioning is aspirational.

### Extensibility

> Easy to add new operations and integrations.

**Current State:** Module structure supports additions; DuckDB SQL provides extensible backend.

---

## The North Star

The tool exists because copy-paste-values-only from CSV export is rational behavior given current alternatives. The goal is to make the step-based model accessible enough that your accounting colleagues would actually use it — not about replacing Excel with a clone, but about making reproducible pipelines transparent and auditable.

---

## See Also

- [`README.md`](../README.md) — Overview of capabilities and honest status
- [`QUICKSTART.md`](../QUICKSTART.md) — Build and run instructions
- [`IMPLEMENTATION_SUMMARY.md`](../IMPLEMENTATION_SUMMARY.md) — Refactor summary and current state
- [`CHANGELOG.md`](../CHANGELOG.md) — Version history
- [`COMPREHENSIVE_ROADMAP.md`](COMPREHENSIVE_ROADMAP.md) — Detailed feature specifications (also being cleaned up)

---

**Version:** 0.1.2  
**Date:** 2026  
**Status:** First honest public release

---

**Note:** This roadmap reflects the actual state of the codebase. Features marked as "aspirational" are documented but not yet built. See version 0.1.2 documentation for clear separation between what's working and what's planned.
