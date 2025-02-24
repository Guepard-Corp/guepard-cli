use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BranchError {
    // TODO
    BranchError
}

impl fmt::Display for BranchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for BranchError  {}
