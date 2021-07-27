pub extern crate firecore_engine as engine;
extern crate firecore_dependencies as deps;
extern crate firecore_pokedex as pokedex;

pub use deps::borrow::*;
pub use pokedex::*;

pub mod context;
pub mod battle_move;
pub mod texture;
pub mod gui;

pub mod serialize;

pub const CRY_ID: deps::str::TinyStr8 = unsafe { deps::str::TinyStr8::new_unchecked(7959107) };