use reqwest::Client;
use crate::config::{API_URL, BEARER_TOKEN};
use crate::application::dto::deployment_dto::{DeploymentDTO, ListDeploymentDTO};
use crate::domain::errors::deployment_error::DeploymentError;
use super::utils::build_basic_headers;

pub async fn list_deployments() -> anyhow::Result<Vec<ListDeploymentDTO>> {
    let client = Client::new();
    let url = format!("{}/deploy", *API_URL);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Send API request and handle response
    let response = client
        .get(url)
        .headers(headers)
        .send().await?
        .error_for_status()?;
    let deployments = response.json::<Vec<ListDeploymentDTO>>().await?;

    Ok(deployments)
}

pub async fn get_deployment(deployment_id: &str) -> anyhow::Result<DeploymentDTO> {
    let client = Client::new();
    let url = format!("{}/deploy/{}", *API_URL, deployment_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Send API request and handle response
    let response = client
        .get(url)
        .headers(headers)
        .send().await?
        .error_for_status()?;
    let deployment = response.json::<DeploymentDTO>().await?;

    Ok(deployment)
}
