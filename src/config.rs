use std::env::var;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SCRIPTS_PATH: String = var("SCRIPTS_PATH").unwrap_or("/etc/guepard".to_string());
}
