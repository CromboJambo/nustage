# Lesson 05: Live Reload
# Goal: Set up a working live-reloading dashboard loop
# Run this: nu lessons/05_watch.nu

# === WATCHING FOR CHANGES ===
# Now let's make our dashboard live. When files change, the dashboard updates automatically.

# 1. Basic watch: `watch . { command }`
# This watches the current directory for changes and runs the command
# Example: Watch the dashboard script and regenerate it when it changes
# watch . { nu scripts/dashboard.nu }

# 2. Watch specific files
# You can watch specific files instead of the whole directory
# watch lessons/*.nu { nu scripts/dashboard.nu }

# 3. Watch with a delay
# Some commands need a moment to stabilize after changes
# watch --delay 1s . { nu scripts/dashboard.nu }

# === TERMINAL PREVIEW WITH GLOW ===
# Glow is a terminal markdown renderer that makes reading .md files beautiful

# 1. Basic glow preview
# Example: Preview the dashboard in the terminal
# glow outputs/dashboard.md

# 2. Glow with line numbers
# Example: Show line numbers for easier reference
# glow --line-numbers outputs/dashboard.md

# 3. Glow with syntax highlighting
# Example: Enable syntax highlighting for code blocks
# glow --syntax outputs/dashboard.md

# 4. Glow with custom theme
# Example: Use a specific glow theme
# glow --theme monokai outputs/dashboard.md

# === COMBINING WATCH AND GLOW ===
# The holy grail: watch for changes and auto-refresh the glow preview

# 1. Manual combination
# Run both commands in separate terminals
# Terminal 1: watch . { nu scripts/dashboard.nu }
# Terminal 2: glow outputs/dashboard.md

# 2. Use a shell function to run both
# Add this to your Nushell config (~/.config/nushell/config.nu)
#
# def run-dashboard [] {
#     nu scripts/dashboard.nu
#     glow outputs/dashboard.md
# }
#
# alias d = run-dashboard

# === SPLIT-PANE LAYOUTS ===
# For a better experience, use a terminal multiplexer with split panes

# 1. Zellij split pane
# Zellij allows you to have multiple panes in one window
#
# # Create a new layout file: .zellij/kdl/dashboard.kdl
# layout {
#     pane size=1:3, borderless=true {
#         run "nu scripts/dashboard.nu"
#         split normal {
#             run "glow outputs/dashboard.md"
#         }
#     }
# }
#
# # Then in Zellij: zellij -l dashboard

# 2. WezTerm split pane
# WezTerm also supports split panes
#
# # Add this to your WezTerm config
# local split = wezterm.action.SplitPane { direction = "Right", params = { command = { program = "glow", args = { "outputs/dashboard.md" } } } }
#
# local run = wezterm.action{ SpawnWindow = { args = { program = "nu", args = { "scripts/dashboard.nu" } } } }
#
# local dashboard = wezterm.action{ Combine = { run, split } }

# === EDITOR-SIDE PREVIEW ===
# Zed has built-in markdown preview, perfect for this use case

# 1. Open the dashboard in Zed
# Just run: zed outputs/dashboard.md
# Zed will show you the rendered markdown in a preview pane

# 2. Use Zed with the terminal
# You can have the terminal open in Zed and the markdown preview side-by-side
# This is often the best of both worlds: interactive shell + visual preview

# === BUILDING A LIVE DASHBOARD LOOP ===
# Let's create the complete workflow

# Step 1: Create the dashboard script
# We already have scripts/dashboard.nu from previous lessons

# Step 2: Create a watch command
# Add this to your Nushell config for easy dashboard updates
#
# def watch-dashboard [] {
#     # Regenerate the dashboard
#     nu scripts/dashboard.nu
#     # Show the preview
#     glow outputs/dashboard.md
# }
#
# alias wd = watch-dashboard

# Step 3: Run the live loop
# Run: wd
# Now whenever you save the dashboard script or any lesson file, it updates automatically

# === PRACTICE EXERCISES ===
# Try these on your own - modify and rerun:
#
# Exercise 1: Create a watch command that only regenerates when lesson files change
# Hint: Use watch with a specific pattern
#
# Exercise 2: Set up a Zellij layout for your dashboard
# Hint: Create a .zellij/kdl/dashboard.kdl file with split panes
#
# Exercise 3: Create a shell alias to run both the dashboard and glow
# Hint: Use the alias command with a custom function
#
# Exercise 4: Test the live reload by modifying a lesson file
# Hint: Change something in one of the .nu files and watch the dashboard update

# === KEY INSIGHT ===
# The magic of Nushell dashboards is that they're living documents.
# They're not static reports—they're dynamic views of your system that update
# in real-time. Combine watch for automation, glow for terminal beauty,
# and Zed for editor-side preview, and you have a powerful monitoring
# system that's entirely scriptable and maintainable.

# === THE FINAL PRODUCT ===
# You now have everything you need to build custom dashboards for any project.
# The dashboard script (scripts/dashboard.nu) is your base.
# Add your own sections, customize the output, and make it yours.
#
# Remember: Every lesson you complete is also a usability test for Nustage.
# Pay attention to what feels natural vs. what feels clunky.
# That feedback is gold for designing the Nushell integration layer.

# === CLEANUP ===
# Clear the screen and show off your new skills
clear
```

This lesson completes the core tutorial! Let me create the final dashboard script and the .gitkeep file to complete the sandbox structure.<tool_call>edit_file<arg_key>display_description</arg_key><arg_value>Create .gitkeep file in outputs directory</arg_value><arg_key>mode</arg_key><arg_value>create</arg_value><arg_key>path</arg_key><arg_value>nu-sandbox/outputs/.gitkeep</arg_value></tool_call>
