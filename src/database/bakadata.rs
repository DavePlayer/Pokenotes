use serde::{Serialize, Deserialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
   pub games: Vec<GameYaml> ,
   pub pokemons: Vec<PokemonYaml>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// simple user object
pub struct PokemonYaml {
    pub id: Uuid,
    pub name: String,
    pub games_occurrence: Vec<u32>,
    // pokedexes: []
    // genders: []
    // moves: []
    // abilities: []
    // types: []
    // locations: []
    pub stats: [StatYaml; 6]
    // pub stats: Vec<StatYaml>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatYaml {
    pub base_stat: u32,
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameYaml {
    pub id: Uuid,
    pub name: String,
    pub pokemons: Vec<Uuid>
    // pub pokedexes: null
    // pub locations: null
}

impl Data {
    fn new(games: Vec<GameYaml>, pokemons: Vec<PokemonYaml>) -> Data {
        Data {
            games,
            pokemons
        }
    }
}