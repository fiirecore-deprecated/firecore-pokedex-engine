use deps::hash::HashMap;
use pokedex::trainer::TrainerId;

use engine::{
    tetra::graphics::Texture,
    graphics::TextureManager,
};

pub struct TrainerTextures;

static mut TRAINER_TEXTURES: Option<HashMap<TrainerId, Texture>> = None;

impl TextureManager for TrainerTextures {
    type Id = TrainerId;

    fn map<'a>() -> &'a mut Option<HashMap<Self::Id, Texture>> {
        unsafe { &mut TRAINER_TEXTURES }
    }
}
