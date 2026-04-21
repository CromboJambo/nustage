# Nustage 0.1.2 — Honest Documentation Release

> Historical note: this file reflects the repository state and documentation claims at the time of the 0.1.2 release. It is not a description of the current codebase after later scope cuts. For the current surface, use `README.md` and `QUICKSTART.md`.

**Release Date:** 2026-03-03  
**Version:** 0.1.2  
**Status:** First honest public release

---

## Overview

Version 0.1.2 marks a significant transition in how we document the Nustage project. This release addresses the disconnect between our codebase and our documentation, replacing aspirational claims with honest assessments of what's actually implemented versus what's planned.

---

## What Changed

### Documentation Files Updated

1. **README.md**
   - Removed duplicate "Demo Status: READY" claims that overstated implemented features
   - Clearly marked aspirational features with status indicators (✅ Implemented, ⏳ Aspirational, Partial)
   - Rewrote "The Witness Layer (TUI)" section to distinguish current capabilities from aspirational work
   - Simplified "Sidecar" section to indicate it's not yet built
   - Consolidated "What To Build Next" and "What To Ignore" into single "Current Implementation Status" section
   - Updated tech stack table with clear status for each component

2. **QUICKSTART.md**
   - Removed code examples referencing unbuilt features
   - Added clear status indicators for each feature
   - Simplified structure to focus on what's actually working
   - Moved aspirational features to dedicated section
   - Removed implementation-specific code examples (M code, etc.)

3. **IMPLEMENTATION_SUMMARY.md**
   - Completely rewrote to reflect honest current state
   - Distinguished between what was removed from refactor vs what remains
   - Clearly marked aspirational features with priority levels (High, Medium, Long-term)
   - Added "What Was Removed" section documenting refactor cleanup
   - Added "Next Steps" with realistic timelines

4. **CHANGELOG.md**
   - Updated from version 0.1.1 to 0.1.2
   - Added new changelog entry documenting the cleanup effort
   - Removed unreleased [0.1.0] entry (it was superseded)
   - Consolidated "Future Work" into "Known Limitations" section
   - Added status note about the transition to honest documentation

5. **PROJECT_ROADMAP.md**
   - Added version header and current status section at top
   - Marked aspirational features with status indicators
   - Rewrote implementation priorities with realistic expectations
   - Updated "Success Metrics" to reflect current state
   - Simplified "Next Steps" to be less aspirational

6. **COMPREHENSIVE_ROADMAP.md**
   - Added version header and current status section at top
   - Marked aspirational features in sections 6-9 (Live SQL, Column Management, etc.)
   - Updated Technical Challenges section to clarify what needs implementation
   - Rewrote Development Priorities with status indicators
   - Updated Early Milestones table with current completion status
   - Marked Differentiators with implementation status

7. **IRONCALC_INTEGRATION.md**
   - Rewrote to reflect IronCalc as compatibility layer, not full spreadsheet engine
   - Marked read-only Excel support as implemented, write support as aspirational
   - Simplified API reference to current implemented capabilities
   - Added "Known Limitations" section
   - Updated usage examples to use CSV/Parquet instead of aspirational Excel features

8. **Cargo.toml**
   - Updated version from `0.1.1` to `0.1.2`

---

## Current Implementation Status

### What's Working ✅

- CLI binary with `--tui` flag for interactive mode
- CSV and Parquet data loading via Polars
- Schema inference on load
- TUI with grid preview (row count, shape display)
- Unit tests pass
- Examples (`simple_demo`, `ironcalc_integration`) compile and run
- Core transformation operations (filter, add column, select, group by, sort)
- DuckDB execution engine
- IronCalc read-only Excel support

### What's Planned (Aspirational) ⏳

- Step list panel in TUI
- Sidecar read/write (`.nustage.json` format)
- SQL transparency display
- Real Excel loader with full write support
- Charts and visualization
- Nushell integration
- Content-addressed sidecars
- WASM/web frontend

---

## Key Principles

### Honest Documentation

This release commits to:
- **No aspirational marketing** — If it's not built, we don't claim it is
- **Status indicators** — Clear visual markers for what's working vs planned
- **Transparent limitations** — Document current constraints without sugarcoating
- **Prioritized roadmap** — Feature lists reflect actual implementation priority

### What the Code Actually Does

Nustage is a terminal-native pipeline orchestration layer for tabular data. It:
- Pipes commands over ranges, not cell formulas
- Uses immutable, ordered, named transformation steps
- Executes via DuckDB for SQL processing
- Displays results in a terminal grid (via Tabiew)
- Supports CSV and Parquet files as data sources

It is **not**:
- A grid viewer (use Tabiew for that)
- A full spreadsheet replacement (yet)
- An Excel clone
- A collaboration tool
- A WASM/web application

---

## Why This Matters

The previous documentation presented an aspirational vision that didn't match the codebase. This release:

1. **Builds trust** — Users know what will actually work before using the tool
2. **Sets expectations** — Clear distinction between MVP features and future work
3. **Focuses development** — Team can prioritize based on honest status indicators
4. **Attracts right users** — Accounting colleagues won't be misled about capabilities

---

## Migration Guide

### For Existing Users

No action required. The codebase changes are internal. CLI usage remains:

```bash
cargo build --release
./target/release/nustage test_data/sales.csv
```

### For Documentation Consumers

Update your mental model:
- Features without ✅ or ⏳ indicators may not work as described
- Check the "Aspirational" sections before planning workflows
- Sidecar files are not yet supported for version control

---

## See Also

- [`README.md`](../README.md) — Project overview with current implementation status
- [`QUICKSTART.md`](../QUICKSTART.md) — Build and run instructions
- [`IMPLEMENTATION_SUMMARY.md`](../IMPLEMENTATION_SUMMARY.md) — Refactor summary and current state
- [`ROADMAP.md`](../roadmap/ROADMAP.md) — Prioritized feature list
- [`CHANGELOG.md`](../CHANGELOG.md) — Version history

---

**Version:** 0.1.2  
**Date:** 2026-03-03  
**Note:** This release marks the transition from aspirational claims to honest documentation.
