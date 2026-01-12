use anyhow::Result;
use crate::config::config::Config;
use crate::structure::ListArgs;
use crate::application::services::{deploy, branch, commit, clone};
use colored::Colorize;
use std::collections::HashSet;

// Available columns for each resource type
const DEPLOYMENT_COLUMNS: &[&str] = &[
    "id", "name", "repository", "provider", "version", "status", "fqdn", 
    "region", "datacenter", "created", "type", "port", "connection"
];

const PERFORMANCE_COLUMNS: &[&str] = &["id", "label_name", "database_provider", "database_version", "min_cpu", "min_memory", "is_default"];

const BRANCH_COLUMNS: &[&str] = &["id", "branch_name", "label_name", "job_status", "snapshot_id"];

const COMMIT_COLUMNS: &[&str] = &["id", "name", "message", "created", "dataset_id", "parent_id", "status", "type"];

const CLONE_COLUMNS: &[&str] = &["id", "name", "status", "snapshot_parent", "created", "repository_name", "fqdn", "provider", "version", "region", "connection"];

use crate::application::output::{OutputFormat, print_json};

pub async fn list(args: &ListArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    match args.resource.as_str() {
        "deployments" => list_deployments(args, config, output_format).await,
        "branches" => list_branches(args, config, output_format).await,
        "commits" => list_commits(args, config, output_format).await,
        "clones" => list_clones(args, config, output_format).await,
        "performance" => list_performance(args, config, output_format).await,
        _ => {
            if output_format == OutputFormat::Table {
                println!("{} Unknown resource: {}", "❌".red(), args.resource);
                println!("Available resources: deployments, branches, commits, clones, performance");
            } else {
                print_json(&serde_json::json!({
                    "error": format!("Unknown resource: {}", args.resource),
                    "available_resources": ["deployments", "branches", "commits", "clones", "performance"]
                }));
            }
            Ok(())
        }
    }
}

fn parse_columns(columns_str: &Option<String>, available_columns: &[&str]) -> Vec<String> {
    if let Some(cols) = columns_str {
        let selected: HashSet<String> = cols.split(',')
            .map(|s| s.trim().to_lowercase())
            .collect();
        
        available_columns.iter()
            .filter(|col| selected.contains(&col.to_string()))
            .map(|s| s.to_string())
            .collect()
    } else {
        // Default columns
        match available_columns {
            DEPLOYMENT_COLUMNS => vec!["id".to_string(), "name".to_string(), "repository".to_string(), 
                                      "provider".to_string(), "version".to_string(), "status".to_string(), "fqdn".to_string()],
            PERFORMANCE_COLUMNS => vec!["id".to_string(), "label_name".to_string(), "database_provider".to_string(), "database_version".to_string(), "is_default".to_string()],
            BRANCH_COLUMNS => vec!["id".to_string(), "branch_name".to_string(), "label_name".to_string(), "job_status".to_string(), "snapshot_id".to_string()],
            COMMIT_COLUMNS => vec!["id".to_string(), "name".to_string(), "message".to_string(), "created".to_string(), "dataset_id".to_string(), "parent_id".to_string()],
            CLONE_COLUMNS => vec!["id".to_string(), "name".to_string(), "status".to_string(), "repository_name".to_string(), "fqdn".to_string(), "snapshot_parent".to_string()],
            _ => available_columns.iter().map(|s| s.to_string()).collect(),
        }
    }
}

fn show_available_columns(resource: &str) {
    match resource {
        "deployments" => {
            println!("{} Available columns for deployments:", "ℹ️".blue());
            println!("{}", DEPLOYMENT_COLUMNS.join(", "));
        },
        "performance" => {
            println!("{} Available columns for performance:", "ℹ️".blue());
            println!("{}", PERFORMANCE_COLUMNS.join(", "));
        },
        "branches" => {
            println!("{} Available columns for branches:", "ℹ️".blue());
            println!("{}", BRANCH_COLUMNS.join(", "));
        },
        "commits" => {
            println!("{} Available columns for commits:", "ℹ️".blue());
            println!("{}", COMMIT_COLUMNS.join(", "));
        },
        "clones" => {
            println!("{} Available columns for clones:", "ℹ️".blue());
            println!("{}", CLONE_COLUMNS.join(", "));
        },
        _ => {}
    }
}

async fn list_performance(args: &ListArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let mut profiles = crate::application::services::performance::list_performance_profiles(config).await?;
    
    if profiles.is_empty() {
        if output_format == OutputFormat::Json {
            print_json(&serde_json::json!([]));
        } else {
            println!("{} No performance profiles found", "ℹ️".blue());
        }
        return Ok(());
    }
    
    // Apply limit if specified
    let total_count = profiles.len();
    if let Some(limit) = args.limit {
        profiles.truncate(limit);
    }
    
    let selected_columns = parse_columns(&args.columns, PERFORMANCE_COLUMNS);
    
    if selected_columns.is_empty() {
        println!("{} No valid columns selected. Available columns:", "❌".red());
        show_available_columns("performance");
        return Ok(());
    }
    
    // Create dynamic rows based on selected columns
    let mut rows = Vec::new();
    for profile in profiles {
        let mut row_data = std::collections::HashMap::new();
        
        for col in &selected_columns {
            let value = match col.as_str() {
                "id" => profile.id.clone(),
                "label_name" => profile.label_name.clone(),
                "database_provider" => profile.database_provider.clone(),
                "database_version" => profile.database_version.clone(),
                "min_cpu" => profile.min_cpu.to_string(),
                "min_memory" => profile.min_memory.to_string(),
                "is_default" => profile.is_default.to_string(),
                _ => "".to_string(),
            };
            row_data.insert(col.clone(), value);
        }
        rows.push(row_data);
    }
    
    if output_format == OutputFormat::Json {
        print_json(&rows);
        return Ok(());
    }

    println!("{} Found {} performance profiles{}", 
        "✅".green(), 
        total_count,
        if let Some(limit) = args.limit {
            if limit < total_count {
                format!(" (showing first {})", limit)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    );
    
    // Display the table with selected columns
    display_dynamic_table(rows, &selected_columns);
    Ok(())
}

async fn list_clones(args: &ListArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let deployment_id = args.deployment_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Deployment ID is required for listing clones. Use -x <deployment_id>"))?;
    
    let mut clones = clone::list_clones(deployment_id, config).await?;
    
    if clones.is_empty() {
        if output_format == OutputFormat::Json {
            print_json(&serde_json::json!([]));
        } else {
            println!("{} No clones found for deployment: {}", "ℹ️".blue(), deployment_id);
        }
        return Ok(());
    }
    
    // Apply limit if specified
    let total_count = clones.len();
    if let Some(limit) = args.limit {
        clones.truncate(limit);
    }
    
    let selected_columns = parse_columns(&args.columns, CLONE_COLUMNS);
    
    if selected_columns.is_empty() {
        println!("{} No valid columns selected. Available columns:", "❌".red());
        show_available_columns("clones");
        return Ok(());
    }
    
    // Create dynamic rows based on selected columns
    let mut rows = Vec::new();
    for clone in &clones {
        let mut row_data = std::collections::HashMap::new();
        
        let connection = clone.connection_string.clone().unwrap_or_else(|| {
            // Fallback to constructing a basic URI if connection_string is missing
            format!("postgresql://{}:{}@{}:5432/{}", 
                clone.database_username.as_deref().unwrap_or("user"),
                if clone.database_password.is_some() { "****" } else { "pass" },
                clone.fqdn, 
                clone.repository_name
            )
        });

        for col in &selected_columns {
            let value = match col.as_str() {
                "id" => clone.id.clone(),
                "name" => clone.name.clone(),
                "status" => clone.status.clone(),
                "snapshot_parent" => clone.snapshot_parent.clone().unwrap_or_default(),
                "created" => clone.created_date.clone(),
                "repository_name" => clone.repository_name.clone(),
                "fqdn" => clone.fqdn.clone(),
                "provider" => clone.database_provider.clone(),
                "version" => clone.database_version.clone(),
                "region" => clone.region.clone().unwrap_or_default(),
                "connection" => connection.clone(),
                _ => "".to_string(),
            };
            row_data.insert(col.clone(), value);
        }
        rows.push(row_data);
    }
    
    if output_format == OutputFormat::Json {
        print_json(&rows);
        return Ok(());
    }

    println!("{} Found {} clones for deployment: {}{}", 
        "✅".green(), 
        total_count,
        deployment_id,
        if let Some(limit) = args.limit {
            if limit < total_count {
                format!(" (showing first {})", limit)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    );
    
    display_dynamic_table(rows, &selected_columns);
    Ok(())
}

async fn list_deployments(args: &ListArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let mut deployments = deploy::list_deployments(config).await?;
    
    if deployments.is_empty() {
        if output_format == OutputFormat::Json {
            print_json(&serde_json::json!([]));
        } else {
            println!("{} No deployments found", "ℹ️".blue());
        }
        return Ok(());
    }
    
    // Apply limit if specified
    let total_count = deployments.len();
    if let Some(limit) = args.limit {
        deployments.truncate(limit);
    }
    
    let selected_columns = parse_columns(&args.columns, DEPLOYMENT_COLUMNS);
    
    if selected_columns.is_empty() {
        println!("{} No valid columns selected. Available columns:", "❌".red());
        show_available_columns("deployments");
        return Ok(());
    }
    
    // Create dynamic rows based on selected columns
    let mut rows = Vec::new();
    for deployment in deployments {
        let mut row_data = std::collections::HashMap::new();
        
        for col in &selected_columns {
            let value = match col.as_str() {
                "id" => deployment.id.clone(),
                "name" => deployment.name.clone(),
                "repository" => deployment.repository_name.clone(),
                "provider" => deployment.database_provider.clone(),
                "version" => deployment.database_version.clone(),
                "status" => deployment.status.clone(),
                "fqdn" => deployment.fqdn.clone(),
                "region" => deployment.region.clone(),
                "datacenter" => deployment.datacenter.clone(),
                "created" => deployment.created_date.clone(),
                "type" => deployment.deployment_type.clone(),
                "port" => deployment.port.map(|p| p.to_string()).unwrap_or_default(),
                "connection" => deployment.connection_string.as_ref().map(|s| s.clone()).unwrap_or_default(),
                _ => "".to_string(),
            };
            row_data.insert(col.clone(), value);
        }
        rows.push(row_data);
    }
    
    if output_format == OutputFormat::Json {
        print_json(&rows);
        return Ok(());
    }

    println!("{} Found {} deployments{}", 
        "✅".green(), 
        total_count,
        if let Some(limit) = args.limit {
            if limit < total_count {
                format!(" (showing first {})", limit)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    );
    
    // Display the table with selected columns
    display_dynamic_table(rows, &selected_columns);
    Ok(())
}

async fn list_branches(args: &ListArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let deployment_id = args.deployment_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Deployment ID is required for listing branches. Use -x <deployment_id>"))?;
    
    let mut branches = branch::list_branches(deployment_id, config).await?;
    
    if branches.is_empty() {
        if output_format == OutputFormat::Json {
            print_json(&serde_json::json!([]));
        } else {
            println!("{} No branches found for deployment: {}", "ℹ️".blue(), deployment_id);
        }
        return Ok(());
    }
    
    // Apply limit if specified
    let total_count = branches.len();
    if let Some(limit) = args.limit {
        branches.truncate(limit);
    }
    
    let selected_columns = parse_columns(&args.columns, BRANCH_COLUMNS);
    
    if selected_columns.is_empty() {
        println!("{} No valid columns selected. Available columns:", "❌".red());
        show_available_columns("branches");
        return Ok(());
    }
    
    // Create dynamic rows based on selected columns
    let mut rows = Vec::new();
    for branch in branches {
        let mut row_data = std::collections::HashMap::new();
        
        for col in &selected_columns {
            let value = match col.as_str() {
                "id" => branch.id.clone(),
                "branch_name" => branch.branch_name.as_ref().map(|s| s.clone()).unwrap_or_default(),
                "label_name" => branch.label_name.as_ref().map(|s| s.clone()).unwrap_or_default(),
                "job_status" => branch.job_status.as_ref().map(|s| s.clone()).unwrap_or_default(),
                "snapshot_id" => branch.snapshot_id.clone(),
                _ => "".to_string(),
            };
            row_data.insert(col.clone(), value);
        }
        rows.push(row_data);
    }
    
    if output_format == OutputFormat::Json {
        print_json(&rows);
        return Ok(());
    }

    println!("{} Found {} branches for deployment: {}{}", 
        "✅".green(), 
        total_count,
        deployment_id,
        if let Some(limit) = args.limit {
            if limit < total_count {
                format!(" (showing first {})", limit)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    );
    display_dynamic_table(rows, &selected_columns);
    Ok(())
}

async fn list_commits(args: &ListArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let deployment_id = args.deployment_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Deployment ID is required for listing commits. Use -x <deployment_id>"))?;
    
    let mut commits = commit::list_all_commits(deployment_id, config).await?;
    
    // Filter out AUTO SNAPs unless -a flag is used
    if !args.all {
        commits.retain(|commit| !commit.snapshot_comment.contains("AUTO SNAP"));
    }
    
    if commits.is_empty() {
        if output_format == OutputFormat::Json {
            print_json(&serde_json::json!([]));
        } else {
            println!("{} No commits found for deployment: {}", "ℹ️".blue(), deployment_id);
        }
        return Ok(());
    }
    
    // Sort commits by creation date (oldest first for proper graph)
    commits.sort_by(|a, b| a.created_date.cmp(&b.created_date));
    
    // Apply limit if specified
    let total_count = commits.len();
    if let Some(limit) = args.limit {
        commits.truncate(limit);
    }
    
    // Check if user wants git graph format
    if args.graph && output_format == OutputFormat::Table {
        display_git_graph(&commits, deployment_id, config).await?;
    } else {
        let selected_columns = parse_columns(&args.columns, COMMIT_COLUMNS);
        
        if selected_columns.is_empty() {
            println!("{} No valid columns selected. Available columns:", "❌".red());
            show_available_columns("commits");
            return Ok(());
        }
        
        // Create dynamic rows based on selected columns
        let mut rows = Vec::new();
        for commit in commits {
            let mut row_data = std::collections::HashMap::new();
            
            for col in &selected_columns {
                let value = match col.as_str() {
                    "id" => commit.id.clone(),
                    "name" => commit.name.clone(),
                    "message" => commit.snapshot_comment.clone(),
                    "created" => commit.created_date.clone(),
                    "dataset_id" => commit.dataset_id.as_ref().map(|s| s.clone()).unwrap_or_default(),
                    "parent_id" => commit.parent_id.as_ref().map(|s| s.clone()).unwrap_or_default(),
                    "status" => commit.status.clone(),
                    "type" => commit.snapshot_type.clone(),
                    _ => "".to_string(),
                };
                row_data.insert(col.clone(), value);
            }
            rows.push(row_data);
        }
        
        if output_format == OutputFormat::Json {
            print_json(&rows);
            return Ok(());
        }

        println!("{} Found {} commits for deployment: {}{}", 
            "✅".green(), 
            total_count,
            deployment_id,
            if let Some(limit) = args.limit {
                if limit < total_count {
                    format!(" (showing first {})", limit)
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        );
        display_dynamic_table(rows, &selected_columns);
    }
    Ok(())
}

fn display_dynamic_table(rows: Vec<std::collections::HashMap<String, String>>, columns: &[String]) {
    if rows.is_empty() {
        return;
    }
    
    // Create a simple but properly formatted table
    let mut table_string = String::new();
    
    // Calculate column widths more efficiently
    let mut col_widths = Vec::new();
    for col in columns {
        let header_width = col.to_uppercase().len(); // Use uppercase header length
        let mut max_width = header_width;
        for row in &rows {
            if let Some(value) = row.get(col) {
                max_width = max_width.max(value.len());
            }
        }
        // Content width + 2 spaces for padding (no extra border space)
        col_widths.push((max_width + 2).max(4)); // Minimum width of 4
    }
    
    // Create header
    table_string.push_str("┌");
    for (i, width) in col_widths.iter().enumerate() {
        for _ in 0..*width {
            table_string.push('─');
        }
        if i < col_widths.len() - 1 {
            table_string.push_str("┬");
        }
    }
    table_string.push_str("┐\n");
    
    // Add header row
    table_string.push_str("│");
    for (i, col) in columns.iter().enumerate() {
        let header = col.to_uppercase();
        table_string.push_str(&format!(" {:<width$} │", header, width = col_widths[i] - 2));
    }
    table_string.push_str("\n");
    
    // Add separator
    table_string.push_str("├");
    for (i, width) in col_widths.iter().enumerate() {
        for _ in 0..*width {
            table_string.push('─');
        }
        if i < col_widths.len() - 1 {
            table_string.push_str("┼");
        }
    }
    table_string.push_str("┤\n");
    
    // Add data rows
    for row in rows {
        table_string.push_str("│");
        for (i, col) in columns.iter().enumerate() {
            let empty = String::new();
            let value = row.get(col).unwrap_or(&empty);
            table_string.push_str(&format!(" {:<width$} │", value, width = col_widths[i] - 2));
        }
        table_string.push_str("\n");
    }
    
    // Add footer
    table_string.push_str("└");
    for (i, width) in col_widths.iter().enumerate() {
        for _ in 0..*width {
            table_string.push('─');
        }
        if i < col_widths.len() - 1 {
            table_string.push_str("┴");
        }
    }
    table_string.push_str("┘\n");
    
    println!("{}", table_string);
}

pub async fn display_git_graph(commits: &[crate::application::dto::commit::GetCommitResponse], deployment_id: &str, config: &Config) -> Result<()> {
    println!("{} Found {} commits for deployment: {}", "✅".green(), commits.len(), deployment_id);
    println!();
    
    // Print legend/explanation (git-style)
    println!("{} Graph Legend:", "📖".blue());
    println!("  {} Commit marker", "*".green());
    println!("  {} Link between commits on same branch", "|".green());
    println!("  {} Branch splits to left", "\\".green());
    println!("  {} Branch splits to right", "/".green());
    println!("  {} Active commit (hash in red brackets)", "[hash]".red());
    println!();
    
    // Get branch information to map dataset_id to branch names
    let branches = match branch::list_branches(deployment_id, config).await {
        Ok(branches) => branches,
        Err(_) => {
            // If we can't get branch info, fall back to simple display
            display_simple_git_graph(commits);
            return Ok(());
        }
    };
    
    // Get current compute snapshot to show pin
    let current_compute_snapshot = match crate::application::services::compute::list_compute(deployment_id, config).await {
        Ok(compute_result) => {
            // Find the branch that compute is attached to and get its snapshot
            branches.iter()
                .find(|b| b.id == compute_result.attached_branch)
                .map(|b| b.snapshot_id.clone())
        },
        Err(_) => None,
    };
    
    // Create a mapping from dataset_id to branch name
    let mut branch_names: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for branch in &branches {
        let dataset_id = branch.id.clone();
        let branch_name = branch.branch_name.as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(|| branch.label_name.as_ref().map(|s| s.clone()).unwrap_or_else(|| dataset_id.clone()));
        branch_names.insert(dataset_id, branch_name);
    }
    
    // Sort all commits by date (newest first - descending)
    let mut sorted_commits: Vec<_> = commits.iter().collect();
    sorted_commits.sort_by(|a, b| b.created_date.cmp(&a.created_date));
    
    // Create git-style tree visualization with proper table padding
    let mut branch_positions: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut current_positions: Vec<Option<String>> = Vec::new();
    let mut max_width = 0;
    
    // Find the last commit for each branch
    let mut last_commits_per_branch: std::collections::HashMap<String, &crate::application::dto::commit::GetCommitResponse> = std::collections::HashMap::new();
    for commit in &sorted_commits {
        let branch_id = commit.dataset_id.as_ref()
            .map(|id| id.clone())
            .unwrap_or_else(|| "main".to_string());
        last_commits_per_branch.insert(branch_id, commit);
    }
    
    // Calculate column widths for proper alignment
    let mut max_hash_width = 0;
    let mut max_branch_name_width = 0;
    let mut max_message_width = 0;
    let mut max_date_width = 0;
    
    for commit in &sorted_commits {
        let branch_id = commit.dataset_id.as_ref()
            .map(|id| id.clone())
            .unwrap_or_else(|| "main".to_string());
        
        let branch_name = branch_names.get(&branch_id)
            .map(|s| s.as_str())
            .unwrap_or_else(|| if branch_id.as_str() == "main" { "main" } else { &branch_id[..8] });
        
        // Find or create position for this branch
        let position = if let Some(&pos) = branch_positions.get(&branch_id) {
            pos
        } else {
            // Find first available position
            let pos = current_positions.iter().position(|p| p.is_none()).unwrap_or(current_positions.len());
            if pos >= current_positions.len() {
                current_positions.resize(pos + 1, None);
            }
            branch_positions.insert(branch_id.clone(), pos);
            pos
        };
        
        // Update current positions
        current_positions[position] = Some(branch_id.clone());
        max_width = max_width.max(position + 1);
        
        // Calculate hash width (always 8 for hash)
        max_hash_width = max_hash_width.max(8);
        
        // Calculate max branch name width (for last commits)
        let is_last_commit = last_commits_per_branch.get(&branch_id).map(|last_commit| last_commit.id == commit.id).unwrap_or(false);
        if is_last_commit {
            max_branch_name_width = max_branch_name_width.max(branch_name.len());
        }
        
        // Calculate message width (without emoji, emojis are 2 bytes but display as 1 char)
        let message_width = commit.snapshot_comment.len();
        max_message_width = max_message_width.max(message_width);
        
        // Calculate date+time width (format: "YYYY-MM-DD HH:MM:SS")
        let date_time_str = if commit.created_date.contains('T') {
            let parts: Vec<&str> = commit.created_date.split('T').collect();
            if parts.len() == 2 {
                let date_part = parts[0];
                let time_part = parts[1].split('.').next().unwrap_or(parts[1]); // Remove milliseconds if present
                let time_short = time_part.split(':').take(3).collect::<Vec<&str>>().join(":"); // HH:MM:SS
                format!("{} {}", date_part, time_short)
            } else {
                commit.created_date.clone()
            }
        } else {
            commit.created_date.clone()
        };
        max_date_width = max_date_width.max(date_time_str.len());
    }
    
    // Calculate tree line width after all positions are determined
    // Each position takes 1 character ("*", "|*", "|", "\", "/", " ")
    let max_tree_width = max_width;
    
    // Use fixed column widths for better alignment
    let tree_width = max_tree_width.max(4); // Minimum tree width (ensure at least 4 chars)
    let hash_width = 10; // Fixed width for hash column (centered, accommodates brackets)
    // Branch column: now first column, just branch name
    let branch_width = if max_branch_name_width > 0 {
        max_branch_name_width.max(15) // Minimum 15 chars for branch name column
    } else {
        15 // Minimum space for branch column
    };
    // Message width is no longer fixed - messages display in full
    let date_width = max_date_width.max(19) + 2; // "YYYY-MM-DD HH:MM:SS" = 19 chars + padding
    
    // Display commits with proper table alignment
    // Structure: Each commit gets a row with "*", then a link row with "|", "\", "/"
    for (commit_idx, commit) in sorted_commits.iter().enumerate() {
        let branch_id = commit.dataset_id.as_ref()
            .map(|id| id.clone())
            .unwrap_or_else(|| "main".to_string());
        
        let branch_name = branch_names.get(&branch_id)
            .map(|s| s.as_str())
            .unwrap_or_else(|| if branch_id.as_str() == "main" { "main" } else { &branch_id[..8] });
        
        let position = branch_positions.get(&branch_id).unwrap();
        
        // Check if this is the current compute snapshot
        let is_current_compute = current_compute_snapshot.as_ref().map(|snapshot_id| snapshot_id == &commit.id).unwrap_or(false);
        
        // Color coding based on commit type
        let commit_symbol = if commit.snapshot_comment.contains("AUTO SNAP") {
            "🔧"
        } else if commit.snapshot_comment.contains("deleted") {
            "🗑️"
        } else if commit.snapshot_comment.contains("inserted") {
            "➕"
        } else {
            "•"
        };
        
        // Parse and format date with time (including seconds)
        let date_time = if commit.created_date.contains('T') {
            // Format: "2025-12-18T14:30:00" -> "2025-12-18 14:30:00"
            let parts: Vec<&str> = commit.created_date.split('T').collect();
            if parts.len() == 2 {
                let date_part = parts[0];
                let time_part = parts[1].split('.').next().unwrap_or(parts[1]); // Remove milliseconds if present
                let time_full = time_part.split(':').take(3).collect::<Vec<&str>>().join(":"); // HH:MM:SS
                format!("{} {}", date_part, time_full)
            } else {
                commit.created_date.clone()
            }
        } else {
            commit.created_date.clone()
        };
        
        // Check if this is the last commit for this branch
        let is_last_commit = last_commits_per_branch.get(&branch_id).map(|last_commit| last_commit.id == commit.id).unwrap_or(false);
        
        // Format date column (first column) - fixed width, left-aligned
        let date_col = format!("{:<date_w$}", date_time.dimmed(), date_w = date_width);
        
        // Format branch column (second column) - fixed width
        let branch_col = if is_last_commit {
            // Show branch name for the latest commit on each branch
            format!("{:<branch_w$}", branch_name.blue(), branch_w = branch_width.max(20))
        } else {
            // Empty for non-latest commits
            format!("{:<branch_w$}", "", branch_w = branch_width.max(20))
        };
        
        // Format hash column (fourth column, after graph) - fixed width, centered
        // Make it framed/boxed if it's the active commit
        let hash_short = &commit.id[..8];
        let hash_col = if is_current_compute {
            // Active commit - frame it with brackets and make it red
            // \x1b[31m = red, \x1b[0m = reset
            // Center the bracketed hash in the column
            let bracketed = format!("\x1b[31m[{}]\x1b[0m", hash_short);
            format!("{:^hash_w$}", bracketed, hash_w = hash_width)
        } else {
            // Regular commit - center the hash in the column
            format!("{:^hash_w$}", hash_short.yellow(), hash_w = hash_width)
        };
        
        // Format message column - show full message without truncation
        let message_text = format!("{} {}", commit_symbol, commit.snapshot_comment);
        let message_display = message_text.cyan();
        
        // COMMIT ROW: Build commit line with "*" at commit's column
        let mut commit_line = String::new();
        for i in 0..max_width {
            if i == *position {
                commit_line.push_str("*");
            } else {
                commit_line.push_str(" ");
            }
        }
        // Ensure fixed width
        if commit_line.len() < max_width {
            commit_line.push_str(&" ".repeat(max_width - commit_line.len()));
        }
        let commit_tree_display = format!("{:<tree_w$}", commit_line.green(), tree_w = tree_width);
        
        // Print commit row: Date Time | Branch | Graph | Snapshot Hash | Commit Message
        println!("{:<date_w$} {:<branch_w$} {:<tree_w$} {:<hash_w$} {}", 
            date_col,
            branch_col,
            commit_tree_display,
            hash_col,
            message_display,
            date_w = date_width,
            branch_w = branch_width.max(20),
            tree_w = tree_width,
            hash_w = hash_width
        );
        
        // LINK ROW: Build link line with "|", "\", "/" between this commit and the next one
        // Only show link row if this is not the last commit
        if commit_idx < sorted_commits.len() - 1 {
            let next_commit = &sorted_commits[commit_idx + 1];
            let next_branch_id = next_commit.dataset_id.as_ref()
                .map(|id| id.clone())
                .unwrap_or_else(|| "main".to_string());
            let next_position = branch_positions.get(&next_branch_id).unwrap();
            
            let mut link_line = String::new();
            for i in 0..max_width {
                // Check if branch at position i has commits after current commit (continues)
                let branch_continues = sorted_commits.iter().enumerate().any(|(idx, c)| {
                    if idx <= commit_idx {
                        return false;
                    }
                    let c_branch_id = c.dataset_id.as_ref()
                        .map(|id| id.clone())
                        .unwrap_or_else(|| "main".to_string());
                    branch_positions.get(&c_branch_id) == Some(&i)
                });
                
                // Determine what to show at this column
                if *position != *next_position {
                    // Different branches - show split
                    if i == *position {
                        // Current branch column - show "\" or "/" depending on direction
                        if *next_position > *position {
                            // Next branch is to the right - show "\" (current branch splits right)
                            link_line.push_str("\\");
                        } else {
                            // Next branch is to the left - show "/" (current branch splits left)
                            link_line.push_str("/");
                        }
                    } else if i == *next_position {
                        // Next branch column - show "|" (branch continues)
                        link_line.push_str("|");
                    } else if branch_continues {
                        // Other branches that continue - show "|"
                        link_line.push_str("|");
                    } else {
                        // Empty space
                        link_line.push_str(" ");
                    }
                } else {
                    // Same branch - show "|" for this branch, check others
                    if i == *position {
                        // Same branch continues - show "|"
                        link_line.push_str("|");
                    } else if branch_continues {
                        // Other branches that continue - show "|"
                        link_line.push_str("|");
                    } else {
                        // Empty space
                        link_line.push_str(" ");
                    }
                }
            }
            
            // Ensure fixed width
            if link_line.len() < max_width {
                link_line.push_str(&" ".repeat(max_width - link_line.len()));
            } else if link_line.len() > max_width {
                let link_line_chars: Vec<char> = link_line.chars().collect();
                link_line = link_line_chars.iter().take(max_width).collect();
            }
            
            let link_tree_display = format!("{:<tree_w$}", link_line.green(), tree_w = tree_width);
            
            // Print link row: empty date, empty branch, graph links, empty hash, empty message
            println!("{:<date_w$} {:<branch_w$} {:<tree_w$} {:<hash_w$}", 
                format!("{:<date_w$}", "", date_w = date_width),
                format!("{:<branch_w$}", "", branch_w = branch_width.max(20)),
                link_tree_display,
                format!("{:<8}", ""),
                date_w = date_width,
                branch_w = branch_width.max(20),
                tree_w = tree_width,
                hash_w = hash_width
            );
        }
    }
    
    println!();
    Ok(())
}

fn display_simple_git_graph(commits: &[crate::application::dto::commit::GetCommitResponse]) {
    // Since we don't have branch information directly in commit data,
    // we'll create a simple chronological graph (newest first)
    let mut sorted_commits: Vec<_> = commits.iter().collect();
    sorted_commits.sort_by(|a, b| b.created_date.cmp(&a.created_date));
    
    for (i, commit) in sorted_commits.iter().enumerate() {
        let is_last = i == sorted_commits.len() - 1;
        let connector = if is_last { "└─" } else { "├─" };
        
        // Color coding based on commit type
        let commit_symbol = if commit.snapshot_comment.contains("AUTO SNAP") {
            "🔧"
        } else if commit.snapshot_comment.contains("deleted") {
            "🗑️"
        } else if commit.snapshot_comment.contains("inserted") {
            "➕"
        } else {
            "•"
        };
        
        // Format date with time (including seconds)
        let date_time = if commit.created_date.contains('T') {
            let parts: Vec<&str> = commit.created_date.split('T').collect();
            if parts.len() == 2 {
                let date_part = parts[0];
                let time_part = parts[1].split('.').next().unwrap_or(parts[1]);
                let time_full = time_part.split(':').take(3).collect::<Vec<&str>>().join(":"); // HH:MM:SS
                format!("{} {}", date_part, time_full)
            } else {
                commit.created_date.clone()
            }
        } else {
            commit.created_date.clone()
        };
        
        let line = format!("{} {} {} {} {}", 
            connector.green(),
            &commit.id[..8].yellow(),
            commit_symbol,
            commit.snapshot_comment.cyan(),
            date_time.dimmed()
        );
        println!("{}", line);
    }
    
    println!();
}
