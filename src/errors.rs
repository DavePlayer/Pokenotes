use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DatabaseError {
    InitializingError,
    EstablishConnectionError(String),
    Other,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("rusqulite database error")
    }
}

impl Error for DatabaseError {}
