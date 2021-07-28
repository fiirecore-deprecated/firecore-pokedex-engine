use hashbrown::HashMap;

use engine::{
    tetra::graphics::Texture,
    graphics::TextureManager,
};

use pokedex::item::ItemId;

type ItemTextureMap = HashMap<ItemId, Texture>;

pub struct ItemTextures(pub(crate) ItemTextureMap);

impl ItemTextures {
    pub fn with_capacity(capacity: usize) -> ItemTextureMap {
        ItemTextureMap::with_capacity(capacity)
    }
}

impl TextureManager for ItemTextures {
    type Id = ItemId;

    fn map(&self) -> &HashMap<Self::Id, Texture> {
        &self.0
    }
    
    fn map_mut(&mut self) -> &mut HashMap<Self::Id, Texture> {
        &mut self.0
    }

}