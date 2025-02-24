use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ComputeError {
    // TODO
    ComputeError
}

impl fmt::Display for ComputeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ComputeError  {}
