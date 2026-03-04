# Nustage
> **Piping commands over ranges, not formulas in cells.**

---

## What This Is

A terminal-native pipeline orchestration layer for tabular data. Not a spreadsheet. Not a grid editor. The thing that sits between your data and your understanding of it — making every transformation explicit, reversible, and reproducible.

Built because copy-paste-values-only from a CSV export is rational behavior given current alternatives. Nustage makes the step-based model accessible enough that an accounting colleague would actually use it.

---

## What This Is Not

- Not a grid viewer — use Tabiew for that
- Not a formula editor — pipes over ranges, not cells
- Not Excel with a terminal skin
- Not a Nushell wrapper (yet)
- Not a collaboration tool (yet)

---

## The Core Insight

Power Query is honest engineering that almost nobody uses because the abstraction gap is too wide. Excel versioning is recoverable but not readable — snapshots, not intent. The formula in F7 that references a sheet from 2019 that someone renamed is the failure mode of cell-based computation.

The step list is the product. Everything else serves it.

---

## Architecture

```
Data Source (CSV / Parquet / Excel)
  ↓
Schema Inference
  ↓
Step Pipeline (immutable, ordered, named)
  ↓
DuckDB Execution Engine
  ↓
Tabular Result
  ↓
Witness Layer (TUI — pipeline state, not pixel-perfect grid)
  ↓
Sidecar (.nustage.json — versionable, diffable, readable)
```

### Layers and Responsibilities

| Layer | Tool | Responsibility |
|-------|------|----------------|
| Grid viewing | Tabiew | Raw data display, navigation |
| Pipeline orchestration | Nustage | Steps, schema, transforms |
| Execution | DuckDB | SQL generation, query engine |
| Scripting (future) | Nushell | Complex expressions, macros |
| Version control | Git + sidecar | Diff pipeline definitions |

---

## The Step Model

Each transformation is:
- **Named** — readable by a human
- **Immutable** — applying a step never mutates the source
- **Ordered** — steps form a reproducible chain
- **Reversible** — delete or reorder without starting over
- **Transparent** — generates visible SQL for learning and debugging

```
1. Source: sales.csv
2. Filter: Revenue > 1000
3. Add Column: Margin = Revenue - Cost
4. Group By: Region | Sum(Margin)
5. Sort: Margin descending
```

This is the whole product. The TUI makes it visible. DuckDB makes it fast. The sidecar makes it versionable.

---

## Expression Syntax

Pipes over ranges. Field references, not cell addresses.

```nu
@Revenue - @Cost
@Date >= 2024-01-01
@Region in ["North", "South"]
@data | group-by Region | sum Revenue
```

No `=SUM(F7:F23)`. No cell coordinates. No silent breakage when someone inserts a column.

---

## The Sidecar

Lives alongside the data file. Plain JSON. Diffs cleanly in git.

```json
{
  "version": 1,
  "source": "sales.csv",
  "pipeline": [
    { "step": "filter", "column": "Revenue", "condition": "> 1000" },
    { "step": "add_column", "name": "Margin", "expr": "@Revenue - @Cost" },
    { "step": "group_by", "columns": ["Region"], "agg": [{"col": "Margin", "op": "sum"}] }
  ],
  "schema": {
    "Revenue": "f64",
    "Cost": "f64",
    "Region": "str",
    "Date": "date"
  }
}
```

The sidecar is the artifact. The CSV is just the data.

---

## Domain Advantages

Built by a cost accountant in manufacturing. The problems Nustage solves are real and daily:

- **BOM structures** — hierarchical, box in box in box, maps cleanly to recursive pipeline steps
- **Standard vs actual variance** — two-range comparison as a named pipeline, not a copy-paste ritual  
- **Period-end allocations** — reproducible, auditable, not locked in one person's head
- **Schema drift** — when columns change, the sidecar tells you exactly what broke and where

The target user is not a data engineer. It is the person who currently exports to CSV, opens in Excel, copies, pastes values only, and formats manually — because that is the rational path given current tools.

---

## The Witness Layer (TUI)

The TUI is not the product. It is the feedback loop that makes the pipeline tangible.

Minimum viable witness:
- **Step list** — left panel, named steps, current active step highlighted
- **Schema panel** — column names and types at current step
- **Preview** — row count, shape, sample output
- **SQL transparency** — generated query visible on demand
- **Status bar** — rows in / rows out, active filters

The grid is Tabiew's job. Nustage shows pipeline state.

---

## Tech Stack

| Concern | Crate | Why |
|---------|-------|-----|
| Data processing | Polars | Columnar, fast, Rust-native |
| Query engine | DuckDB | Embedded SQL, no server |
| Excel I/O | IronCalc + Calamine | Read/write .xlsx natively |
| TUI | Ratatui + Crossterm | Proven, keyboard-driven |
| CLI | Clap | Standard, derive-based |
| Serialization | Serde + serde_json | Sidecar format |
| Error handling | Thiserror + Anyhow | Clean propagation |

---

## What To Build Next (Prioritized)

1. **Real cell rendering in tui_grid.rs** — actual data, not placeholders
2. **Fix Model lifetime in ironcalc/mod.rs** — `'static` is wrong, scope it properly  
3. **Real Excel loader in data/mod.rs** — replace empty DataFrame placeholder
4. **Step list panel in TUI** — the thing that makes Nustage distinct from Tabiew
5. **Sidecar read/write** — serialize/deserialize pipeline to `.nustage.json`
6. **SQL transparency** — show generated DuckDB query per step

Everything else is future.

---

## What To Ignore For Now

- Nushell integration — aspirational, not load-bearing yet
- Collaboration features — not the problem
- WASM / web frontend — not the problem
- Charts and visualization — not the problem
- Custom expression language — DuckDB SQL is sufficient for MVP

---

## North Star

A cost accountant can open a CSV, apply five named transformations, save the sidecar to git, share it with a colleague, and that colleague can replay the exact same pipeline on next month's data without asking anyone how it works.