# Nustage Web Interface & PowerQuery Integration Roadmap

## Vision Statement

Build a companion tool for Excel that helps users build structured transformations through a visual pipeline builder, which can then be exported as M code for PowerQuery — without ever requiring users to leave their data files or learn a new interface.

---

## Core Philosophy

1. **Companion, Not Replacement** — Nustage doesn't replace Excel; it augments the transformation workflow
2. **Preserve Original File** — The source file is never modified; all changes live in sidecar files
3. **Human-Readable Diffs** — Changes are visible as plain text before they're applied
4. **Export to M Code** — Advanced users can copy-paste into Excel's PowerQuery Editor

---

## Phase 1: Foundation (Weeks 1-2)

### Goals
- Set up WASM build pipeline
- Create basic web interface structure
- Implement file upload from browser

### Tasks

#### 1.1 WASM Build Setup
- [ ] Configure `Cargo.toml` with WASM dependencies (`wasm-bindgen`, `web-sys`)
- [ ] Add crate type `cdylib` for WASM compilation
- [ ] Set up wasm-pack build scripts
- [ ] Create build script to generate `nustage.js` and `nustage_bg.wasm`

#### 1.2 Web Interface Structure
- [ ] Create HTML template (`web/index.html`)
- [ ] Add CSS styles for dark theme UI (`web/styles.css`)
- [ ] Set up basic page layout with sections:
  - File upload area
  - Pipeline builder panel
  - Preview/output panels

#### 1.3 File Upload
- [ ] Implement drag-and-drop file upload zone
- [ ] Add file type validation (CSV, Excel, Parquet)
- [ ] Display file metadata (name, size, type)
- [ ] Store uploaded file reference for processing

### Deliverables
- Working WASM build pipeline
- Basic web UI with file upload
- Can display uploaded file info

---

## Phase 2: Pipeline Builder (Weeks 3-4)

### Goals
- Implement transformation step types
- Create dynamic form generation
- Build sidecar file management

### Tasks

#### 2.1 Step Type Definitions
Define the following transformation steps in `src/mcode/mod.rs`:
- [ ] **Filter Rows** — Filter based on condition (`Revenue > 1000`)
- [ ] **Select Columns** — Choose which columns to keep
- [ ] **Add Calculated Column** — Formula-based column (@Revenue - @Cost)
- [ ] **Group & Aggregate** — Group by with sum/avg/count operations
- [ ] **Sort** — Sort by column ascending/descending
- [ ] **Rename Column** — Change column names
- [ ] **Drop Columns** — Remove unwanted columns

#### 2.2 Dynamic Form Generation
For each step type, create an input form:
- [ ] Filter Rows → Select column dropdown + condition input
- [ ] Add Column → Name field + expression textarea
- [ ] Group By → Columns list + aggregation selectors
- [ ] Sort → Column selector + order radio buttons

#### 2.3 Sidecar File Management
Implement `src/sidecar/mod.rs`:
- [ ] Create new sidecar file for uploaded data
- [ ] Add step to pipeline (append to array)
- [ ] Remove step from pipeline (by ID)
- [ ] Serialize/deserialize sidecar JSON
- [ ] Generate human-readable diff between schemas

#### 2.4 WASM Bindings
Export functions to JavaScript:
```rust
#[wasm_bindgen]
pub fn create_sidecar(source_path: &str) -> Result<String, JsValue>

#[wasm_bindgen]
pub fn add_step_to_sidecar(
    sidecar_json: &str,
    name: &str,
    step_type: &str,
    params_json: &str
) -> Result<String, JsValue>

#[wasm_bindgen]
pub fn remove_step_from_sidecar(
    sidecar_json: &str,
    step_id: &str
) -> Result<String, JsValue>
```

### Deliverables
- Complete pipeline builder UI
- Sidecar file creation and management
- Can add/remove transformation steps

---

## Phase 3: M Code Export (Weeks 5-6)

### Goals
- Convert Nustage steps to PowerQuery M language
- Generate complete, copy-paste-ready code
- Add syntax highlighting for preview

### Tasks

#### 3.1 M Code Generator
Implement `src/mcode/mod.rs`:

```rust
pub fn generate_m_code(pipeline: &[TransformationStep], source_name: &str) -> String
```

Convert each step type:
- [ ] Filter → `Table.SelectRows(Source, each ...)`
- [ ] Select Columns → `Table.SelectColumns(Source, {...})`
- [ ] Add Column → `Table.AddColumn(Source, "Name", each ...)`
- [ ] Group By → `Table.Group(Source, {columns}, {{name, each List.Sum(...)}})`
- [ ] Sort → `Table.Sort(Source, {{column, Order.Ascending}})`

#### 3.2 Expression Translation
Convert Nustage expressions to M syntax:
- [ ] Replace `@ColumnName` with `[ColumnName]`
- [ ] Handle arithmetic operations (`+`, `-`, `*`, `/`)
- [ ] Translate comparison operators (`>`, `<`, `=`, etc.)

#### 3.3 Complete Pipeline Generation
Generate full M code structure:
```powerquery-m
let
    Source = Excel.CurrentWorkbook(){[Name="Table1"]}[Content],
    FilteredRows = Table.SelectRows(Source, each [Revenue] > 1000),
    GroupedRows = Table.Group(FilteredRows, {"Region"}, {{...}})
in
    GroupedRows
```

#### 3.4 Export UI
- [ ] Display M code in `<pre>` block with monospace font
- [ ] Add syntax highlighting (keywords purple, functions blue, strings green)
- [ ] "Copy to Clipboard" button with feedback
- [ ] Download as `.pq` file option

### Deliverables
- Working M code generator
- Syntax-highlighted preview
- Copy/download functionality

---

## Phase 4: Preview & Diff (Weeks 7-8)

### Goals
- Show transformation results before export
- Generate human-readable schema diffs
- Display changes in plain text format

### Tasks

#### 4.1 Schema Comparison
Implement `src/sidecar/mod.rs`:
```rust
pub fn generate_diff(
    original_schema: &[ColumnSchema],
    transformed_schema: &[ColumnSchema]
) -> String
```

Show:
- [ ] Added columns (`+ ColumnName`)
- [ ] Removed columns (`- ColumnName`)
- [ ] Type changes (`~ ColumnName: int → float`)

#### 4.2 Preview Panel
Create `web/preview.html` (or inline panel):
- [ ] Display row count after transformations
- [ ] Show sample of transformed data (first 10 rows)
- [ ] Render as table with headers
- [ ] Pagination for large datasets

#### 4.3 Sidecar Content View
- [ ] Load existing `.nustage.json` file
- [ ] Display pipeline steps in readable format
- [ ] Allow editing step parameters inline
- [ ] Save changes back to sidecar file

### Deliverables
- Schema diff viewer
- Data preview panel
- Sidecar file editor

---

## Phase 5: Polish & Optimization (Weeks 9-10)

### Goals
- Improve UX and error handling
- Optimize WASM build size
- Add accessibility features

### Tasks

#### 5.1 Error Handling
- [ ] Show user-friendly error messages for invalid inputs
- [ ] Graceful handling of file read errors
- [ ] Validation before step submission
- [ ] Toast notifications for success/failure states

#### 5.2 Performance Optimization
- [ ] Minimize WASM binary size with `opt-level = "z"`
- [ ] Lazy load large datasets (pagination)
- [ ] Debounce input fields to reduce re-renders
- [ ] Cache computed values where possible

#### 5.3 Accessibility
- [ ] Add ARIA labels for screen readers
- [ ] Keyboard navigation support
- [ ] Color contrast compliance (WCAG AA)
- [ ] Focus indicators for all interactive elements

#### 5.4 Testing
- [ ] Unit tests for M code generator
- [ ] Integration tests for WASM bindings
- [ ] Manual testing with various file types
- [ ] Cross-browser compatibility check

### Deliverables
- Production-ready web interface
- Optimized WASM build
- Comprehensive documentation

---

## Technical Architecture

### File Structure

```
nustage/
├── Cargo.toml                          # Updated with WASM dependencies
├── src/
│   ├── lib.rs                        # Library entry point
│   ├── wasm.rs                       # WASM bindings (NEW)
│   ├── mcode/
│   │   └── mod.rs                    # PowerQuery M generator (NEW)
│   ├── sidecar/
│   │   └── mod.rs                    # Sidecar file handling (NEW)
│   ├── data/                         # Data loading
│   ├── transformations/              # Pipeline steps
│   └── tui.rs                        # Terminal UI (existing)
├── web/                              # Web interface (NEW)
│   ├── index.html                    # Main page template
│   ├── styles.css                    # Dark theme CSS
│   └── app.js                        # Frontend JavaScript logic
└── target/
    └── wasm32-unknown-unknown/       # WASM build output
        ├── nustage.js                # JS glue code
        └── nustage_bg.wasm           # Compiled WASM binary
```

### Data Flow

1. **User uploads file** → Stored in browser memory (File API)
2. **Create sidecar** → `src/sidecar::SidecarFile::new(filename)`
3. **Add transformation** → User selects step type, fills form → `add_step_to_sidecar()`
4. **Update UI** → Render pipeline steps list
5. **Preview results** → `generate_diff(original_schema, transformed_schema)`
6. **Export M code** → `generate_m_code(pipeline_steps, "Source")`
7. **Copy to clipboard** → Browser Clipboard API

### WASM Function Map

| JavaScript Call | Rust Function | Description |
|-----------------|---------------|-------------|
| `get_version()` | `wasm::get_version()` | Returns Nustage version string |
| `create_sidecar(path)` | `wasm::create_sidecar()` | Creates new sidecar for file |
| `load_sidecar(path)` | `wasm::load_sidecar()` | Loads existing .nustage.json |
| `save_sidecar_json(json)` | `wasm::save_sidecar_json()` | Saves sidecar to disk (if supported) |
| `add_step_to_sidecar(...)` | `wasm::add_step_to_sidecar()` | Appends step to pipeline |
| `remove_step_from_sidecar(id)` | `wasm::remove_step_from_sidecar()` | Removes step by ID |
| `export_as_m_code(json, source)` | `wasm::export_as_m_code()` | Generates PowerQuery M code |
| `generate_diff_json(orig, trans)` | `wasm::generate_diff_json()` | Schema comparison text |

---

## Success Criteria

### Minimum Viable Product (MVP)
- [ ] Can upload CSV/Excel file from browser
- [ ] Add at least 3 transformation steps (filter, select, group)
- [ ] Export valid M code that works in Excel PowerQuery Editor
- [ ] Sidecar file is created and viewable

### Full Release
- [ ] All 7 step types implemented
- [ ] Human-readable schema diffs
- [ ] Data preview panel with pagination
- [ ] Syntax-highlighted M code output
- [ ] Copy/download functionality
- [ ] Error handling for all edge cases
- [ ] Works in Chrome, Firefox, Safari (latest)

---

## Dependencies

### Rust/WASM
```toml
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["console"] }
serde-wasm-bindgen = "0.6"
```

### Build Tools
- `wasm-pack` — Rust to WASM compiler
- `esbuild` or `vite` — JavaScript bundler (optional)
- `prettier` — Code formatting

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Excel file parsing in browser is slow | Poor UX for large files | Use streaming API, show progress indicator |
| Complex M code generation errors | Invalid output | Extensive testing with known patterns |
| WASM binary too large (>5MB) | Slow initial load | Optimize crate features, use `opt-level = "z"` |
| Browser compatibility issues | Limited user base | Test on major browsers, add polyfills if needed |

---

## Next Steps After Launch

### Phase 6: Advanced Features (Weeks 11-12)
- [ ] Direct Excel integration via browser extension
- [ ] Cloud sync for sidecar files (optional)
- [ ] Collaborative editing of pipelines
- [ ] Custom expression builder with autocomplete

### Phase 7: Desktop App (Optional, Week 13+)
- [ ] Electron/Tauri wrapper for offline desktop use
- [ ] Native file system access
- [ ] Background processing for large files

---

## Conclusion

This roadmap transforms Nustage from a terminal-only tool into a **companion application** that makes PowerQuery accessible to users who find the Excel UI intimidating. By focusing on human-readable diffs, explicit transformations, and clean M code export, we can help more people do better data work without abandoning their familiar tools.