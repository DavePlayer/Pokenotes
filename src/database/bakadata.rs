use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
   games: Vec<GameYaml> ,
   pokemons: Vec<PokemonYaml>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// simple user object
pub struct PokemonYaml {
    pub id: u32,
    pub name: String,
    pub games_occurrence: Vec<u32>,
    // pokedexes: []
    // genders: []
    // moves: []
    // abilities: []
    // types: []
    // locations: []
    stats: [StatYaml; 6]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatYaml {
    base_stat: u32,
    name: String
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameYaml {
    pub id: u32,
    pub name: String,
    pub pokemons: Vec<u32>
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