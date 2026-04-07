# Nustage Integration Notes

This document describes the integration boundary that exists in the current repository.

## Current Exported Surface

Other tools can reasonably depend on these modules:

- `nustage::data`
- `nustage::transformations`
- `nustage::core`

Today that means:

- schema extraction from a Polars `DataFrame`
- serializable schema structs
- serializable transformation pipeline and step structs

## What This Repo Does Not Currently Provide

The following were referenced by older architecture docs but are not present in the current tree:

- sidecar file types such as `.nustage.json`
- pipeline validation APIs
- rename-propagation helpers
- editor or LSP integration contracts
- git-sheets specific snapshot hooks

Any external project that wants to integrate with Nustage should treat those as non-existent unless they are reintroduced in code.

## Safe Integration Pattern

If another tool needs Nustage today, integrate at the library level:

```rust
use nustage::data::get_schema;
use nustage::transformations::TransformationPipeline;
```

Use Nustage for step metadata and schema-aware workflow structures. Keep UI, editor, versioning, and snapshot concerns outside this crate.

## Boundary Reminder

Nustage owns workflow semantics over tables and fields.

Nustage does not own:

- grid viewers
- spreadsheet navigation
- cell-addressed editing
- editor-specific UX contracts

If those integrations return later, this document should be expanded from the code, not from old plans.
