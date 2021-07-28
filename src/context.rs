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

                if !pokemon.cry_ogg.is_empty() {
                    engine::audio::add_sound(
                        &ctx.audio.sound,
                        SoundData {
                            bytes: pokemon.cry_ogg,
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
