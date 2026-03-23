# Nushell Dashboard Sandbox — Build Spec

## What This Actually Is

This sandbox has two jobs running simultaneously:

**Job 1 — Learning tool.** The learner gets hands-on with Nushell's structured data pipeline, building toward a live-reloading dashboard they actually use.

**Job 2 — Design probe for Nustage.** The learner is also the author of [Nustage](https://github.com/CromboJambo/nustage) — a terminal-native pipeline orchestration layer for tabular data (Rust, DuckDB, Polars, Ratatui). Nushell integration is item #9 on the Nustage aspirational roadmap. This sandbox is how we figure out what that integration should actually feel like before writing a line of Rust.

**The thesis:** If you can script sensible default views in Nushell, you'll know exactly what Nustage's `StepType` enum needs to support, what the `@field` expression syntax should feel like from a user's perspective, and how `embed-nu` should be wired into the Nustage execution pipeline.

```
Nushell sandbox (learn to script structured data)
    ↓
Understand how pipelines, | to md, and explore behave
    ↓
Design the Nushell scripting layer for Nustage
    ↓
Wire embed-nu into Nustage as the expression engine
    ↓
nustage process sales.csv | filter Revenue > 1000 | to md
```

The key crate for the eventual integration is `embed-nu` — `github.com/nustage/embed-nu` — which allows embedding a Nushell runtime into a Rust application. The sandbox is the prerequisite: you can't design that API well without first knowing what Nushell pipelines feel like to write.

### What the Agent Should Watch For

As the sandbox is built and used, the following questions are implicitly being answered for Nustage design. The agent should note these as open design questions in the README:

- What does `| where` feel like vs Nustage's `FilterRows(column, condition)` — is the string-based condition model the right abstraction?
- Does `| group-by` + `| math sum` map cleanly to Nustage's `GroupBy(columns, aggregations)` step model?
- What does `| to md` output that Nustage's sidecar/output layer should replicate or defer to?
- Where does Nushell's pipeline model break down for the kinds of tabular transforms Nustage targets (BOM hierarchies, variance tracking)?
- What would `nustage` need to expose as a Nushell custom command for the integration to feel native?

These don't need answers yet — just flagging them as the meta-layer of the exercise.

---

## Context & Stack

The learner is coming from:
- **Zed** as primary IDE (built-in terminal, native .md preview)
- **WezTerm** as standalone terminal emulator
- **Zellij** for multiplexing / floating panes
- **Nushell** as shell (assumed installed, no prior scripting experience)
- Comfortable with structured thinking, Lua (Assetto Corsa), general programming intuition
- Not a terminal power user — this should feel approachable, not arcane

---

## Goal

A self-contained, runnable tutorial sandbox that teaches the learner to script **sensible default dashboard views** from Nushell's structured data pipeline — outputting to both live terminal views and `.md` files renderable in Zed or `glow`. Every lesson is also implicitly a usability test of the pipeline model that Nustage is being built to productize.

---

## What to Build

### 1. Sandbox Directory Structure

```
nu-sandbox/
├── README.md              # How to run the sandbox, prereqs
├── lessons/
│   ├── 01_tables.nu       # Structured data basics
│   ├── 02_pipelines.nu    # Filtering, sorting, shaping
│   ├── 03_to_md.nu        # Exporting to markdown tables
│   ├── 04_dashboard.nu    # Composing a multi-section .md dashboard
│   └── 05_watch.nu        # Live-reloading with watch + glow
├── outputs/
│   └── .gitkeep           # Where generated .md files land
└── scripts/
    └── dashboard.nu       # Final working dashboard script to unlock
```

---

### 2. Lesson Briefs

Each lesson is a `.nu` file with **inline comments as the tutorial text**. Learner runs it, reads the comments, modifies it, reruns. No separate docs needed.

**01 — Tables**
- `ls`, `ps`, `sys`, `help commands` as data sources
- `| get`, `| select`, `| where` basics
- Goal: learner understands everything is a table

**02 — Pipelines**
- `| sort-by`, `| reverse`, `| first N`, `| last N`
- `| group-by`, `| length`
- Introduce `| explore` as interactive drill-down
- Goal: learner can shape any table into a useful slice

**03 — To Markdown**
- `| to md` — show it just works
- `| to csv`, `| to json` as sidebars
- String interpolation `$"## Header\n($table)"` for wrapping tables in context
- `| save outputs/example.md`
- Goal: learner can write any table to a renderable .md file

**04 — Dashboard Composition**
- Composing multiple pipeline outputs into one .md document
- `let` bindings for named sections
- Multi-section string interpolation pattern
- Goal: produce a `outputs/dashboard.md` with 3+ sections from real system data

**05 — Live Preview Loop**
- `watch . { nu scripts/dashboard.nu }` — regenerate on any save
- Pair with `glow outputs/dashboard.md` in a Zellij split or floating pane
- Optionally: open `outputs/dashboard.md` in Zed for the eyeball preview
- Goal: learner has a working live-reloading dashboard loop

**06 — Design Probe (Nustage Bridge Lesson)**
- Load one of Nustage's test CSVs (`sales.csv`, `expenses.csv`) into Nushell
- Replicate the Nustage `power_query_workflow.rs` example entirely in Nushell:
  - `open test_data/sales.csv | where Revenue > 1000 | group-by Region | math sum`
- Compare Nushell pipeline syntax to Nustage's `StepType` enum — note friction and gaps in comments
- Try to express a `SortBy`, `AddColumn`, and `GroupBy+Aggregation` step in Nushell idiom
- Output: `outputs/nustage_probe.md` — a translation table of "Nustage step → Nushell equivalent"
- Goal: produce a concrete mapping the agent hands back as a Nustage design artifact

This is the meta-output of the whole sandbox. It answers: *what should the `embed-nu` integration layer actually expose?*

---

### 3. The Unlock — `scripts/dashboard.nu`

The final script the learner builds toward. Should include sensible defaults:

- **System** — CPU, memory, uptime (`sys`)
- **Disk** — usage by mount (`sys disks`)
- **Processes** — top 10 by CPU
- **Current directory** — `ls` sorted by modified date
- **Git status** — if in a repo, surface branch + dirty files (use `^git` passthrough)

Output: `outputs/dashboard.md` — a clean multi-section markdown file with tables.

---

### 4. Prereqs the README Should Cover

- Nushell installed (`brew install nushell` / `winget` / `cargo install nu`)
- `glow` installed for terminal preview (`brew install glow`) — stub: [charmbracelet/glow](https://github.com/charmbracelet/glow)
- Zellij for split-pane live view (optional but recommended)
- Zed with the `.md` preview for the editor-side view
- No external Nushell plugins required — stdlib only

---

## Constraints & Guardrails for the Agent

- **Nushell stdlib only** — no plugins, no third-party nu scripts. Learner should be able to run everything with a fresh `nu` install.
- **Comments are the tutorial** — no separate markdown lesson files. The `.nu` files are self-documenting.
- **Real data sources only** — no mock/fake data. Everything pulls from the actual system so output is immediately meaningful to the learner.
- **Each lesson should be runnable in under 5 seconds** — no slow operations, no network calls.
- **Lessons are additive** — each one builds on the last, learner can run them in order or jump in anywhere.
- **The dashboard script should be idempotent** — safe to run repeatedly, always overwrites `outputs/dashboard.md`.

---

## Stretch / Bonus (Only If Time Allows)

- A `Justfile` or `run.nu` entrypoint that runs all lessons in sequence
- A Zellij layout file (`.kdl`) that opens the sandbox in a pre-configured split: editor left, `glow` preview right
- A `07_custom.nu` lesson stub with blank sections for the learner to fill in their own views
- A `scripts/nustage_probe_summary.nu` that formats the lesson 06 translation table into a clean design doc and saves it to `outputs/nustage_integration_design.md` — ready to drop directly into the Nustage repo as a spec for the Nushell layer

---

## References (Stubs — Agent Can Pull as Needed)

- Nushell book: `nushell.sh/book`
- Nushell stdlib command reference: `nushell.sh/commands`
- Glow: `github.com/charmbracelet/glow`
- Zellij layouts: `zellij.dev/documentation/layouts`
- WezTerm Lua config: `wezfurlong.org/wezterm`
- Nustage repo: `github.com/CromboJambo/nustage`
- embed-nu (Nushell embedded runtime for Rust): `github.com/nustage/embed-nu`
- Nustage `StepType` enum (for lesson 06 reference): `src/transformations/mod.rs` in the Nustage repo
- Nustage power query workflow example (lesson 06 replication target): `examples/power_query_workflow.rs`
