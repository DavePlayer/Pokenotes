use crate::errors;
use crate::graphql::schemas::{Game, Pokemon};
use error_stack::{IntoReport, Result, ResultExt};
use rusqlite::Connection;

#[derive(Clone, Default)]
pub struct Database {
    ///this could be a database connection
    pub pokemons: Vec<Pokemon>,
    /// games that user have
    pub games: Vec<Game>,
}

impl Database {
    pub async fn new() -> Result<Database, errors::DatabaseError> {
        let conn = Connection::open("database.db")
            .report()
            .attach_printable(format!("error when establishing database connection"))
            .change_context(errors::DatabaseError::EstablishConnectionError(
                "couldn't connect to database.db".into(),
            ))?;
        // conn.execute(
        //     "INSERT INTO pokemons (id, name) VALUES (NULL, 'bulbasaur')",
        //     (), // empty list of parameters.
        // )
        // .report()
        // .attach_printable(format!("couldn't insert dummy pokemon into database"))
        // .change_context(errors::DatabaseError::Other)?;
        println!("db ok");
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
        return Ok(Database {
            pokemons: users,
            games,
        });
    }
    pub async fn establish_sql_connection(&self) -> Result<Connection, errors::DatabaseError> {
        Connection::open("database.db")
            .report()
            .attach_printable(format!("error when establishing database connection"))
            .change_context(errors::DatabaseError::EstablishConnectionError(
                "couldn't connect to database.db".into(),
            ))
    }
    pub async fn init_database(self) -> Result<Self, errors::DatabaseError> {
        // establish connection
        let conn = Connection::open("database.db")
            .report()
            .attach_printable(format!("error when establishing database connection"))
            .change_context(errors::DatabaseError::EstablishConnectionError(
                "couldn't connect to database.db".into(),
            ))?;

        // check if table Pokemons exist
        let mut statement = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='pokemons'")
            .report()
            .attach_printable(format!("couldn't execute database query"))
            .change_context(errors::DatabaseError::InitializingError);
        match statement {
            Ok(statement) => {
                let table_count = statement.column_count();
                if (table_count != 1) {
                    println!("table does exist: {}", table_count);
                    return Ok(self);
                } else {
                    println!("table does not exist");
                    return Err(
                        error_stack::Report::new(errors::DatabaseError::InitializingError)
                            .attach_printable("table pokemons does not exist"),
                    );
                }
            }
            Err(err) => return Err(err),
        }
        // conn.execute(
        //     "CREATE TABLE pokemons (id INTEGER PRIMARY KEY AUTOINCREMENT, name  TEXT NOT NULL)",
        //     (), // empty list of parameters.
        // )
        // .report()
        // .attach_printable(format!("couldn't execute database query"))
        // .change_context(errors::DatabaseError::DatabaseExist("sda".into()))?;

        Ok(self)
    }
    pub async fn get_game(&self, id: &i32) -> Option<&Pokemon> {
        // let conn = self.establish_sql_connection().await;
        self.pokemons.iter().find(|&pokemon| pokemon.id == *id)
    }
    pub async fn get_all_games(&self) -> Option<&Vec<Pokemon>> {
        // let conn = self.establish_sql_connection().await;
        Some(&self.pokemons)
    }
    pub async fn get_pokemon(&self, id: &i32) -> Option<&Pokemon> {
        // let conn = self.establish_sql_connection().await;
        self.pokemons.iter().find(|&pokemon| pokemon.id == *id)
    }
    pub async fn get_all_pokemon(&self) -> Option<&Vec<Pokemon>> {
        if let Ok(conn) = self.establish_sql_connection().await {
            if let Ok(statement) = conn.prepare("SELECT * FROM pokemons") {
                return Some(&self.pokemons);
            } else {
                println!("couldn't select all pokemons")
            }
            return Some(&self.pokemons);
        } else {
            return None;
        }
    }
}

// To make our Database usable by Juniper, marker trait has to be implemented
impl juniper::Context for Database {}
