use anyhow::Result;
use crate::config::config::Config;
use crate::structure::DeployArgs;
use crate::application::dto::deploy::{CreateDeploymentRequest, UpdateDeploymentRequest};
use crate::application::services::{deploy, performance, compute, branch, commit};
use crate::application::output::{OutputFormat, print_json};
use colored::Colorize;
use std::io::{self, Write};

pub async fn deploy(args: &DeployArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    // Check for interactive mode
    if args.interactive {
        return interactive_deploy(config).await;
    }
    
    if let Some(deployment_id) = &args.deployment_id {
        // We have a deployment ID, determine operation based on other args
        if args.repository_name.is_some() {
            // Update deployment
            update_deployment(deployment_id, args, config, output_format).await?;
        } else if args.yes {
            // Delete deployment
            delete_deployment(deployment_id, args, config, output_format).await?;
        } else {
            // Get deployment details
            get_deployment(deployment_id, config, output_format).await?;
        }
    } else {
        // No deployment ID, check if we have create args
        if args.database_provider.is_some() && args.database_version.is_some() && 
           args.region.is_some() && args.instance_type.is_some() && 
           args.datacenter.is_some() && args.database_password.is_some() {
            // Create new deployment
            create_deployment(args, config, output_format).await?;
        } else {
            println!("{} Please provide either:", "❌".red());
            println!("  • Create args: -p, -v, -r, -i, -d, -w (and optionally -n, -u)");
            println!("  • Get/Update/Delete: -x <deployment_id> (and optionally -n for update, -y for delete)");
            println!("{} Use 'guepard deploy --help' for more information", "💡".yellow());
        }
    }
    Ok(())
}

async fn create_deployment(args: &DeployArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let database_provider = args.database_provider.clone().unwrap();
    let database_version = args.database_version.clone().unwrap();
    
    // Get performance profile ID
    let performance_profile_label = args.performance_profile.clone().unwrap_or_else(|| "gp.g1.xsmall".to_string());
    let performance_profile_id = performance::get_performance_profile_by_label(
        &performance_profile_label,
        &database_provider,
        &database_version,
        config,
    ).await?;
    
    let request = CreateDeploymentRequest {
        repository_name: args.repository_name.clone().unwrap_or("default-repo".to_string()),
        database_provider,
        database_version,
        deployment_type: args.instance_type.clone().unwrap_or("REPOSITORY".to_string()),
        region: args.region.clone().unwrap(),
        datacenter: args.datacenter.clone().unwrap(),
        database_username: args.user.clone().unwrap_or("guepard".to_string()),
        database_password: args.database_password.clone().unwrap(),
        performance_profile_id,
        node_id: args.node_id.clone(),
    };
    
    let deployment = deploy::create_deployment(request, config).await?;
    
    if output_format == OutputFormat::Json {
        print_json(&deployment);
        return Ok(());
    }
    
    println!("{} Deployment created successfully!", "✅".green());
    println!();
    
    // Display the created deployment details in a more natural format
    println!("{} Deployment Details", "📋".blue());
    println!("  {} {}", "ID:".yellow(), deployment.id);
    println!("  {} {}", "Name:".yellow(), deployment.name);
    println!("  {} {}", "Repository:".yellow(), deployment.repository_name);
    println!("  {} {}", "Provider:".yellow(), deployment.database_provider);
    println!("  {} {}", "Version:".yellow(), deployment.database_version);
    println!("  {} {}", "Status:".yellow(), deployment.status);
    println!("  {} {}", "FQDN:".yellow(), deployment.fqdn);
    println!("  {} {}", "Region:".yellow(), deployment.region);
    println!("  {} {}", "Datacenter:".yellow(), deployment.datacenter);
    println!("  {} {}", "Created:".yellow(), deployment.created_date);
    
    // Try to get compute information for the real port
    let port = match compute::list_compute(&deployment.id, config).await {
        Ok(compute_info) => compute_info.port.to_string(),
        Err(_) => deployment.port.map(|p| p.to_string()).unwrap_or_else(|| "5432".to_string()),
    };
    
    // Show database connection information
    if let Some(port_display) = deployment.port {
        println!("  {} {}", "Port:".yellow(), port_display);
    }
    if let Some(connection_string) = &deployment.connection_string {
        println!("  {} {}", "Connection URI:".yellow(), connection_string);
    }
    
    // Show helpful connection information
    println!();
    println!("{} Database Connection", "🔗".blue());
    println!("  {} {}", "Host:".yellow(), deployment.fqdn);
    println!("  {} {}", "Port:".yellow(), port);
    println!("  {} {}", "Database:".yellow(), deployment.repository_name);
    println!("  {} {}", "Username:".yellow(), deployment.database_username);
    println!("  {} {}", "Password:".yellow(), deployment.database_password);
    
    // Construct and show the connection URI
    let connection_uri = format!("postgresql://{}:{}@{}:{}/{}", 
        deployment.database_username,
        deployment.database_password,
        deployment.fqdn, 
        port,
        deployment.repository_name
    );
    println!();
    println!("{} Ready-to-use Connection URI:", "💡".green());
    println!("{}", connection_uri.cyan().bold());
    println!();
    println!("{} Connect with psql:", "📝".yellow());
    println!("{} psql '{}'", "  $".dimmed(), connection_uri);
    println!();
    println!("{} Connect with any PostgreSQL client using the URI above", "ℹ️".blue());
    
    println!();
    
    println!("{} Use 'guepard deploy -x {}' to get more details", "💡".yellow(), deployment.id);
    
    Ok(())
}

fn generate_password() -> String {
    // Generate a strong 13-character password with alphanumeric and special characters
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*".chars().collect();
    let mut password = String::new();
    
    // Use current time with nanoseconds for better randomness
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    // Add some process-specific entropy for more randomness
    let process_id = std::process::id() as u128;
    
    for i in 0..13 {
        let index = ((timestamp as usize + process_id as usize + i * 1007) as usize) % chars.len();
        password.push(chars[index]);
    }
    
    password
}

async fn update_deployment(deployment_id: &str, args: &DeployArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let request = UpdateDeploymentRequest {
        repository_name: args.repository_name.clone().unwrap(),
    };
    
    deploy::update_deployment(deployment_id, request, config).await?;
    if output_format == OutputFormat::Table {
        println!("{} Deployment updated successfully!", "✅".green());
    } else {
        print_json(&serde_json::json!({"status": "updated", "deployment_id": deployment_id}));
    }
    Ok(())
}

async fn get_deployment(deployment_id: &str, config: &Config, output_format: OutputFormat) -> Result<()> {
    let deployment = deploy::get_deployment(deployment_id, config).await?;
    
    if output_format == OutputFormat::Json {
        print_json(&deployment);
        return Ok(());
    }
    
    println!("{} Deployment Details", "📋".blue());
    println!("  {} {}", "ID:".yellow(), deployment.id);
    println!("  {} {}", "Name:".yellow(), deployment.name);
    println!("  {} {}", "Repository:".yellow(), deployment.repository_name);
    println!("  {} {}", "Provider:".yellow(), deployment.database_provider);
    println!("  {} {}", "Version:".yellow(), deployment.database_version);
    println!("  {} {}", "Status:".yellow(), deployment.status);
    println!("  {} {}", "FQDN:".yellow(), deployment.fqdn);
    println!("  {} {}", "Region:".yellow(), deployment.region);
    println!("  {} {}", "Datacenter:".yellow(), deployment.datacenter);
    println!("  {} {}", "Created:".yellow(), deployment.created_date);
    
    // Show branch and snapshot information
    // Try to get branch information
    if let Some(branch_id) = &deployment.branch_id {
        // If branch_id is set in deployment, fetch its details
        let branch_name = match branch::list_branches(deployment_id, config).await {
            Ok(branches) => {
                branches.iter()
                    .find(|b| b.id == *branch_id)
                    .and_then(|b| b.branch_name.as_ref().or(b.label_name.as_ref()))
                    .map(|s| s.clone())
                    .unwrap_or_else(|| branch_id.clone())
            }
            Err(_) => branch_id.clone(),
        };
        
        println!("  {} {}", "Branch:".yellow(), branch_name.cyan());
        println!("  {} {}", "Branch ID:".yellow(), branch_id.dimmed());
    } else {
        // Try to list branches to see if there are any
        match branch::list_branches(deployment_id, config).await {
            Ok(branches) if !branches.is_empty() => {
                // Show the first branch or find the active one
                let active_branch = branches.iter()
                    .find(|b| b.job_status.as_ref().map(|s| s == "ACTIVE" || s == "CHECKED_OUT").unwrap_or(false))
                    .or_else(|| branches.first());
                
                if let Some(branch) = active_branch {
                    let branch_name = branch.branch_name.as_ref()
                        .or(branch.label_name.as_ref())
                        .map(|s| s.clone())
                        .unwrap_or_else(|| branch.id.clone());
                    println!("  {} {}", "Branch:".yellow(), branch_name.cyan());
                    println!("  {} {}", "Branch ID:".yellow(), branch.id.dimmed());
                }
            }
            _ => {
                // No branches available
            }
        }
    }
    
    // Try to get snapshot information
    if let Some(snapshot_id) = &deployment.snapshot_id {
        // If snapshot_id is set in deployment, fetch its details
        match commit::list_all_commits(deployment_id, config).await {
            Ok(snapshots) => {
                if let Some(snapshot) = snapshots.iter().find(|s| s.id == *snapshot_id) {
                    println!("  {} {}", "Snapshot:".yellow(), snapshot.name.cyan());
                    if !snapshot.snapshot_comment.is_empty() {
                        println!("  {} {}", "Comment:".yellow(), snapshot.snapshot_comment.cyan());
                    }
                    println!("  {} {}", "Snapshot ID:".yellow(), snapshot_id.dimmed());
                } else {
                    println!("  {} {}", "Snapshot:".yellow(), snapshot_id.cyan());
                    println!("  {} {}", "Snapshot ID:".yellow(), snapshot_id.dimmed());
                }
            }
            Err(_) => {
                println!("  {} {}", "Snapshot:".yellow(), snapshot_id.cyan());
                println!("  {} {}", "Snapshot ID:".yellow(), snapshot_id.dimmed());
            }
        }
    } else {
        // Try to list snapshots to see if there are any
        match commit::list_all_commits(deployment_id, config).await {
            Ok(snapshots) if !snapshots.is_empty() => {
                // Show the most recent snapshot
                if let Some(snapshot) = snapshots.first() {
                    println!("  {} {}", "Snapshot:".yellow(), snapshot.name.cyan());
                    if !snapshot.snapshot_comment.is_empty() {
                        println!("  {} {}", "Comment:".yellow(), snapshot.snapshot_comment.cyan());
                    }
                    println!("  {} {}", "Snapshot ID:".yellow(), snapshot.id.dimmed());
                }
            }
            _ => {
                // No snapshots available
            }
        }
    }
    
    // Try to get compute information for the real port
    let port = match compute::list_compute(deployment_id, config).await {
        Ok(compute_info) => compute_info.port.to_string(),
        Err(_) => deployment.port.map(|p| p.to_string()).unwrap_or_else(|| "5432".to_string()),
    };
    
    // Show database connection information
    println!();
    println!("{} Database Connection", "🔗".blue());
    println!("  {} {}", "Host:".yellow(), deployment.fqdn);
    println!("  {} {}", "Port:".yellow(), port);
    println!("  {} {}", "Database:".yellow(), deployment.repository_name);
    println!("  {} {}", "Username:".yellow(), deployment.database_username);
    println!("  {} {}", "Password:".yellow(), deployment.database_password);
    
    // Construct database connection URI
    let connection_uri = format!("postgresql://{}:{}@{}:{}/{}", 
        deployment.database_username, 
        deployment.database_password, 
        deployment.fqdn,
        port,
        deployment.repository_name
    );
    println!();
    println!("{} Ready-to-use Connection URI:", "💡".green());
    println!("{}", connection_uri.cyan().bold());
    println!();
    println!("{} Connect with psql:", "📝".yellow());
    println!("{} psql '{}'", "  $".dimmed(), connection_uri);
    println!();
    println!("{} Connect with any PostgreSQL client using the URI above", "ℹ️".blue());
    
    println!();
    Ok(())
}

async fn delete_deployment(deployment_id: &str, args: &DeployArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    // Confirm deletion unless -y flag is used
    if !args.yes {
        print!("{} Are you sure you want to delete deployment {}? (y/N): ", 
               "⚠️".yellow(), deployment_id);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("{} Deletion cancelled.", "ℹ️".blue());
            return Ok(());
        }
    }
    
    // Call the actual delete API
    deploy::delete_deployment(deployment_id, config).await?;
    
    if output_format == OutputFormat::Table {
        println!("{} Deployment {} deleted successfully!", "✅".green(), deployment_id);
    } else {
        print_json(&serde_json::json!({"status": "deleted", "deployment_id": deployment_id}));
    }
    
    Ok(())
}

async fn interactive_deploy(config: &Config) -> Result<()> {
    println!("{} Welcome to Interactive Deployment! 🚀", "🐆".cyan());
    println!("{} Let's create your database deployment step by step.", "💡".yellow());
    println!();
    
    // Step 1: Database Provider
    println!("{} Step 1: Choose Database Provider", "1️⃣".blue());
    println!("Available options: PostgreSQL, MySQL, MongoDB");
    print!("{} Database Provider [PostgreSQL]: ", "🔧".green());
    io::stdout().flush()?;
    
    let mut database_provider = String::new();
    io::stdin().read_line(&mut database_provider)?;
    let database_provider = database_provider.trim();
    let database_provider = if database_provider.is_empty() { "PostgreSQL" } else { database_provider };
    
    // Step 2: Database Version
    println!();
    println!("{} Step 2: Choose Database Version", "2️⃣".blue());
    let default_version = match database_provider {
        "PostgreSQL" => "16",
        "MySQL" => "8.0",
        "MongoDB" => "7.0",
        _ => "16"
    };
    print!("{} Database Version [{}]: ", "🔧".green(), default_version);
    io::stdout().flush()?;
    
    let mut database_version = String::new();
    io::stdin().read_line(&mut database_version)?;
    let database_version = database_version.trim();
    let database_version = if database_version.is_empty() { default_version } else { database_version };
    
    // Step 3: Region
    println!();
    println!("{} Step 3: Choose Region", "3️⃣".blue());
    println!("Available options: us-west, us-east, eu-west, asia-pacific");
    print!("{} Region [us-west]: ", "🌍".green());
    io::stdout().flush()?;
    
    let mut region = String::new();
    io::stdin().read_line(&mut region)?;
    let region = region.trim();
    let region = if region.is_empty() { "us-west" } else { region };
    
    // Step 4: Deployment Type
    println!();
    println!("{} Step 4: Choose Deployment Type", "4️⃣".blue());
    println!("Available options: REPOSITORY, F2");
    print!("{} Deployment Type [REPOSITORY]: ", "🏗️".green());
    io::stdout().flush()?;
    
    let mut instance_type = String::new();
    io::stdin().read_line(&mut instance_type)?;
    let instance_type = instance_type.trim();
    let instance_type = if instance_type.is_empty() { "REPOSITORY" } else { instance_type };
    
    // Step 5: Datacenter
    println!();
    println!("{} Step 5: Choose Datacenter", "5️⃣".blue());
    println!("Available options: us-west-aws, us-east-aws, eu-west-aws");
    print!("{} Datacenter [us-west-aws]: ", "🏢".green());
    io::stdout().flush()?;
    
    let mut datacenter = String::new();
    io::stdin().read_line(&mut datacenter)?;
    let datacenter = datacenter.trim();
    let datacenter = if datacenter.is_empty() { "us-west-aws" } else { datacenter };
    
    // Step 6: Repository Name
    println!();
    println!("{} Step 6: Repository Name", "6️⃣".blue());
    print!("{} Repository Name [my-database]: ", "📁".green());
    io::stdout().flush()?;
    
    let mut repository_name = String::new();
    io::stdin().read_line(&mut repository_name)?;
    let repository_name = repository_name.trim();
    let repository_name = if repository_name.is_empty() { "my-database" } else { repository_name };
    
    // Step 7: Database Username (optional)
    println!();
    println!("{} Step 7: Database Username", "7️⃣".blue());
    print!("{} Database Username [postgres]: ", "👤".green());
    io::stdout().flush()?;
    
    let mut user = String::new();
    io::stdin().read_line(&mut user)?;
    let user = user.trim();
    let user = if user.is_empty() { "postgres" } else { user };
    
    // Step 8: Database Password
    println!();
    println!("{} Step 8: Database Password", "8️⃣".blue());
    print!("{} Database Password [press Enter to auto-generate]: ", "🔐".green());
    io::stdout().flush()?;
    
    let mut database_password = String::new();
    io::stdin().read_line(&mut database_password)?;
    let database_password = database_password.trim();
    let database_password = if database_password.is_empty() { 
        generate_password() 
    } else { 
        database_password.to_string() 
    };
    
    // Step 9: Performance Profile
    println!();
    println!("{} Step 9: Performance Profile", "9️⃣".blue());
    println!("Available options: gp.g1.xsmall, gp.g1.small, gp.g1.medium, gp.g1.large");
    print!("{} Performance Profile [gp.g1.xsmall]: ", "⚡".green());
    io::stdout().flush()?;
    
    let mut performance_profile = String::new();
    io::stdin().read_line(&mut performance_profile)?;
    let performance_profile = performance_profile.trim();
    let performance_profile = if performance_profile.is_empty() { "gp.g1.xsmall" } else { performance_profile };
    
    // Summary
    println!();
    println!("{} Deployment Summary", "📋".blue());
    println!("  {} {}", "Database Provider:".yellow(), database_provider);
    println!("  {} {}", "Database Version:".yellow(), database_version);
    println!("  {} {}", "Region:".yellow(), region);
    println!("  {} {}", "Deployment Type:".yellow(), instance_type);
    println!("  {} {}", "Datacenter:".yellow(), datacenter);
    println!("  {} {}", "Repository Name:".yellow(), repository_name);
    println!("  {} {}", "Username:".yellow(), user);
    println!("  {} {}", "Password:".yellow(), database_password);
    println!("  {} {}", "Performance Profile:".yellow(), performance_profile);
    println!();
    
    // Confirmation
    print!("{} Proceed with deployment? (Y/n): ", "❓".yellow());
    io::stdout().flush()?;
    
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;
    
    let confirm_input = confirmation.trim().to_lowercase();
    if !confirm_input.is_empty() && confirm_input.starts_with('n') {
        println!("{} Deployment cancelled.", "ℹ️".blue());
        return Ok(());
    }
    
    // Create deployment
    println!();
    println!("{} Creating deployment...", "🚀".cyan());
    
    // Get performance profile ID
    let performance_profile_id = performance::get_performance_profile_by_label(
        &performance_profile,
        &database_provider,
        &database_version,
        config,
    ).await?;
    
    let request = CreateDeploymentRequest {
        repository_name: repository_name.to_string(),
        database_provider: database_provider.to_string(),
        database_version: database_version.to_string(),
        deployment_type: instance_type.to_string(),
        region: region.to_string(),
        datacenter: datacenter.to_string(),
        database_username: user.to_string(),
        database_password: database_password.to_string(),
        performance_profile_id,
        node_id: None, // Interactive mode doesn't support node_id yet
    };
    
    let deployment = deploy::create_deployment(request, config).await?;
    
    println!("{} Deployment created successfully!", "✅".green());
    println!();
    
    // Display the created deployment details in a more natural format
    println!("{} Deployment Details", "📋".blue());
    println!("  {} {}", "ID:".yellow(), deployment.id);
    println!("  {} {}", "Name:".yellow(), deployment.name);
    println!("  {} {}", "Repository:".yellow(), deployment.repository_name);
    println!("  {} {}", "Provider:".yellow(), deployment.database_provider);
    println!("  {} {}", "Version:".yellow(), deployment.database_version);
    println!("  {} {}", "Status:".yellow(), deployment.status);
    println!("  {} {}", "FQDN:".yellow(), deployment.fqdn);
    println!("  {} {}", "Region:".yellow(), deployment.region);
    println!("  {} {}", "Datacenter:".yellow(), deployment.datacenter);
    println!("  {} {}", "Created:".yellow(), deployment.created_date);
    
    // Try to get compute information for the real port
    let port = match compute::list_compute(&deployment.id, config).await {
        Ok(compute_info) => compute_info.port.to_string(),
        Err(_) => deployment.port.map(|p| p.to_string()).unwrap_or_else(|| "5432".to_string()),
    };
    
    // Show database connection information
    if let Some(port_display) = deployment.port {
        println!("  {} {}", "Port:".yellow(), port_display);
    }
    if let Some(connection_string) = &deployment.connection_string {
        println!("  {} {}", "Connection URI:".yellow(), connection_string);
    }
    
    // Show helpful connection information
    println!();
    println!("{} Database Connection", "🔗".blue());
    println!("  {} {}", "Host:".yellow(), deployment.fqdn);
    println!("  {} {}", "Port:".yellow(), port);
    println!("  {} {}", "Database:".yellow(), deployment.repository_name);
    println!("  {} {}", "Username:".yellow(), deployment.database_username);
    println!("  {} {}", "Password:".yellow(), deployment.database_password);
    
    // Construct and show the connection URI
    let connection_uri = format!("postgresql://{}:{}@{}:{}/{}", 
        deployment.database_username,
        deployment.database_password,
        deployment.fqdn, 
        port,
        deployment.repository_name
    );
    println!();
    println!("{} Ready-to-use Connection URI:", "💡".green());
    println!("{}", connection_uri.cyan().bold());
    println!();
    println!("{} Connect with psql:", "📝".yellow());
    println!("{} psql '{}'", "  $".dimmed(), connection_uri);
    println!();
    println!("{} Connect with any PostgreSQL client using the URI above", "ℹ️".blue());
    
    println!();
    
    println!("{} Use 'guepard deploy -x {}' to get more details", "💡".yellow(), deployment.id);
    
    Ok(())
}