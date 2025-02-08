pub extern crate game_features;
pub extern crate hibitset;

extern crate crossterm;

pub use game_clock::*;
pub use game_features::*;
pub use hibitset::BitSet as HBitSet;
pub use stopwatch2::*;

pub use game_engine_core::*;
pub use planck_ecs::*;
pub use planck_ecs_bundle::*;
pub use planck_game_features::*;

// macro re-export
pub use derive_new::*;

mod components;
mod event;
mod pathfinding;
mod render;
mod resources;
mod systems;
mod terminal;
mod utils;

pub use self::components::*;
pub use self::event::*;
pub use self::pathfinding::*;
pub use self::render::*;
pub use self::resources::*;
pub use self::systems::*;
pub use self::terminal::*;
pub use self::utils::*;

use std::collections::HashMap;

use std::fmt::Debug;
use std::hash::Hash;
