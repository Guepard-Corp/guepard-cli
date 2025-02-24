use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BookmarkError {
    // TODO
    BookmarkError
}

impl fmt::Display for BookmarkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for BookmarkError  {}
