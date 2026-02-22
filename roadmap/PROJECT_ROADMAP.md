# Spreadsheet TUI Project Roadmap

## Project Vision
Build a terminal-based spreadsheet application that captures Power Query's core value: a step-based transformation pipeline with schema-aware expressions, while leveraging modern tech stacks for better performance and extensibility.

## Core Philosophy
- **Step model over drag-and-drop**: Immutable, reorderable, deletable transformation steps
- **Field awareness**: Schema introspection with autocomplete
- **SQL transparency**: Show generated SQL for learning and debugging
- **Terminal-first UX**: Keyboard-driven, lightweight, scriptable

## Architecture

```
CodeTUI (Rust)
  ↓
Query AST builder
  ↓
SQL generator (DuckDB backend)
  ↓
DuckDB (embedded, columnar, fast)
  ↓
Tabular result
  ↓
Grid renderer in TUI
```

## Tech Stack

### Core Components
- **Backend Engine**: DuckDB (embedded, analytics-ready)
- **TUI Framework**: Ratatui (Rust terminal UI library)
- **Data Reading**: Calamine (Excel/CSV/Parquet support)
- **Shell Integration**: embed-nu (optional, for scripting)

### Optional Integrations
- VisiData (UX reference)
- Nushell (for advanced transforms and macros)

## MVP Requirements

### Phase 1: Minimal but Real
- [ ] Load CSV/Parquet files
- [ ] Display tabular data in grid
- [ ] Schema introspection with field sidebar
- [ ] Add transformation steps:
  - [ ] Filter
  - [ ] Add column
  - [ ] Select columns
  - [ ] Group by
- [ ] Live SQL generation display
- [ ] Expression editor with `@field` syntax support
- [ ] Basic autocomplete for fields

**Success Criteria**: Replaces 80% of common Excel use cases

## Key UX Features

### 1. Field Awareness
- Dropdown of available fields from current schema
- Schema introspection in state
- Autocomplete in expression editor

### 2. Column Expressions
- Support `@field` syntax (e.g., `@Revenue - @Cost`)
- Parse and convert to SQL identifiers
- Validate expressions against schema

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

## Technical Challenges

1. **Expression Parser**: Need robust parsing of `@field` syntax
2. **Cursor-Aware Autocomplete**: Context-sensitive completion in expression editor
3. **Step Graph State**: Maintain transformation pipeline state
4. **Grid Performance**: Handle large datasets in TUI

## Development Priorities

### Short Term
1. DuckDB integration and SQL generation
2. Basic grid rendering with Ratatui
3. Schema introspection and field sidebar
4. Step management (add/delete/reorder)

### Medium Term
1. Expression editor with `@field` support
2. Autocomplete implementation
3. Live SQL preview
4. Performance optimization for larger datasets

### Long Term
1. Advanced transforms (joins, pivots, custom SQL)
2. Scripting capabilities via Nushell
3. File format support (Excel, Parquet, multiple CSVs)
4. Collaboration features
5. Export capabilities

## Success Metrics

- **Usability**: Can users perform common spreadsheet operations without learning curve
- **Performance**: Handle datasets that would choke Excel's Power Query
- **Extensibility**: Easy to add new transforms and integrations
- **Community**: Build momentum in terminal data tools ecosystem

## Next Steps

1. Set up project structure with Rust + Ratatui + DuckDB
2. Implement basic grid and file loading
3. Build schema introspection
4. Create step management system
5. Implement expression parser with `@field` support
6. Add live SQL generation
7. Test with real-world datasets
8. Gather feedback and iterate