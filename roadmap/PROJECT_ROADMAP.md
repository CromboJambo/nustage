# NuStage: The Alternate Spreadsheet Lineage

## Project Vision
Build a terminal-native, locally-first computational environment that treats data as streams and formulas as composable operations, combining spreadsheet utility with the philosophy of Smalltalk and the determinism of build systems. NuStage captures the essence of what spreadsheet software could have become if Lotus 1-2-3 had not been crushed by Excel's corporate standardization.

## Core Philosophy

### The Alternate Spreadsheet Concept
The spreadsheet metaphor was co-opted by corporate interests. The alternate lineage would have been about:
- **Clarity over compatibility**
- **Sovereignty over lock-in**  
- **Composability over feature accumulation**
- **Transparency over opacity**

### Core Design Principles
1. **Data as Streams**: Data is immutable, flowing through composable operations
2. **Composable Operations**: Each operation is pure, reversible, and testable
3. **Cells Above the Grid**: Views and transformations happen above the data, not within cells
4. **Terminal-First**: Keyboard-driven, lightweight, scriptable, and efficient
5. **Sovereign Computation**: Local-first, version-controlled, and transparent

### Piping Commands Over Ranges, Not Formulas in Cells
This is the fundamental departure from Excel's legacy. Instead of cell-based formulas that are navigationally silent and break everything when moved:
- Pipe commands over ranges: `data | filter "region='West'" | group "product"`
- Range syntax with field awareness: `$field.revenue - $field.cost` or `$row.revenue - $row.cost`
- Each pipeline step is visible, named, repeatable, and auditable
- The tool exists because copy-paste-values-only from CSV export is rational behavior given current alternatives

### The "Cells Above the Grid" Architecture
```
┌─────────────────────────────────────┐
│  Layer 1: Composable Operations      │  ← "Cells above the grid"
│  ─────────────────────────────────   │
│  data.csv | filter "region='West'"   │
│    | select "name, revenue"          │
│    | sort "revenue"                  │
│    | group "product" | sum "revenue" │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│  Layer 2: Views & Transformations    │  ← Pivot, slicer, timeline
│  ─────────────────────────────────   │
│  [Pivot Table]  [Filters]  [Slicers]│
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│  Layer 3: Immutable Data Source     │  ← CSV/Parquet/Excel
│  ─────────────────────────────────   │
│  [Raw Data Grid]                     │
└─────────────────────────────────────┘
```

### The Witness Layer Distinction
Nustage and Tabiew serve different purposes:
- **Tabiew**: Grid viewing, inspection, cell-oriented navigation (the witness)
- **Nustage**: Pipeline orchestration, transformation steps, reproducible workflows (the stage)
This boundary should remain explicit to avoid accidentally rebuilding what already exists.

### Domain Advantage: Hierarchical Cost Data
The real edge is manufacturing cost data structures that Excel struggles with:
- Bill of Materials: box in box in box hierarchies
- Standard vs actual variance tracking
- Multi-level rollups with clear provenance
This is why copy-paste-values-only from CSV export is rational behavior today — the step-based model makes this accessible to accounting colleagues.

## Architecture

```
NuStage Core (Rust)
  ↓
Data Model (Immutable DataFrames)
  ↓
DAG System (Composable Operations)
  ↓
Transform Engine (Pure Functions)
  ↓
Tabular Results
  ↓
TUI/GUI Visualization
```

## Tech Stack

### Core Components
- **Data Processing**: Polars (columnar, fast, Python-compatible)
- **TUI Framework**: Ratatui (Rust terminal UI library)
- **Data Reading**: Calamine (Excel/CSV/Parquet support)
- **CLI**: Clap (Rust CLI parsing)
- **State Management**: Custom reactive system for DAG state

### Optional Integrations
- VisiData (UX reference for terminal data tools)
- IronCalc/Excel (as output format, not computational layer)
- Calamine (reading Excel/CSV files as input sources)

Note: Nushell integration is aspirational weight rather than core motivation. The goal is to make step-based pipelines accessible without requiring users to learn a new scripting language.

## Implementation Roadmap

### Phase 1: Foundation (Weeks 1-2)
- [ ] Project structure and module organization
- [ ] Data loading (CSV/Parquet/Excel)
- [ ] Basic DataFrame operations
- [ ] Schema introspection
- [ ] Immutable data model

### Phase 2: Core Engine (Weeks 3-4)
- [ ] DAG system implementation
- [ ] Transform operations (filter, select, sort)
- [ ] Composable operation builder
- [ ] Pipeline state management
- [ ] Version tracking for nodes

### Phase 3: CLI Layer (Weeks 5-6)
- [ ] Clap-based CLI interface
- [ ] Pipeline construction commands
- [ ] Inspect and visualize commands
- [ ] Interactive pipeline editing

### Phase 4: TUI Layer (Weeks 7-8)
- [ ] Ratatui grid renderer
- [ ] DAG visualization
- [ ] Node exploration interface
- [ ] Live data preview at each node
- [ ] Basic pipeline editing

### Phase 5: Advanced Features (Weeks 9-12)
- [ ] Group and aggregate operations
- [ ] Custom transform creation
- [ ] Export capabilities (Excel, CSV, Parquet, TSV)
- [ ] Performance optimization
- [ ] Cell-oriented diff mode for version comparison

### Phase 6: Extensions (Weeks 13-16)
- [ ] Custom expression language with `$field` and `$row` syntax
- [ ] Content-addressed sidecar files (hash-based identity)
- [ ] Advanced visualization

## Core Data Structures

### Dataset Model
```rust
pub struct Dataset {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub created_at: DateTime,
    pub columns: Vec<String>,
}
```

### Transform Node
```rust
pub struct TransformNode {
    pub id: String,
    pub name: String,
    pub operation: Operation,
    pub dependencies: Vec<String>,
    pub outputs: Vec<String>,
    pub version: u64,
}
```

### Pipeline
```rust
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub nodes: HashMap<String, TransformNode>,
    pub edges: Vec<(String, String)>,
}
```

### Operation Types
```rust
pub enum Operation {
    Filter(String),
    Select(Vec<String>),
    Group(String),
    Aggregate(String, String),
    Sort(String),
    Join(String),
    Custom(String),
}
```

## Key Features

### 1. Composable Operations
Instead of cell formulas like `=SUM(A1:A10)`, use pipeline syntax:
```bash
data.csv | 
  filter "region='West'" | 
  select "name, revenue" | 
  sort "revenue"
```

### 2. DAG-Based Pipeline Management
- Each operation is a node in a directed acyclic graph
- Dependencies are explicit and navigable
- Version tracking enables reproducibility
- Branching and merging supported

### 3. Terminal-First UX
- Keyboard-driven workflows
- Lightweight and efficient
- Scriptable via CLI
- Integrated with shell environment

### 4. Immutable Data Model
- Original data never mutated
- Each operation produces new immutable version
- Clear dependency graph
- Easy to understand and debug

### 5. Field Awareness
- Schema introspection with autocomplete
- Context-sensitive operations
- Type inference and validation
- Error messages that guide correct usage

### 6. Export Format Flexibility
- Excel as one output option among many (CSV, Parquet, TSV)
- Cell-oriented diffs for change tracking across pipeline versions
- Sidecar files with content-addressed identities (hash-based versioning)

## Technical Challenges

1. **Expression Parser**: Need robust parsing for custom expression language
2. **DAG State Management**: Maintain transformation pipeline state efficiently
3. **Performance**: Handle large datasets in TUI without lag
4. **Visualization**: Effective DAG and data visualization
5. **Extensibility**: Easy to add new operations and integrations

## Development Priorities

### Short Term (Weeks 1-4)
1. Data loading and schema introspection
2. Basic transform operations (filter, select, sort)
3. Simple DAG builder
4. Basic TUI grid rendering

### Medium Term (Weeks 5-8)
1. Advanced transforms (group, aggregate, join)
2. Interactive pipeline editing
3. Live data preview
4. Performance optimization

### Long Term (Weeks 9+)
1. Custom expression language
2. Nushell scripting integration
3. WASM frontend support
4. Collaboration features
5. Advanced visualization

## Success Metrics

### Success Metrics

- **Usability**: Can accounting colleagues actually use the step-based model without reverting to copy-paste-values-only from CSV export
- **Performance**: Handle datasets that would choke Excel
- **Composability**: Operations are pure, reversible, and testable
- **Sovereignty**: Local-first, version-controlled, and transparent
- **Extensibility**: Easy to add new operations and integrations

The north star: making the step-based model accessible enough that your accounting colleagues would actually use it instead of relying on CSV exports.

## The "Cells Above the Grid" Concept

This is the key innovation that separates NuStage from traditional spreadsheets:

**Cells in current spreadsheets:**
- Navigationally silent
- Break everything
- Are modular but opaque
- Hide the full picture

**Cells above the grid in NuStage:**
- Make data immutable
- Enable multiple perspectives
- Make operations explicit
- Make the system testable
- Enable composable transformations

## Example Pipeline

```bash
# Load data
nustage load transactions.csv

# Build pipeline
nustage pipeline new "sales_analysis"
nustage pipeline add filter "region='West'"
nustage pipeline add group "product"
nustage pipeline add aggregate "sum" "revenue"

# Run pipeline
nustage pipeline run

# Visualize
nustage visualize

# Inspect
nustage inspect
```

## The "Dangerous" Part

NuStage makes spreadsheets **comprehensible** instead of **opaque**. It makes data **transparent** instead of **black-boxed**. It makes computation **reproducible** instead of **ad-hoc**.

This is not just a spreadsheet tool - it's a computational worldview that never got corporatized. It's the spreadsheet metaphor applied to the modern world of data, computation, and sovereignty.

## Next Steps

### Next Steps

1. **Compression before rebuild**: Throw away the Model<'static> lifetime hack, dummy cell rendering, placeholder Excel loader, .bak files — those are symptoms of building forward through uncertainty
2. Set up project structure with Rust + Polars + Ratatui
3. Implement data loading and schema introspection
4. Build DAG system and transform operations
5. Create CLI interface
6. Develop TUI visualization
7. Test with manufacturing cost data structures (BOM hierarchies, variance tracking)
8. Make the step-based model accessible to accounting colleagues — that's the real test