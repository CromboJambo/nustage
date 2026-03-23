# NUSTAGE SANDBOX - RUN ALL LESSONS
# This script runs all lessons in sequence with progress feedback
# Run this: nu run.nu

# === CONFIGURATION ===
# You can customize these settings
let show_detailed_output = false  # Set to true for verbose output
let skip_clean = false             # Set to true to skip clear commands

# === LESSON RUNNER ===
# Helper function to run a lesson with feedback
def run-lesson [lesson_path: string] {
    print ""
    print $"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    print $"📚 Running: ($lesson_path)"
    print $"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    try {
        nu $lesson_path
        if $show_detailed_output {
            print ""
            print $"✅ Lesson completed successfully!"
        }
    } catch { |err|
        print ""
        print $"❌ Error running lesson: ($err.msg)"
        print $"   Continuing to next lesson..."
    }
}

# === MAIN RUNNER ===
print "🚀 NUSTAGE NUSHELL SANDBOX - LESSON RUNNER"
print "=========================================="
print ""
print "This script will run all lessons in sequence."
print "Each lesson builds on the last, so they should be run in order."
print ""
print "Press Enter to start, or Ctrl+C to cancel."
read

# === RUN LESSONS ===

# Lesson 1: Tables
run-lesson "lessons/01_tables.nu"

# Lesson 2: Pipelines
run-lesson "lessons/02_pipelines.nu"

# Lesson 3: To Markdown
run-lesson "lessons/03_to_md.nu"

# Lesson 4: Dashboard Composition
run-lesson "lessons/04_dashboard.nu"

# Lesson 5: Live Reload
run-lesson "lessons/05_watch.nu"

# Lesson 6: Nustage Bridge Probe (The Design Probe!)
run-lesson "lessons/06_nustage_bridge.nu"

# === DONE ===
print ""
print "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
print "🎉 ALL LESSONS COMPLETED!"
print "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
print ""
print "Your output files are in the `outputs/` directory:"
print "  - outputs/dashboard.md (main dashboard)"
print "  - outputs/nustage_integration_design.md (design spec)"
print "  - outputs/nustage_side_by_side.md (comparisons)"
print ""
print "You can now:"
print "  1. Run the final dashboard: nu scripts/dashboard.nu"
print "  2. Explore the design documents in outputs/"
print "  3. Modify lessons to experiment with Nushell pipelines"
print "  4. Use these insights to inform Nustage's Nushell integration"
print ""
print "Happy learning and happy hacking! 🚀"
print ""
