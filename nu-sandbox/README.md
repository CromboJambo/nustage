# Nushell Dashboard Sandbox

A self-contained, runnable tutorial sandbox that teaches you to script sensible default dashboard views from Nushell's structured data pipeline.

## What This Is

This sandbox has two jobs running simultaneously:

**Job 1 — Learning tool.** You get hands-on with Nushell's structured data pipeline, building toward a live-reloading dashboard you actually use.

**Job 2 — Design probe for Nustage.** You're also the author of Nustage — a terminal-native pipeline orchestration layer for tabular data (Rust, DuckDB, Polars, Ratatui). This sandbox helps us figure out what Nushell integration should feel like before writing a line of Rust.

The key crate for the eventual integration is `embed-nu` — [github.com/nustage/embed-nu](https://github.com/nustage/embed-nu) — which allows embedding a Nushell runtime into a Rust application. This sandbox is the prerequisite: you can't design that API well without first knowing what Nushell pipelines feel like to write.

## Quick Start

```bash
# Install Nushell (if not already installed)
brew install nushell  # macOS
winget install nushell # Windows
cargo install nu       # Linux/macOS via cargo

# Install glow for terminal preview
brew install glow

# Navigate to the sandbox
cd nu-sandbox

# Run the lessons in order
nu lessons/01_tables.nu
nu lessons/02_pipelines.nu
nu lessons/03_to_md.nu
nu lessons/04_dashboard.nu
nu lessons/05_watch.nu

# Or run the final dashboard
nu scripts/dashboard.nu
```

## Prerequisites

- **Nushell** — shell with structured data pipeline ([install guide](https://www.nushell.sh/book/installation))
- **glow** — terminal markdown renderer ([install guide](https://github.com/charmbracelet/glow))
- **Zed** (optional) — IDE with built-in terminal and .md preview
- **Zellij** (optional) — multiplexer for split-pane layouts
- **WezTerm** (optional) — standalone terminal emulator

## Project Structure

```
nu-sandbox/
├── README.md              # This file
├── lessons/
│   ├── 01_tables.nu       # Structured data basics
│   ├── 02_pipelines.nu    # Filtering, sorting, shaping
│   ├── 03_to_md.nu        # Exporting to markdown tables
│   ├── 04_dashboard.nu    # Composing a multi-section .md dashboard
│   └── 05_watch.nu        # Live-reloading with watch + glow
├── outputs/
│   └── .gitkeep           # Where generated .md files land
└── scripts/
    └── dashboard.nu       # Final working dashboard script
```

## Learning Path

### Lesson 1: Tables (`01_tables.nu`)
- Learn that `ls`, `ps`, `sys`, and `help commands` are all tables
- Understand `| get`, `| select`, and `| where` basics
- **Goal:** You understand everything is a table

### Lesson 2: Pipelines (`02_pipelines.nu`)
- Master `| sort-by`, `| reverse`, `| first N`, `| last N`
- Use `| group-by` and `| length`
- Introduce `| explore` for interactive drill-down
- **Goal:** You can shape any table into a useful slice

### Lesson 3: To Markdown (`03_to_md.nu`)
- See `| to md` in action
- Learn `| to csv`, `| to json` as sidebars
- Use string interpolation for context
- Save outputs to `.md` files
- **Goal:** You can write any table to a renderable .md file

### Lesson 4: Dashboard Composition (`04_dashboard.nu`)
- Compose multiple pipeline outputs into one .md document
- Use `let` bindings for named sections
- Multi-section string interpolation pattern
- **Goal:** Produce a multi-section markdown dashboard

### Lesson 5: Live Preview Loop (`05_watch.nu`)
- Set up `watch . { nu scripts/dashboard.nu }` for auto-regeneration
- Pair with `glow outputs/dashboard.md` in a split pane
- Open in Zed for editor-side preview
- **Goal:** A working live-reloading dashboard loop

### The Unlock: `scripts/dashboard.nu`
The final script you build toward. It includes sensible defaults:
- **System** — CPU, memory, uptime
- **Disk** — usage by mount
- **Processes** — top 10 by CPU
- **Current directory** — `ls` sorted by modified date
- **Git status** — branch + dirty files (if in a repo)

## Design Probe Questions

As you work through this sandbox, these questions are implicitly answered for Nustage design:

- What does `| where` feel like vs Nustage's `FilterRows(column, condition)` — is the string-based condition model the right abstraction?
- Does `| group-by` + `| math sum` map cleanly to Nustage's `GroupBy(columns, aggregations)` step model?
- What does `| to md` output that Nustage's sidecar/output layer should replicate or defer to?
- Where does Nushell's pipeline model break down for the kinds of tabular transforms Nustage targets (BOM hierarchies, variance tracking)?
- What would `nustage` need to expose as a Nushell custom command for the integration to feel native?

## Running the Lessons

Each lesson is a `.nu` file with **inline comments as the tutorial text**. Just run it:

```bash
nu lessons/01_tables.nu
```

The comments explain what's happening, and you can modify and rerun to experiment.

## Constraints

- **Nushell stdlib only** — no plugins, no third-party nu scripts. Everything works with a fresh `nu` install.
- **Comments are the tutorial** — no separate markdown lesson files. The `.nu` files are self-documenting.
- **Real data sources only** — no mock/fake data. Everything pulls from your actual system.
- **Each lesson is runnable in under 5 seconds** — no slow operations, no network calls.
- **Lessons are additive** — each builds on the last, but you can jump in anywhere.

## Stretch / Bonus

If you have time, check out these optional enhancements:

- A `Justfile` or `run.nu` entrypoint that runs all lessons in sequence
- A Zellij layout file (`.kdl`) for pre-configured split panes
- A `07_custom.nu` lesson stub for your own views
- A `nustage_probe_summary.nu` that formats the design translation into a clean spec

## References

- [Nushell Book](https://www.nushell.sh/book)
- [Nushell Commands Reference](https://www.nushell.sh/commands)
- [Glow](https://github.com/charmbracelet/glow)
- [Zellij Layouts](https://zellij.dev/documentation/layouts)
- [WezTerm Lua Config](https://wezfurlong.org/wezterm)
- [Nustage Repo](https://github.com/CromboJambo/nustage)
- [embed-nu](https://github.com/nustage/embed-nu)
- [Nustage StepType Enum](https://github.com/CromboJambo/nustage/blob/main/src/transformations/mod.rs)
- [Nustage Power Query Example](https://github.com/CromboJambo/nustage/blob/main/examples/power_query_workflow.rs)

## License

Same as Nustage — see [LICENSE](../../LICENSE) for details.