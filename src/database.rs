use crate::errors::{self, DatabaseError};
use crate::graphql::schemas::{Game, Pokemon};
use colored::Colorize;
use directories::ProjectDirs;
use error_stack::{IntoReport, Report, Result, ResultExt};
use std::fs::create_dir;
use std::path::PathBuf;
use surrealdb::{Datastore, Session};

// #[derive(Default)]
pub struct Database {
    ///this could be a database connection
    pub pokemons: Vec<Pokemon>,
    /// games that user have
    pub games: Vec<Game>,
    pub db_path: String,
}

impl Database {
    pub async fn new() -> Result<Database, errors::DatabaseError> {
        let path = std::env::var("DBFILE")
            .into_report()
            .attach_printable(format!("error in database"))
            .change_context(errors::DatabaseError::EstablishConnectionError(
                "DBFILE in .env is wrong somehow".into(),
            ))?;

        let mut users: Vec<Pokemon> = Vec::new();
        let mut games: Vec<Game> = Vec::new();
        games.push(Game {
            id: 1,
            name: "Pokémon Diamond".into(),
        });
        games.push(Game {
            id: 1,
            name: "Pokémon Pearl".into(),
        });
        games.push(Game {
            id: 2,
            name: "Pokémon Red".into(),
        });
        games.push(Game {
            id: 2,
            name: "Pokémon Blue".into(),
        });
        games.push(Game {
            id: 3,
            name: "Pokémon X".into(),
        });
        games.push(Game {
            id: 3,
            name: "Pokémon Y".into(),
        });
        users.push(Pokemon {
            id: 1,
            name: "Bulbasaur".into(),
            games_occurrence: vec![games[0].clone(), games[1].clone()],
        });
        users.push(Pokemon {
            id: 2,
            games_occurrence: vec![games[1].clone(), games[2].clone()],
            name: "Squirtle".into(),
        });
        users.push(Pokemon {
            id: 3,
            name: "Ratata".into(),
            games_occurrence: vec![games[0].clone(), games[2].clone()],
        });
        users.push(Pokemon {
            id: 4,
            name: "Biduf".into(),
            games_occurrence: vec![games[1].clone(), games[0].clone()],
        });
        return Ok(Database {
            pokemons: users,
            games,
            db_path: path,
        });
    }

    pub async fn fill_dummy_data() -> Result<(), DatabaseError> {
        let name = std::env::var("DBFILE")
            .into_report()
            .attach_printable(format!("couldn't read DBFILE from .env"))
            .change_context(DatabaseError::Other)?;
        let path =
            match ProjectDirs::from("io", "OmegaLoveIssac", "pokenotes") {
                Some(val) => Ok(val),
                None => Err(Report::new(DatabaseError::Other)
                    .attach_printable("couldn't find project folder")),
            }?;
        let path = path.config_dir().join(name);
        let path = path
            .into_os_string()
            .into_string()
            .unwrap_or("".to_string())
            .chars()
            .filter(|l| *l != '"')
            .collect::<String>();
        println!("{}{}", "connecting to database: ".cyan(), path.cyan());
        let connection = Datastore::new(&format!("file://{path}"))
            .await
            .into_report()
            .attach_printable(format!("error in database"))
            .change_context(errors::DatabaseError::EstablishConnectionError(
                "couldn't establish connection with db".to_string(),
            ))?;

        let session = Session::for_db("sth", "pokemons");

        let sql = "CREATE pokemon SET name='Bulbasaur'";
        let results = connection
            .execute(sql, &session, None, false)
            .await
            .into_report()
            .attach_printable(format!("error with database"))
            .change_context(errors::DatabaseError::ExecuteSQL(
                "couldn't CREATE pokemon in table".into(),
                sql.to_string(),
            ))?;
        println!("{results:?}");

        let sql = "SELECT * FROM pokemon";
        let results = connection
            .execute(sql, &session, None, false)
            .await
            .into_report()
            .attach_printable(format!("error with database"))
            .change_context(errors::DatabaseError::ExecuteSQL(
                "couldn't CREATE pokemon in table".into(),
                sql.to_string(),
            ))?;
        println!("{results:?}");
        Ok(())
    }
    pub fn reset_db() -> Result<(), errors::DatabaseError> {
        if let Some(proj_dir) = ProjectDirs::from("io", "OmegaLoveIssac", "pokenotes") {
            let config_dir = proj_dir.config_dir();
            let name = std::env::var("DBFILE").unwrap_or("database.db".to_string());
            if config_dir.exists() == false {
                println!("craeting config direcotries: {}", config_dir.display());
                match std::fs::create_dir(config_dir).into_report() {
                    Ok(_) => {
                        println!(
                            "{}: {}",
                            "craeted new config directory".yellow(),
                            config_dir.display()
                        )
                    }
                    Err(err) => {
                        return Err(err.change_context(DatabaseError::Other).attach_printable(
                            format!("couldn't craete condig directory: {}", config_dir.display()),
                        ));
                    }
                }
            }
            let db_path: PathBuf = config_dir.join(std::path::Path::new(&name));
            if db_path.exists() {
                match std::fs::remove_dir_all(&db_path).into_report() {
                    Ok(_) => {
                        println!(
                            "{}{}{}",
                            "removing ".on_red(),
                            db_path.into_os_string().into_string().unwrap().on_red(),
                            " database".on_red()
                        );
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(err
                            .change_context(errors::DatabaseError::Other)
                            .attach_printable(format!(
                                "couldn't remove database file: {}",
                                db_path.display()
                            )));
                    }
                }
            } else {
                return Err(
                    Report::new(DatabaseError::Other).attach_printable("db file does not exist")
                );
            }
        }
        Err(Report::new(DatabaseError::Other).attach_printable("problem with config folder"))
    }

    pub async fn get_game(&self, id: &i32) -> Option<&Pokemon> {
        // let conn = self.establish_sql_connection().await;
        self.pokemons.iter().find(|&pokemon| pokemon.id == *id)
    }
    pub async fn get_all_games(&self) -> Option<&Vec<Pokemon>> {
        // let conn = self.establish_sql_connection().await;
        Some(&self.pokemons)
    }
    pub async fn get_pokemon(&self, id: &i32) -> Option<&Pokemon> {
        // let conn = self.establish_sql_connection().await;
        self.pokemons.iter().find(|&pokemon| pokemon.id == *id)
    }
    pub async fn get_all_pokemon(&self) -> Option<&Vec<Pokemon>> {
        return Some(&self.pokemons);
    }
}

// To make our Database usable by Juniper, marker trait has to be implemented
impl juniper::Context for Database {}
