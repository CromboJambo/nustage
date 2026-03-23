# Lesson 01: Tables
# Goal: Understand that everything in Nushell is a table
# Run this: nu lessons/01_tables.nu

# === DATA SOURCES ===
# In Nushell, almost everything is a table. Let's see what we can get.

# 1. File system: `ls` returns a table with columns like name, type, size, modified, etc.
ls

# 2. Process list: `ps` shows running processes with columns like pid, name, cpu, etc.
ps

# 3. System info: `sys` gives system metrics (CPU, memory, disk, etc.)
sys

# 4. Help commands: `help commands` shows all available commands as a table
help commands

# === ACCESSING TABLE DATA ===
# Now let's see how to work with these tables.

# Use `| get column_name` to extract a single column from a table
# Example: Get the name column from `ls`
ls | get name

# Use `| select column1 column2` to pick specific columns
# Example: Show only name and type from `ls`
ls | select name type

# Use `| where condition` to filter rows
# Example: Show only directories from `ls`
ls | where type == dir

# === PRACTICE EXERCISE ===
# Try these on your own - modify and rerun:

# Exercise 1: Show only files (not directories) from `ls`
# Hint: Use `where type == file`

# Exercise 2: Show the first 5 processes by CPU usage
# Hint: Use `ps` then `sort-by cpu` then `first 5`

# Exercise 3: Get all commands that contain "sys" in their name
# Hint: Use `help commands` then `where name | str contains "sys"`

# === KEY INSIGHT ===
# Everything you see above is a table. The power of Nushell comes from chaining these tables
# together with the pipeline operator `|`. That's what we'll explore in the next lesson!

# === CLEANUP ===
# Let's clear the screen for the next lesson
clear
