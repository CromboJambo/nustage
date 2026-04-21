//! Nustage - Excel workbook CSV export module

use calamine::{Data, Reader, open_workbook_auto};

/// Export all sheets from an xlsx workbook to CSV files in the target directory
pub fn export_xlsx(
    path: &str,
    output_dir: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut workbook = open_workbook_auto(path)?;
    let sheet_names = workbook.sheet_names().to_owned();
    let mut csv_paths = Vec::new();

    for sheet_name in sheet_names {
        let sanitized = sheet_name.replace(' ', "_").replace('/', "-");
        let csv_path = format!("{}/{}.csv", output_dir, sanitized);

        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let rows: Vec<Vec<String>> = range
                .rows()
                .map(|row| {
                    row.iter()
                        .map(|v| match v {
                            Data::String(s) => s.to_string(),
                            Data::Empty => "".to_string(),
                            Data::Float(f) => f.to_string(),
                            Data::Int(i) => i.to_string(),
                            Data::Bool(b) => b.to_string(),
                            Data::Error(e) => e.to_string(),
                            Data::DateTime(_) => "".to_string(),
                            Data::DateTimeIso(s) => s.to_string(),
                            Data::DurationIso(s) => s.to_string(),
                        })
                        .collect::<Vec<String>>()
                })
                .collect();

            if rows.is_empty() {
                continue;
            }

            let header = rows[0].clone();
            let mut writer = csv::Writer::from_path(&csv_path)?;
            writer.write_record(&header)?;
            for row in rows.iter() {
                writer.write_record(row)?;
            }
            writer.flush()?;
        }

        csv_paths.push(csv_path);
    }

    Ok(csv_paths)
}

/// Export a single sheet from an xlsx workbook to CSV
pub fn export_single_sheet(
    path: &str,
    sheet: &str,
    output_dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut workbook = open_workbook_auto(path)?;
    let sanitized = sheet.replace(' ', "_").replace('/', "-");
    let csv_path = format!("{}/{}.csv", output_dir, sanitized);

    if let Ok(range) = workbook.worksheet_range(sheet) {
        let rows: Vec<Vec<String>> = range
            .rows()
            .map(|row| {
                row.iter()
                    .map(|v| match v {
                        Data::String(s) => s.to_string(),
                        Data::Empty => "".to_string(),
                        Data::Float(f) => f.to_string(),
                        Data::Int(i) => i.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::Error(e) => e.to_string(),
                        Data::DateTime(_) => "".to_string(),
                        Data::DateTimeIso(s) => s.to_string(),
                        Data::DurationIso(s) => s.to_string(),
                    })
                    .collect::<Vec<String>>()
            })
            .collect();

        if rows.is_empty() {
            return Ok(csv_path);
        }

        let header = rows[0].clone();
        let mut writer = csv::Writer::from_path(&csv_path)?;
        writer.write_record(&header)?;
        for row in rows.iter() {
            writer.write_record(row)?;
        }
        writer.flush()?;
    }

    Ok(csv_path)
}
