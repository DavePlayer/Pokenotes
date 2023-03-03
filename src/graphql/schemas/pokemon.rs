use std::collections::BTreeMap;

use error_stack::{IntoReport, ResultExt};
use juniper::graphql_object;
use serde::{Serialize, Deserialize};
use surrealdb::sql::Value;

use crate::{database::Database, utils::remove_surrealdb_id_prefix, errors::{AnyError, self, DatabaseError}};

use super::game::Game;

pub mod stat;


#[derive(Clone, Debug, Serialize, Deserialize)]
/// simple user object
pub struct Pokemon {
    #[serde(deserialize_with = "remove_surrealdb_id_prefix")]
    pub id: uuid::Uuid,
    pub name: String,
    pub games_occurrence: Vec<String>,
    // pokedexes: []
    // genders: []
    // moves: []
    // abilities: []
    // types: []
    // locations: []
    // stats: [stat::Stat; 7]
}

#[graphql_object(context = Database)]
impl Pokemon {
    /// pokemon name (don't know what you expected)
    fn name(&self) -> &str {
        self.name.as_str()
    }
    /// pokemon id
    fn id(&self) -> uuid::Uuid {
        self.id.clone()
    }
    /// games in which pokemon occours
    async fn games_occurrence(&self, context: &Database) -> Vec<Game> {
        let sql = "SELECT id, name, pokemons FROM games WHERE $array contains id";
        let games: Vec<&str> = self.games_occurrence.iter().map(|s| s.as_ref()).collect();
        let vars: BTreeMap<String, Value> = [
            ("array".into(), games.into()),
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

        // Database::print_surreal_response(&results).unwrap();

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
        // println!("json: {:#?}", json);
        let games: Result<Vec<Game>, _> = serde_json::from_str(&json).into_report().change_context(AnyError::DatabaseError(DatabaseError::ReadDummyData)).attach_printable("kill me not working aaaaaa");
        let games = match games {
            Ok(val) => val,
            Err(err) => {
                println!("{:#?}",err);
                return vec![];
            }
        };
        return games;
    }
}