use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Array, Object, Value};

use juniper::graphql_object;

use crate::{database::Database, errors::AnyError};

use super::pokemon::{self, Pokemon};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub pokemons: Vec<Pokemon>
    // pub pokedexes: null
    // pub locations: null
}



#[graphql_object(context = Database)]
impl Game {
    /// pokemons which occour in specific game
    async fn pokemons(&self, context: &Database) -> Vec<&pokemon::Pokemon> {
        let data = context.get_all_pokemon().await;
        if let Some(data) = data {
            let pokemons = data
                .iter()
                .filter(|user| user.games_occurrence.iter().any(|game| game.id == self.id))
                .collect();
            return pokemons;
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
    fn id(&self) -> i32 {
        self.id
    }
}

// impl From<Game> for Value {
//     fn from(val: Game) -> Self {

//         let mut value: BTreeMap<String, Value> = BTreeMap::new();
//         value.insert("id".into(), val.id.into());
//         value.insert("name".into(), val.name.into());
//         // value.insert("pokemons".into(), val.pokemons.into());
//         Value::from(value)
//     }
// }

impl TryFrom<Game> for Value {
    fn try_from(val: Game) -> Result<surrealdb::sql::Value, AnyError> {

        let mut value: BTreeMap<String, Value> = BTreeMap::new();
        value.insert("id".into(), val.id.into());
        value.insert("name".into(), val.name.into());
        // value.insert("pokemons".into(), val.pokemons.into());
        Ok(Value::from(value))
    }

    type Error = AnyError;
}

// impl Into<Game> for Value {
//     fn into(self) -> Game {
//         match self {
//             Value::Object(obj) => {
//                let json = serde_json::to_value(obj).unwrap();
//                let data = serde_json::from_value(json).unwrap();
//                todo!();
//             },
//             _ => {}
//         }
//     }
// }