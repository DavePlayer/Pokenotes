use std::collections::BTreeMap;

use juniper::graphql_object;
use surrealdb::sql::Value;
use serde::{Serialize, Deserialize};

use crate::database::Database;

use super::game::Game;

pub mod stat;


#[derive(Clone, Debug, Serialize, Deserialize)]
/// simple user object
pub struct Pokemon {
    pub id: String,
    pub name: String,
    // pub games_occurrence: Vec<Game>,
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
    fn id(&self) -> String {
        self.id.to_string()
    }
    /// games in which pokemon occours
    fn games_occurrence(&self) -> Vec<Game> {
        // &self.games_occurrence
        vec![]
    }
}

impl From<Pokemon> for Value {
    fn from(val: Pokemon) -> Self {

        let mut value: BTreeMap<String, Value> = BTreeMap::new();
        value.insert("id".into(), val.id.into());
        value.insert("name".into(), val.name.into());
        Value::from(value)
    }
}
