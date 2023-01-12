use juniper::graphql_object;

use crate::database::Database;

use super::game::Game;

pub mod stat;


#[derive(Clone, Debug)]
/// simple user object
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub games_occurrence: Vec<Game>,
    // pokedexes: []
    // genders: []
    // moves: []
    // abilities: []
    // types: []
    // locations: []
    stats: [stat::Stat; 6]
}

#[graphql_object(context = Database)]
impl Pokemon {
    /// pokemon identyfier (not sure why i can't add uuid)
    async fn pokemon(context: &Database, id: Option<i32>) -> Option<&Pokemon> {
        if let Some(id) = id {
            context.get_pokemon(&id).await
        } else {
            None
        }
    }
    /// pokemon name (don't know what you expected)
    fn name(&self) -> &str {
        self.name.as_str()
    }
    /// pokemon id
    fn id(&self) -> i32 {
        self.id
    }
    /// games in which pokemon occours
    fn games_occurrence(&self) -> &Vec<Game> {
        &self.games_occurrence
    }
}