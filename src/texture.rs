use deps::hash::HashMap;
use engine::tetra::{Context, Result, graphics::Texture};

use pokedex::pokemon::PokemonId;

use crate::serialize::SerializedPokemon;

mod item;
mod trainer;

pub use item::ItemTextures;
pub use trainer::TrainerTextures;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PokemonTexture {
    Front,
    Back,
    Icon,
}

pub struct PokemonTextures {
    pub front: HashMap<PokemonId, Texture>,
    pub back: HashMap<PokemonId, Texture>,
    pub icon: HashMap<PokemonId, Texture>,
}

impl PokemonTextures {

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            front: HashMap::with_capacity(capacity),
            back: HashMap::with_capacity(capacity),
            icon: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, ctx: &mut Context, pokemon: &SerializedPokemon) -> Result {
        self.front.insert(pokemon.pokemon.id, Texture::from_file_data(ctx, &pokemon.front_png)?);
		self.back.insert(pokemon.pokemon.id, Texture::from_file_data(ctx, &pokemon.back_png)?);
		self.icon.insert(pokemon.pokemon.id, Texture::from_file_data(ctx, &pokemon.icon_png)?);
        Ok(())
    }

    pub fn get(&self, id: &PokemonId, side: PokemonTexture) -> &Texture {
        match side {
            PokemonTexture::Front => self.front.get(id),
            PokemonTexture::Back => self.back.get(id),
            PokemonTexture::Icon => self.icon.get(id),
        }.unwrap_or_else(|| panic!("Could not get texture for pokemon with ID {}", id))
    }

}