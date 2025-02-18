use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListVolumeDTO {
    pub name: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
}

#[derive(Serialize, Deserialize)]
pub struct VolumeDTO {
    pub name: String,
    pub size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct NewVolumeDTO {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewVolumeGuidDTO {
    pub name: String,
    pub uid: u32,
    pub gid: u32,
    pub customer_id: String,
    pub quota: u32,
}

#[derive(Serialize, Deserialize)]
pub struct NewVolumeSnapshotDTO {
    pub volume: String,
    pub name: String,
    pub customer_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewVolumeCloneDTO {
    pub volume: String,
    pub snapshot: String,
    pub name: String,
    pub uid: u32,
    pub gid: u32,
    pub customer_id: String,
}

impl NewVolumeCloneDTO {
    pub fn new(volume: &str, snapshot: &str, name: &str, uid: u32, gid: u32, customer_id: &str) -> Self {
        NewVolumeCloneDTO {
            volume: volume.to_string(),
            snapshot: snapshot.to_string(),
            name: name.to_string(),
            uid,
            gid,
            customer_id: customer_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewMountDTO {
    pub volume: String,
    pub host: String,
}

#[derive(Debug, Clone)]
pub struct VolumeUsageDTO {
    pub name: String,
    pub used_space: String,
    pub available_space: String,
}
