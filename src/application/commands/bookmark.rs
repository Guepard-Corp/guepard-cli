use crate::application::dto::bookmark_dto::{BookmarkDTO, NewBookmarkDTO, CheckoutBookmarkDTO};
use crate::application::services::bookmark_service;
use crate::domain::errors::bookmark_error::BookmarkError;

pub async fn list(deployment_id: &str, clone_id: &str) -> anyhow::Result<Vec<BookmarkDTO>> {
    let bookmarks = bookmark_service::list_bookmarks(deployment_id, clone_id).await?;
    Ok(bookmarks)
}

pub async fn create(message: &str, deployment_id: &str, clone_id: &str) -> anyhow::Result<NewBookmarkDTO> {
    let bookmark = bookmark_service::create_bookmark(message, deployment_id, clone_id).await?;
    Ok(bookmark)
}

pub async fn checkout(deployment_id: &str, clone_id: &str, snapshot_id: &str) -> anyhow::Result<CheckoutBookmarkDTO> {
    let bookmark = bookmark_service::checkout_bookmark(deployment_id, clone_id, snapshot_id).await?;
    Ok(bookmark)
}
