use serde::{Deserialize, Serialize};

use hashbrown::HashMap;

use pokedex::{pokemon::PokemonId, item::ItemId};

pub type SerializedTrainers = HashMap<crate::TrainerId, Vec<u8>>;
pub type SerializedTexture = Vec<u8>;
pub type SerializedAudio = Vec<u8>;

#[derive(Deserialize, Serialize)]
pub struct SerializedPokedexEngine {
    pub pokemon: HashMap<PokemonId, SerializedPokemon>,
    pub items: HashMap<ItemId, SerializedTexture>,
    pub trainers: SerializedTrainers,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedPokemon {
    pub cry: SerializedAudio,
    pub front: SerializedTexture,
    pub back: SerializedTexture,
    pub icon: SerializedTexture,
}
