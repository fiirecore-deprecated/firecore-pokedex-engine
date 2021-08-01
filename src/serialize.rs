use serde::{Deserialize, Serialize};

use hashbrown::HashMap;

use crate::{battle_move::serialized::SerializedBattleMoveBytes, pokemon::Pokemon, item::Item, moves::Move};

pub type SerializedTrainers = HashMap<crate::TrainerId, Vec<u8>>;
pub type SerializedTexture = Vec<u8>;
pub type SerializedAudio = Vec<u8>;

#[derive(Deserialize, Serialize)]
pub struct SerializedDex {
    pub pokemon: Vec<SerializedPokemon>,
    pub moves: Vec<Move>,
    pub items: Vec<SerializedItem>,
    pub trainers: SerializedTrainers,
    pub battle_moves: Vec<SerializedBattleMoveBytes>,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedPokemon {
    pub pokemon: Pokemon,
    pub cry: SerializedAudio,
    pub front: SerializedTexture,
    pub back: SerializedTexture,
    pub icon: SerializedTexture,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedItem {
    pub item: Item,
    pub texture: SerializedTexture,
}
