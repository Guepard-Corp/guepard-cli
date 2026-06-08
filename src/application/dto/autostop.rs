use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutostopResponse {
    pub action: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutostopStatusResponse {
    pub deployment_id: String,
    pub autostop: bool,
    pub idle_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutostopConfigureRequest {
    pub idle_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutostopConfigureResponse {
    pub job_id: String,
    pub idle_duration: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autostop_responses_deserialize() {
        let enable: AutostopResponse =
            serde_json::from_str(r#"{"action":"enable","message":"ok","skipped":true}"#)
                .unwrap();
        assert_eq!(enable.skipped, Some(true));

        let status: AutostopStatusResponse = serde_json::from_str(
            r#"{"deployment_id":"d1","autostop":true,"idle_duration":"15m"}"#,
        )
        .unwrap();
        assert!(status.autostop);

        let configure: AutostopConfigureResponse = serde_json::from_str(
            r#"{"job_id":"j1","idle_duration":"45m","message":"configured"}"#,
        )
        .unwrap();
        assert_eq!(configure.idle_duration, "45m");
    }
}
