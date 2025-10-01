use anyhow::Result;
use crate::config::config::Config;

pub async fn rev_parse(_config: &Config) -> Result<()> {
    let mut current_dir = std::env::current_dir()?;
    
    loop {
        let gfs_dir = current_dir.join(".gfs");
        if gfs_dir.exists() {
            println!("{}", current_dir.display());
            return Ok(());
        }
        
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            return Err(anyhow::anyhow!("Not a Guepard repository (or any of the parent directories): .gfs"));
        }
    }
}
