use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LogResponse {
    pub stdout_logs: String,
    pub stderr_logs: String,
}

#[derive(Debug, Serialize)]
pub struct LogLine {
    pub timestamp: Option<String>,
    pub level: LogLevel,
    pub content: String,
    pub source: LogSource,
}

#[derive(Debug, Serialize)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
    Trace,
}

#[derive(Debug, Serialize)]
pub enum LogSource {
    Stdout,
    Stderr,
}

impl LogResponse {
    pub fn parse_logs(&self) -> Vec<LogLine> {
        let mut lines = Vec::new();
        
        // Parse stdout logs
        for line in self.stdout_logs.lines() {
            if !line.trim().is_empty() {
                lines.push(LogLine {
                    timestamp: Self::extract_timestamp(line),
                    level: Self::extract_level(line),
                    content: line.to_string(),
                    source: LogSource::Stdout,
                });
            }
        }
        
        // Parse stderr logs
        for line in self.stderr_logs.lines() {
            if !line.trim().is_empty() {
                lines.push(LogLine {
                    timestamp: Self::extract_timestamp(line),
                    level: Self::extract_level(line),
                    content: line.to_string(),
                    source: LogSource::Stderr,
                });
            }
        }
        
        lines.sort_by(|a, b| {
            match (&a.timestamp, &b.timestamp) {
                (Some(ts_a), Some(ts_b)) => ts_a.cmp(ts_b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
        
        lines
    }
    
    fn extract_timestamp(line: &str) -> Option<String> {
        // Look for PostgreSQL timestamp format: "2025-10-08 08:52:16.178 UTC"
        if let Some(start) = line.find("2025-") {
            if let Some(end) = line[start..].find(" UTC") {
                return Some(line[start..start + end + 4].to_string());
            }
        }
        None
    }
    
    fn extract_level(line: &str) -> LogLevel {
        if line.contains("ERROR") || line.contains("FATAL") {
            LogLevel::Error
        } else if line.contains("WARNING") || line.contains("WARN") {
            LogLevel::Warning
        } else if line.contains("DEBUG") {
            LogLevel::Debug
        } else if line.contains("TRACE") {
            LogLevel::Trace
        } else {
            LogLevel::Info
        }
    }
}
