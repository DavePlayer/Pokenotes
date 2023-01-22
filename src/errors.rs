use std::{error::Error, fmt};
use error_stack::{
    report, Report,
};

#[derive(Debug)]
pub enum AnyError {
    DatabaseError(DatabaseError),
    ConfigError(ConfigError),
}

#[derive(Debug)]
pub enum DatabaseError {
    EstablishConnectionError(String),
    ExecuteSQL(String, String),
    ReadDummyData,
    Other,
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidYamlStructure,
    InvalidConfigPath,
    WritingError,
    Other,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error with config")
    }
}

impl fmt::Display for AnyError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Some error accoured")
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Surreal database error")
    }
}

impl Error for DatabaseError {}
impl Error for AnyError {}
impl Error for ConfigError {}
