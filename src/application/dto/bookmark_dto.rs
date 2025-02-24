use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub id: String,
    pub name: String,
    pub tables: Vec<String>, // so far, this list is always empty in responses, String is a dummy type
}

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub databases: Vec<Database>,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkDTO {
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
    pub schema: Schema,
}

#[derive(Debug, Deserialize)]
pub struct NewBookmarkDTO {
    pub id: String,
    pub name: String,
    pub status: String,
    pub snapshot_type: String,
    pub snapshot_comment: String,
    pub snapshot_schema: Schema,
    pub customer_id: String,
    pub dataset_id: String,
    pub deployment_id: String,
    pub parent_id: String,
    pub is_ephemeral: bool,
    pub is_golden: bool,
    pub created_by: String,
    pub created_date: String,
    pub last_modified_by: Option<String>,
    pub last_modified_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutBookmarkDTO {
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
