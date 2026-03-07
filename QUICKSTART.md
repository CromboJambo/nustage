# Nustage Quick Start Guide

## Build and Run

```bash
# Build from source
cargo build --release

# Run the CLI on sample data
./target/release/nustage test_data/sales.csv

# Run the TUI mode
./target/release/nustage --tui test_data/sales.csv
```

## Sidecar Workflow

Nustage keeps source data unchanged and stores transformation intent in a sidecar:

```
sales.csv
sales.nustage.json
```

The sidecar uses the canonical `TransformationStep` model from `src/transformations/mod.rs`.

## M Code Export

The `src/mcode` module provides conversion from canonical pipeline steps into Power Query M.
This is currently a library capability, not a full web product.

Example usage from Rust:

```rust
use nustage::{StepType, TransformationStep, generate_m_code};
use std::collections::HashMap;

let pipeline = vec![
    TransformationStep {
        id: "1".into(),
        name: "filter_revenue".into(),
        step_type: StepType::FilterRows("Revenue".into(), "> 1000".into()),
        parameters: HashMap::new(),
        output_schema: vec![],
    }
];

let m = generate_m_code(&pipeline, "Excel.CurrentWorkbook(){[Name=\"Table1\"]}[Content]");
println!("{m}");
```

## Current Scope

1. CLI/TUI pipeline work remains the primary interface.
2. Sidecar storage and diff helpers are implemented in `src/sidecar`.
3. Power Query M generation is implemented in `src/mcode`.
4. Browser/WASM interface is deferred until the core API stabilizes.
