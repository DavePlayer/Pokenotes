use crate::errors::ConfigError;
use directories::ProjectDirs;
use error_stack::{IntoReport, Report, Result, ResultExt};
use serde::{Deserialize, Serialize};
use std::{io::Write, process::Termination};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub dbFilePath: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Config, ConfigError> {
        let file_string = std::fs::read_to_string(path);
        let mut err_type: Option<std::io::ErrorKind> = None;
        if let Err(err) = &file_string {
            err_type = Some(err.kind());
        }
        let file_string = file_string
            .into_report()
            .attach_printable("can't parse file to string")
            .change_context(ConfigError::InvalidConfigPath);
        let file_string = match file_string {
            Ok(val) => val,
            Err(err) => {
                println!("{:?}", err);
                if err_type.unwrap() == std::io::ErrorKind::NotFound {
                    print!("\nMost likely the config.yaml does not exist. Do you want me to create it at {}? (yes/_): ", path);
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
                    };
                    let example_config = serde_yaml::to_string(&example_config)
                        .into_report()
                        .change_context(ConfigError::InvalidYamlStructure)
                        .attach_printable("at this point you can kill me. no clue what happens")?;
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            input = input.trim().to_string();
                            if input == String::from("yes") {
                                println!("creating config.yaml at {}", path);
                                std::fs::write(path, &example_config)
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
}
