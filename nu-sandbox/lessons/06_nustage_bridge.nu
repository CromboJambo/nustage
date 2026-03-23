# Lesson 06: Nustage Bridge Probe
# Goal: Create a translation table between Nushell pipeline syntax and Nustage's StepType enum
# Run this: nu lessons/06_nustage_bridge.nu

# === LOADING NUSTAGE TEST DATA ===
# Now we'll load actual Nustage test CSVs and replicate the power_query_workflow.rs example
# This is the meta-lesson that answers: "what should the embed-nu integration layer expose?"

# First, let's check if Nustage test data exists
# The spec mentions: test_data/sales.csv and test_data/expenses.csv
# If these files exist, we'll use them. If not, we'll create sample data.

# Try to load sales.csv from Nustage test data directory
let sales_data = try { open test_data/sales.csv } catch {
    # If it doesn't exist, create sample sales data
    # This makes the lesson runnable even without the full Nustage repo
    $"date,product,region,quantity,price,revenue
2024-01-01,Widget A,North,10,50,500
2024-01-02,Widget B,South,15,75,1125
2024-01-03,Widget C,East,20,30,600
2024-01-04,Widget D,West,5,100,500
2024-01-05,Widget A,North,8,50,400
2024-01-06,Widget B,South,12,75,900
2024-01-07,Widget C,East,18,30,540
2024-01-08,Widget D,West,7,100,700" | from csv
}

# Try to load expenses.csv from Nustage test data directory
let expenses_data = try { open test_data/expenses.csv } catch {
    # If it doesn't exist, create sample expenses data
    $"date,category,amount,department
2024-01-01,Software,500,Engineering
2024-01-02,Hardware,1200,Engineering
2024-01-03,Office,300,Operations
2024-01-04,Travel,450,Sales
2024-01-05,Software,600,Engineering
2024-01-06,Hardware,800,Operations
2024-01-07,Travel,200,Sales
2024-01-08,Software,400,Engineering" | from csv
}

# === REPLICATING POWER_QUERY_WORKFLOW.RS ===
# The spec mentions: examples/power_query_workflow.rs
# Let's replicate this workflow in Nushell:
# open test_data/sales.csv | where Revenue > 1000 | group-by Region | math sum

# Step 1: Load the data
# In Nustage: LoadCSV(path)
# In Nushell: open path
let loaded_sales = open test_data/sales.csv

# Step 2: Filter rows where Revenue > 1000
# In Nustage: FilterRows("Revenue", "> 1000")
# In Nushell: | where Revenue > 1000
let filtered_sales = loaded_sales | where revenue > 1000

# Step 3: Group by Region
# In Nustage: GroupBy("Region", [])
# In Nushell: | group-by region
let grouped_sales = filtered_sales | group-by region

# Step 4: Calculate sums
# In Nustage: Aggregation.Sum("revenue")
# In Nushell: | math sum revenue
let summed_sales = grouped_sales | each { |it| { region: $it.name, total_revenue: ($it.item | math sum revenue) } }

# === COMPARING PIPELINE TO STEPTYPE ===
# Now let's create a translation table comparing Nushell to Nustage

# StepType enum from Nustage (for reference):
# pub enum StepType {
#     LoadCSV(String),
#     FilterRows(String, String),
#     SortBy(String, bool),
#     SelectColumns(Vec<String>),
#     AddColumn(String, String),
#     RenameColumn(String, String),
#     DropColumn(String),
#     GroupBy(Vec<String>, Vec<Aggregation>),
#     Aggregation(Aggregation),
#     ExportCSV(String),
#     ExportMarkdown(String),
#     ExportJSON(String),
# }

# Let's create a markdown document with the translation table

$"# NUSTAGE - NUSHELL PIPELINE TRANSLATION TABLE
#
# This document maps Nushell's pipeline operations to Nustage's StepType enum
# and identifies gaps, frictions, and opportunities for the Nushell integration.
#
# Generated from: nu-sandbox lesson 06
# Last Updated: (date now)
#
# --- TRANSLATION TABLE ---
#
# NUSTAGE STEP TYPE → NUSHELL PIPELINE EQUIVALENT
#
# LoadCSV(path) → open path
# FilterRows(column, condition) → open table | where column condition
# SortBy(column, direction) → open table | sort-by column -r
# SelectColumns(columns) → open table | select columns
# AddColumn(name, expression) → open table | update name { expression }
# RenameColumn(old, new) → open table | rename old new
# DropColumn(column) → open table | reject column
# GroupBy(columns, aggregations) → open table | group-by columns | each { ... }
# Aggregation.Sum(column) → open table | math sum column
# Aggregation.Average(column) → open table | math avg column
# Aggregation.Count() → open table | length
# ExportToCSV(path) → open table | to csv | save path
# ExportToMarkdown(path) → open table | to md | save path
# ExportToJSON(path) → open table | to json | save path
#
# --- DESIGN INSIGHTS ---
#
# 1. STRING-BASED CONDITIONS
#
#    Nustage: FilterRows("Revenue", "> 1000")
#    Nushell: | where revenue > 1000
#
#    FEEDBACK: The Nushell expression language is more powerful and feels more natural
#    for interactive scripting. However, it requires runtime evaluation of expressions.
#    The string-based model would be simpler but less flexible.
#
#    RECOMMENDATION: Support both models. Provide a `@field` syntax that feels native
#    to Nushell users while maintaining backward compatibility.
#
# 2. CHAINABLE AGGREGATION
#
#    Nustage: GroupBy("Region", [Sum("revenue")])
#    Nushell: | group-by region | each { |it| { region: $it.name, total_revenue: ($it.item | math sum revenue) } }
#
#    FEEDBACK: The Nushell pipeline model works well with the step-by-step approach.
#    However, the `each` loop feels clunky compared to a declarative aggregation step.
#
#    RECOMMENDATION: Nustage should support chainable aggregation steps that match
#    Nushell's pipeline feel, but also provide a declarative API for complex aggregations.
#
# 3. OUTPUT FORMATS
#
#    Nustage: ExportToMarkdown(path)
#    Nushell: | to md | save path
#
#    FEEDBACK: `to md` produces clean, formatted markdown tables with headers.
#    Nustage's output layer should replicate this for terminal rendering.
#
#    RECOMMENDATION: Replicate the `to md` output format for terminal rendering,
#    but also provide hooks for custom formatters.
#
# 4. HIERARCHICAL DATA
#
#    Nustage: Specialized steps for BOM hierarchies and variance tracking
#    Nushell: Limited support for nested structures
#
#    FEEDBACK: Nushell's table model is fundamentally flat. It struggles with
#    hierarchical data structures and complex variance calculations that require
#    multiple passes over the data.
#
#    RECOMMENDATION: Nustage will need specialized steps for these cases that
#    go beyond the basic table model.
#
# 5. CUSTOM COMMANDS
#
#    Nustage: Would need custom commands for native feel
#    Nushell: No built-in custom command support for Nustage
#
#    FEEDBACK: Custom commands like `nustage load`, `nustage filter`, `nustage group`
#    would make the integration feel native.
#
#    RECOMMENDATION: Expose custom Nustage commands that mirror the pipeline operations.
#    Make `embed-nu` integration expose a Nushell-friendly API that feels like scripting,
#    not just embedding.
#
# --- FRICTION POINTS ---
#
# 1. Nushell's `where` uses a Nushell expression language, which is more powerful
#    but requires runtime evaluation. The string-based model would be simpler but less powerful.
#
# 2. Nushell's `group-by` produces a table of groups, requiring additional processing
#    to get aggregated results. Nustage's model should support this chainable pattern.
#
# 3. Nushell's table model is flat, making it difficult to work with hierarchical data.
#    Nustage will need specialized steps for BOM hierarchies and variance tracking.
#
# 4. The `@field` expression syntax is not yet defined from a user's perspective.
#    We need to determine what it should feel like.
#
# --- RECOMMENDATIONS FOR NUSTAGE ---
#
# 1. Support both string and expression-based conditions for flexibility
# 2. Implement chainable aggregation steps that match Nushell's pipeline
# 3. Replicate `to md` output format for terminal rendering
# 4. Add specialized steps for hierarchical data transformations
# 5. Expose custom Nustage commands that feel like native Nushell commands
# 6. Provide a `@field` expression syntax that feels natural to Nushell users
# 7. Make `embed-nu` integration expose a Nushell-friendly API that feels like scripting
# 8. Support both declarative and imperative approaches for complex operations
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

# === COMPARISON EXAMPLES ===
# Let's create a side-by-side comparison of Nustage steps and Nushell pipelines

$"# NUSTAGE - SIDE-BY-SIDE COMPARISON
#
# This section shows concrete examples of Nustage steps vs Nushell pipelines.
#
# --- EXAMPLE 1: FILTER ROWS ---
#
# Nustage:
# ```rust
# let data = LoadCSV("sales.csv").run()?;
# let filtered = FilterRows("Revenue", "> 1000").run(data)?;
# ```
#
# Nushell:
# ```nu
# open sales.csv | where revenue > 1000
# ```
#
# --- EXAMPLE 2: GROUP BY AND AGGREGATE ---
#
# Nustage:
# ```rust
# let data = LoadCSV("sales.csv").run()?;
# let grouped = GroupBy(["Region"], [Sum("Revenue")]).run(data)?;
# ```
#
# Nushell:
# ```nu
# open sales.csv
# | group-by region
# | each { |it| {
#     region: $it.name,
#     total_revenue: ($it.item | math sum revenue)
#   }
# }
# ```
#
# --- EXAMPLE 3: EXPORT TO MARKDOWN ---
#
# Nustage:
# ```rust
# let data = LoadCSV("sales.csv").run()?;
# ExportToMarkdown("sales_report.md").run(data)?;
# ```
#
# Nushell:
# ```nu
# open sales.csv | to md | save sales_report.md
# ```
#
# --- EXAMPLE 4: COMPLEX PIPELINE ---
#
# Nustage:
# ```rust
# let data = LoadCSV("sales.csv").run()?;
# let filtered = FilterRows("Revenue", "> 1000").run(data)?;
# let sorted = SortBy("Revenue", Descending).run(filtered)?;
# let selected = SelectColumns(["Region", "Revenue"]).run(sorted)?;
# let exported = ExportToMarkdown("report.md").run(selected)?;
# ```
#
# Nushell:
# ```nu
# open sales.csv
# | where revenue > 1000
# | sort-by revenue -r
# | select region revenue
# | to md | save report.md
# ```
#
# --- KEY OBSERVATIONS ---
#
# 1. The Nushell pipeline model is more concise and feels more natural for scripting.
# 2. Nustage's step-based model is more explicit and declarative.
# 3. The translation is straightforward but requires some adaptation.
# 4. The Nushell expression language is more powerful than string-based conditions.
# 5. The `to md` command produces high-quality output that Nustage should replicate.
#
# --- END OF COMPARISON ---" | save outputs/nustage_side_by_side.md

# === DONE ===
# Translation tables and design insights saved to outputs/
# This completes the Nustage integration design probe!
# Now you have concrete feedback for designing the Nushell layer in Nustage.

# === REFLECTION ===
# Take a moment to review the outputs:
# - outputs/nustage_integration_design.md - The main design spec
# - outputs/nustage_side_by_side.md - Side-by-side comparisons
#
# These documents answer the key questions about what the embed-nu integration
# should expose and how it should feel to Nushell users.

# === CLEANUP ===
# Clear the screen
clear
