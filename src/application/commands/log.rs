use anyhow::Result;
use crate::config::config::Config;
use crate::structure::LogArgs;
use crate::application::auth;
use crate::application::dto::log::{LogResponse, LogLine, LogLevel, LogSource};
use colored::Colorize;
use reqwest::Client;
use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;

pub async fn log(args: &LogArgs, config: &Config) -> Result<()> {
    let jwt_token = auth::get_auth_token()?;
    let client = Client::new();
    
    if args.follow {
        return follow_logs(args, config, &client, &jwt_token).await;
    }
    
    let response = client
        .get(format!("{}/deploy/{}/logs", config.api_url, args.deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await?;

    if response.status().is_success() {
        let logs_text = response.text().await?;
        
        // Try to parse as structured JSON first
        if let Ok(log_response) = serde_json::from_str::<LogResponse>(&logs_text) {
            display_structured_logs(&log_response, args)?;
        } else {
            // Fallback to raw text display
            display_raw_logs(&logs_text, args)?;
        }
    } else {
        let error_text = response.text().await.unwrap_or_default();
        println!("{} Failed to fetch logs: {}", "❌".red(), error_text);
    }
    
    Ok(())
}

fn display_structured_logs(log_response: &LogResponse, args: &LogArgs) -> Result<()> {
    let mut lines = log_response.parse_logs();
    
    // Apply filters
    if args.stdout_only {
        lines.retain(|line| matches!(line.source, LogSource::Stdout));
    } else if args.stderr_only {
        lines.retain(|line| matches!(line.source, LogSource::Stderr));
    }
    
    // Apply line limit
    if lines.len() > args.lines {
        lines.truncate(args.lines);
        println!("{} Showing last {} lines (use -n to change)", "📄".yellow(), args.lines);
    }
    
    // Display header
    println!("{} Deployment Logs for: {}", "📋".blue(), args.deployment_id);
    println!("{}", "=".repeat(60).dimmed());
    
    if lines.is_empty() {
        println!("{} No logs available", "ℹ️".blue());
        return Ok(());
    }
    
    // Display logs with pagination if needed
    if lines.len() > 20 {
        display_with_pagination(lines, args)?;
    } else {
        display_logs_direct(lines, args)?;
    }
    
    Ok(())
}

fn display_raw_logs(logs_text: &str, args: &LogArgs) -> Result<()> {
    let lines: Vec<&str> = logs_text.lines().collect();
    
    println!("{} Deployment Logs for: {}", "📋".blue(), args.deployment_id);
    println!("{}", "=".repeat(60).dimmed());
    
    if lines.is_empty() {
        println!("{} No logs available", "ℹ️".blue());
        return Ok(());
    }
    
    let start_idx = if lines.len() > args.lines {
        lines.len() - args.lines
    } else {
        0
    };
    
    if start_idx > 0 {
        println!("{} Showing last {} lines (use -n to change)", "📄".yellow(), args.lines);
    }
    
    for line in &lines[start_idx..] {
        println!("{}", format_log_line_raw(line));
    }
    
    Ok(())
}

fn display_logs_direct(lines: Vec<LogLine>, args: &LogArgs) -> Result<()> {
    for line in lines {
        println!("{}", format_log_line(&line, args.timestamps));
    }
    Ok(())
}

fn display_with_pagination(lines: Vec<LogLine>, args: &LogArgs) -> Result<()> {
    let page_size = 20;
    let total_pages = (lines.len() + page_size - 1) / page_size;
    let mut current_page = total_pages; // Start from last page
    
    loop {
        let start_idx = (current_page - 1) * page_size;
        let end_idx = std::cmp::min(start_idx + page_size, lines.len());
        
        // Clear screen and show current page
        print!("\x1B[2J\x1B[1;1H");
        println!("{} Deployment Logs for: {}", "📋".blue(), args.deployment_id);
        println!("{}", "=".repeat(60).dimmed());
        println!("{} Page {}/{} ({} lines)", "📄".yellow(), current_page, total_pages, lines.len());
        println!("{}", "-".repeat(60).dimmed());
        
        for line in &lines[start_idx..end_idx] {
            println!("{}", format_log_line(line, args.timestamps));
        }
        
        println!("{}", "-".repeat(60).dimmed());
        println!("{} Press 'n' for next, 'p' for previous, 'q' to quit", "💡".cyan());
        
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().to_lowercase().as_str() {
            "n" | "next" => {
                if current_page < total_pages {
                    current_page += 1;
                }
            }
            "p" | "prev" | "previous" => {
                if current_page > 1 {
                    current_page -= 1;
                }
            }
            "q" | "quit" | "exit" => break,
            _ => {
                // Stay on current page
            }
        }
    }
    
    Ok(())
}

fn format_log_line(line: &LogLine, show_timestamps: bool) -> String {
    let mut formatted = String::new();
    
    // Add timestamp if requested and available
    if show_timestamps {
        if let Some(timestamp) = &line.timestamp {
            formatted.push_str(&format!("{} ", timestamp.dimmed()));
        }
    }
    
    // Add source indicator
    let source_indicator = match line.source {
        LogSource::Stdout => "📤".green(),
        LogSource::Stderr => "📥".red(),
    };
    formatted.push_str(&format!("{} ", source_indicator));
    
    // Add level indicator
    let level_indicator = match line.level {
        LogLevel::Error => "❌".red(),
        LogLevel::Warning => "⚠️".yellow(),
        LogLevel::Info => "ℹ️".blue(),
        LogLevel::Debug => "🐛".purple(),
        LogLevel::Trace => "🔍".dimmed(),
    };
    formatted.push_str(&format!("{} ", level_indicator));
    
    // Add content with appropriate color
    let content_color = match line.level {
        LogLevel::Error => line.content.red(),
        LogLevel::Warning => line.content.yellow(),
        LogLevel::Info => line.content.white(),
        LogLevel::Debug => line.content.purple(),
        LogLevel::Trace => line.content.dimmed(),
    };
    
    formatted.push_str(&content_color.to_string());
    
    formatted
}

fn format_log_line_raw(line: &str) -> String {
    if line.contains("ERROR") || line.contains("FATAL") {
        line.red().to_string()
    } else if line.contains("WARNING") || line.contains("WARN") {
        line.yellow().to_string()
    } else if line.contains("LOG:") {
        line.blue().to_string()
    } else {
        line.white().to_string()
    }
}

async fn follow_logs(args: &LogArgs, config: &Config, client: &Client, jwt_token: &str) -> Result<()> {
    println!("{} Following logs for deployment: {} (Press Ctrl+C to stop)", "👀".green(), args.deployment_id);
    println!("{}", "=".repeat(60).dimmed());
    
    let mut last_logs = String::new();
    
    loop {
        let response = client
            .get(format!("{}/deploy/{}/logs", config.api_url, args.deployment_id))
            .header("Authorization", format!("Bearer {}", jwt_token))
            .send()
            .await?;
        
        if response.status().is_success() {
            let current_logs = response.text().await?;
            
            if current_logs != last_logs {
                // Parse and display new logs
                if let Ok(log_response) = serde_json::from_str::<LogResponse>(&current_logs) {
                    let lines = log_response.parse_logs();
                    let new_lines = if last_logs.is_empty() {
                        lines
                    } else {
                        // Find new lines (simplified - in real implementation you'd track line counts)
                        lines.into_iter().skip_while(|_| last_logs.is_empty()).collect()
                    };
                    
                    for line in new_lines {
                        println!("{}", format_log_line(&line, args.timestamps));
                    }
                } else {
                    // Raw text fallback
                    if current_logs != last_logs {
                        println!("{}", format_log_line_raw(&current_logs));
                    }
                }
                
                last_logs = current_logs;
            }
        }
        
        sleep(Duration::from_secs(2)).await;
    }
}
