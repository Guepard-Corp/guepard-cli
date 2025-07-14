use crate::application::dto::deploy_dto::{CreateDeploymentRequest, UpdateDeploymentRequest};
use crate::application::services::deploy_service;
use crate::structure::{CreateDeployArgs, UpdateDeployArgs};
use anyhow::Result;
use crate::config::config::Config;
use tabled::{Table, Tabled, settings::Style};
use colored::Colorize;

#[derive(Tabled)]
struct DeployRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Repository")]
    repository_name: String,
    #[tabled(rename = "Region")]
    region: String,
}
pub async fn create(args: &CreateDeployArgs, config: &Config) -> Result<()> {
    let request = CreateDeploymentRequest {
        repository_name: args.repository_name.clone(),
        database_provider: args.database_provider.clone(),
        database_version: args.database_version.clone(),
        deployment_type: args.deployment_type.clone(),
        region: args.region.clone(),
        datacenter: args.datacenter.clone(),
        instance_type: args.instance_type.clone(),
        database_username: args.database_username.clone(),
        database_password: args.database_password.clone(),
        performance_profile_id: args.performance_profile_id.clone(),
        node_id: args.node_id.clone(),
    };
    let deployment = deploy_service::create_deployment(request, config).await?;
    println!(
        "{} Created deployment [{}] '{}' (Status: {}) with repo [{}], provider [{}], region [{}], username [{}]",
        "✅".green(),
        deployment.id.cyan(),
        deployment.name,
        deployment.status,
        deployment.repository_name,
        deployment.database_provider,
        deployment.region,
        deployment.database_username
    );
    Ok(())
}

pub async fn update(args: &UpdateDeployArgs, config: &Config) -> Result<()> {
    let request = UpdateDeploymentRequest {
        repository_name: args.repository_name.clone(),
    };

    deploy_service::update_deployment(&args.deployment_id, request, config).await?; 
    println!(
        "{} Updated deployment  [{}] to the name : [{}]",
        "✅".green(),
        args.deployment_id.cyan(),
        args.repository_name
    );
    Ok(())
}



pub async fn list(config: &Config) -> Result<()> {
    let deployments = deploy_service::list_deployments(config).await?;
    if deployments.is_empty() {
        println!("{} No deployments found", "ℹ️".blue());
        return Ok(());
    }
    let rows: Vec<DeployRow> = deployments.into_iter().map(|d| DeployRow {
        id: d.id,
        name: d.name,
        status: d.status,
        repository_name: d.repository_name,
        region: d.region,
    }).collect();

    println!("{} Retrieved {} deployments:", "✅".green(), rows.len());
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}

pub async fn get(deployment_id: &str, config: &Config) -> Result<()> {
    let deployment: crate::application::dto::deploy_dto::GetDeploymentResponse = deploy_service::get_deployment(deployment_id, config).await?;
    println!(
        "{} Deployment [{}]: '{}', Status: [{}], Repo: [{}], Provider: [{}], Region: [{}]",
        "✅".green(),
        deployment.id.cyan(),
        deployment.name,
        deployment.status,
        deployment.repository_name,
        deployment.database_provider,
        deployment.region,
    );
    Ok(())
}