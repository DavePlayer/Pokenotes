use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use juniper::{graphql_object};
use error_stack::{IntoReport, Result, ResultExt};
use surrealdb::sql::Value;

use crate::{database::Database, errors::{AnyError, self, DatabaseError}, utils::remove_surrealdb_id_prefix};

use super::pokemon;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    // making
    #[serde(deserialize_with = "remove_surrealdb_id_prefix")]
    pub id: uuid::Uuid,
    pub name: String,
    pub pokemons: Vec<String>
}

#[graphql_object(context = Database)]
impl Game {
    /// pokemons which occour in specific game
    async fn pokemons(&self, context: &Database) -> Vec<pokemon::Pokemon> {
        let sql = "SELECT id, name, games_occurrence from pokemons where $array contains id";
        let pokemons: Vec<&str> = self.pokemons.iter().map(|s| s.as_ref()).collect();
        println!("pokemons from games: {:?}", pokemons);
        let vars: BTreeMap<String, Value> = [
            ("array".into(), pokemons.into()),
        ].into();
        let Ok(results) = context.connection
            .execute(sql, &context.session, Some(vars), false)
            .await
            .into_report()
            .attach_printable(format!("error with database"))
            .change_context(AnyError::DatabaseError(errors::DatabaseError::ExecuteSQL(
                "couldn't CREATE pokemon in table".into(),
                sql.to_string(),
            ))) else {println!("error when executing sql");return vec![];};

        Database::print_surreal_response(&results).unwrap();

        let result = results
        .into_iter()
        .next().
        map(|r| r.result)
        .transpose()
        .into_report()
        .change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData));

        let res =  match result {
           Ok(res) => match res {
            Some(res) => res,
            None => {return vec![]}  
           },
           Err(err) => {println!("error: {:?}", err); return vec![]}
        };
        let Ok(json) = serde_json::to_string(&res).into_report().change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData)) else {return vec![]};
        println!("json: {:#?}", json);
        let poks: Result<Vec<pokemon::Pokemon>, AnyError> = serde_json::from_str(&json).into_report().change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData)).attach_printable("kill me not working aaaaaa");
        let poks = match poks {
            Ok(val) => val,
            Err(err) => {
                println!("{:#?}",err);
                return vec![];
            }
        };
        return poks;
    }
    fn new() -> Game {
        todo!();
    }
    /// name of the game
    fn name(&self) -> &str {
        self.name.as_str()
    }
    /// game id
    fn id(&self) -> uuid::Uuid {
        self.id
    }
}