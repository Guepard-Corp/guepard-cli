use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessibleNode {
    pub id: String,
    pub label_name: String,
    #[serde(default)]
    pub node_status: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub node_pool: Option<String>,
    #[serde(default)]
    pub datacenter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CpuResources {
    pub total_mhz: i64,
    pub allocated_mhz: i64,
    pub reserved_mhz: i64,
    pub available_mhz: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryResources {
    pub total_mb: i64,
    pub allocated_mb: i64,
    pub reserved_mb: i64,
    pub available_mb: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeResourceAvailability {
    pub node_id: String,
    pub label_name: String,
    pub node_type: String,
    pub node_pool: String,
    pub datacenter: String,
    pub cpu: CpuResources,
    pub memory: MemoryResources,
    pub schedulable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_resources_deserialize_without_clients() {
        let json = r#"{
            "node_id":"n1",
            "label_name":"Guepard-Shared-Node",
            "node_type":"public",
            "node_pool":"us-west-aws",
            "datacenter":"us-west-aws",
            "cpu":{"total_mhz":5000,"allocated_mhz":0,"reserved_mhz":0,"available_mhz":5000},
            "memory":{"total_mb":1907,"allocated_mb":0,"reserved_mb":410,"available_mb":1497},
            "schedulable":true
        }"#;
        let r: NodeResourceAvailability = serde_json::from_str(json).unwrap();
        assert_eq!(r.node_type, "public");
        assert!(r.schedulable);
    }
}
