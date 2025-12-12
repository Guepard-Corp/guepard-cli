use anyhow::Result;
use crate::config::config::Config;
use crate::structure::{CloneArgs, CloneSubCommand};
use crate::application::services::{clone, deploy, performance, compute};
use crate::application::dto::clone::CreateCloneRequest;
use crate::application::output::{OutputFormat, print_table_or_json, print_json};
use colored::Colorize;
use serde::Serialize;
use tabled::{Tabled};

#[derive(Tabled, Serialize)]
struct CloneRow {
    #[tabled(rename = "Clone ID")]
    #[serde(rename = "id")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Snapshot ID")]
    #[serde(rename = "snapshot_id")]
    snapshot_id: String,
    #[tabled(rename = "Created")]
    #[serde(rename = "created_at")]
    created_at: String,
}

pub async fn clone_command(args: &CloneArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    match &args.command {
        Some(CloneSubCommand::List { deployment_id }) => {
            list_clones(deployment_id, config, output_format).await
        }
        None => {
            // If both deployment_id and snapshot_id are provided, create a clone
            if let (Some(deployment_id), Some(snapshot_id)) = (&args.deployment_id, &args.snapshot_id) {
                create_clone(args, deployment_id, snapshot_id, config, output_format).await
            } else {
                Err(anyhow::anyhow!("Either provide both -x and -s to create a clone, or use 'guepard clone list -x <deployment_id>' to list clones"))
            }
        }
    }
}

async fn create_clone(args: &CloneArgs, deployment_id: &str, snapshot_id: &str, config: &Config, output_format: OutputFormat) -> Result<()> {
    // Get deployment info to get database_provider and database_version for performance profile lookup
    let deployment = deploy::get_deployment(deployment_id, config).await?;
    
    // Get performance profile ID
    let performance_profile_label = args.performance_profile.clone().unwrap_or_else(|| "gp.g1.xsmall".to_string());
    let performance_profile_id = performance::get_performance_profile_by_label(
        &performance_profile_label,
        &deployment.database_provider,
        &deployment.database_version,
        config,
    ).await?;
    
    // Build clone request
    let repository_name = args.repository_name.clone().unwrap_or_else(|| format!("clone-{}", deployment.repository_name));
    let branch_name = args.branch_name.clone().unwrap_or_else(|| "main".to_string());
    
    let request = CreateCloneRequest {
        repository_name,
        branch_name,
        performance_profile_id,
    };
    
    let clone_response = clone::create_clone(deployment_id, snapshot_id, request, config).await?;
    
    if output_format == OutputFormat::Json {
        print_json(&clone_response);
        return Ok(());
    }
    
    println!("{} Clone created successfully!", "✅".green());
    println!();
    
    // Display the created clone details exactly like deploy -x
    println!("{} Clone Details", "📋".blue());
    println!("  {} {}", "ID:".yellow(), clone_response.id);
    println!("  {} {}", "Name:".yellow(), clone_response.name);
    println!("  {} {}", "Deployment:".yellow(), clone_response.repository_name);
    println!("  {} {}", "Provider:".yellow(), clone_response.database_provider);
    println!("  {} {}", "Version:".yellow(), clone_response.database_version);
    println!("  {} {}", "Status:".yellow(), clone_response.status);
    println!("  {} {}", "FQDN:".yellow(), clone_response.fqdn);
    if let Some(region) = &clone_response.region {
        println!("  {} {}", "Region:".yellow(), region);
    }
    if let Some(datacenter) = &clone_response.datacenter {
        println!("  {} {}", "Datacenter:".yellow(), datacenter);
    }
    println!("  {} {}", "Created:".yellow(), clone_response.created_date);
    
    // Show deployment parent
    if let Some(deployment_parent) = &clone_response.deployment_parent {
        println!("  {} {}", "Deployment Parent:".yellow(), deployment_parent.cyan());
    }
    
    // Show snapshot parent
    if let Some(snapshot_parent) = &clone_response.snapshot_parent {
        println!("  {} {}", "Snapshot Parent:".yellow(), snapshot_parent.cyan());
    }
    
    println!();
    
    // Show branch and snapshot information from compute (what compute is currently pointing to)
    match compute::list_compute(&clone_response.id, config).await {
        Ok(compute_info) => {
            // Get the branch that compute is attached to
            let attached_branch_id = compute_info.branch_id.as_ref()
                .or(Some(&compute_info.attached_branch))
                .unwrap();
            
            // Get branch details
            match crate::application::services::branch::list_branches(&clone_response.id, config).await {
                Ok(branches) => {
                    if let Some(branch) = branches.iter().find(|b| b.id == *attached_branch_id) {
                        let branch_name = branch.branch_name.as_ref()
                            .or(branch.label_name.as_ref())
                            .map(|s| s.clone())
                            .unwrap_or_else(|| branch.id.clone());
                        
                        println!("  {} {}", "Branch:".yellow(), branch_name.cyan());
                        println!("  {} {}", "Branch ID:".yellow(), branch.id.dimmed());
                        
                        // Get snapshot information from the branch
                        let snapshot_id = &branch.snapshot_id;
                        match crate::application::services::commit::list_all_commits(&clone_response.id, config).await {
                            Ok(snapshots) => {
                                if let Some(snapshot) = snapshots.iter().find(|s| s.id == *snapshot_id) {
                                    println!("  {} {}", "Snapshot:".yellow(), snapshot.name.cyan());
                                    if !snapshot.snapshot_comment.is_empty() {
                                        println!("  {} {}", "Comment:".yellow(), snapshot.snapshot_comment.cyan());
                                    }
                                    println!("  {} {}", "Snapshot ID:".yellow(), snapshot.id.dimmed());
                                } else {
                                    println!("  {} {}", "Snapshot ID:".yellow(), snapshot_id.dimmed());
                                }
                            }
                            Err(_) => {
                                println!("  {} {}", "Snapshot ID:".yellow(), snapshot_id.dimmed());
                            }
                        }
                    } else {
                        // Branch not found, but show the ID
                        println!("  {} {}", "Branch ID:".yellow(), attached_branch_id.dimmed());
                    }
                }
                Err(_) => {
                    // Couldn't fetch branches, but show the branch ID from compute
                    println!("  {} {}", "Branch ID:".yellow(), attached_branch_id.dimmed());
                }
            }
        }
        Err(_) => {
            // Compute not available yet, skip branch/snapshot info
        }
    }
    
    // Try to get compute information for the real port
    let port = match compute::list_compute(&clone_response.id, config).await {
        Ok(compute_info) => compute_info.port.to_string(),
        Err(_) => "5432".to_string(),
    };
    
    // Show database connection information (exactly like deploy -x)
    println!();
    println!("{} Database Connection", "🔗".blue());
    println!("  {} {}", "Host:".yellow(), clone_response.fqdn);
    println!("  {} {}", "Port:".yellow(), port);
    println!("  {} {}", "Database:".yellow(), clone_response.repository_name);
    
    // Show username and password
    if let Some(username) = &clone_response.database_username {
        println!("  {} {}", "Username:".yellow(), username);
    }
    if let Some(password) = &clone_response.database_password {
        println!("  {} {}", "Password:".yellow(), password);
    }
    
    // Construct database connection URI if we have username and password
    if let (Some(username), Some(password)) = (&clone_response.database_username, &clone_response.database_password) {
        let connection_uri = format!("postgresql://{}:{}@{}:{}/{}", 
            username,
            password,
            clone_response.fqdn,
            port,
            clone_response.repository_name
        );
        println!();
        println!("{} Ready-to-use Connection URI:", "💡".green());
        println!("{}", connection_uri.cyan().bold());
        println!();
        println!("{} Connect with psql:", "📝".yellow());
        println!("{} psql '{}'", "  $".dimmed(), connection_uri);
        println!();
        println!("{} Connect with any PostgreSQL client using the URI above", "ℹ️".blue());
    }
    
    println!();
    
    Ok(())
}

async fn list_clones(deployment_id: &str, config: &Config, output_format: OutputFormat) -> Result<()> {
    let result = clone::list_clones(deployment_id, config).await?;
    
    if result.shadows.is_empty() {
        if output_format == OutputFormat::Json {
            print_json(&serde_json::json!([]));
        } else {
            println!("{} No clones found for deployment: {}", "ℹ️".blue(), deployment_id);
        }
        return Ok(());
    }
    
    if output_format == OutputFormat::Json {
        print_json(&result.shadows);
        return Ok(());
    }
    
    let rows: Vec<CloneRow> = result.shadows.into_iter().map(|c| {
        CloneRow {
            id: c.id,
            name: c.name,
            status: c.status,
            snapshot_id: c.snapshot_id,
            created_at: c.created_at,
        }
    }).collect();
    
    println!("{} Found {} clone(s) for deployment: {}", "✅".green(), rows.len(), deployment_id);
    print_table_or_json(rows, output_format);
    Ok(())
}

