use std::collections::BTreeMap;

use crate::config::*;
use crate::errors::{self, AnyError, DatabaseError};
use colored::Colorize;
use error_stack::{IntoReport, Result, ResultExt};
use surrealdb::sql::Value;
use surrealdb::{Datastore, Session};

use super::Database;
use crate::database::Bakadata;

impl Database {
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

        
        // reading dummy data from bakadata.yaml
        let baka_data_string = std::fs::read_to_string("./baka_data.yaml")
        .into_report()
        .change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData))
        .attach_printable(format!["couldn't parse file to string: {}", "./baka_data.yaml"])?;

        let baka_data: Bakadata = serde_yaml::from_str(&baka_data_string).unwrap();
        let games = &baka_data.games;
        let pokemons = &baka_data.pokemons;

        // adding dummy data pokemons to database
        for pokemon in pokemons {
            let sql = r#"CREATE type::thing("pokemons", $id) CONTENT $data"#;
            let stats = serde_json::to_string(&pokemon.stats).into_report().change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData))?;

            let data: BTreeMap<String, Value> = [
                ("name".into(), (*pokemon.name).into()),
                ("stats".into(), (stats).into()),
                ("games_occurrence".into(), (pokemon.games_occurrence.iter().map(|s| &**s).collect::<Vec<&str>>()).into()),
            ].into();

            let vars: BTreeMap<String, Value> = [
                ("id".into(), pokemon.id.to_string().into()),
                ("data".into(), data.into())
            ].into();

            let _response = connection.execute(sql, &session, Some(vars), false)
            .await
            .into_report()
            .change_context(AnyError::DatabaseError(DatabaseError::ExecuteSQL("error when executing sql: ".into(), sql.into())))
            .attach_printable(format!["error when executing sql: {}", sql])?;

        }

        // adding every dummy data game to database
        for game in games {
            let sql = r#"CREATE type::thing("games", $id) CONTENT $data"#;
            let data: BTreeMap<String, Value> = [
                ("name".into(), (*game.name).into())
            ].into();

            let vars: BTreeMap<String, Value> = [
                ("id".into(), game.id.to_string().into()),
                ("data".into(), data.into())
            ].into();

            let _response = connection.execute(sql, &session, Some(vars), false)
            .await
            .into_report()
            .change_context(AnyError::DatabaseError(DatabaseError::ExecuteSQL("error when executing sql: ".into(), sql.into())))
            .attach_printable(format!["error when executing sql: {}", sql])?;


            // linking pokemons to game.pokemons in database
            for id in game.pokemons.iter() {
                let sql = r#"UPDATE type::thing("games", $id) SET pokemons += [type::thing("pokemons", $pokemonId)]"#;
                let vars: BTreeMap<String, Value> = [
                    ("id".into(), game.id.to_string().into()),
                    ("pokemonId".into(), id.to_string().into())
                ].into();

                let _response = connection.execute(sql, &session, Some(vars), false)
                .await
                .into_report()
                .change_context(AnyError::DatabaseError(DatabaseError::ExecuteSQL("error when executing sql: ".into(), sql.into())))
                .attach_printable(format!["error when executing sql: {}", sql])?;
            }
        }
        Ok(())
    }
}