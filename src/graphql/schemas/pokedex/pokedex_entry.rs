use serde::Serialize;
use crate::{graphql::schemas::pokemon::Pokemon, utils::remove_surrealdb_id_prefix};


#[derive(Clone, Debug, Serialize, serde::Deserialize)]
pub struct PokedexEntry {
    #[serde(deserialize_with = "remove_surrealdb_id_prefix")]
    /// id of specific pokedex entry
    pub id: uuid::Uuid,
    /// pokemon which entry applies to
    pub pokemon: Pokemon,
    /// what the description contains (pokemon info)
    pub description: String,
}