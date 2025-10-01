use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StartLoginResponse {
    pub url: String,
    
}