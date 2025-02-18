use std::env;
use std::process::Command;

use crate::config::SCRIPTS_PATH;
use crate::domain::model::version::Version;
use crate::domain::port::output::file_system_adapter::FileSystemAdapter;

pub struct EngineAdapter {}

impl EngineAdapter {
    pub fn new() -> Self {
        EngineAdapter {}
    }
}

impl FileSystemAdapter for EngineAdapter {
    fn version(&self) -> Result<Version, std::io::Error> {
        println!("{}", format!("{}/engine/zfs_version.sh", *SCRIPTS_PATH));
        let zfs_version = Command::new("sh")
            .arg(format!("{}/engine/zfs_version.sh", *SCRIPTS_PATH))
            .output()
            .expect("Failed to execute script");

        let os_version = Command::new("sh")
            .arg(format!("{}/engine/os_version.sh", *SCRIPTS_PATH))
            .output()
            .expect("Failed to execute script");

        let db_version = Command::new("sh")
            .arg(format!("{}/engine/db_version.sh", *SCRIPTS_PATH))
            .output()
            .expect("Failed to execute script");

        Ok(Version::new(
            env!("CARGO_PKG_VERSION").to_string(),
            String::from_utf8_lossy(&db_version.stdout).to_string(),
            String::from_utf8_lossy(&os_version.stdout).to_string(),
            String::from_utf8_lossy(&zfs_version.stdout).to_string(),
        ))
    }
    /*
    fn mount(&self, _volume: crate::domain::model::data_volume::DataVolume) {
        self.connector.execute(EngineCommands::MOUNT);
    }

    fn unmount(&self, _volume: crate::domain::model::data_volume::DataVolume) {
        self.connector.execute(EngineCommands::UNMOUNT);
    }

    fn create_volume(
        &self,
        _volume_name: &str,
        _size_in_gb: i32,
    ) -> crate::domain::model::data_volume::DataVolume {
        todo!()
    }

    fn get_volume(&self, _volume_name: &str) -> crate::domain::model::data_volume::DataVolume {
        todo!()
    }

    fn delete_volume(
        &self,
        _volume: crate::domain::model::data_volume::DataVolume,
    ) -> crate::domain::model::data_volume::DataVolume {
        todo!()
    }

    fn clone_from_snapshot(
        &self,
        _snapshot: crate::domain::model::snapshot::Snapshot,
        _clone_name: &str,
    ) -> crate::domain::model::data_volume::DataVolume {
        todo!()
    }

    fn create_snapshot(
        &self,
        _volume: crate::domain::model::data_volume::DataVolume,
        _snapshot_name: &str,
    ) -> crate::domain::model::snapshot::Snapshot {
        todo!()
    }

    fn delete_snapshot(
        &self,
        _snapshot: crate::domain::model::snapshot::Snapshot,
    ) -> crate::domain::model::snapshot::Snapshot {
        todo!()
    }

    fn clone_snapshot(
        &self,
        _snapshot: crate::domain::model::snapshot::Snapshot,
        _clone_name: &str,
    ) -> crate::domain::model::snapshot::Snapshot {
        todo!()
    }

    fn send_snapshot(&self) -> crate::domain::model::snapshot::Snapshot {
        todo!()
    }

    fn receive_snapshot(&self) -> crate::domain::model::snapshot::Snapshot {
        todo!()
    }

    fn add_compression(&self, _volume: crate::domain::model::data_volume::DataVolume) {
        todo!()
    }

    fn enable_deduplication(&self, _volume: crate::domain::model::data_volume::DataVolume) {
        todo!()
    }

    fn set_permissions(&self, _volume: crate::domain::model::data_volume::DataVolume) {
        todo!()
    }

    fn expose_as_nfs(&self, _volume: crate::domain::model::data_volume::DataVolume) -> String {
        todo!()
    }
     */
}