# Nustage Quick Start Guide

> **Version 0.1.2** — The first public release with honest documentation.

---

## Build from Source

```bash
cargo build --release
```

---

## Run Examples

### CSV Data (Recommended for Demo)

```bash
cargo run --release --example simple_demo
```

### Parquet Data

```bash
cargo run --release --example simple_demo
```

### IronCalc Integration

```bash
cargo run --release --example ironcalc_integration
```

---

## Run Tests

```bash
cargo test --release
```

---

## Use the CLI

```bash
# Build
cargo build --release

# Run with CSV (recommended for demo)
./target/release/nustage test_data/sales.csv

# Run with Parquet
./target/release/nustage test_data/*.parquet
```

---

## TUI Mode

```bash
./target/release/nustage --tui test_data/sales.csv
```

---

## See Also

- [`README.md`](../README.md) — Overview of capabilities
- [`IMPLEMENTATION_SUMMARY.md`](../IMPLEMENTATION_SUMMARY.md) — Refactor summary
- [`ROADMAP.md`](roadmap/ROADMAP.md) — Feature priorities
- [`COMPREHENSIVE_ROADMAP.md`](roadmap/COMPREHENSIVE_ROADMAP.md) — Detailed specifications

---

## Current Status

**Implemented:**
- ✅ CLI binary with `--tui` flag
- ✅ CSV and Parquet data loading
- ✅ TUI with grid preview (row count, shape)
- ✅ Examples compile and run
- ✅ Unit tests pass

**Aspirational (not yet built):**
- ⏳ Step list panel in TUI
- ⏳ Sidecar read/write (`.nustage.json`)
- ⏳ SQL transparency display
- ⏳ Real Excel loader (currently partial support)
- ⏳ Charts and visualization
- ⏳ Nushell integration
- ⏳ Content-addressed sidecars

See the [`ROADMAP`](roadmap/ROADMAP.md) for what's being built next.