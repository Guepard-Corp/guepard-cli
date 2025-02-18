use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VolumeError {
    VolumeAlreadyExists,
    RollbackError,
    ScriptExecutionFailed,
    InvalidOutput,
    VolumeNotFound,
}

impl fmt::Display for VolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for VolumeError {}