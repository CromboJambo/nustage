//! Nustage - A data processing and analysis toolkit for Rust
//!
//! This is the main library module for the nustage project.

pub mod data;
pub mod transformations;

/// Core data structures and utilities
pub mod core {
    use serde::{Deserialize, Serialize};

    /// Column schema definition
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ColumnSchema {
        pub index: usize,
        pub name: String,
        pub data_type: String,
    }

    /// Data frame schema
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DataFrameSchema {
        pub columns: Vec<ColumnSchema>,
    }
}

/// Main entry point for the nustage application
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Nustage is running...");
    Ok(())
}
