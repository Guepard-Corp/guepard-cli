use anyhow::Result;
use crate::config::config::Config;
use crate::structure::ListArgs;
use crate::application::services::{deploy, branch, commit};
use colored::Colorize;
use std::collections::HashSet;

// Available columns for each resource type
const DEPLOYMENT_COLUMNS: &[&str] = &[
    "id", "name", "repository", "provider", "version", "status", "fqdn", 
    "region", "datacenter", "created", "type", "port", "connection"
];

const BRANCH_COLUMNS: &[&str] = &["id", "branch_name", "label_name", "job_status", "snapshot_id"];

const COMMIT_COLUMNS: &[&str] = &["id", "name", "message", "created", "dataset_id", "parent_id", "status", "type"];

pub async fn list(args: &ListArgs, config: &Config) -> Result<()> {
    match args.resource.as_str() {
        "deployments" => list_deployments(args, config).await,
        "branches" => list_branches(args, config).await,
        "commits" => list_commits(args, config).await,
        _ => {
            println!("{} Unknown resource: {}", "❌".red(), args.resource);
            println!("Available resources: deployments, branches, commits");
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
            BRANCH_COLUMNS => vec!["id".to_string(), "branch_name".to_string(), "label_name".to_string(), "job_status".to_string(), "snapshot_id".to_string()],
            COMMIT_COLUMNS => vec!["id".to_string(), "name".to_string(), "message".to_string(), "created".to_string(), "dataset_id".to_string(), "parent_id".to_string()],
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
        "branches" => {
            println!("{} Available columns for branches:", "ℹ️".blue());
            println!("{}", BRANCH_COLUMNS.join(", "));
        },
        "commits" => {
            println!("{} Available columns for commits:", "ℹ️".blue());
            println!("{}", COMMIT_COLUMNS.join(", "));
        },
        _ => {}
    }
}

async fn list_deployments(args: &ListArgs, config: &Config) -> Result<()> {
    let deployments = deploy::list_deployments(config).await?;
    
    if deployments.is_empty() {
        println!("{} No deployments found", "ℹ️".blue());
        return Ok(());
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
    
    println!("{} Found {} deployments", "✅".green(), rows.len());
    
    // Display the table with selected columns
    display_dynamic_table(rows, &selected_columns);
    Ok(())
}

async fn list_branches(args: &ListArgs, config: &Config) -> Result<()> {
    let deployment_id = args.deployment_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Deployment ID is required for listing branches. Use -x <deployment_id>"))?;
    
    let branches = branch::list_branches(deployment_id, config).await?;
    
    if branches.is_empty() {
        println!("{} No branches found for deployment: {}", "ℹ️".blue(), deployment_id);
        return Ok(());
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
    
    println!("{} Found {} branches for deployment: {}", "✅".green(), rows.len(), deployment_id);
    display_dynamic_table(rows, &selected_columns);
    Ok(())
}

async fn list_commits(args: &ListArgs, config: &Config) -> Result<()> {
    let deployment_id = args.deployment_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Deployment ID is required for listing commits. Use -x <deployment_id>"))?;
    
    let mut commits = commit::list_all_commits(deployment_id, config).await?;
    
    // Filter out AUTO SNAPs unless -a flag is used
    if !args.all {
        commits.retain(|commit| !commit.snapshot_comment.contains("AUTO SNAP"));
    }
    
    if commits.is_empty() {
        println!("{} No commits found for deployment: {}", "ℹ️".blue(), deployment_id);
        return Ok(());
    }
    
    // Sort commits by creation date (oldest first for proper graph)
    commits.sort_by(|a, b| a.created_date.cmp(&b.created_date));
    
    // Check if user wants git graph format
    if args.graph {
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
        
        println!("{} Found {} commits for deployment: {}", "✅".green(), rows.len(), deployment_id);
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

async fn display_git_graph(commits: &[crate::application::dto::commit::GetCommitResponse], deployment_id: &str, config: &Config) -> Result<()> {
    println!("{} Found {} commits for deployment: {}", "✅".green(), commits.len(), deployment_id);
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
    
    // Sort all commits by date (oldest first)
    let mut sorted_commits: Vec<_> = commits.iter().collect();
    sorted_commits.sort_by(|a, b| a.created_date.cmp(&b.created_date));
    
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
        
        // Calculate hash width (including branch name for last commits)
        let is_last_commit = last_commits_per_branch.get(&branch_id).map(|last_commit| last_commit.id == commit.id).unwrap_or(false);
        let hash_width = if is_last_commit {
            8 + 3 + branch_name.len() // hash + " (" + branch_name + ")"
        } else {
            8
        };
        max_hash_width = max_hash_width.max(hash_width);
        
        // Calculate message width
        let commit_symbol = if commit.snapshot_comment.contains("AUTO SNAP") {
            "🔧"
        } else if commit.snapshot_comment.contains("deleted") {
            "🗑️"
        } else if commit.snapshot_comment.contains("inserted") {
            "➕"
        } else {
            "📝"
        };
        let message_width = commit_symbol.len() + 1 + commit.snapshot_comment.len();
        max_message_width = max_message_width.max(message_width);
        
        // Calculate date width
        let date = commit.created_date.split('T').next().unwrap_or(&commit.created_date);
        max_date_width = max_date_width.max(date.len());
    }
    
    // Calculate tree line width after all positions are determined
    let max_tree_width = max_width * 2; // Each position takes 2 characters ("* " or "| " or "  ")
    
    // Add padding to column widths
    let max_tree_width = max_tree_width + 2;
    let max_hash_width = max_hash_width + 2;
    let max_message_width = max_message_width + 4; // Extra padding for message column
    let max_date_width = max_date_width + 2;
    
    // Display commits with proper table alignment
    for (commit_idx, commit) in sorted_commits.iter().enumerate() {
        let branch_id = commit.dataset_id.as_ref()
            .map(|id| id.clone())
            .unwrap_or_else(|| "main".to_string());
        
        let branch_name = branch_names.get(&branch_id)
            .map(|s| s.as_str())
            .unwrap_or_else(|| if branch_id.as_str() == "main" { "main" } else { &branch_id[..8] });
        
        let position = branch_positions.get(&branch_id).unwrap();
        
        // Create the tree line - only show | when there are actual branches active at this point
        let mut tree_line = String::new();
        for i in 0..max_width {
            if i == *position {
                tree_line.push_str("* ");
            } else {
                // Check if there's a branch at this position that has commits before/at this point AND after this point
                let has_active_branch_at_position = sorted_commits.iter().enumerate().any(|(idx, c)| {
                    let c_branch_id = c.dataset_id.as_ref()
                        .map(|id| id.clone())
                        .unwrap_or_else(|| "main".to_string());
                    let c_position = branch_positions.get(&c_branch_id);
                    
                    // This branch exists at position i AND has commits up to this point AND has commits after this point
                    if c_position == Some(&i) && idx <= commit_idx {
                        // Check if this branch has commits after the current commit
                        sorted_commits.iter().enumerate().any(|(after_idx, after_c)| {
                            let after_branch_id = after_c.dataset_id.as_ref()
                                .map(|id| id.clone())
                                .unwrap_or_else(|| "main".to_string());
                            after_idx > commit_idx && branch_positions.get(&after_branch_id) == Some(&i)
                        })
                    } else {
                        false
                    }
                });
                
                if has_active_branch_at_position {
                    tree_line.push_str("| ");
                } else {
                    tree_line.push_str("  ");
                }
            }
        }
        
        // Pad the tree line to ensure consistent width
        while tree_line.len() < max_width * 2 {
            tree_line.push(' ');
        }
        
        // Color coding based on commit type
        let commit_symbol = if commit.snapshot_comment.contains("AUTO SNAP") {
            "🔧"
        } else if commit.snapshot_comment.contains("deleted") {
            "🗑️"
        } else if commit.snapshot_comment.contains("inserted") {
            "➕"
        } else {
            "📝"
        };
        
        let date = commit.created_date.split('T').next().unwrap_or(&commit.created_date);
        
        // Check if this is the last commit for this branch
        let is_last_commit = last_commits_per_branch.get(&branch_id).map(|last_commit| last_commit.id == commit.id).unwrap_or(false);
        
        // Check if this is the current compute snapshot
        let is_current_compute = current_compute_snapshot.as_ref().map(|snapshot_id| snapshot_id == &commit.id).unwrap_or(false);
        
        let hash_display = if is_last_commit {
            if is_current_compute {
                format!("{} ({}) 📌", &commit.id[..8].yellow(), branch_name.blue())
            } else {
                format!("{} ({})", &commit.id[..8].yellow(), branch_name.blue())
            }
        } else {
            if is_current_compute {
                format!("{} 📌", commit.id[..8].to_string().yellow())
            } else {
                commit.id[..8].to_string().yellow().to_string()
            }
        };
        
        // Format with proper table padding - right-align the date
        println!("{:<tree_width$} {:<hash_width$} {:<message_width$} {:>date_width$}", 
            tree_line.green(),
            hash_display,
            format!("{} {}", commit_symbol, commit.snapshot_comment.cyan()),
            date.dimmed(),
            tree_width = max_tree_width,
            hash_width = max_hash_width,
            message_width = max_message_width,
            date_width = max_date_width
        );
    }
    
    println!();
    Ok(())
}

fn display_simple_git_graph(commits: &[crate::application::dto::commit::GetCommitResponse]) {
    // Since we don't have branch information directly in commit data,
    // we'll create a simple chronological graph
    let mut sorted_commits: Vec<_> = commits.iter().collect();
    sorted_commits.sort_by(|a, b| a.created_date.cmp(&b.created_date));
    
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
            "📝"
        };
        
        let line = format!("{} {} {} {} {}", 
            connector.green(),
            &commit.id[..8].yellow(),
            commit_symbol,
            commit.snapshot_comment.cyan(),
            commit.created_date.split('T').next().unwrap_or(&commit.created_date).dimmed()
        );
        println!("{}", line);
    }
    
    println!();
}
