# Lesson 02: Pipelines
# Goal: Learn to shape any table into a useful slice
# Run this: nu lessons/02_pipelines.nu

# === SORTING AND ORDERING ===
# Now that we know everything is a table, let's learn how to shape them.

# 1. Sort by a column: `sort-by column_name`
# Example: Sort processes by CPU usage
ps | sort-by cpu

# 2. Sort in descending order: `sort-by column_name -r` (reverse)
# Example: Sort processes by CPU usage, highest first
ps | sort-by cpu -r

# 3. Reverse a table: `reverse`
# Example: Show the last 5 files in reverse order
ls | reverse | first 5

# === TAKING SUBSETS ===
# Get slices of your data without filtering

# 4. Get the first N rows: `first N`
# Example: Show the first 10 processes
ps | first 10

# 5. Get the last N rows: `last N`
# Example: Show the last 5 directories
ls | last 5

# 6. Skip N rows: `skip N`
# Example: Skip the first 3 processes and show the rest
ps | skip 3

# === GROUPING AND AGGREGATION ===
# Group data and calculate statistics

# 7. Group by a column: `group-by column_name`
# Example: Group files by their extension
ls | group-by extension

# 8. Get the length of a table: `length`
# Example: Count how many files we have
ls | length

# Example: Count how many processes are running
ps | length

# Example: Count how many commands start with "sys"
help commands | where name | str starts-with "sys" | length

# === INTERACTIVE EXPLORATION ===
# The `explore` command lets you interactively drill into your data

# 9. Explore a table: `explore`
# Example: Open an interactive explorer for system info
sys | explore

# === COMBINE IT ALL ===
# The real power comes from chaining these operations together

# Example: Show top 5 processes by CPU, sorted by memory
ps | sort-by cpu -r | select pid name cpu mem | first 5

# Example: Group processes by their user and count them
ps | group-by user | each { |it| { name: $it.name, count: ($it.item | length) } }

# Example: Show directory sizes, sorted by size, top 10
ls | where type == dir | sort-by size -r | select name size modified

# === PRACTICE EXERCISES ===
# Try these on your own - modify and rerun:

# Exercise 1: Show only the top 3 processes by memory usage
# Hint: Use `ps | sort-by mem -r | first 3`

# Exercise 2: Show files sorted by their modified date (newest first)
# Hint: Use `ls | sort-by modified -r`

# Exercise 3: Group all files by their type and show the counts
# Hint: Use `ls | group-by type | each { |it| { type: $it.name, count: ($it.item | length) } }`

# Exercise 4: Skip the first 5 commands and show the rest
# Hint: Use `help commands | skip 5`

# === KEY INSIGHT ===
# By chaining these operations, you can transform any table into exactly what you need.
# The pipeline operator `|` connects these operations together, passing the result of one
# command as input to the next. This is the heart of Nushell's structured data processing!

# === NEXT STEPS ===
# In the next lesson, we'll learn how to export these tables to markdown files
# that you can view in your editor or share with others.

# === CLEANUP ===
clear
