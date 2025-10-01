use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct CompleteLoginRequest {
    pub session_id: String,
    pub verification_code: String,
}

#[derive(Deserialize, Debug)]
pub struct CompleteLoginResponse {
    pub token: String,
}