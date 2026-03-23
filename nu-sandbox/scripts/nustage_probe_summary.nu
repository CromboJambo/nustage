# NUSTAGE INTEGRATION DESIGN SPEC
# This script formats the Nushell-to-Nustage translation table into a clean design document
# Run this: nu scripts/nustage_probe_summary.nu

# === TRANSLATION TABLE ===
# This documents how Nushell pipeline operations map to Nustage's StepType enum
# and helps inform the design of the Nushell integration layer

let nustage_integration_design = $"# NUSTAGE - NUSHELL INTEGRATION DESIGN
#
# This document captures the translation between Nushell's pipeline model
# and Nustage's Rust-based pipeline orchestration layer.
#
# Generated from: nu-sandbox lessons
# Last Updated: (date now)
#
# --- DESIGN PROBE QUESTIONS ANSWERED ---
#
# 1. What does `| where` feel like vs Nustage's `FilterRows(column, condition)` —
#    is the string-based condition model the right abstraction?
#
#    ANSWER: Nushell's `where` uses a Nushell expression language for conditions,
#    which is more flexible but requires runtime evaluation. The string-based model
#    would be simpler but less powerful. The current Nushell approach feels more
#    natural for interactive scripting, but Nustage might want to support both.
#
# 2. Does `| group-by` + `| math sum` map cleanly to Nustage's `GroupBy(columns, aggregations)`
#    step model?
#
#    ANSWER: Yes, the mapping is very clean. Nushell's `group-by` produces a table of groups,
#    and subsequent aggregation commands can be chained. Nustage's model should support
#    this chainable pattern.
#
# 3. What does `| to md` output that Nustage's sidecar/output layer should replicate
#    or defer to?
#
#    ANSWER: `to md` produces clean, formatted markdown tables with headers. Nustage's
#    output layer should replicate this for terminal rendering, but could also defer to
#    external renderers for richer formatting.
#
# 4. Where does Nushell's pipeline model break down for the kinds of tabular transforms
#    Nustage targets (BOM hierarchies, variance tracking)?
#
#    ANSWER: Nushell struggles with hierarchical data structures and complex variance
#    calculations that require multiple passes over the data. Nustage's Rust layer would
#    need to provide specialized steps for these cases.
#
# 5. What would `nustage` need to expose as a Nushell custom command for the integration
#    to feel native?
#
#    ANSWER: Custom commands like `nustage load`, `nustage filter`, `nustage group`,
#    `nustage aggregate`, and `nustage export` would make the integration feel native.
#
# --- TRANSLATION TABLE ---
#
# NUSTAGE STEP TYPE → NUSHELL PIPELINE EQUIVALENT
#
# `LoadCSV(path)` → `open path`
# `FilterRows(column, condition)` → `open table | where column condition`
# `SortBy(column, direction)` → `open table | sort-by column -r`
# `SelectColumns(columns)` → `open table | select columns`
# `AddColumn(name, expression)` → `open table | update name { expression }`
# `RenameColumn(old, new)` → `open table | rename old new`
# `DropColumn(column)` → `open table | reject column`
# `GroupBy(columns, aggregations)` → `open table | group-by columns | each { ... }`
# `Aggregation.Sum(column)` → `open table | math sum column`
# `Aggregation.Average(column)` → `open table | math avg column`
# `Aggregation.Count()` → `open table | length`
# `ExportToCSV(path)` → `open table | to csv | save path`
# `ExportToMarkdown(path)` → `open table | to md | save path`
# `ExportToJSON(path)` → `open table | to json | save path`
#
# --- KEY FINDINGS ---
#
# 1. **String-based conditions are not ideal** - Nushell's expression language is more powerful
#    but requires runtime evaluation. Nustage should support both string and expression models.
#
# 2. **Chainable aggregation is natural** - Nushell's pipeline model works well with the
#    step-by-step approach of Nustage's transformation pipeline.
#
# 3. **Output format matters** - `to md` produces high-quality markdown that Nustage should
#    replicate for terminal rendering.
#
# 4. **Hierarchical data is a gap** - Nushell's table model is flat; Nustage will need
#    specialized steps for BOM hierarchies and variance tracking.
#
# 5. **Custom commands would feel native** - Nustage should expose custom commands that
#    mirror the pipeline operations.
#
# --- RECOMMENDATIONS ---
#
# 1. Support both string and expression-based conditions for flexibility
# 2. Implement chainable aggregation steps that match Nushell's pipeline
# 3. Replicate `to md` output format for terminal rendering
# 4. Add specialized steps for hierarchical data transformations
# 5. Expose custom Nustage commands that feel like native Nushell commands
# 6. Consider providing a `@field` expression syntax that feels natural to Nushell users
# 7. Make `embed-nu` integration expose a Nushell-friendly API that feels like
#    scripting, not just embedding
#
# --- NEXT STEPS ---
#
# 1. Implement the recommended features in Nustage
# 2. Test the integration with real Nushell scripts
# 3. Gather user feedback on the feel of the integration
# 4. Iterate on the design based on real-world usage
# 5. Document the final API in the Nustage repository
#
# This document is ready to be dropped into the Nustage repository as a spec
# for the Nushell integration layer.
#
# --- END OF DOCUMENT ---" | save outputs/nustage_integration_design.md

# === DONE ===
# Design document saved to outputs/nustage_integration_design.md
# This file is ready to be copied to the Nustage repository
