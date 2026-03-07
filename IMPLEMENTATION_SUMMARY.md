# Nustage Refactor Integration Summary

## Objective

Salvage the refactor into a mergeable branch by keeping the useful parts:

1. Canonical pipeline model (`transformations`)
2. Sidecar persistence (`sidecar`)
3. Power Query M export (`mcode`)

and removing unfinished surfaces that were causing integration and correctness risk.

## What Was Changed

1. Removed non-functional web/WASM implementation files:
   - `src/wasm.rs`
   - `web/app.js`
   - `web/styles.css`
2. Removed WASM-only dependencies and profile from `Cargo.toml`.
3. Fixed `src/lib.rs` re-export duplication and kept a clean public API.
4. Reworked `src/sidecar/mod.rs` to use canonical `TransformationStep` and `StepType` instead of a duplicate step model.
5. Replaced `src/mcode/mod.rs` with a compile-safe implementation that:
   - consumes canonical steps
   - generates valid multi-step `let ... in` bindings
   - correctly translates `@Field` references to `[Field]`
   - provides unit tests for core conversions
6. Updated docs (`README.md`, `QUICKSTART.md`) to reflect actual scope.

## Integration Decision

This branch now targets a clean integration path:

1. Merge the sidecar and M-code library capabilities.
2. Defer browser/WASM delivery until runtime boundaries and API contracts are stable.

## Follow-Up Work

1. Add fixture-based M-code validation tests against real Power Query snippets.
2. Expand sidecar round-trip tests with representative pipelines.
3. Reintroduce web/WASM only after a dedicated browser-safe data I/O layer is designed.
