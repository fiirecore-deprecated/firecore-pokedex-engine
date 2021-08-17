use core::cell::Cell;
use engine::tetra::graphics::Texture;

use pokedex::{pokemon::OwnedRefPokemon, Identifiable};

use crate::{context::PokedexClientContext, gui::IntegerStr4, texture::PokemonTexture};

#[derive(Default)]
pub struct PartyCell {
    pub icon: Cell<Option<Texture>>,
    pub level: IntegerStr4,
    pub health: CellHealth,
}

impl PartyCell {
    pub const ICON_TICK: f32 = 0.15;

    pub fn init<'d, U>(&self, ctx: &PokedexClientContext<U>, pokemon: &OwnedRefPokemon<'d, U>) {
        self.level.update_or_default(pokemon.level as _);
        self.health.update_or_default(pokemon);
        self.icon.set(Some(
            ctx.pokemon_textures
                .get(pokemon.pokemon.id(), PokemonTexture::Icon)
                .clone(),
        ));
    }

    pub fn clear(&self) {
        self.icon.set(Default::default());
        self.level.clear();
        self.health.clear();
    }
}

#[derive(Default, Clone)]
pub struct CellHealth {
    pub current: IntegerStr4,
    pub maximum: IntegerStr4,
    pub percent: Cell<f32>,
}

impl CellHealth {
    pub fn clear(&self) {
        self.current.clear();
        self.maximum.clear();
        self.percent.set(0.0);
    }
    pub fn update_or_default<'d, U>(&self, pokemon: &OwnedRefPokemon<'d, U>) {
        self.current.update_or_default(pokemon.hp());
        self.maximum.update_or_default(pokemon.max_hp());
        self.percent.set(pokemon.percent_hp());
    }
}
