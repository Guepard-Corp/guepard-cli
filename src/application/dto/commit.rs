use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetCommitResponse {
    pub id: String, // bookmark_id = snapshot_id
    pub name: String,  // bookmark_name = snapshot_name
    pub status: String,
    pub dataset_id: Option<String>, // dataset_id = branch_id
    pub parent_id: Option<String>,
    pub created_by: String,
    pub created_date: String,
    pub snapshot_type: String,
    pub is_ephemeral: bool,
    pub snapshot_comment: String,
    pub schema: Option<BookmarkSchema>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommitResponse {
    pub id: String, // bookmark_id = snapshot_id
    pub name: String,
    pub status: String,
    pub snapshot_type: String,
    pub snapshot_comment: String,
    pub snapshot_schema: BookmarkSchema,
    pub customer_id: String,
    pub dataset_id: String, // clone_id = branch_id
    pub deployment_id: String, // repository_id
    pub parent_id: Option<String>,
    pub is_ephemeral: bool,
    pub is_golden: bool,
    pub created_by: String,
    pub created_date: String,
    pub last_modified_by: Option<String>,
    pub last_modified_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutCommitResponse {
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
    pub info: Option<String>,
    pub databases: Option<Vec<Database>>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub id: String,
    pub name: String,
    pub tables: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommitRequest {
    pub snapshot_comment: String,
}