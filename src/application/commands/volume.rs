use crate::application::dto::volume_dto::{ListVolumeDTO, NewVolumeCloneDTO, NewVolumeDTO, NewVolumeSnapshotDTO};
use crate::application::services::volume_service;
use crate::domain::errors::volume_error::VolumeError;

pub async fn list() -> Result<Vec<ListVolumeDTO>, VolumeError> {
    let result = volume_service::list_volumes();
    match result {
        Ok(snapshots) => Ok(snapshots),
        Err(error) => Err(error),
    }
}