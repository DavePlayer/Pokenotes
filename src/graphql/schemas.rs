use crate::graphql::Database;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

#[derive(Clone, Debug)]
/// simple user object
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub games_occurrence: Vec<Game>,
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

#[derive(Clone, Debug)]
/// Pokemon game object
pub struct Game {
    pub id: i32,
    pub name: String,
}

#[graphql_object(context = Database)]
impl Game {
    /// pokemons which occour in specific game
    async fn pokemons(&self, context: &Database) -> Vec<&Pokemon> {
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
    /// name of the game
    fn name(&self) -> &str {
        self.name.as_str()
    }
    /// game id
    fn id(&self) -> i32 {
        self.id
    }
}
pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    /// pokemon array
    pub async fn pokemons(context: &Database) -> Option<&Vec<Pokemon>> {
        context.get_all_pokemon().await
    }
    /// games array
    pub fn games(context: &Database) -> &Vec<Game> {
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
