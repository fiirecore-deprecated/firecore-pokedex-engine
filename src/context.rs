use engine::{
    graphics::byte_texture,
    tetra::{graphics::Texture, Result, Context},
};

use crate::{
    battle_move::BattleMovedex,
    item::Itemdex,
    moves::Movedex,
    pokemon::Pokedex,
    serialize::SerializedDex,
    texture::{ItemTextures, PokemonTextures, TrainerTextures},
    id::Dex, 
};

#[cfg(feature = "audio")]
use crate::serialize::SerializedPokemon;

pub struct PokedexClientContext {
    pub health_bar: Texture,
    pub bag_background: Texture,
    pub pokemon_textures: PokemonTextures,
    pub item_textures: ItemTextures,
    pub trainer_textures: TrainerTextures,
}

impl PokedexClientContext {
    pub fn new(
        ctx: &mut Context,
        dex: SerializedDex,
        #[cfg(feature = "audio")] callback: impl Fn(&mut SerializedPokemon),
    ) -> Result<Self> {

        let mut pokedex = Pokedex::with_capacity(dex.pokemon.len());

        let mut pokemon_textures = PokemonTextures::with_capacity(dex.pokemon.len());
    
        for pokemon in dex.pokemon {
            pokemon_textures.insert(ctx, &pokemon)?;
    
            #[cfg(feature = "audio")]
            let mut pokemon = pokemon;
    
            #[cfg(feature = "audio")]
            (callback)(&mut pokemon);
    
            pokedex.insert(pokemon.pokemon.id, pokemon.pokemon);
        }
    
        Pokedex::set(pokedex);
    
        let mut movedex = Movedex::with_capacity(dex.moves.len());
    
        let mut battle_movedex = BattleMovedex::with_capacity(0);
    
        for serialized_move in dex.moves {
            let pmove = serialized_move.pokemon_move;
            if let Some(battle_move) = serialized_move.battle_move {
                battle_movedex.insert(pmove.id, battle_move.into(ctx));
            }
            movedex.insert(pmove.id, pmove);
        }
    
        Movedex::set(movedex);
        BattleMovedex::set(battle_movedex);
    
        let mut itemdex = Itemdex::with_capacity(dex.items.len());
    
        let mut item_textures = ItemTextures::with_capacity(dex.items.len());
    
        for item in dex.items.into_iter() {
            item_textures.insert(item.item.id, Texture::from_file_data(ctx, &item.texture)?);
            itemdex.insert(item.item.id, item.item);
        }
    
        let trainer_textures = TrainerTextures(
            dex.trainers
                .into_iter()
                .map(|(k, bytes)| (k, Texture::from_file_data(ctx, &bytes).unwrap()))
                .collect(),
        );
    
        Itemdex::set(itemdex);
    
        let item_textures = ItemTextures(item_textures);

        Ok(Self {
            health_bar: byte_texture(ctx, include_bytes!("../assets/health.png")),
            bag_background: byte_texture(ctx, include_bytes!("../assets/bag/items.png")),
            pokemon_textures,
            item_textures,
            trainer_textures,
        })
    }
}
