use anyhow::Result;
use crate::config::config::Config;
use std::fs;

pub async fn log(_config: &Config) -> Result<()> {
    // Find the .gfs directory
    let gfs_dir = find_gfs_dir()?;
    
    // Read commits directory
    let commits_dir = gfs_dir.join("commits");
    if !commits_dir.exists() {
        println!("No commits found.");
        return Ok(());
    }
    
    // Read all commit files
    let mut commits = Vec::new();
    for entry in fs::read_dir(&commits_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                let content = fs::read_to_string(&path)?;
                commits.push((filename.to_string(), content));
            }
        }
    }
    
    // Sort commits by timestamp (filename is timestamp-based)
    commits.sort_by(|a, b| b.0.cmp(&a.0));
    
    // Display commits
    for (_hash, content) in commits {
        println!("{}", content);
        println!(); // Empty line between commits
    }
    
    Ok(())
}

fn find_gfs_dir() -> Result<std::path::PathBuf> {
    let mut current_dir = std::env::current_dir()?;
    
    loop {
        let gfs_dir = current_dir.join(".gfs");
        if gfs_dir.exists() {
            return Ok(gfs_dir);
        }
        
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            return Err(anyhow::anyhow!("Not a Guepard repository (or any of the parent directories): .gfs"));
        }
    }
}
