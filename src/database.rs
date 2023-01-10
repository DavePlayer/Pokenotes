use crate::config::*;
use crate::errors::{self, AnyError, DatabaseError};
use crate::graphql::schemas::{Game, Pokemon};
use colored::Colorize;
use error_stack::{IntoReport, Result, ResultExt};
use std::path::Path;
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
    pub async fn new() -> Result<Database, AnyError> {
        let config = Config::get_config()?;
        let path = config.dbFilePath;

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


    pub async fn fill_dummy_data() -> Result<(), AnyError> {
        let config = Config::get_config()?;
        let path = config.dbFilePath;
        println!("{}{}", "connecting to database: ".cyan(), path.cyan());
        let connection = Datastore::new(&format!("file://{path}"))
            .await
            .into_report()
            .attach_printable(format!("couldn't establish connection to database: {path}"))
            .change_context(AnyError::DatabaseError(
                errors::DatabaseError::EstablishConnectionError(
                    "couldn't establish connection with db".to_string(),
                ),
            ))?;

        let session = Session::for_db("sth", "pokemons");

        // let sql = "CREATE pokemon SET name='Bulbasaur'";
        // let results = connection
        //     .execute(sql, &session, None, false)
        //     .await
        //     .into_report()
        //     .attach_printable(format!("error with database"))
        //     .change_context(AnyError::DatabaseError(errors::DatabaseError::ExecuteSQL(
        //         "couldn't CREATE pokemon in table".into(),
        //         sql.to_string(),
        //     )))?;
        // println!("{results:?}");
        let baka_data_string = std::fs::read_to_string("./baka_data.yaml")
        .into_report()
        .change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData))
        .attach_printable(format!["couldn't parse file to string: {}", "./baka_data.yaml"])?;

        // let baka_data = serde_yaml::from_str(&baka_data_string);

        // must refactor graphql schemas

        // let sql = "SELECT * FROM pokemon";
        // let results = connection
        //     .execute(sql, &session, None, false)
        //     .await
        //     .into_report()
        //     .attach_printable(format!("error with database"))
        //     .change_context(AnyError::DatabaseError(errors::DatabaseError::ExecuteSQL(
        //         "couldn't CREATE pokemon in table".into(),
        //         sql.to_string(),
        //     )))?;
        // println!("{results:?}");
        Ok(())
    }
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
