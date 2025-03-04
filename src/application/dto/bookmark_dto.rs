use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetBookmarkResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub clone_id: String,
    pub parent_id: Option<String>,
    pub created_by: String,
    pub created_date: String,
    pub snapshot_type: String,
    pub is_ephemeral: bool,
    pub snapshot_comment: String,
    pub schema: BookmarkSchema,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookmarkResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub snapshot_type: String,
    pub snapshot_comment: String,
    pub snapshot_schema: BookmarkSchema,
    pub customer_id: String,
    pub dataset_id: String,
    pub deployment_id: String,
    pub parent_id: Option<String>,
    pub is_ephemeral: bool,
    pub is_golden: bool,
    pub created_by: String,
    pub created_date: String,
    pub last_modified_by: Option<String>,
    pub last_modified_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutBookmarkResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub snapshot_id: String,
    pub deployment_id: String,
    pub environment_type: Option<String>,
    pub database_provider: String,
    pub database_username: String,
    pub database_password: String,
    pub is_ephemeral: bool,
    pub is_masked: bool,
    pub is_purged: bool,
    pub created_by: String,
    pub created_date: String,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkSchema {
    pub databases: Vec<Database>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub id: String,
    pub name: String,
    pub tables: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBookmarkRequest {
    pub snapshot_comment: String,
}