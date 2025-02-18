use std::string::ToString;

use crate::application::dto::volume_dto::{ListVolumeDTO, NewVolumeCloneDTO, NewVolumeDTO, NewVolumeGuidDTO, NewVolumeSnapshotDTO, VolumeDTO};
use crate::config::SCRIPTS_PATH;
use crate::domain::errors::volume_error::VolumeError;
use crate::domain::model::command::CommandList;
use crate::infrastructure::adapter::engine::local_connector::LocalConnector;
use crate::infrastructure::adapter::engine::parsers::engine_parser::parse_zfs_list;

pub fn list_volumes() -> Result<Vec<ListVolumeDTO>, VolumeError> {
    let commands = vec![
        CommandList {
            cmd: "sh".to_string(),
            args: vec![
                format!("{}/engine/zfs_list.sh", *SCRIPTS_PATH),
            ],
            rev: None,
        },
    ];

    let output = LocalConnector::execute(commands).unwrap();

    let volumes = parse_zfs_list(&output.first().unwrap());

    //convert volumes to VolumeDTO
    let mut volumes_dto = Vec::new();
    for volume in volumes {
        volumes_dto.push(ListVolumeDTO {
            name: volume.name.to_string(),
            used: volume.used,
            available: volume.avail,
            size: volume.size,
        });
    }

    Ok(volumes_dto)
}

pub fn create_volume(volume: NewVolumeDTO) -> Result<VolumeDTO, VolumeError> {
    let commands = vec![
        CommandList {
            cmd: "sh".to_string(),
            args: vec![
                format!("{}/engine/zfs_volume_create.sh", *SCRIPTS_PATH),
                volume.name.to_string(),
            ],
            rev: None,
        },
    ];

    let ret = LocalConnector::execute(commands);
    match ret {
        Ok(_) => Ok(VolumeDTO {
            name: volume.name.to_string(),
            size: 0,
        }),
        Err(_) => Err(VolumeError::VolumeAlreadyExists)
    }
}

pub fn create_dataset(volume: NewVolumeGuidDTO) -> Result<VolumeDTO, VolumeError> {

    let commands = vec![
        CommandList {
            cmd: "sh".to_string(),
            args: vec![
                format!("{}/engine/zfs_dataset_create.sh", *SCRIPTS_PATH),
                volume.name.to_string(),
                volume.uid.to_string(),
                volume.gid.to_string(),
                volume.customer_id.to_string(),
                volume.quota.to_string(),
            ],
            rev: None,
        },
    ];

    let ret = LocalConnector::execute(commands);
    match ret {
        Ok(_) => Ok(VolumeDTO {
            name: volume.name.to_string(),
            size: 0,
        }),
        Err(_) => Err(VolumeError::VolumeAlreadyExists)
    }
}

pub fn destroy_volume(volume: NewVolumeDTO) -> Result<(), VolumeError> {
    let commands = vec![
        CommandList {
            cmd: "sh".to_string(),
            args: vec![
                format!("{}/engine/zfs_destroy.sh", *SCRIPTS_PATH),
                volume.name.to_string(),
            ],
            rev: None,
        },
    ];

    let ret = LocalConnector::execute(commands);
    match ret {
        Ok(_) => Ok(()),
        Err(_) => Err(VolumeError::VolumeAlreadyExists)
    }
}

pub fn clone_volume(clone: NewVolumeCloneDTO) -> Result<(), VolumeError> {
    let commands = vec![
        CommandList {
            cmd: "sh".to_string(),
            args: vec![
                format!("{}/engine/zfs_clone.sh", *SCRIPTS_PATH),
                clone.volume,
                clone.snapshot,
                clone.name,
                clone.uid.to_string(),
                clone.gid.to_string(),
                clone.customer_id.to_string(),
            ],
            rev: None,
        },
    ];

    let ret = LocalConnector::execute(commands);
    match ret {
        Ok(_) => Ok(()),
        Err(_) => {
            Err(VolumeError::VolumeAlreadyExists)
        }
    }
}

pub fn discard_changes(snapshot: NewVolumeSnapshotDTO) -> Result<(), VolumeError> {
    let commands = vec![
        CommandList {
            cmd: "sh".to_string(),
            args: vec![
                format!("{}/engine/zfs_discard.sh", *SCRIPTS_PATH),
                snapshot.volume,
                snapshot.name,
            ],
            rev: None,
        },
    ];

    let ret = LocalConnector::execute(commands);
    match ret {
        Ok(_) => Ok(()),
        Err(_) => Err(VolumeError::RollbackError)
    }
}

pub fn get_volume_usage(volume_names: &[String]) -> Result<f64, VolumeError> {

    // Prepare the command to execute the script
    let mut args = vec![format!("{}/engine/zfs_volume_usage.sh", *SCRIPTS_PATH)];
    
    // Prepend "tank/" to each volume name
    let tank_volumes: Vec<String> = volume_names.iter()
        .map(|name| format!("tank/{}", name))
        .collect();

    args.extend(tank_volumes);

    let commands = vec![
        CommandList {
            cmd: "sh".to_string(),
            args,
            rev: None,
        },
    ];

    // Execute the command using LocalConnector
    let output = LocalConnector::execute(commands).map_err(|_| VolumeError::ScriptExecutionFailed)?;

    // Parse the output
    if let Some(stdout) = output.first() {
        let usage: f64 = stdout.trim().parse().map_err(|_| VolumeError::InvalidOutput)?;

        // Return the usage
        return Ok(usage);
    }

    Err(VolumeError::InvalidOutput)
}