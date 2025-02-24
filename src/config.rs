use std::env::var;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SCRIPTS_PATH: String = var("SCRIPTS_PATH").unwrap_or("/etc/guepard".to_string());
    pub static ref API_URL: String = var("API_URL").unwrap_or("https://api.guepard.run".to_string());
    pub static ref BEARER_TOKEN: String = var("BEARER_TOKEN").expect("BEARER_TOKEN in .env is required");
}
