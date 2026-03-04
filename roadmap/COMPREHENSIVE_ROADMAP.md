# Nustage: Spreadsheet TUI Project - Comprehensive Roadmap

## Executive Summary
Nustage is a terminal-first, staged data transformation layer that captures Power Query's core value proposition while leveraging modern Rust tools for better performance, extensibility, and terminal-native UX.

**Vision**: Empower users with sandboxed transform freedom without Excel's lock-in through a composable, reversible, versionable, terminal-first approach.

## Core Philosophy
- **Step Model Over Drag-and-Drop**: Immutable, reorderable, deletable transformation steps
- **Field Awareness**: Schema introspection with autocomplete
- **SQL Transparency**: Show generated SQL for learning and debugging
- **Stationary Filters**: Slicer/timeline-style UI elements for ad-hoc filtering
- **Terminal-First UX**: Keyboard-driven, lightweight, scriptable
- **Local-First**: Works entirely offline with no vendor lock-in

### Piping Commands Over Ranges, Not Formulas in Cells
This is the fundamental departure from Excel's legacy. Instead of cell-based formulas that are navigationally silent:
- Pipe commands over ranges: `data | filter "region='West'" | group "product"`
- Range syntax with field awareness: `$field.revenue - $field.cost` or `$row.revenue - $row.cost`
- Each pipeline step is visible, named, repeatable, and auditable

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

### The North Star
The tool exists because copy-paste-values-only from CSV export is rational behavior given current alternatives. The goal is to make the step-based model accessible enough that your accounting colleagues would actually use it. This is not about replacing Excel with a clone, but about making reproducible pipelines transparent and auditable.

## Architecture

### The Nustage Pipe and Stage Model
```
┌─────────────────────────────────────┐
│  Layer 1: Pipeline Definition        │  ← Named, repeatable steps
│  ─────────────────────────────────   │
│  data.csv | filter ... | group ...  │
└─────────────────────────────────────┘
              ↓ (deterministic execution)
┌─────────────────────────────────────┐
│  Layer 2: Transformation Engine      │  ← DuckDB backend, pure functions
│  ─────────────────────────────────   │
│  [Immutability guaranteed]           │
└─────────────────────────────────────┘
              ↓ (tabular result)
┌─────────────────────────────────────┐
│  Layer 3: Output Format Selection    │  ← Export to any grid format
│  ─────────────────────────────────   │
│  Excel | CSV | Parquet | TSV         │
└─────────────────────────────────────┘
```

### The IronCalc Compatibility Layer
IronCalc integration is not a shortcut — it's a compatibility layer:
- **Read**: Open existing Excel/CSV files as input sources (read-only)
- **Transform**: Data flows through immutable pipeline steps
- **Export**: Return format of choice with cell-oriented diff tracking
- The data never changes in the source; the Excel file is just a snapshot

### Witness vs Stage Separation
```
┌───────────────┐     ┌──────────────────┐
│   Tabiew      │     │    Nustage       │
│  (Witness)    │     │    (Stage)       │
├───────────────┤     ├──────────────────┤
│ Grid viewing  │     │ Pipeline         │
│ Inspection    │     │ Orchestration    │
│ Cell nav      │     │ Transformations  │
│ Diff view     │     │ Step history     │
└───────────────┘     └──────────────────┘
```

## Tech Stack

### Core Components
| Layer | Purpose | Candidate Tools / Crates |
|-------|---------|--------------------------|
| File Loader / I/O | Load spreadsheets, Parquet, CSV as input sources | Calamine (Excel/ODS), CSV/Parquet loaders |
| Step Pipeline / Transform Model | Named, repeatable, immutable transformations | Internal Rust AST for transforms |
| Expression Language | Piping commands over ranges | Custom syntax with `$field` and `$row` accessors |
| Execution Engine | Heavy-lifting analytics | DuckDB (embedded SQL engine) |
| TUI / Grid | Preview and interact with data | Ratatui, custom grid renderer |
| Autocomplete / Schema Awareness | Field discovery, formula assistance | Schema registry from loaded tables |
| Step Editor / Sidebar | Show step history, reorder, delete | Immutable list UI in TUI |
| Output Format Selection | Export to any grid format | IronCalc (Excel), CSV, Parquet, TSV writers |

### Compatibility Layer
- **IronCalc**: Read existing Excel files as input sources; export transformed data with cell-oriented diffs
- **Calamine**: Reading Excel/CSV files as input sources (read-only)
- The compatibility layer is not a shortcut — it's how Nustage speaks the language of existing workflows without adopting their computational model

### Optional / Aspirational
- **VisiData**: UX reference for terminal spreadsheet patterns
- **Content-addressed sidecars**: Hash-based identity for pipeline definitions (long-term)
- Note: Nushell integration is aspirational weight rather than core motivation. The goal is accessibility, not requiring users to learn a new scripting language

## MVP Requirements

### Phase 1: Essential Features
- [ ] Load CSV/Parquet/Excel files with automatic schema inference
- [ ] Display tabular data in grid format
- [ ] Schema introspection with field sidebar
- [ ] Add transformation steps:
  - [ ] Filter
  - [ ] Add column
  - [ ] Select columns
  - [ ] Group by
  - [ ] Sort
- [ ] Expression editor with `@field` syntax support
- [ ] Basic autocomplete for fields
- [ ] Live SQL generation display
- [ ] Save/load pipeline definition (versionable, text-based)

**Success Criteria**: Replaces 80% of common Excel use cases in a terminal environment

### Phase 2: Enhanced Features (Post-MVP)
- [ ] Advanced transforms (joins, pivots, custom SQL)
- [ ] Richer autocomplete with context awareness
- [ ] Performance optimization for larger datasets
- [ ] Export capabilities (CSV, Parquet, Excel, TSV)
- [ ] **Cell-oriented diff mode**: Compare pipeline versions at the cell level
- [ ] **Hierarchical cost data support**: BOM hierarchies, standard vs actual variance tracking
- [ ] Scripting via Nushell integration (aspirational weight, not core motivation)
- [ ] Multiple file format support
- [ ] **Cell-oriented diff mode**: Compare pipeline versions at the cell level for auditable change tracking

## Key UX Features

### North Star: Accessibility Over Replacement
The tool exists because copy-paste-values-only from CSV export is rational behavior given current alternatives. The goal is to make the step-based model accessible enough that your accounting colleagues would actually use it — not about replacing Excel with a clone, but about making reproducible pipelines transparent and auditable in manufacturing cost contexts (BOM hierarchies, standard vs actual variance).

### 1. Field Awareness
- Dropdown of available fields from current schema
- Schema introspection maintained in state
- Autocomplete in expression editor
- Field type information display

### 6. Cell-Oriented Diff Mode
Compare pipeline versions at the cell level:
- Side-by-side view of transformed vs source data
- Highlight changes between pipeline iterations
- Export diff reports for audit trails
- Content-addressed sidecar files (hash-based identity) enable stable version comparison

### 7. Stationary Filters (Slicer/Timeline Style)
Stationary filters are persistent UI elements that apply ad-hoc filtering without creating new steps. The witness layer (Tabiew) handles this, while Nustage focuses on the stage (pipelines).

### 8. Content-Addressed Sidecars
Long-term vision for pipeline versioning:
- Hash as stable identity for pipeline definitions
- Embed distance for traversal between versions
- Make the "archaeological evidence" in your file tree intentional and navigable

**UI Pattern:**
```
┌─────────────────────────────────────┐
│  📊 Sales Data                       │
│  ┌─────────┐  ┌─────────┐  ┌─────┐ │
│  │ Region  │  │  Date   │  │Amt  │ │
│  │ ○ North │  │[2024-01]│  │▼    │ │
│  │ ○ South │  │[2024-03]│  │[100]│ │
│  │ ○ East  │  │[2024-06]│  │     │ │
│  │ ○ West  │  │[2024-09]│  │     │ │
│  └─────────┘  └─────────┘  └─────┘ │
│  Showing 247 of 1,000 rows         │
└─────────────────────────────────────┘
```

**Key Characteristics:**
- **Non-destructive**: Filters don't modify the underlying data
- **Reversible**: Can be toggled on/off anytime
- **Composable**: Multiple filters can be combined (AND/OR logic)
- **Context-aware**: Filter options change based on column data types
- **Persisted**: Filters survive step transformations
- **Shareable**: Apply same filter to multiple columns

**Implementation Notes:**
- Separate filter state from step pipeline state
- Filter state = column_id + filter_type + filter_values
- Filter application = WHERE clause injection into DuckDB queries
- Filter UI = independent TUI panel (draggable, resizable)

### 2. Stationary Filters (Slicer/Timeline Style)

Stationary filters are persistent UI elements that apply ad-hoc filtering without creating new steps:

**UI Pattern:**
```
┌─────────────────────────────────────┐
│  📊 Sales Data                       │
│  ┌─────────┐  ┌─────────┐  ┌─────┐ │
│  │ Region  │  │  Date   │  │Amt  │ │
│  │ ○ North │  │[2024-01]│  │▼    │ │
│  │ ○ South │  │[2024-03]│  │[100]│ │
│  │ ○ East  │  │[2024-06]│  │     │ │
│  │ ○ West  │  │[2024-09]│  │     │ │
│  └─────────┘  └─────────┘  └─────┘ │
│  Showing 247 of 1,000 rows         │
└─────────────────────────────────────┘
```

**Key Characteristics:**
- **Non-destructive**: Filters don't modify the underlying data
- **Reversible**: Can be toggled on/off anytime
- **Composable**: Multiple filters can be combined (AND/OR logic)
- **Context-aware**: Filter options change based on column data types
- **Persisted**: Filters survive step transformations
- **Shareable**: Apply same filter to multiple columns

**Implementation Notes:**
- Separate filter state from step pipeline state
- Filter state = column_id + filter_type + filter_values
- Filter application = WHERE clause injection into DuckDB queries
- Filter UI = independent TUI panel (draggable, resizable)

### 3. Column Expressions
- Support `@field` syntax (e.g., `@Revenue - @Cost`)
- Parse and convert to SQL identifiers
- Validate expressions against schema
- Error handling and helpful messages

### 4. Step List (Power Query Style)
Left sidebar showing:
```
1. Source
2. Filter: @Date >= 2024-01-01
3. Add Column: Profit = @Revenue - @Cost
4. Group By: Region
```

Each step is:
- Selectable
- Reorderable
- Deletable
- Immutable (functional approach)
- Previewable

### 5. Stationary Filters UI

**Slicer Components:**
- **Slicers**: Column-based filters with visual selection (checkboxes, toggles)
- **Timelines**: Date range pickers for temporal columns
- **Custom Filters**: Input fields with operators (equals, contains, >, <, etc.)
- **Filter Summary**: "Showing X of Y rows" indicator

**Filter Behaviors:**
- **Multi-select**: Checkboxes allow selecting multiple values
- **Range filters**: Sliders or input fields for numeric ranges
- **Dropdowns**: Quick-select from unique values
- **Dynamic options**: Filter options regenerate when data changes
- **Filter sharing**: Apply same filter criteria to multiple columns
- **Filter stacking**: Multiple filters combine with AND logic by default
- **Filter reset**: Clear all filters with single action

**Filter State Management:**
- Separate storage: `filters: HashMap<ColumnId, FilterState>`
- FilterState: `{ type, values, operator }`
- Filter application: `WHERE` clause injection into DuckDB queries
- Filter persistence: Stored in pipeline definition alongside steps
- Filter sharing: Reference same filter state by ID across columns

**Filter vs. Steps:**
- **Stationary Filters**: Ad-hoc, reversible, UI-driven, don't create new steps
- **Steps**: Permanent, irreversible, pipeline-based, create new datasets
- **Hybrid Approach**: Filters apply to current step output, persist across subsequent steps

### 3. Step List (Power Query Style)
Left sidebar showing:
```
1. Source
2. Filter: @Date >= 2024-01-01
3. Add Column: Profit = @Revenue - @Cost
4. Group By: Region
```

Each step is:
- Selectable
- Reorderable
- Deletable
- Immutable (functional approach)
- Previewable

### 4. Stationary Filters UI
- Slicer-style filter controls positioned outside the main grid
- Timeline-style date range pickers for temporal filtering
- Draggable filter panels for flexible layout
- Visual indicators showing active filters and their values
- Filter summary showing "Showing X of Y rows"
- Filter persistence across step transformations
- Filter sharing: Apply same filter to multiple columns

Each step is:
- Selectable
- Reorderable
- Deletable
- Immutable (functional approach)
- Previewable

### 5. Stationary Filters UI

**Slicer Components:**
- **Slicers**: Column-based filters with visual selection (checkboxes, toggles)
- **Timelines**: Date range pickers for temporal columns
- **Custom Filters**: Input fields with operators (equals, contains, >, <, etc.)
- **Filter Summary**: "Showing X of Y rows" indicator

**Filter Behaviors:**
- **Multi-select**: Checkboxes allow selecting multiple values
- **Range filters**: Sliders or input fields for numeric ranges
- **Dropdowns**: Quick-select from unique values
- **Dynamic options**: Filter options regenerate when data changes
- **Filter sharing**: Apply same filter criteria to multiple columns
- **Filter stacking**: Multiple filters combine with AND logic by default
- **Filter reset**: Clear all filters with single action

**Filter State Management:**
- Separate storage: `filters: HashMap<ColumnId, FilterState>`
- FilterState: `{ type, values, operator }`
- Filter application: `WHERE` clause injection into DuckDB queries
- Filter persistence: Stored in pipeline definition alongside steps
- Filter sharing: Reference same filter state by ID across columns

**Filter vs. Steps:**
- **Stationary Filters**: Ad-hoc, reversible, UI-driven, don't create new steps
- **Steps**: Permanent, irreversible, pipeline-based, create new datasets
- **Hybrid Approach**: Filters apply to current step output, persist across subsequent steps

### 6. Live SQL Transparency
- Option to show generated SQL
- Helpful for learning SQL
- Debugging support
- Educational value

**SQL Generation:**
```
-- Step-based transforms
1. Source: SELECT * FROM data.csv
2. Filter: WHERE @Date >= '2024-01-01'
3. Add Column: Profit = @Revenue - @Cost
4. Group By: Region, SUM(Profit)

-- Stationary filters
WHERE Region IN ('North', 'South') 
  AND @Date BETWEEN '2024-01-01' AND '2024-12-31'
  AND @Amt > 0

-- Combined
WHERE (Region IN ('North', 'South')) 
  AND (@Date BETWEEN '2024-01-01' AND '2024-12-31')
  AND (@Amt > 0)
```

**SQL Generation:**
```
-- Step-based transforms
1. Source: SELECT * FROM data.csv
2. Filter: WHERE @Date >= '2024-01-01'
3. Add Column: Profit = @Revenue - @Cost
4. Group By: Region, SUM(Profit)

-- Stationary filters
WHERE Region IN ('North', 'South') 
  AND @Date BETWEEN '2024-01-01' AND '2024-12-31'
  AND @Amt > 0

-- Combined
WHERE (Region IN ('North', 'South')) 
  AND (@Date BETWEEN '2024-01-01' AND '2024-12-31')
  AND (@Amt > 0)
```

### 9. Column Management & Rendering
- **Soft-Width Columns**: Variable-width column sizing with line-break tolerance
- **Column Hats**: Dynamic semantic keys inferred from data patterns
- **Schema Persistence**: TOML-based schema storage for versionable transformations
- **Column-Level Transforms**: Vectorized formulas applied to entire columns
- **Replayable Pipeline**: Each transformation step can be replayed on new data

**Soft-Width Columns**:
- Every column has a "soft width" that determines how much space it displays/allocates
- Width is not rigid; it expands dynamically to fit content
- Line breaks inside a column don't break parsing — they're contained within the column
- Visually, this lets humans "read it like a table" without forcing strict alignment

**Column Hats**:
- Each column can acquire a semantic key at runtime — metadata about its identity
- Keys are attached dynamically based on inference, confidence, or user annotation
- Think: Vendor | Amount | Date → internally becomes Column { key: "vendor", type: text, ... }
- Keys allow the system to:
  - Reassign misaligned data if rows shift
  - Apply column-level formulas
  - Track history in a replayable pipeline
  - Validate new rows against expected type/distribution

**Replayable Pipeline**:
- Stepwise flow: Parse raw PSV → Split on | → Trim whitespace → Respect line breaks → Infer column hats → Analyze first N rows → Detect type, pattern, distribution → Assign keys → Dynamic alignment
- For incoming rows: Score each value against each column hat → Swap values if it improves fit → Column-level transforms only → Applied to the entire vector
- Each transform is a step: Can replay on new raw data → Version-controlled

## Technical Challenges

1. **Expression Parser**: Need robust parsing of `@field` syntax
2. **Cursor-Aware Autocomplete**: Context-sensitive completion in expression editor
3. **Step Graph State**: Maintain transformation pipeline state
4. **Grid Performance**: Handle large datasets in TUI
5. **Schema Evolution**: Handle schema changes across steps
6. **Filter State Management**: Track stationary filters independently from steps
7. **Filter UI Layout**: Implement draggable, resizable filter panels in TUI
8. **Filter Performance**: Apply filters efficiently without reprocessing entire datasets
9. **Soft-Width Column Rendering**: Calculate dynamic column widths based on content
10. **Column Hat Inference**: Implement heuristic-based key detection from data patterns
11. **Schema Persistence**: Store and load TOML-based schema definitions
12. **Replayable Pipeline**: Track transformation history for replayability

## Development Priorities

### Short Term (0-30 days)
1. Set up project structure with Rust + Ratatui + DuckDB
2. Implement basic file loading (CSV/Parquet)
3. Build simple grid renderer
4. Create schema introspection
5. Implement basic step management (add/delete)

### Medium Term (30-90 days)
1. DuckDB integration and SQL generation
2. Expression parser with `@field` support
3. Autocomplete implementation
4. Live SQL preview
5. Step reordering functionality
6. Performance optimization for larger datasets
7. Implement soft-width column rendering
8. Build column hat inference engine
9. Add schema persistence (TOML)
10. Implement replayable pipeline tracking

### Long Term (90+ days)
1. Advanced transforms (joins, pivots, custom SQL)
2. Richer Nushell integration
3. Multiple file format support (Excel, Parquet)
4. Export capabilities
5. Scripting capabilities via Nushell
6. Collaboration features
7. Richer visualization options

## Early Milestones (90-day runway)

| Milestone | Description | Output |
|-----------|-------------|--------|
| M1 | Data ingestion layer | Load Excel/CSV/Parquet → internal table struct |
| M2 | Step pipeline & AST | Filter/AddColumn/GroupBy steps, immutable chain |
| M3 | Embed Nushell | Run pipelines with inline @field expressions |
| M4 | Preview grid | Minimal TUI grid to show current step |
| M5 | Step sidebar | Add/remove/reorder steps; show step history |
| M6 | SQL export | Generate SQL from step pipeline (DuckDB backend) |
| M7 | MVP release | Crate published as nustage with basic CLI + TUI |

## Strategic Positioning

### Nustage = Liberation Layer
**Audience**:
- Terminal users who prefer CLI workflows
- Data engineers tired of Excel lock-in
- Accountants needing Power Query features
- DuckDB enthusiasts
- Developers building terminal-based tools

### Differentiators
- **Step-based**: Unlike Excel's drag-and-drop, Nustage uses immutable, composable steps
- **Previewable**: Every step is reversible and previewable
- **Reversible**: Easy to undo and modify transformations
- **Terminal-native**: Keyboard-driven, lightweight, scriptable
- **Versionable**: Text-based pipeline definitions
- **Composable**: Integrates with CLI ecosystem for heavier transforms

### Optional Later Growth
- WASM spreadsheet GUI integration for web
- Drag-and-drop support for accessibility
- Richer visualization options
- Cloud collaboration features
- Integration with popular data tools

## Success Metrics

### Usability
- Can users perform common spreadsheet operations without learning curve
- Keyboard-driven workflow is intuitive
- Expression syntax is easy to learn

### Performance
- Handle datasets that would choke Excel's Power Query
- Fast query execution with DuckDB
- Responsive TUI even with large datasets

### Extensibility
- Easy to add new transforms and integrations
- Open architecture for community contributions
- Clear extension points in the codebase

### Community
- Build momentum in terminal data tools ecosystem
- Active development and community engagement
- Documentation and examples for users

## Why This Matters

### The Gap
Power Query is powerful but locked inside Excel, which is bloated, Windows-centric, and difficult to script. No TUI equivalent exists that captures this value proposition.

### The Opportunity
Terminal users, data engineers, and developers have been asking for a Power Query experience in the terminal. Nustage fills this gap by combining:
- Power Query's step-based model
- Terminal-native UX (keyboard-driven, lightweight)
- Modern tech stack (Rust, DuckDB, Nushell)
- Composable and scriptable architecture

### The Impact
- Empowers users with sandboxed transform freedom
- Reduces vendor lock-in
- Provides a powerful alternative to Excel for data transformation
- Builds momentum in the terminal data tools ecosystem

## Next Steps

1. **Immediate (Week 1)**
   - Set up Rust project with Cargo
   - Add dependencies: ratatui, duckdb, calamine
   - Create basic project structure

2. **Short-term (Weeks 2-4)**
   - Implement file loading
   - Build simple grid renderer
   - Create schema introspection
   - Implement basic step management

3. **Medium-term (Weeks 5-12)**
   - Add DuckDB integration
   - Build expression parser
   - Implement autocomplete
   - Add live SQL generation
   - Polish TUI experience

4. **Long-term (Weeks 12+)**
   - Add advanced transforms
   - Enhance Nushell integration
   - Improve performance
   - Write documentation
   - Release MVP

## Community & Ecosystem

### Where the Community Lives
- **Nushell & CLI Data Tools**: Terminal-first data processing
- **DuckDB**: Embedded analytics database
- **VisiData**: Terminal spreadsheet UX reference
- **Ratatui**: Rust terminal UI framework
- **Calamine**: Rust data loading library

### How to Contribute
- Star the repository
- Share feedback and feature requests
- Submit bug reports
- Contribute code or documentation
- Share your use cases and success stories

## Conclusion

Nustage represents a bold step toward terminal-native data transformation. By combining Power Query's proven step model with modern Rust tools and terminal-first UX, we can create a powerful, accessible alternative to Excel for data manipulation and analysis.

The key is to stay focused on the core value proposition: empower users with sandboxed transform freedom without Excel's lock-in. Everything else can be built on top of this foundation.

Let's build something that changes how people work with data in the terminal! 🚀