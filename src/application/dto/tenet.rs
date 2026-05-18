use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct TenetDeployRequest {
    pub tenant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_job_id: Option<String>,
    pub upstream_host: String,
    pub upstream_port: u16,
    pub masking_salt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_dir: Option<String>,
    pub config_yaml: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<u16>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenetDeployResponse {
    pub job_id: String,
    pub eval_id: String,
    pub alloc_id: String,
    pub node_id: String,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub proxy_port: Option<u16>,
    #[serde(default)]
    pub api_port: Option<u16>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deploy_request_json_omits_ports_when_none() {
        let r = TenetDeployRequest {
            tenant_id: "t1".to_string(),
            compute_job_id: None,
            upstream_host: "h".to_string(),
            upstream_port: 5432,
            masking_salt: "s".to_string(),
            config_dir: None,
            config_yaml: "x: 1".to_string(),
            proxy_port: None,
            api_port: None,
        };
        let v = serde_json::to_value(&r).unwrap();
        assert!(v.get("proxy_port").is_none());
        assert!(v.get("api_port").is_none());
        assert_eq!(v.get("tenant_id").unwrap(), "t1");
    }

    #[test]
    fn deploy_request_json_includes_ports_when_set() {
        let r = TenetDeployRequest {
            tenant_id: "t1".to_string(),
            compute_job_id: Some("t1-compute".to_string()),
            upstream_host: "h".to_string(),
            upstream_port: 29789,
            masking_salt: "s".to_string(),
            config_dir: None,
            config_yaml: "rules: []".to_string(),
            proxy_port: Some(6544),
            api_port: Some(3010),
        };
        let v = serde_json::to_value(&r).unwrap();
        assert_eq!(v.get("proxy_port").and_then(|x| x.as_u64()), Some(6544));
        assert_eq!(v.get("api_port").and_then(|x| x.as_u64()), Some(3010));
    }

    #[test]
    fn deploy_response_deserializes_optional_host_ports() {
        let j = r#"{"job_id":"j","eval_id":"e","alloc_id":"a","node_id":"n","host":"10.0.0.1","proxy_port":1234,"api_port":3010}"#;
        let r: TenetDeployResponse = serde_json::from_str(j).unwrap();
        assert_eq!(r.host.as_deref(), Some("10.0.0.1"));
        assert_eq!(r.proxy_port, Some(1234));
        assert_eq!(r.api_port, Some(3010));
    }

    #[test]
    fn deploy_response_deserializes_without_host_ports() {
        let j = r#"{"job_id":"j","eval_id":"e","alloc_id":"a","node_id":"n"}"#;
        let r: TenetDeployResponse = serde_json::from_str(j).unwrap();
        assert!(r.host.is_none());
        assert!(r.proxy_port.is_none());
        assert!(r.api_port.is_none());
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TenetProxyYamlSetRequest {
    pub config_yaml: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TenetLifecycleResponse {
    #[serde(default)]
    pub message: Option<String>,
}
