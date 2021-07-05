use deps::hash::HashMap;

use engine::{
    tetra::graphics::Texture,
    graphics::TextureManager,
};

use pokedex::item::ItemId;

pub struct ItemTextures;

static mut TEXTURES: Option<HashMap<<ItemTextures as TextureManager>::Id, Texture>> = None;

impl TextureManager for ItemTextures {
    type Id = ItemId;
    
    fn map<'a>() -> &'a mut Option<HashMap<Self::Id, Texture>> {
        unsafe { &mut TEXTURES }
    }

}