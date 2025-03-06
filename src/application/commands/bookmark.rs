use crate::application::dto::bookmark_dto::{CreateBookmarkRequest}; 
use crate::config::config::Config;
use crate::structure::{CreateBookmarkArgs, CheckoutBookmarkArgs};
use anyhow::Result;
use crate::application::dto::branch_dto::BranchRequest;
use crate::application::services::bookmark_service;

pub async fn list_all(deployment_id: &str, config: &Config) -> Result<()> {
    let bookmarks = bookmark_service::list_all_bookmarks(deployment_id, config).await?;
    if bookmarks.is_empty() {
        println!("ℹ️ No bookmarks found for deployment ID: {}", deployment_id);
        return Ok(());
    }
    println!("✅ Retrieved {} bookmarks:", bookmarks.len());
    for (i, bookmark) in bookmarks.iter().enumerate() {
        println!(
            "Bookmark #{}:\n\
             ID: {}\nName: {}\nStatus: {}\nClone ID: {}\nParent ID: {}\n\
             Created By: {}\nCreated Date: {}\nSnapshot Type: {}\nEphemeral: {}\nComment: {}\n\
             Databases: {}",
            i + 1, bookmark.id, bookmark.name, bookmark.status, bookmark.clone_id,
            bookmark.parent_id.as_ref().unwrap_or(&"None".to_string()),
            bookmark.created_by, bookmark.created_date, bookmark.snapshot_type,
            bookmark.is_ephemeral, bookmark.snapshot_comment,
            bookmark.schema.databases.iter().map(|db| db.name.clone()).collect::<Vec<String>>().join(", ")
        );
    }
    Ok(())
}

pub async fn list(deployment_id: &str, clone_id: &str, config: &Config) -> Result<()> {
    let bookmarks = bookmark_service::list_bookmark(deployment_id, clone_id, config).await?;
    if bookmarks.is_empty() {
        println!("ℹ️ No bookmarks found for clone ID: {} in deployment ID: {}", clone_id, deployment_id);
        return Ok(());
    }
    println!("✅ Retrieved {} bookmarks:", bookmarks.len());
    for (i, bookmark) in bookmarks.iter().enumerate() {
        println!(
            "Bookmark #{}:\n\
             ID: {}\nName: {}\nStatus: {}\nClone ID: {}\nParent ID: {}\n\
             Created By: {}\nCreated Date: {}\nSnapshot Type: {}\nEphemeral: {}\nComment: {}\n\
             Databases: {}",
            i + 1, bookmark.id, bookmark.name, bookmark.status, bookmark.clone_id,
            bookmark.parent_id.as_ref().unwrap_or(&"None".to_string()),
            bookmark.created_by, bookmark.created_date, bookmark.snapshot_type,
            bookmark.is_ephemeral, bookmark.snapshot_comment,
            bookmark.schema.databases.iter().map(|db| db.name.clone()).collect::<Vec<String>>().join(", ")
        );
    }
    Ok(())
}

pub async fn create(args: &CreateBookmarkArgs, config: &Config) -> Result<()> {
    let request = CreateBookmarkRequest {
        snapshot_comment: args.snapshot_comment.clone(),
    };
    let bookmark = bookmark_service::create_bookmark(&args.deployment_id, &args.clone_id, request, config).await?;
    println!(
        "✅ Bookmark Updated:\n\
         ID: {}\nName: {}\nStatus: {}\nSnapshot Type: {}\nComment: {}\n\
         Deployment ID: {}\nClone ID: {}\nParent ID: {}\nEphemeral: {}\nGolden: {}\n\
         Created By: {}\nCreated Date: {}\nLast Modified By: {}\nLast Modified Date: {}",
        bookmark.id, bookmark.name, bookmark.status, bookmark.snapshot_type, bookmark.snapshot_comment,
        bookmark.deployment_id, bookmark.dataset_id,
        bookmark.parent_id.as_ref().unwrap_or(&"None".to_string()),
        bookmark.is_ephemeral, bookmark.is_golden,
        bookmark.created_by, bookmark.created_date,
        bookmark.last_modified_by.as_ref().unwrap_or(&"None".to_string()),
        bookmark.last_modified_date.as_ref().unwrap_or(&"None".to_string())
    );
    Ok(())
}

pub async fn checkout(args: &CheckoutBookmarkArgs, config: &Config) -> Result<()> {
    let request = BranchRequest {
        discard_changes: Some(args.discard_changes.clone()),
        checkout: args.checkout,
        ephemeral: args.ephemeral,
    };
    let bookmark = bookmark_service::checkout_bookmark(
        &args.deployment_id, &args.clone_id, &args.snapshot_id, request, config
    ).await?;
    println!(
        "✅ Bookmark Checked Out:\n\
         ID: {}\nName: {}\nStatus: {}\nSnapshot ID: {}\nEnvironment Type: {}\n\
         Database Provider: {}\nUsername: {}\nPassword: {}\nEphemeral: {}\n\
         Masked: {}\nPurged: {}\nCreated By: {}\nCreated Date: {}",
        bookmark.id, bookmark.name, bookmark.status,
        bookmark.snapshot_id,
        bookmark.environment_type.as_ref().unwrap_or(&"None".to_string()),
        bookmark.database_provider,
        bookmark.database_username,
        bookmark.database_password,
        bookmark.is_ephemeral,
        bookmark.is_masked,
        bookmark.is_purged,
        bookmark.created_by, bookmark.created_date
    );
    Ok(())
}