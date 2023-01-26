use crate::config::*;
use crate::errors::{self, AnyError, DatabaseError};
use crate::graphql::schemas::pokemon;
use crate::graphql::schemas::{game::Game, pokemon::Pokemon};
use crate::prelude::W;
use colored::Colorize;
use error_stack::{IntoReport, Result, ResultExt};
use surrealdb::sql::Value;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::os::fd::AsFd;
use std::path::Path;
use std::result;
use surrealdb::{Datastore, Session};

// use surrealdb::Surreal;

pub(crate) mod bakadata;
use bakadata::Data as Bakadata;

// #[derive(Default)]
pub struct Database {
    pub db_path: String,
    pub connection: Datastore,
    pub session: Session
}

impl Database {
    pub async fn new() -> Result<Database, AnyError> {
        let config = Config::get_config()?;
        let path = config.dbFilePath;

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


        return Ok(Database {
            db_path: path,
            connection,
            session
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

        let baka_data: Bakadata = serde_yaml::from_str(&baka_data_string).unwrap();
        let games = &baka_data.games;
        let pokemons = &baka_data.pokemons;

        for pokemon in pokemons {
            let sql = r#"CREATE type::thing("pokemons", $id) CONTENT $data"#;
            let stats = serde_json::to_string(&pokemon.stats).into_report().change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData))?;

            let data: BTreeMap<String, Value> = [
                ("name".into(), (*pokemon.name).into()),
                ("stats".into(), (stats).into()),
            ].into();

            let vars: BTreeMap<String, Value> = [
                ("id".into(), pokemon.id.to_string().into()),
                ("data".into(), data.into())
            ].into();

            let response = connection.execute(sql, &session, Some(vars), false)
            .await
            .into_report()
            .change_context(AnyError::DatabaseError(DatabaseError::ExecuteSQL("error when executing sql: ".into(), sql.into())))
            .attach_printable(format!["error when executing sql: {}", sql])?;

            Database::print_surreal_response(response)?;
        }

        for game in games {
            let sql = r#"CREATE type::thing("games", $id) CONTENT $data"#;
            let data: BTreeMap<String, Value> = [
                ("name".into(), (*game.name).into())
            ].into();

            let vars: BTreeMap<String, Value> = [
                ("id".into(), game.id.to_string().into()),
                ("data".into(), data.into())
            ].into();

            let response = connection.execute(sql, &session, Some(vars), false)
            .await
            .into_report()
            .change_context(AnyError::DatabaseError(DatabaseError::ExecuteSQL("error when executing sql: ".into(), sql.into())))
            .attach_printable(format!["error when executing sql: {}", sql])?;

            Database::print_surreal_response(response)?;

            for id in game.pokemons.iter() {
                let sql = r#"UPDATE type::thing("games", $id) SET pokemons += [type::thing("pokemons", $pokemonId)]"#;
                let vars: BTreeMap<String, Value> = [
                    ("id".into(), game.id.to_string().into()),
                    ("pokemonId".into(), id.to_string().into())
                ].into();

                let response = connection.execute(sql, &session, Some(vars), false)
                .await
                .into_report()
                .change_context(AnyError::DatabaseError(DatabaseError::ExecuteSQL("error when executing sql: ".into(), sql.into())))
                .attach_printable(format!["error when executing sql: {}", sql])?;

                Database::print_surreal_response(response)?;
            }
        }

        let sql = "SELECT id, name from games FETCH pokemons";
        let results = connection
            .execute(sql, &session, None, false)
            .await
            .into_report()
            .attach_printable(format!("error with database"))
            .change_context(AnyError::DatabaseError(errors::DatabaseError::ExecuteSQL(
                "couldn't CREATE pokemon in table".into(),
                sql.to_string(),
            )))?;
        Database::print_surreal_response(&results)?;
        let result = results
        .into_iter()
        .next().
        map(|r| r.result)
        .transpose()
        .into_report()
        .change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData))?;
        if let Some(res) = result {
           let json = serde_json::to_string(&res).into_report().change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData))?;
           let game: Game = serde_json::from_str(&json).into_report().change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData)).attach_printable("kill me not working aaaaaa")?;;
           println!("KURWA TAK NARESZCIE: {:#?}", game);
        }
        Ok(())
    }

    pub fn print_surreal_response<B>(response: B) -> Result<(), AnyError> 
        where
        B: AsRef<[surrealdb::Response]>,
    {
        let res = response.as_ref();
        for r in res {
            match &r.result {
                Ok(result) => println!("record: {}\n", result),
                Err(e) => println!("Error: {:?}", e),
            }
        }
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

    pub async fn get_game(&self, id: &i32) -> Option<&Vec<Game>> {
        None
    }
    pub async fn get_all_games(&self) -> Option<&Vec<Game>> {
        let sql = "SELECT * FROM games";
        let games: Vec<Game> = vec![];
        let results = self.connection
            .execute(sql, &self.session, None, false)
            .await
            .into_report()
            .attach_printable(format!("error with database"))
            .change_context(AnyError::DatabaseError(errors::DatabaseError::ExecuteSQL(
                "couldn't CREATE pokemon in table".into(),
                sql.to_string(),
            )));
        // let games: Vec<Game> = results.take(0);
        None
    }
    pub async fn get_pokemon(&self, id: &i32) -> Option<&Pokemon> {
        // let conn = self.establish_sql_connection().await;
        None
    }
    pub async fn get_all_pokemon(&self) -> Option<&Vec<Pokemon>> {
        None
    }
}

// To make our Database usable by Juniper, marker trait has to be implemented
impl juniper::Context for Database {}
