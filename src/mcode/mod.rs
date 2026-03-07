//! PowerQuery M Code Generator
//!
//! Converts Nustage transformation steps into Excel PowerQuery M language

use crate::transformations::{StepType, TransformationStep};
use serde_json::Value;

/// Generate M code from a single transformation step
pub fn step_to_m(step: &TransformationStep) -> Result<String, String> {
    match &step.step_type {
        StepType::SelectColumns(columns) => Ok(format!(
            "Table.SelectColumns(Source, {{ {} }})",
            columns
                .iter()
                .map(|c| format!("\"{}\"", c))
                .collect::<Vec<_>>()
                .join(", ")
        )),

        StepType::FilterRows(column, condition) => {
            let m_condition = parse_condition_to_m(column, condition)?;
            Ok(format!("Table.SelectRows(Source, each {})", m_condition))
        }

        StepType::GroupBy(columns, aggregations) => {
            let group_cols = columns
                .iter()
                .map(|c| format!("\"{}\"", c))
                .collect::<Vec<_>>()
                .join(", ");

            let agg_table = aggregations
                .iter()
                .map(|agg| {
                    let op = match &agg.operation {
                        crate::transformations::AggregationOperation::Sum => "List.Sum",
                        crate::transformations::AggregationOperation::Mean => "List.Average",
                        crate::transformations::AggregationOperation::Count => "List.Count",
                        crate::transformations::AggregationOperation::Min => "List.Min",
                        crate::transformations::AggregationOperation::Max => "List.Max",
                        crate::transformations::AggregationOperation::First => "List.First",
                        crate::transformations::AggregationOperation::Last => "List.Last",
                        crate::transformations::AggregationOperation::StdDev => {
                            "List.StandardDeviation"
                        }
                        crate::transformations::AggregationOperation::Variance => "List.Variance",
                    };

                    format!("[{}] = {}([{}])", agg.column, op, agg.column)
                })
                .collect::<Vec<_>>()
                .join(", ");

            Ok(format!(
                "Table.Group(Source, {{ {} }}, {{ {} }})",
                group_cols, agg_table
            ))
        }

        StepType::SortBy(columns, descending) => {
            let sort_specs = columns
                .iter()
                .map(|c| {
                    format!(
                        "{{\"{}\", Order.{}}}",
                        c,
                        if *descending {
                            "Descending"
                        } else {
                            "Ascending"
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");

            Ok(format!("Table.Sort(Source, {{ {} }})", sort_specs))
        }

        StepType::RenameColumn(old_name, new_name) => Ok(format!(
            "Table.RenameColumns(Source, {{{{\"{}\", \"{}\"}}}})",
            old_name, new_name
        )),

        StepType::DropColumns(columns) => {
            let drop_cols = columns
                .iter()
                .map(|c| format!("\"{}\"", c))
                .collect::<Vec<_>>()
                .join(", ");

            Ok(format!("Table.RemoveColumns(Source, {{ {} }})", drop_cols))
        }

        StepType::CustomSql(sql) => {
            // Custom SQL - convert to M if needed or wrap as text
            Err(format!(
                "Custom SQL step requires manual M conversion: {}",
                sql
            ))
        }

        StepType::AddColumn(name, expr) => {
            let m_expr = transform_expression_to_m(expr)?;
            Ok(format!(
                "Table.AddColumn(Source, \"{}\", each {})",
                name, m_expr
            ))
        }

        StepType::RemoveDuplicates(columns) => {
            if columns.is_empty() {
                Ok("Table.Distinct(Source)".to_string())
            } else {
                let dup_cols = columns
                    .iter()
                    .map(|c| format!("\"{}\"", c))
                    .collect::<Vec<_>>()
                    .join(", ");

                Ok(format!(
                    "Table.Distinct(Source, EquivalenceCriteria.FromColumns({}))",
                    dup_cols
                ))
            }
        }
    }
}

/// Parse a condition and convert to M language
fn parse_condition_to_m(column: &str, condition: &str) -> Result<String, String> {
    // Simple condition parsing for common patterns
    let parts: Vec<&str> = condition.split_whitespace().collect();

    if parts.len() < 3 {
        return Err(format!("Invalid condition format: {}", condition));
    }

    let col = parts[0];
    let op = parts[1];
    let value = parts[2..].join(" ");

    // Map operators to M equivalents
    let m_op = match op {
        ">" => ">",
        ">=" => ">=",
        "<" => "<",
        "<=" => "<=",
        "=" | "==" => "=",
        "!=" | "<>" => "<>",
        "in" => "List.Contains({})",
        "contains" => "Text.Contains",
        _ => return Err(format!("Unsupported operator: {}", op)),
    };

    // Determine value type and format accordingly
    let formatted_value = if value.starts_with('"') {
        // Already a string literal in M syntax
        value.trim_matches('"').to_string()
    } else if value.parse::<f64>().is_ok() || value.parse::<i64>().is_ok() {
        // Numeric - no quotes needed
        value
    } else {
        // String - add quotes
        format!("\"{}\"", value.trim_matches('"'))
    };

    match op {
        "in" => Ok(format!(m_op, formatted_value)),
        "contains" => Ok(format!("Text.Contains([{}], {})", col, formatted_value)),
        _ => Ok(format!("[{}] {} {}", col, m_op, formatted_value)),
    }
}

/// Transform an expression to M language (e.g., @Revenue - @Cost)
fn transform_expression_to_m(expr: &str) -> Result<String, String> {
    // Replace field references from Rust syntax (@Field) to M syntax ([Field])
    let m_expr = expr.replace('@', "[");

    Ok(m_expr)
}

/// Generate complete M code for a pipeline
pub fn generate_m_code(pipeline: &[TransformationStep], source_name: &str) -> String {
    let mut steps: Vec<String> = vec![];

    // Start with the source
    steps.push(format!("let Source = {} in", source_name));

    // Add each transformation step
    for step in pipeline {
        if let Ok(m_step) = step_to_m(step) {
            steps.push(m_step);
        } else {
            eprintln!("Warning: Could not convert step '{}' to M code", step.name);
        }
    }

    steps.join("\n  ")
}

/// Generate a human-readable diff between original and transformed data
pub fn generate_diff(
    original_schema: &[crate::data::ColumnSchema],
    transformed_schema: &[crate::data::ColumnSchema],
) -> String {
    let mut lines = vec![String::from("=== Schema Changes ===")];

    // Find removed columns
    let original_names: std::collections::HashSet<&str> =
        original_schema.iter().map(|c| c.name.as_str()).collect();

    for transformed in transformed_schema {
        if !original_names.contains(transformed.name.as_str()) {
            lines.push(format!("  + Added column: {}", transformed.name));
        }
    }

    // Find new columns that weren't there before
    let transformed_names: std::collections::HashSet<&str> =
        transformed_schema.iter().map(|c| c.name.as_str()).collect();

    for original in original_schema {
        if !transformed_names.contains(original.name.as_str()) {
            lines.push(format!("  - Removed column: {}", original.name));
        }
    }

    // Find type changes
    let orig_map: std::collections::HashMap<&str, &str> = original_schema
        .iter()
        .map(|c| (c.name.as_str(), c.data_type.as_str()))
        .collect();

    for transformed in transformed_schema {
        if let Some(orig) = orig_map.get(transformed.name.as_str()) {
            if orig != &transformed.data_type[..] {
                lines.push(format!(
                    "  ~ Type changed: {} from {} to {}",
                    transformed.name, orig, transformed.data_type
                ));
            }
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{Aggregation, AggregationOperation};

    #[test]
    fn test_filter_to_m() {
        let step = TransformationStep {
            id: "test".to_string(),
            name: "filter".to_string(),
            step_type: StepType::FilterRows("Revenue".to_string(), "> 1000".to_string()),
            parameters: Value::Null,
            output_schema: vec![],
        };

        let m = step_to_m(&step).unwrap();
        assert!(m.contains("Table.SelectRows"));
        assert!(m.contains("[Revenue] > 1000"));
    }

    #[test]
    fn test_select_to_m() {
        let step = TransformationStep {
            id: "test".to_string(),
            name: "select".to_string(),
            step_type: StepType::SelectColumns(vec!["Name".to_string(), "Revenue".to_string()]),
            parameters: Value::Null,
            output_schema: vec![],
        };

        let m = step_to_m(&step).unwrap();
        assert!(m.contains("Table.SelectColumns"));
    }
}
