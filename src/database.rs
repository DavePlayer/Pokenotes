use crate::config::*;
use crate::errors::{self, AnyError};
use crate::graphql::schemas::{game::Game, pokemon::Pokemon};
use error_stack::{IntoReport, Result, ResultExt};
use surrealdb::{Datastore, Session};

// use surrealdb::Surreal;

pub(crate) mod bakadata;
use bakadata::Data as Bakadata;

mod fill_dummy_data;
mod reset_db;
mod get_all_games;
mod get_all_pokemons;
mod get_all_pokedexes;

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


    #[allow(unused)]
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


    #[allow(dead_code, unused_variables)]
    pub async fn get_game(&self, id: &i32) -> Option<&Vec<Game>> {
        None
    }
    #[allow(dead_code, unused_variables)]
    pub async fn get_pokemon(&self, id: &i32) -> Option<&Pokemon> {
        // let conn = self.establish_sql_connection().await;
        None
    }
}

// To make our Database usable by Juniper, marker trait has to be implemented
impl juniper::Context for Database {}
