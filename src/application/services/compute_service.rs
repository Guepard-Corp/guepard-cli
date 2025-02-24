use reqwest::{Client, StatusCode};
use crate::config::{API_URL, BEARER_TOKEN};
use crate::application::dto::compute_dto::{ComputeDTO, ComputeStatus};
use crate::domain::errors::compute_error::ComputeError;
use super::utils::build_basic_headers;

pub async fn get_compute(deployment_id: &str, clone_id: &str) -> anyhow::Result<ComputeDTO> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/{}/", *API_URL, deployment_id, clone_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Send API request and handle response
    let response = client
        .get(url)
        .headers(headers)
        .send().await?
        .error_for_status()?;
    let compute = response.json::<ComputeDTO>().await?;

    Ok(compute)
}

pub async fn get_compute_status(deployment_id: &str, clone_id: &str) -> anyhow::Result<ComputeStatus> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/{}/status", *API_URL, deployment_id, clone_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Send API request and handle response
    let response = client
        .get(url)
        .headers(headers)
        .send().await?;

    let compute_status = match response.status() {
        StatusCode::OK => Ok(ComputeStatus::Online),
        StatusCode::GONE => Ok(ComputeStatus::Offline),
        _ => Err(response.error_for_status().unwrap_err()),
    };

    Ok(compute_status?)
}

pub async fn start_compute(deployment_id: &str, clone_id: &str) -> anyhow::Result<ComputeDTO> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/{}/start", *API_URL, deployment_id, clone_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Send API request and handle response
    let response = client
        .get(url)
        .headers(headers)
        .send().await?
        .error_for_status()?;
    let compute = response.json::<ComputeDTO>().await?;

    Ok(compute)
}

pub async fn stop_compute(deployment_id: &str, clone_id: &str) -> anyhow::Result<ComputeDTO> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/{}/stop", *API_URL, deployment_id, clone_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Send API request and handle response
    let response = client
        .get(url)
        .headers(headers)
        .send().await?
        .error_for_status()?;
    let compute = response.json::<ComputeDTO>().await?;

    Ok(compute)
}
