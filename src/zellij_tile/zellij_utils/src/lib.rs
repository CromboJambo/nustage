//! Nustage Zellij Utils
//!
//! Utility functions and helpers for integrating Nustage with Zellij tiles.

use std::path::PathBuf;
use zellij_tile_utils::DataFrame;
use zellij_tile_utils::{Event, EventWrapper, Key, Pane};

/// Load data from a file path
pub fn load_data_from_path(path: &PathBuf) -> Result<DataFrame, String> {
    // This will be implemented by integrating with nustage's data loading
    // For now, return a placeholder error
    Err("Data loading not yet implemented in zellij_utils".to_string())
}

/// Check if a key press corresponds to a transformation action
pub fn is_transformation_key(key: &Key) -> bool {
    matches!(
        key,
        Key::Char('c') // Add column
        | Key::Char('f') // Filter
        | Key::Char('g') // Group
        | Key::Char('r') // Rename
        | Key::Char('s') // Select
        | Key::Char('t') // Sort
        | Key::Char('d') // Drop
        | Key::Char('u') // Remove duplicates
    )
}

/// Check if a key press corresponds to navigation
pub fn is_navigation_key(key: &Key) -> bool {
    matches!(
        key,
        Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End
    )
}

/// Check if a key press corresponds to view switching
pub fn is_view_switch_key(key: &Key) -> bool {
    matches!(
        key,
        Key::Char('s') // Steps view
        | Key::Char('d') // Data view
        | Key::Char('z') // Schema view
        | Key::Char('x') // SQL view
        | Key::Char('h') // Help view
    )
}

/// Check if a key press corresponds to file operations
pub fn is_file_key(key: &Key) -> bool {
    matches!(
        key,
        Key::Char('l') // Load
        | Key::Char('l') // Save
        | Key::Char('e') // Export
        | Key::Char('p') // Print
    )
}

/// Check if a key press corresponds to tile operations
pub fn is_tile_key(key: &Key) -> bool {
    matches!(
        key,
        Key::Char('q') // Quit
        | Key::Char('n') // New pipeline
        | Key::Char('a') // Add step
        | Key::Char('r') // Remove step
    )
}

/// Format a key for display
pub fn format_key_for_display(key: &Key) -> String {
    match key {
        Key::Char(c) => format!("'{}'", c),
        Key::Backspace => "Backspace".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::Enter => "Enter".to_string(),
        Key::Esc => "Esc".to_string(),
        Key::Up => "Up".to_string(),
        Key::Down => "Down".to_string(),
        Key::Left => "Left".to_string(),
        Key::Right => "Right".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PageUp".to_string(),
        Key::PageDown => "PageDown".to_string(),
        Key::Delete => "Delete".to_string(),
        Key::F(n) => format!("F{}", n),
        Key::Null => "Null".to_string(),
        Key::Unknown => "Unknown".to_string(),
    }
}

/// Render a help section for a specific action
pub fn render_help_section(pane: &Pane, title: &str, description: &str) {
    pane.set_style("bold cyan");
    pane.print(format!("  {}", title));
    pane.set_style("reset");
    pane.print("\n");
    pane.set_style("dim gray");
    pane.print(format!("    {}", description));
    pane.set_style("reset");
}

/// Render a keybinding list
pub fn render_keybindings(pane: &Pane, bindings: &[(&str, &str)]) {
    pane.set_style("bold cyan");
    pane.print("  Keybindings:");
    pane.set_style("reset");

    for (key, description) in bindings {
        pane.set_style("dim gray");
        pane.print(format!("    {}", key));
        pane.set_style("reset");
        pane.print(format!("  - {}", description));
    }

    pane.print("\n");
}

/// Generate a SQL query from a transformation pipeline
pub fn generate_sql_from_pipeline(steps: &[zellij_tile_utils::TransformationStep]) -> String {
    // This will be implemented by integrating with nustage's SQL generation
    // For now, return a placeholder
    "SELECT * FROM source".to_string()
}

/// Validate a file path
pub fn validate_file_path(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("File does not exist: {}", path.display()));
    }

    if !path.is_file() {
        return Err(format!("Not a file: {}", path.display()));
    }

    Ok(())
}

/// Get the file extension for a path
pub fn get_file_extension(path: &PathBuf) -> Option<&str> {
    path.extension().and_then(|ext| ext.to_str())
}

/// Check if a file is a supported data format
pub fn is_supported_format(path: &PathBuf) -> bool {
    let ext = get_file_extension(path);
    matches!(
        ext,
        Some("csv") | Some("parquet") | Some("json") | Some("tsv")
    )
}

/// Parse a file path from user input
pub fn parse_file_path(input: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(input);

    if input.is_empty() {
        return Err("File path cannot be empty".to_string());
    }

    if !path.exists() {
        return Err(format!("File does not exist: {}", path.display()));
    }

    if !path.is_file() {
        return Err(format!("Not a file: {}", path.display()));
    }

    if !is_supported_format(&path) {
        return Err(format!(
            "Unsupported file format. Supported: CSV, Parquet, JSON, TSV"
        ));
    }

    Ok(path)
}

/// Format a dataframe for display in TUI
pub fn format_dataframe_for_display(
    pane: &Pane,
    data: &DataFrame,
    max_rows: usize,
    max_columns: usize,
) {
    let schema = data.schema();
    let columns = schema.names();

    // Truncate column names if too long
    let display_columns: Vec<String> = columns
        .iter()
        .map(|name| {
            if name.len() > max_columns {
                format!("{}...", &name[..max_columns - 3])
            } else {
                name.clone()
            }
        })
        .collect();

    // Print header
    pane.set_style("bold cyan");
    pane.print(format!("  {}", display_columns.join(" | ")));
    pane.set_style("reset");

    // Print data rows
    for row in data.iter().take(max_rows) {
        pane.print("\n");
        pane.set_style("dim gray");
        pane.print("  |");

        for val in row {
            pane.set_style("reset");
            pane.print(format!(" {}", val));
        }
    }

    pane.print("\n");
    pane.set_style("dim gray");
    pane.print(format!(
        "  (Showing {} rows of {} total)",
        max_rows,
        data.len()
    ));
}

/// Handle a file load event
pub fn handle_file_load(pane: &Pane, path: &PathBuf) -> Result<DataFrame, String> {
    validate_file_path(path)?;

    match load_data_from_path(path) {
        Ok(df) => {
            pane.set_style("bold green");
            pane.print(format!("  ✓ Loaded: {}", path.display()));
            pane.set_style("reset");
            Ok(df)
        }
        Err(e) => {
            pane.set_style("bold red");
            pane.print(format!("  ✗ Failed to load: {}", e));
            pane.set_style("reset");
            Err(e)
        }
    }
}

/// Render a status bar
pub fn render_status_bar(pane: &Pane, status: &str, details: Option<&str>) {
    pane.set_style("dim gray");
    pane.print(format!("  [{}]", status));

    if let Some(detail) = details {
        pane.print(format!("  - {}", detail));
    }

    pane.set_style("reset");
    pane.print("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_transformation_key() {
        assert!(is_transformation_key(&Key::Char('c')));
        assert!(is_transformation_key(&Key::Char('f')));
        assert!(!is_transformation_key(&Key::Char('q')));
    }

    #[test]
    fn test_is_navigation_key() {
        assert!(is_navigation_key(&Key::Up));
        assert!(is_navigation_key(&Key::Down));
        assert!(!is_navigation_key(&Key::Char('c')));
    }

    #[test]
    fn test_format_key_for_display() {
        assert_eq!(format_key_for_display(&Key::Char('c')), "'c'");
        assert_eq!(format_key_for_display(&Key::Up), "Up");
        assert_eq!(format_key_for_display(&Key::F(1)), "F1");
    }

    #[test]
    fn test_validate_file_path() {
        let valid_path = PathBuf::from("/tmp/test.csv");
        assert!(validate_file_path(&valid_path).is_ok());

        let non_existent = PathBuf::from("/tmp/nonexistent.csv");
        assert!(validate_file_path(&non_existent).is_err());

        let not_file = PathBuf::from("/tmp");
        assert!(validate_file_path(&not_file).is_err());
    }

    #[test]
    fn test_is_supported_format() {
        assert!(is_supported_format(&PathBuf::from("test.csv")));
        assert!(is_supported_format(&PathBuf::from("test.parquet")));
        assert!(!is_supported_format(&PathBuf::from("test.txt")));
    }

    #[test]
    fn test_parse_file_path() {
        let valid_path = PathBuf::from("test.csv");
        assert!(parse_file_path("test.csv").is_ok());

        let non_existent = PathBuf::from("nonexistent.csv");
        assert!(parse_file_path("nonexistent.csv").is_err());
    }
}
