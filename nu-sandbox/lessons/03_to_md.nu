# Lesson 03: Export to Markdown
# Goal: Learn to write any table to a renderable .md file
# Run this: nu lessons/03_to_md.nu

# === EXPORTING TO MARKDOWN ===
# Now that we can shape tables, let's learn how to save them as markdown files.

# 1. Basic markdown export: `| to md`
# Example: Export the current directory listing to markdown
ls | to md

# 2. Save directly to a file: `| save path/to/file.md`
# Example: Save the process list to a markdown file
ps | save outputs/processes.md

# 3. Export to CSV: `| to csv`
# Example: Export processes as CSV (good for spreadsheets)
ps | to csv | save outputs/processes.csv

# 4. Export to JSON: `| to json`
# Example: Export processes as JSON (good for APIs/data exchange)
ps | to json | save outputs/processes.json

# === WRAPPING TABLES IN CONTEXT ===
# Often you want to add context around a table. Use string interpolation!

# 5. Add a header to your markdown output
# Example: Create a markdown file with a header and a table
$"# Process List\n\n($ps | to md)" | save outputs/processes_with_header.md

# 6. Add a footer or notes
# Example: Add a timestamp to your markdown
$"# System Status\nGenerated: (date now)\n\n($sys | to md)" | save outputs/system_status.md

# 7. Combine multiple tables in one document
# Example: Create a multi-section dashboard
let top_processes = ps | sort-by cpu -r | first 10
$"# Top Processes\n\n($top_processes | to md)" | save outputs/top_processes.md

let top_memory = ps | sort-by mem -r | first 10
$"# Top Memory Usage\n\n($top_memory | to md)" | save outputs/top_memory.md

# === SAVING TO SPECIFIC OUTPUT DIRECTORY ===
# The spec mentions an outputs/ directory. Let's set that up.

# Create the outputs directory if it doesn't exist
mkdir -p outputs

# Save a clean version of the directory listing
ls | select name type size modified | save outputs/directory_listing.md

# === PRACTICE EXERCISES ===
# Try these on your own - modify and rerun:

# Exercise 1: Export the system info to a markdown file
# Hint: Use `sys | to md | save outputs/system_info.md`

# Exercise 2: Create a markdown file that shows the top 5 files by size
# Hint: Use `ls | sort-by size -r | first 5 | to md | save outputs/top_files.md`

# Exercise 3: Create a markdown document with a header and the system info table
# Hint: Use string interpolation with `$"# System Info\n\n($sys | to md)" | save outputs/system_info_with_header.md`

# Exercise 4: Export help commands that start with "sys" as JSON
# Hint: Use `help commands | where name | str starts-with "sys" | to json | save outputs/sys_commands.json`

# === KEY INSIGHT ===
# The `to md` command transforms any table into a readable markdown format.
# Combine it with `save` to persist your data, and use string interpolation
# to add context, headers, and structure. This is how you create documentation
# and shareable reports from your shell data!

# === NEXT STEPS ===
# In lesson 04, we'll learn how to compose multiple tables into a single
# multi-section markdown dashboard document.

# === CLEANUP ===
clear
