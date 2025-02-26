use crate::application::dto::deploy_dto::{CreateDeploymentRequest, UpdateDeploymentRequest};
use crate::application::services::deploy_service;
use crate::structure::{CreateDeployArgs, UpdateDeployArgs};
use anyhow::Result;
/// (Handles Command Execution)
/// 
/// 
/// Handles the creation of a new deployment.
///
/// This function takes the parsed arguments for creating a deployment,
/// constructs a `CreateDeploymentRequest`, and calls the service to create the deployment.
///
/// # Arguments
///
/// * `args` - A reference to the parsed arguments for creating a deployment.
pub async fn create(args: &CreateDeployArgs) -> Result<()> {
    let request = CreateDeploymentRequest {
        database_provider: args.database_provider.clone(),
        database_version: args.database_version.clone(),
        region: args.region.clone(),
        instance_type: args.instance_type.clone(),
        datacenter: args.datacenter.clone(),
        repository_name: args.repository_name.clone(),
        database_password: args.database_password.clone(),
    };

    deploy_service::create_deployment(request).await?;
    Ok(())
}
// Handles the update of an existing deployment.
///
/// This function takes the parsed arguments for updating a deployment,
/// constructs an `UpdateDeploymentRequest`, and calls the service to update the deployment.
///
/// # Arguments
///
/// * `args` - A reference to the parsed arguments for updating a deployment.
pub async fn update(args: &UpdateDeployArgs) -> Result<()> {
    let request = UpdateDeploymentRequest {
        repository_name: args.repository_name.clone(),
    };

    deploy_service::update_deployment(&args.deployment_id, request).await?;
    Ok(())
}
