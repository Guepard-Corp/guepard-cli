use reqwest::Client;
use crate::config::{API_URL, BEARER_TOKEN};
use crate::application::dto::bookmark_dto::{BookmarkDTO, NewBookmarkDTO, CheckoutBookmarkDTO};
use crate::domain::errors::bookmark_error::BookmarkError;
use super::utils::build_basic_headers;

pub async fn list_bookmarks(deployment_id: &str, clone_id: &str) -> anyhow::Result<Vec<BookmarkDTO>> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/{}/snap", *API_URL, deployment_id, clone_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Send API request and handle response
    let response = client
        .get(url)
        .headers(headers)
        .send().await?
        .error_for_status()?;
    let bookmarks = response.json::<Vec<BookmarkDTO>>().await?;

    Ok(bookmarks)
}

pub async fn create_bookmark(message: &str, deployment_id: &str, clone_id: &str) -> anyhow::Result<NewBookmarkDTO> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/{}/snap", *API_URL, deployment_id, clone_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Set body
    let body = format!("{{ \"snapshot_comment\": \"{message}\" }}");

    // Send API request and handle response
    let response = client
        .put(url)
        .headers(headers)
        .body(body)
        .send().await?
        .error_for_status()?;
    let bookmark = response.json::<NewBookmarkDTO>().await?;

    Ok(bookmark)
}

pub async fn checkout_bookmark(deployment_id: &str, clone_id: &str, snapshot_id: &str) -> anyhow::Result<CheckoutBookmarkDTO> {
    let client = Client::new();
    let url = format!("{}/deploy/{}/{}/{}/branch", *API_URL, deployment_id, clone_id, snapshot_id);
    let headers = build_basic_headers(&BEARER_TOKEN);

    // Set body
    let body = "{ \"discard_changes\": \"true\", \"checkout\": true, \"ephemeral\": true }";

    // Send API request and handle response
    let response = client
        .post(url)
        .headers(headers)
        .body(body)
        .send().await?
        .error_for_status()?;
    let bookmark = response.json::<CheckoutBookmarkDTO>().await?;

    Ok(bookmark)
}
