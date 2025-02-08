pub extern crate bracket_lib;
pub extern crate game_features;
pub extern crate hibitset;

extern crate crossterm;

pub use bracket_lib::prelude::{
    a_star_search, field_of_view, main_loop, to_cp437, Algorithm2D, BError, BEvent, BTerm,
    BTermBuilder, BaseMap, GameState, MultiTileSprite, NavigationPath, Point, Rect, SmallVec,
    SpriteSheet, VirtualKeyCode, BLACK, BLUE, EMBED, GREEN, INPUT, RED, RGBA, WHITE, YELLOW,
};
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

//mod commands;
mod components;
mod event;
mod macros;
mod render;
mod resources;
mod systems;
mod utils;

//pub use self::commands::*;
pub use self::components::*;
pub use self::event::*;
pub use self::render::*;
pub use self::resources::*;
pub use self::systems::*;
pub use self::utils::*;

use std::collections::HashMap;

use std::fmt::Debug;
use std::hash::Hash;
