use crate::graphql::schemas::{game::Game, pokemon::Pokemon};


pub struct Data {
   games: Vec<Game> ,
   pokemons: Vec<Pokemon>
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct StatYaml {
    base_stat: u32,
    name: String
}
#[derive(Clone, Debug)]
pub struct GameYaml {
    pub id: u32,
    pub name: String,
    pub pokemons: Vec<u32>
    // pub pokedexes: null
    // pub locations: null
}

impl Data {
    fn new(games: Vec<GameYaml>, pokemons: Vec<PokemonYaml>) {
        let mut pokemons_to_parse: Vec<Pokemon> = vec![];
        let mut games_to_parse: Vec<Game> = vec![];
        for game_yaml in games {
            for pokemon_id in game_yaml.pokemons {
                let pokemon = pokemons.iter().find(|pok| pok.id == pokemon_id);
                if let Some(pokemon) = pokemon {
                    // penis
                }
            }
        }
    }
}