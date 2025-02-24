use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DeploymentError {
    // TODO
    DeploymentError
}

impl fmt::Display for DeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DeploymentError  {}
