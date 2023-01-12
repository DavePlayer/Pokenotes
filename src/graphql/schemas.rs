use crate::graphql::Database;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

pub mod pokemon;
pub mod game;

/// Pokemon game object
pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    /// pokemon array
    pub async fn pokemons(context: &Database) -> Option<&Vec<pokemon::Pokemon>> {
        context.get_all_pokemon().await
    }
    /// games array
    pub fn games(context: &Database) -> &Vec<game::Game> {
        &context.games
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
