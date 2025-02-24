use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BranchDTO {
    pub id: String,
    pub name: String,
    pub status: String,
    pub snapshot_id: String,
    pub deployment_id: String,
    pub environment_type: Option<String>, // these values are all null, for now set to String as placeholder
    pub database_provider: String,
    pub is_ephemeral: bool,
    pub is_masked: bool,
    pub is_purged: bool,
    pub created_by: String,
    pub created_date: String,
    pub clone_id: String,
}

#[derive(Debug, Deserialize)]
pub struct NewBranchDTO {
    pub id: String,
    pub name: String,
    pub status: String,
    pub snapshot_id: String,
    pub deployment_id: String,
    pub environment_type: Option<String>, // these values are all null, for now set to String as placeholder
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
pub struct CheckoutBranchDTO {
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
