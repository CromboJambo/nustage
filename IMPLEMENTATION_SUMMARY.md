# Nustage Implementation Summary

This file summarizes the current implementation after scope reduction.

## Present in the Codebase

- A library crate exporting `data` and `transformations`
- A minimal CLI in [`src/main.rs`](/home/crombo/nustage/src/main.rs)
- Schema extraction helpers in [`src/data.rs`](/home/crombo/nustage/src/data.rs)
- Serializable pipeline types in [`src/transformations.rs`](/home/crombo/nustage/src/transformations.rs)
- Two runnable examples in [`examples/simple_demo.rs`](/home/crombo/nustage/examples/simple_demo.rs) and [`examples/power_query_workflow.rs`](/home/crombo/nustage/examples/power_query_workflow.rs)

## Removed or No Longer in Scope Here

- TUI and grid-preview layers
- Spreadsheet-style interaction
- Sidecar persistence modules
- DuckDB execution plumbing
- Excel compatibility layers
- Integration contracts built around those removed modules

## Practical Interpretation

Nustage currently behaves more like a focused library skeleton plus examples than a full workflow product. The important public surface is:

- `nustage::data::get_schema`
- `nustage::transformations::TransformationPipeline`
- `nustage::transformations::TransformationStep`
- `nustage::transformations::TransformationFactory`

The CLI exists, but it is intentionally thin and should not be documented as a complete workflow runner.

## Documentation Policy

Docs in this repo should:

- describe the code that exists now
- avoid calling removed systems "aspirational" when they are out of scope
- keep Nustage focused on reproducible tabular workflow semantics rather than spreadsheet UX
