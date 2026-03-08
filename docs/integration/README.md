# Nustage Integration - Project Perspective

This document explains how `nustage` integrates with `zed-sheet-lsp` and `git-sheets` from the perspective of the intent engine.

## Nustage's Role in the Stack

```
┌─────────────────┐         ┌──────────────────┐      ┌─────────────┐
│  git-sheets     │         │   zed-sheet-lsp  │      │  (external) │
│  (history)      │         │   (editor)       │      │             │
└─────────────────┘         └────────┬─────────┘      └─────────────┘
                                     │ depends on:          ▲
                                     │ nustage types        │
                                     ▼                      │
                          ┌──────────────────┐              │
                          │   nustage        │◄─────────────┘
                          │  (intent engine)│   file system
                          │                  │   contracts
                          └──────────────────┘
```

**Core Responsibility:** Define what transformations mean and how they should be applied to tabular data.

## What We Export for Others

### For zed-sheet-lsp (Editor Integration)

We provide these stable types that the editor can consume:

```rust
// From nustage::sidecar
pub struct SidecarFile {
    pub version: u32,
    pub source: String,
    pub pipeline: Vec<TransformationStep>,
    pub schema_history: HashMap<String, Vec<ColumnSchema>>,
    // ...
}

pub struct SidecarMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    // ...
}

// From nustage::transformations
pub enum StepType {
    FilterRows(String, String),
    AddColumn { name: String, expr: String },
    GroupBy(Vec<String>, Vec<Aggregation>),
    SortBy(Vec<(String, SortOrder)>),
    // ...
}

pub struct TransformationStep {
    pub step_id: String,
    pub r#type: StepType,
    // ...
}

pub struct ColumnSchema {
    pub name: String,
    pub type_name: String,
}
```

**Usage in zed-sheet-lsp:**
```rust
use nustage::sidecar::{SidecarFile, SidecarMetadata};
use nustage::transformations::{TransformationStep, ColumnSchema};

pub fn load_sidecar(path: &Path) -> Result<SidecarFile> {
    SidecarFile::load(path.to_str().unwrap())
}
```

### For git-sheets (History Manager)

We don't export types directly. Instead, we produce files that can be snapshot:

- **`.nustage.json`** - The sidecar file itself can be snapshot for audit trails
- **Processed output files** - Results of pipeline execution can be snapshotted

Example workflow:
```bash
# Execute pipeline via nustage
nustage process data.csv  # Produces processed_data.csv

# Snapshot the result via git-sheets
git-sheets snapshot processed_data.csv -m "Post-transformation state"
```

## What We Don't Export (Intentionally)

We deliberately keep these concerns separate:

- **Editor-specific types** - No LSP protocol handling, no Zed integration code
- **Snapshot/diff logic** - No knowledge of git-sheets format or history management
- **File watching** - No built-in support for detecting file changes (editor's job)

This separation ensures we remain usable without the other projects.

## Integration Points

### Point 1: Sidecar Loading (Primary Integration)

**Trigger:** Editor opens a file with matching `.nustage.json`  
**Flow:**
1. `zed-sheet-lsp` detects `.nustage.json` next to data file
2. Calls `SidecarFile::load()` from our library
3. Uses returned types for diagnostics/completion

**Code location in us:** `nustage/src/sidecar/mod.rs`  
**Implementation status:** ✅ Complete - stable API

### Point 2: Pipeline Validation (Shared Contract)

Both projects need to agree on what constitutes a valid pipeline:

- **Validation happens in:** `nustage::transformations::validate_pipeline()`
- **Editor surfaces errors as:** LSP diagnostics from `zed-sheet-lsp`
- **git-sheets can snapshot:** Validated sidecar for audit trail

**Code location in us:** `nustage/src/transformations/mod.rs`  
**Integration status:** ✅ Complete - shared validation logic

### Point 3: Column Rename (Future Integration)

**Trigger:** User renames column "Revenue" to "Gross Revenue" in editor  
**Flow:**
1. Editor requests rename via LSP
2. Calls `nustage::transformations::rename_column()`
3. Updates all pipeline steps atomically
4. Saves updated sidecar

**Code location for us:** To be added - `nustage/src/transformations/rename.rs`  
**Integration status:** ⏳ Planned - requires coordination with editor team

## Stability Guarantees

### Types Marked as Stable Contracts

The following types and their serialization formats are **stable contracts**:

1. **`SidecarFile`** (and `SidecarMetadata`)
   - Format: `.nustage.json` schema
   - Breaking changes require version bump + migration path
   - Guaranteed backward compatibility within major versions

2. **`TransformationStep` and `StepType`**
   - Pipeline step model for transformation semantics
   - Used by editor for diagnostics/completion
   - New step types can be added (extensible enum pattern)

3. **Schema versioning**
   - `SidecarFile.version` field controls format compatibility
   - Version 1: Current stable format
   - Future versions will follow semver with migration paths

### Types That May Change

Types not listed above are considered internal implementation and may change without notice:

- Helper functions for sidecar loading/saving
- Internal validation logic (as long as API contract holds)
- CLI-specific types and commands

## Dependency Direction

**We do NOT depend on:** `zed-sheet-lsp` or `git-sheets`

This is intentional. We remain standalone-capable:

```bash
# Works without editor integration
nustage process data.csv

# Works without version control
nustage init sales.csv && nustage add-step filter Revenue > 1000
```

**We MAY provide types for:** `zed-sheet-lsp` to consume

The editor is the only project that should depend on us as a library. This makes sense:
- Editor needs to understand our pipeline model
- We don't need to know about their LSP protocol or Zed integration

## Testing Integration Points

### Unit Tests (Standalone Verification)

We maintain tests that verify we work without other projects:

```bash
# Test sidecar serialization independently
cd nustage && cargo test sidecar::test_serialization_roundtrip

# Test pipeline validation independently  
cd nustage && cargo test transformations::test_pipeline_validation
```

### Integration Tests (With Editor)

When editor integration tests are added, they will verify:

- Editor can load our `SidecarFile` types correctly
- Diagnostics match expected validation errors
- Rename operations update all steps atomically

**Status:** Pending - requires coordination with editor team

## Migration Paths for Breaking Changes

If we need to change a stable contract:

1. **Bump the version field** in relevant type (e.g., `SidecarFile.version`)
2. **Maintain old format support** for at least one major release
3. **Document migration path** in this file and user-facing docs
4. **Notify dependent projects** before making breaking change

Example: Adding new step type

```rust
// GOOD - Extensible enum pattern
pub enum StepType {
    // ... existing variants
    NewStepType { /* fields */ },  // Future addition
}

// Serialization handles unknown variants gracefully
```

## Related Documents

- **[../docs/integration/README.md](./README.md)** - Master integration document (this project's perspective)
- **[../../git-sheets/docs/integration/AUDIT.md](../../git-sheets/docs/integration/AUDIT.md)** - Type overlap analysis from other projects' view
- **[../../git-sheets/docs/integration/dependency-config.md](../../git-sheets/docs/integration/dependency-config.md)** - Dependency management strategy

## Questions for Contributors

**Q: Can I add a new transformation type?**  
A: Yes, extend `StepType` enum. Document in CHANGELOG and ensure backward compatibility.

**Q: Should I modify `SidecarFile` structure?**  
A: Only if adding optional fields. Breaking changes require version bump + migration path.

**Q: Can we depend on git-sheets for snapshotting?**  
A: No. We produce output files; git-sheets snapshots them independently.

**Q: How do I know if a type is stable?**  
A: Check this document's "Stability Guarantees" section or the public API documentation.

---

**Last updated:** 2025-01-XX  
**Maintainer:** nustage team  
**Contact:** Open an issue for integration-related questions
