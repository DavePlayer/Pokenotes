use crate::{graphql::schemas::pokedex, errors::AnyError};

use super::Database;

impl Database {
    pub async fn get_all_pokedexes(&self) -> Result<Vec<pokedex::Pokedex>, AnyError> {

        Ok(vec![])
    }
}