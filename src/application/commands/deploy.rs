use anyhow::Result;
use crate::config::config::Config;
use crate::structure::{DeployArgs, CreateDeployArgs, UpdateDeployArgs};
use crate::application::dto::deploy::{CreateDeploymentRequest, UpdateDeploymentRequest};
use crate::application::services::deploy;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

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
        if let Some(repo_name) = &args.repository_name {
            // Update deployment
            let update_args = UpdateDeployArgs {
                deployment_id: deployment_id.clone(),
                repository_name: repo_name.clone(),
            };
            update(&update_args, config).await?;
        } else {
            // Get deployment details
            get(deployment_id, config).await?;
        }
    } else {
        // Create new deployment
        let create_args = CreateDeployArgs {
            database_provider: args.database_provider.clone(),
            database_version: args.database_version.clone(),
            region: args.region.clone(),
            instance_type: args.instance_type.clone(),
            datacenter: args.datacenter.clone(),
            repository_name: args.repository_name.clone().unwrap_or("default-repo".to_string()),
            database_password: args.database_password.clone(),
        };
        create(&create_args, config).await?;
    }
    Ok(())
}

pub async fn create(args: &CreateDeployArgs, config: &Config) -> Result<()> {
    let request = CreateDeploymentRequest {
        repository_name: args.repository_name.clone(),
        database_provider: args.database_provider.clone(),
        database_version: args.database_version.clone(),
        deployment_type: "REPOSITORY".to_string(),
        region: args.region.clone(),
        datacenter: args.datacenter.clone(),
        database_username: "guepard".to_string(),
        database_password: args.database_password.clone(),
        performance_profile_id: "default".to_string(),
    };
    
    deploy::create_deployment(request, config).await?;
    
    println!("{} Deployment created successfully!", "✅".green());
    println!("{} Use 'gfs deploy -x <deployment_id>' to get details", "💡".yellow());
    
    Ok(())
}

pub async fn update(args: &UpdateDeployArgs, config: &Config) -> Result<()> {
    let request = UpdateDeploymentRequest {
        repository_name: args.repository_name.clone(),
    };
    
    deploy::update_deployment(&args.deployment_id, request, config).await?;
    println!("{} Deployment updated successfully!", "✅".green());
    Ok(())
}

pub async fn list(config: &Config) -> Result<()> {
    let deployments = deploy::list_deployments(config).await?;
    
    if deployments.is_empty() {
        println!("{} No deployments found", "ℹ️".blue());
        return Ok(());
    }
    
    let rows: Vec<DeployRow> = deployments.into_iter().map(|d| DeployRow {
        id: d.id,
        name: d.name,
        repository_name: d.repository_name,
        database_provider: d.database_provider,
        database_version: d.database_version,
        status: d.status,
        fqdn: d.fqdn,
    }).collect();
    
    println!("{} Found {} deployments", "✅".green(), rows.len());
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}

pub async fn get(deployment_id: &str, config: &Config) -> Result<()> {
    let deployment = deploy::get_deployment(deployment_id, config).await?;
    
    let deploy_row = DeployRow {
        id: deployment.id,
        name: deployment.name,
        repository_name: deployment.repository_name,
        database_provider: deployment.database_provider,
        database_version: deployment.database_version,
        status: deployment.status,
        fqdn: deployment.fqdn,
    };
    
    println!("{} Deployment Details", "📋".blue());
    println!("{}", Table::new(vec![deploy_row]).with(Style::rounded()));
    Ok(())
}