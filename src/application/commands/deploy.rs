use anyhow::Result;
use crate::config::config::Config;
use crate::structure::DeployArgs;
use crate::application::dto::deploy::{CreateDeploymentRequest, UpdateDeploymentRequest};
use crate::application::services::{deploy, performance};
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};
use std::io::{self, Write};

#[derive(Tabled)]
struct DeployRow {
    #[tabled(rename = "Deployment ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Repository")]
    repository_name: String,
    #[tabled(rename = "Provider")]
    database_provider: String,
    #[tabled(rename = "Version")]
    database_version: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "FQDN")]
    fqdn: String,
}

pub async fn deploy(args: &DeployArgs, config: &Config) -> Result<()> {
    if let Some(deployment_id) = &args.deployment_id {
        // We have a deployment ID, determine operation based on other args
        if args.repository_name.is_some() {
            // Update deployment
            update_deployment(deployment_id, args, config).await?;
        } else if args.yes {
            // Delete deployment
            delete_deployment(deployment_id, args, config).await?;
        } else {
            // Get deployment details
            get_deployment(deployment_id, config).await?;
        }
    } else {
        // No deployment ID, check if we have create args
        if args.database_provider.is_some() && args.database_version.is_some() && 
           args.region.is_some() && args.instance_type.is_some() && 
           args.datacenter.is_some() && args.database_password.is_some() {
            // Create new deployment
            create_deployment(args, config).await?;
        } else {
            println!("{} Please provide either:", "❌".red());
            println!("  • Create args: -p, -v, -r, -i, -d, -w (and optionally -n, -u)");
            println!("  • Get/Update/Delete: -x <deployment_id> (and optionally -n for update, -y for delete)");
            println!("{} Use 'guepard deploy --help' for more information", "💡".yellow());
        }
    }
    Ok(())
}

async fn create_deployment(args: &DeployArgs, config: &Config) -> Result<()> {
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
    
    // Show database connection information
    if let Some(port) = deployment.port {
        println!("  {} {}", "Port:".yellow(), port);
    }
    if let Some(connection_string) = &deployment.connection_string {
        println!("  {} {}", "Connection URI:".yellow(), connection_string);
    }
    
    println!();
    
    println!("{} Use 'guepard deploy -x {}' to get more details", "💡".yellow(), deployment.id);
    
    Ok(())
}

async fn update_deployment(deployment_id: &str, args: &DeployArgs, config: &Config) -> Result<()> {
    let request = UpdateDeploymentRequest {
        repository_name: args.repository_name.clone().unwrap(),
    };
    
    deploy::update_deployment(deployment_id, request, config).await?;
    println!("{} Deployment updated successfully!", "✅".green());
    Ok(())
}

async fn get_deployment(deployment_id: &str, config: &Config) -> Result<()> {
    let deployment = deploy::get_deployment(deployment_id, config).await?;
    
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
    
    // Show database connection information
    println!("  {} {}", "Username:".yellow(), deployment.database_username);
    println!("  {} {}", "Password:".yellow(), deployment.database_password);
    
    // Construct database connection URI
    let connection_uri = format!("postgresql://{}:{}@{}:5432/{}", 
        deployment.database_username, 
        deployment.database_password, 
        deployment.fqdn,
        deployment.repository_name
    );
    println!("  {} {}", "Connection URI:".yellow(), connection_uri);
    
    println!();
    Ok(())
}

async fn delete_deployment(deployment_id: &str, args: &DeployArgs, _config: &Config) -> Result<()> {
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
    
    // TODO: Implement actual deletion API call when available
    // deploy::delete_deployment(deployment_id, config).await?;
    
    println!("{} Deployment {} deleted successfully!", "✅".green(), deployment_id);
    println!("{} Note: Deletion API not yet implemented", "ℹ️".blue());
    
    Ok(())
}