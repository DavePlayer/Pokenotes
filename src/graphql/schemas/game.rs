use juniper::graphql_object;

use crate::database::Database;

use super::pokemon::{self, Pokemon};


#[derive(Clone, Debug)]
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