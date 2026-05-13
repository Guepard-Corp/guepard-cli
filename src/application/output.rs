use std::io::{self, Write};

use serde::Serialize;
use serde_json;
use tabled::{settings::Style, Table};

/// Write a full line to stdout; exit 0 on broken pipe (e.g. `| head`) instead of panicking.
fn writeln_stdout_line(s: &str) {
    let mut out = io::stdout().lock();
    match writeln!(out, "{s}") {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => std::process::exit(0),
        Err(_) => std::process::exit(0),
    }
}

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            _ => OutputFormat::Table,
        }
    }
}

/// Print data in the specified format
pub fn print_output<T: Serialize>(data: &T, format: OutputFormat) {
    match format {
        OutputFormat::Json => match serde_json::to_string_pretty(data) {
            Ok(json) => writeln_stdout_line(&json),
            Err(e) => eprintln!("❌ Failed to serialize output: {}", e),
        },
        OutputFormat::Table => {
            // For table format, we'll let the caller handle it
            // This is a fallback for simple serializable types
        }
    }
}

/// Print a table or JSON based on format
pub fn print_table_or_json<T: Serialize + tabled::Tabled>(rows: Vec<T>, format: OutputFormat) {
    match format {
        OutputFormat::Json => match serde_json::to_string_pretty(&rows) {
            Ok(json) => writeln_stdout_line(&json),
            Err(e) => eprintln!("❌ Failed to serialize output: {}", e),
        },
        OutputFormat::Table => {
            let table = Table::new(rows).with(Style::rounded()).to_string();
            writeln_stdout_line(&table);
        }
    }
}

/// Print a single row as table or JSON
pub fn print_row_or_json<T: Serialize + tabled::Tabled>(row: T, format: OutputFormat) {
    print_table_or_json(vec![row], format);
}

/// Print any serializable data as JSON (for non-table data)
pub fn print_json<T: Serialize>(data: &T) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => writeln_stdout_line(&json),
        Err(e) => eprintln!("❌ Failed to serialize output: {}", e),
    }
}
