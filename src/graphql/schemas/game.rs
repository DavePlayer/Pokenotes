use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Array, Object, Value};

use juniper::{graphql_object, GraphQLValue};
use uuid::Uuid;

use crate::{database::Database, errors::AnyError};

use super::pokemon::{self, Pokemon};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
}

#[graphql_object(context = Database)]
impl Game {
    /// pokemons which occour in specific game
    async fn pokemons(&self, context: &Database) -> Vec<&pokemon::Pokemon> {
        let data = context.get_all_pokemon().await;
        if let Some(data) = data {
            let poks = data
                .iter()
                .filter(|user| user.games_occurrence.iter().any(|game| game.id == self.id))
                .collect();
            return poks;
        } else {
            return vec![];
        }
    }
    fn new() -> Game {
        todo!();
    }
    /// name of the game
    fn name(&self) -> &str {
        self.name.as_str()
    }
    /// game id
    fn id(&self) -> &str {
        self.id.as_str()
    }
}