use juniper::graphql_object;
use serde::{Serialize, Deserialize};

use crate::{database::Database, utils::remove_surrealdb_id_prefix};

use super::game::Game;

pub mod stat;


#[derive(Clone, Debug, Serialize, Deserialize)]
/// simple user object
pub struct Pokemon {
    #[serde(deserialize_with = "remove_surrealdb_id_prefix")]
    pub id: uuid::Uuid,
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
    fn id(&self) -> uuid::Uuid {
        self.id.clone()
    }
    /// games in which pokemon occours
    fn games_occurrence(&self) -> Vec<Game> {
        
        vec![]
    }
}