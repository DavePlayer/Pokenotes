use crate::errors::{ConfigError, AnyError, DatabaseError};
use colored::Colorize;
use directories::ProjectDirs;
use error_stack::{IntoReport, Report, Result, ResultExt};
use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub dbFilePath: String,
    pub configDirPath: String,
}

impl Config {
    fn parse_database_file(path: &str) -> PathBuf {
        let path = PathBuf::from(path);
        let db_path = path.join("config.yaml");
        db_path
    }
    pub fn new(config_dir_path: &str) -> Result<Config, ConfigError> {
        let db_path = Config::parse_database_file(config_dir_path);
        let db_path = db_path.to_str().unwrap_or("");
        let file_string = std::fs::read_to_string(db_path);
        let mut err_type: Option<std::io::ErrorKind> = None;
        if let Err(err) = &file_string {
            err_type = Some(err.kind());
        }
        let file_string = file_string
            .into_report()
            .attach_printable(format!("can't parse file to string ({})", db_path))
            .change_context(ConfigError::InvalidConfigPath);
        let file_string = match file_string {
            Ok(val) => val,
            Err(err) => {
                println!("{:?}", err);
                if err_type.unwrap() == std::io::ErrorKind::NotFound {
                    print!("\nMost likely the config.yaml does not exist. Do you want me to create it at {}? (yes/_): ", db_path);
                    std::io::stdout().flush().unwrap();
                    let mut input: String = format!("");
                    let project_path = match ProjectDirs::from("io", "OmegaLoveIssac", "pokenotes")
                    {
                        Some(val) => Ok(val),
                        None => Err(Report::new(ConfigError::Other).attach_printable(
                            "couldn't get project directory when creating project config file",
                        )),
                    }?
                    .config_dir()
                    .to_owned()
                    .join("database.db");
                    let project_path = project_path.to_str();
                    if project_path.is_none() {
                        return Err(Report::new(ConfigError::Other)
                            .attach_printable("couldn't transform project dir path to string"));
                    }
                    let example_config = Config {
                        dbFilePath: project_path.unwrap().to_string(),
                        configDirPath: config_dir_path.to_string(),
                    };
                    let example_config = serde_yaml::to_string(&example_config)
                        .into_report()
                        .change_context(ConfigError::InvalidYamlStructure)
                        .attach_printable("at this point you can kill me. no clue what happens")?;
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            input = input.trim().to_string();
                            if input == String::from("yes") {
                                println!("{}{}","creating config.yaml at ".cyan(), db_path.cyan());
                                std::fs::write(db_path, &example_config)
                                    .into_report()
                                    .change_context(ConfigError::WritingError)?;
                            } else {
                                println!("if not than go away Baka!");
                                std::process::exit(1);
                            }
                        }
                        Err(_) => {}
                    }
                    example_config
                } else {
                    return Err(err);
                }
            }
        };
        let conf: Config = serde_yaml::from_str(&file_string.trim())
            .into_report()
            .attach_printable(format!(
                "can't parse file string to yaml obj from string: \"{}\"",
                file_string.trim()
            ))
            .change_context(ConfigError::InvalidYamlStructure)?;

        Ok(Config { ..conf })
    }

    pub fn get_config() -> Result<Config, AnyError> {

        let path = match ProjectDirs::from("io", "OmegaLoveIssac", "pokenotes") {
            Some(val) => Ok(val),
            None => Err(Report::new(AnyError::DatabaseError(DatabaseError::Other))
                .attach_printable("error when getting project directories")),
        }?;
        let config_file_path: PathBuf = path.config_dir().to_path_buf();
        if config_file_path.exists() == false {
            print!("{}{}", config_file_path.to_str().unwrap_or("arr - parse to str??? ").bright_red(), " config directory does not exist. Do you want to create it? (yes/_): ".bright_red());
            std::io::stdout().flush().unwrap();
            let mut input: String = format!("");
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    input = input.trim().to_string();
                    if input == String::from("yes") {
                        match std::fs::create_dir(&config_file_path)
                        .into_report()
                        .change_context(AnyError::ConfigError(ConfigError::WritingError))
                        .attach_printable("couldn't create config dir") {
                            Ok(_) => {},
                            Err(err) => {return Err(err)}
                        }
                    } else {
                        println!("if not than go away Baka!");
                        std::process::exit(1);
                    }
                }
                Err(_) => {}
            }
        }
        let config_file_path = match config_file_path.to_str() {
            Some(val) => val,
            None => {
                let err = Report::new(AnyError::ConfigError(ConfigError::Other))
                    .attach_printable("error when parsing config path to string");
                return Err(err);
            }
        };
        let config = match Config::new(config_file_path) {
            Ok(val) => val,
            Err(err) => {
                let err = err.change_context(AnyError::ConfigError(ConfigError::Other));
                return Err(err);
            }
        };
        Ok(config)
    }
}
