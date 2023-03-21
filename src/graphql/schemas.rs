use crate::{graphql::Database};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

pub mod pokemon;
pub mod game;
pub mod pokedex;

/// Pokemon game object
pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    /// pokemon array
    pub async fn pokemons(context: &Database) -> Vec<pokemon::Pokemon> {
        match context.get_all_pokemon().await {
            Ok(val) => val,
            Err(err) => {eprintln!("{:?}",err);return vec![]},
        }
    }
    /// games array
    pub async fn games(context: &Database) -> Vec<game::Game> {
        match context.get_all_games().await {
            Ok(val) => val,
            Err(err) => {eprintln!("{:?}",err);return vec![]},
        }
    }
    
    #[allow(unused)]
    pub async fn pokedexes(context: &Database) -> Vec<pokedex::Pokedex> {
        match context.get_all_pokedexes().await {
            Ok(val) => val,
            Err(err) => {eprintln!("{:?}",err);return vec![]},
        }
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}
