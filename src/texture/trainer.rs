use deps::hash::HashMap;
use pokedex::trainer::TrainerId;

use engine::{
    tetra::graphics::Texture,
    graphics::TextureManager,
};

pub struct TrainerTextures(pub(crate) HashMap<TrainerId, Texture>);

impl TextureManager for TrainerTextures {
    type Id = TrainerId;

    fn map(&self) -> &HashMap<Self::Id, Texture> {
        &self.0
    }

    fn map_mut(&mut self) -> &mut HashMap<Self::Id, Texture> {
        &mut self.0
    }
}