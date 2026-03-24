//! PowerQuery M Code Generator
//!
//! Converts canonical Nustage transformation steps into Power Query M.

use crate::data::ColumnSchema;
use crate::transformations::{Aggregation, AggregationOperation, StepType, TransformationStep};
use std::collections::{HashMap, HashSet};

/// Generate M code from a single transformation step.
pub fn step_to_m(step: &TransformationStep, input_name: &str) -> Result<String, String> {
    match &step.step_type {
        StepType::SelectColumns(columns) => Ok(format!(
            "Table.SelectColumns({}, {{{}}})",
            input_name,
            columns
                .iter()
                .map(|c| format!("\"{}\"", escape_m_string(c)))
                .collect::<Vec<_>>()
                .join(", ")
        )),
        StepType::FilterRows(column, condition) => Ok(format!(
            "Table.SelectRows({}, each {})",
            input_name,
            parse_condition_to_m(column, condition)?
        )),
        StepType::GroupBy(columns, aggregations) => Ok(format!(
            "Table.Group({}, {{{}}}, {{{}}})",
            input_name,
            columns
                .iter()
                .map(|c| format!("\"{}\"", escape_m_string(c)))
                .collect::<Vec<_>>()
                .join(", "),
            aggregations
                .iter()
                .map(aggregation_to_m)
                .collect::<Result<Vec<_>, _>>()?
                .join(", ")
        )),
        StepType::SortBy(columns, descending) => Ok(format!(
            "Table.Sort({}, {{{}}})",
            input_name,
            columns
                .iter()
                .map(|c| {
                    format!(
                        "{{\"{}\", Order.{}}}",
                        escape_m_string(c),
                        if *descending {
                            "Descending"
                        } else {
                            "Ascending"
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join(", ")
        )),
        StepType::RenameColumn(old_name, new_name) => Ok(format!(
            "Table.RenameColumns({}, {{{{\"{}\", \"{}\"}}}})",
            input_name,
            escape_m_string(old_name),
            escape_m_string(new_name)
        )),
        StepType::DropColumns(columns) => Ok(format!(
            "Table.RemoveColumns({}, {{{}}})",
            input_name,
            columns
                .iter()
                .map(|c| format!("\"{}\"", escape_m_string(c)))
                .collect::<Vec<_>>()
                .join(", ")
        )),
        StepType::CustomSql(sql) => Err(format!(
            "Custom SQL step requires manual Power Query translation: {}",
            sql
        )),
        StepType::AddColumn(name, expr) => Ok(format!(
            "Table.AddColumn({}, \"{}\", each {})",
            input_name,
            escape_m_string(name),
            transform_expression_to_m(expr)?
        )),
        StepType::RemoveDuplicates(all_columns) => {
            if *all_columns {
                Ok(format!("Table.Distinct({})", input_name))
            } else {
                Err("Column-scoped duplicate removal is not represented in the canonical step model"
                    .to_string())
            }
        }
    }
}

/// Generate complete M code for a pipeline.
pub fn generate_m_code(pipeline: &[TransformationStep], source_name: &str) -> String {
    let mut bindings = vec![format!("Source = {}", source_name)];
    let mut last_name = "Source".to_string();

    for (index, step) in pipeline.iter().enumerate() {
        let binding_name = step_binding_name(index, &step.name);
        match step_to_m(step, &last_name) {
            Ok(expr) => {
                bindings.push(format!("{} = {}", binding_name, expr));
                last_name = binding_name;
            }
            Err(err) => {
                bindings.push(format!(
                    "{} = {} /* skipped: {} */",
                    binding_name, last_name, err
                ));
                last_name = binding_name;
            }
        }
    }

    format!(
        "let\n    {}\nin\n    {}",
        bindings.join(",\n    "),
        last_name
    )
}

/// Generate a human-readable diff between original and transformed data.
pub fn generate_diff(
    original_schema: &[ColumnSchema],
    transformed_schema: &[ColumnSchema],
) -> String {
    let mut lines = vec![String::from("=== Schema Changes ===")];

    let original_names: HashSet<&str> = original_schema.iter().map(|c| c.name.as_str()).collect();
    let transformed_names: HashSet<&str> =
        transformed_schema.iter().map(|c| c.name.as_str()).collect();

    for transformed in transformed_schema {
        if !original_names.contains(transformed.name.as_str()) {
            lines.push(format!("  + Added column: {}", transformed.name));
        }
    }

    for original in original_schema {
        if !transformed_names.contains(original.name.as_str()) {
            lines.push(format!("  - Removed column: {}", original.name));
        }
    }

    let original_types: HashMap<&str, &str> = original_schema
        .iter()
        .map(|c| (c.name.as_str(), c.data_type.as_str()))
        .collect();

    for transformed in transformed_schema {
        if let Some(original_type) = original_types.get(transformed.name.as_str())
            && *original_type != transformed.data_type.as_str()
        {
            lines.push(format!(
                "  ~ Type changed: {} from {} to {}",
                transformed.name, original_type, transformed.data_type
            ));
        }
    }

    lines.join("\n")
}

fn aggregation_to_m(aggregation: &Aggregation) -> Result<String, String> {
    let function_name = match aggregation.operation {
        AggregationOperation::Sum => "List.Sum",
        AggregationOperation::Mean => "List.Average",
        AggregationOperation::Count => "List.Count",
        AggregationOperation::Min => "List.Min",
        AggregationOperation::Max => "List.Max",
        AggregationOperation::First => "List.First",
        AggregationOperation::Last => "List.Last",
        AggregationOperation::StdDev => "List.StandardDeviation",
        AggregationOperation::Variance => "List.Variance",
    };

    let label = format!("{:?}_{}", aggregation.operation, aggregation.column);
    Ok(format!(
        "{{\"{}\", each {}([{}])}}",
        escape_m_string(&label),
        function_name,
        aggregation.column
    ))
}

/// Parse a condition and convert it to M.
fn parse_condition_to_m(column: &str, condition: &str) -> Result<String, String> {
    let trimmed = condition.trim();
    let operators = [
        ">=", "<=", "!=", "<>", "==", "=", ">", "<", "contains", "in",
    ];

    let matched = operators
        .iter()
        .find_map(|op| trimmed.strip_prefix(op).map(|rest| (*op, rest.trim())));

    let (operator, raw_value) = matched.ok_or_else(|| {
        format!(
            "Unsupported condition format for column '{}': {}",
            column, condition
        )
    })?;

    let value = format_m_value(raw_value);
    let escaped_column = escape_m_identifier(column);

    match operator {
        "contains" => Ok(format!("Text.Contains([{}], {})", escaped_column, value)),
        "in" => Ok(format!("List.Contains({}, [{}])", value, escaped_column)),
        "==" => Ok(format!("[{}] = {}", escaped_column, value)),
        "!=" => Ok(format!("[{}] <> {}", escaped_column, value)),
        "<>" => Ok(format!("[{}] <> {}", escaped_column, value)),
        _ => Ok(format!("[{}] {} {}", escaped_column, operator, value)),
    }
}

/// Transform an expression to M language, converting `@Field` to `[Field]`.
fn transform_expression_to_m(expr: &str) -> Result<String, String> {
    let mut result = String::with_capacity(expr.len());
    let chars: Vec<char> = expr.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        if chars[index] == '@' {
            index += 1;
            let start = index;
            while index < chars.len() && (chars[index].is_alphanumeric() || chars[index] == '_') {
                index += 1;
            }

            let field = chars[start..index]
                .iter()
                .collect::<String>()
                .trim()
                .to_string();
            if field.is_empty() {
                return Err(format!("Invalid field reference in expression: {}", expr));
            }

            result.push('[');
            result.push_str(&field);
            result.push(']');
            continue;
        }

        result.push(chars[index]);
        index += 1;
    }

    Ok(result)
}

fn format_m_value(raw_value: &str) -> String {
    let trimmed = raw_value.trim();
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let inner = &trimmed[1..trimmed.len() - 1];
        let values = inner
            .split(',')
            .map(|part| format_m_value(part.trim()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{{{}}}", values)
    } else if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        format!("\"{}\"", escape_m_string(&trimmed[1..trimmed.len() - 1]))
    } else if trimmed.parse::<f64>().is_ok()
        || trimmed.eq_ignore_ascii_case("true")
        || trimmed.eq_ignore_ascii_case("false")
        || trimmed.eq_ignore_ascii_case("null")
    {
        trimmed.to_string()
    } else {
        format!("\"{}\"", escape_m_string(trimmed.trim_matches('\'')))
    }
}

fn escape_m_string(input: &str) -> String {
    input.replace('"', "\"\"")
}

fn escape_m_identifier(input: &str) -> String {
    input.to_string()
}

fn step_binding_name(index: usize, name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    let trimmed = sanitized.trim_matches('_');
    if trimmed.is_empty() {
        format!("Step{}", index + 1)
    } else {
        format!("Step{}_{}", index + 1, trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{Aggregation, AggregationOperation};

    fn base_step(step_type: StepType) -> TransformationStep {
        TransformationStep {
            id: "test".to_string(),
            name: "step".to_string(),
            step_type,
            parameters: HashMap::new(),
            output_schema: vec![],
        }
    }

    #[test]
    fn test_filter_to_m() {
        let step = base_step(StepType::FilterRows(
            "Revenue".to_string(),
            "> 1000".to_string(),
        ));

        let m = step_to_m(&step, "Source").unwrap();
        assert!(m.contains("Table.SelectRows"));
        assert!(m.contains("[Revenue] > 1000"));
    }

    #[test]
    fn test_select_to_m() {
        let step = base_step(StepType::SelectColumns(vec![
            "Name".to_string(),
            "Revenue".to_string(),
        ]));

        let m = step_to_m(&step, "Source").unwrap();
        assert!(m.contains("Table.SelectColumns"));
        assert!(m.contains("\"Name\""));
    }

    #[test]
    fn test_add_column_expression_to_m() {
        let step = base_step(StepType::AddColumn(
            "Margin".to_string(),
            "@Revenue - @Cost".to_string(),
        ));

        let m = step_to_m(&step, "Source").unwrap();
        assert!(m.contains("[Revenue] - [Cost]"));
    }

    #[test]
    fn test_group_by_to_m() {
        let step = base_step(StepType::GroupBy(
            vec!["Region".to_string()],
            vec![Aggregation {
                column: "Revenue".to_string(),
                operation: AggregationOperation::Sum,
            }],
        ));

        let m = step_to_m(&step, "Source").unwrap();
        assert!(m.contains("Table.Group"));
        assert!(m.contains("List.Sum"));
    }

    #[test]
    fn test_generate_m_code_pipeline() {
        let pipeline = vec![
            base_step(StepType::FilterRows(
                "Revenue".to_string(),
                "> 1000".to_string(),
            )),
            base_step(StepType::SortBy(vec!["Revenue".to_string()], true)),
        ];

        let m = generate_m_code(
            &pipeline,
            "Excel.CurrentWorkbook(){[Name=\"Table1\"]}[Content]",
        );
        assert!(m.starts_with("let"));
        assert!(m.contains("Step1_step"));
        assert!(m.contains("in"));
    }
}
