use juniper::graphql_object;
use serde::{Serialize, Deserialize};

use crate::{utils::remove_surrealdb_id_prefix, database::Database};

use self::pokedex_entry::PokedexEntry;

mod pokedex_entry;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pokedex {
    #[serde(deserialize_with = "remove_surrealdb_id_prefix")]
    /// uuid of pokedex
    pub id: uuid::Uuid,
    /// name of pokedex
    pub name: String,
    /// entry
    pub entries: Vec<PokedexEntry>
}

#[allow(unused)]
impl Pokedex {
    fn new() -> Pokedex {
        todo!();
    }
}

#[graphql_object(context = Database)]
impl Pokedex {
    fn id(&self) -> uuid::Uuid {
        self.id
    }
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}