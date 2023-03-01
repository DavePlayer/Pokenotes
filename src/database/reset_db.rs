use crate::config::*;
use crate::errors::{self, AnyError, DatabaseError};
use colored::Colorize;
use error_stack::{IntoReport, Result, ResultExt};
use std::path::Path;

use super::Database;

impl Database {
    pub fn reset_db() -> Result<(), AnyError> {
            let config = Config::get_config()?;
            let db_path = Path::new(&config.dbFilePath);
            if db_path.exists() {
                match std::fs::remove_dir_all(&db_path).into_report() {
                    Ok(_) => {
                        println!(
                            "{}{}{}",
                            "removing ".on_red(),
                            config.dbFilePath.on_red(),
                            " database".on_red()
                        );
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(err
                            .change_context(errors::AnyError::DatabaseError(DatabaseError::Other)))
                            .attach_printable(format!(
                                "couldn't remove database file: {}",
                                config.dbFilePath
                            ));
                    }
                }
            } else {
                println!(
                    "{}",
                    "db file does not exist. reset was not necessary".blue()
                );
                    Ok(())
            }
    }
}