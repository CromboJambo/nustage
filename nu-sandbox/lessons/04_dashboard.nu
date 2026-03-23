# Lesson 04: Dashboard Composition
# Goal: Produce a multi-section markdown dashboard
# Run this: nu lessons/04_dashboard.nu

# === COMPOSING MULTIPLE TABLES ===
# Now we'll put it all together to create a comprehensive dashboard.

# 1. Use `let` to bind tables to named variables
# This makes complex dashboards more readable and reusable

let system_info = sys
let top_processes = ps | sort-by cpu -r | first 10
let top_memory = ps | sort-by mem -r | first 10
let directory_listing = ls | select name type size modified

# 2. Combine multiple tables into one markdown document
# Use string interpolation to create a structured document

$"# NUSTAGE SYSTEM DASHBOARD
# Generated: (date now)
#
# ## System Information
#
($system_info | to md)
#
# ## Top CPU Processes
#
($top_processes | to md)
#
# ## Top Memory Processes
#
($top_memory | to md)
#
# ## Current Directory
#
($directory_listing | to md)" | save outputs/dashboard.md

# 3. Create a cleaner, more professional-looking dashboard
# Add formatting with blank lines and section headers

$"# 🚀 NUSTAGE SYSTEM OVERVIEW
# Last Updated: (date now)
#
# ## 📊 System Metrics
#
($sys | to md)
#
# ## 💻 CPU Usage (Top 5)
#
($ps | sort-by cpu -r | first 5 | to md)
#
# ## 💾 Memory Usage (Top 5)
#
($ps | sort-by mem -r | first 5 | to md)
#
# ## 📁 Current Directory
#
($ls | select name type size modified | to md)" | save outputs/dashboard_clean.md

# 4. Add conditional sections based on git status
# Check if we're in a git repository

let git_branch = ^git branch --show-current 2> /dev/null | str trim
let git_dirty = ^git status --porcelain 2> /dev/null | str trim | is-not-empty

if ($git_branch | is-not-empty) {
    $"# 🌿 Git Repository Status
    #
    # Branch: ($git_branch)
    #
    # Files Modified: ($git_dirty | str length)
    #
    # Git Status:
    #
    (#git_dirty | str trim | to md)" | save outputs/git_status.md
}

# 5. Create a minimal dashboard with just essential info
# Sometimes less is more

$"# 📋 Quick System Snapshot
#
## System
($sys | select host arch kernel version | to md)
#
## Top Processes
($ps | sort-by cpu -r | first 5 | select pid name cpu mem | to md)
#
## Disk Usage
($sys disks | to md)" | save outputs/quick_snapshot.md

# === PRACTICE EXERCISES ===
# Try these on your own - modify and rerun:

# Exercise 1: Create a dashboard that shows only the top 3 processes
# Hint: Use `first 3` in your pipeline

# Exercise 2: Add a section showing only directories from the current folder
# Hint: Use `ls | where type == dir`

# Exercise 3: Create a dashboard with a custom header and only system info
# Hint: Use string interpolation with `$"# Custom Header\n\n($sys | to md)"`

# Exercise 4: Save a dashboard that shows help commands starting with "sys"
# Hint: Use `help commands | where name | str starts-with "sys"`

# === KEY INSIGHT ===
# The power of Nushell dashboards comes from combining multiple data sources
# into a single, coherent document. Use `let` to bind complex expressions,
# then use string interpolation to assemble them into a structured markdown
# document. The result is a living, breathing documentation of your system
# that updates every time you run it!

# === NEXT STEPS ===
# In lesson 05, we'll learn how to make this dashboard live-reload
# automatically when files change, using the `watch` command.

# === CLEANUP ===
clear
