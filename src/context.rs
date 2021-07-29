use engine::{
    graphics::byte_texture,
    tetra::{graphics::Texture, Result},
    EngineContext,
};

use crate::{
    battle_move::BattleMovedex,
    id::Dex,
    item::Itemdex,
    moves::Movedex,
    pokemon::Pokedex,
    serialize::SerializedDex,
    texture::{ItemTextures, PokemonTextures, TrainerTextures},
};

pub struct PokedexClientContext {
    pub health_bar: Texture,
    pub bag_background: Texture,
    pub party: PokedexPartyContext,
    pub pokemon_textures: PokemonTextures,
    pub item_textures: ItemTextures,
    pub trainer_textures: TrainerTextures,
}

pub struct PokedexPartyContext {
    pub background: Texture,
    pub ball: Texture,
    pub select: Texture,
    pub summary: PokedexSummaryContext,
}

pub struct PokedexSummaryContext {
    pub pages: [Texture; 3],
    pub background: Texture,
}

impl PokedexClientContext {
    pub fn new(ctx: &mut EngineContext, dex: SerializedDex) -> Result<Self> {
        let mut pokedex = Pokedex::with_capacity(dex.pokemon.len());

        let mut pokemon_textures = PokemonTextures::with_capacity(dex.pokemon.len());

        for pokemon in dex.pokemon {
            pokemon_textures.insert(ctx, &pokemon)?;

            #[cfg(feature = "audio")]
            {
                use engine::audio::{serialized::SerializedSoundData as SoundData, sound::Sound};

                if !pokemon.cry.is_empty() {
                    engine::audio::add_sound(
                        &ctx.audio.sound,
                        SoundData {
                            bytes: pokemon.cry,
                            sound: Sound {
                                name: crate::CRY_ID,
                                variant: Some(pokemon.pokemon.id),
                            },
                        },
                    )
                }
            }

            pokedex.insert(pokemon.pokemon.id, pokemon.pokemon);
        }

        Pokedex::set(pokedex);

        Movedex::set(dex.moves.into_iter().map(|m| (m.id, m)).collect());
        BattleMovedex::set(dex.battle_moves.into_iter().map(|m| (m.id, m.into(ctx))).collect());

        let mut itemdex = Itemdex::with_capacity(dex.items.len());

        let mut item_textures = ItemTextures::with_capacity(dex.items.len());

        for item in dex.items.into_iter() {
            item_textures.insert(item.item.id, Texture::from_file_data(ctx, &item.texture)?);
            itemdex.insert(item.item.id, item.item);
        }

        let mut trainer_textures = TrainerTextures::with_capacity(dex.trainers.len());

        for (id, texture) in dex.trainers {
            trainer_textures.insert(id, Texture::from_file_data(ctx, &texture)?);
        }

        Itemdex::set(itemdex);

        Ok(Self {
            health_bar: byte_texture(ctx, include_bytes!("../assets/health.png")),
            bag_background: byte_texture(ctx, include_bytes!("../assets/bag/items.png")),
            party: PokedexPartyContext {
                background: byte_texture(ctx, include_bytes!("../assets/party/background.png")),
                ball: byte_texture(ctx, include_bytes!("../assets/party/ball.png")),
                select: byte_texture(ctx, include_bytes!("../assets/party/select.png")),
                summary: PokedexSummaryContext {
                    background: byte_texture(
                        ctx,
                        include_bytes!("../assets/party/summary/pokemon.png"),
                    ),
                    pages: [
                        byte_texture(ctx, include_bytes!("../assets/party/summary/info.png")),
                        byte_texture(ctx, include_bytes!("../assets/party/summary/skills.png")),
                        byte_texture(ctx, include_bytes!("../assets/party/summary/moves.png")),
                    ],
                },
            },
            pokemon_textures,
            item_textures,
            trainer_textures,
        })
    }
}
