use engine::{
    graphics::byte_texture,
    tetra::{graphics::Texture, Result},
    EngineContext,
};

use pokedex::{
    item::{Item, Itemdex},
    moves::{Move, Movedex},
    pokemon::{Pokedex, Pokemon},
    Dex,
};

use crate::{
    serialize::SerializedPokedexEngine,
    texture::{ItemTextures, PokemonTextures, TrainerTextures},
};

pub struct PokedexClientContext<'d> {
    pub pokedex: &'d Pokedex,
    pub movedex: &'d Movedex,
    pub itemdex: &'d Itemdex,
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

impl<'d> PokedexClientContext<'d> {
    pub fn new(
        ctx: &mut EngineContext,
        pokedex: &'d Dex<Pokemon>,
        movedex: &'d Dex<Move>,
        itemdex: &'d Dex<Item>,
        engine: SerializedPokedexEngine,
    ) -> Result<Self> {
        let mut pokemon_textures = PokemonTextures::with_capacity(engine.pokemon.len());

        engine.pokemon.into_iter().for_each(|(id, pokemon)| {
            if let Err(err) = pokemon_textures.insert(ctx, id, &pokemon) {
                log::warn!("Cannot add pokemon texture for {} with error {}", id, err)
            }

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
                                variant: Some(id),
                            },
                        },
                    )
                }
            }
        });

        let mut item_textures = ItemTextures::with_capacity(engine.items.len());

        for (id, texture) in engine.items.into_iter() {
            item_textures.insert(id, Texture::from_file_data(ctx, &texture)?);
        }

        let mut trainer_textures = TrainerTextures::with_capacity(engine.trainers.len());

        for (id, texture) in engine.trainers {
            trainer_textures.insert(id, Texture::from_file_data(ctx, &texture)?);
        }

        Ok(Self {
            pokedex,
            movedex,
            itemdex,
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
