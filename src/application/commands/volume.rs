use crate::application::dto::volume_dto::{ListVolumeDTO, NewVolumeCloneDTO, NewVolumeDTO, NewVolumeSnapshotDTO};
use crate::application::services::volume_service;
use crate::domain::errors::volume_error::VolumeError;
//act as an intermediary between the cli.rs and the volume_service.rs.
pub async fn list() -> Result<Vec<ListVolumeDTO>, VolumeError> {
    let result = volume_service::list_volumes();
    match result {
        Ok(snapshots) => Ok(snapshots),
        Err(error) => Err(error),
    }
}