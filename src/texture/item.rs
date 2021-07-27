use deps::hash::HashMap;

use engine::{
    tetra::graphics::Texture,
    graphics::TextureManager,
};

use pokedex::item::ItemId;

pub struct ItemTextures(pub(crate) HashMap<ItemId, Texture>);

impl TextureManager for ItemTextures {
    type Id = ItemId;

    fn map(&self) -> &HashMap<Self::Id, Texture> {
        &self.0
    }
    
    fn map_mut(&mut self) -> &mut HashMap<Self::Id, Texture> {
        &mut self.0
    }

}