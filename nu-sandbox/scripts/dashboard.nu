# NUSTAGE SYSTEM DASHBOARD
# The final unlock script - a working dashboard with sensible defaults
# Run this: nu scripts/dashboard.nu

# === SYSTEM METRICS ===
# Get basic system information
let system_info = sys

# === TOP PROCESSES ===
# Get top 10 processes by CPU usage, sorted by memory
let top_processes = ps | sort-by cpu -r | first 10

# === TOP MEMORY PROCESSES ===
# Get top 10 processes by memory usage
let top_memory = ps | sort-by mem -r | first 10

# === CURRENT DIRECTORY ===
# Get current directory listing, sorted by modified date (newest first)
let directory_listing = ls | sort-by modified -r

# === DISK USAGE ===
# Get disk information
let disk_usage = sys disks

# === GIT STATUS (optional) ===
# Check if we're in a git repository
let git_branch = ^git branch --show-current 2> /dev/null | str trim
let git_dirty = ^git status --porcelain 2> /dev/null | str trim | is-not-empty

# === CREATE DASHBOARD ===
# Compose everything into a single markdown document
$"# 🚀 NUSTAGE SYSTEM DASHBOARD
# Generated: (date now)
#
# ## 📊 System Information
#
($system_info | to md)
#
# ## 💻 Top CPU Processes (Top 10)
#
($top_processes | to md)
#
# ## 💾 Top Memory Processes (Top 10)
#
($top_memory | to md)
#
# ## 📁 Current Directory (Newest First)
#
($directory_listing | select name type size modified | to md)
#
# ## 💿 Disk Usage
#
($disk_usage | to md)
#
# ## 🌿 Git Status
#
if ($git_branch | is-not-empty) {
    $"# Branch: ($git_branch)
    # Dirty Files: ($git_dirty | str length)
    #
    # ($git_dirty | str trim | to md)"
} else {
    "# Not in a git repository"
} | save outputs/dashboard.md

# === DONE ===
# Dashboard saved to outputs/dashboard.md
# Run this file to regenerate the dashboard
