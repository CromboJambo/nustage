# Nustage Nushell Sandbox - Quick Start Guide

Get up and running with the Nushell Dashboard Sandbox in 5 minutes!

## 🚀 Quick Start

### 1. Install Prerequisites

```bash
# Install Nushell (if not already installed)
brew install nushell  # macOS
winget install nushell # Windows
cargo install nu       # Linux/macOS via cargo

# Install glow for terminal preview (optional but recommended)
brew install glow
```

### 2. Navigate to the Sandbox

```bash
cd nu-sandbox
```

### 3. Run Your First Lesson

```bash
nu lessons/01_tables.nu
```

That's it! You'll see tables from your system and learn how to work with them.

### 4. Run All Lessons (Recommended)

```bash
nu run.nu
```

This will guide you through all 6 lessons with progress feedback.

### 5. Build Your First Dashboard

```bash
nu scripts/dashboard.nu
```

This creates a comprehensive system dashboard with:
- System metrics (CPU, memory, disk)
- Top processes by CPU and memory
- Current directory listing
- Git status (if in a repo)

View the output:
```bash
# In terminal
glow outputs/dashboard.md

# In Zed editor
zed outputs/dashboard.md
```

## 📚 What You'll Learn

| Lesson | What You'll Learn | Time |
|--------|------------------|------|
| **01 Tables** | Everything is a table, how to access columns | 5 min |
| **02 Pipelines** | Sorting, filtering, grouping, shaping tables | 10 min |
| **03 To Markdown** | Export tables to .md files for sharing | 5 min |
| **04 Dashboard** | Compose multiple tables into one document | 10 min |
| **05 Watch** | Live-reloading dashboards with auto-update | 10 min |
| **06 Nustage Bridge** | Translation table for Nustage integration | 15 min |

## 🎯 The Goal

By the end, you'll be able to:
1. ✅ Script sensible default dashboard views in Nushell
2. ✅ Export tables to beautiful markdown files
3. ✅ Set up live-reloading dashboards
4. ✅ Understand how Nushell maps to Nustage's Rust pipeline

## 🔍 Design Probe Questions

As you work through this sandbox, you're also helping design Nustage's Nushell integration:

- What does `| where` feel like vs Nustage's `FilterRows`?
- Does `| group-by` + `| math sum` map cleanly to Nustage's model?
- What should Nustage's `@field` expression syntax feel like?
- Where does Nushell's pipeline model break down for tabular transforms?

## 🛠️ Project Structure

```
nu-sandbox/
├── README.md              # Full documentation
├── QUICKSTART.md          # This file
├── run.nu                 # Run all lessons
├── lessons/
│   ├── 01_tables.nu       # Structured data basics
│   ├── 02_pipelines.nu    # Filtering, sorting, shaping
│   ├── 03_to_md.nu        # Exporting to markdown
│   ├── 04_dashboard.nu    # Composing dashboards
│   ├── 05_watch.nu        # Live-reloading
│   └── 06_nustage_bridge.nu # Design probe
├── scripts/
│   ├── dashboard.nu       # Final working dashboard
│   └── nustage_probe_summary.nu # Design spec generator
└── outputs/               # Generated .md files
```

## 🎓 Learning Tips

1. **Read the comments** - Every lesson file has inline tutorial text
2. **Experiment** - Modify lessons and rerun to see what happens
3. **Take breaks** - Each lesson is designed to be 5-10 minutes
4. **Use real data** - Everything pulls from your actual system
5. **Check outputs** - Look at `outputs/` to see what you've created

## 🚦 Next Steps

- **Start here:** Read this QUICKSTART guide
- **Run lessons:** Execute `nu run.nu` or individual lessons
- **Explore outputs:** Check `outputs/dashboard.md` and design docs
- **Customize:** Modify lessons to create your own views
- **Contribute:** Use your insights to inform Nustage's Nushell integration

## 💡 Pro Tips

### Run a Single Lesson
```bash
nu lessons/02_pipelines.nu
```

### Create Custom Views
Edit `lessons/04_dashboard.nu` and add your own sections:
```nu
let my_custom_data = ls | where name | str contains "important"
$"## My Custom Section\n\n($my_custom_data | to md)"
```

### Live Preview
Set up a watch loop for auto-updating dashboards:
```bash
# Terminal 1
watch . { nu scripts/dashboard.nu }

# Terminal 2
glow outputs/dashboard.md
```

### Quick Snapshot
Generate a minimal dashboard:
```bash
nu -c "sys | to md | save outputs/quick_snapshot.md"
```

## 📖 Further Reading

- [Nushell Book](https://www.nushell.sh/book) - Official Nushell documentation
- [Nushell Commands](https://www.nushell.sh/commands) - Command reference
- [Glow](https://github.com/charmbracelet/glow) - Terminal markdown renderer
- [Nustage Repo](https://github.com/CromboJambo/nustage) - The project this sandbox informs

## ❓ Common Issues

**"Command not found: nu"**
- Install Nushell using the commands above

**"Command not found: glow"**
- Install glow using `brew install glow`

**"outputs/ directory doesn't exist"**
- The lessons create it automatically when you run them

**"My changes aren't showing"**
- Make sure you're running `nu` (not `bash` or `zsh`) when executing `.nu` files

## 🎉 You're Ready!

Start with Lesson 01 and enjoy learning Nushell's structured data pipeline while helping design Nustage's future!

```bash
nu lessons/01_tables.nu
```

Happy scripting! 🚀