use reqwest::Client;
use crate::config::{API_URL, BEARER_TOKEN};
use crate::application::dto::branch_dto::{BranchDTO, NewBranchDTO, CheckoutBranchDTO};
use crate::domain::errors::branch_error::BranchError;
use super::utils::build_basic_headers;

pub async fn list_branches(deployment_id: &str) -> anyhow::Result<Vec<BranchDTO>> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/clone", *API_URL, deployment_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Send API request and handle response
    let response = client
        .get(url)
        .headers(headers)
        .send().await?
        .error_for_status()?;
    let branches = response.json::<Vec<BranchDTO>>().await?;

    Ok(branches)
}

pub async fn create_branch(deployment_id: &str, clone_id: &str, snapshot_id: &str) -> anyhow::Result<NewBranchDTO> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/{}/{}/branch", *API_URL, deployment_id, clone_id, snapshot_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // TODO: make branch bodies into structs (instead of raw JSON string)
    // TODO: arg to switch to branch automatically after createing (i.e. "checkout": true)
    // Set body
    let body = "{ \"discard_changes\": \"true\", \"checkout\": false, \"ephemeral\": false }";

    // Send API request and handle response
    let response = client
        .post(url)
        .headers(headers)
        .body(body)
        .send().await?
        .error_for_status()?;
    let branch = response.json::<NewBranchDTO>().await?;

    Ok(branch)
}

pub async fn checkout_branch(deployment_id: &str, clone_id: &str) -> anyhow::Result<CheckoutBranchDTO> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/{}/checkout", *API_URL, deployment_id, clone_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Send API request and handle response
    let response = client
        .post(url)
        .headers(headers)
        .send().await?
        .error_for_status()?;
    let branch = response.json::<CheckoutBranchDTO>().await?;

    Ok(branch)
}
