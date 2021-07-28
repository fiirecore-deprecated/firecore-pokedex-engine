use engine::tetra::graphics::Texture;
use hashbrown::HashMap;

use pokedex::{
    id::{Dex, Identifiable, IdentifiableRef},
    moves::{MoveId, Movedex},
};

pub mod script;
pub mod serialized;

#[derive(Debug, Clone)]
pub struct BattleMove {
    pub id: MoveId,

    pub texture: Option<Texture>,

    pub script: script::BattleActionScript,
}

impl BattleMove {
    pub fn script(&self) -> script::BattleActionScriptInstance {
        script::BattleActionScriptInstance {
            script: self.script.clone(),
            texture: self.texture.clone(),
        }
    }
}

pub type BattleMoveRef = IdentifiableRef<BattleMovedex>;

impl Identifiable for BattleMove {
    type Id = MoveId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub struct BattleMovedex;

static mut BATTLE_MOVE_DEX: Option<HashMap<MoveId, BattleMove>> = None;

impl Dex for BattleMovedex {
    type Kind = BattleMove;

    const UNKNOWN: MoveId = Movedex::UNKNOWN;

    fn dex() -> &'static HashMap<MoveId, BattleMove> {
        unsafe { BATTLE_MOVE_DEX.as_ref().unwrap() }
    }

    fn dex_mut() -> &'static mut Option<HashMap<MoveId, BattleMove>> {
        unsafe { &mut BATTLE_MOVE_DEX }
    }
}
