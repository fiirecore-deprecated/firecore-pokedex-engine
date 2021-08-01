pub extern crate firecore_engine as engine;
extern crate firecore_pokedex as pokedex;

pub use pokedex::*;

pub mod battle_move;
pub mod context;
pub mod gui;
pub mod itemstack;
pub mod texture;

pub mod serialize;

pub const CRY_ID: tinystr::TinyStr8 = unsafe { tinystr::TinyStr8::new_unchecked(7959107) };

pub type TrainerId = tinystr::TinyStr16;
