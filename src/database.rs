use crate::graphql::schemas::{Game, Pokemon};

#[derive(Clone, Default)]
pub struct Database {
    ///this could be a database connection
    pub users: Vec<Pokemon>,
    /// games that user have
    pub games: Vec<Game>,
}

impl Database {
    pub fn new() -> Database {
        let mut users: Vec<Pokemon> = Vec::new();
        let mut games: Vec<Game> = Vec::new();
        games.push(Game {
            id: 1,
            name: "Pokémon Diamond".into(),
        });
        games.push(Game {
            id: 1,
            name: "Pokémon Pearl".into(),
        });
        games.push(Game {
            id: 2,
            name: "Pokémon Red".into(),
        });
        games.push(Game {
            id: 2,
            name: "Pokémon Blue".into(),
        });
        games.push(Game {
            id: 3,
            name: "Pokémon X".into(),
        });
        games.push(Game {
            id: 3,
            name: "Pokémon Y".into(),
        });
        users.push(Pokemon {
            id: 1,
            name: "Bulbasaur".into(),
            games_occurrence: vec![games[0].clone(), games[1].clone()],
        });
        users.push(Pokemon {
            id: 2,
            games_occurrence: vec![games[1].clone(), games[2].clone()],
            name: "Squirtle".into(),
        });
        users.push(Pokemon {
            id: 3,
            name: "Ratata".into(),
            games_occurrence: vec![games[0].clone(), games[2].clone()],
        });
        users.push(Pokemon {
            id: 4,
            name: "Biduf".into(),
            games_occurrence: vec![games[1].clone(), games[0].clone()],
        });
        Database { users, games }
    }
    pub fn get_user(&self, id: &i32) -> Option<&Pokemon> {
        self.users.iter().find(|&pokemon| pokemon.id == *id)
    }
    pub fn get_all_users(&self) -> Option<&Vec<Pokemon>> {
        Some(&self.users)
    }
}

// To make our Database usable by Juniper, marker trait has to be implemented
impl juniper::Context for Database {}
